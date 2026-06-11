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
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// An implementation of a generic unordered set, a.k.a. hash set.
///
///   This implementation uses separate chaining and automatically rehashes the set
///   when the load factor becomes too large to keep the performance up.
#[derive(Clone, metamodelica::ReferenceEq)]
pub struct UnorderedSet<T: Clone> {
    pub buckets: Mutable::Mutable<metamodelica::Array<Arc<metamodelica::List<T>>>>,
    pub size: Mutable::Mutable<i32>,
    pub hashFn: Hash<T>,
    pub eqFn: KeyEq<T>,
}

impl<T: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace for UnorderedSet<T> {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.buckets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.size, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hashFn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqFn, __mmv)?;
        Ok(())
    }
}
impl<T: Clone + 'static + PartialEq> PartialEq for UnorderedSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.buckets == other.buckets && self.size == other.size && std::sync::Arc::ptr_eq((&self.hashFn), (&other.hashFn)) && std::sync::Arc::ptr_eq((&self.eqFn), (&other.eqFn))
    }
}
impl<T: Clone + 'static + PartialEq + Eq> Eq for UnorderedSet<T> {}
impl<T: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> PartialOrd for UnorderedSet<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> Ord for UnorderedSet<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.buckets.cmp(&other.buckets).then_with(|| self.size.cmp(&other.size).then_with(|| (std::sync::Arc::as_ptr((&self.hashFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.hashFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.eqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.eqFn)) as *const ())))))
    }
}
impl<T: Clone + 'static + std::fmt::Debug> std::fmt::Debug for UnorderedSet<T> {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("UnorderedSet");
        __ds.field("buckets", &self.buckets);
        __ds.field("size", &self.size);
        __ds.field("hashFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.hashFn))));
        __ds.field("eqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.eqFn))));
        __ds.finish()
    }
}

impl<T: Clone + 'static + metamodelica::gc::MMTrace> Default for UnorderedSet<T> {
    fn default() -> Self {
        Self {
            buckets: Default::default(),
            size: Default::default(),
            hashFn: { let __placeholder: Hash<T> = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder },
            eqFn: { let __placeholder: KeyEq<T> = std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called")); __placeholder },
        }
    }
}

pub type UNORDERED_SET<T> = UnorderedSet<T>;

pub type Hash<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>;

pub type KeyEq<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

pub fn new<T: Clone + 'static + metamodelica::gc::MMTrace>(mut hash: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEq: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut bucketCount: i32) -> Arc<UnorderedSet<T>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut buckets: Mutable::Mutable<metamodelica::Array<Arc<metamodelica::List<T>>>>;
    buckets = Mutable::create(arrayCreate(bucketCount, metamodelica::nil()));
    set = Arc::new(UnorderedSet { buckets: buckets, size: Mutable::create(0), hashFn: hash.clone(), eqFn: keyEq.clone() });
    set
}

pub fn fromList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut elements: Arc<metamodelica::List<T>>, mut hash: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEq: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    set = new(hash.clone(), keyEq.clone(), Util::nextPrime((elements.clone().len() as i32)));
    for mut e in &*elements {
        let mut e = e.clone();
        add(e.clone(), set.clone())?;
    }
    Ok(set)
}

pub fn copy<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> Arc<UnorderedSet<T>> {
    let mut outSet: Arc<UnorderedSet<T>>;
    outSet = Arc::new(UnorderedSet { buckets: Mutable::create(metamodelica::arrayFromVec(Mutable::access(set.buckets.clone()).borrow().clone())), size: Mutable::create(Mutable::access(set.size.clone())), hashFn: set.hashFn.clone(), eqFn: set.eqFn.clone() });
    outSet
}

pub fn add<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut hash: i32;
    let mut okey: Option<T>;
    (okey, hash) = find(key.clone(), set.clone())?;
    if isNone(okey) {
        addKey(key, hash, set)?;
    }
    Ok(())
}

pub fn addNew<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut hash: i32;
    hash = intMod(hashfn(key.clone())?, metamodelica::arrayLength(Mutable::access(set.buckets.clone())));
    addKey(key, hash, set)?;
    Ok(())
}

