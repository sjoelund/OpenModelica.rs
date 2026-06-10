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

use crate::BackendDAEFunc;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DumpGraphML;
use crate::IndexReduction;
use crate::Sorting;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::Ceval;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// just a matching algorithm
// - PerfectMatching
// - RegularMatching
//
// =============================================================================
pub fn PerfectMatching(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut N: i32 = metamodelica::arrayLength(m.clone());
    ass1 = arrayCreate(N.clone(), -1);
    ass2 = arrayCreate(N.clone(), -1);
    let (__pa0, __pa1, true, _, _) = (ContinueMatching(m.clone(), N.clone(), N.clone(), ass1.clone(), ass2.clone(), true)?) else { bail!("pattern mismatch") };
    ass1 = __pa0.clone();
    ass2 = __pa1.clone();
    Ok((ass1, ass2))
}

pub fn RegularMatching(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nVars: i32, mut nEqns: i32) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, bool, metamodelica::Array<bool>, metamodelica::Array<bool>)> {
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut perfectMatching: bool;
    let mut eMark: metamodelica::Array<bool>;
    let mut vMark: metamodelica::Array<bool>;
    ass1 = arrayCreate(nVars.clone(), -1);
    ass2 = arrayCreate(nEqns.clone(), -1);
    (ass1, ass2, perfectMatching, eMark, vMark) = ContinueMatching(m.clone(), nVars.clone(), nEqns.clone(), ass1.clone(), ass2.clone(), false)?;
    Ok((ass1, ass2, perfectMatching, eMark, vMark))
}

pub fn ContinueMatching(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nVars: i32, mut nEqns: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut stopAtSingularity: bool) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, bool, metamodelica::Array<bool>, metamodelica::Array<bool>)> {
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut perfectMatching: bool = true;
    let mut eMark: metamodelica::Array<bool>;
    let mut vMark: metamodelica::Array<bool>;
    let mut i: i32;
    let mut j: i32;
    let mut eMarkIx: metamodelica::Array<i32>;
    let mut vMarkIx: metamodelica::Array<i32>;
    let mut eMarkN: i32 = 0;
    let mut vMarkN: i32 = 0;
    let mut success: bool;
    vMark = arrayCreate(nVars.clone(), false);
    eMark = arrayCreate(nEqns.clone(), false);
    vMarkIx = arrayCreate(nVars.clone(), 0);
    eMarkIx = arrayCreate(nEqns.clone(), 0);
    i = 1;
    while i.clone() <= nEqns.clone() {
        j = ({let __elt = ass2.borrow()[(i.clone()-1) as usize].clone(); __elt});
        if !(j.clone() > 0 && ({let __elt = ass1.borrow()[(j.clone()-1) as usize].clone(); __elt}) == i.clone()) {
            clearArrayWithKnownSetIndexes(eMark.clone(), eMarkIx.clone(), eMarkN.clone())?;
            clearArrayWithKnownSetIndexes(vMark.clone(), vMarkIx.clone(), vMarkN.clone())?;
            (success, eMarkN, vMarkN) = BBPathFound(i.clone(), m.clone(), eMark.clone(), vMark.clone(), ass1.clone(), ass2.clone(), eMarkIx.clone(), vMarkIx.clone(), 0, 0)?;
            if !(success.clone()) {
                perfectMatching = false;
                if stopAtSingularity.clone() {
                    return Ok((ass1.clone(), ass2.clone(), perfectMatching.clone(), eMark.clone(), vMark.clone()));
                }
            }
        }
        i = i.clone() + 1;
    }
    Ok((ass1, ass2, perfectMatching, eMark, vMark))
}

