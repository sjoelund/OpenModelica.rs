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

use crate::Print;
use crate::System;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Diff {
    Add = 1,
    Delete = 2,
    Equal = 3,
}
impl PartialOrd for Diff {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Diff {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for Diff {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub fn diff<T: Clone + 'static + metamodelica::gc::MMTrace>(mut seq1: Arc<metamodelica::List<T>>, mut seq2: Arc<metamodelica::List<T>>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>;
    let mut start1: i32;
    let mut end1: i32;
    let mut start2: i32;
    let mut end2: i32;
    let mut arr1: metamodelica::Array<T>;
    let mut arr2: metamodelica::Array<T>;
    arr1 = metamodelica::arrayFromVec(seq1.into_iter().cloned().collect());
    arr2 = metamodelica::arrayFromVec(seq2.into_iter().cloned().collect());
    start1 = 1;
    start2 = 1;
    end1 = metamodelica::arrayLength(arr1.clone());
    end2 = metamodelica::arrayLength(arr2.clone());
    out = diffSeq(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), isWhitespaceNotComment.clone(), toString.clone(), start1, end1, start2, end2, metamodelica::nil(), metamodelica::nil())?;
    Ok(out)
}

pub type partialPrintDiff<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, fn(T) -> Result<ArcStr>) -> Result<ArcStr> + 'static>;

pub fn printDiffTerminalColor<T: Clone + 'static + metamodelica::gc::MMTrace>(mut seq: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    let mut res: ArcStr;
    let mut open: ArcStr;
    let mut close: ArcStr;
    let mut ts: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut b: bool;
    let mut i: i32;
    i = Print::saveAndClearBuf().unwrap();
    for mut d in &*seq {
        let mut d = d.clone();
        (open, close, ts, b) = (::match_deref::match_deref! { match &(d.clone()) {
        (Diff::Equal, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!(""), literal!(""), ts.clone(), true)
        },
        (Diff::Add, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!("\u{1b}[4;32m"), literal!("\u{1b}[0m"), ts.clone(), true)
        },
        (Diff::Delete, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!("\u{1b}[9;31m"), literal!("\u{1b}[0m"), ts.clone(), true)
        },
        _ => panic!("match: no arm matched"),
    } });
        if !(ts.clone().is_empty()) && (b || true && true && true) {
            Print::printBuf((open.clone()).clone()).unwrap();
            for mut t in &*ts.clone() {
                let mut t = t.clone();
                Print::printBuf((toString(t.clone()).unwrap()).clone()).unwrap();
            }
            Print::printBuf((close.clone()).clone()).unwrap();
        }
    }
    res = (Print::getString().unwrap()).clone();
    Print::restoreBuf(i).unwrap();
    res
}

pub fn printDiffXml<T: Clone + 'static + metamodelica::gc::MMTrace>(mut seq: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    let mut res: ArcStr;
    let mut open: ArcStr;
    let mut close: ArcStr;
    let mut ts: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut b: bool;
    let mut i: i32;
    i = Print::saveAndClearBuf().unwrap();
    for mut d in &*seq {
        let mut d = d.clone();
        (open, close, ts, b) = (::match_deref::match_deref! { match &(d.clone()) {
        (Diff::Equal, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!("<equal>"), literal!("</equal>"), ts.clone(), true)
        },
        (Diff::Add, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!("<add>"), literal!("</add>"), ts.clone(), true)
        },
        (Diff::Delete, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!("<del>"), literal!("</del>"), ts.clone(), true)
        },
        _ => panic!("match: no arm matched"),
    } });
        if !(ts.clone().is_empty()) && (b || true && true && true) {
            Print::printBuf((open.clone()).clone()).unwrap();
            for mut t in &*ts.clone() {
                let mut t = t.clone();
                Print::printBuf((toString(t.clone()).unwrap()).clone()).unwrap();
            }
            Print::printBuf((close.clone()).clone()).unwrap();
        }
    }
    res = (Print::getString().unwrap()).clone();
    Print::restoreBuf(i).unwrap();
    res
}

pub fn printActual<T: Clone + 'static + metamodelica::gc::MMTrace>(mut seq: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    let mut res: ArcStr;
    let mut open: ArcStr;
    let mut close: ArcStr;
    let mut ts: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut b: bool;
    let mut i: i32;
    i = Print::saveAndClearBuf().unwrap();
    for mut d in &*seq {
        let mut d = d.clone();
        (open, close, ts, b) = (::match_deref::match_deref! { match &(d.clone()) {
        (Diff::Equal, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!(""), literal!(""), ts.clone(), true)
        },
        (Diff::Add, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!(""), literal!(""), ts.clone(), true)
        },
        (Diff::Delete, __esc_ts) => {
            ts = (*__esc_ts).clone();
            (literal!(""), literal!(""), ts.clone(), false)
        },
        _ => panic!("match: no arm matched"),
    } });
        if !(ts.clone().is_empty()) && (b || true && true && false) {
            Print::printBuf((open.clone()).clone()).unwrap();
            for mut t in &*ts.clone() {
                let mut t = t.clone();
                Print::printBuf((toString(t.clone()).unwrap()).clone()).unwrap();
            }
            Print::printBuf((close.clone()).clone()).unwrap();
        }
    }
    res = (Print::getString().unwrap()).clone();
    Print::restoreBuf(i).unwrap();
    res
}

