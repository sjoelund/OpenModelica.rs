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

use crate::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::Mutable;

/// Implementation of an expandable array
///
/// This provides a generic implementation of an expandable array. It basically
/// behaves like an ordinary array, which means all elements can get accessed via
/// index. When the array runs out of space, it get automatically resized. It is
/// also possible to delete an element from any position.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ExpandableArray<T: Clone> {
    pub numberOfElements: Mutable::Mutable<i32>,
    pub lastUsedIndex: Mutable::Mutable<i32>,
    pub capacity: Mutable::Mutable<i32>,
    pub data: Mutable::Mutable<metamodelica::Array<Option<T>>>,
}

impl<T: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace for ExpandableArray<T> {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfElements, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lastUsedIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.capacity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.data, __mmv)?;
        Ok(())
    }
}
impl<T: Clone + 'static + metamodelica::gc::MMTrace> Default for ExpandableArray<T> {
    fn default() -> Self {
        Self {
            numberOfElements: Default::default(),
            lastUsedIndex: Default::default(),
            capacity: Default::default(),
            data: Default::default(),
        }
    }
}

pub type EXPANDABLE_ARRAY<T> = ExpandableArray<T>;

pub fn new<T: Clone + 'static + metamodelica::gc::MMTrace>(mut capacity: i32, mut dummy: T) -> Arc<ExpandableArray<T>> {
    let mut exarray: Arc<ExpandableArray<T>>;
    exarray = Arc::new(ExpandableArray { numberOfElements: Mutable::create(0), lastUsedIndex: Mutable::create(0), capacity: Mutable::create(capacity), data: Mutable::create(arrayCreate(capacity, None)) });
    exarray
}

pub fn clear<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> Arc<ExpandableArray<T>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut n: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    Mutable::update(exarray.numberOfElements.clone(), 0);
    Mutable::update(exarray.lastUsedIndex.clone(), 0);
    for mut i in 1..=lastUsedIndex {
        if isSome(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), i.clone())) {
            n = n - 1;
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(data.clone(), i.clone(), None);
            if n == 0 {
                return exarray.clone();
            }
        }
    }
    exarray
}

pub fn copy<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inExarray: Arc<ExpandableArray<T>>, mut dummy: T) -> Arc<ExpandableArray<T>> {
    let mut outExarray: Arc<ExpandableArray<T>>;
    outExarray = new(Mutable::access(inExarray.capacity.clone()), dummy);
    assign_field!(
        outExarray.numberOfElements = Mutable::create(Mutable::access(inExarray.numberOfElements.clone())),
        outExarray.lastUsedIndex = Mutable::create(Mutable::access(inExarray.lastUsedIndex.clone())),
        outExarray.capacity = Mutable::create(Mutable::access(inExarray.capacity.clone())),
        outExarray.data = Mutable::create(metamodelica::arrayFromVec(Mutable::access(inExarray.data.clone()).borrow().clone()))
    );
    outExarray
}

pub fn occupied<T: Clone + 'static + metamodelica::gc::MMTrace>(mut index: i32, mut exarray: Arc<ExpandableArray<T>>) -> bool {
    let mut b: bool;
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    b = index >= 1 && index <= lastUsedIndex && isSome(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), index));
    b
}

pub fn get<T: Clone + 'static + metamodelica::gc::MMTrace>(mut index: i32, mut exarray: Arc<ExpandableArray<T>>) -> Result<T> {
    let mut value: T;
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    let true = (index >= 1 && index <= Mutable::access(exarray.lastUsedIndex.clone())) else { bail!("pattern mismatch") };
    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), index)) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    value = __pa0.clone();
    Ok(value)
}

pub fn expandToSize<T: Clone + 'static + metamodelica::gc::MMTrace>(mut minCapacity: i32, mut exarray: Arc<ExpandableArray<T>>) -> Result<Arc<ExpandableArray<T>>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut capacity: i32 = Mutable::access(exarray.capacity.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    if minCapacity > capacity {
        Mutable::update(exarray.capacity.clone(), minCapacity);
        data = Array::expandToSize(minCapacity, data.clone(), None)?;
        Mutable::update(exarray.data.clone(), data.clone());
    }
    Ok(exarray)
}

pub fn set<T: Clone + 'static + metamodelica::gc::MMTrace>(mut index: i32, mut value: T, mut exarray: Arc<ExpandableArray<T>>) -> Result<Arc<ExpandableArray<T>>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut capacity: i32 = Mutable::access(exarray.capacity.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    if index > 0 && (index > capacity || isNone(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), index))) {
        if index > capacity {
            capacity = std::cmp::max(capacity, 1);
            while index > capacity {
                capacity = capacity * 2;
            }
            expandToSize(capacity, exarray.clone())?;
            data = Mutable::access(exarray.data.clone());
        }
        metamodelica::arrayUpdate(data.clone(), index, Some(value))?;
        Mutable::update(exarray.numberOfElements.clone(), numberOfElements + 1);
        if index > lastUsedIndex {
            Mutable::update(exarray.lastUsedIndex.clone(), index);
        }
    } else {
        bail!("fail");
    }
    Ok(exarray)
}