pub fn addUnique<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut hash: i32;
    let __pa0 = ::match_deref::match_deref! { match &(find(key.clone(), set.clone())?) {
        (None, __pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    hash = __pa0.clone();
    addKey(key, hash, set)?;
    Ok(())
}

pub fn remove<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut removed: bool;
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>> = Mutable::access(set.buckets.clone());
    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut eqfn: KeyEq<T> = set.eqFn.clone();
    let mut hash: i32;
    let mut bucket: Arc<metamodelica::List<T>>;
    let mut okey: Option<T>;
    hash = intMod(hashfn(key.clone())?, metamodelica::arrayLength(buckets.clone()));
    bucket = metamodelica::arrayGet(buckets.clone(), hash + 1)?;
    (bucket, okey) = List::deleteMemberOnTrue(key, bucket, eqfn.clone())?;
    removed = isSome(okey);
    if removed {
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(buckets.clone(), hash + 1, bucket);
        Mutable::update(set.size.clone(), Mutable::access(set.size.clone()) - 1);
    }
    Ok(removed)
}

pub(crate) fn get<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<Option<T>> {
    let mut outKey: Option<T>;
    (outKey, _) = find(key, set)?;
    Ok(outKey)
}

pub(crate) fn getOrFail<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<T> {
    let mut outKey: T;
    let mut okey: Option<T>;
    (okey, _) = find(key, set)?;
    let __pa0 = ::match_deref::match_deref! { match &(okey) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outKey = __pa0.clone();
    Ok(outKey)
}

pub fn contains<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut res: bool;
    res = isSome((find(key, set)?).0);
    Ok(res)
}

pub fn first<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> Result<T> {
    let mut val: T;
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            val = k.clone();
            return Ok(val.clone());
        }
    }
    bail!("fail");
    Ok(val)
}

pub fn isEqual<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut equal: bool = true;
    if Mutable::access(set1.size.clone()) != Mutable::access(set2.size.clone()) {
        equal = false;
        return Ok(equal.clone());
    }
    let __range0 = Mutable::access(set1.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set2.clone())?) {
                equal = false;
                return Ok(equal.clone());
            }
        }
    }
    Ok(equal)
}

pub fn toList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            outList = metamodelica::cons(k.clone(), outList.clone());
        }
    }
    outList
}

pub fn toArray<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(mut set: Arc<UnorderedSet<T>>) -> metamodelica::Array<T> {
    let mut outArray: metamodelica::Array<T>;
    let mut dummy: T;
    let mut i: i32 = 1;
    outArray = metamodelica::arrayCreateDefault(Mutable::access(set.size.clone()));
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i, k.clone()) };
            i = i + 1;
        }
    }
    outArray
}

pub fn fold<T: Clone + 'static + metamodelica::gc::MMTrace, FT: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>, mut startValue: FT) -> Result<FT> {
    pub type FoldFn<T: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>;

    let mut result: FT = startValue.clone();
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            result = r#fn(k.clone(), result.clone())?;
        }
    }
    Ok(result)
}

pub fn apply<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> Result<()> {
    pub type ApplyFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut eqfn: KeyEq<T> = set.eqFn.clone();
    let mut bucket_count: i32;
    let mut hash: i32;
    let mut size: i32 = 0;
    let mut new_buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    let mut newKey: T;
    let mut bucket: Arc<metamodelica::List<T>>;
    let mut duplicate: bool;
    bucket_count = Util::nextPrime(Mutable::access(set.size.clone()));
    new_buckets = arrayCreate(bucket_count, metamodelica::nil());
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            newKey = r#fn(k.clone())?;
            hash = intMod(hashfn(newKey.clone())?, bucket_count);
            bucket = metamodelica::arrayGet(new_buckets.clone(), hash + 1)?;
            duplicate = false;
            for mut nk in &*bucket.clone() {
                let mut nk = nk.clone();
                if eqfn(nk.clone(), newKey.clone())? {
                    duplicate = true;
                    break;
                }
            }
            if !(duplicate) {
                metamodelica::arrayUpdate(new_buckets.clone(), hash + 1, metamodelica::cons(newKey.clone(), bucket.clone()))?;
                size = size + 1;
            }
        }
    }
    Mutable::update(set.buckets.clone(), new_buckets.clone());
    Mutable::update(set.size.clone(), size);
    Ok(())
}

