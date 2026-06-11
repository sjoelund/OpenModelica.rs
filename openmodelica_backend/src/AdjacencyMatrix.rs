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

use openmodelica_backend_types::BackendDAE;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn copyAdjacencyMatrix(mut inAdjacencyMatrix: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outAdjacencyMatrix: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>;
    outAdjacencyMatrix = (match inAdjacencyMatrix {
        Some(mut m) => {
            m = metamodelica::arrayFromVec(m.clone().borrow().clone());
            Some(m.clone())
        },
        _ => {
            None
        },
    });
    outAdjacencyMatrix
}

pub use copyAdjacencyMatrix as copyAdjacencyMatrixT;

pub(crate) fn traverseAdjacencyMatrix<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = traverseAdjacencyMatrix1(inM.clone(), func.clone(), 1, metamodelica::arrayLength(inM.clone()), inTypeA)?;
    Ok((outM, outTypeA))
}

fn traverseAdjacencyMatrix1<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut pos: i32, mut len: i32, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = traverseAdjacencyMatrix2(inM.clone(), func.clone(), pos, len, intGt(pos, len), inTypeA)?;
    Ok((outM, outTypeA))
}

fn traverseAdjacencyMatrix2<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut pos: i32, mut len: i32, mut stop: bool, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>;

    '__tco: loop {
        match stop {
        true => {
            return Ok((inM.clone(), inTypeA))
        },
        false => {
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut m2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut extArg: T;
            let mut extArg1: T;
            let mut extArg2: T;
            let mut eqns: Arc<metamodelica::List<i32>>;
            let mut eqns1: Arc<metamodelica::List<i32>>;
            (eqns, extArg) = func(({let __elt = inM.borrow()[(pos-1) as usize].clone(); __elt}), pos, inTypeA)?;
            eqns1 = List::removeOnTrue(pos, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), eqns.clone())?;
            (m1, extArg1) = traverseAdjacencyMatrixList(eqns1.clone(), inM.clone(), func.clone(), metamodelica::arrayLength(inM.clone()), pos, extArg.clone())?;
            { (inM, func, pos, len, stop, inTypeA) = (m1.clone(), func.clone(), pos + 1, len, intGt(pos + 1, len), extArg1.clone()); continue '__tco; }
        },
    }
    }
}

