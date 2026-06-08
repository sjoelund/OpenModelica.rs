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

use crate::SBInterval;
use crate::UnorderedSet;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBMultiInterval {
    pub intervals: metamodelica::Array<Arc<SBInterval::SBInterval>>,
    pub ndim: i32,
}

impl Default for SBMultiInterval {
    fn default() -> Self {
        Self {
            intervals: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type MULTI_INTERVAL = SBMultiInterval;

pub fn newEmpty() -> Arc<SBMultiInterval> {
    let mut mi: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    mi = Arc::new(SBMultiInterval { intervals: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), ndim: 0 });
    mi
}

pub fn copy(mut mi: Arc<SBMultiInterval>) -> Arc<SBMultiInterval> {
    let mut outMI: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    outMI = Arc::new(SBMultiInterval { intervals: metamodelica::arrayFromVec(mi.intervals.clone().borrow().clone()), ndim: mi.ndim.clone() });
    outMI
}

pub fn fromList(mut ints: Arc<metamodelica::List<Arc<SBInterval::SBInterval>>>) -> Result<Arc<SBMultiInterval>> {
    let mut outMI: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    if List::any(ints.clone(), (std::sync::Arc::new(fnptr!(SBInterval::isEmpty, Arc<SBInterval::SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>) -> Result<bool> + 'static>))? {
        outMI = newEmpty();
    } else {
        outMI = Arc::new(SBMultiInterval { intervals: metamodelica::arrayFromVec(ints.clone().into_iter().cloned().collect()), ndim: (ints.clone().len() as i32) });
    }
    Ok(outMI)
}

pub fn fromArray(mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>>) -> Result<Arc<SBMultiInterval>> {
    let mut outMI: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    if Array::any(ints.clone(), (std::sync::Arc::new(fnptr!(SBInterval::isEmpty, Arc<SBInterval::SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>) -> Result<bool> + 'static>))? {
        outMI = newEmpty();
    } else {
        outMI = Arc::new(SBMultiInterval { intervals: metamodelica::arrayFromVec(ints.clone().borrow().clone()), ndim: metamodelica::arrayLength(ints.clone()) });
    }
    Ok(outMI)
}

pub fn isEmpty(mut mi: Arc<SBMultiInterval>) -> bool {
    let mut empty: bool = false;
    empty = mi.intervals.clone().borrow().is_empty();
    empty
}

pub fn contains(mut vals: metamodelica::Array<i32>, mut mi: Arc<SBMultiInterval>) -> Result<bool> {
    let mut res: bool = false;
    if metamodelica::arrayLength(vals.clone()) != mi.ndim.clone() {
        res = false;
    } else {
        res = Array::isEqualOnTrue(vals.clone(), mi.intervals.clone(), (std::sync::Arc::new(fnptr!(SBInterval::contains, i32, Arc<SBInterval::SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<SBInterval::SBInterval>) -> Result<bool> + 'static>))?;
    }
    Ok(res)
}

pub fn intersection(mut mi1: Arc<SBMultiInterval>, mut mi2: Arc<SBMultiInterval>) -> Result<Arc<SBMultiInterval>> {
    let mut outMI: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>> = Default::default();
    let mut ires: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    if mi1.ndim.clone() != mi2.ndim.clone() || isEmpty(mi1.clone()) {
        outMI = newEmpty();
        return Ok(outMI.clone());
    }
    ints = metamodelica::arrayCreate(mi1.ndim.clone(), metamodelica::arrayGet(mi1.intervals.clone(), 1)?);
    for mut i in 1..=metamodelica::arrayLength(ints.clone()) {
        ires = SBInterval::intersection(metamodelica::arrayGet(mi1.intervals.clone(), i.clone())?, metamodelica::arrayGet(mi2.intervals.clone(), i.clone())?);
        if SBInterval::isEmpty(ires.clone()) {
            outMI = newEmpty();
            return Ok(outMI.clone());
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(ints.clone(), i.clone(), ires.clone()) };
    }
    outMI = fromArray(ints.clone())?;
    Ok(outMI)
}

pub fn complement(mut mi1: Arc<SBMultiInterval>, mut mi2: Arc<SBMultiInterval>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>>> {
    fn add_interval(mut i: Arc<SBInterval::SBInterval>, mut count: i32, mut size: i32, mut ints1: metamodelica::Array<Arc<SBInterval::SBInterval>>, mut ints2: metamodelica::Array<Arc<SBInterval::SBInterval>>, mut res: Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>>> {
        let mut res: Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>> = res;
        let mut dummyi: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
        let mut resi: metamodelica::Array<Arc<SBInterval::SBInterval>> = Default::default();
        if !(SBInterval::isEmpty(i.clone())) {
            resi = metamodelica::arrayCreate(size.clone(), dummyi.clone());
            Array::copyN(ints1.clone(), resi.clone(), count.clone(), 0, 0)?;
            {
                let __cell0 = i.clone();
                let __idx0 = count.clone() + 1;
                unsafe { metamodelica::Dangerous::arrayInitSlot(resi.clone().clone(), __idx0, __cell0); }
            }
            Array::copyN(ints2.clone(), resi.clone(), metamodelica::arrayLength(ints2.clone()) - count.clone() - 1, count.clone() + 1, count.clone() + 1)?;
            UnorderedSet::add(fromArray(resi.clone())?, res.clone())?;
        }
        Ok(res)
    }

    let mut res: Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>> = <Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>> as ::std::default::Default>::default();
    let mut tmp_mi: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    let mut dummys: Arc<UnorderedSet::UnorderedSet<Arc<SBInterval::SBInterval>>> = <Arc<UnorderedSet::UnorderedSet<Arc<SBInterval::SBInterval>>> as ::std::default::Default>::default();
    let mut diffs: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<SBInterval::SBInterval>>>> = Default::default();
    let mut count: i32 = 0;
    let mut mi1_size: i32 = 0;
    let mut resi: metamodelica::Array<Arc<SBInterval::SBInterval>> = Default::default();
    res = UnorderedSet::new((std::sync::Arc::new(fnptr!(hash, Arc<SBMultiInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBMultiInterval>) -> Result<i32> + 'static>), (std::sync::Arc::new(isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBMultiInterval>, Arc<SBMultiInterval>) -> Result<bool> + 'static>), 13);
    if isEmpty(mi1.clone()) || mi1.ndim.clone() != mi2.ndim.clone() {
        return Ok(res.clone());
    }
    tmp_mi = intersection(mi1.clone(), mi2.clone())?;
    if isEmpty(tmp_mi.clone()) {
        UnorderedSet::add(mi1.clone(), res.clone())?;
        return Ok(res.clone());
    }
    if isEqual(mi1.clone(), tmp_mi.clone())? {
        return Ok(res.clone());
    }
    mi1_size = metamodelica::arrayLength(mi1.intervals.clone());
    diffs = metamodelica::arrayCreate(mi1_size.clone(), dummys.clone());
    for mut i in 1..=mi1_size.clone() {
        {
            let __cell0 = SBInterval::complement(metamodelica::Dangerous::arrayGetNoBoundsChecking(mi1.intervals.clone(), i.clone()), metamodelica::arrayGet(tmp_mi.intervals.clone(), i.clone())?)?;
            let __idx0 = i.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(diffs.clone().clone(), __idx0, __cell0); }
        }
    }
    count = 0;
    let __range1 = diffs.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut vdiff in __range1 {
        UnorderedSet::fold(vdiff.clone(), (std::sync::Arc::new({ let __pe_b1 = count.clone(); let __pe_b2 = mi1_size.clone(); let __pe_b3 = tmp_mi.intervals.clone(); let __pe_b4 = mi1.intervals.clone(); move |__pe_a0, __pe_a5| add_interval(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>, Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval>>>> + 'static>), res.clone())?;
        count = count.clone() + 1;
    }
    Ok(res)
}

pub fn crossProd(mut mi1: Arc<SBMultiInterval>, mut mi2: Arc<SBMultiInterval>) -> Result<Arc<SBMultiInterval>> {
    let mut res: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>> = Default::default();
    ints = Array::join(mi1.intervals.clone(), mi2.intervals.clone())?;
    res = Arc::new(SBMultiInterval { intervals: ints.clone(), ndim: metamodelica::arrayLength(ints.clone()) });
    Ok(res)
}

pub fn cardinality(mut mi: Arc<SBMultiInterval>) -> i32 {
    let mut card: i32 = 0;
    for mut i in 1..=mi.ndim.clone() {
        card = card.clone() + SBInterval::cardinality(({let __elt = mi.intervals.borrow()[(i.clone()-1) as usize].clone(); __elt}));
    }
    card
}

pub fn intervals(mut mi: Arc<SBMultiInterval>) -> metamodelica::Array<Arc<SBInterval::SBInterval>> {
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>> = mi.intervals.clone();
    ints
}

pub fn ndim(mut mi: Arc<SBMultiInterval>) -> i32 {
    let mut ndim: i32 = metamodelica::arrayLength(mi.intervals.clone());
    ndim
}

pub fn minElem(mut mi: Arc<SBMultiInterval>) -> Result<metamodelica::Array<i32>> {
    let mut res: metamodelica::Array<i32> = Default::default();
    let __range0 = mi.intervals.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        if SBInterval::isEmpty(i.clone()) {
            res = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            return Ok(res.clone());
        }
    }
    res = Array::map(mi.intervals.clone(), (std::sync::Arc::new(fnptr!(SBInterval::lowerBound, Arc<SBInterval::SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>) -> Result<i32> + 'static>))?;
    Ok(res)
}

pub fn replace(mut i: Arc<SBInterval::SBInterval>, mut dim: i32, mut mi: Arc<SBMultiInterval>) -> Result<Arc<SBMultiInterval>> {
    let mut res: Arc<SBMultiInterval> = Arc::new(<SBMultiInterval as ::std::default::Default>::default());
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>> = Default::default();
    ints = metamodelica::arrayFromVec(mi.intervals.clone().borrow().clone());
    {
        let __cell0 = i.clone();
        let __idx0 = dim.clone();
        ints.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
    }
    res = fromArray(ints.clone())?;
    Ok(res)
}

pub fn isEqual(mut mi1: Arc<SBMultiInterval>, mut mi2: Arc<SBMultiInterval>) -> Result<bool> {
    let mut equal: bool = false;
    equal = Array::isEqualOnTrue(mi1.intervals.clone(), mi2.intervals.clone(), (std::sync::Arc::new(fnptr!(SBInterval::isEqual, Arc<SBInterval::SBInterval>, Arc<SBInterval::SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>, Arc<SBInterval::SBInterval>) -> Result<bool> + 'static>))?;
    Ok(equal)
}

pub fn hash(mut mi: Arc<SBMultiInterval>) -> i32 {
    let mut res: i32 = 0;
    res = metamodelica::arrayLength(mi.intervals.clone());
    res
}

pub fn size(mut mi: Arc<SBMultiInterval>) -> i32 {
    let mut sz: i32 = 1;
    let __range0 = mi.intervals.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        sz = sz.clone() * SBInterval::size(i.clone());
    }
    sz
}

pub fn toString(mut mi: Arc<SBMultiInterval>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    if isEmpty(mi.clone()) {
        r#str = (literal!("emptyInterval")).clone();
    } else {
        r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (mi.intervals.clone()).borrow().iter() {
            let __x = SBInterval::toString(i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("x")).clone());
    }
    r#str
}


