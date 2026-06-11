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

use crate::Autoconf;
use crate::ClockIndexes;
use crate::Global;
use crate::Print;
use crate::System;
use openmodelica_util_datatypes_basic::List;

/// Used to signal success or failure of a function call
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Status {
    SUCCESS,
    FAILURE,
}
impl metamodelica::gc::MMTrace for Status {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Status::SUCCESS => Ok(()),
            Status::FAILURE => Ok(()),
        }
    }
}
impl Default for Status {
    fn default() -> Self { Self::SUCCESS }
}
pub use self::Status::{SUCCESS,FAILURE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DateTime {
    pub sec: i32,
    pub min: i32,
    pub hour: i32,
    pub mday: i32,
    pub mon: i32,
    pub year: i32,
}

impl metamodelica::gc::MMTrace for DateTime {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.sec, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.min, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hour, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mday, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mon, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.year, __mmv)?;
        Ok(())
    }
}
impl Default for DateTime {
    fn default() -> Self {
        Self {
            sec: Default::default(),
            min: Default::default(),
            hour: Default::default(),
            mday: Default::default(),
            mon: Default::default(),
            year: Default::default(),
        }
    }
}

pub type DATETIME = DateTime;


pub const HASH_SEED: i32 = 5381;

pub(crate) static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

pub fn isIntGreater(mut lhs: i32, mut rhs: i32) -> bool {
    let mut b: bool = lhs > rhs;
    b
}

pub(crate) fn isRealGreater(mut lhs: metamodelica::Real, mut rhs: metamodelica::Real) -> bool {
    let mut b: bool = lhs > rhs;
    b
}

pub(crate) fn linuxDotSlash() -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (arcstr::literal!(Autoconf::os)).clone();
    r#str = (if (r#str.clone() == literal!("linux") || r#str == literal!("darwin")) {literal!("./")} else {literal!("")}).clone();
    r#str
}

pub fn flagValue(mut flag: ArcStr, mut arguments: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut flagVal: ArcStr;
    let mut arg: ArcStr;
    let mut rest: Arc<metamodelica::List<ArcStr>> = arguments.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest = __pa1.clone();
        if arg.clone() == flag.clone() {
            break;
        }
    }
    flagVal = (if (rest.clone().is_empty()) {literal!("")} else {listHead(rest)?}).clone();
    Ok(flagVal)
}

pub(crate) fn selectFirstNonEmptyString(mut inStrings: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut outResult: ArcStr;
    for mut e in &*inStrings {
        let mut e = e.clone();
        if e.clone() != literal!("") {
            outResult = (e.clone()).clone();
            return outResult.clone();
        }
    }
    outResult = (literal!("")).clone();
    outResult
}

pub fn compareTupleIntGt<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inTplA: (i32, T), mut inTplB: (i32, T)) -> bool {
    let mut res: bool;
    let mut a: i32;
    let mut b: i32;
    (a, _) = inTplA;
    (b, _) = inTplB;
    res = intGt(a, b);
    res
}

pub(crate) fn compareTupleIntLt<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inTplA: (i32, T), mut inTplB: (i32, T)) -> bool {
    let mut res: bool;
    let mut a: i32;
    let mut b: i32;
    (a, _) = inTplA;
    (b, _) = inTplB;
    res = intLt(a, b);
    res
}

pub fn compareTuple2IntGt<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inTplA: (T, i32), mut inTplB: (T, i32)) -> bool {
    let mut res: bool;
    let mut a: i32;
    let mut b: i32;
    (_, a) = inTplA;
    (_, b) = inTplB;
    res = intGt(a, b);
    res
}

pub fn compareTuple2IntLt<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inTplA: (T, i32), mut inTplB: (T, i32)) -> bool {
    let mut res: bool;
    let mut a: i32;
    let mut b: i32;
    (_, a) = inTplA;
    (_, b) = inTplB;
    res = intLt(a, b);
    res
}

pub fn tuple21<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2)) -> T1 {
    let mut outValue: T1;
    (outValue, _) = inTuple;
    outValue
}

pub fn tuple22<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2)) -> T2 {
    let mut outValue: T2;
    (_, outValue) = inTuple;
    outValue
}

pub(crate) fn optTuple22<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: Option<(T1, T2)>) -> Result<T2> {
    let mut outValue: T2;
    let __pa0 = ::match_deref::match_deref! { match &(inTuple) {
        Some((_, __pa0)) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn tuple312<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3)) -> (T1, T2) {
    let mut outTuple: (T1, T2);
    let mut e1: T1;
    let mut e2: T2;
    (e1, e2, _) = inTuple;
    outTuple = (e1, e2);
    outTuple
}

pub fn tuple31<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: (T1, T2, T3)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _) = inValue;
    outValue
}

pub fn tuple32<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: (T1, T2, T3)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _) = inValue;
    outValue
}

pub fn tuple33<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: (T1, T2, T3)) -> T3 {
    let mut outValue: T3;
    (_, _, outValue) = inValue;
    outValue
}

pub fn tuple41<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _, _) = inTuple;
    outValue
}

pub fn tuple42<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _, _) = inTuple;
    outValue
}

pub fn tuple43<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4)) -> T3 {
    let mut outValue: T3;
    (_, _, outValue, _) = inTuple;
    outValue
}

pub fn tuple44<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4)) -> T4 {
    let mut outValue: T4;
    (_, _, _, outValue) = inTuple;
    outValue
}

pub(crate) fn tuple51<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _, _, _) = inTuple;
    outValue
}

pub(crate) fn tuple52<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _, _, _) = inTuple;
    outValue
}

pub(crate) fn tuple53<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5)) -> T3 {
    let mut outValue: T3;
    (_, _, outValue, _, _) = inTuple;
    outValue
}