pub fn all<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut res: bool;
    if isEmpty(set.clone()) {
        res = true;
        return Ok(res.clone());
    }
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(r#fn(k.clone())?) {
                res = false;
                return Ok(res.clone());
            }
        }
    }
    res = true;
    Ok(res)
}

pub(crate) fn any<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut res: bool;
    if isEmpty(set.clone()) {
        res = false;
        return Ok(res.clone());
    }
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if r#fn(k.clone())? {
                res = true;
                return Ok(res.clone());
            }
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn none<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut res: bool;
    if isEmpty(set.clone()) {
        res = true;
        return Ok(res.clone());
    }
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if r#fn(k.clone())? {
                res = false;
                return Ok(res.clone());
            }
        }
    }
    res = true;
    Ok(res)
}

pub fn filterOnFalse<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<Arc<UnorderedSet<T>>> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut falseSet: Arc<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(r#fn(k.clone())?) {
                add(k.clone(), falseSet.clone())?;
            }
        }
    }
    Ok(falseSet)
}

pub fn splitOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(Arc<UnorderedSet<T>>, Arc<UnorderedSet<T>>)> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut trueSet: Arc<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let mut falseSet: Arc<UnorderedSet<T>> = new(set.hashFn.clone(), set.eqFn.clone(), 13);
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            add(k.clone(), if (r#fn(k.clone())?) {trueSet.clone()} else {falseSet.clone()})?;
        }
    }
    Ok((trueSet, falseSet))
}

pub fn size<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> i32 {
    let mut s: i32 = Mutable::access(set.size.clone());
    s
}

pub fn isEmpty<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> bool {
    let mut empty: bool = Mutable::access(set.size.clone()) == 0;
    empty
}

pub(crate) fn bucketCount<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> i32 {
    let mut count: i32 = metamodelica::arrayLength(Mutable::access(set.buckets.clone()));
    count
}

pub(crate) fn loadFactor<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> metamodelica::Real {
    let mut load: metamodelica::Real = intReal(Mutable::access(set.size.clone())) / metamodelica::OrderedFloat((bucketCount(set.clone())) as f64);
    load
}

pub(crate) fn rehash<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut old_buckets: metamodelica::Array<Arc<metamodelica::List<T>>> = Mutable::access(set.buckets.clone());
    let mut new_buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    let mut bucket_count: i32;
    let mut hash: i32;
    let mut hashfn: Hash<T> = set.hashFn.clone();
    bucket_count = Util::nextPrime(Mutable::access(set.size.clone()) * 2);
    new_buckets = arrayCreate(bucket_count, metamodelica::nil());
    let __range0 = old_buckets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            hash = intMod(hashfn(k.clone())?, bucket_count);
            metamodelica::arrayUpdate(new_buckets.clone(), hash + 1, metamodelica::cons(k.clone(), metamodelica::arrayGet(new_buckets.clone(), hash + 1)?))?;
        }
    }
    Mutable::update(set.buckets.clone(), new_buckets.clone());
    Ok(())
}

pub fn toString<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(mut set: Arc<UnorderedSet<T>>, mut stringFn: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut delimiter: ArcStr) -> Result<ArcStr> {
    pub type StringFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut k in (toArray(set)).borrow().iter() {
            let __x = stringFn(k.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (delimiter).clone());
    Ok(r#str)
}

pub(crate) fn dump<T: Clone + 'static + metamodelica::gc::MMTrace + Default>(mut set: Arc<UnorderedSet<T>>, mut stringFn: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<()> {
    pub type StringFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    metamodelica::print((toString(set, stringFn.clone(), (literal!("\n")).clone())?).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub fn unique_list<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList: Arc<metamodelica::List<T>>, mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = if (List::hasSeveralElements(inList.clone())) {toList(fromList(inList.clone(), hashFunc.clone(), keyEqFunc.clone())?)} else {inList.clone()};
    Ok(outList)
}

pub fn union<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set = set1;
        buckets = Mutable::access(set2.buckets.clone());
    } else {
        set = set2;
        buckets = Mutable::access(set1.buckets.clone());
    }
    let __range0 = buckets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            add(k.clone(), set.clone())?;
        }
    }
    Ok(set)
}