pub fn add<T: Clone + 'static + metamodelica::gc::MMTrace>(mut value: T, mut exarray: Arc<ExpandableArray<T>>) -> Result<(Arc<ExpandableArray<T>>, i32)> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut index: i32;
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    index = lastUsedIndex + 1;
    exarray = set(index, value, exarray)?;
    Ok((exarray, index))
}

pub fn delete<T: Clone + 'static + metamodelica::gc::MMTrace>(mut index: i32, mut exarray: Arc<ExpandableArray<T>>) -> Result<Arc<ExpandableArray<T>>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    if index >= 1 && index <= lastUsedIndex && isSome(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), index)) {
        metamodelica::arrayUpdate(data.clone(), index, None)?;
        Mutable::update(exarray.numberOfElements.clone(), numberOfElements - 1);
        if index == lastUsedIndex {
            lastUsedIndex = lastUsedIndex - 1;
            while lastUsedIndex > 0 && isNone(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), lastUsedIndex)) {
                lastUsedIndex = lastUsedIndex - 1;
            }
            Mutable::update(exarray.lastUsedIndex.clone(), lastUsedIndex);
        }
    } else {
        bail!("fail");
    }
    Ok(exarray)
}

pub fn update<T: Clone + 'static + metamodelica::gc::MMTrace>(mut index: i32, mut value: T, mut exarray: Arc<ExpandableArray<T>>) -> Result<Arc<ExpandableArray<T>>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    if index >= 1 && index <= lastUsedIndex && isSome(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), index)) {
        metamodelica::arrayUpdate(data.clone(), index, Some(value))?;
    } else {
        bail!("fail");
    }
    Ok(exarray)
}

pub fn toList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut listT: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    if numberOfElements == 0 {
        listT = metamodelica::nil();
    } else if lastUsedIndex == 1 {
        listT = list![Util::getOption(({let __elt = data.borrow()[(1-1) as usize].clone(); __elt}))?];
    } else {
        listT = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut i in (1..=lastUsedIndex).into_iter() {
            if !(isSome(({let __elt = data.borrow()[(i.clone()-1) as usize].clone(); __elt}))) { continue; }
            let __x = Util::getOption(({let __elt = data.borrow()[(i.clone()-1) as usize].clone(); __elt}))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    }
    Ok(listT)
}

pub fn compress<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> Arc<ExpandableArray<T>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    let mut i: i32 = 0;
    while lastUsedIndex > numberOfElements {
        i = i + 1;
        if isNone(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), i)) {
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(data.clone(), i, metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), lastUsedIndex));
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(data.clone(), lastUsedIndex, None);
            lastUsedIndex = lastUsedIndex - 1;
            while isNone(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), lastUsedIndex)) {
                lastUsedIndex = lastUsedIndex - 1;
            }
        }
    }
    Mutable::update(exarray.lastUsedIndex.clone(), lastUsedIndex);
    exarray
}

pub(crate) fn shrink<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> Arc<ExpandableArray<T>> {
    let mut exarray: Arc<ExpandableArray<T>> = exarray;
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    let mut newData: metamodelica::Array<Option<T>>;
    exarray = compress(exarray);
    Mutable::update(exarray.capacity.clone(), numberOfElements);
    newData = metamodelica::arrayCreate(numberOfElements, metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), 1));
    for mut i in 1..=numberOfElements {
        unsafe { metamodelica::Dangerous::arrayInitSlot(newData.clone(), i.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), i.clone())) };
    }
    Mutable::update(exarray.data.clone(), newData.clone());
    exarray
}

pub fn toString<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>, mut header: ArcStr, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut debug: bool) -> Result<ArcStr> {
    pub type PrintFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    let mut capacity: i32 = Mutable::access(exarray.capacity.clone());
    let mut value: T;
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    if debug {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*header); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(numberOfElements)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(capacity)); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
    } else {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*header); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(numberOfElements)); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("========================================\n")); ArcStr::from(__mm_s) }).clone();
    if numberOfElements == 0 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("<empty>\n")); ArcStr::from(__mm_s) }).clone();
    } else {
        for mut i in 1..=capacity {
            if isSome(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), i.clone())) {
                let __pa0 = ::match_deref::match_deref! { match &(metamodelica::Dangerous::arrayGetNoBoundsChecking(data.clone(), i.clone())) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                value = __pa0.clone();
                numberOfElements = numberOfElements - 1;
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*func(value.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                if numberOfElements == 0 {
                    return Ok(r#str.clone());
                }
            }
        }
    }
    Ok(r#str)
}

pub fn getNumberOfElements<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> i32 {
    let mut numberOfElements: i32 = Mutable::access(exarray.numberOfElements.clone());
    numberOfElements
}

pub fn getLastUsedIndex<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> i32 {
    let mut lastUsedIndex: i32 = Mutable::access(exarray.lastUsedIndex.clone());
    lastUsedIndex
}

pub(crate) fn getCapacity<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> i32 {
    let mut capacity: i32 = Mutable::access(exarray.capacity.clone());
    capacity
}

pub fn getData<T: Clone + 'static + metamodelica::gc::MMTrace>(mut exarray: Arc<ExpandableArray<T>>) -> metamodelica::Array<Option<T>> {
    let mut data: metamodelica::Array<Option<T>> = Mutable::access(exarray.data.clone());
    data
}