pub fn BBMatching(mut inSys: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outSys: Arc<BackendDAE::EqSystem> = inSys.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = inArg.clone();
    let mut i: i32;
    let mut success: bool = true;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nVars: i32;
    let mut nEqns: i32;
    let mut j: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut eMark: metamodelica::Array<bool>;
    let mut vMark: metamodelica::Array<bool>;
    let mut eMarkIx: metamodelica::Array<i32>;
    let mut vMarkIx: metamodelica::Array<i32>;
    let mut eMarkN: i32 = 0;
    let mut vMarkN: i32 = 0;
    let mut mEqns: Arc<metamodelica::List<i32>>;
    let __pa0 = ::match_deref::match_deref! { match &(outSys.m.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    nEqns = BackendDAEUtil::systemSize(outSys.clone())?;
    nVars = BackendVariable::daenumVariables(outSys.clone());
    ass2 = arrayCreate(nEqns.clone(), -1);
    ass1 = arrayCreate(nVars.clone(), -1);
    vMark = arrayCreate(nVars.clone(), false);
    eMark = arrayCreate(nEqns.clone(), false);
    vMarkIx = arrayCreate(nVars.clone(), 0);
    eMarkIx = arrayCreate(nEqns.clone(), 0);
    i = 1;
    while i.clone() <= nEqns.clone() && success.clone() {
        j = ({let __elt = ass2.borrow()[(i.clone()-1) as usize].clone(); __elt});
        if j.clone() > 0 && ({let __elt = ass1.borrow()[(j.clone()-1) as usize].clone(); __elt}) == i.clone() {
            success = true;
        } else {
            clearArrayWithKnownSetIndexes(eMark.clone(), eMarkIx.clone(), eMarkN.clone())?;
            clearArrayWithKnownSetIndexes(vMark.clone(), vMarkIx.clone(), vMarkN.clone())?;
            (success, eMarkN, vMarkN) = BBPathFound(i.clone(), m.clone(), eMark.clone(), vMark.clone(), ass1.clone(), ass2.clone(), eMarkIx.clone(), vMarkIx.clone(), 0, 0)?;
            if !(success.clone()) {
                mEqns = metamodelica::nil();
                for mut j in 1..=nEqns.clone() {
                    if ({let __elt = eMark.borrow()[(j.clone()-1) as usize].clone(); __elt}) {
                        mEqns = metamodelica::cons(j.clone(), mEqns.clone());
                    }
                }
                (_, i, outSys, outShared, ass1, ass2, outArg) = sssHandler(list![mEqns.clone()], i.clone(), outSys.clone(), outShared.clone(), ass1.clone(), ass2.clone(), outArg.clone())?;
                let __pa1 = ::match_deref::match_deref! { match &(outSys.m.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                m = __pa1.clone();
                success = true;
                i = i.clone() - 1;
            }
        }
        i = i.clone() + 1;
    }
    if success.clone() {
        outSys = BackendDAEUtil::setEqSystMatching(outSys.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }))?;
    } else {
        metamodelica::print((literal!("\nSingular System!!!\n")).clone());
    }
    Ok((outSys, outShared, outArg))
}

fn BBPathFound(mut i: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eMark: metamodelica::Array<bool>, mut vMark: metamodelica::Array<bool>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut eMarkIx: metamodelica::Array<i32>, mut vMarkIx: metamodelica::Array<i32>, mut eMarkN: i32, mut vMarkN: i32) -> Result<(bool, i32, i32)> {
    let mut success: bool = false;
    let mut eMarkN: i32 = eMarkN;
    let mut vMarkN: i32 = vMarkN;
    if metamodelica::arrayGet(eMark.clone(), i.clone())? {
        return Ok((success.clone(), eMarkN.clone(), vMarkN.clone()));
    }
    metamodelica::arrayUpdate(eMark.clone(), i.clone(), true)?;
    eMarkN = eMarkN.clone() + 1;
    metamodelica::arrayUpdate(eMarkIx.clone(), eMarkN.clone(), i.clone())?;
    let __range0 = &*({let __elt = m.borrow()[(i.clone()-1) as usize].clone(); __elt});
    for mut j in __range0 {
        let mut j = j.clone();
        if j.clone() > 0 && ({let __elt = ass1.borrow()[(j.clone()-1) as usize].clone(); __elt}) <= 0 {
            success = true;
            metamodelica::arrayUpdate(ass1.clone(), j.clone(), i.clone())?;
            metamodelica::arrayUpdate(ass2.clone(), i.clone(), j.clone())?;
            return Ok((success.clone(), eMarkN.clone(), vMarkN.clone()));
        }
    }
    let __range1 = &*({let __elt = m.borrow()[(i.clone()-1) as usize].clone(); __elt});
    for mut j in __range1 {
        let mut j = j.clone();
        if j.clone() > 0 && !(({let __elt = vMark.borrow()[(j.clone()-1) as usize].clone(); __elt})) {
            metamodelica::arrayUpdate(vMark.clone(), j.clone(), true)?;
            vMarkN = vMarkN.clone() + 1;
            metamodelica::arrayUpdate(vMarkIx.clone(), vMarkN.clone(), j.clone())?;
            (success, eMarkN, vMarkN) = BBPathFound(({let __elt = ass1.borrow()[(j.clone()-1) as usize].clone(); __elt}), m.clone(), eMark.clone(), vMark.clone(), ass1.clone(), ass2.clone(), eMarkIx.clone(), vMarkIx.clone(), eMarkN.clone(), vMarkN.clone())?;
            if success.clone() {
                metamodelica::arrayUpdate(ass1.clone(), j.clone(), i.clone())?;
                metamodelica::arrayUpdate(ass2.clone(), i.clone(), j.clone())?;
                return Ok((success.clone(), eMarkN.clone(), vMarkN.clone()));
            }
        }
    }
    Ok((success, eMarkN, vMarkN))
}

fn BBCheapMatching(mut nEqns: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let mut i: i32 = 0;
    let mut j: i32;
    let mut success: bool = false;
    let mut vars: Arc<metamodelica::List<i32>>;
    for mut i in 1..=nEqns.clone() {
        vars = ({let __elt = m.borrow()[(i.clone()-1) as usize].clone(); __elt});
        while !(success.clone()) && !(vars.clone().is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vars.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            j = __pa0.clone();
            vars = __pa1.clone();
            if j.clone() > 0 && ({let __elt = ass1.borrow()[(j.clone()-1) as usize].clone(); __elt}) <= 0 {
                success = true;
                metamodelica::arrayUpdate(ass1.clone(), j.clone(), i.clone())?;
                metamodelica::arrayUpdate(ass2.clone(), i.clone(), j.clone())?;
            }
        }
    }
    Ok(())
}

pub fn invertMatching(mut inAss: metamodelica::Array<i32>) -> metamodelica::Array<i32> {
    let mut outAss: metamodelica::Array<i32>;
    let mut N: i32 = metamodelica::arrayLength(inAss.clone());
    let mut j: i32;
    outAss = arrayCreate(N.clone(), -1);
    for mut i in 1..=N.clone() {
        j = ({let __elt = inAss.borrow()[(i.clone()-1) as usize].clone(); __elt});
        if j.clone() > 0 {
            {
                let __cell0 = i.clone();
                let __idx0 = ({let __elt = inAss.borrow()[(i.clone()-1) as usize].clone(); __elt});
                outAss.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
        }
    }
    outAss
}

// =============================================================================
// Matching Algorithms
//
// =============================================================================
pub fn DFSLH(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut emark: metamodelica::Array<i32> = Default::default();
                    let mut vmark: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vmark = arrayCreate(nvars.clone(), -1);
                    emark = arrayCreate(neqns.clone(), -1);
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = DFSLH2(isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), 1, emark.clone(), vmark.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec1.clone(), ass2: vec2.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.DFSLH failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn DFSLH2(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut nf: i32, mut i: i32, mut emark: metamodelica::Array<i32>, mut vmark: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut match_opts: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outAssignments1, outAssignments2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = (isyst.clone(), match_opts.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    let true = (intGe(i.clone(), nv.clone())) else { bail!("pattern mismatch") };
                    (ass1_1, ass2_1) = pathFound(m.clone(), mt.clone(), i.clone(), i.clone(), emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone(), syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { m: Some(_), mT: Some(_), .. }, _) => {
                    let mut ass1_2: metamodelica::Array<i32> = Default::default();
                    let mut ass2_2: metamodelica::Array<i32> = Default::default();
                    let mut i_1: i32 = 0;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut syst = (*syst).clone();
                    i_1 = i.clone() + 1;
                    let true = (intGt(({let __elt = ass2.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    (ass1_2, ass2_2, syst, shared, arg) = DFSLH2(syst.clone(), ishared.clone(), nv.clone(), nf.clone(), i_1.clone(), emark.clone(), vmark.clone(), ass1.clone(), ass2.clone(), match_opts.clone(), sssHandler.clone(), inArg.clone())?;
                    Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    let mut ass1_2: metamodelica::Array<i32> = Default::default();
                    let mut ass2_2: metamodelica::Array<i32> = Default::default();
                    let mut i_1: i32 = 0;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut syst = (*syst).clone();
                    i_1 = i.clone() + 1;
                    (ass1_1, ass2_1) = pathFound(m.clone(), mt.clone(), i.clone(), i.clone(), emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
                    (ass1_2, ass2_2, syst, shared, arg) = DFSLH2(syst.clone(), ishared.clone(), nv.clone(), nf.clone(), i_1.clone(), emark.clone(), vmark.clone(), ass1_1.clone(), ass2_1.clone(), match_opts.clone(), sssHandler.clone(), inArg.clone())?;
                    Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    let mut ass1_2: metamodelica::Array<i32> = Default::default();
                    let mut ass2_2: metamodelica::Array<i32> = Default::default();
                    let mut ass1_3: metamodelica::Array<i32> = Default::default();
                    let mut ass2_3: metamodelica::Array<i32> = Default::default();
                    let mut emark1: metamodelica::Array<i32> = Default::default();
                    let mut vmark1: metamodelica::Array<i32> = Default::default();
                    let mut i_1: i32 = 0;
                    let mut nv_1: i32 = 0;
                    let mut nf_1: i32 = 0;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
                    let mut meqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    meqns = getMarked(nf.clone(), i.clone(), emark.clone(), metamodelica::nil());
                    (_, i_1, syst, shared, ass1_1, ass2_1, arg) = sssHandler(list![meqns.clone()], i.clone(), isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    nf_1 = BackendEquation::equationArraySize(eqns.clone())?;
                    nv_1 = BackendVariable::varsSize(BackendVariable::daeVars(syst.clone()));
                    ass1_2 = assignmentsArrayExpand(ass1_1.clone(), nv_1.clone(), metamodelica::arrayLength(ass1_1.clone()), -1)?;
                    ass2_2 = assignmentsArrayExpand(ass2_1.clone(), nf_1.clone(), metamodelica::arrayLength(ass2_1.clone()), -1)?;
                    vmark1 = assignmentsArrayExpand(vmark.clone(), nv_1.clone(), metamodelica::arrayLength(vmark.clone()), -1)?;
                    emark1 = assignmentsArrayExpand(emark.clone(), nf_1.clone(), metamodelica::arrayLength(emark.clone()), -1)?;
                    (ass1_3, ass2_3, syst, shared, arg1) = DFSLH2(syst.clone(), shared.clone(), nv_1.clone(), nf_1.clone(), i_1.clone(), emark1.clone(), vmark1.clone(), ass1_2.clone(), ass2_2.clone(), match_opts.clone(), sssHandler.clone(), arg.clone())?;
                    Ok((ass1_3.clone(), ass2_3.clone(), syst.clone(), shared.clone(), arg1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut eqn_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    eqn_lst = getMarked(nf.clone(), i.clone(), emark.clone(), metamodelica::nil());
                    singularSystemError(list![eqn_lst.clone()], i.clone(), isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAssignments1, outAssignments2, osyst, oshared, outArg))
}

fn pathFound(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut i: i32, mut imark: i32, mut emark: metamodelica::Array<i32>, mut vmark: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    (outAssignments1, outAssignments2) = 'mc: {
        let __mc_input = ass2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            metamodelica::arrayUpdate(emark.clone(), i.clone(), imark.clone())?;
            (ass1_1, ass2_1) = assignOneInEqn(m.clone(), mt.clone(), i.clone(), ass1.clone(), ass2.clone())?;
            Ok((ass1_1.clone(), ass2_1.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            (ass1_1, ass2_1) = forallUnmarkedVarsInEqn(m.clone(), mt.clone(), i.clone(), imark.clone(), emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
            Ok((ass1_1.clone(), ass2_1.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAssignments1, outAssignments2))
}

fn assignOneInEqn(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut i: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut vars: Arc<metamodelica::List<i32>>;
    vars = BackendDAEUtil::varsInEqn(m.clone(), i.clone())?;
    (outAssignments1, outAssignments2) = assignFirstUnassigned(i.clone(), vars.clone(), ass1.clone(), ass2.clone())?;
    Ok((outAssignments1, outAssignments2))
}

fn assignFirstUnassigned(mut i: i32, mut inIntegerLst2: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    (outAssignments1, outAssignments2) = 'mc: {
        let __mc_input = inIntegerLst2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: _ } => {
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    let false = (intGt(({let __elt = ass1.borrow()[(v.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    ass1_1 = metamodelica::arrayUpdate(ass1.clone(), v.clone(), i.clone())?;
                    ass2_1 = metamodelica::arrayUpdate(ass2.clone(), i.clone(), v.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: vs } => {
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    (ass1_1, ass2_1) = assignFirstUnassigned(i.clone(), vs.clone(), ass1.clone(), ass2.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAssignments1, outAssignments2))
}

fn forallUnmarkedVarsInEqn(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut i: i32, mut imark: i32, mut emark: metamodelica::Array<i32>, mut vmark: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut vars: Arc<metamodelica::List<i32>>;
    let mut vars_1: Arc<metamodelica::List<i32>>;
    vars = BackendDAEUtil::varsInEqn(m.clone(), i.clone())?;
    vars_1 = List::filter1OnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(isNotVMarked, i32, (i32, metamodelica::Array<i32>))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, metamodelica::Array<i32>)) -> Result<bool> + 'static>), (imark.clone(), vmark.clone()))?;
    (outAssignments1, outAssignments2) = forallUnmarkedVarsInEqnBody(m.clone(), mt.clone(), i.clone(), imark.clone(), emark.clone(), vmark.clone(), vars_1.clone(), ass1.clone(), ass2.clone())?;
    Ok((outAssignments1, outAssignments2))
}

fn isNotVMarked(mut i: i32, mut inTpl: (i32, metamodelica::Array<i32>)) -> bool {
    let mut outB: bool;
    let mut imark: i32;
    let mut vmark: metamodelica::Array<i32>;
    (imark, vmark) = inTpl.clone();
    outB = !(intEq(imark.clone(), ({let __elt = vmark.borrow()[(i.clone()-1) as usize].clone(); __elt})));
    outB
}

fn forallUnmarkedVarsInEqnBody(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut i: i32, mut imark: i32, mut emark: metamodelica::Array<i32>, mut vmark: metamodelica::Array<i32>, mut inIntegerLst4: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    (outAssignments1, outAssignments2) = 'mc: {
        let __mc_input = inIntegerLst4.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: _ } => {
                    let mut assarg: i32 = 0;
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    let mut ass1_2: metamodelica::Array<i32> = Default::default();
                    let mut ass2_2: metamodelica::Array<i32> = Default::default();
                    metamodelica::arrayUpdate(vmark.clone(), v.clone(), imark.clone())?;
                    assarg = ({let __elt = ass1.borrow()[(v.clone()-1) as usize].clone(); __elt});
                    (ass1_1, ass2_1) = pathFound(m.clone(), mt.clone(), assarg.clone(), imark.clone(), emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
                    ass1_2 = metamodelica::arrayUpdate(ass1_1.clone(), v.clone(), i.clone())?;
                    ass2_2 = metamodelica::arrayUpdate(ass2_1.clone(), i.clone(), v.clone())?;
                    Ok((ass1_2.clone(), ass2_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: vs } => {
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    (ass1_1, ass2_1) = forallUnmarkedVarsInEqnBody(m.clone(), mt.clone(), i.clone(), imark.clone(), emark.clone(), vmark.clone(), vs.clone(), ass1.clone(), ass2.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAssignments1, outAssignments2))
}

pub fn BFSB(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut parentcolum: metamodelica::Array<i32> = Default::default();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    parentcolum = arrayCreate(nvars.clone(), -1);
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = BFSB1(1, 1, nvars.clone(), neqns.clone(), m.clone(), mt.clone(), rowmarks.clone(), parentcolum.clone(), vec1.clone(), vec2.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.BFSB failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn BFSB1(mut i: i32, mut rowmark: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut parentcolum: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
            Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut visitedcolums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut mt1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut parentcolum1: metamodelica::Array<i32> = Default::default();
            let false = (intGt(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            visitedcolums = BFSBphase(list![i.clone()], rowmark.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), metamodelica::nil())?;
            let (__pa0, __pa3, __pa1, __pa2, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(reduceIndexifNecessary(visitedcolums.clone(), i.clone(), isyst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?) {
                (_, __pa0, __pa3 @ Deref @ BackendDAE::EqSystem { m: Some(__pa1), mT: Some(__pa2), .. }, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i_1 = __pa0.clone();
            m1 = __pa1.clone();
            mt1 = __pa2.clone();
            syst = __pa3.clone();
            shared = __pa4.clone();
            nv_1 = __pa5.clone();
            ne_1 = __pa6.clone();
            ass1_1 = __pa7.clone();
            ass2_1 = __pa8.clone();
            arg = __pa9.clone();
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1.clone(), metamodelica::arrayLength(rowmarks.clone()), -1)?;
            parentcolum1 = assignmentsArrayExpand(parentcolum.clone(), nv_1.clone(), metamodelica::arrayLength(parentcolum.clone()), -1)?;
            (ass1_2, ass2_2, syst, shared, arg) = BFSB1(i_1.clone(), rowmark.clone() + 1, nv_1.clone(), ne_1.clone(), m1.clone(), mt1.clone(), rowmarks1.clone(), parentcolum1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone())?;
            Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let true = (intGt(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            (ass1_1, ass2_1, syst, shared, arg) = BFSB1(i.clone() + 1, rowmark.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            Ok((ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function BFSB1 failed in equation ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn BFSBphase(mut queue: Arc<metamodelica::List<i32>>, mut rowmark: i32, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut parentcolum: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextQueue: Arc<metamodelica::List<i32>>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((queue.clone(), nextQueue.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(inVisitedColums.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            { (queue, rowmark, i, nv, ne, m, mT, rowmarks, parentcolum, ass1, ass2, nextQueue, inVisitedColums) = (nextQueue.clone(), rowmark.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), inVisitedColums.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _) => {
            let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (queue1, b) = BFSBtraverseRows(rows.clone(), nextQueue.clone(), rowmark.clone(), i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone())?;
            return Ok(BFSBphase1(b.clone(), rest.clone(), rowmark.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), queue1.clone(), metamodelica::cons(c.clone(), inVisitedColums.clone()))?)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function BFSBphase failed in equation ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn BFSBphase1(mut inPathFound: bool, mut queue: Arc<metamodelica::List<i32>>, mut rowmark: i32, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut parentcolum: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextQueue: Arc<metamodelica::List<i32>>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = (match inPathFound.clone() {
        true => metamodelica::nil(),
        false => BFSBphase(queue.clone(), rowmark.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), nextQueue.clone(), inVisitedColums.clone())?,
        _ => {
            Error::addInternalError((literal!("function BFSBphase1 failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
    });
    Ok(outVisitedColums)
}

fn BFSBtraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut queue: Arc<metamodelica::List<i32>>, mut rowmark: i32, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut parentcolum: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, bool)> {
    let mut outEqnqueue: Arc<metamodelica::List<i32>>;
    let mut pathFound: bool;
    (outEqnqueue, pathFound) = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((queue.clone().reverse(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    BFSBreasign(i.clone(), c.clone(), parentcolum.clone(), r.clone(), ass1.clone(), ass2.clone())?;
                    Ok((metamodelica::nil(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut queue2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rc: i32 = 0;
                    let mut b: bool = false;
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    queue1 = BFSBenque(queue.clone(), rowmark.clone(), c.clone(), rc.clone(), r.clone(), intLt(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), rowmark.clone()), rowmarks.clone(), parentcolum.clone())?;
                    (queue2, b) = BFSBtraverseRows(rest.clone(), queue1.clone(), rowmark.clone(), i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone())?;
                    Ok((queue2.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function BFSBtraverseRows failed in equation ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqnqueue, pathFound))
}

fn BFSBreasign(mut i: i32, mut c: i32, mut parentcolum: metamodelica::Array<i32>, mut l: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = ass2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(i.clone(), c.clone())) else { bail!("pattern mismatch") };
            metamodelica::arrayUpdate(ass1.clone(), c.clone(), l.clone())?;
            metamodelica::arrayUpdate(ass2.clone(), l.clone(), c.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r: i32 = 0;
            r = ({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt});
            metamodelica::arrayUpdate(ass1.clone(), c.clone(), l.clone())?;
            metamodelica::arrayUpdate(ass2.clone(), l.clone(), c.clone())?;
            BFSBreasign(i.clone(), ({let __elt = parentcolum.borrow()[(r.clone()-1) as usize].clone(); __elt}), parentcolum.clone(), r.clone(), ass1.clone(), ass2.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("function BFSBreasign failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn BFSBenque(mut queue: Arc<metamodelica::List<i32>>, mut rowmark: i32, mut c: i32, mut rc: i32, mut r: i32, mut visited: bool, mut rowmarks: metamodelica::Array<i32>, mut parentcolum: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outEqnqueue: Arc<metamodelica::List<i32>>;
    outEqnqueue = (match visited.clone() {
        false => queue.clone(),
        true => {
            metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), rowmark.clone())?;
            metamodelica::arrayUpdate(parentcolum.clone(), r.clone(), c.clone())?;
            metamodelica::cons(rc.clone(), queue.clone())
        },
        _ => {
            Error::addInternalError((literal!("function BFSBenque failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
    });
    Ok(outEqnqueue)
}

pub fn DFSB(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = DFSB1(1, 1, nvars.clone(), neqns.clone(), m.clone(), mt.clone(), rowmarks.clone(), vec1.clone(), vec2.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.BFSB failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn DFSB1(mut i: i32, mut rowmark: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
            Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut visitedcolums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut mt1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let false = (intGt(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            visitedcolums = DFSBphase(list![i.clone()], rowmark.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), list![i.clone()])?;
            let (__pa0, __pa3, __pa1, __pa2, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(reduceIndexifNecessary(visitedcolums.clone(), i.clone(), isyst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?) {
                (_, __pa0, __pa3 @ Deref @ BackendDAE::EqSystem { m: Some(__pa1), mT: Some(__pa2), .. }, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i_1 = __pa0.clone();
            m1 = __pa1.clone();
            mt1 = __pa2.clone();
            syst = __pa3.clone();
            shared = __pa4.clone();
            nv_1 = __pa5.clone();
            ne_1 = __pa6.clone();
            ass1_1 = __pa7.clone();
            ass2_1 = __pa8.clone();
            arg = __pa9.clone();
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1.clone(), metamodelica::arrayLength(rowmarks.clone()), -1)?;
            (ass1_2, ass2_2, syst, shared, arg) = DFSB1(i_1.clone(), rowmark.clone() + 1, nv_1.clone(), ne_1.clone(), m1.clone(), mt1.clone(), rowmarks1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone())?;
            Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let true = (intGt(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            (ass1_1, ass2_1, syst, shared, arg) = DFSB1(i.clone() + 1, rowmark.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            Ok((ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function DFSB1 failed in equation ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn DFSBphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            inVisitedColums.clone()
        },
        _ => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            DFSBtraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function DFSBphase failed in equation ")); __mm_s.push_str(&*intString(c.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVisitedColums)
}

fn DFSBtraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inVisitedColums.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    DFSBreasign(stack.clone(), r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut visitedColums: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rc: i32 = 0;
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    visitedColums = DFSBphase(metamodelica::cons(rc.clone(), stack.clone()), i.clone(), rc.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(rc.clone(), inVisitedColums.clone()))?;
                    Ok(DFSBtraverseRows1(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), visitedColums.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(DFSBtraverseRows(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function DFSBtraverseRows failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVisitedColums)
}

fn DFSBtraverseRows1(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = (::match_deref::match_deref! { match &(inVisitedColums.clone()) {
        Deref @ metamodelica::List::Nil => inVisitedColums.clone(),
        _ => DFSBtraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVisitedColums)
}

fn DFSBreasign(mut stack: Arc<metamodelica::List<i32>>, mut r: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
            let mut rc: i32 = 0;
            rc = ({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt});
            metamodelica::arrayUpdate(ass1.clone(), c.clone(), r.clone())?;
            metamodelica::arrayUpdate(ass2.clone(), r.clone(), c.clone())?;
            DFSBreasign(rest.clone(), rc.clone(), ass1.clone(), ass2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn MC21A(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut lookahead: metamodelica::Array<i32> = Default::default();
                    let mut syst = (*syst).clone();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    lookahead = arrayCreate(neqns.clone(), 0);
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = MC21A1(1, 1, nvars.clone(), neqns.clone(), m.clone(), mt.clone(), rowmarks.clone(), lookahead.clone(), vec1.clone(), vec2.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.MC21A failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn MC21A1(mut i: i32, mut rowmark: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
            Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut visitedcolums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut changedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut mt1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut lookahead1: metamodelica::Array<i32> = Default::default();
            let false = (intGt(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            visitedcolums = MC21Aphase(list![i.clone()], rowmark.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), list![i.clone()])?;
            let (__pa0, __pa1, __pa4, __pa2, __pa3, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(reduceIndexifNecessary(visitedcolums.clone(), i.clone(), isyst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?) {
                (__pa0, __pa1, __pa4 @ Deref @ BackendDAE::EqSystem { m: Some(__pa2), mT: Some(__pa3), .. }, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) => (__pa0.clone(), __pa1.clone(), __pa4.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
                _ => bail!("pattern mismatch"),
            } };
            changedEqns = __pa0.clone();
            i_1 = __pa1.clone();
            m1 = __pa2.clone();
            mt1 = __pa3.clone();
            syst = __pa4.clone();
            shared = __pa5.clone();
            nv_1 = __pa6.clone();
            ne_1 = __pa7.clone();
            ass1_1 = __pa8.clone();
            ass2_1 = __pa9.clone();
            arg = __pa10.clone();
            (rowmarks1, lookahead1) = MC21A1fixArrays(visitedcolums.clone(), nv_1.clone(), ne_1.clone(), rowmarks.clone(), lookahead.clone(), changedEqns.clone())?;
            (ass1_2, ass2_2, syst, shared, arg) = MC21A1(i_1.clone(), rowmark.clone() + 1, nv_1.clone(), ne_1.clone(), m1.clone(), mt1.clone(), rowmarks1.clone(), lookahead1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone())?;
            Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let true = (intGt(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            (ass1_1, ass2_1, syst, shared, arg) = MC21A1(i.clone() + 1, rowmark.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            Ok((ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function MC21A1 failed in equation ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn MC21A1fixArrays(mut meqns: Arc<metamodelica::List<i32>>, mut nv: i32, mut ne: i32, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut changedEqns: Arc<metamodelica::List<i32>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outlookahead: metamodelica::Array<i32>;
    (outrowmarks, outlookahead) = (::match_deref::match_deref! { match &(meqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            (rowmarks.clone(), lookahead.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
            let mut memsize: i32 = 0;
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut lookahead1: metamodelica::Array<i32> = Default::default();
            memsize = metamodelica::arrayLength(rowmarks.clone());
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv.clone(), memsize.clone(), -1)?;
            lookahead1 = assignmentsArrayExpand(lookahead.clone(), ne.clone(), memsize.clone(), 0)?;
            MC21A1fixArray(changedEqns.clone(), lookahead1.clone())?;
            (rowmarks1.clone(), lookahead1.clone())
        },
        _ => {
            Error::addInternalError((literal!("function MC21A1fixArrays failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outrowmarks, outlookahead))
}

fn MC21A1fixArray(mut meqns: Arc<metamodelica::List<i32>>, mut arr: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(meqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
            metamodelica::arrayUpdate(arr.clone(), e.clone(), 0)?;
            MC21A1fixArray(rest.clone(), arr.clone())?;
            ()
        },
        _ => {
            Error::addInternalError((literal!("function MC21A1fixArray failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn MC21Aphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            inVisitedColums.clone()
        },
        _ => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = lookahead.borrow()[(c.clone()-1) as usize].clone(); __elt}), (rows.clone().len() as i32));
            MC21Achecklookahead(b.clone(), rows.clone(), stack.clone(), i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function MC21Aphase failed in equation ")); __mm_s.push_str(&*intString(c.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVisitedColums)
}

fn MC21Achecklookahead(mut dolookahaed: bool, mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = (match dolookahaed.clone() {
        true => MC21AtraverseRowsUnmatched(rows.clone(), rows.clone(), stack.clone(), i.clone(), c.clone(), (rows.clone().len() as i32), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?,
        _ => MC21AtraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?,
    });
    Ok(outVisitedColums)
}

fn MC21AtraverseRowsUnmatched(mut rows: Arc<metamodelica::List<i32>>, mut rows1: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut l: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::arrayUpdate(lookahead.clone(), c.clone(), l.clone())?;
                    Ok(MC21AtraverseRows(rows1.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    DFSBreasign(stack.clone(), r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(MC21AtraverseRowsUnmatched(rest.clone(), rows1.clone(), stack.clone(), i.clone(), c.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVisitedColums)
}

fn MC21AtraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inVisitedColums.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut visitedColums: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rc: i32 = 0;
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    visitedColums = MC21Aphase(metamodelica::cons(rc.clone(), stack.clone()), i.clone(), rc.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(rc.clone(), inVisitedColums.clone()))?;
                    Ok(MC21AtraverseRows1(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), visitedColums.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(MC21AtraverseRows(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function MC21AtraverseRows failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVisitedColums)
}

fn MC21AtraverseRows1(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inVisitedColums: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVisitedColums: Arc<metamodelica::List<i32>>;
    outVisitedColums = (::match_deref::match_deref! { match &(inVisitedColums.clone()) {
        Deref @ metamodelica::List::Nil => inVisitedColums.clone(),
        _ => MC21AtraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVisitedColums)
}

pub fn PF(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut lookahead: metamodelica::Array<i32> = Default::default();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    lookahead = arrayCreate(neqns.clone(), 0);
                    unmatched = cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = PF1(0, unmatched.clone(), rowmarks.clone(), lookahead.clone(), isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.PF failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn PF1(mut i: i32, mut unmatched: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        },
        (_, Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut lookahead1: metamodelica::Array<i32> = Default::default();
            (i_1, unmatched1) = PFaugmentmatching(i.clone(), unmatched.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), (unmatched.clone().len() as i32), metamodelica::nil())?;
            meqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            (unmatched1, rowmarks1, lookahead1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = PF2(meqns.clone(), unmatched1.clone(), metamodelica::nil(), rowmarks.clone(), lookahead.clone(), isyst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            { (i, unmatched, rowmarks, lookahead, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1.clone() + 1, unmatched1.clone(), rowmarks1.clone(), lookahead1.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn PF2(mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unmatched: Arc<metamodelica::List<i32>>, mut changedEqns: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outunmatched: Arc<metamodelica::List<i32>>;
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outlookahead: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outunmatched, outrowmarks, outlookahead, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (unmatched.clone(), rowmarks.clone(), lookahead.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone())
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut lookahead1: metamodelica::Array<i32> = Default::default();
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            ne_1 = BackendDAEUtil::systemSize(syst.clone())?;
            nv_1 = BackendVariable::daenumVariables(syst.clone());
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1.clone(), ne.clone(), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1.clone(), nv.clone(), -1)?;
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1.clone(), nv.clone(), -1)?;
            lookahead1 = assignmentsArrayExpand(lookahead.clone(), ne_1.clone(), ne.clone(), 0)?;
            MC21A1fixArray(unmatched1.clone(), lookahead1.clone())?;
            (unmatched1.clone(), rowmarks1.clone(), lookahead1.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone())
        },
        (_, _) => {
            singularSystemError(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outunmatched, outrowmarks, outlookahead, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg))
}

fn PFaugmentmatching(mut i: i32, mut U: Arc<metamodelica::List<i32>>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut previousUnmatched: i32, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut outI: i32;
    let mut outUnmatched: Arc<metamodelica::List<i32>>;
    (outI, outUnmatched) = 'mc: {
        let __mc_input = U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let true = (intEq(previousUnmatched.clone(), (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok((i.clone(), unMatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    (i_1, unmatched) = PFaugmentmatching(i.clone() + 1, unMatched.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), (unMatched.clone().len() as i32), metamodelica::nil())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    let true = (intGt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), -1)) else { bail!("pattern mismatch") };
                    (i_1, unmatched) = PFaugmentmatching(i.clone(), rest.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched.clone(), unMatched.clone())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    let mut b: bool = false;
                    b = PFphase(list![c.clone()], i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?;
                    unmatched = List::consOnTrue(!(b.clone()), c.clone(), unMatched.clone());
                    (i_1, unmatched) = PFaugmentmatching(i.clone(), rest.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched.clone(), unmatched.clone())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function PFaugmentmatching failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outI, outUnmatched))
}

fn PFphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        _ => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = lookahead.borrow()[(c.clone()-1) as usize].clone(); __elt}), (rows.clone().len() as i32));
            PFchecklookahead(b.clone(), rows.clone(), stack.clone(), i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function PFphase failed in equation ")); __mm_s.push_str(&*intString(c.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn PFchecklookahead(mut dolookahaed: bool, mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<bool> {
    let mut matched: bool;
    matched = (match dolookahaed.clone() {
        true => PFtraverseRowsUnmatched(rows.clone(), rows.clone(), stack.clone(), i.clone(), c.clone(), (rows.clone().len() as i32), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?,
        _ => PFtraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?,
    });
    Ok(matched)
}

fn PFtraverseRowsUnmatched(mut rows: Arc<metamodelica::List<i32>>, mut rows1: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut l: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::arrayUpdate(lookahead.clone(), c.clone(), l.clone())?;
                    Ok(PFtraverseRows(rows1.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    DFSBreasign(stack.clone(), r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(PFtraverseRowsUnmatched(rest.clone(), rows1.clone(), stack.clone(), i.clone(), c.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matched)
}

fn PFtraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut rc: i32 = 0;
                    let mut b: bool = false;
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    b = PFphase(metamodelica::cons(rc.clone(), stack.clone()), i.clone(), rc.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?;
                    Ok(PFtraverseRows1(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), b.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(PFtraverseRows(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function PFtraverseRows failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matched)
}

fn PFtraverseRows1(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatched: bool) -> Result<bool> {
    let mut matched: bool;
    matched = (match inMatched.clone() {
        true => inMatched.clone(),
        _ => PFtraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?,
    });
    Ok(matched)
}

pub fn PFPlus(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut lookahead: metamodelica::Array<i32> = Default::default();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    lookahead = arrayCreate(neqns.clone(), 0);
                    unmatched = cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (_, vec1, vec2, syst, shared, arg) = PFPlus1(0, unmatched.clone(), rowmarks.clone(), lookahead.clone(), isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.PFPlus failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn PFPlus1(mut i: i32, mut unmatched: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((i.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        },
        (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut lookahead1: metamodelica::Array<i32> = Default::default();
            let mut syst = (*syst).clone();
            (i_1, unmatched1) = PFPlusaugmentmatching(i.clone(), unmatched.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), (unmatched.clone().len() as i32), metamodelica::nil(), false)?;
            meqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            (unmatched1, rowmarks1, lookahead1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = PF2(meqns.clone(), unmatched1.clone(), metamodelica::nil(), rowmarks.clone(), lookahead.clone(), syst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            { (i, unmatched, rowmarks, lookahead, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1.clone() + 1, unmatched1.clone(), rowmarks1.clone(), lookahead1.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn PFPlusaugmentmatching(mut i: i32, mut U: Arc<metamodelica::List<i32>>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut previousUnmatched: i32, mut unMatched: Arc<metamodelica::List<i32>>, mut reverseRows: bool) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut outI: i32;
    let mut outUnMatched: Arc<metamodelica::List<i32>>;
    (outI, outUnMatched) = 'mc: {
        let __mc_input = U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let true = (intEq(previousUnmatched.clone(), (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok((i.clone(), unMatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    (i_1, unmatched) = PFPlusaugmentmatching(i.clone() + 1, unMatched.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), (unMatched.clone().len() as i32), metamodelica::nil(), reverseRows.clone())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    let true = (intGt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), -1)) else { bail!("pattern mismatch") };
                    (i_1, unmatched) = PFPlusaugmentmatching(i.clone(), rest.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched.clone(), unMatched.clone(), reverseRows.clone())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    let mut b: bool = false;
                    b = PFPlusphase(list![c.clone()], i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?;
                    unmatched = List::consOnTrue(!(b.clone()), c.clone(), unMatched.clone());
                    (i_1, unmatched) = PFPlusaugmentmatching(i.clone(), rest.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched.clone(), unmatched.clone(), !(reverseRows.clone()))?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function PFPlusaugmentmatching failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outI, outUnMatched))
}

fn PFPlusphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut reverseRows: bool) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match &((stack.clone(), reverseRows.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            false
        },
        (_, false) => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = lookahead.borrow()[(c.clone()-1) as usize].clone(); __elt}), (rows.clone().len() as i32));
            PFPluschecklookahead(b.clone(), rows.clone(), stack.clone(), i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?
        },
        (_, true) => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = lookahead.borrow()[(c.clone()-1) as usize].clone(); __elt}), (rows.clone().len() as i32));
            PFPluschecklookahead(b.clone(), rows.clone().reverse(), stack.clone(), i.clone(), c.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function PFPlusphase failed in equation ")); __mm_s.push_str(&*intString(c.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn PFPluschecklookahead(mut dolookahaed: bool, mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut reverseRows: bool) -> Result<bool> {
    let mut matched: bool;
    matched = (match dolookahaed.clone() {
        true => PFPlustraverseRowsUnmatched(rows.clone(), rows.clone(), stack.clone(), i.clone(), c.clone(), (rows.clone().len() as i32), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?,
        _ => PFPlustraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?,
    });
    Ok(matched)
}

fn PFPlustraverseRowsUnmatched(mut rows: Arc<metamodelica::List<i32>>, mut rows1: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut c: i32, mut l: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut reverseRows: bool) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::arrayUpdate(lookahead.clone(), c.clone(), l.clone())?;
                    Ok(PFPlustraverseRows(rows1.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    DFSBreasign(stack.clone(), r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(PFPlustraverseRowsUnmatched(rest.clone(), rows1.clone(), stack.clone(), i.clone(), c.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matched)
}

fn PFPlustraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut reverseRows: bool) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut rc: i32 = 0;
                    let mut b: bool = false;
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    b = PFPlusphase(metamodelica::cons(rc.clone(), stack.clone()), i.clone(), rc.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?;
                    Ok(PFPlustraverseRows1(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), b.clone(), reverseRows.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(PFPlustraverseRows(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function PFPlustraverseRows failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matched)
}

fn PFPlustraverseRows1(mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut lookahead: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatched: bool, mut reverseRows: bool) -> Result<bool> {
    let mut matched: bool;
    matched = (match inMatched.clone() {
        true => inMatched.clone(),
        _ => PFPlustraverseRows(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows.clone())?,
    });
    Ok(matched)
}

pub fn HK(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut level: metamodelica::Array<i32> = Default::default();
                    let mut collummarks: metamodelica::Array<i32> = Default::default();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    collummarks = arrayCreate(neqns.clone(), -1);
                    level = arrayCreate(neqns.clone(), -1);
                    unmatched = cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = HK1(0, unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.HK failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn HK1(mut i: i32, mut unmatched: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        },
        (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut collummarks1: metamodelica::Array<i32> = Default::default();
            let mut level1: metamodelica::Array<i32> = Default::default();
            let mut syst = (*syst).clone();
            (i_1, unmatched1) = HKphase(i.clone(), unmatched.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), (unmatched.clone().len() as i32), metamodelica::nil())?;
            meqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            (unmatched1, rowmarks1, collummarks1, level1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = HK2(meqns.clone(), unmatched1.clone(), metamodelica::nil(), rowmarks.clone(), collummarks.clone(), level.clone(), syst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            { (i, unmatched, rowmarks, collummarks, level, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1.clone() + 1, unmatched1.clone(), rowmarks1.clone(), collummarks1.clone(), level1.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn HK2(mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unmatched: Arc<metamodelica::List<i32>>, mut changedEqns: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outunmatched: Arc<metamodelica::List<i32>>;
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outcollummarks: metamodelica::Array<i32>;
    let mut outlevel: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outunmatched, outrowmarks, outcollummarks, outlevel, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone())
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut collummarks1: metamodelica::Array<i32> = Default::default();
            let mut level1: metamodelica::Array<i32> = Default::default();
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            ne_1 = BackendDAEUtil::systemSize(syst.clone())?;
            nv_1 = BackendVariable::daenumVariables(syst.clone());
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1.clone(), metamodelica::arrayLength(ass1.clone()), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1.clone(), metamodelica::arrayLength(ass2.clone()), -1)?;
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1.clone(), metamodelica::arrayLength(rowmarks.clone()), -1)?;
            collummarks1 = assignmentsArrayExpand(collummarks.clone(), ne_1.clone(), metamodelica::arrayLength(collummarks.clone()), -1)?;
            level1 = assignmentsArrayExpand(level.clone(), ne_1.clone(), metamodelica::arrayLength(level.clone()), -1)?;
            (unmatched1.clone(), rowmarks1.clone(), collummarks1.clone(), level1.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone())
        },
        (_, _) => {
            singularSystemError(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outunmatched, outrowmarks, outcollummarks, outlevel, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg))
}

fn HKphase(mut i: i32, mut U: Arc<metamodelica::List<i32>>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut previousUnmatched: i32, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut outI: i32;
    let mut outunMatched: Arc<metamodelica::List<i32>>;
    (outI, outunMatched) = 'mc: {
        let __mc_input = U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let true = (intEq(previousUnmatched.clone(), (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok((i.clone(), unMatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    (i_1, unmatched) = HKphase(i.clone() + 1, unMatched.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), (unMatched.clone().len() as i32), metamodelica::nil())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    rows = HKBFS(U.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), i.clone(), level.clone(), None, ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    HKDFS(rows.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    unmatched = HKgetUnmatched(U.clone(), ass1.clone(), metamodelica::nil())?;
                    (i_1, unmatched) = HKphase(i.clone(), metamodelica::nil(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), previousUnmatched.clone(), unmatched.clone())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function HKphase failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outI, outunMatched))
}

fn HKgetUnmatched(mut U: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut inUnmatched: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnmatched: Arc<metamodelica::List<i32>>;
    outUnmatched = 'mc: {
        let __mc_input = U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inUnmatched.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let true = (intGt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    Ok(HKgetUnmatched(rest.clone(), ass1.clone(), inUnmatched.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    Ok(HKgetUnmatched(rest.clone(), ass1.clone(), metamodelica::cons(c.clone(), inUnmatched.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outUnmatched)
}

fn HKBFS(mut colums: Arc<metamodelica::List<i32>>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut i: i32, mut level: metamodelica::Array<i32>, mut lowestL: Option<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inRows: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(colums.clone()) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inRows.clone())
        },
        Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
            let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut ll: Option<i32> = None;
            (rows, ll) = HKBFSBphase(list![c.clone()], i.clone(), 0, lowestL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), metamodelica::nil())?;
            { (colums, nv, ne, m, mT, rowmarks, i, level, lowestL, ass1, ass2, inRows) = (rest.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), i.clone(), level.clone(), ll.clone(), ass1.clone(), ass2.clone(), rows.clone()); continue '__tco; }
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKBFS failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn HKBFSBphase(mut queue: Arc<metamodelica::List<i32>>, mut i: i32, mut l: i32, mut lowestL: Option<i32>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inRows: Arc<metamodelica::List<(i32, i32)>>, mut queue1: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<(i32, i32)>>, Option<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((queue.clone(), lowestL.clone(), queue1.clone())) {
        (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil) => {
            return Ok((inRows.clone(), lowestL.clone()))
        },
        (Deref @ metamodelica::List::Nil, Some(lowl), _) => {
            let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut l_1: i32 = 0;
            let mut b: bool = false;
            let mut ll: Option<i32> = None;
            l_1 = l.clone() + 1;
            b = intGt(l_1.clone(), lowl.clone());
            return Ok(HKBFSBphase1(b.clone(), queue1.clone(), i.clone(), l_1.clone(), lowestL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), metamodelica::nil())?)
        },
        (Deref @ metamodelica::List::Nil, None, _) => {
            let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut ll: Option<i32> = None;
            { (queue, i, l, lowestL, nv, ne, m, mT, rowmarks, level, ass1, ass2, inRows, queue1) = (queue1.clone(), i.clone(), l.clone() + 1, lowestL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), metamodelica::nil()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _, _) => {
            let mut queue2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut cr: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut b: bool = false;
            let mut ll: Option<i32> = None;
            cr = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            metamodelica::arrayUpdate(level.clone(), c.clone(), l.clone())?;
            (queue2, rows, b) = HKBFStraverseRows(cr.clone(), metamodelica::nil(), i.clone(), l.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), false)?;
            queue2 = listAppend(queue1.clone(), queue2.clone());
            ll = if (b.clone()) {Some(l.clone())} else {lowestL.clone()};
            { (queue, i, l, lowestL, nv, ne, m, mT, rowmarks, level, ass1, ass2, inRows, queue1) = (rest.clone(), i.clone(), l.clone(), ll.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), rows.clone(), queue2.clone()); continue '__tco; }
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKBFSBphase failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn HKBFSBphase1(mut inUnMaRowFound: bool, mut queue: Arc<metamodelica::List<i32>>, mut i: i32, mut l: i32, mut lowestL: Option<i32>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inRows: Arc<metamodelica::List<(i32, i32)>>, mut queue1: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<(i32, i32)>>, Option<i32>)> {
    let mut outRows: Arc<metamodelica::List<(i32, i32)>>;
    let mut outlowestL: Option<i32>;
    (outRows, outlowestL) = (match inUnMaRowFound.clone() {
        true => {
            (inRows.clone(), Some(l.clone()))
        },
        false => {
            let mut ll: Option<i32> = None;
            let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            (rows, ll) = HKBFSBphase(queue.clone(), i.clone(), l.clone(), lowestL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), queue1.clone())?;
            (rows.clone(), ll.clone())
        },
        _ => {
            Error::addInternalError((literal!("function HKBFSBphase1 failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
    });
    Ok((outRows, outlowestL))
}

fn HKBFStraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut queue: Arc<metamodelica::List<i32>>, mut i: i32, mut l: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inRows: Arc<metamodelica::List<(i32, i32)>>, mut inunmarowFound: bool) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, bool)> {
    let mut outEqnqueue: Arc<metamodelica::List<i32>>;
    let mut outRows: Arc<metamodelica::List<(i32, i32)>>;
    let mut unmarowFound: bool;
    (outEqnqueue, outRows, unmarowFound) = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((queue.clone().reverse(), inRows.clone(), inunmarowFound.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rowstpl: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut b: bool = false;
                    let false = (intLt(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    (queue1, rowstpl, b) = HKBFStraverseRows(rest.clone(), queue.clone(), i.clone(), l.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), inunmarowFound.clone())?;
                    Ok((queue1.clone(), rowstpl.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rowstpl: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut b: bool = false;
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    (queue1, rowstpl, b) = HKBFStraverseRows(rest.clone(), queue.clone(), i.clone(), l.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::cons((r.clone(), l.clone()), inRows.clone()), true)?;
                    Ok((queue1.clone(), rowstpl.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rowstpl: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut rc: i32 = 0;
                    let mut b: bool = false;
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    (queue1, rowstpl, b) = HKBFStraverseRows(rest.clone(), metamodelica::cons(rc.clone(), queue.clone()), i.clone(), l.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows.clone(), inunmarowFound.clone())?;
                    Ok((queue1.clone(), rowstpl.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKBFStraverseRows failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqnqueue, outRows, unmarowFound))
}

fn HKDFS(mut unmatchedRows: Arc<metamodelica::List<(i32, i32)>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inUnmatchedRows: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(unmatchedRows.clone()) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inUnmatchedRows.clone())
        },
        Deref @ metamodelica::List::Cons { head: (r, l), tail: rest } => {
            let mut ur: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b: bool = false;
            b = HKDFSphase(list![r.clone()], i.clone(), r.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), false)?;
            ur = List::consOnTrue(!(b.clone()), r.clone(), inUnmatchedRows.clone());
            { (unmatchedRows, i, nv, ne, m, mT, collummarks, level, ass1, ass2, inUnmatchedRows) = (rest.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ur.clone()); continue '__tco; }
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDFS failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn HKDFSphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut r: i32, mut l: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatched: bool) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            inMatched.clone()
        },
        _ => {
            let mut collums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            collums = List::select(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            HKDFStraverseCollums(collums.clone(), stack.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDFSphase failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn HKDFStraverseCollums(mut collums: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut l: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatched: bool) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = collums.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inMatched.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let false = (intEq(({let __elt = level.borrow()[(c.clone()-1) as usize].clone(); __elt}), l.clone())) else { bail!("pattern mismatch") };
                    Ok(HKDFStraverseCollums(rest.clone(), stack.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: _ } => {
                    let true = (intEq(({let __elt = level.borrow()[(c.clone()-1) as usize].clone(); __elt}), l.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = collummarks.borrow()[(c.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    HKDFSreasign(stack.clone(), c.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut r: i32 = 0;
                    let mut b: bool = false;
                    let true = (intEq(({let __elt = level.borrow()[(c.clone()-1) as usize].clone(); __elt}), l.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = collummarks.borrow()[(c.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    r = ({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(r.clone(), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(collummarks.clone(), c.clone(), i.clone())?;
                    b = HKDFSphase(metamodelica::cons(r.clone(), stack.clone()), i.clone(), r.clone(), l.clone() - 1, nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?;
                    Ok(HKDFStraverseCollums1(b.clone(), rest.clone(), stack.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let true = (intEq(({let __elt = level.borrow()[(c.clone()-1) as usize].clone(); __elt}), l.clone())) else { bail!("pattern mismatch") };
                    let false = (intLt(({let __elt = collummarks.borrow()[(c.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    Ok(HKDFStraverseCollums(rest.clone(), stack.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function HKDFStraverseCollums failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matched)
}

fn HKDFStraverseCollums1(mut inMatched: bool, mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut l: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<bool> {
    let mut matched: bool;
    matched = (match inMatched.clone() {
        true => inMatched.clone(),
        _ => HKDFStraverseCollums(rows.clone(), stack.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?,
    });
    Ok(matched)
}

fn HKDFSreasign(mut stack: Arc<metamodelica::List<i32>>, mut c: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
            let mut cr: i32 = 0;
            cr = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
            metamodelica::arrayUpdate(ass1.clone(), c.clone(), r.clone())?;
            metamodelica::arrayUpdate(ass2.clone(), r.clone(), c.clone())?;
            HKDFSreasign(rest.clone(), cr.clone(), ass1.clone(), ass2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn HKDW(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut level: metamodelica::Array<i32> = Default::default();
                    let mut collummarks: metamodelica::Array<i32> = Default::default();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    collummarks = arrayCreate(neqns.clone(), -1);
                    level = arrayCreate(neqns.clone(), -1);
                    unmatched = cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = HKDW1(0, unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.HKDW failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn HKDW1(mut i: i32, mut unmatched: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        },
        (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut collummarks1: metamodelica::Array<i32> = Default::default();
            let mut level1: metamodelica::Array<i32> = Default::default();
            let mut syst = (*syst).clone();
            (i_1, unmatched1) = HKDWphase(i.clone(), unmatched.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), (unmatched.clone().len() as i32), metamodelica::nil())?;
            meqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            (unmatched1, rowmarks1, collummarks1, level1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = HK2(meqns.clone(), unmatched1.clone(), metamodelica::nil(), rowmarks.clone(), collummarks.clone(), level.clone(), syst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            { (i, unmatched, rowmarks, collummarks, level, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1.clone() + 1, unmatched1.clone(), rowmarks1.clone(), collummarks1.clone(), level1.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn HKDWphase(mut i: i32, mut U: Arc<metamodelica::List<i32>>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut previousUnmatched: i32, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut outI: i32;
    let mut outunMatched: Arc<metamodelica::List<i32>>;
    (outI, outunMatched) = 'mc: {
        let __mc_input = U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let true = (intEq(previousUnmatched.clone(), (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok((i.clone(), unMatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    (i_1, unmatched) = HKphase(i.clone() + 1, unMatched.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), (unMatched.clone().len() as i32), metamodelica::nil())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rows: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut ur: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    rows = HKBFS(U.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), i.clone(), level.clone(), None, ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    ur = HKDFS(rows.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    HKDWDFS(ur.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone())?;
                    unmatched = HKgetUnmatched(U.clone(), ass1.clone(), metamodelica::nil())?;
                    (i_1, unmatched) = HKphase(i.clone(), metamodelica::nil(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), previousUnmatched.clone(), unmatched.clone())?;
                    Ok((i_1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function HKDWphase failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outI, outunMatched))
}

fn HKDWDFS(mut unmatchedRows: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(unmatchedRows.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
            HKDWDFSphase(list![r.clone()], i.clone(), r.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), false)?;
            HKDWDFS(rest.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone())?;
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDWDFS failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn HKDWDFSphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut r: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatched: bool) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            inMatched.clone()
        },
        _ => {
            let mut collums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            collums = List::select(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            HKDWDFStraverseCollums(collums.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDWDFSphase failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn HKDWDFStraverseCollums(mut collums: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatched: bool) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = collums.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inMatched.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: _ } => {
                    let true = (intLt(({let __elt = collummarks.borrow()[(c.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    HKDFSreasign(stack.clone(), c.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut r: i32 = 0;
                    let mut b: bool = false;
                    let true = (intLt(({let __elt = collummarks.borrow()[(c.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    r = ({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(r.clone(), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(collummarks.clone(), c.clone(), i.clone())?;
                    b = HKDWDFSphase(metamodelica::cons(r.clone(), stack.clone()), i.clone(), r.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?;
                    Ok(HKDWDFStraverseCollums1(b.clone(), rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let false = (intLt(({let __elt = collummarks.borrow()[(c.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    Ok(HKDWDFStraverseCollums(rest.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function HKDWDFStraverseCollums failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matched)
}

fn HKDWDFStraverseCollums1(mut inMatched: bool, mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut collummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<bool> {
    let mut matched: bool;
    matched = (match inMatched.clone() {
        true => inMatched.clone(),
        _ => HKDWDFStraverseCollums(rows.clone(), stack.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched.clone())?,
    });
    Ok(matched)
}

pub fn ABMP(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut rowmarks: metamodelica::Array<i32> = Default::default();
                    let mut level: metamodelica::Array<i32> = Default::default();
                    let mut collummarks: metamodelica::Array<i32> = Default::default();
                    let mut rlevel: metamodelica::Array<i32> = Default::default();
                    let mut colptrs: metamodelica::Array<i32> = Default::default();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    rowmarks = arrayCreate(nvars.clone(), -1);
                    collummarks = arrayCreate(neqns.clone(), -1);
                    level = arrayCreate(neqns.clone(), -1);
                    rlevel = arrayCreate(nvars.clone(), nvars.clone());
                    colptrs = arrayCreate(neqns.clone(), -1);
                    unmatched = cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = ABMP1(1, unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), rlevel.clone(), colptrs.clone(), isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.ABMP failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn ABMP1(mut i: i32, mut unmatched: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut rlevel: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        },
        (_, Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut lim: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut collummarks1: metamodelica::Array<i32> = Default::default();
            let mut level1: metamodelica::Array<i32> = Default::default();
            let mut rlevel1: metamodelica::Array<i32> = Default::default();
            lim = ((metamodelica::OrderedFloat(0.1_f64) * (metamodelica::OrderedFloat((metamodelica::arrayLength(ass1.clone())) as f64)).sqrt()).0.floor() as i32);
            unmatched1 = ABMPphase(unmatched.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), rowmarks.clone(), rlevel.clone(), colptrs.clone(), lim.clone(), ass1.clone(), ass2.clone())?;
            (i_1, unmatched1) = HKphase(i.clone() + 1, unmatched.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), (unmatched.clone().len() as i32), metamodelica::nil())?;
            meqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            (unmatched1, rowmarks1, collummarks1, level1, rlevel1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = ABMP2(meqns.clone(), unmatched1.clone(), metamodelica::nil(), rowmarks.clone(), collummarks.clone(), level.clone(), rlevel.clone(), isyst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            { (i, unmatched, rowmarks, collummarks, level, rlevel, colptrs, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1.clone() + 1, unmatched1.clone(), rowmarks1.clone(), collummarks1.clone(), level1.clone(), rlevel1.clone(), colptrs.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn ABMP2(mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unmatched: Arc<metamodelica::List<i32>>, mut changedEqns: Arc<metamodelica::List<i32>>, mut rowmarks: metamodelica::Array<i32>, mut collummarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut rlevel: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outunmatched: Arc<metamodelica::List<i32>>;
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outcollummarks: metamodelica::Array<i32>;
    let mut outlevel: metamodelica::Array<i32>;
    let mut outrlevel: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outunmatched, outrowmarks, outcollummarks, outlevel, outrlevel, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), rlevel.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone())
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut rowmarks1: metamodelica::Array<i32> = Default::default();
            let mut collummarks1: metamodelica::Array<i32> = Default::default();
            let mut level1: metamodelica::Array<i32> = Default::default();
            let mut rlevel1: metamodelica::Array<i32> = Default::default();
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            ne_1 = BackendDAEUtil::systemSize(syst.clone())?;
            nv_1 = BackendVariable::daenumVariables(syst.clone());
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1.clone(), metamodelica::arrayLength(ass1_1.clone()), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1.clone(), metamodelica::arrayLength(ass2_1.clone()), -1)?;
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1.clone(), metamodelica::arrayLength(rowmarks.clone()), -1)?;
            collummarks1 = assignmentsArrayExpand(collummarks.clone(), ne_1.clone(), metamodelica::arrayLength(collummarks.clone()), -1)?;
            rlevel1 = arrayCreate(metamodelica::arrayLength(ass2_1.clone()), metamodelica::arrayLength(ass2_1.clone()));
            level1 = assignmentsArrayExpand(level.clone(), ne_1.clone(), metamodelica::arrayLength(level.clone()), -1)?;
            (unmatched1.clone(), rowmarks1.clone(), collummarks1.clone(), level1.clone(), rlevel1.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone())
        },
        (_, _) => {
            singularSystemError(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outunmatched, outrowmarks, outcollummarks, outlevel, outrlevel, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg))
}

fn ABMPphase(mut U: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut lim: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut unMatched: Arc<metamodelica::List<i32>>;
    unMatched = (::match_deref::match_deref! { match &(U.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        _ => {
            let mut ur: Arc<metamodelica::List<i32>> = metamodelica::nil();
            ur = ABMPBFSphase(U.clone(), i.clone(), 0, lim.clone(), (U.clone().len() as i32), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), metamodelica::nil())?;
            ABMPphase1(U.clone(), ur.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), colptrs.clone(), lim.clone(), ass1.clone(), ass2.clone())?
        },
        _ => {
            Error::addInternalError((literal!("function ABMPphase failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(unMatched)
}

fn ABMPphase1(mut U: Arc<metamodelica::List<i32>>, mut unmatchedRows: Arc<metamodelica::List<i32>>, mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut lim: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut unMatched: Arc<metamodelica::List<i32>>;
    unMatched = (::match_deref::match_deref! { match &(unmatchedRows.clone()) {
        Deref @ metamodelica::List::Nil => {
            U.clone()
        },
        Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
            let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut L: i32 = 0;
            L = ({let __elt = level.borrow()[(r.clone()-1) as usize].clone(); __elt});
            ABMPDFS(unmatchedRows.clone(), 0, L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
            unmatched = HKgetUnmatched(U.clone(), ass1.clone(), metamodelica::nil())?;
            ABMPphase2(unmatched.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), colptrs.clone(), lim.clone(), ass1.clone(), ass2.clone())?
        },
        _ => {
            Error::addInternalError((literal!("function ABMPphase1 failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(unMatched)
}

fn ABMPphase2(mut U: Arc<metamodelica::List<i32>>, mut i: i32, mut L: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut lim: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut unMatched: Arc<metamodelica::List<i32>>;
    unMatched = 'mc: {
        let __mc_input = U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(U.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(50 * L.clone(), (U.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(U.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ABMPphase(U.clone(), i.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), colptrs.clone(), lim.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ABMPphase2 failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(unMatched)
}

fn ABMPBFSphase(mut queue: Arc<metamodelica::List<i32>>, mut i: i32, mut L: i32, mut lim: i32, mut lim1: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextqueue: Arc<metamodelica::List<i32>>, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((queue.clone(), nextqueue.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(unMatched.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            let mut l: i32 = 0;
            let mut b: bool = false;
            l = L.clone() + 2;
            b = intGt(l.clone(), lim.clone()) || intGt(50 * l.clone(), lim1.clone());
            return Ok(ABMPBFSphase1(b.clone(), nextqueue.clone(), i.clone(), l.clone(), lim.clone(), lim1.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), unMatched.clone())?)
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _) => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
            rows = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (queue1, unmatched) = ABMPBFStraverseRows(rows.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), nextqueue.clone(), unMatched.clone())?;
            { (queue, i, L, lim, lim1, nv, ne, m, mT, rowmarks, level, ass1, ass2, nextqueue, unMatched) = (rest.clone(), i.clone(), L.clone(), lim.clone(), lim1.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), queue1.clone(), unmatched.clone()); continue '__tco; }
        },
        _ => {
            Error::addInternalError((literal!("function ABMPBFSphase failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn ABMPBFSphase1(mut inStop: bool, mut queue: Arc<metamodelica::List<i32>>, mut i: i32, mut L: i32, mut lim: i32, mut lim1: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextqueue: Arc<metamodelica::List<i32>>, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outunMatched: Arc<metamodelica::List<i32>>;
    outunMatched = (match inStop.clone() {
        true => unMatched.clone(),
        false => ABMPBFSphase(queue.clone(), i.clone(), L.clone(), lim.clone(), lim1.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), nextqueue.clone(), unMatched.clone())?,
        _ => {
            Error::addInternalError((literal!("function ABMPBFSphase1 failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
    });
    Ok(outunMatched)
}

fn ABMPBFStraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut i: i32, mut L: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowmarks: metamodelica::Array<i32>, mut level: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut queue: Arc<metamodelica::List<i32>>, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outEqnqueue: Arc<metamodelica::List<i32>>;
    let mut outUnmatched: Arc<metamodelica::List<i32>>;
    (outEqnqueue, outUnmatched) = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((queue.clone().reverse(), unMatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(level.clone(), r.clone(), L.clone())?;
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    (queue1, unmatched) = ABMPBFStraverseRows(rest.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), queue.clone(), metamodelica::cons(r.clone(), unMatched.clone()))?;
                    Ok((queue1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rc: i32 = 0;
                    let false = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intLt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i.clone())?;
                    (queue1, unmatched) = ABMPBFStraverseRows(rest.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(rc.clone(), queue.clone()), unMatched.clone())?;
                    Ok((queue1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), i.clone())) else { bail!("pattern mismatch") };
                    (queue1, unmatched) = ABMPBFStraverseRows(rest.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), queue.clone(), unMatched.clone())?;
                    Ok((queue1.clone(), unmatched.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ABMPBFStraverseRows failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqnqueue, outUnmatched))
}

fn ABMPDFS(mut unmatchedRows: Arc<metamodelica::List<i32>>, mut i: i32, mut L: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut unMatched: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = unmatchedRows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (intLt(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i_1: i32 = 0;
                    let mut b: bool = false;
                    metamodelica::arrayUpdate(colptrs.clone(), r.clone(), 0)?;
                    (i_1, b) = ABMPDFSphase(list![r.clone()], i.clone(), r.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    unmatched = List::consOnTrue(!(b.clone()), r.clone(), unMatched.clone());
                    ABMPDFS1(b.clone(), r.clone(), rest.clone(), unmatched.clone(), i_1.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ABMPBFS failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn ABMPDFS1(mut inMatched: bool, mut r: i32, mut unmatchedRows: Arc<metamodelica::List<i32>>, mut unMatched: Arc<metamodelica::List<i32>>, mut i: i32, mut L: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inMatched.clone(), unmatchedRows.clone(), unMatched.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _) => {
                    let true = (intGt(50 * L.clone(), (unmatchedRows.clone().len() as i32) + (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, Deref @ metamodelica::List::Nil) => {
                    let false = (intGt(50 * L.clone(), (unmatchedRows.clone().len() as i32) + (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    ABMPDFS(unmatchedRows.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::List::Cons { head: r1, tail: _ }, Deref @ metamodelica::List::Cons { head: r2, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut l: i32 = 0;
                    let false = (intGt(50 * L.clone(), (unmatchedRows.clone().len() as i32) + (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let false = (intEq(L.clone(), ({let __elt = level.borrow()[(r1.clone()-1) as usize].clone(); __elt}))) else { bail!("pattern mismatch") };
                    l = ({let __elt = level.borrow()[(r2.clone()-1) as usize].clone(); __elt});
                    ABMPDFS(metamodelica::cons(r2.clone(), unmatchedRows.clone()), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::List::Cons { head: r1, tail: _ }, _) => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r2: i32 = 0;
                    let mut l: i32 = 0;
                    let false = (intGt(50 * L.clone(), (unmatchedRows.clone().len() as i32) + (unMatched.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let false = (intEq(L.clone(), ({let __elt = level.borrow()[(r1.clone()-1) as usize].clone(); __elt}))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unMatched.clone().reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    unmatched = __pa1.clone();
                    l = ({let __elt = level.borrow()[(r2.clone()-1) as usize].clone(); __elt});
                    unmatched = listAppend(unmatched.clone(), metamodelica::cons(r2.clone(), unmatchedRows.clone()));
                    ABMPDFS(unmatchedRows.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: r1, tail: _ }, Deref @ metamodelica::List::Nil) => {
                    let mut l: i32 = 0;
                    let false = (intEq(L.clone(), ({let __elt = level.borrow()[(r1.clone()-1) as usize].clone(); __elt}))) else { bail!("pattern mismatch") };
                    l = ({let __elt = level.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    ABMPDFS(unmatchedRows.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: r1, tail: _ }, _) => {
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r2: i32 = 0;
                    let mut l: i32 = 0;
                    let false = (intEq(L.clone(), ({let __elt = level.borrow()[(r1.clone()-1) as usize].clone(); __elt}))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unMatched.clone().reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    unmatched = __pa1.clone();
                    l = ({let __elt = level.borrow()[(r2.clone()-1) as usize].clone(); __elt});
                    unmatched = listAppend(metamodelica::cons(r2.clone(), unmatched.clone()), unmatchedRows.clone());
                    ABMPDFS(unmatched.clone(), i.clone(), l.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: r1, tail: _ }, _) => {
                    let true = (intEq(L.clone(), ({let __elt = level.borrow()[(r1.clone()-1) as usize].clone(); __elt}))) else { bail!("pattern mismatch") };
                    ABMPDFS(unmatchedRows.clone(), i.clone(), L.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), unMatched.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ABMPBFS1 failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn ABMPDFSphase(mut stack: Arc<metamodelica::List<i32>>, mut i: i32, mut r: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(i32, bool)> {
    let mut outI: i32;
    let mut matched: bool;
    (outI, matched) = (::match_deref::match_deref! { match &(stack.clone()) {
        Deref @ metamodelica::List::Nil => {
            (i.clone(), false)
        },
        _ => {
            let mut collums: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut desL: i32 = 0;
            let mut i_1: i32 = 0;
            let mut b: bool = false;
            collums = List::select(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            collums = List::stripN(collums.clone(), ({let __elt = colptrs.borrow()[(r.clone()-1) as usize].clone(); __elt}))?;
            desL = ({let __elt = level.borrow()[(r.clone()-1) as usize].clone(); __elt}) - 2;
            (i_1, b) = ABMPDFStraverseCollums(collums.clone(), 1, stack.clone(), r.clone(), i.clone(), desL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
            (i_1.clone(), b.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ABMPDFSphase failed in phase ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outI, matched))
}

fn ABMPDFStraverseCollums(mut collums: Arc<metamodelica::List<i32>>, mut counter: i32, mut stack: Arc<metamodelica::List<i32>>, mut r: i32, mut i: i32, mut desL: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(i32, bool)> {
    let mut outI: i32;
    let mut matched: bool;
    (outI, matched) = 'mc: {
        let __mc_input = collums.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::arrayUpdate(level.clone(), r.clone(), ({let __elt = level.borrow()[(r.clone()-1) as usize].clone(); __elt}) + 2)?;
                    metamodelica::arrayUpdate(colptrs.clone(), r.clone(), 0)?;
                    Ok((i.clone() + 1, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: _ } => {
                    let true = (intLt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(colptrs.clone(), r.clone(), counter.clone())?;
                    HKDFSreasign(stack.clone(), c.clone(), ass1.clone(), ass2.clone())?;
                    Ok((i.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut rc: i32 = 0;
                    let mut i_1: i32 = 0;
                    let mut b: bool = false;
                    let true = (intEq(({let __elt = level.borrow()[(c.clone()-1) as usize].clone(); __elt}), desL.clone())) else { bail!("pattern mismatch") };
                    rc = ({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt});
                    let true = (intGt(rc.clone(), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(colptrs.clone(), r.clone(), counter.clone())?;
                    (i_1, b) = ABMPDFSphase(metamodelica::cons(rc.clone(), stack.clone()), i.clone(), rc.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    (i_1, b) = ABMPDFStraverseCollums1(b.clone(), counter.clone() + 1, rest.clone(), stack.clone(), r.clone(), i_1.clone(), desL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    Ok((i_1.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut i_1: i32 = 0;
                    let mut b: bool = false;
                    (i_1, b) = ABMPDFStraverseCollums(rest.clone(), counter.clone() + 1, stack.clone(), r.clone(), i.clone(), desL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    Ok((i_1.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ABMPDFSBtraverseCollums failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outI, matched))
}

fn ABMPDFStraverseCollums1(mut inMatched: bool, mut counter: i32, mut rows: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut r: i32, mut i: i32, mut desL: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut level: metamodelica::Array<i32>, mut colptrs: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(i32, bool)> {
    let mut outI: i32;
    let mut matched: bool;
    (outI, matched) = (match (inMatched.clone(), i.clone()) {
        (true, mut i_1) => {
            (i_1.clone(), true)
        },
        _ => {
            let mut i_1: i32 = 0;
            let mut b: bool = false;
            (i_1, b) = ABMPDFStraverseCollums(rows.clone(), counter.clone(), stack.clone(), r.clone(), i.clone(), desL.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
            (i_1.clone(), b.clone())
        },
    });
    Ok((outI, matched))
}

pub fn PR_FIFO_FAIR(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut l_label: metamodelica::Array<i32> = Default::default();
                    let mut r_label: metamodelica::Array<i32> = Default::default();
                    let mut unmatched: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
                    l_label = arrayCreate(neqns.clone(), -1);
                    r_label = arrayCreate(nvars.clone(), -1);
                    unmatched = cheapmatchingalgorithm(nvars.clone(), neqns.clone(), m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = PR_FIFO_FAIR1(unmatched.clone(), l_label.clone(), r_label.clone(), isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32 = 0;
                    let mut neqns: i32 = 0;
                    let mut vec1: metamodelica::Array<i32> = Default::default();
                    let mut vec2: metamodelica::Array<i32> = Default::default();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
                    nvars = BackendVariable::daenumVariables(isyst.clone());
                    let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
                    let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("- Matching.PR_FIFO_FAIR failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn PR_FIFO_FAIR1(mut unmatched: Arc<metamodelica::List<i32>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = (unmatched.clone(), isyst.clone(), inArg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut nv_1: i32 = 0;
                    let mut ne_1: i32 = 0;
                    let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    let mut ass1_1: metamodelica::Array<i32> = Default::default();
                    let mut ass1_2: metamodelica::Array<i32> = Default::default();
                    let mut ass2_1: metamodelica::Array<i32> = Default::default();
                    let mut ass2_2: metamodelica::Array<i32> = Default::default();
                    let mut l_label1: metamodelica::Array<i32> = Default::default();
                    let mut r_label1: metamodelica::Array<i32> = Default::default();
                    let mut syst = (*syst).clone();
                    PR_Global_Relabel(l_label.clone(), r_label.clone(), nv.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone())?;
                    PR_FIFO_FAIRphase(0, unmatched.clone(), nv.clone() + ne.clone(), -1, nv.clone(), ne.clone(), m.clone(), mt.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    unmatched1 = getUnassigned(ne.clone(), ass1.clone(), metamodelica::nil());
                    meqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
                    (unmatched1, l_label1, r_label1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = PR_FIFO_FAIR2(meqns.clone(), unmatched1.clone(), metamodelica::nil(), l_label.clone(), r_label.clone(), syst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
                    (ass1_2, ass2_2, syst, shared, arg1) = PR_FIFO_FAIR1(unmatched1.clone(), l_label1.clone(), r_label1.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone())?;
                    Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (_, _, _, mapIncRowEqn, _)) => {
                    let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqn_str: ArcStr = arcstr::literal!("");
                    let mut var_str: ArcStr = arcstr::literal!("");
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    unmatched1 = List::map1r(unmatched.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mapIncRowEqn.clone())?;
                    unmatched1 = List::uniqueIntN(unmatched1.clone(), metamodelica::arrayLength(mapIncRowEqn.clone()))?;
                    eqn_str = (BackendDump::dumpMarkedEqns(isyst.clone(), unmatched1.clone())?).clone();
                    unmatched1 = getUnassigned(nv.clone(), ass2.clone(), metamodelica::nil());
                    var_str = (BackendDump::dumpMarkedVars(isyst.clone(), unmatched1.clone())?).clone();
                    source = BackendEquation::markedEquationSource(isyst.clone(), listHead(unmatched1.clone())?)?;
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Error::addSourceMessage(Error::STRUCT_SINGULAR_SYSTEM.clone(), list![(eqn_str.clone()).clone(), (var_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn PR_FIFO_FAIR2(mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unmatched: Arc<metamodelica::List<i32>>, mut changedEqns: Arc<metamodelica::List<i32>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outunmatched: Arc<metamodelica::List<i32>>;
    let mut outl_label: metamodelica::Array<i32>;
    let mut outr_label: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outunmatched, outl_label, outr_label, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (unmatched.clone(), l_label.clone(), r_label.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone())
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut unmatched1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut l_label1: metamodelica::Array<i32> = Default::default();
            let mut r_label1: metamodelica::Array<i32> = Default::default();
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            ne_1 = BackendDAEUtil::systemSize(syst.clone())?;
            nv_1 = BackendVariable::daenumVariables(syst.clone());
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1.clone(), metamodelica::arrayLength(ass1_1.clone()), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1.clone(), metamodelica::arrayLength(ass2_1.clone()), -1)?;
            l_label1 = assignmentsArrayExpand(l_label.clone(), ne_1.clone(), metamodelica::arrayLength(l_label.clone()), -1)?;
            r_label1 = assignmentsArrayExpand(r_label.clone(), nv_1.clone(), metamodelica::arrayLength(r_label.clone()), -1)?;
            (unmatched1.clone(), l_label1.clone(), r_label1.clone(), nv_1.clone(), ne_1.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), shared.clone(), arg.clone())
        },
        (_, _) => {
            singularSystemError(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outunmatched, outl_label, outr_label, nvars, neqns, outAss1, outAss2, osyst, oshared, outArg))
}

fn PR_Global_Relabel(mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let mut queue: Arc<metamodelica::List<i32>>;
    let mut max: i32;
    max = nv.clone() + ne.clone();
    PR_Global_Relabel_init_l_label(1, ne.clone(), max.clone(), l_label.clone())?;
    queue = PR_Global_Relabel_init_r_label(1, nv.clone(), max.clone(), r_label.clone(), ass2.clone(), metamodelica::nil())?;
    PR_Global_Relabel1(queue.clone(), l_label.clone(), r_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
    Ok(())
}

fn PR_Global_Relabel_init_l_label(mut i: i32, mut ne: i32, mut max: i32, mut l_label: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = l_label.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::arrayUpdate(l_label.clone(), i.clone(), max.clone())?;
            PR_Global_Relabel_init_l_label(i.clone() + 1, ne.clone(), max.clone(), l_label.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn PR_Global_Relabel_init_r_label(mut i: i32, mut nv: i32, mut max: i32, mut r_label: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inQueue: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outQueue: Arc<metamodelica::List<i32>>;
    outQueue = 'mc: {
        let __mc_input = inQueue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(i.clone(), nv.clone())) else { bail!("pattern mismatch") };
                    Ok(inQueue.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (intGt(i.clone(), nv.clone())) else { bail!("pattern mismatch") };
                    let false = (intGt(({let __elt = ass2.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(r_label.clone(), i.clone(), 0)?;
                    Ok(PR_Global_Relabel_init_r_label(i.clone() + 1, nv.clone(), max.clone(), r_label.clone(), ass2.clone(), metamodelica::cons(i.clone(), inQueue.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::arrayUpdate(r_label.clone(), i.clone(), max.clone())?;
                    Ok(PR_Global_Relabel_init_r_label(i.clone() + 1, nv.clone(), max.clone(), r_label.clone(), ass2.clone(), inQueue.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outQueue)
}

fn PR_Global_Relabel1(mut queue: Arc<metamodelica::List<i32>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut max: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextqueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (queue.clone(), nextqueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    PR_Global_Relabel1(nextqueue.clone().reverse(), l_label.clone(), r_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _) => {
                    let mut collums: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    collums = List::select(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    queue1 = PR_Global_Relabel_traverseCollums(collums.clone(), max.clone(), r.clone(), l_label.clone(), r_label.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), nextqueue.clone())?;
                    PR_Global_Relabel1(rest.clone(), l_label.clone(), r_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), queue1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn PR_Global_Relabel_traverseCollums(mut collums: Arc<metamodelica::List<i32>>, mut max: i32, mut r: i32, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextqueue: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outQueue: Arc<metamodelica::List<i32>>;
    outQueue = 'mc: {
        let __mc_input = collums.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(nextqueue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut rc: i32 = 0;
                    let true = (intEq(({let __elt = l_label.borrow()[(c.clone()-1) as usize].clone(); __elt}), max.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(l_label.clone(), c.clone(), ({let __elt = r_label.borrow()[(r.clone()-1) as usize].clone(); __elt}) + 1)?;
                    rc = ({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt});
                    let true = (intGt(rc.clone(), -1)) else { bail!("pattern mismatch") };
                    let true = (intEq(({let __elt = r_label.borrow()[(rc.clone()-1) as usize].clone(); __elt}), max.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(r_label.clone(), rc.clone(), ({let __elt = l_label.borrow()[(c.clone()-1) as usize].clone(); __elt}) + 1)?;
                    Ok(PR_Global_Relabel_traverseCollums(rest.clone(), max.clone(), r.clone(), l_label.clone(), r_label.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(rc.clone(), nextqueue.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(PR_Global_Relabel_traverseCollums(rest.clone(), max.clone(), r.clone(), l_label.clone(), r_label.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), nextqueue.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function PR_Global_Relabel_traverseCollums failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outQueue)
}

fn PR_FIFO_FAIRphase(mut relabels: i32, mut U: Arc<metamodelica::List<i32>>, mut max: i32, mut min_vertex: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut nextqueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (U.clone(), nextqueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    PR_FIFO_FAIRphase(relabels.clone(), nextqueue.clone(), max.clone(), min_vertex.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (intEq(relabels.clone(), max.clone())) else { bail!("pattern mismatch") };
                    PR_Global_Relabel(l_label.clone(), r_label.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
                    PR_FIFO_FAIRphase(0, U.clone(), max.clone(), min_vertex.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), nextqueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _) => {
                    let mut queue: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut min_label: i32 = 0;
                    let mut rlcount: i32 = 0;
                    let mut minvertex: i32 = 0;
                    (rlcount, min_label, minvertex) = PR_FIFO_FAIRphase1(intLt(({let __elt = l_label.borrow()[(c.clone()-1) as usize].clone(); __elt}), max.clone()), relabels.clone() + 1, c.clone(), min_vertex.clone(), max.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
                    queue = PR_FIFO_FAIRrelabel(c.clone(), minvertex.clone(), min_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), nextqueue.clone())?;
                    PR_FIFO_FAIRphase(rlcount.clone(), rest.clone(), max.clone(), minvertex.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), queue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn PR_FIFO_FAIRphase1(mut b: bool, mut relabels: i32, mut max_vertex: i32, mut min_vertec: i32, mut min_label: i32, mut max: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(i32, i32, i32)> {
    let mut outRelabels: i32;
    let mut outMinLabels: i32;
    let mut outMinVertex: i32;
    (outRelabels, outMinLabels, outMinVertex) = (match b.clone() {
        true => {
            let mut rel: i32 = 0;
            let mut minlab: i32 = 0;
            let mut minvert: i32 = 0;
            let mut tmp: i32 = 0;
            tmp = intMod(({let __elt = l_label.borrow()[(max_vertex.clone()-1) as usize].clone(); __elt}), 4);
            (rel, minlab, minvert) = PR_FIFO_FAIRphase2(intEq(tmp.clone(), 1), relabels.clone(), max_vertex.clone(), min_vertec.clone(), min_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
            (rel.clone(), minlab.clone(), minvert.clone())
        },
        _ => {
            (relabels.clone(), min_label.clone(), min_vertec.clone())
        },
    });
    Ok((outRelabels, outMinLabels, outMinVertex))
}

fn PR_FIFO_FAIRphase2(mut b: bool, mut relabels: i32, mut max_vertex: i32, mut min_vertec: i32, mut min_label: i32, mut max: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(i32, i32, i32)> {
    let mut outRelabels: i32;
    let mut outMinLabels: i32;
    let mut outMinVertex: i32;
    (outRelabels, outMinLabels, outMinVertex) = (match b.clone() {
        true => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rel: i32 = 0;
            let mut minlab: i32 = 0;
            let mut minvert: i32 = 0;
            rows = List::select(({let __elt = m.borrow()[(max_vertex.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (rel, minlab, minvert) = PR_FIFO_FAIRphase_traverseRows(rows.clone(), relabels.clone(), max_vertex.clone(), min_vertec.clone(), min_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
            (rel.clone(), minlab.clone(), minvert.clone())
        },
        _ => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rel: i32 = 0;
            let mut minlab: i32 = 0;
            let mut minvert: i32 = 0;
            rows = List::select(({let __elt = m.borrow()[(max_vertex.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            rows = rows.clone().reverse();
            (rel, minlab, minvert) = PR_FIFO_FAIRphase_traverseRows(rows.clone(), relabels.clone(), max_vertex.clone(), min_vertec.clone(), min_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
            (rel.clone(), minlab.clone(), minvert.clone())
        },
    });
    Ok((outRelabels, outMinLabels, outMinVertex))
}

fn PR_FIFO_FAIRphase_traverseRows(mut rows: Arc<metamodelica::List<i32>>, mut relabels: i32, mut max_vertex: i32, mut min_vertex: i32, mut min_label: i32, mut max: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(i32, i32, i32)> {
    let mut outRelabels: i32;
    let mut outMinLabels: i32;
    let mut outMinVertex: i32;
    (outRelabels, outMinLabels, outMinVertex) = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((relabels.clone(), min_label.clone(), min_vertex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let mut minlabel: i32 = 0;
                    let mut minvertex: i32 = 0;
                    let true = (intLt(({let __elt = r_label.borrow()[(r.clone()-1) as usize].clone(); __elt}), min_label.clone())) else { bail!("pattern mismatch") };
                    minlabel = ({let __elt = r_label.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    minvertex = r.clone();
                    let true = (intEq(({let __elt = r_label.borrow()[(minvertex.clone()-1) as usize].clone(); __elt}), ({let __elt = l_label.borrow()[(max_vertex.clone()-1) as usize].clone(); __elt}) - 1)) else { bail!("pattern mismatch") };
                    Ok((relabels.clone() - 1, minlabel.clone(), minvertex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut minlabel: i32 = 0;
                    let mut minvertex: i32 = 0;
                    let mut rel: i32 = 0;
                    let true = (intLt(({let __elt = r_label.borrow()[(r.clone()-1) as usize].clone(); __elt}), min_label.clone())) else { bail!("pattern mismatch") };
                    minlabel = ({let __elt = r_label.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    minvertex = r.clone();
                    let false = (intEq(({let __elt = r_label.borrow()[(minvertex.clone()-1) as usize].clone(); __elt}), ({let __elt = l_label.borrow()[(max_vertex.clone()-1) as usize].clone(); __elt}) - 1)) else { bail!("pattern mismatch") };
                    (rel, minlabel, minvertex) = PR_FIFO_FAIRphase_traverseRows(rest.clone(), relabels.clone(), max_vertex.clone(), minvertex.clone(), minlabel.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
                    Ok((rel.clone(), minlabel.clone(), minvertex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut minlabel: i32 = 0;
                    let mut minvertex: i32 = 0;
                    let mut rel: i32 = 0;
                    (rel, minlabel, minvertex) = PR_FIFO_FAIRphase_traverseRows(rest.clone(), relabels.clone(), max_vertex.clone(), min_vertex.clone(), min_label.clone(), max.clone(), nv.clone(), ne.clone(), m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
                    Ok((rel.clone(), minlabel.clone(), minvertex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function PR_FIFO_FAIRphase_traverseRows failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRelabels, outMinLabels, outMinVertex))
}

fn PR_FIFO_FAIRrelabel(mut max_vertex: i32, mut min_vertex: i32, mut min_label: i32, mut max: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut l_label: metamodelica::Array<i32>, mut r_label: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inQueue: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outQueue: Arc<metamodelica::List<i32>>;
    outQueue = 'mc: {
        let __mc_input = inQueue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intLt(min_label.clone(), max.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass2.borrow()[(min_vertex.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(ass2.clone(), min_vertex.clone(), max_vertex.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), max_vertex.clone(), min_vertex.clone())?;
                    metamodelica::arrayUpdate(r_label.clone(), min_vertex.clone(), min_label.clone() + 2)?;
                    Ok(inQueue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut next_vertex: i32 = 0;
                    let true = (intLt(min_label.clone(), max.clone())) else { bail!("pattern mismatch") };
                    let false = (intLt(({let __elt = ass2.borrow()[(min_vertex.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    next_vertex = ({let __elt = ass2.borrow()[(min_vertex.clone()-1) as usize].clone(); __elt});
                    metamodelica::arrayUpdate(ass2.clone(), min_vertex.clone(), max_vertex.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), max_vertex.clone(), min_vertex.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), next_vertex.clone(), -1)?;
                    metamodelica::arrayUpdate(l_label.clone(), max_vertex.clone(), min_label.clone() + 1)?;
                    metamodelica::arrayUpdate(r_label.clone(), min_vertex.clone(), min_label.clone() + 2)?;
                    Ok(metamodelica::cons(next_vertex.clone(), inQueue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inQueue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outQueue)
}

// =============================================================================
// cheap matching implementations
//
// =============================================================================
fn cheapmatchingalgorithm(mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut intRangeUsed: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnMatched: Arc<metamodelica::List<i32>>;
    outUnMatched = cheapmatchingalgorithm1(Config::getCheapMatchingAlgorithm()?, nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), intRangeUsed.clone())?;
    Ok(outUnMatched)
}

fn cheapmatchingalgorithm1(mut algorithmid: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut intRangeUsed: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnMatched: Arc<metamodelica::List<i32>>;
    outUnMatched = (match (algorithmid.clone(), intRangeUsed.clone()) {
        (1, _) => cheapmatching(1, nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?,
        (3, _) => ks_rand_cheapmatching(nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone())?,
        (_, true) => getUnassigned(ne.clone(), ass1.clone(), metamodelica::nil()),
        _ => metamodelica::nil(),
    });
    Ok(outUnMatched)
}

fn cheapmatching(mut i: i32, mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inUnMatched: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnMatched: Arc<metamodelica::List<i32>>;
    outUnMatched = 'mc: {
        let __mc_input = inUnMatched.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
                    Ok(inUnMatched.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    rows = List::select(({let __elt = m.borrow()[(i.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    cheapmatching1(rows.clone(), i.clone(), ass1.clone(), ass2.clone())?;
                    Ok(cheapmatching(i.clone() + 1, nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), inUnMatched.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cheapmatching(i.clone() + 1, nv.clone(), ne.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(i.clone(), inUnMatched.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function cheapmatching failed in equation ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outUnMatched)
}

fn cheapmatching1(mut rows: Arc<metamodelica::List<i32>>, mut c: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(ass1.clone(), c.clone(), r.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), r.clone(), c.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    cheapmatching1(rest.clone(), c.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn ks_rand_cheapmatching(mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnMatched: Arc<metamodelica::List<i32>>;
    let mut onecolums: Arc<metamodelica::List<i32>>;
    let mut onerows: Arc<metamodelica::List<i32>>;
    let mut col_degrees: metamodelica::Array<i32>;
    let mut row_degrees: metamodelica::Array<i32>;
    let mut randarr: metamodelica::Array<i32>;
    col_degrees = arrayCreate(ne.clone(), 0);
    row_degrees = arrayCreate(ne.clone(), 0);
    onerows = getOneRows(ne.clone(), mT.clone(), row_degrees.clone(), metamodelica::nil())?;
    onecolums = getOneRows(nv.clone(), m.clone(), col_degrees.clone(), metamodelica::nil())?;
    randarr = Array::createIntRange(ne.clone());
    setrandArray(ne.clone(), randarr.clone())?;
    ks_rand_cheapmatching1(1, ne.clone(), onecolums.clone(), onerows.clone(), col_degrees.clone(), row_degrees.clone(), randarr.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
    outUnMatched = getUnassigned(ne.clone(), ass1.clone(), metamodelica::nil());
    Ok(outUnMatched)
}

fn ks_rand_cheapmatching1(mut i: i32, mut ne: i32, mut onecolums: Arc<metamodelica::List<i32>>, mut onerows: Arc<metamodelica::List<i32>>, mut col_degrees: metamodelica::Array<i32>, mut row_degrees: metamodelica::Array<i32>, mut randarr: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = ass2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intLe(i.clone(), ne.clone())) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut onecolums1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut onerows1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut c: i32 = 0;
            let mut b: bool = false;
            ks_rand_match(onerows.clone(), onecolums.clone(), row_degrees.clone(), col_degrees.clone(), mT.clone(), m.clone(), ass2.clone(), ass1.clone())?;
            c = ({let __elt = randarr.borrow()[(i.clone()-1) as usize].clone(); __elt});
            b = intLt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0) && intGt(({let __elt = col_degrees.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0);
            (onecolums1, onerows1) = ks_rand_cheapmatching2(b.clone(), c.clone(), col_degrees.clone(), row_degrees.clone(), randarr.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
            ks_rand_cheapmatching1(i.clone() + 1, ne.clone(), onecolums1.clone(), onerows1.clone(), col_degrees.clone(), row_degrees.clone(), randarr.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn ks_rand_cheapmatching2(mut b: bool, mut c: i32, mut col_degrees: metamodelica::Array<i32>, mut row_degrees: metamodelica::Array<i32>, mut randarr: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut onecolums: Arc<metamodelica::List<i32>>;
    let mut onerows: Arc<metamodelica::List<i32>>;
    (onecolums, onerows) = (match b.clone() {
        true => {
            let mut clst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut e_id: i32 = 0;
            let mut r: i32 = 0;
            e_id = ((realMod(System::realRand(), intReal(({let __elt = col_degrees.borrow()[(c.clone()-1) as usize].clone(); __elt})))).0.floor() as i32);
            lst = List::select(({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (rlst, r) = ks_rand_cheapmatching3(e_id.clone(), lst.clone(), row_degrees.clone(), c.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), 0)?;
            lst = List::select(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            clst = ks_rand_cheapmatching4(lst.clone(), ({let __elt = row_degrees.borrow()[(r.clone()-1) as usize].clone(); __elt}), col_degrees.clone(), ass1.clone(), metamodelica::nil())?;
            (clst.clone(), rlst.clone())
        },
        _ => {
            (metamodelica::nil(), metamodelica::nil())
        },
    });
    Ok((onecolums, onerows))
}

fn ks_rand_cheapmatching3(mut e_id: i32, mut rows: Arc<metamodelica::List<i32>>, mut row_degrees: metamodelica::Array<i32>, mut c: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut onerows: Arc<metamodelica::List<i32>>, mut inR: i32) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut outonerows: Arc<metamodelica::List<i32>>;
    let mut outR: i32;
    (outonerows, outR) = 'mc: {
        let __mc_input = rows.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((onerows.clone(), inR.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    let true = (intEq(e_id.clone(), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(ass1.clone(), c.clone(), r.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), r.clone(), c.clone())?;
                    stack = ks_rand_match_degree(rest.clone(), row_degrees.clone(), ass2.clone(), onerows.clone())?;
                    Ok((stack.clone(), r.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut statck1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r_1: i32 = 0;
                    let true = (intLt(({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(row_degrees.clone(), r.clone(), ({let __elt = row_degrees.borrow()[(r.clone()-1) as usize].clone(); __elt}) - 1)?;
                    stack = List::consOnTrue(intEq(({let __elt = row_degrees.borrow()[(r.clone()-1) as usize].clone(); __elt}), 1), r.clone(), onerows.clone());
                    (statck1, r_1) = ks_rand_cheapmatching3(e_id.clone() - 1, rest.clone(), row_degrees.clone(), c.clone(), ass1.clone(), ass2.clone(), stack.clone(), r.clone())?;
                    Ok((statck1.clone(), r_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut statck1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r_1: i32 = 0;
                    (statck1, r_1) = ks_rand_cheapmatching3(e_id.clone() - 1, rest.clone(), row_degrees.clone(), c.clone(), ass1.clone(), ass2.clone(), onerows.clone(), r.clone())?;
                    Ok((statck1.clone(), r_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outonerows, outR))
}

fn ks_rand_cheapmatching4(mut cols: Arc<metamodelica::List<i32>>, mut count: i32, mut col_degrees: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut inStack: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outStack: Arc<metamodelica::List<i32>>;
    outStack = 'mc: {
        let __mc_input = cols.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inStack.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (intGt(count.clone(), 0)) else { bail!("pattern mismatch") };
                    Ok(inStack.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intLt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(col_degrees.clone(), c.clone(), ({let __elt = col_degrees.borrow()[(c.clone()-1) as usize].clone(); __elt}) - 1)?;
                    stack = List::consOnTrue(intEq(({let __elt = col_degrees.borrow()[(c.clone()-1) as usize].clone(); __elt}), 1), c.clone(), inStack.clone());
                    Ok(ks_rand_cheapmatching4(rest.clone(), count.clone() - 1, col_degrees.clone(), ass1.clone(), stack.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(ks_rand_cheapmatching4(rest.clone(), count.clone(), col_degrees.clone(), ass1.clone(), inStack.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStack)
}

fn getOneRows(mut n: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut degrees: metamodelica::Array<i32>, mut inOneRows: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        match n.clone() {
        0 => {
            return Ok(inOneRows.clone().reverse())
        },
        _ => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut onerows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut l: i32 = 0;
            lst = List::select(({let __elt = m.borrow()[(n.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            l = (lst.clone().len() as i32);
            metamodelica::arrayUpdate(degrees.clone(), n.clone(), l.clone())?;
            onerows = List::consOnTrue(intEq(l.clone(), 1), n.clone(), inOneRows.clone());
            { (n, m, degrees, inOneRows) = (n.clone() - 1, m.clone(), degrees.clone(), onerows.clone()); continue '__tco; }
        },
    }
    }
}

fn setrandArray(mut n: i32, mut randarr: metamodelica::Array<i32>) -> Result<()> {
    let () = (match n.clone() {
        0 => {
            ()
        },
        _ => {
            let mut z: i32 = 0;
            let mut tmp: i32 = 0;
            z = ((realMod(System::realRand(), intReal(n.clone()))).0.floor() as i32) + 1;
            tmp = ({let __elt = randarr.borrow()[(n.clone()-1) as usize].clone(); __elt});
            metamodelica::arrayUpdate(randarr.clone(), n.clone(), ({let __elt = randarr.borrow()[(z.clone()-1) as usize].clone(); __elt}))?;
            metamodelica::arrayUpdate(randarr.clone(), z.clone(), tmp.clone())?;
            setrandArray(n.clone() - 1, randarr.clone())?;
            ()
        },
    });
    Ok(())
}

fn ks_rand_match(mut stack1: Arc<metamodelica::List<i32>>, mut stack2: Arc<metamodelica::List<i32>>, mut degrees1: metamodelica::Array<i32>, mut degrees2: metamodelica::Array<i32>, mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (stack1.clone(), stack2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, Deref @ metamodelica::List::Nil) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intEq(({let __elt = degrees1.borrow()[(e.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass1.borrow()[(e.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    lst = List::select(({let __elt = m1.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    stack = ks_rand_match1(e.clone(), lst.clone(), rest.clone(), degrees1.clone(), degrees2.clone(), m2.clone(), ass1.clone(), ass2.clone())?;
                    ks_rand_match(stack.clone(), metamodelica::nil(), degrees1.clone(), degrees2.clone(), m1.clone(), m2.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, Deref @ metamodelica::List::Nil) => {
                    ks_rand_match(rest.clone(), metamodelica::nil(), degrees1.clone(), degrees2.clone(), m1.clone(), m2.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: e, tail: rest }) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intEq(({let __elt = degrees2.borrow()[(e.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    lst = List::select(({let __elt = m2.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    stack = ks_rand_match1(e.clone(), lst.clone(), rest.clone(), degrees2.clone(), degrees1.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    ks_rand_match(stack.clone(), metamodelica::nil(), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    ks_rand_match(rest.clone(), metamodelica::nil(), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intEq(({let __elt = degrees1.borrow()[(e.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    let true = (intLt(({let __elt = ass1.borrow()[(e.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    lst = List::select(({let __elt = m1.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    stack = ks_rand_match1(e.clone(), lst.clone(), rest.clone(), degrees1.clone(), degrees2.clone(), m2.clone(), ass1.clone(), ass2.clone())?;
                    ks_rand_match(stack2.clone(), stack.clone(), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    ks_rand_match(stack2.clone(), rest.clone(), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn ks_rand_match1(mut i: i32, mut entries: Arc<metamodelica::List<i32>>, mut stack: Arc<metamodelica::List<i32>>, mut degrees1: metamodelica::Array<i32>, mut degrees2: metamodelica::Array<i32>, mut adjacency: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outStack: Arc<metamodelica::List<i32>>;
    outStack = 'mc: {
        let __mc_input = entries.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(stack.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: _ } => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intLt(({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    lst = List::select(({let __elt = adjacency.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    metamodelica::arrayUpdate(ass1.clone(), i.clone(), e.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), e.clone(), i.clone())?;
                    Ok(ks_rand_match_degree(lst.clone(), degrees1.clone(), ass1.clone(), stack.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(ks_rand_match1(i.clone(), rest.clone(), stack.clone(), degrees1.clone(), degrees2.clone(), adjacency.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStack)
}

fn ks_rand_match_degree(mut entries: Arc<metamodelica::List<i32>>, mut degrees: metamodelica::Array<i32>, mut ass: metamodelica::Array<i32>, mut inStack: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outStack: Arc<metamodelica::List<i32>>;
    outStack = 'mc: {
        let __mc_input = entries.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inStack.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
                    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intLt(({let __elt = ass.borrow()[(e.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(degrees.clone(), e.clone(), ({let __elt = degrees.borrow()[(e.clone()-1) as usize].clone(); __elt}) - 1)?;
                    stack = List::consOnTrue(intEq(({let __elt = degrees.borrow()[(e.clone()-1) as usize].clone(); __elt}), 1), e.clone(), inStack.clone());
                    Ok(ks_rand_match_degree(rest.clone(), degrees.clone(), ass.clone(), stack.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(ks_rand_match_degree(rest.clone(), degrees.clone(), ass.clone(), inStack.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStack)
}

// =============================================================================
// C-Implementation Stuff from
// Kamer Kaya, Johannes Langguth and Bora Ucar
// see: http://bmi.osu.edu/~kamer/index.html
// =============================================================================
pub fn DFSBExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 1, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.DFSBExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn BFSBExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 2, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.BFSBExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn MC21AExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 3, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.MC21AExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn PFExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 4, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.PFExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn PFPlusExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut nvars: i32;
    let mut neqns: i32;
    neqns = BackendDAEUtil::systemSize(isyst.clone())?;
    nvars = BackendVariable::daenumVariables(isyst.clone());
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if !((intGt(nvars.clone(), 0) && intGt(neqns.clone(), 0))) { bail!("guard") }
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 5, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if !((!(intGt(nvars.clone(), 0)) && !(intGt(neqns.clone(), 0)))) { bail!("guard") }
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.PFPlusExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn HKExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 6, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.HKExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn HKDWExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 7, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.HKDWExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn ABMPExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 8, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.ABMPExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

pub fn PR_FIFO_FAIRExternal(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut clearMatching: bool, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let true = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let true = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            (vec1, vec2) = getAssignment(clearMatching.clone(), nvars.clone(), neqns.clone(), isyst.clone())?;
            let true = (if (!(clearMatching.clone())) {BackendDAEEXT::setAssignment(neqns.clone(), nvars.clone(), vec1.clone(), vec2.clone())} else {true}) else { bail!("pattern mismatch") };
            (vec1, vec2, syst, shared, arg) = matchingExternal(metamodelica::nil(), false, 10, Config::getCheapMatchingAlgorithm()?, if (clearMatching.clone()) {1} else {0}, isyst.clone(), ishared.clone(), nvars.clone(), neqns.clone(), vec1.clone(), vec2.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone())?;
            syst = BackendDAEUtil::setEqSystMatching(syst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nvars: i32 = 0;
            let mut neqns: i32 = 0;
            let mut vec1: metamodelica::Array<i32> = Default::default();
            let mut vec2: metamodelica::Array<i32> = Default::default();
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            neqns = BackendDAEUtil::systemSize(isyst.clone())?;
            nvars = BackendVariable::daenumVariables(isyst.clone());
            let false = (intGt(nvars.clone(), 0)) else { bail!("pattern mismatch") };
            let false = (intGt(neqns.clone(), 0)) else { bail!("pattern mismatch") };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }))?;
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- Matching.PR_FIFO_FAIRExternal failed\n")).clone())?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outArg))
}

fn matchingExternal(mut meqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut internalCall: bool, mut algIndx: i32, mut cheapMatching: i32, mut clearMatching: i32, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    '__tco: loop {
        ({
        let mut changed: bool = false;
        ::match_deref::match_deref! { match &((meqns.clone(), internalCall.clone(), isyst.clone(), inMatchingOptions.clone())) {
        (Deref @ metamodelica::List::Nil, true, _, _) => {
            return Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
        },
        (Deref @ metamodelica::List::Nil, false, Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut m1t: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut unmatched_eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut meqns1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut m = (*m).clone();
            let mut mt = (*mt).clone();
            matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), m.clone());
            BackendDAEEXT::matching(nv.clone(), ne.clone(), algIndx.clone(), cheapMatching.clone(), metamodelica::OrderedFloat(1.0_f64), clearMatching.clone());
            BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
            (ass1_1, ass2_1) = (ass1.clone(), ass2.clone());
            syst = isyst.clone();
            if !(Flags::getConfigBool(Flags::NO_ASSC.clone())?) && BackendDAEUtil::hasIndexTypeSolvableAndUnprocessedScalar(syst.clone()) && BackendDAEUtil::doIndexReduction(inMatchingOptions.clone()) {
                syst = BackendDAEUtil::setAnalyticalToStructuralProcessed(syst.clone(), true)?;
                (_, m1, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(isyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                comps = Sorting::Tarjan(m1.clone(), ass2_1.clone(), metamodelica::arrayLength(ass2_1.clone()))?;
                for mut comp in &*comps.clone() {
                    let mut comp = comp.clone();
                    (ass1_1, ass2_1, syst, changed) = BackendDAEUtil::analyticalToStructuralSingularity(comp.clone(), ass1_1.clone(), ass2_1.clone(), syst.clone(), changed.clone(), false)?;
                }
                if changed.clone() {
                    BackendDAEEXT::setAssignment(nv.clone(), ne.clone(), ass1_1.clone(), ass2_1.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
                        Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    m = __pa0.clone();
                    mt = __pa1.clone();
                    matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), m.clone());
                    BackendDAEEXT::matching(nv.clone(), ne.clone(), algIndx.clone(), cheapMatching.clone(), metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass1_1.clone(), ass2_1.clone())?;
                }
            }
            unmatched_eqs = getUnassigned(ne.clone(), ass1_1.clone(), metamodelica::nil());
            if Flags::isSet(Flags::BLT_DUMP.clone())? && Flags::isSet(Flags::GRAPHML.clone())? {
                BackendDump::dumpBipartiteGraphEqSystem(isyst.clone(), ishared.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BeforMatching_")); __mm_s.push_str(&*intString(metamodelica::arrayLength(m.clone()))); __mm_s.push_str(&*literal!("_unmatched ")); __mm_s.push_str(&*intString((unmatched_eqs.clone().len() as i32))); ArcStr::from(__mm_s) }).clone())?;
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? && !(unmatched_eqs.clone().is_empty()) {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("unmatched equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(unmatched_eqs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            m1 = metamodelica::arrayFromVec(m.clone().borrow().clone());
            m1t = metamodelica::arrayFromVec(mt.clone().borrow().clone());
            (m1, m1t) = removeEdgesForNoDerivativeFunctionInputs(m1.clone(), m1t.clone(), syst.clone(), ishared.clone())?;
            (m1, m1t) = removeEdgesToDiscreteEquations(m1.clone(), m1t.clone(), syst.clone(), ishared.clone())?;
            meqns1 = getEqnsforIndexReduction(unmatched_eqs.clone(), ne.clone(), m1.clone(), m1t.clone(), ass1_1.clone(), ass2_1.clone(), inArg.clone())?;
            if !(meqns1.clone().is_empty()) {
                (syst, meqns1, ass1_1, ass2_1) = sanityCheckArtificialStates(syst.clone(), ishared.clone(), nv.clone(), ne.clone(), meqns1.clone(), ass1_1.clone(), ass2_1.clone(), algIndx.clone(), cheapMatching.clone(), clearMatching.clone(), inArg.clone())?;
            } else {
                (syst, meqns1, ass1_1, ass2_1) = (syst.clone(), meqns1.clone(), ass1_1.clone(), ass2_1.clone());
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? && !(meqns1.clone().is_empty()) {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Index Reduction neccessary!\n")); __mm_s.push_str(&*literal!("MSS subsets:\n ")); __mm_s.push_str(&*stringDelimitList(List::map(meqns1.clone(), (std::sync::Arc::new(Util::intLstString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?, (literal!("\n ")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            { (meqns, internalCall, algIndx, cheapMatching, clearMatching, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (meqns1.clone(), true, algIndx.clone(), -1, 0, syst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1_1.clone(), ass2_1.clone(), inMatchingOptions.clone(), sssHandler.clone(), inArg.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, _, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut memsize: i32 = 0;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass1_3: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_3: metamodelica::Array<i32> = Default::default();
            memsize = metamodelica::arrayLength(ass1.clone());
            (_, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            ne_1 = BackendDAEUtil::systemSize(syst.clone())?;
            nv_1 = BackendVariable::daenumVariables(syst.clone());
            ass1_2 = assignmentsArrayExpand(ass1_1.clone(), ne_1.clone(), memsize.clone(), -1)?;
            ass2_2 = assignmentsArrayExpand(ass2_1.clone(), nv_1.clone(), memsize.clone(), -1)?;
            let true = (BackendDAEEXT::setAssignment(ne_1.clone(), nv_1.clone(), ass1_2.clone(), ass2_2.clone())) else { bail!("pattern mismatch") };
            { (meqns, internalCall, algIndx, cheapMatching, clearMatching, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (metamodelica::nil(), false, algIndx.clone(), cheapMatching.clone(), clearMatching.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_2.clone(), ass2_2.clone(), inMatchingOptions.clone(), sssHandler.clone(), arg.clone()); continue '__tco; }
        },
        _ => {
            singularSystemError(meqns.clone(), 0, isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    })
    }
}

fn sanityCheckArtificialStates(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut algIndx: i32, mut cheapMatching: i32, mut clearMatching: i32, mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = eqns;
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut eqns_1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut unassignedStates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut flat_unassignedStates: Arc<metamodelica::List<i32>>;
    let mut flat_eqns: Arc<metamodelica::List<i32>>;
    let mut unmatched1: Arc<metamodelica::List<i32>>;
    let mut scalarToArrayMap: metamodelica::Array<i32>;
    let mut artificialStates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut undiffable_artificial: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut residuals: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut equations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut cr: Arc<DAE::ComponentRef>;
    let mut residualExp: Arc<DAE::Exp>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut m1t: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut unique_flag: bool;
    let mut msg: ArcStr;
    if let Ok((__pa0, __pa1, _, _)) = IndexReduction::minimalStructurallySingularSystem(eqns.clone(), syst.clone(), shared.clone(), ass2.clone(), ass1.clone(), arg.clone()) {
        eqns_1 = __pa0.clone();
        unassignedStates = __pa1.clone();
    } else {
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            singularSystemError(eqns.clone(), 0, syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), arg.clone())?;
        }
        bail!("fail");
    }
    flat_unassignedStates = List::flatten(unassignedStates.clone())?;
    for mut state in &*flat_unassignedStates.clone() {
        let mut state = state.clone();
        var = BackendVariable::getVarAt(syst.orderedVars.clone(), state.clone())?;
        if BackendVariable::isArtificialState(var.clone()) && !(listMember(var.clone(), artificialStates.clone())) {
            artificialStates = metamodelica::cons(var.clone(), artificialStates.clone());
        }
    }
    flat_eqns = List::flatten(eqns_1.clone())?;
    let __pa2 = ::match_deref::match_deref! { match &(syst.mapping.clone()) {
        Some((_, __pa2, _, _, _)) => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    scalarToArrayMap = __pa2.clone();
    for mut eqn in &*flat_eqns.clone() {
        let mut eqn = eqn.clone();
        if '__try3: {
            equations = metamodelica::cons(unwrap_break_err!(BackendEquation::get(syst.orderedEqs.clone(), ({let __elt = scalarToArrayMap.borrow()[(eqn.clone()-1) as usize].clone(); __elt})), '__try3), equations.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    for mut var in &*artificialStates.clone() {
        let mut var = var.clone();
        unique_flag = false;
        for mut eqn in &*equations.clone() {
            let mut eqn = eqn.clone();
            if '__try4: {
                residuals = unwrap_break_err!(BackendEquation::equationToScalarResidualForm(eqn.clone(), shared.functionTree.clone()), '__try4);
                for mut res in &*residuals.clone() {
                    let mut res = res.clone();
                    let __pa5 = ::match_deref::match_deref! { match &(res.clone()) {
                        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa5, .. } => __pa5.clone(),
                        _ => break '__try4 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    residualExp = __pa5.clone();
                    cr = unwrap_break_err!(BackendVariable::varCref(var.clone()), '__try4);
                    if unwrap_break_err!(Expression::expHasCref(residualExp.clone(), cr.clone()), '__try4) {
                        (residualExp, _, _) = unwrap_break_err!(Inline::forceInlineExp(residualExp.clone(), (Some(shared.functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), (std::sync::Arc::new(Ceval::cevalSimpleWithFunctionTreeReturnExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<Arc<DAE::Exp>> + 'static>)), '__try4);
                        residualExp = unwrap_break_err!(Expression::replaceDerOpInExp(residualExp.clone()), '__try4);
                        unwrap_break_err!(Differentiate::differentiateExpSolve(residualExp.clone(), cr.clone(), Some(shared.functionTree.clone())), '__try4);
                        let false = (unwrap_break_err!(Expression::expHasCrefInSmoothZero(residualExp.clone(), cr.clone()), '__try4)) else { break '__try4 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                    }
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### The Equation ### \n")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!("\n\n--- could not be differentiated for artificial variable ---\n ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var.varName.clone())?); __mm_s.push_str(&*literal!(".\n\n")); ArcStr::from(__mm_s) }).clone());
                }
                if !(unique_flag.clone()) {
                    undiffable_artificial = metamodelica::cons(BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?, undiffable_artificial.clone());
                    unique_flag = true;
                }
            }
        }
    }
    if !(undiffable_artificial.clone().is_empty()) {
        assign_field!(syst.orderedVars = BackendVariable::addVars(undiffable_artificial.clone(), syst.orderedVars.clone())?);
        (syst, _, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(shared.functionTree.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        if isSome(syst.m.clone()) && isSome(syst.mT.clone()) {
            let __pa6 = ::match_deref::match_deref! { match &(syst.m.clone()) {
                Some(__pa6) => __pa6.clone(),
                _ => bail!("pattern mismatch"),
            } };
            m = __pa6.clone();
            let __pa7 = ::match_deref::match_deref! { match &(syst.mT.clone()) {
                Some(__pa7) => __pa7.clone(),
                _ => bail!("pattern mismatch"),
            } };
            mt = __pa7.clone();
            matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), m.clone());
            BackendDAEEXT::matching(nv.clone(), ne.clone(), algIndx.clone(), cheapMatching.clone(), metamodelica::OrderedFloat(1.0_f64), clearMatching.clone());
            BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
            unmatched1 = getUnassigned(ne.clone(), ass1.clone(), metamodelica::nil());
            m1 = metamodelica::arrayFromVec(m.clone().borrow().clone());
            m1t = metamodelica::arrayFromVec(mt.clone().borrow().clone());
            (m1, m1t) = removeEdgesForNoDerivativeFunctionInputs(m1.clone(), m1t.clone(), syst.clone(), shared.clone())?;
            (m1, m1t) = removeEdgesToDiscreteEquations(m1.clone(), m1t.clone(), syst.clone(), shared.clone())?;
            eqns = getEqnsforIndexReduction(unmatched1.clone(), ne.clone(), m1.clone(), m1t.clone(), ass1.clone(), ass2.clone(), arg.clone())?;
        }
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("----------------------------- INFO -----------------------------\n")); __mm_s.push_str(&*literal!(" Artificial states are those which do not naturally appear\n")); __mm_s.push_str(&*literal!(" differentiated in the system of DAEs, but have been forced\n")); __mm_s.push_str(&*literal!(" to be states with 'StateSelect.always' or 'StateSelect.prefer'.\n")); __mm_s.push_str(&*literal!(" The ones mentioned above will be treated as if they had \n")); __mm_s.push_str(&*literal!("'StateSelect.default'.\n")); __mm_s.push_str(&*literal!("----------------------------------------------------------------\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::varListStringShort(undiffable_artificial.clone(), (literal!("They will be treated as if they had stateSelect=StateSelect.default")).clone())?); __mm_s.push_str(&*literal!("Please use -d=bltdump for more information.\n")); ArcStr::from(__mm_s) }).clone();
        Error::addMessage(Error::STATE_STATESELECT_PREFER_REVERT.clone(), list![(msg.clone()).clone()])?;
    }
    Ok((syst, eqns, ass1, ass2))
}

fn removeEdgesToDiscreteEquations(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sys: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut mOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mtOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut isDiscrete: bool;
    let mut idx: i32;
    let mut idx2: i32;
    let mut size: i32;
    let mut varIdx: i32 = 0;
    let mut varIdxs: Arc<metamodelica::List<i32>>;
    let mut row: Arc<metamodelica::List<i32>>;
    let mut eqIdxs: Arc<metamodelica::List<i32>>;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut eqIdxArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    vars = sys.orderedVars.clone();
    eqs = sys.orderedEqs.clone();
    idx = 1;
    idx2 = 0;
    eqIdxArray = arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), metamodelica::nil());
    for mut eq in &*BackendEquation::equationList(eqs.clone())? {
        let mut eq = eq.clone();
        size = BackendEquation::equationSize(BackendEquation::get(eqs.clone(), idx.clone())?)?;
        eqIdxs = List::map1(List::intRange(size.clone()), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), idx2.clone())?;
        metamodelica::arrayUpdate(eqIdxArray.clone(), idx.clone(), eqIdxs.clone())?;
        idx = idx.clone() + 1;
        idx2 = size.clone() + idx2.clone();
    }
    idx = 1;
    for mut eq in &*BackendEquation::equationList(eqs.clone())? {
        let mut eq = eq.clone();
        isDiscrete = BackendEquation::isWhenEquationOrDiscreteAlgorithm(eq.clone(), vars.clone())?;
        if isDiscrete.clone() {
            varLst = BackendEquation::equationVars(eq.clone(), vars.clone())?;
            varIdxs = BackendVariable::getVarIndexFromVars(varLst.clone(), vars.clone());
            eqIdxs = ({let __elt = eqIdxArray.borrow()[(idx.clone()-1) as usize].clone(); __elt});
            for mut e in &*eqIdxs.clone() {
                let mut e = e.clone();
                row = ({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt});
                row = UnorderedSet::difference_list(row.clone(), varIdxs.clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                metamodelica::arrayUpdate(m.clone(), e.clone(), row.clone())?;
            }
            for mut varIdx in &*varIdxs.clone() {
                let mut varIdx = varIdx.clone();
                row = ({let __elt = mt.borrow()[(varIdx.clone()-1) as usize].clone(); __elt});
                row = UnorderedSet::difference_list(row.clone(), eqIdxs.clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                metamodelica::arrayUpdate(mt.clone(), varIdx.clone(), row.clone())?;
            }
        }
        idx = idx.clone() + 1;
    }
    mOut = m.clone();
    mtOut = mt.clone();
    Ok((mOut, mtOut))
}

fn removeEdgesForNoDerivativeFunctionInputs(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sys: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut mOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mtOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut hasNoDerAnno: bool;
    let mut idx: i32;
    let mut varIdx: i32 = 0;
    let mut varIdxs: Arc<metamodelica::List<i32>>;
    let mut row: Arc<metamodelica::List<i32>>;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut functionTree: Arc<AvlTreePathFunction::Tree>;
    let mut noDerInputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    vars = sys.orderedVars.clone();
    eqs = sys.orderedEqs.clone();
    functionTree = shared.functionTree.clone();
    idx = 1;
    for mut eq in &*BackendEquation::equationList(eqs.clone())? {
        let mut eq = eq.clone();
        (hasNoDerAnno, noDerInputs) = BackendDAEUtil::isFuncCallWithNoDerAnnotation(eq.clone(), functionTree.clone())?;
        if hasNoDerAnno.clone() {
            (_, varIdxs) = BackendVariable::getVarLst(noDerInputs.clone(), vars.clone());
            row = ({let __elt = m.borrow()[(idx.clone()-1) as usize].clone(); __elt});
            row = UnorderedSet::difference_list(row.clone(), varIdxs.clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            metamodelica::arrayUpdate(m.clone(), idx.clone(), row.clone())?;
            for mut varIdx in &*varIdxs.clone() {
                let mut varIdx = varIdx.clone();
                row = metamodelica::arrayGet(mt.clone(), varIdx.clone())?;
                (row, _) = List::deleteMemberOnTrue(idx.clone(), row.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                metamodelica::arrayUpdate(mt.clone(), varIdx.clone(), row.clone())?;
            }
        }
        idx = idx.clone() + 1;
    }
    mOut = m.clone();
    mtOut = mt.clone();
    Ok((mOut, mtOut))
}

fn countadjacencyMatrixEntries(mut n: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut outCount: i32 = 0;
    for mut i in 1..=n.clone() {
        let __range0 = &*({let __elt = m.borrow()[(i.clone()-1) as usize].clone(); __elt});
        for mut e in __range0 {
            let mut e = e.clone();
            if intGt(e.clone(), 0) {
                outCount = outCount.clone() + 1;
            }
        }
    }
    outCount
}

pub fn matchingExternalsetAdjacencyMatrix(mut nv: i32, mut ne: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    let mut nz: i32;
    nz = countadjacencyMatrixEntries(ne.clone(), m.clone());
    BackendDAEEXT::setAdjacencyMatrix(nv.clone(), ne.clone(), nz.clone(), m.clone());
    ()
}

// =============================================================================
// Util Functions
//
// =============================================================================
pub fn reachableEquations(mut eqn: i32, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass2: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut outEqNodes: Arc<metamodelica::List<i32>>;
    let mut var: i32;
    var = ({let __elt = ass2.borrow()[(eqn.clone()-1) as usize].clone(); __elt});
    outEqNodes = if (var.clone() > 0) {({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (({let __elt = mT.borrow()[(var.clone()-1) as usize].clone(); __elt})).into_iter().cloned() {
            if !(e.clone() > 0 && e.clone() != eqn.clone()) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })} else {metamodelica::nil()};
    outEqNodes
}

pub fn incomingEquations(mut eqn: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut outEqNodes: Arc<metamodelica::List<i32>>;
    outEqNodes = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut var in (({let __elt = m.borrow()[(eqn.clone()-1) as usize].clone(); __elt})).into_iter().cloned() {
            if !(var.clone() > 0 && ({let __elt = ass1.borrow()[(var.clone()-1) as usize].clone(); __elt}) != eqn.clone() && ({let __elt = ass1.borrow()[(var.clone()-1) as usize].clone(); __elt}) > 0) { continue; }
            let __x = ({let __elt = ass1.borrow()[(var.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outEqNodes
}

pub fn isAssigned(mut ass: metamodelica::Array<i32>, mut i: i32) -> bool {
    let mut b: bool;
    b = intGt(({let __elt = ass.borrow()[(intAbs(i.clone())-1) as usize].clone(); __elt}), 0);
    b
}

pub fn isUnAssigned(mut ass: metamodelica::Array<i32>, mut i: i32) -> bool {
    let mut b: bool;
    b = intLt(({let __elt = ass.borrow()[(intAbs(i.clone())-1) as usize].clone(); __elt}), 1);
    b
}

pub fn getMarked(mut ne: i32, mut mark: i32, mut markArr: metamodelica::Array<i32>, mut iMarked: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    '__tco: loop {
        match ne.clone() {
        0 => {
            return iMarked.clone()
        },
        _ => {
            let mut marked: Arc<metamodelica::List<i32>> = metamodelica::nil();
            marked = List::consOnTrue(intEq(({let __elt = markArr.borrow()[(ne.clone()-1) as usize].clone(); __elt}), mark.clone()), ne.clone(), iMarked.clone());
            { (ne, mark, markArr, iMarked) = (ne.clone() - 1, mark.clone(), markArr.clone(), marked.clone()); continue '__tco; }
        },
    }
    }
}

pub fn getUnassigned(mut ne: i32, mut ass: metamodelica::Array<i32>, mut inUnassigned: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    '__tco: loop {
        match ne.clone() {
        0 => {
            return inUnassigned.clone()
        },
        _ => {
            let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
            unassigned = List::consOnTrue(intLt(({let __elt = ass.borrow()[(ne.clone()-1) as usize].clone(); __elt}), 1), ne.clone(), inUnassigned.clone());
            { (ne, ass, inUnassigned) = (ne.clone() - 1, ass.clone(), unassigned.clone()); continue '__tco; }
        },
    }
    }
}

pub fn anyUnassigned(mut ne: i32, mut ass: metamodelica::Array<i32>) -> bool {
    '__tco: loop {
        match ne.clone() {
        0 => return false,
        _ if (intLt(({let __elt = ass.borrow()[(ne.clone()-1) as usize].clone(); __elt}), 1)) => return true,
        _ => { (ne, ass) = (ne.clone() - 1, ass.clone()); continue '__tco; },
    }
    }
}

pub fn getAssignedArray(mut ass: metamodelica::Array<i32>) -> Result<metamodelica::Array<bool>> {
    let mut outIsAssigned: metamodelica::Array<bool>;
    let mut N: i32 = metamodelica::arrayLength(ass.clone());
    outIsAssigned = arrayCreate(N.clone(), false);
    for mut i in 1..=N.clone() {
        if ({let __elt = ass.borrow()[(i.clone()-1) as usize].clone(); __elt}) > 0 {
            metamodelica::arrayUpdate(outIsAssigned.clone(), i.clone(), true)?;
        }
    }
    Ok(outIsAssigned)
}

pub fn getAssigned(mut ne: i32, mut ass: metamodelica::Array<i32>, mut inAssigned: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    '__tco: loop {
        match ne.clone() {
        0 => {
            return inAssigned.clone()
        },
        _ => {
            let mut assigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
            assigned = List::consOnTrue(intGt(({let __elt = ass.borrow()[(ne.clone()-1) as usize].clone(); __elt}), 0), ne.clone(), inAssigned.clone());
            { (ne, ass, inAssigned) = (ne.clone() - 1, ass.clone(), assigned.clone()); continue '__tco; }
        },
    }
    }
}

pub fn getEqnsforIndexReduction(mut U: Arc<metamodelica::List<i32>>, mut neqns: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    eqns = (::match_deref::match_deref! { match &((U.clone(), inArg.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (_, (_, _, mapEqnIncRow, mapIncRowEqn, _)) => {
            let mut lengthU: i32 = 0;
            let mut colummarks: metamodelica::Array<i32> = Default::default();
            let mut subsets: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            colummarks = arrayCreate(neqns.clone(), -1);
            lengthU = (U.clone().len() as i32);
            subsets = arrayCreate(lengthU.clone(), metamodelica::nil());
            subsets = getEqnsforIndexReduction1(U.clone(), m.clone(), mT.clone(), 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), subsets.clone())?;
            removeEmptySubsets(1, lengthU.clone(), subsets.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

fn removeEmptySubsets(mut index: i32, mut length: i32, mut subsets: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iAcc.clone()) {
        _ if (intLe(index.clone(), length.clone())) => {
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            eqns = ({let __elt = subsets.borrow()[(index.clone()-1) as usize].clone(); __elt});
            acc = appendNonEmpty(eqns.clone(), iAcc.clone());
            { (index, length, subsets, iAcc) = (index.clone() + 1, length.clone(), subsets.clone(), acc.clone()); continue '__tco; }
        },
        _ => {
            return iAcc.clone()
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn appendNonEmpty(mut eqns: Arc<metamodelica::List<i32>>, mut iAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut oAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    oAcc = (::match_deref::match_deref! { match &(eqns.clone()) {
        Deref @ metamodelica::List::Nil => iAcc.clone(),
        _ => metamodelica::cons(eqns.clone(), iAcc.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oAcc
}

fn getEqnsforIndexReduction1(mut U: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut inSubsets: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(U.clone()) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inSubsets.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } if (!(intGt(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), 0))) => {
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut e1: i32 = 0;
            e1 = ({let __elt = mapIncRowEqn.borrow()[(e.clone()-1) as usize].clone(); __elt});
            eqns = ({let __elt = mapEqnIncRow.borrow()[(e1.clone()-1) as usize].clone(); __elt});
            List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), colummarks.clone())?;
            eqns = getEqnsforIndexReductionphase(eqns.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), eqns.clone())?;
            Array::appendToElement(mark.clone(), eqns.clone(), inSubsets.clone())?;
            { (U, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets) = (rest.clone(), m.clone(), mT.clone(), mark.clone() + 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone()); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            { (U, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets) = (rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getEqnsforIndexReductionphase(mut elst: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut inSubsets: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inEqns: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(elst.clone()) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inEqns.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            rows = List::select(({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            eqns = getEqnsforIndexReductiontraverseRows(rows.clone(), metamodelica::nil(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), inEqns.clone())?;
            { (elst, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets, inEqns) = (rest.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), eqns.clone()); continue '__tco; }
        },
        _ => {
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getEqnsforIndexReductiontraverseRows(mut rows: Arc<metamodelica::List<i32>>, mut nextColums: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut inSubsets: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inEqns: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((rows.clone(), nextColums.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(inEqns.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(getEqnsforIndexReductionphase(nextColums.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), inEqns.clone())?)
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _) => {
            let mut queue: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut nextqueue: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rc: i32 = 0;
            let mut e: i32 = 0;
            rc = ({let __elt = ass2.borrow()[(r.clone()-1) as usize].clone(); __elt});
            if List::exist1(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rc.clone())? && intGt(rc.clone(), 0) && !(intEq(({let __elt = colummarks.borrow()[(rc.clone()-1) as usize].clone(); __elt}), mark.clone())) {
                if intGt(({let __elt = colummarks.borrow()[(rc.clone()-1) as usize].clone(); __elt}), 0) {
                    mergeSubsets(mark.clone(), ({let __elt = colummarks.borrow()[(rc.clone()-1) as usize].clone(); __elt}), inSubsets.clone(), colummarks.clone())?;
                    nextqueue = nextColums.clone();
                    queue = inEqns.clone();
                } else {
                    e = ({let __elt = mapIncRowEqn.borrow()[(rc.clone()-1) as usize].clone(); __elt});
                    eqns = ({let __elt = mapEqnIncRow.borrow()[(e.clone()-1) as usize].clone(); __elt});
                    List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), colummarks.clone())?;
                    nextqueue = listAppend(nextColums.clone(), eqns.clone());
                    queue = listAppend(inEqns.clone(), eqns.clone());
                }
            } else {
                nextqueue = nextColums.clone();
                queue = inEqns.clone();
            }
            { (rows, nextColums, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets, inEqns) = (rest.clone(), nextqueue.clone(), m.clone(), mT.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), queue.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn mergeSubsets(mut mark: i32, mut markColum: i32, mut inSubsets: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut colummarks: metamodelica::Array<i32>) -> Result<()> {
    let mut eqns: Arc<metamodelica::List<i32>>;
    eqns = ({let __elt = inSubsets.borrow()[(markColum.clone()-1) as usize].clone(); __elt});
    Array::appendToElement(mark.clone(), eqns.clone(), inSubsets.clone())?;
    metamodelica::arrayUpdate(inSubsets.clone(), markColum.clone(), metamodelica::nil())?;
    List::fold1r(eqns.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), colummarks.clone())?;
    Ok(())
}

fn reduceIndexifNecessary(mut meqns: Arc<metamodelica::List<i32>>, mut actualEqn: i32, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut nv: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), mut sssHandler: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, i32, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> {
    let mut outchangedEqns: Arc<metamodelica::List<i32>>;
    let mut continueEqn: i32;
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut outArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    (outchangedEqns, continueEqn, osyst, oshared, nvars, neqns, outAss1, outAss2, outArg) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (metamodelica::nil(), actualEqn.clone() + 1, isyst.clone(), ishared.clone(), nv.clone(), ne.clone(), ass1.clone(), ass2.clone(), inArg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32 = 0;
            let mut ne_1: i32 = 0;
            let mut i_1: i32 = 0;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
            let mut ass1_1: metamodelica::Array<i32> = Default::default();
            let mut ass1_2: metamodelica::Array<i32> = Default::default();
            let mut ass2_1: metamodelica::Array<i32> = Default::default();
            let mut ass2_2: metamodelica::Array<i32> = Default::default();
            let mut changedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (changedEqns, i_1, syst, shared, ass2_1, ass1_1, arg) = sssHandler(list![meqns.clone()], actualEqn.clone(), isyst.clone(), ishared.clone(), ass2.clone(), ass1.clone(), inArg.clone())?;
            ne_1 = BackendDAEUtil::systemSize(syst.clone())?;
            nv_1 = BackendVariable::daenumVariables(syst.clone());
            ass1_2 = assignmentsArrayExpand(ass1_1.clone(), ne_1.clone(), metamodelica::arrayLength(ass1_1.clone()), -1)?;
            ass2_2 = assignmentsArrayExpand(ass2_1.clone(), nv_1.clone(), metamodelica::arrayLength(ass2_1.clone()), -1)?;
            (changedEqns.clone(), i_1.clone(), syst.clone(), shared.clone(), nv_1.clone(), ne_1.clone(), ass1_2.clone(), ass2_2.clone(), arg.clone())
        },
        (_, _) => {
            singularSystemError(list![meqns.clone()], actualEqn.clone(), isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outchangedEqns, continueEqn, osyst, oshared, nvars, neqns, outAss1, outAss2, outArg))
}

fn assignmentsArrayExpand(mut ass: metamodelica::Array<i32>, mut needed: i32, mut memsize: i32, mut default: i32) -> Result<metamodelica::Array<i32>> {
    let mut outAss: metamodelica::Array<i32>;
    outAss = 'mc: {
        let __mc_input = default.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(memsize.clone(), needed.clone())) else { bail!("pattern mismatch") };
            Ok(ass.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(memsize.clone(), needed.clone())) else { bail!("pattern mismatch") };
            Ok(Array::expand(needed.clone() - memsize.clone(), ass.clone(), default.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("function assignmentsArrayExpand failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAss)
}

fn assignmentsArrayBooleanExpand(mut ass: metamodelica::Array<bool>, mut needed: i32, mut memsize: i32, mut default: bool) -> Result<metamodelica::Array<bool>> {
    let mut outAss: metamodelica::Array<bool>;
    outAss = 'mc: {
        let __mc_input = default.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(memsize.clone(), needed.clone())) else { bail!("pattern mismatch") };
            Ok(ass.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(memsize.clone(), needed.clone())) else { bail!("pattern mismatch") };
            Ok(Array::expand(needed.clone() - memsize.clone(), ass.clone(), default.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("function assignmentsArrayExpand failed")).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAss)
}

fn checkAssignment(mut indx: i32, mut ne: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut inUnassigned: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnassigned: Arc<metamodelica::List<i32>>;
    outUnassigned = 'mc: {
        let __mc_input = inUnassigned.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(indx.clone(), ne.clone())) else { bail!("pattern mismatch") };
                    Ok(inUnassigned.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r: i32 = 0;
                    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    r = ({let __elt = ass1.borrow()[(indx.clone()-1) as usize].clone(); __elt});
                    unassigned = List::consOnTrue(intLt(r.clone(), 0), indx.clone(), inUnassigned.clone());
                    Ok(checkAssignment(indx.clone() + 1, ne.clone(), ass1.clone(), ass2.clone(), unassigned.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outUnassigned)
}

fn getAssignment(mut clearMatching: bool, mut nVars: i32, mut nEqns: i32, mut iSyst: Arc<BackendDAE::EqSystem>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut ass1: metamodelica::Array<i32> = Default::default();
    let mut ass2: metamodelica::Array<i32> = Default::default();
    (ass1, ass2) = 'mc: {
        let __mc_input = (clearMatching.clone(), iSyst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, .. }, .. }) => {
                    let true = (intGe(nVars.clone(), metamodelica::arrayLength(ass1.clone()))) else { bail!("pattern mismatch") };
                    let true = (intGe(nEqns.clone(), metamodelica::arrayLength(ass2.clone()))) else { bail!("pattern mismatch") };
                    Ok((ass2.clone(), ass1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ass1: metamodelica::Array<i32> = ass1.clone();
                    let mut ass2: metamodelica::Array<i32> = ass2.clone();
                    ass2 = arrayCreate(nEqns.clone(), -1);
                    ass1 = arrayCreate(nVars.clone(), -1);
                    Ok(((ass2.clone(), ass1.clone()), ass1.clone(), ass2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { ass1 = __wb0; ass2 = __wb1; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((ass1, ass2))
}

// =============================================================================
// tests
//
// =============================================================================
pub fn testMatchingAlgorithms(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints)) -> Result<()> {
    let mut t: metamodelica::Real;
    let mut nv: i32;
    let mut ne: i32;
    let mut cheapID: i32;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vec1: metamodelica::Array<i32>;
    let mut vec2: metamodelica::Array<i32>;
    let mut matchingAlgorithms: Arc<metamodelica::List<(ArcStr, BackendDAEFunc::matchingAlgorithmFunc)>>;
    let mut extmatchingAlgorithms: Arc<metamodelica::List<(ArcStr, i32)>>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    ne = BackendDAEUtil::systemSize(isyst.clone())?;
    nv = BackendVariable::daenumVariables(isyst.clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Systemsize: ")); __mm_s.push_str(&*intString(ne.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    matchingAlgorithms = list![(literal!("OMCNew:   "), (std::sync::Arc::new(DFSLH) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("BFSB:     "), (std::sync::Arc::new(BFSB) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("DFSB:     "), (std::sync::Arc::new(DFSB) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("MC21A:    "), (std::sync::Arc::new(MC21A) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("PF:       "), (std::sync::Arc::new(PF) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("PFPlus:   "), (std::sync::Arc::new(PFPlus) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("HK:       "), (std::sync::Arc::new(HK) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("HKDW:     "), (std::sync::Arc::new(HKDW) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("ABMP:     "), (std::sync::Arc::new(ABMP) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)), (literal!("PR:       "), (std::sync::Arc::new(PR_FIFO_FAIR) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>))];
    syst = randSortSystem(isyst.clone(), ishared.clone())?;
    testMatchingAlgorithms1(matchingAlgorithms.clone(), syst.clone(), ishared.clone(), inMatchingOptions.clone())?;
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    (_, m, _) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
    matchingExternalsetAdjacencyMatrix(nv.clone(), ne.clone(), m.clone());
    cheapID = 3;
    t = System::realtimeTock(ClockIndexes::RT_PROFILER0.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SetMEXT:     ")); __mm_s.push_str(&*realString(t.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    extmatchingAlgorithms = list![(literal!("DFSEXT:   "), 1), (literal!("BFSEXT:   "), 2), (literal!("MC21AEXT: "), 3), (literal!("PFEXT:    "), 4), (literal!("PFPlusEXT:"), 5), (literal!("HKEXT:    "), 6), (literal!("HKDWEXT   "), 7), (literal!("ABMPEXT   "), 8), (literal!("PREXT:    "), 10)];
    testExternMatchingAlgorithms1(extmatchingAlgorithms.clone(), cheapID.clone(), nv.clone(), ne.clone());
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    vec1 = arrayCreate(ne.clone(), -1);
    vec2 = arrayCreate(nv.clone(), -1);
    BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GetAssEXT:   ")); __mm_s.push_str(&*realString(t.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    Ok(())
}

pub fn testMatchingAlgorithms1(mut matchingAlgorithms: Arc<metamodelica::List<(ArcStr, Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>)>>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints)) -> Result<()> {
    let () = 'mc: {
        let __mc_input = matchingAlgorithms.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r#str, matchingAlgorithm), tail: rest } => {
                    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
                    testMatchingAlgorithm(10, matchingAlgorithm.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone())?;
                    t = System::realtimeTock(ClockIndexes::RT_PROFILER0.clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*realString(realDiv(t.clone(), metamodelica::OrderedFloat(10.0_f64)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    testMatchingAlgorithms1(rest.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r#str, _), tail: rest } => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("failed!\n")); ArcStr::from(__mm_s) }).clone());
                    testMatchingAlgorithms1(rest.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn testMatchingAlgorithm(mut index: i32, mut matchingAlgorithm: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, (BackendDAE::IndexReduction, BackendDAE::EquationConstraints), Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints)) -> Result<()> {
    let () = 'mc: {
        let __mc_input = index.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (BackendDAE::StateOrder::NOSTATEORDER, Default::default(), Default::default(), Default::default(), 0);
            arg = IndexReduction::getStructurallySingularSystemHandlerArg(isyst.clone(), ishared.clone(), metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()))?;
            matchingAlgorithm(isyst.clone(), ishared.clone(), true, inMatchingOptions.clone(), (std::sync::Arc::new(IndexReduction::pantelidesIndexReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32))> + 'static>), arg.clone())?;
            testMatchingAlgorithm(index.clone() - 1, matchingAlgorithm.clone(), isyst.clone(), ishared.clone(), inMatchingOptions.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn testExternMatchingAlgorithms1(mut matchingAlgorithms: Arc<metamodelica::List<(ArcStr, i32)>>, mut cheapId: i32, mut nv: i32, mut ne: i32) -> () {
    let mut r#str: ArcStr;
    let mut matchingAlgorithm: i32;
    let mut t: metamodelica::Real;
    for mut alg in &*matchingAlgorithms.clone() {
        let mut alg = alg.clone();
        (r#str, matchingAlgorithm) = alg.clone();
        if '__try0: {
            unwrap_break_err!(System::realtimeTick(ClockIndexes::RT_PROFILER0.clone()), '__try0);
            unwrap_break_err!(testExternMatchingAlgorithm(10, matchingAlgorithm.clone(), cheapId.clone(), nv.clone(), ne.clone()), '__try0);
            t = unwrap_break_err!(System::realtimeTock(ClockIndexes::RT_PROFILER0.clone()), '__try0);
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*realString(realDiv(t.clone(), metamodelica::OrderedFloat(10.0_f64)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("failed!\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    ()
}

pub fn testExternMatchingAlgorithm(mut index: i32, mut matchingAlgorithm: i32, mut cheapId: i32, mut nv: i32, mut ne: i32) -> Result<()> {
    let () = 'mc: {
        let __mc_input = index.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            BackendDAEEXT::matching(nv.clone(), ne.clone(), matchingAlgorithm.clone(), cheapId.clone(), metamodelica::OrderedFloat(1.0_f64), 1);
            testExternMatchingAlgorithm(index.clone() - 1, matchingAlgorithm.clone(), cheapId.clone(), nv.clone(), ne.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn randSortSystem(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut ne: i32 = 0;
            let mut nv: i32 = 0;
            let mut randarr: metamodelica::Array<i32> = Default::default();
            let mut randarr1: metamodelica::Array<i32> = Default::default();
            let mut syst = (*syst).clone();
            ne = BackendDAEUtil::systemSize(isyst.clone())?;
            nv = BackendVariable::daenumVariables(isyst.clone());
            randarr = Array::createIntRange(ne.clone());
            setrandArray(ne.clone(), randarr.clone())?;
            randarr1 = Array::createIntRange(nv.clone());
            setrandArray(nv.clone(), randarr1.clone())?;
            assign_field!(
                syst.orderedEqs = randSortSystem1(ne.clone(), 0, randarr.clone(), eqns.clone(), BackendEquation::listEquation(metamodelica::nil())?, (std::sync::Arc::new(BackendEquation::get) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, i32) -> Result<Arc<BackendDAE::Equation>> + 'static>), (std::sync::Arc::new(BackendEquation::add) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>))?,
                syst.orderedVars = randSortSystem1(nv.clone(), 0, randarr1.clone(), vars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), (std::sync::Arc::new(BackendVariable::addVar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>))?
            );
            (syst, _, _) = BackendDAEUtil::getAdjacencyMatrix(BackendDAEUtil::clearEqSyst(syst.clone())?, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osyst)
}

fn randSortSystem1<Type_a: Clone + 'static, Type_b: Clone + 'static>(mut index: i32, mut offset: i32, mut randarr: metamodelica::Array<i32>, mut oldTypeA: Type_a, mut newTypeA: Type_a, mut get: Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<Type_b> + 'static>, mut set: Arc<dyn ::std::ops::Fn(Type_b, Type_a) -> Result<Type_a> + 'static>) -> Result<Type_a> {
    pub type getFunc<Type_a: Clone + 'static, Type_b: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<Type_b> + 'static>;

    pub type setFunc<Type_b: Clone + 'static, Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_b, Type_a) -> Result<Type_a> + 'static>;

    '__tco: loop {
        match index.clone() {
        0 => {
            return Ok(newTypeA.clone())
        },
        _ => {
            let mut tb: Type_b;
            let mut ta: Type_a;
            tb = get(oldTypeA.clone(), ({let __elt = randarr.borrow()[(index.clone()-1) as usize].clone(); __elt}) + offset.clone())?;
            ta = set(tb.clone(), newTypeA.clone())?;
            { (index, offset, randarr, oldTypeA, newTypeA, get, set) = (index.clone() - 1, offset.clone(), randarr.clone(), oldTypeA.clone(), ta.clone(), get.clone(), set.clone()); continue '__tco; }
        },
    }
    }
}

fn singularSystemError(mut eqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut actualEqn: i32, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inAssignments1: metamodelica::Array<i32>, mut inAssignments2: metamodelica::Array<i32>, mut inArg: (BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<()> {
    let mut n: i32;
    let mut unmatched: Arc<metamodelica::List<i32>>;
    let mut unmatched1: Arc<metamodelica::List<i32>>;
    let mut vars: Arc<metamodelica::List<i32>>;
    let mut eqn_str: ArcStr;
    let mut var_str: ArcStr;
    let mut source: Arc<DAE::ElementSource>;
    let mut info: SourceInfo;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    (_, _, _, mapIncRowEqn, _) = inArg.clone();
    n = BackendDAEUtil::systemSize(isyst.clone())?;
    unmatched = List::flatten(eqns.clone())?;
    unmatched1 = List::map1r(unmatched.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mapIncRowEqn.clone())?;
    unmatched1 = List::uniqueIntN(unmatched1.clone(), metamodelica::arrayLength(mapIncRowEqn.clone()))?;
    eqn_str = (BackendDump::dumpMarkedEqns(isyst.clone(), unmatched1.clone())?).clone();
    vars = getUnassigned(n.clone(), inAssignments2.clone(), metamodelica::nil());
    vars = List::fold1(unmatched.clone(), (std::sync::Arc::new(fnptr!(getAssignedVars, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), inAssignments1.clone(), vars.clone())?;
    var_str = (BackendDump::dumpMarkedVars(isyst.clone(), vars.clone())?).clone();
    source = BackendEquation::markedEquationSource(isyst.clone(), listHead(unmatched1.clone())?)?;
    info = ElementSource::getElementSourceFileInfo(source.clone());
    Error::addSourceMessage(Error::STRUCT_SINGULAR_SYSTEM.clone(), list![(eqn_str.clone()).clone(), (var_str.clone()).clone()], info.clone())?;
    Ok(())
}

fn getAssignedVars(mut e: i32, mut ass: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oAcc: Arc<metamodelica::List<i32>>;
    let mut i: i32;
    let mut b: bool;
    i = ({let __elt = ass.borrow()[(e.clone()-1) as usize].clone(); __elt});
    b = intGt(i.clone(), 0);
    oAcc = List::consOnTrue(b.clone(), i.clone(), iAcc.clone());
    oAcc
}

fn clearArrayWithKnownSetIndexes(mut arr: metamodelica::Array<bool>, mut arrIx: metamodelica::Array<i32>, mut n: i32) -> Result<()> {
    let debug: bool = false;
    if metamodelica::OrderedFloat((n.clone()) as f64) > metamodelica::OrderedFloat(0.3_f64) * metamodelica::OrderedFloat((metamodelica::arrayLength(arr.clone())) as f64) {
        for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(arr.clone(), i.clone(), false);
        }
    } else {
        let true = (n.clone() <= metamodelica::arrayLength(arrIx.clone())) else { bail!("pattern mismatch") };
        for mut i in 1..=n.clone() {
            metamodelica::arrayUpdate(arr.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(arrIx.clone(), i.clone()), false)?;
        }
    }
    if debug.clone() {
        for mut e in 1..=metamodelica::arrayLength(arr.clone()) {
            Error::assertion(!(metamodelica::arrayGet(arr.clone(), e.clone())?), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("clearArrayWithKnownSetIndexes failed: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", e.clone()))); __mm_s.push_str(&*literal!(" n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", n.clone()))); __mm_s.push_str(&*literal!(" ixs=")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut i in (1..=n.clone()).into_iter() {
            let __x = ArcStr::from(::std::format!("{}", metamodelica::arrayGet(arrIx.clone(), i.clone())?));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
        }
    }
    Ok(())
}

