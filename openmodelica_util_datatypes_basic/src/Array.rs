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

use crate::List;

pub fn mapNoCopy<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> Result<metamodelica::Array<T>> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outArray: metamodelica::Array<T> = inArray.clone();
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(inArray.clone(), i.clone(), inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone()))?);
    }
    Ok(outArray)
}

pub fn mapNoCopy_1<T: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn((T, ArgT)) -> Result<(T, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(metamodelica::Array<T>, ArgT)> {
    pub type FuncType<T: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn((T, ArgT)) -> Result<(T, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<T> = inArray.clone();
    let mut outArg: ArgT = inArg.clone();
    let mut e: T;
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        (e, outArg) = inFunc((metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone()), outArg.clone()))?;
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(inArray.clone(), i.clone(), e.clone());
    }
    Ok((outArray, outArg))
}

fn downheap(mut inArray: metamodelica::Array<i32>, mut n: i32, mut vIn: i32) -> metamodelica::Array<i32> {
    let mut inArray: metamodelica::Array<i32> = inArray;
    let mut v: i32 = vIn.clone();
    let mut w: i32 = 2 * v.clone() + 1;
    let mut tmp: i32 = 0;
    while w.clone() < n.clone() {
        if w.clone() + 1 < n.clone() {
            if ({let __elt = inArray.borrow()[(w.clone() + 2-1) as usize].clone(); __elt}) > ({let __elt = inArray.borrow()[(w.clone() + 1-1) as usize].clone(); __elt}) {
                w = w.clone() + 1;
            }
        }
        if ({let __elt = inArray.borrow()[(v.clone() + 1-1) as usize].clone(); __elt}) >= ({let __elt = inArray.borrow()[(w.clone() + 1-1) as usize].clone(); __elt}) {
            return inArray.clone();
        }
        tmp = ({let __elt = inArray.borrow()[(v.clone() + 1-1) as usize].clone(); __elt});
        {
            let __cell0 = ({let __elt = inArray.borrow()[(w.clone() + 1-1) as usize].clone(); __elt});
            let __idx0 = v.clone() + 1;
            inArray.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
        {
            let __cell1 = tmp.clone();
            let __idx1 = w.clone() + 1;
            inArray.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
        }
        v = w.clone();
        w = 2 * v.clone() + 1;
    }
    inArray
}

pub fn heapSort(mut inArray: metamodelica::Array<i32>) -> metamodelica::Array<i32> {
    let mut inArray: metamodelica::Array<i32> = inArray;
    let mut n: i32 = metamodelica::arrayLength(inArray.clone());
    let mut tmp: i32 = 0;
    for mut v in ({let __s=intDiv(n.clone(), 2) - 1; let __e=0; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        inArray = downheap(inArray.clone(), n.clone(), v.clone());
    }
    for mut v in ({let __s=n.clone(); let __e=2; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        tmp = ({let __elt = inArray.borrow()[(1-1) as usize].clone(); __elt});
        {
            let __cell0 = ({let __elt = inArray.borrow()[(v.clone()-1) as usize].clone(); __elt});
            let __idx0 = 1;
            inArray.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
        {
            let __cell1 = tmp.clone();
            let __idx1 = v.clone();
            inArray.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
        }
        inArray = downheap(inArray.clone(), v.clone() - 1, 0);
    }
    inArray
}

pub fn findFirstOnTrue<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inPredicate: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<Option<T>> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: Option<T> = None;
    outElement = None;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inPredicate(e.clone())? {
            outElement = Some(e.clone());
            break;
        }
    }
    Ok(outElement)
}

pub fn findFirstOnTrueWithIdx<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inPredicate: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(Option<T>, i32)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: Option<T> = None;
    let mut idxOut: i32 = -1;
    let mut idx: i32 = 1;
    outElement = None;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inPredicate(e.clone())? {
            idxOut = idx.clone();
            outElement = Some(e.clone());
            break;
        }
        idx = idx.clone() + 1;
    }
    Ok((outElement, idxOut))
}

