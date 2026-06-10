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

use crate::SBAtomicSet;
use crate::SBLinearMap;
use crate::SBPWAtomicLinearMap;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SBPWLinearMap {
    pub dom: metamodelica::Array<Arc<SBSet::SBSet>>,
    pub lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>,
    pub ndim: i32,
}

impl metamodelica::gc::MMTrace for SBPWLinearMap {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.dom, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lmap, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ndim, __mmv)?;
        Ok(())
    }
}
impl Default for SBPWLinearMap {
    fn default() -> Self {
        Self {
            dom: Default::default(),
            lmap: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type PW_LINEAR_MAP = SBPWLinearMap;

pub(crate) fn new(mut dom: metamodelica::Array<Arc<SBSet::SBSet>>, mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>) -> Result<Arc<SBPWLinearMap>> {
    let mut map: Arc<SBPWLinearMap>;
    let mut dim: i32 = 0;
    let mut same_dims: bool = false;
    if metamodelica::arrayLength(dom.clone()) != metamodelica::arrayLength(lmap.clone()) {
        map = newEmpty();
        return Ok(map.clone());
    }
    if !(dom.clone().borrow().is_empty()) {
        dim = SBSet::ndim(({let __elt = dom.borrow()[(1-1) as usize].clone(); __elt}));
        same_dims = Array::all(dom.clone(), (std::sync::Arc::new({ let __pe_b1 = dim.clone(); move |__pe_a0| Ok(SBSet::isDim(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBSet::SBSet>) -> Result<bool> + 'static>))? && Array::all(lmap.clone(), (std::sync::Arc::new({ let __pe_b1 = dim.clone(); move |__pe_a0| Ok(SBLinearMap::isDim(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBLinearMap::SBLinearMap>) -> Result<bool> + 'static>))?;
    }
    if !(same_dims.clone()) {
        map = newEmpty();
    } else {
        map = Arc::new(SBPWLinearMap { dom: metamodelica::arrayFromVec(dom.clone().borrow().clone()), lmap: metamodelica::arrayFromVec(lmap.clone().borrow().clone()), ndim: dim.clone() });
    }
    Ok(map)
}

pub fn newScalar(mut dom: Arc<SBSet::SBSet>, mut lmap: Arc<SBLinearMap::SBLinearMap>) -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    if SBSet::ndim(dom.clone()) == SBLinearMap::ndim(lmap.clone()) {
        map = Arc::new(SBPWLinearMap { dom: arrayCreate(1, dom.clone()), lmap: arrayCreate(1, lmap.clone()), ndim: 1 });
    } else {
        map = newEmpty();
    }
    map
}

pub fn newEmpty() -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    map = Arc::new(SBPWLinearMap { dom: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), lmap: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), ndim: 0 });
    map
}

pub(crate) fn newIdentity(mut set: Arc<SBSet::SBSet>) -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    let mut lmap: Arc<SBLinearMap::SBLinearMap> = SBLinearMap::newIdentity(SBSet::ndim(set.clone()));
    map = Arc::new(SBPWLinearMap { dom: arrayCreate(1, set.clone()), lmap: arrayCreate(1, lmap.clone()), ndim: 1 });
    map
}

pub(crate) fn copy(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut map: Arc<SBPWLinearMap> = map;
    assign_field!(
        map.dom = Array::map(map.dom.clone(), (std::sync::Arc::new(fnptr!(SBSet::copy, Arc<SBSet::SBSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> + 'static>))?,
        map.lmap = Array::map(map.lmap.clone(), (std::sync::Arc::new(fnptr!(SBLinearMap::copy, Arc<SBLinearMap::SBLinearMap>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBLinearMap::SBLinearMap>) -> Result<Arc<SBLinearMap::SBLinearMap>> + 'static>))?
    );
    Ok(map)
}

pub(crate) fn dom(mut map: Arc<SBPWLinearMap>) -> metamodelica::Array<Arc<SBSet::SBSet>> {
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    dom
}

pub(crate) fn lmap(mut map: Arc<SBPWLinearMap>) -> metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> {
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    lmap
}

pub(crate) fn ndim(mut map: Arc<SBPWLinearMap>) -> i32 {
    let mut ndim: i32 = map.ndim.clone();
    ndim
}

pub(crate) fn isEmpty(mut map: Arc<SBPWLinearMap>) -> bool {
    let mut empty: bool = map.dom.clone().borrow().is_empty();
    empty
}

pub fn image(mut map: Arc<SBPWLinearMap>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
    fn add_set(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut map: Arc<SBLinearMap::SBLinearMap>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
        let mut set: Arc<SBSet::SBSet> = set;
        let mut aux_map: Arc<SBPWAtomicLinearMap::SBPWAtomicLinearMap>;
        aux_map = SBPWAtomicLinearMap::new(aset.clone(), map.clone());
        set = SBSet::addAtomicSet(SBPWAtomicLinearMap::image(aux_map.clone(), aset.clone())?, set.clone())?;
        Ok(set)
    }

    let mut outSet: Arc<SBSet::SBSet> = SBSet::newEmpty();
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut ss: Arc<SBSet::SBSet>;
    let mut partial_res: Arc<SBSet::SBSet>;
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        ss = ({let __elt = dom.borrow()[(i.clone()-1) as usize].clone(); __elt});
        ss = SBSet::intersection(ss.clone(), set.clone())?;
        partial_res = UnorderedSet::fold(SBSet::asets(ss.clone()), (std::sync::Arc::new({ let __pe_b1 = ({let __elt = lmap.borrow()[(i.clone()-1) as usize].clone(); __elt}); move |__pe_a0, __pe_a2| add_set(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> + 'static>), SBSet::newEmpty())?;
        outSet = SBSet::union(outSet.clone(), partial_res.clone())?;
    }
    Ok(outSet)
}

pub fn preImage(mut map: Arc<SBPWLinearMap>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
    fn add_set(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut map: Arc<SBLinearMap::SBLinearMap>, mut sets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
        let mut set: Arc<SBSet::SBSet> = set;
        let mut aux_map: Arc<SBPWAtomicLinearMap::SBPWAtomicLinearMap>;
        aux_map = SBPWAtomicLinearMap::new(aset.clone(), map.clone());
        let __range0 = sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut as2 in __range0 {
            set = SBSet::addAtomicSet(SBPWAtomicLinearMap::preImage(aux_map.clone(), as2.clone())?, set.clone())?;
        }
        Ok(set)
    }

    let mut outSet: Arc<SBSet::SBSet> = SBSet::newEmpty();
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut ss: Arc<SBSet::SBSet>;
    let mut partial_res: Arc<SBSet::SBSet>;
    let mut sets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>;
    sets = UnorderedSet::toArray(SBSet::asets(set.clone()));
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        ss = ({let __elt = dom.borrow()[(i.clone()-1) as usize].clone(); __elt});
        partial_res = SBSet::newEmpty();
        partial_res = UnorderedSet::fold(SBSet::asets(ss.clone()), (std::sync::Arc::new({ let __pe_b1 = ({let __elt = lmap.borrow()[(i.clone()-1) as usize].clone(); __elt}); let __pe_b2 = sets.clone(); move |__pe_a0, __pe_a3| add_set(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> + 'static>), SBSet::newEmpty())?;
        outSet = SBSet::union(outSet.clone(), partial_res.clone())?;
    }
    Ok(outSet)
}

pub(crate) fn compPW(mut map1: Arc<SBPWLinearMap>, mut map2: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut dom1: metamodelica::Array<Arc<SBSet::SBSet>> = map1.dom.clone();
    let mut dom2: metamodelica::Array<Arc<SBSet::SBSet>> = map2.dom.clone();
    let mut ress: Arc<Vector::Vector<Arc<SBSet::SBSet>>>;
    let mut lmap1: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map1.lmap.clone();
    let mut lmap2: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map2.lmap.clone();
    let mut reslm: Arc<Vector::Vector<Arc<SBLinearMap::SBLinearMap>>>;
    let mut aux_dom: Arc<SBSet::SBSet>;
    let mut new_dom: Arc<SBSet::SBSet>;
    let mut d1: Arc<SBSet::SBSet>;
    let mut d2: Arc<SBSet::SBSet>;
    let mut l1: Arc<SBLinearMap::SBLinearMap>;
    let mut l2: Arc<SBLinearMap::SBLinearMap>;
    let mut new_lm: Arc<SBLinearMap::SBLinearMap>;
    if isEmpty(map1.clone()) || isEmpty(map2.clone()) {
        outMap = newEmpty();
        return Ok(outMap.clone());
    }
    ress = Vector::new(0);
    reslm = Vector::new(0);
    for mut i in 1..=metamodelica::arrayLength(dom1.clone()) {
        d1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(dom1.clone(), i.clone());
        for mut j in 1..=metamodelica::arrayLength(dom2.clone()) {
            d2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(dom2.clone(), j.clone());
            aux_dom = image(map2.clone(), d2.clone())?;
            aux_dom = SBSet::intersection(aux_dom.clone(), d1.clone())?;
            aux_dom = preImage(map2.clone(), aux_dom.clone())?;
            new_dom = SBSet::intersection(aux_dom.clone(), d2.clone())?;
            if !(SBSet::isEmpty(new_dom.clone())) {
                l1 = ({let __elt = lmap1.borrow()[(i.clone()-1) as usize].clone(); __elt});
                l2 = ({let __elt = lmap2.borrow()[(j.clone()-1) as usize].clone(); __elt});
                new_lm = SBLinearMap::compose(l1.clone(), l2.clone());
                Vector::push(ress.clone(), new_dom.clone());
                Vector::push(reslm.clone(), new_lm.clone());
            }
        }
    }
    outMap = new(Vector::toArray(ress.clone()), Vector::toArray(reslm.clone()))?;
    Ok(outMap)
}

pub(crate) fn minInvCompact(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut aux_dom: Arc<SBSet::SBSet>;
    let mut dom_inv: Arc<SBSet::SBSet>;
    let mut aux_map: Arc<SBLinearMap::SBLinearMap>;
    let mut map_inv: Arc<SBLinearMap::SBLinearMap>;
    let mut min: metamodelica::Array<i32>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    if metamodelica::arrayLength(map.dom.clone()) != 1 {
        outMap = newEmpty();
        return Ok(outMap.clone());
    }
    aux_dom = metamodelica::arrayGet(map.dom.clone(), 1)?;
    dom_inv = image(map.clone(), aux_dom.clone())?;
    aux_map = metamodelica::arrayGet(map.lmap.clone(), 1)?;
    map_inv = SBLinearMap::inverse(aux_map.clone());
    min = SBSet::minElem(aux_dom.clone())?;
    g = SBLinearMap::gain(map_inv.clone());
    o = SBLinearMap::offset(map_inv.clone());
    resg = metamodelica::arrayCreate(metamodelica::arrayLength(g.clone()), metamodelica::OrderedFloat(0.0_f64));
    reso = metamodelica::arrayCreate(metamodelica::arrayLength(o.clone()), metamodelica::OrderedFloat(0.0_f64));
    for mut i in 1..=metamodelica::arrayLength(g.clone()) {
        if ({let __elt = g.borrow()[(i.clone()-1) as usize].clone(); __elt}) == intReal(System::intMaxLit()) {
            {
                let __cell0 = metamodelica::OrderedFloat((0) as f64);
                let __idx0 = i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), __idx0, __cell0); }
            }
            {
                let __cell1 = intReal(({let __elt = min.borrow()[(i.clone()-1) as usize].clone(); __elt}));
                let __idx1 = i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), __idx1, __cell1); }
            }
        } else {
            {
                let __cell2 = ({let __elt = g.borrow()[(i.clone()-1) as usize].clone(); __elt});
                let __idx2 = i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), __idx2, __cell2); }
            }
            {
                let __cell3 = ({let __elt = o.borrow()[(i.clone()-1) as usize].clone(); __elt});
                let __idx3 = i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), __idx3, __cell3); }
            }
        }
    }
    outMap = new(arrayCreate(1, dom_inv.clone()), arrayCreate(1, SBLinearMap::new(resg.clone(), reso.clone())?))?;
    Ok(outMap)
}

pub fn wholeDom(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBSet::SBSet>> {
    let mut set: Arc<SBSet::SBSet>;
    set = SBSet::newEmpty();
    let __range0 = map.dom.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut s in __range0 {
        set = SBSet::union(set.clone(), s.clone())?;
    }
    Ok(set)
}

pub fn combine(mut map1: Arc<SBPWLinearMap>, mut map2: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut sres: Arc<Vector::Vector<Arc<SBSet::SBSet>>>;
    let mut lres: Arc<Vector::Vector<Arc<SBLinearMap::SBLinearMap>>>;
    let mut dom2: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lm2: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut aux1: Arc<SBSet::SBSet>;
    let mut s2: Arc<SBSet::SBSet>;
    let mut new_dom: Arc<SBSet::SBSet>;
    if isEmpty(map1.clone()) {
        outMap = copy(map2.clone())?;
        return Ok(outMap.clone());
    }
    if isEmpty(map2.clone()) {
        outMap = copy(map1.clone())?;
        return Ok(outMap.clone());
    }
    sres = Vector::fromArray(map1.dom.clone());
    lres = Vector::fromArray(map1.lmap.clone());
    dom2 = map2.dom.clone();
    lm2 = map2.lmap.clone();
    aux1 = wholeDom(map1.clone())?;
    for mut i in 1..=metamodelica::arrayLength(dom2.clone()) {
        s2 = ({let __elt = dom2.borrow()[(i.clone()-1) as usize].clone(); __elt});
        new_dom = SBSet::complement(s2.clone(), aux1.clone())?;
        if !(SBSet::isEmpty(new_dom.clone())) {
            Vector::push(sres.clone(), new_dom.clone());
            Vector::push(lres.clone(), ({let __elt = lm2.borrow()[(i.clone()-1) as usize].clone(); __elt}));
        }
    }
    outMap = new(Vector::toArray(sres.clone()), Vector::toArray(lres.clone()))?;
    Ok(outMap)
}

pub(crate) fn atomize(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut dres: Arc<metamodelica::List<Arc<SBSet::SBSet>>> = metamodelica::nil();
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lres: Arc<metamodelica::List<Arc<SBLinearMap::SBLinearMap>>> = metamodelica::nil();
    let mut lm: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut d: Arc<SBSet::SBSet>;
    let mut aux: Arc<SBSet::SBSet>;
    let mut l: Arc<SBLinearMap::SBLinearMap>;
    let mut asets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>;
    for mut i in 1..=metamodelica::arrayLength(dom.clone()) {
        d = ({let __elt = dom.borrow()[(i.clone()-1) as usize].clone(); __elt});
        l = ({let __elt = lm.borrow()[(i.clone()-1) as usize].clone(); __elt});
        asets = UnorderedSet::toArray(SBSet::asets(d.clone()));
        let __range0 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range0 {
            aux = SBSet::newEmpty();
            aux = SBSet::addAtomicSet(s.clone(), aux.clone())?;
            dres = metamodelica::cons(aux.clone(), dres.clone());
            lres = metamodelica::cons(l.clone(), lres.clone());
        }
    }
    outMap = new(metamodelica::arrayFromVec(metamodelica::Dangerous::listReverseInPlace(dres.clone()).into_iter().cloned().collect()), metamodelica::arrayFromVec(metamodelica::Dangerous::listReverseInPlace(lres.clone()).into_iter().cloned().collect()))?;
    Ok(outMap)
}

pub(crate) fn isEqual(mut map1: Arc<SBPWLinearMap>, mut map2: Arc<SBPWLinearMap>) -> Result<bool> {
    let mut equal: bool;
    equal = Array::isEqualOnTrue(map1.dom.clone(), map2.dom.clone(), (std::sync::Arc::new(SBSet::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBSet::SBSet>, Arc<SBSet::SBSet>) -> Result<bool> + 'static>))? && Array::isEqualOnTrue(map1.lmap.clone(), map2.lmap.clone(), (std::sync::Arc::new(SBLinearMap::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBLinearMap::SBLinearMap>, Arc<SBLinearMap::SBLinearMap>) -> Result<bool> + 'static>))?;
    Ok(equal)
}

pub fn toString(mut map: Arc<SBPWLinearMap>) -> Result<ArcStr> {
    fn helper(mut set: Arc<SBAtomicSet::SBAtomicSet>, mut lm: Arc<SBLinearMap::SBLinearMap>) -> ArcStr {
        let mut r#str: ArcStr;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*SBPWAtomicLinearMap::toString(Arc::new(SBPWAtomicLinearMap::SBPWAtomicLinearMap { dom: set.clone(), lmap: lm.clone() }))); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        r#str
    }

    let mut r#str: ArcStr;
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut i in ({let __s=metamodelica::arrayLength(dom.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        strl = metamodelica::cons((UnorderedSet::toString(SBSet::asets(({let __elt = dom.borrow()[(i.clone()-1) as usize].clone(); __elt})), (std::sync::Arc::new({ let __pe_b1 = ({let __elt = lmap.borrow()[(i.clone()-1) as usize].clone(); __elt}); move |__pe_a0| Ok(helper(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<ArcStr> + 'static>), (literal!("U")).clone())?).clone(), strl.clone());
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(strl.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}