pub fn union_list<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set_lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>, mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut rest: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>;
    if set_lst.clone().is_empty() {
        set = new(hashFunc.clone(), keyEqFunc.clone(), 13);
    } else {
        (set, rest) = extractFromLst(set_lst, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        for mut tmp in &*rest {
            let mut tmp = tmp.clone();
            set = union(set.clone(), tmp.clone())?;
        }
    }
    Ok(set)
}

pub fn merge<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set1: Arc<UnorderedSet<T>> = set1;
    let __range0 = Mutable::access(set2.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            add(k.clone(), set1.clone())?;
        }
    }
    Ok(set1)
}

pub(crate) fn intersection<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut set_small: Arc<UnorderedSet<T>>;
    let mut set_big: Arc<UnorderedSet<T>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set_small = set2;
        set_big = set1.clone();
    } else {
        set_small = set1.clone();
        set_big = set2;
    }
    let __range0 = Mutable::access(set_small.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if contains(k.clone(), set_big.clone())? {
                acc = metamodelica::cons(k.clone(), acc.clone());
            }
        }
    }
    set = fromList(acc, set1.hashFn.clone(), set1.eqFn.clone())?;
    Ok(set)
}

pub(crate) fn intersection_list<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set_lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>, mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut set_small: Arc<UnorderedSet<T>>;
    let mut rest: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    if !(set_lst.clone().is_empty()) {
        (set_small, rest) = extractFromLst(set_lst, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        let __range0 = Mutable::access(set_small.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b.clone() {
                let mut k = k.clone();
                if List::all(rest.clone(), (std::sync::Arc::new({ let __pe_b0 = k.clone(); move |__pe_a1| contains(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>))? {
                    acc = metamodelica::cons(k.clone(), acc.clone());
                }
            }
        }
    }
    set = fromList(acc, hashFunc.clone(), keyEqFunc.clone())?;
    Ok(set)
}

pub fn difference_list<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut set2: Arc<UnorderedSet<T>>;
    let mut lst1: Arc<metamodelica::List<T>> = inList1.clone();
    let mut lst2: Arc<metamodelica::List<T>> = inList2.clone();
    while !(lst1.clone().is_empty() || lst2.clone().is_empty()) && keyEqFunc(listHead(lst1.clone())?, listHead(lst2.clone())?)? {
        lst1 = listRest(lst1.clone())?;
        lst2 = listRest(lst2.clone())?;
    }
    if lst1.clone().is_empty() || lst2.clone().is_empty() {
        acc = lst1;
        return Ok(acc.clone());
    }
    set2 = fromList(lst2, hashFunc.clone(), keyEqFunc.clone())?;
    for mut k in &*lst1 {
        let mut k = k.clone();
        if !(contains(k.clone(), set2.clone())?) {
            acc = metamodelica::cons(k.clone(), acc.clone());
        }
    }
    Ok(acc)
}

pub fn equal_list<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut hashFunc: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>, mut keyEqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<bool> {
    let mut b: bool = false;
    let mut set1: Arc<UnorderedSet<T>> = fromList(inList1.clone(), hashFunc.clone(), keyEqFunc.clone())?;
    let mut set2: Arc<UnorderedSet<T>> = fromList(inList2.clone(), hashFunc.clone(), keyEqFunc.clone())?;
    if Mutable::access(set1.size.clone()) != Mutable::access(set2.size.clone()) {
        return Ok(b.clone());
    }
    for mut k in &*inList1 {
        let mut k = k.clone();
        if !(contains(k.clone(), set2.clone())?) {
            return Ok(b.clone());
        }
    }
    b = true;
    Ok(b)
}

pub fn difference<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    let __range0 = Mutable::access(set1.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set2.clone())?) {
                acc = metamodelica::cons(k.clone(), acc.clone());
            }
        }
    }
    set = fromList(acc, set1.hashFn.clone(), set1.eqFn.clone())?;
    Ok(set)
}