pub fn select<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inIndices: Arc<metamodelica::List<i32>>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    let mut i: i32 = 1;
    outArray = metamodelica::arrayCreate((inIndices.clone().len() as i32), ({let __elt = inArray.borrow()[(1-1) as usize].clone(); __elt}));
    for mut e in &*inIndices.clone() {
        let mut e = e.clone();
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), metamodelica::arrayGet(inArray.clone(), e.clone())?) };
        i = i.clone() + 1;
    }
    Ok(outArray)
}

pub fn map<TI: Clone + 'static, TO: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO> = Default::default();
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), 1))?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone()))?) };
        }
    }
    Ok(outArray)
}

pub fn map1<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO> = Default::default();
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), 1), inArg.clone())?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone()), inArg.clone())?) };
        }
    }
    Ok(outArray)
}

pub fn map1Ind<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, i32, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, i32, ArgT) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO> = Default::default();
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), 1), 1, inArg.clone())?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone()), i.clone(), inArg.clone())?) };
        }
    }
    Ok(outArray)
}

pub fn mapList<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO> = Default::default();
    let mut i: i32 = 2;
    let mut len: i32 = (inList.clone().len() as i32);
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(listHead(inList.clone())?)?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut e in &*listRest(inList.clone())? {
            let mut e = e.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(e.clone())?) };
            i = i.clone() + 1;
        }
    }
    Ok(outArray)
}

pub fn fold<T: Clone + 'static, FoldT: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, FoldT) -> Result<FoldT> + 'static>, mut inStartValue: FoldT) -> Result<FoldT> {
    pub type FoldFunc<T: Clone + 'static, FoldT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, FoldT) -> Result<FoldT> + 'static>;

    let mut outResult: FoldT = inStartValue.clone();
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        outResult = inFoldFunc(e.clone(), outResult.clone())?;
    }
    Ok(outResult)
}

pub fn foldIndex<T: Clone + 'static, FoldT: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, i32, FoldT) -> Result<FoldT> + 'static>, mut inStartValue: FoldT) -> Result<FoldT> {
    pub type FoldFunc<T: Clone + 'static, FoldT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, i32, FoldT) -> Result<FoldT> + 'static>;

    let mut outResult: FoldT = inStartValue.clone();
    let mut e: T;
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        e = metamodelica::arrayGet(inArray.clone(), i.clone())?;
        outResult = inFoldFunc(e.clone(), i.clone(), outResult.clone())?;
    }
    Ok(outResult)
}

pub fn reduce<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inReduceFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>) -> Result<T> {
    pub type ReduceFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outResult: T;
    outResult = metamodelica::arrayGet(inArray.clone(), 1)?;
    for mut i in 2..=metamodelica::arrayLength(inArray.clone()) {
        outResult = inReduceFunc(outResult.clone(), metamodelica::arrayGet(inArray.clone(), i.clone())?)?;
    }
    Ok(outResult)
}

pub fn updateIndexFirst<T: Clone + 'static>(mut inIndex: i32, mut inValue: T, mut inArray: metamodelica::Array<T>) -> Result<()> {
    {let _arr = inArray.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = inValue.clone(); _arr};
    Ok(())
}

pub fn getIndexFirst<T: Clone + 'static>(mut inIndex: i32, mut inArray: metamodelica::Array<T>) -> Result<T> {
    let mut outElement: T = metamodelica::arrayGet(inArray.clone(), inIndex.clone())?;
    Ok(outElement)
}

pub fn replaceAtWithFill<T: Clone + 'static>(mut inPos: i32, mut inTypeReplace: T, mut inTypeFill: T, mut inArray: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    outArray = expandToSize(inPos.clone(), inArray.clone(), inTypeFill.clone())?;
    {let _arr = outArray.clone(); _arr.borrow_mut()[(inPos.clone()-1) as usize] = inTypeReplace.clone(); _arr};
    Ok(outArray)
}

pub fn expandToSize<T: Clone + 'static>(mut inNewSize: i32, mut inArray: metamodelica::Array<T>, mut inFill: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    if inNewSize.clone() <= metamodelica::arrayLength(inArray.clone()) {
        outArray = inArray.clone();
    } else {
        outArray = arrayCreate(inNewSize.clone(), inFill.clone());
        copy(inArray.clone(), outArray.clone())?;
    }
    Ok(outArray)
}

