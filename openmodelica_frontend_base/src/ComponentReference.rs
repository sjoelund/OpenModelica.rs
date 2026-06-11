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

use crate::Expression;
use crate::ExpressionDump;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
// do not make this public. instead use the function below.
thread_local! { static __dummyCref_TLS: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("dummy")).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() }); }
pub(crate) fn dummyCref() -> Arc<DAE::ComponentRef> { __dummyCref_TLS.with(|__t| __t.clone()) }

pub(crate) fn createEmptyCrefMemory() -> metamodelica::Array<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefMemory: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>;
    crefMemory = arrayCreate(3, metamodelica::nil());
    crefMemory
}

/* **************************************************/
/* generate a ComponentRef */
/* **************************************************/
pub fn makeDummyCref() -> Arc<DAE::ComponentRef> {
    let mut outCrefIdent: Arc<DAE::ComponentRef>;
    outCrefIdent = dummyCref().clone();
    outCrefIdent
}

pub fn makeUntypedCrefIdent(mut ident: ArcStr) -> Arc<DAE::ComponentRef> {
    let mut outCrefIdent: Arc<DAE::ComponentRef>;
    outCrefIdent = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
    outCrefIdent
}

/* **************************************************/
/* transform to other types */
/* **************************************************/
pub fn crefToPath(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, subscriptLst: Deref @ metamodelica::List::Nil, .. } => {
            Arc::new(Absyn::Path::IDENT { name: (i.clone()).clone() })
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: i, subscriptLst: Deref @ metamodelica::List::Nil, componentRef: c, .. } => {
            let mut p: Arc<Absyn::Path>;
            p = crefToPath(c.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (i.clone()).clone(), path: p.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn crefToPathIgnoreSubs(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, .. } => {
            Arc::new(Absyn::Path::IDENT { name: (i.clone()).clone() })
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: i, componentRef: c, .. } => {
            let mut p: Arc<Absyn::Path>;
            p = crefToPathIgnoreSubs(c.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (i.clone()).clone(), path: p.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn pathToCref(mut inPath: Arc<Absyn::Path>) -> Result<Arc<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPath) {
        Deref @ Absyn::Path::IDENT { name: i } => {
            return Ok(ComponentReferenceBasics::makeCrefIdent((i.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path: p } => {
            { inPath = p.clone(); continue '__tco; }
        },
        Deref @ Absyn::Path::QUALIFIED { name: i, path: p } => {
            let mut c: Arc<DAE::ComponentRef>;
            c = pathToCref(p.clone())?;
            return Ok(ComponentReferenceBasics::makeCrefQual((i.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), c.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn creffromVar(mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inVar) {
        Deref @ DAE::Var { name, ty, .. } => {
            ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComponentRef)
}

pub fn unelabCref(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outComponentRef: Arc<Absyn::ComponentRef>;
    outComponentRef = 'mc: {
        let __mc_input = inComponentRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: subs, .. } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    subs_1 = unelabSubscripts(subs.clone())?;
                    Ok(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: subs_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: subs, componentRef: cr, .. } => {
                    let mut subs_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut cr_1: Arc<Absyn::ComponentRef>;
                    cr_1 = unelabCref(cr.clone())?;
                    subs_1 = unelabSubscripts(subs.clone())?;
                    Ok(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: subs_1.clone(), componentRef: cr_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.unelabCref failed on: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComponentRef)
}

fn unelabSubscripts(mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut outAbsynSubscriptLst: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    outAbsynSubscriptLst = (::match_deref::match_deref! { match &(inSubscriptLst) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: xs } => {
            let mut xs_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            xs_1 = unelabSubscripts(xs.clone())?;
            metamodelica::cons(openmodelica_ast::Absyn::Subscript::interned_NOSUB(), xs_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: xs } => {
            let mut xs_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut e_1: Arc<Absyn::Exp>;
            xs_1 = unelabSubscripts(xs.clone())?;
            e_1 = Expression::unelabExp(e.clone())?;
            metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: e_1.clone() }), xs_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e }, tail: xs } => {
            let mut xs_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut e_1: Arc<Absyn::Exp>;
            xs_1 = unelabSubscripts(xs.clone())?;
            e_1 = Expression::unelabExp(e.clone())?;
            metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: e_1.clone() }), xs_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e }, tail: xs } => {
            let mut xs_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut e_1: Arc<Absyn::Exp>;
            xs_1 = unelabSubscripts(xs.clone())?;
            e_1 = Expression::unelabExp(e.clone())?;
            metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: e_1.clone() }), xs_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynSubscriptLst)
}

pub fn toExpCref(mut absynCref: Arc<Absyn::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(absynCref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => return Ok(ComponentReferenceBasics::makeCrefIdent((var_field!((*absynCref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), toExpCrefSubs(var_field!((*absynCref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone())?)),
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => return Ok(ComponentReferenceBasics::makeCrefQual((var_field!((*absynCref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), toExpCrefSubs(var_field!((*absynCref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone())?, toExpCref(var_field!((*absynCref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?)),
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => { absynCref = var_field!((*absynCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(); continue '__tco; },
        Deref @ Absyn::ComponentRef::WILD { .. } => return Ok(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD()),
        Deref @ Absyn::ComponentRef::ALLWILD { .. } => return Ok(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn toExpCrefSubs(mut absynSubs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut daeSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    daeSubs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut sub in (absynSubs).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => Arc::new(DAE::Subscript::INDEX { exp: Expression::fromAbsynExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone())? }),
        Deref @ Absyn::Subscript::NOSUB { .. } => openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(daeSubs)
}

pub fn crefStr(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringDelimitList(toStringList(inComponentRef), (if (Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?) {literal!("__")} else {literal!(".")}).clone());
    Ok(outString)
}

pub(crate) fn crefListStr(mut crList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    for mut cr in &*crList {
        let mut cr = cr.clone();
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*crefStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub fn crefModelicaStr(mut inComponentRef: Arc<DAE::ComponentRef>) -> ArcStr {
    let mut outString: ArcStr;
    outString = stringDelimitList(toStringList(inComponentRef), (literal!("_")).clone());
    outString
}

pub(crate) fn printComponentRefOptStr(mut inComponentRefOpt: Option<Arc<DAE::ComponentRef>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inComponentRefOpt) {
        None => {
            literal!("NONE()")
        },
        Some(cref) => {
            let mut r#str: ArcStr;
            r#str = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOME(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub(crate) fn printComponentRefStrFixDollarDer(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", subscriptLst: Deref @ metamodelica::List::Nil, componentRef: cr, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("der(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            ComponentReferenceBasics::printComponentRefStr(inComponentRef)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn debugPrintComponentRefTypeStr(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::WILD { .. } => {
            literal!("_")
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, identType: ty, subscriptLst: subs } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut str_1: ArcStr;
            str_1 = (ExpressionBasics::printListStr(subs.clone(), (std::sync::Arc::new(ExpressionDump::debugPrintSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>), (literal!(", ")).clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*if (((str_1.clone()).clone().len() as i32) > 0) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*str_1.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }} else {literal!("")}); ArcStr::from(__mm_s) }).clone();
            str2 = (TypesDump::unparseType(ty.clone())?).clone();
            r#str = stringAppendList(list![(r#str.clone()).clone(), (literal!(" [")).clone(), (str2.clone()).clone(), (literal!("]")).clone()]);
            r#str.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: s, identType: ty, subscriptLst: subs, componentRef: cr } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut strrest: ArcStr;
            let mut str_1: ArcStr;
            if Config::modelicaOutput()? {
                r#str = (ComponentReferenceBasics::printComponentRef2Str((s.clone()).clone(), subs.clone())?).clone();
                str2 = (TypesDump::unparseType(ty.clone())?).clone();
                strrest = (debugPrintComponentRefTypeStr(cr.clone())?).clone();
                r#str = stringAppendList(list![(r#str.clone()).clone(), (literal!(" [")).clone(), (str2.clone()).clone(), (literal!("] ")).clone(), (literal!("__")).clone(), (strrest.clone()).clone()]);
            } else {
                str_1 = (ExpressionBasics::printListStr(subs.clone(), (std::sync::Arc::new(ExpressionDump::debugPrintSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>), (literal!(", ")).clone())?).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*if (((str_1.clone()).clone().len() as i32) > 0) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*str_1.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }} else {literal!("")}); ArcStr::from(__mm_s) }).clone();
                str2 = (TypesDump::unparseType(ty.clone())?).clone();
                strrest = (debugPrintComponentRefTypeStr(cr.clone())?).clone();
                r#str = stringAppendList(list![(r#str.clone()).clone(), (literal!(" [")).clone(), (str2.clone()).clone(), (literal!("] ")).clone(), (literal!(".")).clone(), (strrest.clone()).clone()]);
            }
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn crefIsIdent(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn crefIsNotIdent(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isInternalCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    let mut s: ArcStr = arcstr::literal!("");
    b = (::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", .. } => false,
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", .. } => false,
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. } => {
            s = (*__esc_s).clone();
            StringUtil::startsWith((s.clone()).clone(), (literal!("$outputAlias_")).clone())
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. } => {
            s = (*__esc_s).clone();
            StringUtil::startsWith((s.clone()).clone(), (literal!("$")).clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_s, .. } => {
            s = (*__esc_s).clone();
            StringUtil::startsWith((s.clone()).clone(), (literal!("$")).clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isRecord(mut cr: Arc<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. } => {
            return true
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: comp, .. } => {
            { cr = comp.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isArrayElement(mut cr: Arc<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            return true
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            return true
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: comp, .. } => {
            { cr = comp.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isPreCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isPreviousCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStartCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(cr) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$START", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn popPreCref(mut inCR: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCR: Arc<DAE::ComponentRef>;
    outCR = (::match_deref::match_deref! { match &(inCR.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", componentRef: cr, .. } => {
            cr.clone()
        },
        _ => {
            inCR
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCR
}

pub fn popCref(mut inCR: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCR: Arc<DAE::ComponentRef>;
    outCR = (::match_deref::match_deref! { match &(inCR.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
            cr.clone()
        },
        _ => {
            inCR
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCR
}

pub(crate) fn crefIsFirstArrayElt(mut inComponentRef: Arc<DAE::ComponentRef>) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(crefLastSubs(cr.clone())?) {
                            __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        subs = __pa0.clone();
                    } else {
                        let __pa1 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(cr.clone())?) {
                            __pa1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa1.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        subs = __pa1.clone();
                    }
                    Ok(List::all(subs.clone(), (std::sync::Arc::new(Expression::subscriptIsFirst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<bool> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

pub fn crefHaveSubs(mut icr: Arc<DAE::ComponentRef>) -> bool {
    let mut ob: bool;
    ob = 'mc: {
        let __mc_input = icr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: r#str, subscriptLst: Deref @ metamodelica::List::Nil, .. } => {
                    let mut idx: i32;
                    idx = System::stringFind((r#str.clone()).clone(), (literal!("[")).clone())?;
                    let true = (idx.clone() > 0) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: Deref @ metamodelica::List::Nil, componentRef: cr, .. } => {
                    let mut b: bool;
                    b = crefHaveSubs(cr.clone());
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ob
}

pub fn crefHasScalarSubscripts(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut hasScalarSubs: bool;
    hasScalarSubs = 'mc: {
        let __mc_input = cr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ::match_deref::match_deref! { match &(crefLastSubs(cr.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    let mut tp: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(crefLastSubs(cr.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    let true = (Expression::subscriptConstants(subs.clone())) else { bail!("pattern mismatch") };
                    tp = crefLastType(cr.clone())?;
                    dims = Expression::arrayDimension(tp.clone());
                    let true = ((dims.clone().len() as i32) <= (subs.clone().len() as i32)) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    hasScalarSubs
}

pub fn crefIsScalarWithAllConstSubs(mut inCref: Arc<DAE::ComponentRef>) -> bool {
    let mut isScalar: bool;
    isScalar = 'mc: {
        let __mc_input = inCref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(inCref.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(inCref.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    dims = ComponentReferenceBasics::crefDims(inCref.clone())?;
                    let true = ((dims.clone().len() as i32) <= (subs.clone().len() as i32)) else { bail!("pattern mismatch") };
                    let true = (Expression::subscriptConstants(subs.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    isScalar
}

pub fn crefIsScalarWithVariableSubs(mut inCref: Arc<DAE::ComponentRef>) -> bool {
    let mut isScalar: bool;
    isScalar = 'mc: {
        let __mc_input = inCref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(inCref.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    dims = ComponentReferenceBasics::crefDims(inCref.clone())?;
                    let true = ((dims.clone().len() as i32) <= (subs.clone().len() as i32)) else { bail!("pattern mismatch") };
                    let false = (Expression::subscriptConstants(subs.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    isScalar
}

pub(crate) fn containWholeDim(mut inRef: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut wholedim: bool = false;
    wholedim = (::match_deref::match_deref! { match &(inRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: ty, subscriptLst: ssl } => {
            wholedim = containWholeDim2(ssl.clone(), ty.clone())?;
            wholedim
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: _, componentRef: cr } => {
            wholedim = containWholeDim(cr.clone())?;
            wholedim
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(wholedim)
}

pub fn traverseCref<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(mut cref: Arc<DAE::ComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>, mut argIn: Type_a) -> Result<Type_a> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>;

    let mut argOut: Type_a;
    argOut = 'mc: {
        let __mc_input = cref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } => {
                    let mut arg: Type_a;
                    arg = func(cref.clone(), argIn.clone())?;
                    Ok(arg.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: _, componentRef: cr } => {
                    let mut arg: Type_a;
                    arg = func(cref.clone(), argIn.clone())?;
                    Ok(traverseCref(cr.clone(), func.clone(), arg.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("traverseCref failed!")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(argOut)
}

pub fn crefIsRec(mut cref: Arc<DAE::ComponentRef>, mut isRecIn: bool) -> Result<bool> {
    let mut isRec: bool;
    isRec = isRecIn || Types::isRecord(crefLastType(cref)?);
    Ok(isRec)
}

pub fn crefGetFirstRec(mut cref: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut result: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut isRec: bool = false;
    (result, isRec) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            (cref.clone(), Types::isRecord(crefType(cref)?))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            let mut innerCref: Arc<DAE::ComponentRef>;
            if Types::isRecord(crefType(cref.clone())?) {
                result = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (var_field!((*cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), identType: var_field!((*cref).identType, DAE::ComponentRef::CREF_QUAL).clone(), subscriptLst: var_field!((*cref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone() });
                isRec = true;
            } else {
                (innerCref, isRec) = crefGetFirstRec(var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?;
                result = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (var_field!((*cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), identType: var_field!((*cref).identType, DAE::ComponentRef::CREF_QUAL).clone(), subscriptLst: var_field!((*cref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), componentRef: innerCref.clone() });
            }
            (result, isRec)
        },
        _ => {
            (cref, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((result, isRec))
}

fn containWholeDim2(mut inRef: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut wholedim: bool = false;
    wholedim = 'mc: {
        let __mc_input = (inRef, inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: _ }, Deref @ DAE::Type::T_ARRAY { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: es1 }, tail: _ }, Deref @ DAE::Type::T_ARRAY { dims: ad, .. }) => {
                    let true = (containWholeDim3(es1.clone(), ad.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: ssl }, Deref @ DAE::Type::T_ARRAY { ty: tty, dims: ad }) => {
                    let mut b: bool;
                    let mut ad = (*ad).clone();
                    ad = List::restOrEmpty(ad.clone())?;
                    b = containWholeDim2(ssl.clone(), Arc::new(DAE::Type::T_ARRAY { ty: tty.clone(), dims: ad.clone() }))?;
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: ssl }, _) => {
                    let mut wholedim: bool = wholedim.clone();
                    wholedim = containWholeDim2(ssl.clone(), inType.clone())?;
                    Ok((wholedim, wholedim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { wholedim = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(wholedim)
}

fn containWholeDim3(mut inExp: Arc<DAE::Exp>, mut ad: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> bool {
    let mut ob: bool;
    ob = 'mc: {
        let __mc_input = (inExp, ad);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl, .. }, Deref @ metamodelica::List::Cons { head: d, tail: _ }) => {
                    let mut x1: i32;
                    let mut x2: i32;
                    x1 = (expl.clone().len() as i32);
                    x2 = Expression::dimensionSize(d.clone())?;
                    let true = (intEq(x1.clone(), x2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ob
}

/* **************************************************/
/* Getter  */
/* **************************************************/
pub fn crefArrayGetFirstCref(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, identType: ty, subscriptLst: subs } => {
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut newsubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut diff: i32;
            dims = TypesDump::getDimensions(ty.clone());
            diff = (dims.clone().len() as i32) - (subs.clone().len() as i32);
            newsubs = List::fill(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: 1 }) }), diff.clone());
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (i.clone()).clone(), identType: ty.clone(), subscriptLst: listAppend(subs.clone(), newsubs.clone()) })
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: i, identType: ty, subscriptLst: subs, componentRef: cr } => {
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut newsubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut diff: i32;
            let mut cr = (*cr).clone();
            dims = TypesDump::getDimensions(ty.clone());
            diff = (dims.clone().len() as i32) - (subs.clone().len() as i32);
            newsubs = List::fill(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: 1 }) }), diff.clone());
            cr = crefArrayGetFirstCref(cr.clone())?;
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (i.clone()).clone(), identType: ty.clone(), subscriptLst: listAppend(subs.clone(), newsubs.clone()), componentRef: cr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn crefLastPath(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, subscriptLst: Deref @ metamodelica::List::Nil, .. } => {
            return Ok(Arc::new(Absyn::Path::IDENT { name: (i.clone()).clone() }))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: c, subscriptLst: Deref @ metamodelica::List::Nil, .. } => {
            { inComponentRef = c.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn crefRest(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outCref = __pa0.clone();
    Ok(outCref)
}

fn crefTypeFullComputeDims(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut dim: Arc<DAE::Dimension>;
    let mut slice_dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    dims = inDims;
    outDims = metamodelica::nil();
    for mut sub in &*inSubs {
        let mut sub = sub.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim = __pa0.clone();
        dims = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ DAE::Subscript::INDEX { .. } => (),
        Deref @ DAE::Subscript::SLICE { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(TypesDump::getDimensions(Expression::r#typeof(var_field!((*sub).exp, DAE::Subscript::SLICE).clone())?)) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            slice_dim = __pa0.clone();
            outDims = metamodelica::cons(slice_dim.clone(), outDims.clone());
            ()
        },
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            outDims = metamodelica::cons(dim.clone(), outDims.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    outDims = listAppend(outDims, dims);
    Ok(outDims)
}

pub fn crefTypeFull2(mut inCref: Arc<DAE::ComponentRef>, mut accumDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty, subscriptLst: subs, .. } => {
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut ty = (*ty).clone();
            (ty, dims) = TypesDump::flattenArrayType(ty.clone());
            dims = crefTypeFullComputeDims(dims.clone(), subs.clone())?;
            if !(accumDims.clone().is_empty()) {
                dims = List::append_reverse(dims.clone(), accumDims).reverse();
            }
            return Ok((ty.clone(), dims.clone()))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: ty, subscriptLst: subs, componentRef: cr, .. } => {
            let mut basety: Arc<DAE::Type>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut ty = (*ty).clone();
            (ty, dims) = TypesDump::flattenArrayType(ty.clone());
            dims = crefTypeFullComputeDims(dims.clone(), subs.clone())?;
            { (inCref, accumDims) = (cr.clone(), List::append_reverse(dims.clone(), accumDims)); continue '__tco; }
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("ComponentReference.crefTypeFull2 failed on cref: ")).clone())?;
            Debug::traceln((ComponentReferenceBasics::printComponentRefStr(inCref)?).clone())?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn crefTypeFull(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    let mut ty: Arc<DAE::Type>;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    (ty, dims) = crefTypeFull2(inCref, metamodelica::nil())?;
    if dims.clone().is_empty() {
        outType = ty;
    } else {
        outType = Arc::new(DAE::Type::T_ARRAY { ty: ty, dims: dims });
    }
    Ok(outType)
}

pub fn crefType(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty, .. } => {
            ty.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: ty, .. } => {
            ty.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("ComponentReference.crefType failed on cref: ")).clone())?;
            Debug::traceln((ComponentReferenceBasics::printComponentRefStr(inCref)?).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn crefLastType(mut inRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: t2, subscriptLst: _ } => {
            return Ok(t2.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: _, componentRef: cr } => {
            { inRef = cr.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn crefFirstSubs(mut inCref: Arc<DAE::ComponentRef>) -> Arc<metamodelica::List<Arc<DAE::Subscript>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscripts = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => var_field!((*inCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(),
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => var_field!((*inCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSubscripts
}

pub fn crefLastSubs(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. } => {
            return Ok(subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
            { inComponentRef = cr.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn crefTypeConsiderSubs(mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Type>> {
    let mut res: Arc<DAE::Type>;
    res = Expression::unliftArrayTypeWithSubs(crefLastSubs(cr.clone())?, crefLastType(cr)?)?;
    Ok(res)
}

pub(crate) fn crefNameType(mut inRef: Arc<DAE::ComponentRef>) -> Result<(ArcStr, Arc<DAE::Type>)> {
    let mut id: ArcStr;
    let mut res: Arc<DAE::Type>;
    (id, res) = (::match_deref::match_deref! { match &(inRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: t2, subscriptLst: _ } => {
            (name.clone(), t2.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: t2, subscriptLst: _, componentRef: _ } => {
            (name.clone(), t2.clone())
        },
        _ => {
            let mut s: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-ComponentReference.crefType failed on Cref:")).clone())?;
            s = (ComponentReferenceBasics::printComponentRefStr(inRef)?).clone();
            Debug::traceln((s.clone()).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((id, res))
}

pub fn getArrayCref(mut name: Arc<DAE::ComponentRef>) -> Option<Arc<DAE::ComponentRef>> {
    let mut arrayCref: Option<Arc<DAE::ComponentRef>>;
    arrayCref = 'mc: {
        let __mc_input = name.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arrayCrefInner: Arc<DAE::ComponentRef>;
                    let true = (crefIsFirstArrayElt(name.clone())) else { bail!("pattern mismatch") };
                    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
                        arrayCrefInner = ComponentReferenceBasics::crefStripLastSubs(name.clone())?;
                    } else {
                        arrayCrefInner = crefStripSubs(name.clone())?;
                    }
                    Ok(Some(arrayCrefInner.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    arrayCref
}

pub(crate) fn getArraySubs(mut name: Arc<DAE::ComponentRef>) -> Arc<metamodelica::List<Arc<DAE::Subscript>>> {
    let mut arraySubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    arraySubs = 'mc: {
        let __mc_input = name.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arrayCrefSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    arrayCrefSubs = ComponentReferenceBasics::crefSubs(name.clone())?;
                    Ok(arrayCrefSubs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    arraySubs
}

/* **************************************************/
/* Change  */
/* **************************************************/
pub fn crefPrependIdent(mut icr: Arc<DAE::ComponentRef>, mut ident: ArcStr, mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::ComponentRef>> {
    let mut newCr: Arc<DAE::ComponentRef>;
    newCr = (::match_deref::match_deref! { match &(icr) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id1, identType: tp1, subscriptLst: subs1 } => {
            ComponentReferenceBasics::makeCrefQual((id1.clone()).clone(), tp1.clone(), subs1.clone(), ComponentReferenceBasics::makeCrefIdent((ident).clone(), tp, subs))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, identType: tp1, subscriptLst: subs1, componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = crefPrependIdent(cr.clone(), (ident).clone(), subs, tp)?;
            ComponentReferenceBasics::makeCrefQual((id1.clone()).clone(), tp1.clone(), subs1.clone(), cr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newCr)
}

pub fn crefPrefixDer(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::derivativeNamePrefix)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub fn crefPrefixPre(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::preNamePrefix)).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub fn getConcealedCref() -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let mut ident: ArcStr;
    ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$concealed")); __mm_s.push_str(&*intString(System::tmpTick() + 1)); ArcStr::from(__mm_s) }).clone();
    outCref = ComponentReferenceBasics::makeCrefIdent((ident).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    outCref
}

pub fn crefPrefixPrevious(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::previousNamePrefix)).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub fn crefPrefixAux(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::auxNamePrefix)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub fn crefRemovePrePrefix(mut cref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut cref: Arc<DAE::ComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", .. } => var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(),
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$START", .. } => var_field!((*cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(),
        _ => cref,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn crefPrefixStart(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::startNamePrefix)).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub fn crefPrefixString(mut inString: ArcStr, mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual((inString).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub(crate) fn crefPrefixStringList(mut inStrings: Arc<metamodelica::List<ArcStr>>, mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &((inStrings, inCref.clone())) {
        (Deref @ metamodelica::List::Cons { head: r#str, tail: rest_str }, cref) => {
            let mut cref = (*cref).clone();
            cref = crefPrefixStringList(rest_str.clone(), cref.clone());
            cref = crefPrefixString((r#str.clone()).clone(), cref.clone());
            cref.clone()
        },
        _ => {
            inCref
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub(crate) fn prefixWithPath(mut inCref: Arc<DAE::ComponentRef>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPath) {
        Deref @ Absyn::Path::IDENT { name } => {
            return Ok(Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil(), componentRef: inCref }))
        },
        Deref @ Absyn::Path::QUALIFIED { name, path: rest_path } => {
            let mut cref: Arc<DAE::ComponentRef>;
            cref = prefixWithPath(inCref, rest_path.clone())?;
            return Ok(Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil(), componentRef: cref.clone() }))
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path: rest_path } => {
            { (inCref, inPath) = (inCref, rest_path.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn prependStringCref(mut inString: ArcStr, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &((inString, inComponentRef)) {
        (p, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i, identType: t2, subscriptLst: s, componentRef: c }) => {
            let mut i_1: ArcStr;
            i_1 = (stringAppend((p.clone()).clone(), (i.clone()).clone())).clone();
            ComponentReferenceBasics::makeCrefQual((i_1.clone()).clone(), t2.clone(), s.clone(), c.clone())
        },
        (p, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, identType: t2, subscriptLst: s }) => {
            let mut i_1: ArcStr;
            i_1 = (stringAppend((p.clone()).clone(), (i.clone()).clone())).clone();
            ComponentReferenceBasics::makeCrefIdent((i_1.clone()).clone(), t2.clone(), s.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn appendStringCref(mut r#str: ArcStr, mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut ocr: Arc<DAE::ComponentRef>;
    ocr = joinCrefs(cr, Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() }))?;
    Ok(ocr)
}

pub fn appendStringFirstIdent(mut inString: ArcStr, mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: cr } => {
            let mut id = (*id).clone();
            id = (stringAppend((id.clone()).clone(), (inString).clone())).clone();
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: cr.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: subs } => {
            let mut id = (*id).clone();
            id = (stringAppend((id.clone()).clone(), (inString).clone())).clone();
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn appendStringLastIdent(mut inString: ArcStr, mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = appendStringLastIdent((inString).clone(), cr.clone())?;
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: cr.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: subs } => {
            let mut id = (*id).clone();
            id = (stringAppend((id.clone()).clone(), (inString).clone())).clone();
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn joinCrefs(mut inComponentRef1: Arc<DAE::ComponentRef>, mut inComponentRef2: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &((inComponentRef1, inComponentRef2)) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: t2, subscriptLst: sub }, cr2) => {
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), t2.clone(), sub.clone(), cr2.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: sub, componentRef: cr }, cr2) => {
            let mut cr_1: Arc<DAE::ComponentRef>;
            cr_1 = joinCrefs(cr.clone(), cr2.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), t2.clone(), sub.clone(), cr_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn joinCrefsR(mut inComponentRef2: Arc<DAE::ComponentRef>, mut inComponentRef1: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &((inComponentRef2, inComponentRef1)) {
        (cr2, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: t2, subscriptLst: sub }) => {
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), t2.clone(), sub.clone(), cr2.clone())
        },
        (cr2, Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: sub, componentRef: cr }) => {
            let mut cr_1: Arc<DAE::ComponentRef>;
            cr_1 = joinCrefs(cr.clone(), cr2.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), t2.clone(), sub.clone(), cr_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn joinCrefsExp(mut exp: Arc<DAE::Exp>, mut cref: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Exp>, Arc<DAE::ComponentRef>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cref: Arc<DAE::ComponentRef> = cref;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp } => {
            let mut cr = (*cr).clone();
            cr = joinCrefs(cref.clone(), cr.clone())?;
            Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() })
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cref))
}

pub fn subscriptCref(mut inComponentRef: Arc<DAE::ComponentRef>, mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &((inComponentRef, inSubscriptLst)) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: sub, identType: t2 }, newsub) => {
            let mut newsub_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            newsub_1 = listAppend(sub.clone(), newsub.clone());
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), t2.clone(), newsub_1.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: sub, componentRef: cref, identType: t2 }, newsub) => {
            let mut cref_1: Arc<DAE::ComponentRef>;
            cref_1 = subscriptCref(cref.clone(), newsub.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), t2.clone(), sub.clone(), cref_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn subscriptCrefWithInt(mut inComponentRef: Arc<DAE::ComponentRef>, mut inSubscript: i32) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: subs, identType: ty } => {
            let mut new_sub: Arc<DAE::Subscript>;
            let mut subs = (*subs).clone();
            let mut ty = (*ty).clone();
            new_sub = Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: inSubscript }) });
            subs = List::appendElt(new_sub.clone(), subs.clone());
            ty = Expression::unliftArray(ty.clone())?;
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty.clone(), subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: subs, componentRef: rest_cref, identType: ty } => {
            let mut rest_cref = (*rest_cref).clone();
            rest_cref = subscriptCrefWithInt(rest_cref.clone(), inSubscript)?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), subs.clone(), rest_cref.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn crefSetLastSubs(mut inComponentRef: Arc<DAE::ComponentRef>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: tp, .. } => {
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), tp.clone(), inSubs)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp, subscriptLst: subs, componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = crefSetLastSubs(cr.clone(), inSubs)?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), tp.clone(), subs.clone(), cr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn crefApplySubs(mut inComponentRef: Arc<DAE::ComponentRef>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: tp @ Deref @ DAE::Type::T_ARRAY { dims, .. }, subscriptLst: subs } => {
            if (subs.clone().len() as i32) + (inSubs.clone().len() as i32) > (dims.clone().len() as i32) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefApplySubs [")); __mm_s.push_str(&*ExpressionBasics::printListStr(inSubs.clone(), (std::sync::Arc::new(ExpressionBasics::printSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>), (literal!(",")).clone())?); __mm_s.push_str(&*literal!("] to ident ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*intString((dims.clone().len() as i32))); __mm_s.push_str(&*literal!(" dimensions\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
                bail!("fail");
            }
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), tp.clone(), listAppend(subs.clone(), inSubs))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp @ Deref @ DAE::Type::T_ARRAY { dims, .. }, subscriptLst: subs, componentRef: cr } => {
            let mut subs1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut subs2: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut cr = (*cr).clone();
            if (inSubs.clone().len() as i32) > (dims.clone().len() as i32) - (subs.clone().len() as i32) {
                (subs1, subs2) = List::split(inSubs.clone(), (dims.clone().len() as i32) - (subs.clone().len() as i32))?;
                cr = crefApplySubs(cr.clone(), subs2.clone())?;
            } else {
                subs1 = inSubs.clone();
            }
            if (subs.clone().len() as i32) + (subs1.clone().len() as i32) > (dims.clone().len() as i32) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefApplySubs [")); __mm_s.push_str(&*ExpressionBasics::printListStr(inSubs, (std::sync::Arc::new(ExpressionBasics::printSubscriptStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>), (literal!(",")).clone())?); __mm_s.push_str(&*literal!("] to qual ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*intString((dims.clone().len() as i32))); __mm_s.push_str(&*literal!(" dimensions\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
                bail!("fail");
            }
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), tp.clone(), listAppend(subs.clone(), subs1.clone()), cr.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp, subscriptLst: subs, componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = crefApplySubs(cr.clone(), inSubs)?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), tp.clone(), subs.clone(), cr.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefApplySubs to non array ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComponentRef)
}

pub(crate) fn crefSetType(mut cref: Arc<DAE::ComponentRef>, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => DAE::ComponentRef::CREF_IDENT; identType = ty);
            cref
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => DAE::ComponentRef::CREF_QUAL; identType = ty);
            cref
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefSetType")); __mm_s.push_str(&*literal!(" was applied on a cref that has no type: ")); __mm_s.push_str(&*crefStr(cref)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn crefSetLastType(mut inRef: Arc<DAE::ComponentRef>, mut newType: Arc<DAE::Type>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outRef: Arc<DAE::ComponentRef>;
    outRef = (::match_deref::match_deref! { match &(inRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: _, subscriptLst: subs } => {
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), newType, subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: child } => {
            let mut child = (*child).clone();
            child = crefSetLastType(child.clone(), newType)?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), subs.clone(), child.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRef)
}

pub(crate) fn replaceCrefSliceSub(mut inCr: Arc<DAE::ComponentRef>, mut newSub: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCr: Arc<DAE::ComponentRef>;
    outCr = 'mc: {
        let __mc_input = inCr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType, subscriptLst: subs } => {
                    let mut subs = (*subs).clone();
                    subs = replaceSliceSub(subs.clone(), newSub.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), identType.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { identType: t2, subscriptLst: subs, .. } => {
                    let mut child: Arc<DAE::ComponentRef>;
                    let true = ((Expression::arrayTypeDimensions(t2.clone())?.len() as i32) >= (subs.clone().len() as i32) + 1) else { bail!("pattern mismatch") };
                    child = subscriptCref(inCr.clone(), newSub.clone())?;
                    Ok(child.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { identType: t2, subscriptLst: subs, .. } => {
                    let mut child: Arc<DAE::ComponentRef>;
                    let false = ((Expression::arrayTypeDimensions(t2.clone())?.len() as i32) >= (subs.clone().len() as i32) + (newSub.clone().len() as i32)) else { bail!("pattern mismatch") };
                    child = subscriptCref(inCr.clone(), newSub.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("WARNING - Expression.replaceCref_SliceSub setting subscript last, not containing dimension\n")).clone())?;
                    }
                    Ok(child.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType, subscriptLst: subs, componentRef: child } => {
                    let mut subs = (*subs).clone();
                    subs = replaceSliceSub(subs.clone(), newSub.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), identType.clone(), subs.clone(), child.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType, subscriptLst: subs, componentRef: child } => {
                    let true = ((Expression::arrayTypeDimensions(identType.clone())?.len() as i32) >= (subs.clone().len() as i32) + 1) else { bail!("pattern mismatch") };
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), identType.clone(), listAppend(subs.clone(), newSub.clone()), child.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType, subscriptLst: subs, componentRef: child } => {
                    let mut child = (*child).clone();
                    child = replaceCrefSliceSub(child.clone(), newSub.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), identType.clone(), subs.clone(), child.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Expression.replaceCref_SliceSub failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCr)
}

fn replaceSliceSub(mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inSub: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut osubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    osubs = (::match_deref::match_deref! { match &(inSubs) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: _ }, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = listAppend(inSub, subs.clone());
            subs.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = listAppend(inSub, subs.clone());
            subs.clone()
        },
        Deref @ metamodelica::List::Cons { head: sub, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = replaceSliceSub(subs.clone(), inSub)?;
            metamodelica::cons(sub.clone(), subs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osubs)
}

pub(crate) fn stripCrefIdentSliceSubs(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: subs, identType: ty } => {
            let mut subs = (*subs).clone();
            subs = removeSliceSubs(subs.clone());
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty.clone(), subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, subscriptLst: subs, ident: id } => {
            outCref = stripCrefIdentSliceSubs(cr.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), subs.clone(), outCref)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn stripArrayCref(mut crefIn: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, i32, Option<Arc<DAE::ComponentRef>>)> {
    let mut crefHead: Arc<DAE::ComponentRef>;
    let mut idxOut: i32;
    let mut crefTail: Option<Arc<DAE::ComponentRef>>;
    (crefHead, idxOut, crefTail) = (::match_deref::match_deref! { match &(crefIn) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: idx } }, tail: Deref @ metamodelica::List::Nil }, identType: ty } => {
            (ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty.clone(), metamodelica::nil()), idx.clone(), None)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: idx } }, tail: Deref @ metamodelica::List::Nil }, ident: id } => {
            (ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty.clone(), metamodelica::nil()), idx.clone(), Some(cr.clone()))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, ident: id, .. } => {
            let mut outCref: Arc<DAE::ComponentRef>;
            outCref = stripCrefIdentSliceSubs(cr.clone())?;
            (ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), metamodelica::nil(), outCref.clone()), -1, None)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((crefHead, idxOut, crefTail))
}

fn removeSliceSubs(mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Arc<metamodelica::List<Arc<DAE::Subscript>>> {
    let mut osubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    for mut s in &*subs {
        let mut s = s.clone();
        osubs = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ DAE::Subscript::SLICE { .. } => osubs.clone(),
        _ => metamodelica::cons(s.clone(), osubs.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    osubs = Dangerous::listReverseInPlace(osubs);
    osubs
}

pub fn crefStripSubs(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, .. } => {
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty.clone(), metamodelica::nil())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, ident: id, .. } => {
            outCref = crefStripSubs(cr.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), metamodelica::nil(), outCref)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn crefStripSubsExceptModelSubs(mut inCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    fn is_model_array(mut ty: Arc<DAE::Type>) -> bool {
        let mut res: bool;
        let mut state: ClassInf::State = <ClassInf::State as ::std::default::Default>::default();
        res = (::match_deref::match_deref! { match &(ty) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: __esc_state, .. }, .. } => {
            state = (*__esc_state).clone();
            (match state.clone() {
        ClassInf::State::MODEL { .. } => true,
        ClassInf::State::BLOCK { .. } => true,
        _ => false,
    })
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } if (is_model_array(var_field!((*inCref).identType, DAE::ComponentRef::CREF_IDENT).clone())) => {
            inCref.clone()
        },
        cref @ Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } if (is_model_array(var_field!((*inCref).identType, DAE::ComponentRef::CREF_QUAL).clone())) => {
            let mut cref = (*cref).clone();
            outCref = crefStripSubsExceptModelSubs(cr.clone());
            assign_variant_field!(cref => DAE::ComponentRef::CREF_QUAL; componentRef = outCref.clone());
            cref.clone()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, .. } => {
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), ty.clone(), metamodelica::nil())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, ident: id, .. } => {
            outCref = crefStripSubsExceptModelSubs(cr.clone());
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), metamodelica::nil(), outCref.clone())
        },
        _ => {
            inCref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn crefStripPrefix(mut cref: Arc<DAE::ComponentRef>, mut prefix: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((cref, prefix)) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, identType: _, subscriptLst: subs1, componentRef: cr1 }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, identType: _, subscriptLst: subs2 }) => {
            let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
            let true = (ExpressionBasics::subscriptEqual(subs1.clone(), subs2.clone())?) else { bail!("pattern mismatch") };
            return Ok(cr1.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, identType: _, subscriptLst: subs1, componentRef: cr1 }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: id2, identType: _, subscriptLst: subs2, componentRef: cr2 }) => {
            let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
            let true = (ExpressionBasics::subscriptEqual(subs1.clone(), subs2.clone())?) else { bail!("pattern mismatch") };
            { (cref, prefix) = (cr1.clone(), cr2.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn crefStripLastIdent(mut inCr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCr: Arc<DAE::ComponentRef>;
    outCr = (::match_deref::match_deref! { match &(inCr) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: subs, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } } => {
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), t2.clone(), subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: subs, componentRef: cr } => {
            let mut cr1: Arc<DAE::ComponentRef>;
            cr1 = crefStripLastIdent(cr.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), t2.clone(), subs.clone(), cr1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCr)
}

pub fn crefStripIterSub(mut inComponentRef: Arc<DAE::ComponentRef>, mut iter: ArcStr) -> Arc<DAE::ComponentRef> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut index: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_ident, identType: __esc_ty, subscriptLst: __esc_subs @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_index, .. }, .. } }, tail: Deref @ metamodelica::List::Nil } } => {
            ident = (*__esc_ident).clone();
            ty = (*__esc_ty).clone();
            subs = (*__esc_subs).clone();
            index = (*__esc_index).clone();
            ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), if (literal!("") == iter.clone() || index.clone() == iter) {metamodelica::nil()} else {subs.clone()})
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_ident, identType: __esc_ty, componentRef: __esc_cref, subscriptLst: __esc_subs @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_index, .. }, .. } }, tail: Deref @ metamodelica::List::Nil } } => {
            ident = (*__esc_ident).clone();
            ty = (*__esc_ty).clone();
            cref = (*__esc_cref).clone();
            subs = (*__esc_subs).clone();
            index = (*__esc_index).clone();
            if literal!("") == iter.clone() || index.clone() == iter.clone() {
                subs = metamodelica::nil();
            } else {
                cref = crefStripIterSub(cref.clone(), (iter).clone());
            }
            ComponentReferenceBasics::makeCrefQual((ident.clone()).clone(), ty.clone(), subs.clone(), cref.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_ident, identType: __esc_ty, componentRef: __esc_cref, subscriptLst: __esc_subs } => {
            ident = (*__esc_ident).clone();
            ty = (*__esc_ty).clone();
            cref = (*__esc_cref).clone();
            subs = (*__esc_subs).clone();
            ComponentReferenceBasics::makeCrefQual((ident.clone()).clone(), ty.clone(), subs.clone(), crefStripIterSub(cref.clone(), (iter).clone()))
        },
        _ => inComponentRef,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComponentRef
}

pub fn crefStripFirstIdent(mut inCr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCr: Arc<DAE::ComponentRef>;
    outCr = (::match_deref::match_deref! { match &(inCr) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
            cr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCr)
}

pub(crate) fn crefStripLastSubsStringified(mut inComponentRef: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = 'mc: {
        let __mc_input = inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: t2, subscriptLst: Deref @ metamodelica::List::Nil } => {
                    let mut lst: Arc<metamodelica::List<ArcStr>>;
                    let mut lst_1: Arc<metamodelica::List<ArcStr>>;
                    let mut id_1: ArcStr;
                    lst = Util::stringSplitAtChar((id.clone()).clone(), (literal!("[")).clone())?;
                    lst_1 = List::stripLast(lst.clone())?;
                    id_1 = stringDelimitList(lst_1.clone(), (literal!("[")).clone());
                    Ok(ComponentReferenceBasics::makeCrefIdent((id_1.clone()).clone(), t2.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr => {
                    Ok(cr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outComponentRef
}

pub(crate) fn stringifyComponentRef(mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    let mut cr_1: Arc<DAE::ComponentRef>;
    let mut crs: ArcStr;
    let mut ty: Arc<DAE::Type>;
    subs = crefLastSubs(cr.clone())?;
    cr_1 = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
    crs = (ComponentReferenceBasics::printComponentRefStr(cr_1)?).clone();
    ty = crefLastType(cr)?;
    outComponentRef = ComponentReferenceBasics::makeCrefIdent((crs).clone(), ty, subs);
    Ok(outComponentRef)
}

/* **************************************************/
/* Print and Dump */
/* **************************************************/
pub(crate) fn printComponentRef(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inComponentRef) {
        Deref @ DAE::ComponentRef::WILD { .. } => {
            Print::printBuf((literal!("_")).clone())?;
            ()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, subscriptLst: subs, .. } => {
            printComponentRef2((s.clone()).clone(), subs.clone())?;
            ()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: s, subscriptLst: subs, componentRef: cr, .. } => {
            if Config::modelicaOutput()? {
                printComponentRef2((s.clone()).clone(), subs.clone())?;
                Print::printBuf((literal!("__")).clone())?;
                printComponentRef(cr.clone())?;
            } else {
                printComponentRef2((s.clone()).clone(), subs.clone())?;
                Print::printBuf((literal!(".")).clone())?;
                printComponentRef(cr.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printComponentRef2(mut inString: ArcStr, mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inString, inSubscriptLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (s, Deref @ metamodelica::List::Nil) => {
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (s, l) => {
                    if Config::modelicaOutput()? {
                        Print::printBuf((s.clone()).clone())?;
                        Print::printBuf((literal!("_L")).clone())?;
                        ExpressionDump::printList(l.clone(), (std::sync::Arc::new(ExpressionDump::printSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<()> + 'static>), (literal!(",")).clone())?;
                        Print::printBuf((literal!("_R")).clone())?;
                    } else {
                        Print::printBuf((s.clone()).clone())?;
                        Print::printBuf((literal!("[")).clone())?;
                        ExpressionDump::printList(l.clone(), (std::sync::Arc::new(ExpressionDump::printSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<()> + 'static>), (literal!(",")).clone())?;
                        Print::printBuf((literal!("]")).clone())?;
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn printComponentRefList(mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<()> {
    let mut buffer: ArcStr;
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(crs, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
    metamodelica::print((buffer).clone());
    Ok(())
}

pub fn replaceWholeDimSubscript(mut icr: Arc<DAE::ComponentRef>, mut index: i32) -> Result<Arc<DAE::ComponentRef>> {
    let mut ocr: Arc<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = icr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: et, subscriptLst: ss, componentRef: cr } => {
                    let mut ss = (*ss).clone();
                    ss = replaceWholeDimSubscript2(ss.clone(), index)?;
                    Ok(Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: et.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: et, subscriptLst: ss, componentRef: cr } => {
                    let mut cr = (*cr).clone();
                    cr = replaceWholeDimSubscript(cr.clone(), index)?;
                    Ok(Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: et.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: et, subscriptLst: ss } => {
                    let mut ss = (*ss).clone();
                    ss = replaceWholeDimSubscript2(ss.clone(), index)?;
                    Ok(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: et.clone(), subscriptLst: ss.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ocr)
}

pub(crate) fn replaceWholeDimSubscript2(mut isubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut index: i32) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut osubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    osubs = (::match_deref::match_deref! { match &(isubs) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs } => {
            let mut sub: Arc<DAE::Subscript>;
            sub = Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: index }) });
            metamodelica::cons(sub.clone(), subs.clone())
        },
        Deref @ metamodelica::List::Cons { head: sub, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = replaceWholeDimSubscript2(subs.clone(), index)?;
            metamodelica::cons(sub.clone(), subs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osubs)
}

pub fn splitCrefLast(mut inCref: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)> {
    let mut outPrefixCref: Arc<DAE::ComponentRef>;
    let mut outLastCref: Arc<DAE::ComponentRef>;
    (outPrefixCref, outLastCref) = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: last @ Deref @ DAE::ComponentRef::CREF_IDENT { .. } } => {
            (Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() }), last.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: last } => {
            let mut prefix: Arc<DAE::ComponentRef>;
            let mut last = (*last).clone();
            (prefix, last) = splitCrefLast(last.clone())?;
            (Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: prefix.clone() }), last.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outPrefixCref, outLastCref))
}

pub fn firstNCrefs(mut inCref: Arc<DAE::ComponentRef>, mut nIn: i32) -> Arc<DAE::ComponentRef> {
    let mut outFirstCrefs: Arc<DAE::ComponentRef>;
    outFirstCrefs = (::match_deref::match_deref! { match &((inCref.clone(), nIn)) {
        (_, 0) => {
            inCref
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: _ }, 1) => {
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() })
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, _) => {
            inCref
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: last }, _) => {
            let mut prefix: Arc<DAE::ComponentRef>;
            prefix = firstNCrefs(last.clone(), nIn - 1);
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: prefix.clone() })
        },
        _ => {
            inCref
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFirstCrefs
}

pub(crate) fn splitCrefFirst(mut inCref: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)> {
    let mut outCrefFirst: Arc<DAE::ComponentRef>;
    let mut outCrefRest: Arc<DAE::ComponentRef>;
    let mut id: ArcStr;
    let mut ty: Arc<DAE::Type>;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __pa0, identType: __pa1, subscriptLst: __pa2, componentRef: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    id = __pa0.clone();
    ty = __pa1.clone();
    subs = __pa2.clone();
    outCrefRest = __pa3.clone();
    outCrefFirst = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id).clone(), identType: ty, subscriptLst: subs });
    Ok((outCrefFirst, outCrefRest))
}

pub(crate) fn toStringList(mut inCref: Arc<DAE::ComponentRef>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>>;
    outStringList = Dangerous::listReverseInPlace(toStringList_tail(inCref, metamodelica::nil()));
    outStringList
}

fn toStringList_tail(mut inCref: Arc<DAE::ComponentRef>, mut inAccumStrings: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, componentRef: cref, .. } => {
            { (inCref, inAccumStrings) = (cref.clone(), metamodelica::cons((id.clone()).clone(), inAccumStrings)); continue '__tco; }
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. } => {
            return metamodelica::cons((id.clone()).clone(), inAccumStrings)
        },
        _ => {
            return metamodelica::nil()
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn crefDepth(mut inCref: Arc<DAE::ComponentRef>) -> Result<i32> {
    let mut depth: i32;
    depth = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::WILD { .. } => {
            0
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            1
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: n, .. } => {
            crefDepth1(n.clone(), 1)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(depth)
}

fn crefDepth1(mut inCref: Arc<DAE::ComponentRef>, mut iDepth: i32) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::WILD { .. } => {
            return Ok(iDepth)
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            return Ok(1 + iDepth)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: n, .. } => {
            { (inCref, iDepth) = (n.clone(), 1 + iDepth); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn expandCref(mut inCref: Arc<DAE::ComponentRef>, mut expandRecord: bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCref: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    outCref = (match expandRecord {
        _ => expandCref_impl(inCref, expandRecord),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ComponentReference.expandCref failed on ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    });
    Ok(outCref)
}

pub(crate) fn expandCref_impl(mut inCref: Arc<DAE::ComponentRef>, mut expandRecord: bool) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut outCref: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), expandRecord);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. }, subscriptLst: Deref @ metamodelica::List::Nil }, true) => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefs = List::map(varLst.clone(), (std::sync::Arc::new(creffromVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    crefs = List::map1r(crefs.clone(), (std::sync::Arc::new(joinCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), inCref.clone())?;
                    Ok(List::mapFlat(crefs.clone(), (std::sync::Arc::new({ let __pe_b1 = true; move |__pe_a0| Ok(expandCref_impl(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: Deref @ metamodelica::List::Nil }, true) => {
                    let mut basety: Arc<DAE::Type>;
                    let mut correctTy: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>;
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(ty.clone())) {
                        (__pa1 @ Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    basety = __pa1.clone();
                    dims = __pa2.clone();
                    correctTy = Arc::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), (dims.clone().len() as i32));
                    crefs = expandCref2((id.clone()).clone(), correctTy.clone(), subs.clone(), dims.clone())?;
                    Ok(expandCrefLst(crefs.clone(), varLst.clone(), metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: Deref @ metamodelica::List::Nil }, _) => {
                    let mut basety: Arc<DAE::Type>;
                    let mut correctTy: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    (basety, dims) = TypesDump::flattenArrayType(ty.clone());
                    correctTy = Arc::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), (dims.clone().len() as i32));
                    Ok(expandCref2((id.clone()).clone(), correctTy.clone(), subs.clone(), dims.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: subs }, true) => {
                    let mut basety: Arc<DAE::Type>;
                    let mut correctTy: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>;
                    let mut missing_subs: i32;
                    let mut subs = (*subs).clone();
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(ty.clone())) {
                        (__pa1 @ Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    basety = __pa1.clone();
                    dims = __pa2.clone();
                    correctTy = Arc::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    missing_subs = (dims.clone().len() as i32) - (subs.clone().len() as i32);
                    if missing_subs.clone() > 0 {
                        subs = listAppend(subs.clone(), List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), missing_subs.clone()));
                    }
                    crefs = expandCref2((id.clone()).clone(), correctTy.clone(), subs.clone(), dims.clone())?;
                    Ok(expandCrefLst(crefs.clone(), varLst.clone(), metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: subs }, _) => {
                    let mut basety: Arc<DAE::Type>;
                    let mut correctTy: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut missing_subs: i32;
                    let mut subs = (*subs).clone();
                    (basety, dims) = TypesDump::flattenArrayType(ty.clone());
                    correctTy = Arc::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    missing_subs = (dims.clone().len() as i32) - (subs.clone().len() as i32);
                    if missing_subs.clone() > 0 {
                        subs = listAppend(subs.clone(), List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), missing_subs.clone()));
                    }
                    Ok(expandCref2((id.clone()).clone(), correctTy.clone(), subs.clone(), dims.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: subs, componentRef: cref }, _) => {
                    let mut basety: Arc<DAE::Type>;
                    let mut correctTy: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut cref = (*cref).clone();
                    crefs = expandCref_impl(cref.clone(), expandRecord);
                    (basety, dims) = TypesDump::flattenArrayType(ty.clone());
                    correctTy = Arc::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    cref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: correctTy.clone(), subscriptLst: subs.clone() });
                    crefs2 = expandCref_impl(cref.clone(), false);
                    crefs2 = crefs2.clone().reverse();
                    crefs = expandCrefQual(crefs2.clone(), crefs.clone())?;
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: cref }, _) => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefs = expandCref_impl(cref.clone(), expandRecord);
                    crefs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut c in (crefs.clone()).into_iter().cloned() {
                    let __x = ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), ty.clone(), subs.clone(), c.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![inCref.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn expandCrefLst(mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inCrefsAcc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inCrefs) {
        Deref @ metamodelica::List::Nil => {
            return Ok(List::flatten(inCrefsAcc)?)
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: rest } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crefs = List::map(varLst.clone(), (std::sync::Arc::new(creffromVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            crefs = List::map1r(crefs.clone(), (std::sync::Arc::new(joinCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cr.clone())?;
            { (inCrefs, varLst, inCrefsAcc) = (rest.clone(), varLst, metamodelica::cons(crefs.clone(), inCrefsAcc)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn expandCrefQual(mut inHeadCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inRestCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    for mut cref in &*inHeadCrefs {
        let mut cref = cref.clone();
        crefs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut rest_cref in (inRestCrefs.clone()).into_iter().cloned() {
            let __x = joinCrefs(cref.clone(), rest_cref.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        outCrefs = listAppend(crefs.clone(), outCrefs.clone());
    }
    Ok(outCrefs)
}

fn expandCref2(mut inId: ArcStr, mut inType: Arc<DAE::Type>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>;
    subslst = List::threadMap(inSubscripts, inDimensions, (std::sync::Arc::new(Expression::expandSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>, Arc<DAE::Dimension>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> + 'static>))?;
    subslst = List::combination(subslst);
    for mut subs in &*subslst {
        let mut subs = subs.clone();
        outCrefs = metamodelica::cons(ComponentReferenceBasics::makeCrefIdent((inId.clone()).clone(), inType.clone(), subs.clone()), outCrefs.clone());
    }
    outCrefs = outCrefs.reverse();
    Ok(outCrefs)
}

pub fn replaceSubsWithString(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst: Deref @ metamodelica::List::Nil, componentRef: cr } => {
            let mut cr1: Arc<DAE::ComponentRef>;
            cr1 = replaceSubsWithString(cr.clone())?;
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: metamodelica::nil(), componentRef: cr1.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef: cr } => {
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut identType = (*identType).clone();
            let mut cr = (*cr).clone();
            identType = Expression::unliftArrayTypeWithSubs(subscriptLst.clone(), identType.clone())?;
            cr1 = replaceSubsWithString(cr.clone())?;
            cr = makeCrefsFromSubScriptLst(subscriptLst.clone(), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: metamodelica::nil() }))?;
            joinCrefs(cr.clone(), cr1.clone())?
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. } => {
            inCref
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst } => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut identType = (*identType).clone();
            identType = Expression::unliftArrayTypeWithSubs(subscriptLst.clone(), identType.clone())?;
            cr = makeCrefsFromSubScriptLst(subscriptLst.clone(), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: metamodelica::nil() }))?;
            cr.clone()
        },
        Deref @ DAE::ComponentRef::WILD { .. } => {
            inCref
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub(crate) fn makeCrefsFromSubScriptLst(mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inPreCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = inPreCref.clone();
    for mut subScript in &*inSubscriptLst {
        let mut subScript = subScript.clone();
        outCref = (::match_deref::match_deref! { match &(subScript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            let mut cr: Arc<DAE::ComponentRef>;
            cr = makeCrefsFromSubScriptExp(e.clone())?;
            joinCrefs(outCref.clone(), cr.clone())?
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = (ExpressionBasics::printSubscriptStr(subScript.clone())?).clone();
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ComponentReference.makeCrefsFromSubScriptLst for:")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(outCref)
}

pub(crate) fn makeCrefsFromSubScriptExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            let mut r#str: ArcStr;
            r#str = (ExpressionBasics::printExpStr(inExp)?).clone();
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() })
        },
        Deref @ DAE::Exp::CREF { .. } => {
            Expression::expCref(inExp)?
        },
        Deref @ DAE::Exp::BINARY { operator: op, exp1: e1, exp2: e2 } => {
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut cr2: Arc<DAE::ComponentRef>;
            let mut r#str: ArcStr;
            r#str = (ExpressionDump::binopSymbol(op.clone())?).clone();
            cr1 = makeCrefsFromSubScriptExp(e1.clone())?;
            cr2 = makeCrefsFromSubScriptExp(e2.clone())?;
            outCref = prependStringCref((r#str.clone()).clone(), cr1.clone())?;
            outCref = joinCrefs(outCref, cr2.clone())?;
            outCref
        },
        Deref @ DAE::Exp::ENUM_LITERAL { name: enum_lit, .. } => {
            let mut r#str: ArcStr;
            r#str = (System::stringReplace((AbsynUtil::pathString(enum_lit.clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!(".")).clone(), (literal!("$P")).clone())?).clone();
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() })
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = (ExpressionDump::dumpExpStr(inExp, 0)?).clone();
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ComponentReference.makeCrefsFromSubScriptExp for:")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub(crate) fn replaceLast(mut inCref: Arc<DAE::ComponentRef>, mut inNewLast: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst: subs, componentRef: cref } => {
            let mut cref = (*cref).clone();
            cref = replaceLast(cref.clone(), inNewLast)?;
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: cref.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            inNewLast
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn expandArrayCref(mut inCr: Arc<DAE::ComponentRef>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut lasttype: Arc<DAE::Type>;
    let mut tmpcref: Arc<DAE::ComponentRef>;
    lasttype = crefLastType(inCr.clone())?;
    lasttype = Types::liftTypeWithDims(lasttype, inDims)?;
    tmpcref = crefSetLastType(inCr, lasttype)?;
    outCrefs = expandCref(tmpcref, false)?;
    Ok(outCrefs)
}

fn expandArrayCref1(mut inCr: Arc<DAE::ComponentRef>, mut inSubscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>, mut inAccumSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inAccumCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inSubscripts) {
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: sub, tail: subs }, tail: rest_subs } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crefs = expandArrayCref1(inCr.clone(), metamodelica::cons(subs.clone(), rest_subs.clone()), inAccumSubs.clone(), inAccumCrefs)?;
            { (inCr, inSubscripts, inAccumSubs, inAccumCrefs) = (inCr, rest_subs.clone(), metamodelica::cons(sub.clone(), inAccumSubs), crefs.clone()); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
            return Ok(inAccumCrefs)
        },
        _ => {
            let mut cref: Arc<DAE::ComponentRef>;
            cref = crefSetLastSubs(inCr, inAccumSubs)?;
            return Ok(metamodelica::cons(cref.clone(), inAccumCrefs))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn explode(mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outParts: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    outParts = Dangerous::listReverseInPlace(explode_tail(inCref, metamodelica::nil())?);
    Ok(outParts)
}

fn explode_tail(mut inCref: Arc<DAE::ComponentRef>, mut inParts: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: rest_cr, .. } => {
            let mut first_cr: Arc<DAE::ComponentRef>;
            first_cr = ComponentReferenceBasics::crefFirstCref(inCref)?;
            { (inCref, inParts) = (rest_cr.clone(), metamodelica::cons(first_cr.clone(), inParts)); continue '__tco; }
        },
        _ => {
            return Ok(metamodelica::cons(inCref, inParts))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn implode(mut inParts: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = implode_reverse(inParts.reverse())?;
    Ok(outCref)
}

pub fn implode_reverse(mut inParts: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let mut first: Arc<DAE::ComponentRef>;
    let mut rest: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inParts) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    first = __pa0.clone();
    rest = __pa1.clone();
    outCref = implode_tail(rest, first)?;
    Ok(outCref)
}

fn implode_tail(mut inParts: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inAccumCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inParts) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: subs }, tail: rest } => {
            let mut cr: Arc<DAE::ComponentRef>;
            cr = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: inAccumCref });
            { (inParts, inAccumCref) = (rest.clone(), cr.clone()); continue '__tco; }
        },
        Deref @ metamodelica::List::Nil => {
            return Ok(inAccumCref)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn identifierCount(mut inCref: Arc<DAE::ComponentRef>) -> i32 {
    let mut outIdCount: i32;
    outIdCount = identifierCount_tail(inCref, 0);
    outIdCount
}

fn identifierCount_tail(mut inCref: Arc<DAE::ComponentRef>, mut inAccumCount: i32) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
            { (inCref, inAccumCount) = (cr.clone(), inAccumCount + 1); continue '__tco; }
        },
        _ => {
            return inAccumCount + 1
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn checkCrefSubscriptsBounds(mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    checkCrefSubscriptsBounds2(inCref.clone(), inCref, inInfo)?;
    Ok(())
}

fn checkCrefSubscriptsBounds2(mut inCref: Arc<DAE::ComponentRef>, mut inWholeCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inCref) {
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: ty, subscriptLst: subs, componentRef: rest_cr, .. } => {
            checkCrefSubscriptsBounds3(ty.clone(), subs.clone(), inWholeCref.clone(), inInfo.clone())?;
            checkCrefSubscriptsBounds2(rest_cr.clone(), inWholeCref, inInfo)?;
            ()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty, subscriptLst: subs, .. } => {
            checkCrefSubscriptsBounds3(ty.clone(), subs.clone(), inWholeCref, inInfo)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn checkCrefSubscriptsBounds3(mut inCrefType: Arc<DAE::Type>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inWholeCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    dims = TypesDump::getDimensions(inCrefType);
    dims = dims.reverse();
    subs = inSubscripts.reverse();
    checkCrefSubscriptsBounds4(subs, dims, 1, inWholeCref, inInfo)?;
    Ok(())
}

fn checkCrefSubscriptsBounds4(mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inIndex: i32, mut inWholeCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inSubscripts, inDimensions)) {
        (Deref @ metamodelica::List::Cons { head: sub, tail: rest_subs }, Deref @ metamodelica::List::Cons { head: dim, tail: rest_dims }) => {
            let true = (checkCrefSubscriptBounds(sub.clone(), dim.clone(), inIndex, inWholeCref.clone(), inInfo.clone())) else { bail!("pattern mismatch") };
            checkCrefSubscriptsBounds4(rest_subs.clone(), rest_dims.clone(), inIndex + 1, inWholeCref, inInfo)?;
            ()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (_, Deref @ metamodelica::List::Nil) => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn checkCrefSubscriptBounds(mut inSubscript: Arc<DAE::Subscript>, mut inDimension: Arc<DAE::Dimension>, mut inIndex: i32, mut inWholeCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> bool {
    let mut outIsValid: bool;
    outIsValid = 'mc: {
        let __mc_input = (inSubscript, inDimension.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Subscript::INDEX { exp: exp @ Deref @ DAE::Exp::ICONST { integer: idx } }, Deref @ DAE::Dimension::DIM_INTEGER { integer: dim }) => {
                    let false = (idx.clone() > 0 && idx.clone() <= dim.clone()) else { bail!("pattern mismatch") };
                    printSubscriptBoundsError(exp.clone(), inDimension.clone(), inIndex, inWholeCref.clone(), inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { array: expl, .. } }, Deref @ DAE::Dimension::DIM_INTEGER { integer: dim }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = List::getMemberOnTrue(dim.clone(), expl.clone(), (std::sync::Arc::new(fnptr!(subscriptExpOutOfBounds, i32, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    printSubscriptBoundsError(exp.clone(), inDimension.clone(), inIndex, inWholeCref.clone(), inInfo.clone())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outIsValid
}

fn subscriptExpOutOfBounds(mut inDimSize: i32, mut inSubscriptExp: Arc<DAE::Exp>) -> bool {
    let mut outOutOfBounds: bool;
    outOutOfBounds = (::match_deref::match_deref! { match &(inSubscriptExp) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            i.clone() < 1 || i.clone() > inDimSize
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outOutOfBounds
}

fn printSubscriptBoundsError(mut inSubscriptExp: Arc<DAE::Exp>, mut inDimension: Arc<DAE::Dimension>, mut inIndex: i32, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<()> {
    let mut sub_str: ArcStr;
    let mut dim_str: ArcStr;
    let mut idx_str: ArcStr;
    let mut cref_str: ArcStr;
    sub_str = (ExpressionBasics::printExpStr(inSubscriptExp)?).clone();
    dim_str = (ExpressionBasics::dimensionString(inDimension)?).clone();
    idx_str = (intString(inIndex)).clone();
    cref_str = (ComponentReferenceBasics::printComponentRefStr(inCref)?).clone();
    Error::addSourceMessage(Error::ARRAY_INDEX_OUT_OF_BOUNDS.clone(), list![(sub_str).clone(), (idx_str).clone(), (dim_str).clone(), (cref_str).clone()], inInfo)?;
    Ok(())
}

pub(crate) fn crefAppendedSubs(mut cref: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    s1 = stringDelimitList(toStringList(cref.clone()), (literal!("_P")).clone());
    s2 = stringDelimitList(List::mapMap(ComponentReferenceBasics::crefSubs(cref)?, (std::sync::Arc::new(Expression::getSubscriptExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone());
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

pub fn writeCref(mut file: File::File, mut cref: Arc<DAE::ComponentRef>, mut escape: File::Escape) -> Result<()> {
    let mut c: Arc<DAE::ComponentRef> = cref.clone();
    loop {
        c = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            File::writeEscape(file.clone(), (var_field!((*c).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), escape);
            writeSubscripts(file.clone(), var_field!((*c).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), escape)?;
            return Ok(());
            bail!("fail")
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", .. } => {
            File::write(file.clone(), (literal!("der(")).clone());
            writeCref(file.clone(), var_field!((*c).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), escape)?;
            File::write(file.clone(), (literal!(")")).clone());
            return Ok(());
            bail!("fail")
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", .. } => {
            File::write(file.clone(), (literal!("previous(")).clone());
            writeCref(file.clone(), var_field!((*c).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), escape)?;
            File::write(file.clone(), (literal!(")")).clone());
            return Ok(());
            bail!("fail")
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            File::writeEscape(file.clone(), (var_field!((*c).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), escape);
            writeSubscripts(file.clone(), var_field!((*c).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), escape)?;
            File::write(file.clone(), (literal!(".")).clone());
            var_field!((*c).componentRef, DAE::ComponentRef::CREF_QUAL).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    Ok(())
}

pub fn writeSubscripts(mut file: File::File, mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut escape: File::Escape) -> Result<()> {
    let mut first: bool = true;
    let mut i: i32 = 0;
    let mut exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    if subs.clone().is_empty() {
        return Ok(());
    }
    File::write(file.clone(), (literal!("[")).clone());
    for mut s in &*subs {
        let mut s = s.clone();
        if !(first) {
            File::write(file.clone(), (literal!(",")).clone());
        } else {
            first = false;
        }
        let () = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            File::write(file.clone(), (literal!(":")).clone());
            ()
        },
        Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: __esc_i } } => {
            i = (*__esc_i).clone();
            File::writeInt(file.clone(), i.clone(), (literal!("%d")).clone());
            ()
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __esc_i } } => {
            i = (*__esc_i).clone();
            File::writeInt(file.clone(), i.clone(), (literal!("%d")).clone());
            ()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: Deref @ DAE::Exp::ICONST { integer: __esc_i } } => {
            i = (*__esc_i).clone();
            File::writeInt(file.clone(), i.clone(), (literal!("%d")).clone());
            ()
        },
        Deref @ DAE::Subscript::SLICE { exp: __esc_exp } => {
            exp = (*__esc_exp).clone();
            File::write(file.clone(), (ExpressionBasics::printExpStr(exp.clone())?).clone());
            ()
        },
        Deref @ DAE::Subscript::INDEX { exp: __esc_exp } => {
            exp = (*__esc_exp).clone();
            File::write(file.clone(), (ExpressionBasics::printExpStr(exp.clone())?).clone());
            ()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: __esc_exp } => {
            exp = (*__esc_exp).clone();
            File::write(file.clone(), (ExpressionBasics::printExpStr(exp.clone())?).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    File::write(file, (literal!("]")).clone());
    Ok(())
}

pub(crate) fn getConsumedMemory(mut inCref: Arc<DAE::ComponentRef>) -> (metamodelica::Real, metamodelica::Real, metamodelica::Real) {
    let mut szIdents: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut szTypes: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut szSubs: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut cr: Arc<DAE::ComponentRef> = inCref.clone();
    let mut b: bool = true;
    while b {
        (b, cr) = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            szIdents = szIdents + (System::getSizeOfData((var_field!((*cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone())).0;
            szTypes = szTypes + (System::getSizeOfData(var_field!((*cr).identType, DAE::ComponentRef::CREF_IDENT).clone())).0;
            szSubs = szSubs + (System::getSizeOfData(var_field!((*cr).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())).0;
            (false, cr.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            szIdents = szIdents + (System::getSizeOfData((var_field!((*cr).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone())).0;
            szTypes = szTypes + (System::getSizeOfData(var_field!((*cr).identType, DAE::ComponentRef::CREF_QUAL).clone())).0;
            szSubs = szSubs + (System::getSizeOfData(var_field!((*cr).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())).0;
            (true, var_field!((*cr).componentRef, DAE::ComponentRef::CREF_QUAL).clone())
        },
        _ => (false, cr.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (szIdents, szTypes, szSubs)
}

pub fn createDifferentiatedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inX: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    let debug: bool = false;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("inCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    subs = crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = replaceSubsWithString(outCref)?;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after full type: ")); __mm_s.push_str(&*TypesDump::printTypeStr(crefTypeFull(crefStripIterSub(outCref.clone(), (literal!("")).clone()))?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    outCref = crefSetLastType(outCref, DAE::T_UNKNOWN_DEFAULT().clone())?;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after strip: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(expandCref(outCref.clone(), true)?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    outCref = joinCrefs(outCref, ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(DAE::partialDerivativeNamePrefix)); __mm_s.push_str(&*inMatrixName); ArcStr::from(__mm_s) }).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
    outCref = joinCrefs(outCref, inX)?;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after join: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(expandCref(outCref.clone(), true)?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    outCref = crefSetLastSubs(outCref, subs)?;
    outCref = crefSetLastType(outCref, crefLastType(inCref)?)?;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(outCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(outCref)
}

pub fn isTime(mut cref: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(cref) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isWild(mut cref: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(cref) {
        Deref @ DAE::ComponentRef::WILD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn uniqueList(mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut uniqueCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    uniqueCrefs = UnorderedSet::unique_list(crefs, (std::sync::Arc::new(ComponentReferenceBasics::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
    Ok(uniqueCrefs)
}

