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

use crate::HashSet;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn checkModel(mut inDAELst: DAE::DAElist) -> Result<(i32, i32, i32)> {
    let mut varSize: i32;
    let mut eqnSize: i32;
    let mut simpleEqnSize: i32;
    let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut lst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut debug_dump: bool = Flags::isSet(Flags::DUMP_CHECK_MODEL.clone())?;
    let mut arg: CountVarEqnFoldArg;
    ExecStat::execStat((literal!("CheckModel - start counting")).clone())?;
    let DAE::DAE { elementLst: __pa0 } = (inDAELst) else { bail!("pattern mismatch") };
    lst = __pa0.clone();
    hs = HashSet::emptyHashSet();
    (varSize, eqnSize, eqns, hs) = countVarEqnSizeList(lst, (0, 0, metamodelica::nil(), hs), debug_dump)?;
    simpleEqnSize = countSimpleEqnSize(eqns, 0, hs)?;
    ExecStat::execStat((literal!("CheckModel - end counting")).clone())?;
    Ok((varSize, eqnSize, simpleEqnSize))
}

pub type CountVarEqnFoldArg = (i32, i32, Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));

/*varSize*/
/*eqnSize*/
/*eqns*/
/*vars*/
fn countVarEqnSizeList(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut arg: CountVarEqnFoldArg, mut debugDump: bool) -> Result<CountVarEqnFoldArg> {
    let mut arg: CountVarEqnFoldArg = arg;
    for mut e in &*elements {
        let mut e = e.clone();
        arg = countVarEqnSize(e.clone(), arg.clone(), debugDump)?;
    }
    Ok(arg)
}