pub fn expand<T: Clone + 'static>(mut inN: i32, mut inArray: metamodelica::Array<T>, mut inFill: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    let mut len: i32 = 0;
    if inN.clone() < 1 {
        outArray = inArray.clone();
    } else {
        len = metamodelica::arrayLength(inArray.clone());
        outArray = metamodelica::arrayCreate(len.clone() + inN.clone(), inFill.clone());
        copy(inArray.clone(), outArray.clone())?;
        setRange(len.clone() + 1, len.clone() + inN.clone(), outArray.clone(), inFill.clone())?;
    }
    Ok(outArray)
}

pub fn expandOnDemand<T: Clone + 'static>(mut inNewSize: i32, mut inArray: metamodelica::Array<T>, mut inExpansionFactor: metamodelica::Real, mut inFillValue: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    let mut new_size: i32 = 0;
    let mut len: i32 = metamodelica::arrayLength(inArray.clone());
    if inNewSize.clone() <= len.clone() {
        outArray = inArray.clone();
    } else {
        new_size = ((intReal(len.clone()) * inExpansionFactor.clone()).0.floor() as i32);
        outArray = metamodelica::arrayCreate(new_size.clone(), inFillValue.clone());
        copy(inArray.clone(), outArray.clone())?;
        setRange(len.clone() + 1, new_size.clone(), outArray.clone(), inFillValue.clone())?;
    }
    Ok(outArray)
}

pub fn consToElement<T: Clone + 'static>(mut inIndex: i32, mut inElement: T, mut inArray: metamodelica::Array<Arc<metamodelica::List<T>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<T>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<T>>> = Default::default();
    outArray = {let _arr = inArray.clone(); let _idx = inIndex.clone(); let _val = metamodelica::cons(inElement.clone(), ({let __elt = inArray.borrow()[(inIndex.clone()-1) as usize].clone(); __elt})); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    Ok(outArray)
}

pub fn appendToElement<T: Clone + 'static>(mut inIndex: i32, mut inElements: Arc<metamodelica::List<T>>, mut inArray: metamodelica::Array<Arc<metamodelica::List<T>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<T>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<T>>> = Default::default();
    outArray = {let _arr = inArray.clone(); let _idx = inIndex.clone(); let _val = listAppend(({let __elt = inArray.borrow()[(inIndex.clone()-1) as usize].clone(); __elt}), inElements.clone()); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    Ok(outArray)
}

pub fn appendList<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lst: Arc<metamodelica::List<T>>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    let mut arr_len: i32 = metamodelica::arrayLength(arr.clone());
    let mut lst_len: i32 = 0;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    if lst.clone().is_empty() {
        outArray = arr.clone();
    } else if arr_len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect());
    } else {
        lst_len = (lst.clone().len() as i32);
        outArray = metamodelica::arrayCreate(arr_len.clone() + lst_len.clone(), ({let __elt = arr.borrow()[(1-1) as usize].clone(); __elt}));
        copy(arr.clone(), outArray.clone())?;
        rest = lst.clone();
        for mut i in arr_len.clone() + 1..=arr_len.clone() + lst_len.clone() {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            rest = __pa1.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), e.clone()) };
        }
    }
    Ok(outArray)
}

pub fn join<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    let mut len1: i32 = metamodelica::arrayLength(arr1.clone());
    let mut len2: i32 = metamodelica::arrayLength(arr2.clone());
    if len1.clone() == 0 {
        outArray = metamodelica::arrayFromVec(arr2.clone().borrow().clone());
    } else if len2.clone() == 0 {
        outArray = metamodelica::arrayFromVec(arr1.clone().borrow().clone());
    } else {
        outArray = metamodelica::arrayCreate(len1.clone() + len2.clone(), ({let __elt = arr1.borrow()[(1-1) as usize].clone(); __elt}));
        copyRange(arr1.clone(), outArray.clone(), 1, len1.clone(), 1)?;
        copyRange(arr2.clone(), outArray.clone(), 1, len2.clone(), len1.clone() + 1)?;
    }
    Ok(outArray)
}

