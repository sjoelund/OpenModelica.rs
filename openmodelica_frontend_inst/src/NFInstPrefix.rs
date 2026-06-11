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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Prefix {
    EMPTY_PREFIX {
        /// The path of the class the prefix originates from.
        classPath: Option<Arc<Absyn::Path>>,
    },
    PREFIX {
        name: ArcStr,
        dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>,
        restPrefix: Arc<Prefix>,
    },
}
impl metamodelica::gc::MMTrace for Prefix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Prefix::EMPTY_PREFIX { classPath } => {
                metamodelica::gc::MMTrace::mm_accept(classPath, __mmv)?;
                Ok(())
            }
            Prefix::PREFIX { name, dims, restPrefix } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restPrefix, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Prefix {
    fn default() -> Self {
        Self::EMPTY_PREFIX {
            classPath: Default::default(),
        }
    }
}
pub use self::Prefix::{EMPTY_PREFIX,PREFIX};

thread_local! { static __emptyPrefix_TLS: Arc<Prefix> = Arc::new(Prefix::EMPTY_PREFIX { classPath: None }); }
pub fn emptyPrefix() -> Arc<Prefix> { __emptyPrefix_TLS.with(|__t| __t.clone()) }

thread_local! { static __functionPrefix_TLS: Arc<Prefix> = Arc::new(Prefix::EMPTY_PREFIX { classPath: None }); }
pub(crate) fn functionPrefix() -> Arc<Prefix> { __functionPrefix_TLS.with(|__t| __t.clone()) }

pub(crate) fn makePrefix(mut inName: ArcStr, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::PREFIX { name: (inName).clone(), dims: inDims, restPrefix: emptyPrefix().clone() });
    outPrefix
}

pub(crate) fn makeEmptyPrefix(mut inClassPath: Arc<Absyn::Path>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::EMPTY_PREFIX { classPath: Some(inClassPath) });
    outPrefix
}

pub(crate) fn add(mut inName: ArcStr, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::PREFIX { name: (inName).clone(), dims: inDimensions, restPrefix: inPrefix });
    outPrefix
}

pub(crate) fn addPath(mut inPath: Arc<Absyn::Path>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromPath2(inPath, inPrefix)?;
    Ok(outPrefix)
}

pub(crate) fn addOptPath(mut inOptPath: Option<Arc<Absyn::Path>>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &(inOptPath) {
        None => {
            inPrefix
        },
        Some(p) => {
            addPath(p.clone(), inPrefix)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPrefix)
}

pub(crate) fn addString(mut inName: ArcStr, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::PREFIX { name: (inName).clone(), dims: metamodelica::nil(), restPrefix: inPrefix });
    outPrefix
}

pub(crate) fn addStringList(mut inStrings: Arc<metamodelica::List<ArcStr>>, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromStringList2(inStrings, inPrefix);
    outPrefix
}

pub(crate) fn restPrefix(mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outRestPrefix: Arc<Prefix>;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::PREFIX { restPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRestPrefix = __pa0.clone();
    Ok(outRestPrefix)
}

pub(crate) fn firstName(mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            literal!("")
        },
        Deref @ Prefix::PREFIX { name, .. } => {
            name.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

pub(crate) fn prefixCref(mut inCref: Arc<DAE::ComponentRef>, mut inPrefix: Arc<Prefix>) -> Result<Arc<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            return Ok(inCref)
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut cref: Arc<DAE::ComponentRef>;
            cref = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil(), componentRef: inCref });
            { (inCref, inPrefix) = (cref, rest_prefix.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn prefixPath(mut inPath: Arc<Absyn::Path>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            return Ok(inPath)
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut path: Arc<Absyn::Path>;
            path = Arc::new(Absyn::Path::QUALIFIED { name: (name.clone()).clone(), path: inPath });
            { (inPath, inPrefix) = (path, rest_prefix.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn prefixStr(mut inString: ArcStr, mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            inString
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = (toStr(inPrefix)?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*inString); ArcStr::from(__mm_s) }).clone();
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub(crate) fn toCref(mut inPrefix: Arc<Prefix>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::PREFIX { name, restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, .. } => {
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() })
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut cref: Arc<DAE::ComponentRef>;
            cref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            prefixCref(cref, rest_prefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub(crate) fn toPath(mut inPrefix: Arc<Prefix>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::PREFIX { name, restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, .. } => {
            Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut path: Arc<Absyn::Path>;
            path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
            prefixPath(path, rest_prefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub(crate) fn fromPath(mut inPath: Arc<Absyn::Path>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromPath2(inPath, emptyPrefix().clone())?;
    Ok(outPrefix)
}

fn fromPath2(mut inPath: Arc<Absyn::Path>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPath) {
        Deref @ Absyn::Path::QUALIFIED { name, path } => {
            { (inPath, inPrefix) = (path.clone(), Arc::new(Prefix::PREFIX { name: (name.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix })); continue '__tco; }
        },
        Deref @ Absyn::Path::IDENT { name } => {
            return Ok(Arc::new(Prefix::PREFIX { name: (name.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix }))
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
            { (inPath, inPrefix) = (path.clone(), inPrefix); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn fromStringList(mut inStrings: Arc<metamodelica::List<ArcStr>>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromStringList2(inStrings, emptyPrefix().clone());
    outPrefix
}

fn fromStringList2(mut inStrings: Arc<metamodelica::List<ArcStr>>, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inStrings) {
        Deref @ metamodelica::List::Cons { head: r#str, tail: strl } => {
            { (inStrings, inPrefix) = (strl.clone(), Arc::new(Prefix::PREFIX { name: (r#str.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix })); continue '__tco; }
        },
        _ => {
            return inPrefix
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn toStr(mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            literal!("")
        },
        Deref @ Prefix::PREFIX { name, restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, .. } => {
            name.clone()
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toStr(rest_prefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

pub(crate) fn toStrWithEmpty(mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::EMPTY_PREFIX { classPath: None } => {
            literal!("E()")
        },
        Deref @ Prefix::EMPTY_PREFIX { classPath: Some(path) } => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E(")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(path.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toStrWithEmpty(rest_prefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            r#str
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

pub(crate) fn isPackagePrefix(mut inPrefix: Arc<Prefix>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::PREFIX { restPrefix: prefix, .. } => {
            { inPrefix = prefix.clone(); continue '__tco; }
        },
        Deref @ Prefix::EMPTY_PREFIX { classPath: None } => {
            return true
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn toPackagePrefix(mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &(inPrefix) {
        Deref @ Prefix::PREFIX { name, dims, restPrefix: rest_prefix } => {
            let mut rest_prefix = (*rest_prefix).clone();
            rest_prefix = toPackagePrefix(rest_prefix.clone())?;
            Arc::new(Prefix::PREFIX { name: (name.clone()).clone(), dims: dims.clone(), restPrefix: rest_prefix.clone() })
        },
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            Arc::new(Prefix::EMPTY_PREFIX { classPath: None })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPrefix)
}