pub fn sym_difference<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    let __range0 = Mutable::access(set1.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set2.clone())?) {
                acc = metamodelica::cons(k.clone(), acc.clone());
            }
        }
    }
    let __range1 = Mutable::access(set2.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range1 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set1.clone())?) {
                acc = metamodelica::cons(k.clone(), acc.clone());
            }
        }
    }
    set = fromList(acc, set1.hashFn.clone(), set1.eqFn.clone())?;
    Ok(set)
}

pub fn isDisjoint<T: Clone + 'static + metamodelica::gc::MMTrace>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut b: bool = true;
    let mut set_small: Arc<UnorderedSet<T>>;
    let mut set_big: Arc<UnorderedSet<T>>;
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set_small = set2;
        set_big = set1;
    } else {
        set_small = set1;
        set_big = set2;
    }
    let __range0 = Mutable::access(set_small.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut buckets in __range0 {
        for mut k in &*buckets.clone() {
            let mut k = k.clone();
            if contains(k.clone(), set_big.clone())? {
                b = false;
                return Ok(b.clone());
            }
        }
    }
    Ok(b)
}

fn find<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<(Option<T>, i32)> {
    let mut outKey: Option<T> = None;
    let mut hash: i32;
    let mut hashfn: Hash<T> = set.hashFn.clone();
    let mut eqfn: KeyEq<T> = set.eqFn.clone();
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>> = Mutable::access(set.buckets.clone());
    let mut bucket: Arc<metamodelica::List<T>>;
    hash = intMod(hashfn(key.clone())?, metamodelica::arrayLength(buckets.clone()));
    bucket = metamodelica::arrayGet(buckets.clone(), hash + 1)?;
    for mut k in &*bucket {
        let mut k = k.clone();
        if eqfn(k.clone(), key.clone())? {
            outKey = Some(k.clone());
            break;
        }
    }
    Ok((outKey, hash))
}

fn addKey<T: Clone + 'static + metamodelica::gc::MMTrace>(mut key: T, mut hash: i32, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    let mut h: i32;
    let mut hashfn: Hash<T>;
    if loadFactor(set.clone()) > metamodelica::OrderedFloat((1) as f64) {
        rehash(set.clone())?;
        hashfn = set.hashFn.clone();
        buckets = Mutable::access(set.buckets.clone());
        h = intMod(hashfn(key.clone())?, metamodelica::arrayLength(buckets.clone()));
    } else {
        buckets = Mutable::access(set.buckets.clone());
        h = hash;
    }
    metamodelica::arrayUpdate(buckets.clone(), h + 1, metamodelica::cons(key, metamodelica::arrayGet(buckets.clone(), h + 1)?))?;
    Mutable::update(set.size.clone(), Mutable::access(set.size.clone()) + 1);
    Ok(())
}

fn extractFromLst<T: Clone + 'static + metamodelica::gc::MMTrace>(mut lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>, mut func: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>) -> Result<(Arc<UnorderedSet<T>>, Arc<metamodelica::List<Arc<UnorderedSet<T>>>>)> {
    type size_compare = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

    let mut single: Arc<UnorderedSet<T>>;
    let mut rest: Arc<metamodelica::List<Arc<UnorderedSet<T>>>> = metamodelica::nil();
    let mut size: i32;
    let mut tmp_lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    single = __pa0.clone();
    tmp_lst = __pa1.clone();
    size = Mutable::access(single.size.clone());
    for mut tmp in &*tmp_lst {
        let mut tmp = tmp.clone();
        if func(Mutable::access(tmp.size.clone()), size)? {
            size = Mutable::access(tmp.size.clone());
            rest = metamodelica::cons(single.clone(), rest.clone());
            single = tmp.clone();
        } else {
            rest = metamodelica::cons(tmp.clone(), rest.clone());
        }
    }
    Ok((single, rest))
}