pub fn copy<T: Clone + 'static>(mut inArraySrc: metamodelica::Array<T>, mut inArrayDest: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArrayDest.clone();
    if metamodelica::arrayLength(inArraySrc.clone()) > metamodelica::arrayLength(inArrayDest.clone()) {
        bail!("fail");
    }
    for mut i in 1..=metamodelica::arrayLength(inArraySrc.clone()) {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(outArray.clone(), i.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(inArraySrc.clone(), i.clone()));
    }
    Ok(outArray)
}

pub fn copyN<T: Clone + 'static>(mut inArraySrc: metamodelica::Array<T>, mut inArrayDest: metamodelica::Array<T>, mut inN: i32, mut srcOffset: i32, mut dstOffset: i32) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArrayDest.clone();
    if inN.clone() + dstOffset.clone() > metamodelica::arrayLength(inArrayDest.clone()) || inN.clone() + srcOffset.clone() > metamodelica::arrayLength(inArraySrc.clone()) {
        bail!("fail");
    }
    for mut i in 1..=inN.clone() {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(outArray.clone(), i.clone() + dstOffset.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(inArraySrc.clone(), i.clone() + srcOffset.clone()));
    }
    Ok(outArray)
}

pub fn copyRange<T: Clone + 'static>(mut srcArray: metamodelica::Array<T>, mut dstArray: metamodelica::Array<T>, mut srcFirst: i32, mut srcLast: i32, mut dstPos: i32) -> Result<()> {
    let mut offset: i32 = dstPos.clone() - srcFirst.clone();
    if srcFirst.clone() > srcLast.clone() || srcLast.clone() > metamodelica::arrayLength(srcArray.clone()) || offset.clone() + srcLast.clone() > metamodelica::arrayLength(dstArray.clone()) {
        bail!("fail");
    }
    for mut i in srcFirst.clone()..=srcLast.clone() {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(dstArray.clone(), offset.clone() + i.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(srcArray.clone(), i.clone()));
    }
    Ok(())
}

pub fn createIntRange(mut inLen: i32) -> metamodelica::Array<i32> {
    let mut outArray: metamodelica::Array<i32> = Default::default();
    outArray = metamodelica::arrayCreate(inLen.clone(), 0);
    for mut i in 1..=inLen.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), i.clone()) };
    }
    outArray
}

pub fn setRange<T: Clone + 'static>(mut inStart: i32, mut inEnd: i32, mut inArray: metamodelica::Array<T>, mut inValue: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArray.clone();
    if inStart.clone() > metamodelica::arrayLength(inArray.clone()) {
        bail!("fail");
    }
    for mut i in inStart.clone()..=inEnd.clone() {
        {let _arr = inArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = inValue.clone(); _arr};
    }
    Ok(outArray)
}

pub fn getRange<T: Clone + 'static>(mut inStart: i32, mut inEnd: i32, mut inArray: metamodelica::Array<T>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut value: T;
    if inStart.clone() > metamodelica::arrayLength(inArray.clone()) {
        bail!("fail");
    }
    for mut i in inStart.clone()..=inEnd.clone() {
        value = metamodelica::arrayGet(inArray.clone(), i.clone())?;
        outList = metamodelica::cons(value.clone(), outList.clone());
    }
    Ok(outList)
}

pub fn position<T: Clone + 'static + PartialEq>(mut inArray: metamodelica::Array<T>, mut inElement: T, mut inFilledSize: i32) -> i32 {
    let mut outIndex: i32 = 0;
    for mut i in 1..=inFilledSize.clone() {
        if inElement.clone() == ({let __elt = inArray.borrow()[(i.clone()-1) as usize].clone(); __elt}) {
            outIndex = i.clone();
            return outIndex.clone();
        }
    }
    outIndex = 0;
    outIndex
}

pub fn getMemberOnTrue<VT: Clone + 'static, ET: Clone + 'static>(mut inValue: VT, mut inArray: metamodelica::Array<ET>, mut inCompFunc: Arc<dyn ::std::ops::Fn(VT, ET) -> Result<bool> + 'static>) -> Result<(ET, i32)> {
    pub type CompFunc<VT: Clone + 'static, ET: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VT, ET) -> Result<bool> + 'static>;

    let mut outElement: ET;
    let mut outIndex: i32 = 0;
    for mut i in 1..=metamodelica::arrayLength(inArray.clone()) {
        if inCompFunc(inValue.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone()))? {
            outElement = metamodelica::Dangerous::arrayGetNoBoundsChecking(inArray.clone(), i.clone());
            outIndex = i.clone();
            return Ok((outElement.clone(), outIndex.clone()));
        }
    }
    bail!("fail");
    Ok((outElement, outIndex))
}