pub(crate) fn tuple54<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5)) -> T4 {
    let mut outValue: T4;
    (_, _, _, outValue, _) = inTuple;
    outValue
}

pub(crate) fn tuple55<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5)) -> T5 {
    let mut outValue: T5;
    (_, _, _, _, outValue) = inTuple;
    outValue
}

pub fn tuple61<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace, T6: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5, T6)) -> T1 {
    let mut outValue: T1;
    (outValue, _, _, _, _, _) = inTuple;
    outValue
}

pub fn tuple62<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace, T4: Clone + 'static + metamodelica::gc::MMTrace, T5: Clone + 'static + metamodelica::gc::MMTrace, T6: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3, T4, T5, T6)) -> T2 {
    let mut outValue: T2;
    (_, outValue, _, _, _, _) = inTuple;
    outValue
}

pub(crate) fn stringContainsChar(mut r#str: ArcStr, mut char: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let mut ch: i32;
    ch = stringCharInt((char).clone())?;
    for mut i in 1..=((r#str.clone()).clone().len() as i32) {
        if metamodelica::Dangerous::stringGetNoBoundsChecking((r#str.clone()).clone(), i.clone()) == ch {
            res = true;
            return Ok(res.clone());
        }
    }
    Ok(res)
}

pub(crate) fn stringDelimitListPrintBuf(mut inStringLst: Arc<metamodelica::List<ArcStr>>, mut inDelimiter: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inStringLst;
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
                Deref @ metamodelica::List::Cons { head: f, tail: Deref @ metamodelica::List::Nil } => {
                    Print::printBuf((f.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f, tail: r } => {
                    stringDelimitListPrintBuf(r.clone(), (inDelimiter.clone()).clone())?;
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((inDelimiter.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn stringDelimitListAndSeparate(mut r#str: Arc<metamodelica::List<ArcStr>>, mut sep1: ArcStr, mut sep2: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let mut handle: i32;
    handle = Print::saveAndClearBuf()?;
    stringDelimitListAndSeparate2(r#str, (sep1).clone(), (sep2).clone(), n, 0)?;
    res = (Print::getString()?).clone();
    Print::restoreBuf(handle)?;
    Ok(res)
}

fn stringDelimitListAndSeparate2(mut inStringLst1: Arc<metamodelica::List<ArcStr>>, mut inString2: ArcStr, mut inString3: ArcStr, mut inInteger4: i32, mut inInteger5: i32) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inStringLst1, inString2, inString3, inInteger4, inInteger5);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: s, tail: Deref @ metamodelica::List::Nil }, _, _, _, _) => {
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: f, tail: r }, sep1, sep2, n, 0) => {
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((sep1.clone()).clone())?;
                    stringDelimitListAndSeparate2(r.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), 1)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: f, tail: r }, sep1, sep2, n, iter) => {
                    let mut iter_1: i32;
                    let 0 = (intMod(iter.clone(), n.clone())) else { bail!("pattern mismatch") };
                    iter_1 = iter.clone() + 1;
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((sep1.clone()).clone())?;
                    Print::printBuf((sep2.clone()).clone())?;
                    stringDelimitListAndSeparate2(r.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), iter_1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: f, tail: r }, sep1, sep2, n, iter) => {
                    let mut iter_1: i32;
                    iter_1 = iter.clone() + 1;
                    Print::printBuf((f.clone()).clone())?;
                    Print::printBuf((sep1.clone()).clone())?;
                    stringDelimitListAndSeparate2(r.clone(), (sep1.clone()).clone(), (sep2.clone()).clone(), n.clone(), iter_1.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("- stringDelimitListAndSeparate2 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn stringDelimitListNonEmptyElts(mut lst: Arc<metamodelica::List<ArcStr>>, mut delim: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut lst1: Arc<metamodelica::List<ArcStr>>;
    lst1 = List::select(lst, (std::sync::Arc::new(fnptr!(isNotEmptyString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>))?;
    r#str = stringDelimitList(lst1, (delim).clone());
    Ok(r#str)
}

pub fn mulStringDelimit2Int(mut inString: ArcStr, mut delim: ArcStr) -> Result<i32> {
    let mut i: i32;
    let mut lst: Arc<metamodelica::List<ArcStr>>;
    let mut lst2: Arc<metamodelica::List<i32>>;
    lst = stringSplitAtChar((inString).clone(), (delim).clone())?;
    lst2 = List::map(lst, (std::sync::Arc::new(stringInt) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>))?;
    if !(lst2.clone().is_empty()) {
        i = List::fold(lst2, (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
    } else {
        i = 0;
    }
    Ok(i)
}

pub fn stringReplaceChar(mut inString1: ArcStr, mut inString2: ArcStr, mut inString3: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (System::stringReplace((inString1).clone(), (inString2).clone(), (inString3).clone())?).clone();
    Ok(outString)
}

pub fn stringSplitAtChar(mut string: ArcStr, mut token: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ch: i32 = stringCharInt((token.clone()).clone())?;
    let mut cur: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut c in &*stringListStringChar((string).clone()) {
        let mut c = c.clone();
        if stringCharInt((c.clone()).clone())? == ch {
            strings = metamodelica::cons(stringAppendList(cur.clone().reverse()), strings.clone());
            cur = metamodelica::nil();
        } else {
            cur = metamodelica::cons((c.clone()).clone(), cur.clone());
        }
    }
    if !(cur.clone().is_empty()) {
        strings = metamodelica::cons(stringAppendList(cur.reverse()), strings);
    }
    strings = strings.reverse();
    Ok(strings)
}

pub(crate) fn optionToString<T: Clone + 'static + metamodelica::gc::MMTrace>(mut ot: Option<T>, mut f: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    let mut t: T;
    r#str = ((match ot {
        Some(mut __esc_t) => {
            t = __esc_t.clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOME(")); __mm_s.push_str(&*f(t.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => literal!("NONE()"),
    })).clone();
    Ok(r#str)
}

pub fn applyOption<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(mut inOption: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Result<Option<TO>> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outOption: Option<TO>;
    outOption = (match inOption {
        Some(mut ival) => {
            Some(inFunc(ival.clone())?)
        },
        _ => {
            None
        },
    });
    Ok(outOption)
}

pub fn applyOption1<TI: Clone + 'static + metamodelica::gc::MMTrace, ArgT: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(mut inOption: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT) -> Result<Option<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outOption: Option<TO>;
    outOption = (match inOption {
        Some(mut ival) => {
            Some(inFunc(ival.clone(), inArg)?)
        },
        _ => {
            None
        },
    });
    Ok(outOption)
}

pub fn applyOptionOrDefault<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>, mut inDefaultValue: TO) -> Result<TO> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outValue: TO;
    outValue = (match inValue {
        Some(mut value) => {
            inFunc(value.clone())?
        },
        _ => {
            inDefaultValue
        },
    });
    Ok(outValue)
}

pub(crate) fn applyOptionOrDefault1<TI: Clone + 'static + metamodelica::gc::MMTrace, ArgT: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT, mut inDefaultValue: TO) -> Result<TO> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outValue: TO;
    outValue = (match inValue {
        Some(mut value) => {
            inFunc(value.clone(), inArg)?
        },
        _ => {
            inDefaultValue
        },
    });
    Ok(outValue)
}

pub(crate) fn applyOptionOrDefault2<TI: Clone + 'static + metamodelica::gc::MMTrace, ArgT1: Clone + 'static + metamodelica::gc::MMTrace, ArgT2: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: Option<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inDefaultValue: TO) -> Result<TO> {
    pub type FuncType<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outValue: TO;
    outValue = (match inValue {
        Some(mut value) => {
            inFunc(value.clone(), inArg1, inArg2)?
        },
        _ => {
            inDefaultValue
        },
    });
    Ok(outValue)
}

pub(crate) fn applyOption_2<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue1: Option<T>, mut inValue2: Option<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>) -> Result<Option<T>> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outValue: Option<T>;
    outValue = (match (inValue1.clone(), inValue2.clone()) {
        (None, _) => inValue2,
        (_, None) => inValue1,
        _ => Some(inFunc(getOption(inValue1)?, getOption(inValue2)?)?),
    });
    Ok(outValue)
}

pub fn makeOption<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: T) -> Option<T> {
    let mut outOption: Option<T> = Some(inValue.clone());
    outOption
}