fn countVarEqnSize(mut element: Arc<DAE::Element>, mut inArg: CountVarEqnFoldArg, mut debugDump: bool) -> Result<CountVarEqnFoldArg> {
    let mut outArg: CountVarEqnFoldArg;
    outArg = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::PARAM { .. }, .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::CONST { .. }, .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { componentRef: cr, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut elem: Arc<DAE::Element>;
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(var_field!((*element).ty, DAE::Element::VAR).clone());
            varSize = varSize + size;
            dumpVar(cr.clone(), size, debugDump)?;
            if DAEUtil::isInput(element.clone()) && DAEUtil::isPublicVar(element.clone()) {
                eqnSize = eqnSize + size;
                dumpEqn(element, size, debugDump);
            } else {
                if isSome(var_field!((*element).binding, DAE::Element::VAR).clone()) {
                    eqnSize = eqnSize + size;
                    elem = Arc::new(DAE::Element::EQUATION { exp: Expression::crefExp(var_field!((*element).componentRef, DAE::Element::VAR).clone())?, scalar: Util::getOption(var_field!((*element).binding, DAE::Element::VAR).clone())?, source: var_field!((*element).source, DAE::Element::VAR).clone() });
                    dumpEqn(elem.clone(), size, debugDump);
                    eqns = metamodelica::cons(elem, eqns);
                }
                hs = BaseHashSet::add(cr.clone(), hs)?;
            }
            (varSize, eqnSize, eqns, hs)
        },
        Deref @ DAE::Element::EQUATION { exp: e, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(Expression::r#typeof(e.clone())?);
            dumpEqn(element.clone(), size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut tp: Arc<DAE::Type>;
            (varSize, eqnSize, eqns, hs) = inArg;
            tp = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
            size = Expression::sizeOf(tp);
            dumpEqn(element.clone(), size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut tp: Arc<DAE::Type>;
            (varSize, eqnSize, eqns, hs) = inArg;
            tp = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
            size = Expression::sizeOf(tp);
            dumpEqn(element.clone(), size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(Expression::r#typeof(e.clone())?);
            dumpEqn(element.clone(), size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(Expression::r#typeof(var_field!((*element).exp, DAE::Element::ARRAY_EQUATION).clone())?);
            dumpEqn(element.clone(), size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: daeElts, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(daeElts.clone(), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            dumpEqn(element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(var_field!((*element).equations, DAE::Element::INITIAL_FOR_EQUATION).clone(), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            size = size * Expression::sizeOf(Expression::r#typeof(var_field!((*element).range, DAE::Element::INITIAL_FOR_EQUATION).clone())?);
            dumpEqn(element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::FOR_EQUATION { .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(var_field!((*element).equations, DAE::Element::FOR_EQUATION).clone(), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            size = size * Expression::sizeOf(Expression::r#typeof(var_field!((*element).range, DAE::Element::FOR_EQUATION).clone())?);
            dumpEqn(element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: Deref @ metamodelica::List::Nil }, equations3: Deref @ metamodelica::List::Nil, .. } => {
            inArg
        },
        Deref @ DAE::Element::IF_EQUATION { equations2: Deref @ metamodelica::List::Cons { head: daeElts, tail: _ }, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(daeElts.clone(), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            dumpEqn(element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: Deref @ metamodelica::List::Nil }, equations3: Deref @ metamodelica::List::Nil, .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            (varSize, eqnSize, eqns, hs) = inArg;
            crlst = checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            size = (crlst.len() as i32);
            dumpEqn(element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            inArg
        },
        Deref @ DAE::Element::COMP { dAElist: daeElts, .. } => {
            countVarEqnSizeList(daeElts.clone(), inArg, debugDump)?
        },
        Deref @ DAE::Element::REINIT { .. } => {
            inArg
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            inArg
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            inArg
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            inArg
        },
        Deref @ DAE::Element::CONSTRAINT { .. } => {
            inArg
        },
        Deref @ DAE::Element::FLAT_SM { dAElist: daeElts, .. } => {
            countVarEqnSizeList(daeElts.clone(), inArg, debugDump)?
        },
        Deref @ DAE::Element::SM_COMP { dAElist: daeElts, .. } => {
            countVarEqnSizeList(daeElts.clone(), inArg, debugDump)?
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CheckModel.countVarEqnSize failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![element])?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

fn dumpVar(mut cref: Arc<DAE::ComponentRef>, mut size: i32, mut dump: bool) -> Result<()> {
    if dump {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[var: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", size))); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn dumpEqn(mut eqn: Arc<DAE::Element>, mut size: i32, mut dump: bool) -> () {
    if dump {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[eqn: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", size))); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*DAEDump::dumpEquationStr(eqn)); ArcStr::from(__mm_s) }).clone());
    }
    ()
}

pub fn checkAndGetAlgorithmOutputs(mut inAlgorithm: Arc<DAE::Algorithm>, mut inSource: Arc<DAE::ElementSource>, mut inCrefExpansionRule: DAE::Expand) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    outCrefLst = 'mc: {
        let __mc_input = inSource.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ElementSource { instance: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. } => {
                    Ok(algorithmOutputs(inAlgorithm.clone(), inCrefExpansionRule.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ElementSource { .. } => {
                    Ok(if (PrefixUtil::hasSubs(inSource.instance.clone())) {algorithmOutputs(inAlgorithm.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND)?} else {algorithmOutputs(inAlgorithm.clone(), inCrefExpansionRule.clone())?})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("checkAndGetAlgorithmOutputs failed.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefLst)
}

pub fn isCrefListAlgorithmOutput(mut crefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inAlgorithm: Arc<DAE::Algorithm>, mut inSource: Arc<DAE::ElementSource>, mut inCrefExpansionRule: DAE::Expand) -> Result<bool> {
    let mut outResult: bool = false;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    let mut algOutCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    algOutCrefs = checkAndGetAlgorithmOutputs(inAlgorithm, inSource, inCrefExpansionRule)?;
    ht = List::fold(algOutCrefs, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht)?;
    for mut cr in &*crefList {
        let mut cr = cr.clone();
        if !(BaseHashSet::has(cr.clone(), ht.clone())?) {
            return Ok(outResult.clone());
        }
    }
    outResult = true;
    Ok(outResult)
}

fn algorithmOutputs(mut inAlgorithm: Arc<DAE::Algorithm>, mut inCrefExpansion: DAE::Expand) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm) {
        Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    outCrefLst = algorithmStatementListOutputs(stmts, inCrefExpansion)?;
    Ok(outCrefLst)
}

pub fn algorithmStatementListOutputs(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCrefExpansion: DAE::Expand) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    hs = HashSet::emptyHashSet();
    hs = List::fold1(inStmts, (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion, hs)?;
    outCrefLst = BaseHashSet::hashSetList(hs)?;
    Ok(outCrefLst)
}

fn statementOutputs(mut inStatement: Arc<DAE::Statement>, mut inCrefExpansion: DAE::Expand, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    let mut oht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    oht = 'mc: {
        let __mc_input = inStatement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { exp1, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let (_, (_, __pa0)) = Expression::traverseExpTopDown(exp1.clone(), (std::sync::Arc::new(fnptr!(statementOutputsCrefFinder, Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), (inCrefExpansion.clone(), iht.clone()))?;
                    ht = __pa0.clone();
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expl, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let (_, (_, __pa0)) = Expression::traverseExpListTopDown(expl.clone(), (std::sync::Arc::new(fnptr!(statementOutputsCrefFinder, Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), (inCrefExpansion.clone(), iht.clone()))?;
                    ht = __pa0.clone();
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp1, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    cr = Expression::expCref(exp1.clone())?;
                    subs = ComponentReference::crefLastSubs(cr.clone())?;
                    if !(subs.clone().is_empty()) {
                        subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), (subs.clone().len() as i32));
                        cr = ComponentReference::crefSetLastSubs(cr.clone(), subs.clone())?;
                    }
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    ht = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), iht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_IF { statementLst: stmts, else_: elsebranch, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion.clone(), iht.clone())?;
                    ht = statementElseOutputs(elsebranch.clone(), inCrefExpansion.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FOR { type_: tp, iter: iteratorName, range: e, statementLst: stmts, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut stmts = (*stmts).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent((iteratorName.clone()).clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> + 'static>), (cr.clone(), e.clone())))?;
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), openmodelica_frontend_types::DAE::Expand::EXPAND, iht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_PARFOR { type_: tp, iter: iteratorName, range: e, statementLst: stmts, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut stmts = (*stmts).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent((iteratorName.clone()).clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> + 'static>), (cr.clone(), e.clone())))?;
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), openmodelica_frontend_types::DAE::Expand::EXPAND, iht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion.clone(), iht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: None, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion.clone(), iht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: Some(stmt), .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion.clone(), iht.clone())?;
                    ht = statementOutputs(stmt.clone(), inCrefExpansion.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSERT { .. } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TERMINATE { .. } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_REINIT { .. } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_NORETCALL { .. } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_RETURN { source: _ } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_BREAK { source: _ } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_CONTINUE { source: _ } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ARRAY_INIT { .. } => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion.clone(), iht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (DAEDump::ppStatementStr(inStatement.clone())).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CheckModel.statementOutputs failed for ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oht)
}

fn statementElseOutputs(mut inElseBranch: Arc<DAE::Else>, mut inCrefExpansion: DAE::Expand, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElseBranch) {
        Deref @ DAE::Else::NOELSE { .. } => {
            return Ok(iht)
        },
        Deref @ DAE::Else::ELSEIF { statementLst: stmts, else_: elseBranch, .. } => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            ht = List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion.clone(), iht)?;
            { (inElseBranch, inCrefExpansion, iht) = (elseBranch.clone(), inCrefExpansion, ht); continue '__tco; }
        },
        Deref @ DAE::Else::ELSE { statementLst: stmts } => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            return Ok(List::fold1(stmts.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), inCrefExpansion, iht)?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn statementOutputsCrefFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> (Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp, inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", subscriptLst: Deref @ metamodelica::List::Nil, .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } }, (expand, ht)) => {
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut cr = (*cr).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefStripSubs(cr.clone())?;
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    ht = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht.clone())?;
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (expand @ DAE::Expand::NOT_EXPAND { .. }, ht)) => {
                    let mut ht = (*ht).clone();
                    ht = List::fold(list![cr.clone()], (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht.clone())?;
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (expand, ht)) => {
                    let mut first_cref: Arc<DAE::ComponentRef>;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut cr = (*cr).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefStripSubsExceptModelSubs(cr.clone());
                    first_cref = ComponentReference::crefArrayGetFirstCref(cr.clone())?;
                    if !(BaseHashSet::has(first_cref.clone(), ht.clone())?) {
                        crlst = ComponentReference::expandCref(cr.clone(), true)?;
                        ht = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht.clone())?;
                    }
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ASUB { exp, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(fnptr!(statementOutputsCrefFinder, Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), inTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TSUB { exp, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(fnptr!(statementOutputsCrefFinder, Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), inTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expThen: e1, expElse: e2, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(fnptr!(statementOutputsCrefFinder, Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), inTpl.clone())?;
                    (_, outTpl) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(fnptr!(statementOutputsCrefFinder, Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), outTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    Ok((e.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

fn countSimpleEqnSize(mut inEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut isimpleEqnSize: i32, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<i32> {
    let mut osimpleEqnSize: i32;
    osimpleEqnSize = List::applyAndFold1(inEqns, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(countSimpleEqnSizeWork, Arc<DAE::Element>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<i32> + 'static>), ihs, 0)?;
    Ok(osimpleEqnSize)
}

fn countSimpleEqnSizeWork(mut inEqns: Arc<DAE::Element>, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> i32 {
    let mut osimpleEqnSize: i32;
    osimpleEqnSize = 'mc: {
        let __mc_input = inEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, .. } => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. } => {
                    let mut tp: Arc<DAE::Type>;
                    tp = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
                    Ok(Expression::sizeOf(tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e2, .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, .. } => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    osimpleEqnSize
}

fn simpleEquation(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> i32 {
    let mut osimpleEqnSize: i32;
    osimpleEqnSize = 'mc: {
        let __mc_input = (e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { .. }) => {
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut ea1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut ea2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let true = (Expression::isArray(e1.clone()) || Expression::isMatrix(e1.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isArray(e2.clone()) || Expression::isMatrix(e2.clone())) else { bail!("pattern mismatch") };
                    ea1 = Expression::flattenArrayExpToList(e1.clone());
                    ea2 = Expression::flattenArrayExpToList(e2.clone());
                    Ok(simpleEquations(ea1.clone(), ea2.clone(), 0, ihs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(Expression::expSub(e1.clone(), e2.clone())?, (std::sync::Arc::new(fnptr!(traversingComponentRefFinder, Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), (ihs.clone(), metamodelica::nil()))?) {
                        (_, (_, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil })) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    osimpleEqnSize
}

fn traversingComponentRefFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> (Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (hs, crefs)) => {
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut crefs = (*crefs).clone();
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    crefs = getcr(crlst.clone(), hs.clone(), crefs.clone())?;
                    Ok((e.clone(), (hs.clone(), crefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn getcr(mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut iAcc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(crefs) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iAcc)
        },
        Deref @ metamodelica::List::Cons { head: cr, tail: rest } if (BaseHashSet::has(cr.clone(), hs.clone())?) => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crlst = List::unionEltOnTrue(cr.clone(), iAcc, (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            { (crefs, hs, iAcc) = (rest.clone(), hs.clone(), crlst); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            { (crefs, hs, iAcc) = (rest.clone(), hs.clone(), iAcc); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simpleEquations(mut e1lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut e2lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut isimpleEqnSize: i32, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((e1lst, e2lst)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(isimpleEqnSize)
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: r1 }, Deref @ metamodelica::List::Cons { head: e2, tail: r2 }) => {
            let mut size: i32;
            size = simpleEquation(e1.clone(), e2.clone(), ihs.clone());
            { (e1lst, e2lst, isimpleEqnSize, ihs) = (r1.clone(), r2.clone(), size + isimpleEqnSize, ihs); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