pub fn reverse<T: Clone + 'static>(mut inArray: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = Default::default();
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    let mut elem1: T;
    let mut elem2: T;
    outArray = inArray.clone();
    size = metamodelica::arrayLength(inArray.clone());
    for mut i in 1..=((metamodelica::OrderedFloat((size.clone()) as f64) / metamodelica::OrderedFloat((2) as f64)).0 as i32) {
        elem1 = metamodelica::arrayGet(inArray.clone(), i.clone())?;
        elem2 = metamodelica::arrayGet(inArray.clone(), size.clone() - i.clone() + 1)?;
        outArray = {let _arr = outArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = elem2.clone(); _arr};
        outArray = {let _arr = outArray.clone(); _arr.borrow_mut()[(size.clone() - i.clone() + 1-1) as usize] = elem1.clone(); _arr};
    }
    Ok(outArray)
}

pub fn toString<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut inNameStr: ArcStr, mut inBeginStr: ArcStr, mut inDelimitStr: ArcStr, mut inEndStr: ArcStr, mut inPrintEmpty: bool, mut maxLength: i32) -> Result<ArcStr> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut endStr: ArcStr = inEndStr.clone();
    if maxLength.clone() > 0 && metamodelica::arrayLength(inArray.clone()) > maxLength.clone() {
        lst = List::firstN(Arc::new(inArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), maxLength.clone())?;
        endStr = stringAppendList(list![(inDelimitStr.clone()).clone(), (literal!("...")).clone(), (inEndStr.clone()).clone()]);
    } else {
        lst = Arc::new(inArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    }
    outString = ((::match_deref::match_deref! { match &((lst.clone(), inPrintEmpty.clone())) {
        (Deref @ metamodelica::List::Nil, true) => {
            stringAppendList(list![(inNameStr.clone()).clone(), (inBeginStr.clone()).clone(), (inEndStr.clone()).clone()])
        },
        (Deref @ metamodelica::List::Nil, false) => {
            inNameStr.clone()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = stringDelimitList(List::map(lst.clone(), inPrintFunc.clone())?, (inDelimitStr.clone()).clone());
            r#str = stringAppendList(list![(inNameStr.clone()).clone(), (inBeginStr.clone()).clone(), (r#str.clone()).clone(), (endStr.clone()).clone()]);
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn isEqual<T: Clone + 'static + PartialEq>(mut inArr1: metamodelica::Array<T>, mut inArr2: metamodelica::Array<T>) -> Result<bool> {
    let mut outIsEqual: bool = true;
    let mut arrLength: i32 = 0;
    arrLength = metamodelica::arrayLength(inArr1.clone());
    if !(intEq(arrLength.clone(), metamodelica::arrayLength(inArr2.clone()))) {
        bail!("fail");
    }
    for mut i in 1..=arrLength.clone() {
        if !(({let __elt = inArr1.borrow()[(i.clone()-1) as usize].clone(); __elt}) == ({let __elt = inArr2.borrow()[(i.clone()-1) as usize].clone(); __elt})) {
            outIsEqual = false;
            break;
        }
    }
    Ok(outIsEqual)
}

pub fn isEqualOnTrue<T1: Clone + 'static, T2: Clone + 'static>(mut arr1: metamodelica::Array<T1>, mut arr2: metamodelica::Array<T2>, mut pred: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut equal: bool = false;
    equal = metamodelica::arrayLength(arr1.clone()) == metamodelica::arrayLength(arr2.clone());
    if !(equal.clone()) {
        return Ok(equal.clone());
    }
    for mut i in 1..=metamodelica::arrayLength(arr1.clone()) {
        if !(pred(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i.clone()), metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i.clone()))?) {
            equal = false;
            return Ok(equal.clone());
        }
    }
    Ok(equal)
}

pub fn allEqual<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut pred: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut equal: bool = true;
    if arr.clone().borrow().is_empty() {
        return Ok(equal.clone());
    }
    for mut i in 2..=metamodelica::arrayLength(arr.clone()) {
        if !(pred(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), 1), metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i.clone()))?) {
            equal = false;
            return Ok(equal.clone());
        }
    }
    Ok(equal)
}