fn diffSeq<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut inStart1: i32, mut inEnd1: i32, mut inStart2: i32, mut inEnd2: i32, mut inPrefixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut inSuffixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>;
    let mut start1: i32 = inStart1;
    let mut end1: i32 = inEnd1;
    let mut start2: i32 = inStart2;
    let mut end2: i32 = inEnd2;
    let mut len1: i32;
    let mut len2: i32;
    let mut prefixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inPrefixes.clone();
    let mut suffixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inSuffixes.clone();
    len1 = end1 - start1 + 1;
    len2 = end2 - start2 + 1;
    if len1 < 1 && len2 < 1 {
        out = List::append_reverse(prefixes, suffixes);
        return Ok(out.clone());
    } else if len1 < 1 {
        out = List::append_reverse(prefixes, metamodelica::cons((Diff::Add.clone(), ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (start2..=end2).into_iter() {
            let __x = ({let __elt = arr2.borrow()[(e.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), suffixes));
        return Ok(out.clone());
    } else if len2 < 1 {
        out = List::append_reverse(prefixes, metamodelica::cons((Diff::Delete.clone(), ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (start1..=end1).into_iter() {
            let __x = ({let __elt = arr1.borrow()[(e.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), suffixes));
        return Ok(out.clone());
    }
    if if (len1 == len2) {({
        let mut __acc: Option<bool> = None;
        for e in (1..=len1).into_iter() {
            let __x = equals(({let __elt = arr1.borrow()[(e.clone() + start1 - 1-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(e.clone() + start2 - 1-1) as usize].clone(); __elt}))?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })} else {false} {
        out = list![(Diff::Equal.clone(), ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (start1..=end1).into_iter() {
            let __x = ({let __elt = arr1.borrow()[(e.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))];
        return Ok(out.clone());
    }
    (prefixes, start1, start2) = trimCommonPrefix(arr1.clone(), start1, end1, arr2.clone(), start2, end2, equals.clone(), prefixes, isWhitespaceNotComment.clone(), toString.clone())?;
    (suffixes, end1, end2) = trimCommonSuffix(arr1.clone(), start1, end1, arr2.clone(), start2, end2, equals.clone(), suffixes, isWhitespaceNotComment.clone())?;
    if start1 != inStart1 || start2 != inStart2 || end1 != inEnd1 || end2 != inEnd2 {
        out = diffSeq(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), isWhitespaceNotComment.clone(), toString.clone(), start1, end1, start2, end2, prefixes, suffixes)?;
        return Ok(out.clone());
    } else {
        out = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(onlyAdditions(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), toString.clone(), start1, end1, start2, end2)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(onlyRemovals(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), toString.clone(), start1, end1, start2, end2)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(myersGreedyDiff(arr1.clone(), arr2.clone(), equals.clone(), start1, end1, start2, end2)?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        out = List::append_reverse(prefixes, listAppend(out, suffixes));
        return Ok(out.clone());
    }
    bail!("fail");
    Ok(out)
}

fn addToList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inlst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut ind: Diff, mut inacc: Arc<metamodelica::List<T>>, mut newd: Diff, mut t: T) -> (Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, Diff, Arc<metamodelica::List<T>>) {
    let mut lst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inlst.clone();
    let mut d: Diff = newd;
    let mut acc: Arc<metamodelica::List<T>> = inacc.clone();
    if ind == newd {
        acc = metamodelica::cons(t, acc);
    } else {
        if !(inacc.is_empty()) {
            lst = metamodelica::cons((ind, acc.reverse()), lst);
        }
        acc = list![t];
    }
    (lst, d, acc)
}

fn endList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inlst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut ind: Diff, mut inacc: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> {
    let mut lst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inlst.clone();
    if !(inacc.clone().is_empty()) {
        lst = metamodelica::cons((ind, inacc.reverse()), lst);
    }
    lst
}

fn onlyAdditions<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut start1: i32, mut end1: i32, mut start2: i32, mut end2: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: Diff = Diff::Equal.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    out = metamodelica::nil();
    while start1 + x <= end1 && start2 + y <= end2 {
        if equals(({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Equal.clone(), ({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}));
            x = x + 1;
            y = y + 1;
        } else if isWhitespace(({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Delete.clone(), ({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}));
            x = x + 1;
        } else {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Add.clone(), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}));
            y = y + 1;
        }
    }
    while start1 + x <= end1 {
        if isWhitespace(({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Delete.clone(), ({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}));
            x = x + 1;
        } else {
            bail!("fail");
        }
    }
    while start2 + y <= end2 {
        if isWhitespace(({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Add.clone(), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}));
            y = y + 1;
        } else {
            bail!("fail");
        }
    }
    out = endList(out, d, lst);
    out = out.reverse();
    Ok(out)
}

fn onlyRemovals<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut start1: i32, mut end1: i32, mut start2: i32, mut end2: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: Diff = Diff::Equal.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    out = metamodelica::nil();
    while start1 + x <= end1 && start2 + y <= end2 {
        if equals(({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Equal.clone(), ({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}));
            x = x + 1;
            y = y + 1;
        } else if isWhitespace(({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Add.clone(), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}));
            y = y + 1;
        } else {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Delete.clone(), ({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}));
            x = x + 1;
        }
    }
    while start1 + x <= end1 {
        if isWhitespace(({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Delete.clone(), ({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}));
            x = x + 1;
        } else {
            bail!("fail");
        }
    }
    while start2 + y <= end2 {
        if isWhitespace(({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}))? {
            (out, d, lst) = addToList(out.clone(), d, lst.clone(), Diff::Add.clone(), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}));
            y = y + 1;
        } else {
            bail!("fail");
        }
    }
    out = endList(out, d, lst);
    out = out.reverse();
    Ok(out)
}

fn myersGreedyDiff<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut start1: i32, mut end1: i32, mut start2: i32, mut end2: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>;
    let mut len1: i32;
    let mut len2: i32;
    let mut maxIter: i32;
    let mut sz: i32;
    let mut middle: i32;
    let mut x: i32;
    let mut y: i32;
    let mut V: metamodelica::Array<i32>;
    let mut paths: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>;
    let mut prevPath: Arc<metamodelica::List<(i32, i32)>>;
    len1 = end1 - start1 + 1;
    len2 = end2 - start2 + 1;
    maxIter = len1 + len2;
    sz = 2 * maxIter + 1;
    middle = maxIter + 1;
    V = arrayCreate(sz, 0);
    paths = arrayCreate(sz, metamodelica::nil());
    for mut D in 0..=maxIter {
        for mut k in (-(D.clone())..=D.clone()).step_by((2) as usize) {
            if k.clone() == -(D.clone()) || k.clone() != D.clone() && ({let __elt = V.borrow()[(k.clone() - 1 + middle-1) as usize].clone(); __elt}) < ({let __elt = V.borrow()[(k.clone() + 1 + middle-1) as usize].clone(); __elt}) {
                x = ({let __elt = V.borrow()[(k.clone() + 1 + middle-1) as usize].clone(); __elt});
                prevPath = ({let __elt = paths.borrow()[(k.clone() + 1 + middle-1) as usize].clone(); __elt});
            } else {
                x = ({let __elt = V.borrow()[(k.clone() - 1 + middle-1) as usize].clone(); __elt}) + 1;
                prevPath = ({let __elt = paths.borrow()[(k.clone() - 1 + middle-1) as usize].clone(); __elt});
            }
            y = x - k.clone();
            {
                let __cell0 = metamodelica::cons((x, y), prevPath.clone());
                let __idx0 = k.clone() + middle;
                paths.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
            while if (x < len1 && y < len2) {equals(({let __elt = arr1.borrow()[(start1 + x-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(start2 + y-1) as usize].clone(); __elt}))?} else {false} {
                x = x + 1;
                y = y + 1;
                {
                    let __cell1 = metamodelica::cons((x, y), ({let __elt = paths.borrow()[(k.clone() + middle-1) as usize].clone(); __elt}));
                    let __idx1 = k.clone() + middle;
                    paths.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
                }
            }
            {
                let __cell2 = x;
                let __idx2 = k.clone() + middle;
                V.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
            }
            if x >= len1 && y >= len2 {
                out = myersGreedyPathToDiff(arr1.clone(), arr2.clone(), start1, start2, ({let __elt = paths.borrow()[(k.clone() + middle-1) as usize].clone(); __elt}))?;
                return Ok(out.clone());
            }
        }
    }
    metamodelica::print((literal!("myersDiff: This cannot happen")).clone());
    bail!("fail");
    Ok(out)
}

fn myersGreedyPathToDiff<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut start1: i32, mut start2: i32, mut paths: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut x1: i32;
    let mut x2: i32;
    let mut y1: i32;
    let mut y2: i32;
    let mut d1: Diff = Diff::Equal.clone();
    let mut d2: Diff = Diff::Equal.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut t: T;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(paths.clone()) {
        Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: _ } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x2 = __pa0.clone();
    y2 = __pa1.clone();
    for mut path in &*listRest(paths)? {
        let mut path = path.clone();
        (x1, y1) = path.clone();
        if x2 - x1 == 1 && y2 - y1 == 1 {
            d1 = Diff::Equal.clone();
            t = ({let __elt = arr1.borrow()[(start1 + x1-1) as usize].clone(); __elt});
        } else if x2 - x1 == 1 && y2 == y1 {
            d1 = Diff::Delete.clone();
            t = ({let __elt = arr1.borrow()[(start1 + x1-1) as usize].clone(); __elt});
        } else if y2 - y1 == 1 && x2 == x1 {
            d1 = Diff::Add.clone();
            t = ({let __elt = arr2.borrow()[(start2 + y1-1) as usize].clone(); __elt});
        } else {
            metamodelica::print((literal!("myersGreedyPathToDiff: This cannot happen\n")).clone());
            bail!("fail");
        }
        if lst.clone().is_empty() {
            lst = list![t.clone()];
        } else if d1 == d2 {
            lst = metamodelica::cons(t.clone(), lst.clone());
        } else {
            out = metamodelica::cons((d2, lst.clone()), out.clone());
            lst = list![t.clone()];
        }
        d2 = d1;
        x2 = x1;
        y2 = y1;
    }
    if !(lst.clone().is_empty()) {
        out = metamodelica::cons((d2, lst), out);
    }
    Ok(out)
}

fn trimCommonPrefix<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut inStart1: i32, mut end1: i32, mut arr2: metamodelica::Array<T>, mut inStart2: i32, mut end2: i32, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<(Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, i32, i32)> {
    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut prefixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = acc.clone();
    let mut start1: i32 = inStart1;
    let mut start2: i32 = inStart2;
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    while start1 <= end1 && start2 <= end2 {
        if equals(({let __elt = arr1.borrow()[(start1-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(start2-1) as usize].clone(); __elt}))? {
            lst = metamodelica::cons(({let __elt = arr1.borrow()[(start1-1) as usize].clone(); __elt}), lst.clone());
            start1 = start1 + 1;
            start2 = start2 + 1;
        } else if start2 + 1 <= end2 && isWhitespaceNotComment(({let __elt = arr2.borrow()[(start2-1) as usize].clone(); __elt}))? {
            if !(equals(({let __elt = arr1.borrow()[(start1-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(start2 + 1-1) as usize].clone(); __elt}))?) {
                break;
            }
            start2 = start2 + 1;
        } else {
            break;
        }
    }
    if !(lst.clone().is_empty()) {
        prefixes = metamodelica::cons((Diff::Equal.clone(), lst.reverse()), prefixes);
    }
    Ok((prefixes, start1, start2))
}

fn trimCommonSuffix<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr1: metamodelica::Array<T>, mut start1: i32, mut inEnd1: i32, mut arr2: metamodelica::Array<T>, mut start2: i32, mut inEnd2: i32, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, i32, i32)> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut suffixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = acc.clone();
    let mut end1: i32 = inEnd1;
    let mut end2: i32 = inEnd2;
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    while start1 <= end1 && start2 <= end2 {
        if equals(({let __elt = arr1.borrow()[(end1-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(end2-1) as usize].clone(); __elt}))? {
            lst = metamodelica::cons(({let __elt = arr1.borrow()[(end1-1) as usize].clone(); __elt}), lst.clone());
            end1 = end1 - 1;
            end2 = end2 - 1;
        } else if start2 <= end2 - 1 && isWhitespaceNotComment(({let __elt = arr2.borrow()[(end2-1) as usize].clone(); __elt}))? {
            if !(equals(({let __elt = arr1.borrow()[(end1-1) as usize].clone(); __elt}), ({let __elt = arr2.borrow()[(end2 - 1-1) as usize].clone(); __elt}))?) {
                break;
            }
            end2 = end2 - 1;
        } else {
            break;
        }
    }
    if !(lst.clone().is_empty()) {
        suffixes = metamodelica::cons((Diff::Equal.clone(), lst), suffixes);
    }
    Ok((suffixes, end1, end2))
}

fn printStartToEnd<T: Clone + 'static + metamodelica::gc::MMTrace>(mut arr: metamodelica::Array<T>, mut startIndex: i32, mut endIndex: i32, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut res: ArcStr;
    res = stringAppendList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut index in (startIndex..=endIndex).into_iter() {
            let __x = toString(metamodelica::arrayGet(arr.clone(), index.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(res)
}