pub(crate) fn makeOptionOnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inCondition: bool, mut inValue: T) -> Option<T> {
    let mut outOption: Option<T> = if (inCondition) {Some(inValue.clone())} else {None};
    outOption
}

pub fn getOption<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inOption: Option<T>) -> Result<T> {
    let mut outValue: T;
    let __pa0 = ::match_deref::match_deref! { match &(inOption) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getOptionOrDefault<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inOption: Option<T>, mut inDefault: T) -> T {
    let mut outValue: T;
    outValue = (match inOption {
        Some(mut value) => {
            value.clone()
        },
        _ => {
            inDefault
        },
    });
    outValue
}

pub(crate) fn intGreaterZero(mut v: i32) -> bool {
    let mut res: bool = v > 0;
    res
}

pub fn intPositive(mut v: i32) -> bool {
    let mut res: bool = v >= 0;
    res
}

pub(crate) fn intNegative(mut v: i32) -> bool {
    let mut res: bool = v < 0;
    res
}

pub fn intSign(mut i: i32) -> i32 {
    let mut o: i32 = if (i == 0) {0} else if (i > 0) {1} else {-1};
    o
}

pub fn intCompare(mut inN: i32, mut inM: i32) -> i32 {
    let mut outResult: i32 = if (inN == inM) {0} else if (inN > inM) {1} else {-1};
    outResult
}

pub fn intPow(mut base: i32, mut exponent: i32) -> Result<i32> {
    let mut result: i32 = 1;
    if exponent >= 0 {
        for mut i in 1..=exponent {
            result = result * base;
        }
    } else {
        bail!("fail");
    }
    Ok(result)
}

pub(crate) fn realNegative(mut v: metamodelica::Real) -> bool {
    let mut res: bool = v < metamodelica::OrderedFloat((0) as f64);
    res
}

pub fn realCompare(mut inN: metamodelica::Real, mut inM: metamodelica::Real) -> i32 {
    let mut outResult: i32 = if (inN == inM) {0} else if (inN > inM) {1} else {-1};
    outResult
}

pub fn boolCompare(mut inN: bool, mut inM: bool) -> i32 {
    let mut outResult: i32 = if (inN == inM) {0} else if (inN > inM) {1} else {-1};
    outResult
}

pub fn isNotEmptyString(mut inString: ArcStr) -> bool {
    let mut outIsNotEmpty: bool = ((inString.clone()).clone().len() as i32) > 0;
    outIsNotEmpty
}

pub(crate) fn writeFileOrErrorMsg(mut inFilename: ArcStr, mut inString: ArcStr) -> Result<()> {
    if '__try0: {
        unwrap_break_err!(System::writeFile((inFilename.clone()).clone(), (inString.clone()).clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Print::printErrorBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("# Cannot write to file: ")); __mm_s.push_str(&*inFilename.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(())
}

pub fn strncmp(mut inString1: ArcStr, mut inString2: ArcStr, mut inLength: i32) -> bool {
    let mut outEqual: bool;
    outEqual = 0 == System::strncmp((inString1).clone(), (inString2).clone(), inLength);
    outEqual
}

pub fn notStrncmp(mut inString1: ArcStr, mut inString2: ArcStr, mut inLength: i32) -> bool {
    let mut outEqual: bool;
    outEqual = 0 != System::strncmp((inString1).clone(), (inString2).clone(), inLength);
    outEqual
}

pub fn tickStr() -> ArcStr {
    let mut s: ArcStr = intString(tick());
    s
}

pub fn replaceWindowsBackSlashWithPathDelimiter(mut inPath: ArcStr) -> Result<ArcStr> {
    let mut outPath: ArcStr;
    if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
        outPath = (System::stringReplace((inPath).clone(), (literal!("\\")).clone(), (arcstr::literal!(Autoconf::pathDelimiter)).clone())?).clone();
    } else {
        outPath = (inPath).clone();
    }
    Ok(outPath)
}

pub fn getAbsoluteDirectoryAndFile(mut filename: ArcStr) -> Result<(ArcStr, ArcStr)> {
    let mut dirname: ArcStr;
    let mut basename: ArcStr;
    let mut realpath: ArcStr;
    realpath = (System::realpath((filename).clone())?).clone();
    dirname = (System::dirname((realpath.clone()).clone())).clone();
    basename = (System::basename((realpath).clone())).clone();
    dirname = (replaceWindowsBackSlashWithPathDelimiter((dirname).clone())?).clone();
    Ok((dirname, basename))
}

pub(crate) fn rawStringToInputString(mut inString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (System::stringReplace((inString).clone(), (literal!("\\\"")).clone(), (literal!("\"")).clone())?).clone();
    outString = (System::stringReplace((outString).clone(), (literal!("\\\\")).clone(), (literal!("\\")).clone())?).clone();
    Ok(outString)
}

pub fn escapeModelicaStringToCString(mut modelicaString: ArcStr) -> ArcStr {
    let mut cString: ArcStr;
    cString = (System::escapedString((modelicaString).clone(), true)).clone();
    cString
}

pub fn escapeModelicaStringToJLString(mut modelicaString: ArcStr) -> Result<ArcStr> {
    let mut cString: ArcStr;
    cString = (System::stringReplace((modelicaString).clone(), (literal!("$")).clone(), (literal!("")).clone())?).clone();
    cString = (System::stringReplace((cString).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
    cString = (System::stringReplace((cString).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
    cString = (System::stringReplace((cString).clone(), (literal!("\"\"")).clone(), (literal!("")).clone())?).clone();
    cString = (System::escapedString((cString).clone(), true)).clone();
    Ok(cString)
}

pub fn escapeModelicaStringToXmlString(mut modelicaString: ArcStr) -> Result<ArcStr> {
    let mut xmlString: ArcStr;
    xmlString = (System::stringReplace((modelicaString).clone(), (literal!("&")).clone(), (literal!("&amp;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString).clone(), (literal!("\"")).clone(), (literal!("&quot;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString).clone(), (literal!("<")).clone(), (literal!("&lt;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString).clone(), (literal!(">")).clone(), (literal!("&gt;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString).clone(), (literal!("\n")).clone(), (literal!("&#10;")).clone())?).clone();
    xmlString = (System::stringReplace((xmlString).clone(), (literal!("\r")).clone(), (literal!("&#13;")).clone())?).clone();
    Ok(xmlString)
}

pub fn makeQuotedIdentifier(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut quotedIdentifier: ArcStr;
    quotedIdentifier = (System::stringReplace((r#str).clone(), (literal!("\\")).clone(), (literal!("\\\\")).clone())?).clone();
    quotedIdentifier = (System::stringReplace((quotedIdentifier).clone(), (literal!("'")).clone(), (literal!("\\'")).clone())?).clone();
    quotedIdentifier = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*quotedIdentifier); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone();
    Ok(quotedIdentifier)
}

pub fn escapeQuotes(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut quotes: ArcStr;
    quotes = (System::stringReplace((r#str).clone(), (literal!("\\")).clone(), (literal!("\\\\")).clone())?).clone();
    quotes = (System::stringReplace((quotes).clone(), (literal!("'")).clone(), (literal!("\\'")).clone())?).clone();
    Ok(quotes)
}

pub fn makeTuple<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue1: T1, mut inValue2: T2) -> (T1, T2) {
    let mut outTuple: (T1, T2) = (inValue1.clone(), inValue2.clone());
    outTuple
}

pub(crate) fn makeTupleR<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue1: T1, mut inValue2: T2) -> (T2, T1) {
    let mut outTuple: (T2, T1) = (inValue2.clone(), inValue1.clone());
    outTuple
}

pub fn make3Tuple<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue1: T1, mut inValue2: T2, mut inValue3: T3) -> (T1, T2, T3) {
    let mut outTuple: (T1, T2, T3) = (inValue1.clone(), inValue2.clone(), inValue3.clone());
    outTuple
}

pub(crate) fn mulListIntegerOpt(mut inList: Arc<metamodelica::List<Option<i32>>>, mut inAccum: i32) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inList) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inAccum)
        },
        Deref @ metamodelica::List::Cons { head: Some(i), tail: rest } => {
            { (inList, inAccum) = (rest.clone(), i.clone() * inAccum); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: None, tail: rest } => {
            { (inList, inAccum) = (rest.clone(), inAccum); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

/// A single boolean value that can be updated (a destructive operation). NOTE: Use Mutable<Boolean> instead. This implementation is kept since Susan cannot use that type.
pub type StatefulBoolean = metamodelica::Array<bool>;

pub fn makeStatefulBoolean(mut b: bool) -> StatefulBoolean {
    let mut sb: StatefulBoolean = arrayCreate(1, b);
    sb
}

pub fn getStatefulBoolean(mut sb: StatefulBoolean) -> bool {
    let mut b: bool = ({let __elt = sb.borrow()[(1-1) as usize].clone(); __elt});
    b
}

pub fn setStatefulBoolean(mut sb: StatefulBoolean, mut b: bool) -> Result<()> {
    metamodelica::arrayUpdate(sb.clone(), 1, b)?;
    Ok(())
}

pub fn optionEqual<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inOption1: Option<T1>, mut inOption2: Option<T2>, mut inFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>) -> Result<bool> {
    pub type CompareFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outEqual: bool;
    outEqual = (match (inOption1, inOption2) {
        (Some(mut val1), Some(mut val2)) => {
            inFunc(val1.clone(), val2.clone())?
        },
        (None, None) => {
            true
        },
        _ => {
            false
        },
    });
    Ok(outEqual)
}

pub fn makeValueOrDefault<TI: Clone + 'static + metamodelica::gc::MMTrace, TO: Clone + 'static + metamodelica::gc::MMTrace>(mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>, mut inArg: TI, mut inDefaultValue: TO) -> TO {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outValue: TO;
    match '__try0: {
        outValue = unwrap_break_err!(inFunc(inArg.clone()), '__try0);
        Ok::<_, anyhow::Error>((outValue.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outValue = __try0_o0;
        }
        Err(_) => {
            outValue = inDefaultValue.clone();
        }
    }
    outValue
}

pub fn xmlEscape(mut s1: ArcStr) -> Result<ArcStr> {
    let mut s2: ArcStr;
    s2 = (stringReplaceChar((s1).clone(), (literal!("&")).clone(), (literal!("&amp;")).clone())?).clone();
    s2 = (stringReplaceChar((s2).clone(), (literal!("<")).clone(), (literal!("&lt;")).clone())?).clone();
    s2 = (stringReplaceChar((s2).clone(), (literal!(">")).clone(), (literal!("&gt;")).clone())?).clone();
    s2 = (stringReplaceChar((s2).clone(), (literal!("\"")).clone(), (literal!("&quot;")).clone())?).clone();
    Ok(s2)
}

pub fn strcmpBool(mut s1: ArcStr, mut s2: ArcStr) -> bool {
    let mut b: bool = stringCompare((s1.clone()).clone(), (s2.clone()).clone()) > 0;
    b
}

pub fn strcmpNoCaseBool(mut s1: ArcStr, mut s2: ArcStr) -> bool {
    let mut b: bool = stringCompare((System::tolower((s1.clone()).clone())).clone(), (System::tolower((s2.clone()).clone())).clone()) > 0;
    b
}

pub(crate) fn stringAppendReverse(mut str1: ArcStr, mut str2: ArcStr) -> ArcStr {
    let mut r#str: ArcStr = stringAppend((str2.clone()).clone(), (str1.clone()).clone());
    r#str
}

pub(crate) fn stringAppendNonEmpty(mut inString1: ArcStr, mut inString2: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inString2.clone()) {
        Deref @ "" => inString2,
        _ => stringAppend((inString1).clone(), (inString2).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn getCurrentDateTime() -> DateTime {
    let mut dt: DateTime;
    let mut sec: i32;
    let mut min: i32;
    let mut hour: i32;
    let mut mday: i32;
    let mut mon: i32;
    let mut year: i32;
    (sec, min, hour, mday, mon, year) = System::getCurrentDateTime();
    dt = DateTime { sec: sec, min: min, hour: hour, mday: mday, mon: mon, year: year };
    dt
}

pub fn isSuccess(mut status: Status) -> Result<bool> {
    let mut bool: bool;
    bool = (match status {
        Status::SUCCESS { .. } => true,
        Status::FAILURE { .. } => false,
    });
    Ok(bool)
}

pub fn id<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inValue: T) -> T {
    let mut outValue: T = inValue.clone();
    outValue
}

pub fn buildMapStr(mut inLst1: Arc<metamodelica::List<ArcStr>>, mut inLst2: Arc<metamodelica::List<ArcStr>>, mut inMiddleDelimiter: ArcStr, mut inEndDelimiter: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &((inLst1, inLst2, inMiddleDelimiter, inEndDelimiter)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: fa, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: fb, tail: Deref @ metamodelica::List::Nil }, md, _) => {
            let mut r#str: ArcStr;
            r#str = stringAppendList(list![(fa.clone()).clone(), (md.clone()).clone(), (fb.clone()).clone()]);
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: fa, tail: ra }, Deref @ metamodelica::List::Cons { head: fb, tail: rb }, md, ed) => {
            let mut r#str: ArcStr;
            r#str = (buildMapStr(ra.clone(), rb.clone(), (md.clone()).clone(), (ed.clone()).clone())?).clone();
            r#str = stringAppendList(list![(fa.clone()).clone(), (md.clone()).clone(), (fb.clone()).clone(), (ed.clone()).clone(), (r#str.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

pub fn assoc<Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq, Val: Clone + 'static + metamodelica::gc::MMTrace>(mut inKey: Key, mut inList: Arc<metamodelica::List<(Key, Val)>>) -> Result<Val> {
    '__tco: loop {
        let mut k: Key;
        let mut v: Val;
        (k, v) = listHead(inList.clone())?;
        if (inKey.clone() == k) {return Ok(v)} else {{ (inKey, inList) = (inKey, listRest(inList)?); continue '__tco; }}
    }
}

pub fn boolInt(mut inBoolean: bool) -> i32 {
    let mut outInteger: i32 = if (inBoolean) {1} else {0};
    outInteger
}

pub(crate) fn intBool(mut inInteger: i32) -> bool {
    let mut outBoolean: bool = inInteger > 0;
    outBoolean
}

pub(crate) fn stringBool(mut inString: ArcStr) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = stringBool2((System::tolower((inString).clone())).clone())?;
    Ok(outBoolean)
}

fn stringBool2(mut inString: ArcStr) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inString) {
        Deref @ "true" => true,
        Deref @ "false" => false,
        Deref @ "yes" => true,
        Deref @ "no" => false,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

pub fn stringEqCaseInsensitive(mut str1: ArcStr, mut str2: ArcStr) -> bool {
    let mut eq: bool;
    eq = stringEq((System::tolower((str1).clone())).clone(), (System::tolower((str2).clone())).clone());
    eq
}

pub fn stringPadRight(mut inString: ArcStr, mut inPadWidth: i32, mut inPadString: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    let mut pad_length: i32;
    let mut pad_str: ArcStr;
    pad_length = inPadWidth - ((inString.clone()).clone().len() as i32);
    if pad_length > 0 {
        pad_str = stringAppendList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (1..=pad_length).into_iter() {
            let __x = inPadString.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inString); __mm_s.push_str(&*pad_str); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = (inString).clone();
    }
    outString
}

pub fn stringPadLeft(mut inString: ArcStr, mut inPadWidth: i32, mut inPadString: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    let mut pad_length: i32;
    let mut pad_str: ArcStr;
    pad_length = inPadWidth - ((inString.clone()).clone().len() as i32);
    if pad_length > 0 {
        pad_str = stringAppendList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (1..=pad_length).into_iter() {
            let __x = inPadString.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*pad_str); __mm_s.push_str(&*inString); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = (inString).clone();
    }
    outString
}

pub(crate) fn intProduct(mut lst: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut i: i32 = List::fold(lst.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
    Ok(i)
}

pub fn nextPrime(mut inN: i32) -> i32 {
    let mut outNextPrime: i32;
    outNextPrime = if (inN <= 2) {2} else {nextPrime2(inN + intMod(inN + 1, 2))};
    outNextPrime
}

fn nextPrime2(mut inN: i32) -> i32 {
    '__tco: loop {
        if (nextPrime_isPrime(inN)) {return inN} else {{ inN = inN + 2; continue '__tco; }}
    }
}

fn nextPrime_isPrime(mut inN: i32) -> bool {
    let mut outIsPrime: bool;
    let mut i: i32 = 3;
    let mut q: i32 = intDiv(inN, 3);
    while q >= i {
        if inN == q * i {
            outIsPrime = false;
            return outIsPrime.clone();
        }
        i = i + 2;
        q = intDiv(inN, i);
    }
    outIsPrime = true;
    outIsPrime
}

pub(crate) fn anyToEmptyString<T: Clone + 'static + metamodelica::gc::MMTrace>(mut a: T) -> ArcStr {
    let mut empty: ArcStr = literal!("");
    empty
}

pub fn removeLast3Char(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = substring((r#str.clone()).clone(), 1, ((r#str).clone().len() as i32) - 3)?;
    Ok(outStr)
}

pub fn removeLast4Char(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = substring((r#str.clone()).clone(), 1, ((r#str).clone().len() as i32) - 4)?;
    Ok(outStr)
}

pub fn removeLastNChar(mut r#str: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = substring((r#str.clone()).clone(), 1, ((r#str).clone().len() as i32) - n)?;
    Ok(outStr)
}

pub fn stringNotEqual(mut str1: ArcStr, mut str2: ArcStr) -> bool {
    let mut b: bool = !(stringEq((str1.clone()).clone(), (str2.clone()).clone()));
    b
}

pub fn swap<T: Clone + 'static + metamodelica::gc::MMTrace>(mut cond: bool, mut in1: T, mut in2: T) -> (T, T) {
    let mut out1: T;
    let mut out2: T;
    (out1, out2) = (match cond {
        true => (in2, in1),
        _ => (in1, in2),
    });
    (out1, out2)
}

pub fn replace<T: Clone + 'static + metamodelica::gc::MMTrace>(mut replaced: T, mut arg: T) -> T {
    let mut outArg: T = arg.clone();
    outArg
}

pub fn realRangeSize(mut inStart: metamodelica::Real, mut inStep: metamodelica::Real, mut inStop: metamodelica::Real) -> i32 {
    let mut outSize: i32;
    outSize = ((((inStop - inStart) / inStep + metamodelica::OrderedFloat(5e-15_f64)).floor()).0.floor() as i32) + 1;
    outSize = std::cmp::max(outSize, 0);
    outSize
}

fn createDirectoryTreeH(mut inString: ArcStr, mut parentDir: ArcStr, mut parentDirExists: bool) -> bool {
    let mut outBool: bool;
    outBool = 'mc: {
        let __mc_input = parentDirExists;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool;
            let true = (stringEqual((parentDir.clone()).clone(), (System::dirname((parentDir.clone()).clone())).clone())) else { bail!("pattern mismatch") };
            b = System::createDirectory((inString.clone()).clone());
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool;
            b = System::createDirectory((inString.clone()).clone());
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool;
            let true = (createDirectoryTree((parentDir.clone()).clone())) else { bail!("pattern mismatch") };
            b = System::createDirectory((inString.clone()).clone());
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outBool
}

pub fn createDirectoryTree(mut inString: ArcStr) -> bool {
    let mut outBool: bool;
    let mut parentDir: ArcStr;
    let mut parentDirExists: bool;
    if System::directoryExists((inString.clone()).clone()) {
        outBool = true;
    } else {
        parentDir = (System::dirname((inString.clone()).clone())).clone();
        parentDirExists = System::directoryExists((parentDir.clone()).clone());
        outBool = createDirectoryTreeH((inString).clone(), (parentDir).clone(), parentDirExists);
    }
    outBool
}

pub(crate) fn nextPowerOf2(mut i: i32) -> i32 {
    let mut v: i32;
    v = i - 1;
    v = intBitOr(v, intBitRShift(v, 1));
    v = intBitOr(v, intBitRShift(v, 2));
    v = intBitOr(v, intBitRShift(v, 4));
    v = intBitOr(v, intBitRShift(v, 8));
    v = intBitOr(v, intBitRShift(v, 16));
    v = v + 1;
    v
}

pub fn isCIdentifier(mut r#str: ArcStr) -> bool {
    let mut b: bool;
    let mut i: i32;
    (i, _) = System::regex((r#str).clone(), (literal!("^[_A-Za-z][_A-Za-z0-9]*$")).clone(), 0, true, false);
    b = i == 1;
    b
}

pub(crate) fn isIntegerString(mut r#str: ArcStr) -> bool {
    let mut b: bool;
    let mut i: i32;
    (i, _) = System::regex((r#str).clone(), (literal!("^[0-9][0-9]*$")).clone(), 0, true, false);
    b = i == 1;
    b
}

pub fn stringTrunc(mut r#str: ArcStr, mut len: i32) -> Result<ArcStr> {
    let mut truncatedStr: ArcStr;
    truncatedStr = (if (((r#str.clone()).clone().len() as i32) <= len) {r#str} else {substring((r#str).clone(), 0, len)?}).clone();
    Ok(truncatedStr)
}

pub fn getTempVariableIndex() -> ArcStr {
    let mut name: ArcStr;
    name = (stringAppend((literal!("$tmpVar")).clone(), (intString(System::tmpTickIndex(Global::tmpVariableIndex.clone()))).clone())).clone();
    name
}

pub fn anyReturnTrue<T: Clone + 'static + metamodelica::gc::MMTrace>(mut a: T) -> bool {
    let mut b: bool = true;
    b
}

pub fn absoluteOrRelative(mut inFileName: ArcStr) -> ArcStr {
    let mut outFileName: ArcStr = inFileName.clone();
    let mut pwd: ArcStr;
    let mut pd: ArcStr;
    let mut f: ArcStr;
    pwd = (System::pwd()).clone();
    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
    if !(System::regularFileExists((inFileName.clone()).clone())) {
        f = stringAppendList(list![(pwd).clone(), (pd).clone(), (inFileName).clone()]);
        outFileName = (if (System::regularFileExists((f.clone()).clone())) {f} else {outFileName}).clone();
    }
    outFileName
}

pub fn hashFileNamePrefix(mut inFileNamePrefix: ArcStr) -> Result<ArcStr> {
    let mut hashStr: ArcStr = substring((intString(stringHashDjb2((inFileNamePrefix.clone()).clone()))).clone(), 1, 3)?;
    Ok(hashStr)
}

pub fn intLstString(mut lst: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(lst, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
    Ok(s)
}

pub fn sourceInfoIsEmpty(mut inInfo: SourceInfo) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match &(inInfo) {
        SourceInfo { fileName: Deref @ "", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn sourceInfoIsEqual(mut inInfo1: SourceInfo, mut inInfo2: SourceInfo) -> bool {
    let mut outIsEqual: bool;
    outIsEqual = (match (inInfo1.clone(), inInfo2.clone()) {
        (SourceInfo { .. }, SourceInfo { .. }) => inInfo1.fileName.clone() == inInfo2.fileName.clone() && inInfo1.isReadOnly.clone() == inInfo2.isReadOnly.clone() && inInfo1.lineNumberStart.clone() == inInfo2.lineNumberStart.clone() && inInfo1.columnNumberStart.clone() == inInfo2.columnNumberStart.clone() && inInfo1.lineNumberEnd.clone() == inInfo2.lineNumberEnd.clone() && inInfo1.columnNumberEnd.clone() == inInfo2.columnNumberEnd.clone(),
        _ => false,
    });
    outIsEqual
}

/* ************************************************
 * profiler stuff
 ************************************************/
pub fn profilerinit() -> Result<()> {
    { let __v = metamodelica::OrderedFloat(0.0_f64); crate::Globals::profilerTime1Index.with(|__root| *__root.borrow_mut() = __v) };
    { let __v = metamodelica::OrderedFloat(0.0_f64); crate::Globals::profilerTime2Index.with(|__root| *__root.borrow_mut() = __v) };
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    Ok(())
}

pub(crate) fn profilerresults() -> Result<()> {
    let mut tg: metamodelica::Real;
    let mut t1: metamodelica::Real;
    let mut t2: metamodelica::Real;
    tg = System::realtimeTock(ClockIndexes::RT_PROFILER0.clone())?;
    t1 = profilertime1();
    t2 = profilertime2();
    metamodelica::print((literal!("Time all: ")).clone());
    metamodelica::print((realString(tg)).clone());
    metamodelica::print((literal!("\n")).clone());
    metamodelica::print((literal!("Time t1: ")).clone());
    metamodelica::print((realString(t1)).clone());
    metamodelica::print((literal!("\n")).clone());
    metamodelica::print((literal!("Time t2: ")).clone());
    metamodelica::print((realString(t2)).clone());
    metamodelica::print((literal!("\n")).clone());
    metamodelica::print((literal!("Time all-t1-t2: ")).clone());
    metamodelica::print((realString(((tg) - (t1)) - (t2))).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub fn profilertime1() -> metamodelica::Real {
    let mut t1: metamodelica::Real;
    t1 = crate::Globals::profilerTime1Index.with(|__root| __root.borrow().clone());
    t1
}

pub fn profilertime2() -> metamodelica::Real {
    let mut t2: metamodelica::Real;
    t2 = crate::Globals::profilerTime2Index.with(|__root| __root.borrow().clone());
    t2
}

pub fn profilerstart1() -> Result<()> {
    System::realtimeTick(ClockIndexes::RT_PROFILER1.clone())?;
    Ok(())
}

pub fn profilerstart2() -> Result<()> {
    System::realtimeTick(ClockIndexes::RT_PROFILER2.clone())?;
    Ok(())
}

pub fn profilerstop1() -> Result<()> {
    let mut t: metamodelica::Real;
    t = System::realtimeTock(ClockIndexes::RT_PROFILER1.clone())?;
    { let __v = (crate::Globals::profilerTime1Index.with(|__root| __root.borrow().clone())) + (t); crate::Globals::profilerTime1Index.with(|__root| *__root.borrow_mut() = __v) };
    Ok(())
}

pub fn profilerstop2() -> Result<()> {
    let mut t: metamodelica::Real;
    t = System::realtimeTock(ClockIndexes::RT_PROFILER2.clone())?;
    { let __v = (crate::Globals::profilerTime2Index.with(|__root| __root.borrow().clone())) + (t); crate::Globals::profilerTime2Index.with(|__root| *__root.borrow_mut() = __v) };
    Ok(())
}

pub fn profilerreset1() -> () {
    { let __v = metamodelica::OrderedFloat(0.0_f64); crate::Globals::profilerTime1Index.with(|__root| *__root.borrow_mut() = __v) };
    ()
}

pub(crate) fn profilerreset2() -> () {
    { let __v = metamodelica::OrderedFloat(0.0_f64); crate::Globals::profilerTime2Index.with(|__root| *__root.borrow_mut() = __v) };
    ()
}

pub(crate) fn profilertock1() -> Result<metamodelica::Real> {
    let mut t: metamodelica::Real;
    t = System::realtimeTock(ClockIndexes::RT_PROFILER1.clone())?;
    Ok(t)
}

pub(crate) fn profilertock2() -> Result<metamodelica::Real> {
    let mut t: metamodelica::Real;
    t = System::realtimeTock(ClockIndexes::RT_PROFILER2.clone())?;
    Ok(t)
}

pub fn applyTuple21<T1: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2), mut func: Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>) -> Result<(T1, T2)> {
    pub type FuncT<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>;

    let mut outTuple: (T1, T2);
    let mut e1_1: T1;
    let mut e1_2: T1;
    let mut e2: T2;
    (e1_1, e2) = inTuple.clone();
    e1_2 = func(e1_1.clone())?;
    outTuple = if (metamodelica::ReferenceEq::reference_eq(&(e1_1), &(e1_2.clone()))) {inTuple} else {(e1_2, e2)};
    Ok(outTuple)
}

pub(crate) fn applyTuple22<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inTuple: (T1, T2), mut func: Arc<dyn ::std::ops::Fn(T2) -> Result<T2> + 'static>) -> Result<(T1, T2)> {
    pub type FuncT<T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T2) -> Result<T2> + 'static>;

    let mut outTuple: (T1, T2);
    let mut e1: T1;
    let mut e2_1: T2;
    let mut e2_2: T2;
    (e1, e2_1) = inTuple.clone();
    e2_2 = func(e2_1.clone())?;
    outTuple = if (metamodelica::ReferenceEq::reference_eq(&(e2_1), &(e2_2.clone()))) {inTuple} else {(e1, e2_2)};
    Ok(outTuple)
}

pub fn applyTuple31<T1: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq, T2: Clone + 'static + metamodelica::gc::MMTrace, T3: Clone + 'static + metamodelica::gc::MMTrace>(mut inTuple: (T1, T2, T3), mut func: Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>) -> Result<(T1, T2, T3)> {
    pub type FuncT<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<T1> + 'static>;

    let mut outTuple: (T1, T2, T3);
    let mut t1: T1;
    let mut t1_new: T1;
    let mut t2: T2;
    let mut t3: T3;
    (t1, t2, t3) = inTuple.clone();
    t1_new = func(t1.clone())?;
    outTuple = if (metamodelica::ReferenceEq::reference_eq(&(t1), &(t1_new.clone()))) {inTuple} else {(t1_new, t2, t3)};
    Ok(outTuple)
}

pub fn referenceCompare<T1: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(mut ref1: T1, mut ref2: T2) -> i32 {
    let mut result: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("referenceCompareExt"), lang: Some("C"), output_: Some(CREF_IDENT { name: "result", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "ref1", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "ref2", subscripts: Nil } }, tail: Nil } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\n  static inline int referenceCompareExt(void *ref1, void *ref2)\n  {\n    return (ref1 < ref2) ? -1 : (ref1 > ref2);\n  }\n" }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Util.mo", isReadOnly: false, lineNumberStart: 1650, columnNumberStart: 73, lineNumberEnd: 1655, columnNumberEnd: 2, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Util.mo", isReadOnly: false, lineNumberStart: 1650, columnNumberStart: 66, lineNumberEnd: 1655, columnNumberEnd: 2, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    result
}

pub(crate) fn gcd(mut a: i32, mut b: i32) -> i32 {
    '__tco: loop {
        if (b == 0) {return a} else {{ (a, b) = (b, intMod(a, b)); continue '__tco; }}
    }
}

pub(crate) fn lcm(mut a: i32, mut b: i32) -> i32 {
    let mut res: i32;
    res = if (a < 0 || b < 0) {-1} else {intDiv(a * b, gcd(a, b))};
    res
}

pub(crate) fn msb(mut n: i32) -> i32 {
    let mut res: i32 = 0;
    let mut i: i32 = n;
    while i > 0 {
        i = intBitRShift(i, 1);
        res = res + 1;
    }
    res
}

pub fn foldcallN<FT: Clone + 'static + metamodelica::gc::MMTrace>(mut n: i32, mut inFoldFunc: Arc<dyn ::std::ops::Fn(FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut i in 1..=n {
        outResult = inFoldFunc(outResult.clone())?;
    }
    Ok(outResult)
}