pub fn isLess<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: bool = false;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut e1: T;
    let mut e2: T;
    len1 = metamodelica::arrayLength(arr1.clone());
    len2 = metamodelica::arrayLength(arr2.clone());
    for mut i in 1..=std::cmp::min(len1.clone(), len2.clone()) {
        e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i.clone());
        e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i.clone());
        if lessFn(e1.clone(), e2.clone())? {
            res = true;
            return Ok(res.clone());
        } else if lessFn(e2.clone(), e1.clone())? {
            res = false;
            return Ok(res.clone());
        }
    }
    res = len1.clone() < len2.clone();
    Ok(res)
}

pub fn insertList<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lst: Arc<metamodelica::List<T>>, mut startPos: i32) -> metamodelica::Array<T> {
    let mut arr: metamodelica::Array<T> = arr;
    let mut i: i32 = startPos.clone();
    for mut e in &*lst.clone() {
        let mut e = e.clone();
        {
            let __cell0 = e.clone();
            let __idx0 = i.clone();
            arr.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
        i = i.clone() + 1;
    }
    arr
}

pub fn remove<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut index: i32) -> Result<metamodelica::Array<T>> {
    let mut outArr: metamodelica::Array<T> = Default::default();
    let mut len: i32 = metamodelica::arrayLength(arr.clone());
    let true = (index.clone() <= len.clone() && index.clone() >= 1) else { bail!("pattern mismatch") };
    if len.clone() <= 1 {
        outArr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        outArr = metamodelica::arrayCreate(len.clone() - 1, ({let __elt = arr.borrow()[(1-1) as usize].clone(); __elt}));
        for mut i in 1..=index.clone() - 1 {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArr.clone(), i.clone(), metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i.clone())) };
        }
        for mut i in index.clone() + 1..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArr.clone(), i.clone() - 1, metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i.clone())) };
        }
    }
    Ok(outArr)
}

pub fn all<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if !(inFunc(e.clone())?) {
            outResult = false;
            return Ok(outResult.clone());
        }
    }
    outResult = true;
    Ok(outResult)
}

pub fn any<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inFunc(e.clone())? {
            outResult = true;
            return Ok(outResult.clone());
        }
    }
    outResult = false;
    Ok(outResult)
}

pub fn minElement<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    let mut e: T;
    res = ({let __elt = arr.borrow()[(1-1) as usize].clone(); __elt});
    for mut i in 2..=metamodelica::arrayLength(arr.clone()) {
        e = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i.clone());
        if lessFn(e.clone(), res.clone())? {
            res = e.clone();
        }
    }
    Ok(res)
}

pub fn maxElement<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    let mut e: T;
    res = ({let __elt = arr.borrow()[(1-1) as usize].clone(); __elt});
    for mut i in 2..=metamodelica::arrayLength(arr.clone()) {
        e = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i.clone());
        if lessFn(res.clone(), e.clone())? {
            res = e.clone();
        }
    }
    Ok(res)
}

pub fn compare<T1: Clone + 'static, T2: Clone + 'static>(mut arr1: metamodelica::Array<T1>, mut arr2: metamodelica::Array<T2>, mut compFn: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>) -> Result<i32> {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>;

    let mut res: i32 = 0;
    let mut l1: i32 = 0;
    let mut l2: i32 = 0;
    l1 = metamodelica::arrayLength(arr1.clone());
    l2 = metamodelica::arrayLength(arr2.clone());
    res = if (l1.clone() == l2.clone()) {0} else if (l1.clone() > l2.clone()) {1} else {-1};
    if res.clone() != 0 {
        return Ok(res.clone());
    }
    for mut i in 1..=l1.clone() {
        res = compFn(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i.clone()), metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i.clone()))?;
        if res.clone() != 0 {
            return Ok(res.clone());
        }
    }
    Ok(res)
}