fn traverseAdjacencyMatrixList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inLst: Arc<metamodelica::List<i32>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut len: i32, mut maxpos: i32, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = 'mc: {
        let __mc_input = inLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inM.clone(), inTypeA.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: pos, tail: rest } => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut extArg: T;
                    let mut extArg1: T;
                    let mut eqns: Arc<metamodelica::List<i32>>;
                    let mut eqns1: Arc<metamodelica::List<i32>>;
                    let mut alleqns: Arc<metamodelica::List<i32>>;
                    let true = (intLt(pos.clone(), len + 1)) else { bail!("pattern mismatch") };
                    let true = (intLt(pos.clone(), maxpos)) else { bail!("pattern mismatch") };
                    (eqns, extArg) = func(({let __elt = inM.borrow()[(pos.clone()-1) as usize].clone(); __elt}), pos.clone(), inTypeA.clone())?;
                    eqns1 = List::removeOnTrue(maxpos, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), eqns.clone())?;
                    alleqns = List::unionOnTrueList(list![rest.clone(), eqns1.clone()], (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (m, extArg1) = traverseAdjacencyMatrixList(alleqns.clone(), inM.clone(), func.clone(), len, maxpos, extArg.clone())?;
                    Ok((m.clone(), extArg1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: pos, tail: rest } => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut extArg: T;
                    let true = (intLt(pos.clone(), len + 1)) else { bail!("pattern mismatch") };
                    (m, extArg) = traverseAdjacencyMatrixList(rest.clone(), inM.clone(), func.clone(), len, maxpos, inTypeA.clone())?;
                    Ok((m.clone(), extArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- BackendDAEOptimize.traverseAdjacencyMatrixList failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outM, outTypeA))
}

pub(crate) fn getOtherEqSysAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut size: i32, mut index: i32, mut skip: metamodelica::Array<i32>, mut rowskip: metamodelica::Array<i32>, mut mnew: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    '__tco: loop {
        match m.clone() {
        _ if (intGt(index, size)) => {
            return Ok(mnew.clone())
        },
        _ if (intGt(({let __elt = skip.borrow()[(index-1) as usize].clone(); __elt}), 0)) => {
            let mut row: Arc<metamodelica::List<i32>>;
            row = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut r in (({let __elt = m.borrow()[(index-1) as usize].clone(); __elt})).into_iter().cloned() {
            if !(intGt(r.clone(), 0) && intGt(({let __elt = rowskip.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0)) { continue; }
            let __x = r.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            metamodelica::arrayUpdate(mnew.clone(), index, row.clone())?;
            { (m, size, index, skip, rowskip, mnew) = (m.clone(), size, index + 1, skip.clone(), rowskip.clone(), mnew.clone()); continue '__tco; }
        },
        _ => {
            metamodelica::arrayUpdate(mnew.clone(), index, metamodelica::nil())?;
            { (m, size, index, skip, rowskip, mnew) = (m.clone(), size, index + 1, skip.clone(), rowskip.clone(), mnew.clone()); continue '__tco; }
        },
    }
    }
}

fn isAssigned(mut ass: metamodelica::Array<i32>, mut i: i32) -> bool {
    let mut b: bool;
    b = intGt(({let __elt = ass.borrow()[(i-1) as usize].clone(); __elt}), 0);
    b
}

pub fn transposeAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nRowsMt: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut i: i32 = 1;
    mt = arrayCreate(nRowsMt, metamodelica::nil());
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        (mt, i) = transposeRow(e.clone(), mt.clone(), i)?;
    }
    Ok(mt)
}

fn transposeRow(mut row: Arc<metamodelica::List<i32>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut indx: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)> {
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>> = mt;
    let mut indx: i32 = indx;
    (mt, indx) = (::match_deref::match_deref! { match &(row) {
        Deref @ metamodelica::List::Nil => {
            (mt.clone(), indx + 1)
        },
        Deref @ metamodelica::List::Cons { head: i, tail: res } => {
            let mut indx1: i32;
            let mut iabs: i32;
            let mut col: Arc<metamodelica::List<i32>>;
            iabs = intAbs(i.clone());
            mt = Array::expand(iabs.clone() - metamodelica::arrayLength(mt.clone()), mt.clone(), metamodelica::nil())?;
            col = ({let __elt = mt.borrow()[(iabs.clone()-1) as usize].clone(); __elt});
            indx1 = if (intLt(i.clone(), 0)) {-(indx)} else {indx};
            metamodelica::arrayUpdate(mt.clone(), iabs.clone(), metamodelica::cons(indx1.clone(), col.clone()))?;
            transposeRow(res.clone(), mt.clone(), indx)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((mt, indx))
}

pub(crate) fn absAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut res: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut i: i32 = 1;
    let mut minn: i32;
    res = metamodelica::arrayCreate(metamodelica::arrayLength(m.clone()), metamodelica::nil());
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut v in __range0 {
        minn = List::fold(v.clone(), (std::sync::Arc::new(fnptr!(intMin, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)?;
        if minn < 0 {
            unsafe { metamodelica::Dangerous::arrayInitSlot(res.clone(), i, List::map(v.clone(), Arc::new(fnptr!(intAbs, i32)))?) };
        } else {
            unsafe { metamodelica::Dangerous::arrayInitSlot(res.clone(), i, v.clone()) };
        }
        i = i + 1;
    }
    Ok(res)
}

pub(crate) fn isEmpty(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> bool {
    let mut b: bool = true;
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut element in __range0 {
        if !(element.clone().is_empty()) {
            b = false;
            return b.clone();
        }
    }
    b
}