pub fn mapFold<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static>(mut arr: metamodelica::Array<TI>, mut func: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<(TO, ArgT)> + 'static>, mut arg: ArgT) -> Result<(metamodelica::Array<TO>, ArgT)> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<(TO, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<TO> = Default::default();
    let mut outArg: ArgT = arg.clone();
    let mut len: i32 = metamodelica::arrayLength(arr.clone());
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        (res, outArg) = func(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), 1), outArg.clone())?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            (res, outArg) = func(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), i.clone()), outArg.clone())?;
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), res.clone()) };
        }
    }
    Ok((outArray, outArg))
}

pub fn transpose<T: Clone + 'static>(mut arr: metamodelica::Array<metamodelica::Array<T>>) -> metamodelica::Array<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<metamodelica::Array<T>> = Default::default();
    let mut c_len: i32 = 0;
    let mut r_len: i32 = 0;
    let mut val: T;
    let mut row: metamodelica::Array<T> = Default::default();
    if arr.clone().borrow().is_empty() {
        outArray = arr.clone();
        return outArray.clone();
    }
    row = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), 1);
    if row.clone().borrow().is_empty() {
        outArray = arr.clone();
        return outArray.clone();
    }
    val = metamodelica::Dangerous::arrayGetNoBoundsChecking(row.clone(), 1);
    c_len = metamodelica::arrayLength(arr.clone());
    r_len = metamodelica::arrayLength(row.clone());
    outArray = metamodelica::arrayCreate(r_len.clone(), row.clone());
    for mut i in 1..=r_len.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), metamodelica::arrayCreate(c_len.clone(), val.clone())) };
    }
    for mut r in 1..=r_len.clone() {
        for mut c in 1..=c_len.clone() {
            val = metamodelica::Dangerous::arrayGetNoBoundsChecking(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr.clone(), c.clone()), r.clone());
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(metamodelica::Dangerous::arrayGetNoBoundsChecking(outArray.clone(), r.clone()), c.clone(), val.clone());
        }
    }
    outArray
}

pub fn threadMap<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static>(mut arr1: metamodelica::Array<T1>, mut arr2: metamodelica::Array<T2>, mut func: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>) -> Result<metamodelica::Array<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO> = Default::default();
    let mut res: TO;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    if arr1.clone().borrow().is_empty() {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
        return Ok(outArray.clone());
    }
    len1 = metamodelica::arrayLength(arr1.clone());
    len2 = metamodelica::arrayLength(arr2.clone());
    if len1.clone() != len2.clone() {
        bail!("fail");
    }
    res = func(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), 1), metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), 1))?;
    outArray = metamodelica::arrayCreate(len1.clone(), res.clone());
    unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
    for mut i in 2..=len1.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), func(metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i.clone()), metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i.clone()))?) };
    }
    Ok(outArray)
}

pub fn generate<T: Clone + 'static>(mut n: i32, mut generator: Arc<dyn ::std::ops::Fn() -> Result<T> + 'static>) -> Result<metamodelica::Array<T>> {
    pub type Generator<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn() -> Result<T> + 'static>;

    let mut arr: metamodelica::Array<T> = Default::default();
    let mut e: T;
    if n.clone() <= 0 {
        arr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        e = generator()?;
        arr = metamodelica::arrayCreate(n.clone(), e.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), 1, e.clone()) };
        for mut i in 2..=n.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), generator()?) };
        }
    }
    Ok(arr)
}

pub fn filter<T: Clone + 'static + Default>(mut arr: metamodelica::Array<T>, mut fun: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<metamodelica::Array<T>> {
    pub type filterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut new_arr: metamodelica::Array<T> = Default::default();
    let mut new_size: i32 = 0;
    let mut dummy: T;
    let mut index: i32 = 1;
    new_size = metamodelica::arrayLength(arr.clone()) - ({
        let mut __acc: i32 = 0;
        for mut e in (arr.clone()).borrow().iter() {
            if !(fun(e.clone())?) { continue; }
            let __x = 1;
            __acc += __x;
        }
        __acc
    });
    new_arr = metamodelica::arrayCreateDefault(new_size.clone());
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if !(fun(e.clone())?) {
            unsafe { metamodelica::Dangerous::arrayInitSlot(new_arr.clone(), index.clone(), e.clone()) };
            index = index.clone() + 1;
        }
    }
    Ok(new_arr)
}

