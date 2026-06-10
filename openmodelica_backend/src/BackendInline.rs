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

use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::InlineArrayEquations;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// late inline functions stuff
//
// =============================================================================
pub fn lateInlineFunction(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = inlineCalls(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE], inDAE.clone())?;
    Ok(outDAE)
}

// =============================================================================
// normal inline functions stuff
//
// =============================================================================
pub fn normalInlineFunction(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    if Flags::getConfigEnum(Flags::INLINE_METHOD.clone())? == 1 {
        outDAE = inlineCalls(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE], inDAE.clone())?;
    } else {
        outDAE = inlineCallsBDAE(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE], inDAE.clone())?;
    }
    Ok(outDAE)
}

// =============================================================================
// inline calls stuff
//
// =============================================================================
fn inlineCalls(mut inITLst: Arc<metamodelica::List<DAE::InlineType>>, mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut tpl: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>);
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    match '__try0: {
        shared = inBackendDAE.shared.clone();
        eqs = inBackendDAE.eqs.clone();
        tpl = (Some(shared.functionTree.clone()), inITLst.clone());
        eqs = unwrap_break_err!(List::map1(eqs.clone(), (std::sync::Arc::new(inlineEquationSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), tpl.clone()), '__try0);
        assign_field!(
            shared.globalKnownVars = unwrap_break_err!(inlineVariables(shared.globalKnownVars.clone(), tpl.clone()), '__try0).0,
            shared.externalObjects = unwrap_break_err!(inlineVariables(shared.externalObjects.clone(), tpl.clone()), '__try0).0,
            shared.initialEqs = unwrap_break_err!(inlineEquationArray(shared.initialEqs.clone(), tpl.clone()), '__try0).0,
            shared.removedEqs = unwrap_break_err!(inlineEquationArray(shared.removedEqs.clone(), tpl.clone()), '__try0).0
        );
        unwrap_break_err!(inlineEventInfo(shared.eventInfo.clone(), tpl.clone()), '__try0);
        outBackendDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() });
        Ok::<_, anyhow::Error>((eqs.clone(), outBackendDAE.clone(), shared.clone(), tpl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            eqs = __try0_o0;
            outBackendDAE = __try0_o1;
            shared = __try0_o2;
            tpl = __try0_o3;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln((literal!("BackendInline.inlineCalls failed")).clone())?;
            }
            return Err(__try0_err);
        }
    }
    Ok(outBackendDAE)
}

fn inlineEquationSystem(mut eqs: Arc<BackendDAE::EqSystem>, mut tpl: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut oeqs: Arc<BackendDAE::EqSystem> = eqs.clone();
    inlineVariables(oeqs.orderedVars.clone(), tpl.clone())?;
    inlineEquationArray(oeqs.orderedEqs.clone(), tpl.clone())?;
    inlineEquationArray(oeqs.removedEqs.clone(), tpl.clone())?;
    Ok(oeqs)
}

fn inlineEquationArray(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inElementList: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEquationArray.clone();
    let mut oInlined: bool;
    if let Ok(__iflet0) = inlineEquationOptArray(outEquationArray.clone(), inElementList.clone()) {
        oInlined = __iflet0;
    } else {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::trace((literal!("Inline.inlineEquationArray failed\n")).clone())?;
        }
        bail!("fail");
    }
    Ok((outEquationArray, oInlined))
}

fn inlineEquationOptArray(mut inEqnArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<bool> {
    let mut oInlined: bool = false;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut inlined: bool;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(inEqnArray.clone()) {
        if ExpandableArray::occupied(i.clone(), inEqnArray.clone()) {
            (eqn, inlined) = inlineEq(ExpandableArray::get(i.clone(), inEqnArray.clone())?, fns.clone());
            if inlined.clone() {
                ExpandableArray::update(i.clone(), eqn.clone(), inEqnArray.clone())?;
                oInlined = true;
            }
        }
    }
    Ok(oInlined)
}

pub fn inlineEq(mut inEquation: Arc<BackendDAE::Equation>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> (Arc<BackendDAE::Equation>, bool) {
    let mut outEquation: Arc<BackendDAE::Equation>;
    let mut inlined: bool;
    (outEquation, inlined) = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr, recordSize } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    eqn = (::match_deref::match_deref! { match &((e1_1.clone(), e2_1.clone())) {
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, .. }) => Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr.clone() }),
        _ => Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: attr.clone(), recordSize: recordSize.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok((eqn.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::FOR_EQUATION { iter: e, start: e1, stop: e2, body: eqn, source, attr } => {
                    let mut eqn = (*eqn).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(inlineEq(eqn.clone(), fns.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqn = __pa0.clone();
                    Ok((Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: e.clone(), start: e1.clone(), stop: e2.clone(), body: eqn.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e, source, attr } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Inline::inlineExp(e.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    source = __pa1.clone();
                    Ok((Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref.clone(), exp: e_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, attr } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Inline::inlineExp(e.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    source = __pa1.clone();
                    Ok((Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, expand: crefExpand, attr } => {
                    let mut alg: Arc<DAE::Algorithm>;
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Inline::inlineStatements(stmts.clone(), fns.clone(), metamodelica::nil(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    stmts1 = __pa0.clone();
                    alg = Arc::new(DAE::Algorithm { statementLst: stmts1.clone() });
                    Ok((Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: weq, source, attr } => {
                    let mut weq_1: Arc<BackendDAE::WhenEquation>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineWhenEq(weq.clone(), fns.clone(), source.clone())?) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    weq_1 = __pa0.clone();
                    source = __pa1.clone();
                    Ok((Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: weq_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { conditions: explst, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut explst = (*explst).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut source = (*source).clone();
                    (explst, source, b1) = Inline::inlineExps(explst.clone(), fns.clone(), source.clone());
                    (eqnslst, b2) = inlineEqsLst(eqnslst.clone(), fns.clone(), metamodelica::nil(), false);
                    (eqns, b3) = inlineEqs(eqns.clone(), fns.clone(), metamodelica::nil(), false);
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEquation.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEquation, inlined)
}

fn inlineEqsLst(mut inEqnsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inFunctions: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut iAcc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut iInlined: bool) -> (Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inEqnsList.clone()) {
        Deref @ metamodelica::List::Nil => {
            return (iAcc.clone().reverse(), iInlined.clone())
        },
        Deref @ metamodelica::List::Cons { head: eqn, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
            let mut inlined: bool;
            let mut eqn = (*eqn).clone();
            (eqn, inlined) = inlineEqs(eqn.clone(), inFunctions.clone(), metamodelica::nil(), false);
            { (inEqnsList, inFunctions, iAcc, iInlined) = (rest.clone(), inFunctions.clone(), metamodelica::cons(eqn.clone(), iAcc.clone()), inlined.clone() || iInlined.clone()); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn inlineEqs(mut inEqnsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFunctions: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iInlined: bool) -> (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inEqnsList.clone()) {
        Deref @ metamodelica::List::Nil => {
            return (iAcc.clone().reverse(), iInlined.clone())
        },
        Deref @ metamodelica::List::Cons { head: eqn, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut inlined: bool;
            let mut eqn = (*eqn).clone();
            (eqn, inlined) = inlineEq(eqn.clone(), inFunctions.clone());
            { (inEqnsList, inFunctions, iAcc, iInlined) = (rest.clone(), inFunctions.clone(), metamodelica::cons(eqn.clone(), iAcc.clone()), inlined.clone() || iInlined.clone()); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn inlineWhenEq(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut inSource: Arc<DAE::ElementSource>) -> Result<(Arc<BackendDAE::WhenEquation>, Arc<DAE::ElementSource>, bool)> {
    let mut outWhenEquation: Arc<BackendDAE::WhenEquation>;
    let mut outSource: Arc<DAE::ElementSource>;
    let mut inlined: bool;
    (outWhenEquation, outSource, inlined) = (::match_deref::match_deref! { match &(inWhenEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: oelsewe } => {
            let mut source: Arc<DAE::ElementSource>;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut elsewe: Arc<BackendDAE::WhenEquation>;
            let mut cond = (*cond).clone();
            let mut whenStmtLst = (*whenStmtLst).clone();
            let mut oelsewe = (*oelsewe).clone();
            (cond, source, b1, _) = Inline::inlineExp(cond.clone(), fns.clone(), inSource.clone());
            (whenStmtLst, b2) = inlineWhenOps(whenStmtLst.clone(), fns.clone())?;
            if isSome(oelsewe.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                elsewe = __pa0.clone();
                (elsewe, source, b3) = inlineWhenEq(elsewe.clone(), fns.clone(), source.clone())?;
                oelsewe = Some(elsewe.clone());
            } else {
                oelsewe = None;
                b3 = false;
            }
            (Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oelsewe.clone() }), source.clone(), b1.clone() || b2.clone() || b3.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outWhenEquation, outSource, inlined))
}

fn inlineWhenOps(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<(Arc<metamodelica::List<BackendDAE::WhenOperator>>, bool)> {
    let mut outWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut inlined: bool = false;
    for mut whenOp in &*inWhenOps.clone() {
        let mut whenOp = whenOp.clone();
        let () = (match whenOp.clone() {
        BackendDAE::WhenOperator::ASSIGN { left: ref e1, right: ref e2, source: mut source } => {
            let mut b: bool;
            let mut e2 = e2.clone();
            let mut source = source.clone();
            (e2, source, b, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
            outWhenOps = metamodelica::cons(if (b.clone()) {BackendDAE::WhenOperator::ASSIGN { left: e1.clone(), right: e2.clone(), source: source.clone() }} else {whenOp.clone()}, outWhenOps.clone());
            inlined = inlined.clone() || b.clone();
            ()
        },
        BackendDAE::WhenOperator::REINIT { stateVar: ref cr, value: ref e2, source: mut source } => {
            let mut b: bool;
            let mut e2 = e2.clone();
            let mut source = source.clone();
            (e2, source, b, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
            outWhenOps = metamodelica::cons(if (b.clone()) {BackendDAE::WhenOperator::REINIT { stateVar: cr.clone(), value: e2.clone(), source: source.clone() }} else {whenOp.clone()}, outWhenOps.clone());
            inlined = inlined.clone() || b.clone();
            ()
        },
        BackendDAE::WhenOperator::ASSERT { condition: ref e1, message: ref e2, level: mut level, source: mut source } => {
            let mut b: bool;
            let mut b2: bool;
            let mut e1 = e1.clone();
            let mut e2 = e2.clone();
            let mut source = source.clone();
            (e1, source, b, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
            (e2, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
            outWhenOps = metamodelica::cons(if (b.clone() || b2.clone()) {BackendDAE::WhenOperator::ASSERT { condition: e1.clone(), message: e2.clone(), level: level.clone(), source: source.clone() }} else {whenOp.clone()}, outWhenOps.clone());
            inlined = inlined.clone() || b.clone() || b2.clone();
            ()
        },
        BackendDAE::WhenOperator::TERMINATE { message: ref e1, source: mut source } => {
            let mut b: bool;
            let mut e1 = e1.clone();
            let mut source = source.clone();
            (e1, source, b, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
            outWhenOps = metamodelica::cons(if (b.clone()) {BackendDAE::WhenOperator::TERMINATE { message: e1.clone(), source: source.clone() }} else {whenOp.clone()}, outWhenOps.clone());
            inlined = inlined.clone() || b.clone();
            ()
        },
        BackendDAE::WhenOperator::NORETCALL { exp: ref e1, source: mut source } => {
            let mut b: bool;
            let mut e1 = e1.clone();
            let mut source = source.clone();
            (e1, source, b, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
            outWhenOps = metamodelica::cons(if (b.clone()) {BackendDAE::WhenOperator::NORETCALL { exp: e1.clone(), source: source.clone() }} else {whenOp.clone()}, outWhenOps.clone());
            inlined = inlined.clone() || b.clone();
            ()
        },
    });
    }
    Ok((outWhenOps, inlined))
}

fn inlineVariables(mut inVariables: BackendDAE::Variables, mut inElementList: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<(BackendDAE::Variables, bool)> {
    let mut outVariables: BackendDAE::Variables;
    let mut inlined: bool = false;
    (outVariables, inlined) = 'mc: {
        let __mc_input = (inVariables.clone(), inElementList.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Variables { crefIndices: crefind, varArr: BackendDAE::VariableArray { numberOfElements: i3, varOptArr: vararr }, bucketSize: i1, numberOfVars: i2 }, fns) => {
                    let mut inlined: bool = inlined.clone();
                    inlined = inlineVarOptArray(vararr.clone(), fns.clone())?;
                    Ok(((BackendDAE::Variables { crefIndices: crefind.clone(), varArr: BackendDAE::VariableArray { numberOfElements: i3.clone(), varOptArr: vararr.clone() }, bucketSize: i1.clone(), numberOfVars: i2.clone() }, inlined.clone()), inlined.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { inlined = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Inline.inlineVariables failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVariables, inlined))
}

fn inlineVarOptArray(mut inVarArray: metamodelica::Array<Option<BackendDAE::Var>>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<bool> {
    let mut oInlined: bool = false;
    let mut b: bool;
    let mut var: Option<BackendDAE::Var>;
    for mut index in 1..=metamodelica::arrayLength(inVarArray.clone()) {
        var = ({let __elt = inVarArray.borrow()[(index.clone()-1) as usize].clone(); __elt});
        (var, b) = inlineVarOpt(var.clone(), fns.clone());
        if b.clone() {
            metamodelica::arrayUpdate(inVarArray.clone(), index.clone(), var.clone())?;
        }
        oInlined = oInlined.clone() || b.clone();
    }
    Ok(oInlined)
}

fn inlineVarOpt(mut inVarOption: Option<BackendDAE::Var>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> (Option<BackendDAE::Var>, bool) {
    let mut outVarOption: Option<BackendDAE::Var>;
    let mut inlined: bool;
    (outVarOption, inlined) = (match inVarOption.clone() {
        None => {
            (None, false)
        },
        Some(mut var) => {
            let mut var2: BackendDAE::Var;
            let mut b: bool;
            (var2, b) = inlineVar(var.clone(), fns.clone());
            (if ({ let __refeq_sl = &(var.clone()); let __refeq_sr = &(var2.clone()); referenceEq(&*(__refeq_sl.varName),&*(__refeq_sr.varName)) && (match (&(__refeq_sl.varKind), &(__refeq_sr.varKind)) { (BackendDAE::VarKind::ALG_STATE, BackendDAE::VarKind::ALG_STATE) => true, (BackendDAE::VarKind::ALG_STATE_OLD, BackendDAE::VarKind::ALG_STATE_OLD) => true, (BackendDAE::VarKind::CLOCKED_STATE { previousName: __refeq_v0l, isStartFixed: __refeq_v1l }, BackendDAE::VarKind::CLOCKED_STATE { previousName: __refeq_v0r, isStartFixed: __refeq_v1r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && ((*__refeq_v1l) == (*__refeq_v1r)), (BackendDAE::VarKind::CONST, BackendDAE::VarKind::CONST) => true, (BackendDAE::VarKind::DAE_AUX_VAR, BackendDAE::VarKind::DAE_AUX_VAR) => true, (BackendDAE::VarKind::DAE_RESIDUAL_VAR, BackendDAE::VarKind::DAE_RESIDUAL_VAR) => true, (BackendDAE::VarKind::DISCRETE, BackendDAE::VarKind::DISCRETE) => true, (BackendDAE::VarKind::DUMMY_DER, BackendDAE::VarKind::DUMMY_DER) => true, (BackendDAE::VarKind::DUMMY_STATE, BackendDAE::VarKind::DUMMY_STATE) => true, (BackendDAE::VarKind::EXTOBJ { fullClassName: __refeq_v0l }, BackendDAE::VarKind::EXTOBJ { fullClassName: __refeq_v0r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)), (BackendDAE::VarKind::JAC_TMP_VAR, BackendDAE::VarKind::JAC_TMP_VAR) => true, (BackendDAE::VarKind::JAC_VAR, BackendDAE::VarKind::JAC_VAR) => true, (BackendDAE::VarKind::LOOP_ITERATION, BackendDAE::VarKind::LOOP_ITERATION) => true, (BackendDAE::VarKind::LOOP_SOLVED, BackendDAE::VarKind::LOOP_SOLVED) => true, (BackendDAE::VarKind::OPT_CONSTR, BackendDAE::VarKind::OPT_CONSTR) => true, (BackendDAE::VarKind::OPT_FCONSTR, BackendDAE::VarKind::OPT_FCONSTR) => true, (BackendDAE::VarKind::OPT_INPUT_DER, BackendDAE::VarKind::OPT_INPUT_DER) => true, (BackendDAE::VarKind::OPT_INPUT_WITH_DER, BackendDAE::VarKind::OPT_INPUT_WITH_DER) => true, (BackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: __refeq_v0l }, BackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: __refeq_v0r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)), (BackendDAE::VarKind::OPT_TGRID, BackendDAE::VarKind::OPT_TGRID) => true, (BackendDAE::VarKind::PARAM, BackendDAE::VarKind::PARAM) => true, (BackendDAE::VarKind::SEED_VAR, BackendDAE::VarKind::SEED_VAR) => true, (BackendDAE::VarKind::STATE { index: __refeq_v0l, derName: __refeq_v1l, natural: __refeq_v2l }, BackendDAE::VarKind::STATE { index: __refeq_v0r, derName: __refeq_v1r, natural: __refeq_v2r }) => ((*__refeq_v0l) == (*__refeq_v0r)) && (match (&(*__refeq_v1l), &(*__refeq_v1r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && ((*__refeq_v2l) == (*__refeq_v2r)), (BackendDAE::VarKind::STATE_DER, BackendDAE::VarKind::STATE_DER) => true, (BackendDAE::VarKind::VARIABLE, BackendDAE::VarKind::VARIABLE) => true, _ => false }) && (match (&(__refeq_sl.varDirection), &(__refeq_sr.varDirection)) { (DAE::VarDirection::BIDIR, DAE::VarDirection::BIDIR) => true, (DAE::VarDirection::INPUT, DAE::VarDirection::INPUT) => true, (DAE::VarDirection::OUTPUT, DAE::VarDirection::OUTPUT) => true, _ => false }) && (match (&(__refeq_sl.varParallelism), &(__refeq_sr.varParallelism)) { (DAE::VarParallelism::NON_PARALLEL, DAE::VarParallelism::NON_PARALLEL) => true, (DAE::VarParallelism::PARGLOBAL, DAE::VarParallelism::PARGLOBAL) => true, (DAE::VarParallelism::PARLOCAL, DAE::VarParallelism::PARLOCAL) => true, _ => false }) && referenceEq(&*(__refeq_sl.varType),&*(__refeq_sr.varType)) && (match (&(__refeq_sl.bindExp), &(__refeq_sr.bindExp)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(__refeq_sl.tplExp), &(__refeq_sr.tplExp)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(__refeq_sl.arryDim), &*(__refeq_sr.arryDim)) && referenceEq(&*(__refeq_sl.source),&*(__refeq_sr.source)) && (match (&(__refeq_sl.values), &(__refeq_sr.values)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(__refeq_sl.tearingSelectOption), &(__refeq_sr.tearingSelectOption)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (BackendDAE::TearingSelect::ALWAYS, BackendDAE::TearingSelect::ALWAYS) => true, (BackendDAE::TearingSelect::AVOID, BackendDAE::TearingSelect::AVOID) => true, (BackendDAE::TearingSelect::DEFAULT, BackendDAE::TearingSelect::DEFAULT) => true, (BackendDAE::TearingSelect::NEVER, BackendDAE::TearingSelect::NEVER) => true, (BackendDAE::TearingSelect::PREFER, BackendDAE::TearingSelect::PREFER) => true, _ => false }), _ => false }) && (match (&(__refeq_sl.hideResult), &(__refeq_sr.hideResult)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(__refeq_sl.comment), &(__refeq_sr.comment)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && referenceEq(&*(__refeq_sl.connectorType),&*(__refeq_sr.connectorType)) && (match (&(__refeq_sl.innerOuter), &(__refeq_sr.innerOuter)) { (DAE::VarInnerOuter::INNER, DAE::VarInnerOuter::INNER) => true, (DAE::VarInnerOuter::INNER_OUTER, DAE::VarInnerOuter::INNER_OUTER) => true, (DAE::VarInnerOuter::NOT_INNER_OUTER, DAE::VarInnerOuter::NOT_INNER_OUTER) => true, (DAE::VarInnerOuter::OUTER, DAE::VarInnerOuter::OUTER) => true, _ => false }) && ((__refeq_sl.unreplaceable) == (__refeq_sr.unreplaceable)) && ((__refeq_sl.initNonlinear) == (__refeq_sr.initNonlinear)) && ((__refeq_sl.encrypted) == (__refeq_sr.encrypted)) }) {inVarOption.clone()} else {Some(var2.clone())}, b.clone())
        },
    });
    (outVarOption, inlined)
}

fn inlineVar(mut inVar: BackendDAE::Var, mut inElementList: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> (BackendDAE::Var, bool) {
    let mut outVar: BackendDAE::Var;
    let mut inlined: bool;
    (outVar, inlined) = (match inVar.clone() {
        BackendDAE::Var { varName: mut varName, varKind: mut varKind, varDirection: mut varDirection, varParallelism: mut varParallelism, varType: mut varType, bindExp: mut bind, tplExp: mut tplExp, arryDim: ref arrayDim, source: mut source, values: mut values, tearingSelectOption: mut ts, hideResult: mut hideResult, comment: mut comment, connectorType: ref ct, innerOuter: mut io, unreplaceable: mut unreplaceable, initNonlinear: _, encrypted: mut e } => {
            let mut values1: Option<Arc<DAE::VariableAttributes>>;
            let mut b1: bool;
            let mut b2: bool;
            let mut source = source.clone();
            (bind, source, b1) = Inline::inlineExpOpt(bind.clone(), inElementList.clone(), source.clone());
            (values1, source, b2) = Inline::inlineStartAttribute(values.clone(), source.clone(), inElementList.clone());
            (BackendDAE::Var { varName: varName.clone(), varKind: varKind.clone(), varDirection: varDirection.clone(), varParallelism: varParallelism.clone(), varType: varType.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: arrayDim.clone(), source: source.clone(), values: values1.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: unreplaceable.clone(), initNonlinear: false, encrypted: e.clone() }, b1.clone() || b2.clone())
        },
        _ => {
            (inVar.clone(), false)
        },
    });
    (outVar, inlined)
}

fn inlineEventInfo(mut inEventInfo: BackendDAE::EventInfo, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inEventInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::EventInfo { zeroCrossings: mut zclst, relations: mut relations, .. } = __mc_input.clone() else { bail!("nomatch") };
            inlineZeroCrossings(zclst.zc.clone(), fns.clone())?;
            inlineZeroCrossings(relations.clone(), fns.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("Inline.inlineEventInfo failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn inlineZeroCrossings(mut inStmts: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<()> {
    DoubleEnded::mapNoCopy_1(inStmts.clone(), (std::sync::Arc::new(fnptr!(inlineZeroCrossing, BackendDAE::ZeroCrossing, (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>))) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<BackendDAE::ZeroCrossing> + 'static>), fns.clone())?;
    Ok(())
}

fn inlineZeroCrossing(mut zc: BackendDAE::ZeroCrossing, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> BackendDAE::ZeroCrossing {
    let mut zc: BackendDAE::ZeroCrossing = zc;
    zc = (match zc.clone() {
        BackendDAE::ZeroCrossing { relation_: ref e, .. } => {
            let mut e_1: Arc<DAE::Exp>;
            (e_1, _, _, _) = Inline::inlineExp(e.clone(), fns.clone(), DAE::emptyElementSource().clone());
            if (!(referenceEq(&*(e.clone()),&*(e_1.clone())))) {BackendDAE::ZeroCrossing { index: zc.index.clone(), relation_: e_1.clone(), occurEquLst: zc.occurEquLst.clone(), iter: zc.iter.clone() }} else {zc.clone()}
        },
        _ => {
            zc.clone()
        },
    });
    zc
}

// =============================================================================
// inline append functions
//
// =============================================================================
fn inlineCallsBDAE(mut inITLst: Arc<metamodelica::List<DAE::InlineType>>, mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut tpl: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>);
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DUMPBACKENDINLINE.clone()), '__try0) {
            if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 1 {
                metamodelica::print((literal!("\n############ BackendInline Method: replace ############")).clone());
            } else if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 2 {
                metamodelica::print((literal!("\n############ BackendInline Method: append ############")).clone());
            }
        }
        shared = inBackendDAE.shared.clone();
        eqs = inBackendDAE.eqs.clone();
        tpl = (Some(shared.functionTree.clone()), inITLst.clone());
        if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 1 {
            eqs = unwrap_break_err!(List::map1(eqs.clone(), (std::sync::Arc::new(inlineEquationSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>)) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), tpl.clone()), '__try0);
        } else if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 2 {
            eqs = unwrap_break_err!(List::map2(eqs.clone(), (std::sync::Arc::new(inlineEquationSystemAppend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), tpl.clone(), shared.clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMPBACKENDINLINE.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqSystems(eqs.clone(), (literal!("Result DAE after Inline.")).clone()), '__try0);
        }
        assign_field!(
            shared.globalKnownVars = unwrap_break_err!(inlineVariables(shared.globalKnownVars.clone(), tpl.clone()), '__try0).0,
            shared.externalObjects = unwrap_break_err!(inlineVariables(shared.externalObjects.clone(), tpl.clone()), '__try0).0,
            shared.initialEqs = unwrap_break_err!(inlineEquationArray(shared.initialEqs.clone(), tpl.clone()), '__try0).0,
            shared.removedEqs = unwrap_break_err!(inlineEquationArray(shared.removedEqs.clone(), tpl.clone()), '__try0).0
        );
        outBackendDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() });
        Ok::<_, anyhow::Error>((eqs.clone(), outBackendDAE.clone(), shared.clone(), tpl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            eqs = __try0_o0;
            outBackendDAE = __try0_o1;
            shared = __try0_o2;
            tpl = __try0_o3;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln((literal!("BackendInline.inlineCallsBDAE failed")).clone())?;
            }
            return Err(__try0_err);
        }
    }
    outBackendDAE = BackendDAEOptimize::simplifyComplexFunction1(outBackendDAE.clone(), false)?;
    Ok(outBackendDAE)
}

fn inlineEquationSystemAppend(mut eqs: Arc<BackendDAE::EqSystem>, mut tpl: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut ishared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut oeqs: Arc<BackendDAE::EqSystem> = eqs.clone();
    let mut shared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut new: Arc<BackendDAE::EqSystem>;
    let mut inlined: bool = true;
    let mut eqnsArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (eqnsArray, new, inlined, shared) = inlineEquationArrayAppend(oeqs.orderedEqs.clone(), tpl.clone(), shared.clone())?;
    if inlined.clone() {
        assign_field!(oeqs.orderedEqs = eqnsArray.clone());
        new = inlineEquationSystemAppend(new.clone(), tpl.clone(), shared.clone())?;
        oeqs = BackendDAEUtil::mergeEqSystems(new.clone(), oeqs.clone())?;
    }
    Ok(oeqs)
}

fn inlineEquationArrayAppend(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut iShared: Arc<BackendDAE::Shared>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::EqSystem>, bool, Arc<BackendDAE::Shared>)> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEquationArray.clone();
    let mut outEqs: Arc<BackendDAE::EqSystem>;
    let mut oInlined: bool;
    let mut shared: Arc<BackendDAE::Shared> = iShared.clone();
    match '__try0: {
        (outEqs, oInlined, shared) = unwrap_break_err!(inlineEquationOptArrayAppend(outEquationArray.clone(), fns.clone(), shared.clone()), '__try0);
        Ok::<_, anyhow::Error>((oInlined.clone(), outEqs.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            oInlined = __try0_o0;
            outEqs = __try0_o1;
        }
        Err(_) => {
            oInlined = false;
            outEqs = BackendDAEUtil::createEqSystem(BackendVariable::listVar(metamodelica::nil())?, BackendEquation::listEquation(metamodelica::nil())?, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("BackendInline.inlineEquationArrayAppend failed\n")).clone())?;
            }
        }
    }
    Ok((outEquationArray, outEqs, oInlined, shared))
}

fn inlineEquationOptArrayAppend(mut inEqnArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut iShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, bool, Arc<BackendDAE::Shared>)> {
    let mut outEqs: Arc<BackendDAE::EqSystem>;
    let mut oInlined: bool = false;
    let mut shared: Arc<BackendDAE::Shared> = iShared.clone();
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut inlined: bool;
    let mut tmpEqs: Arc<BackendDAE::EqSystem>;
    outEqs = BackendDAEUtil::createEqSystem(BackendVariable::listVar(metamodelica::nil())?, BackendEquation::listEquation(metamodelica::nil())?, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    for mut i in 1..=ExpandableArray::getLastUsedIndex(inEqnArray.clone()) {
        if ExpandableArray::occupied(i.clone(), inEqnArray.clone()) {
            (eqn, tmpEqs, inlined, shared) = inlineEqAppend_debug(ExpandableArray::get(i.clone(), inEqnArray.clone())?, fns.clone(), shared.clone())?;
            if inlined.clone() {
                outEqs = BackendDAEUtil::mergeEqSystems(tmpEqs.clone(), outEqs.clone())?;
                ExpandableArray::update(i.clone(), eqn.clone(), inEqnArray.clone())?;
                oInlined = true;
            }
        }
    }
    Ok((outEqs, oInlined, shared))
}

pub fn inlineEqAppend_debug(mut inEquationOption: Arc<BackendDAE::Equation>, mut inElementList: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut iShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::Equation>, Arc<BackendDAE::EqSystem>, bool, Arc<BackendDAE::Shared>)> {
    let mut outEquationOption: Arc<BackendDAE::Equation>;
    let mut outEqs: Arc<BackendDAE::EqSystem>;
    let mut inlined: bool;
    let mut shared: Arc<BackendDAE::Shared> = iShared.clone();
    outEqs = BackendDAEUtil::createEqSystem(BackendVariable::listVar(metamodelica::nil())?, BackendEquation::listEquation(metamodelica::nil())?, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (outEquationOption, outEqs, inlined, shared) = inlineEqAppend(inEquationOption.clone(), inElementList.clone(), outEqs.clone(), shared.clone());
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? && inlined.clone() {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Equation before inline: ")); __mm_s.push_str(&*BackendDump::equationString(inEquationOption.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        BackendDump::dumpEqSystem(outEqs.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tmp DAE after Inline Eqn: ")); __mm_s.push_str(&*BackendDump::equationString(outEquationOption.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok((outEquationOption, outEqs, inlined, shared))
}

fn inlineEqAppend(mut inEquation: Arc<BackendDAE::Equation>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut inEqs: Arc<BackendDAE::EqSystem>, mut iShared: Arc<BackendDAE::Shared>) -> (Arc<BackendDAE::Equation>, Arc<BackendDAE::EqSystem>, bool, Arc<BackendDAE::Shared>) {
    let mut outEquation: Arc<BackendDAE::Equation>;
    let mut outEqs: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut inlined: bool;
    let mut shared: Arc<BackendDAE::Shared> = iShared.clone();
    (outEquation, outEqs, inlined) = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: Arc<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), outEqs.clone(), shared.clone());
                    b3 = b1.clone() || b2.clone();
                    Ok(((BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?, outEqs.clone(), b3.clone()), outEqs.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqs = __wb0; shared = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr, .. } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: Arc<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), outEqs.clone(), shared.clone());
                    b3 = b1.clone() || b2.clone();
                    if b2.clone() && Expression::isScalar(e1.clone())? && Expression::isTuple(e2.clone()) {
                        e2 = Arc::new(DAE::Exp::TSUB { exp: e2.clone(), ix: 1, ty: Expression::r#typeof(e1.clone())? });
                    }
                    Ok(((BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?, outEqs.clone(), b3.clone()), outEqs.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqs = __wb0; shared = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, source, attr, .. } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: Arc<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), outEqs.clone(), shared.clone());
                    b3 = b1.clone() || b2.clone();
                    Ok(((BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?, outEqs.clone(), b3.clone()), outEqs.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqs = __wb0; shared = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e2, source, attr } => {
                    let mut b2: bool;
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: Arc<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    Ok(((Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref.clone(), exp: e2.clone(), source: source.clone(), attr: attr.clone() }), outEqs.clone(), b2.clone()), outEqs.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqs = __wb0; shared = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr } => {
                    let mut b1: bool;
                    let mut e1 = (*e1).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: Arc<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    Ok(((Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1.clone(), source: source.clone(), attr: attr.clone() }), outEqs.clone(), b1.clone()), outEqs.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqs = __wb0; shared = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, expand: crefExpand, attr } => {
                    let mut b1: bool;
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut eqn = (*eqn).clone();
                    (stmts1, b1) = Inline::inlineStatements(stmts.clone(), fns.clone(), metamodelica::nil(), false);
                    if b1.clone() {
                        eqn = Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: stmts1.clone() }), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
                    }
                    Ok((eqn.clone(), inEqs.clone(), b1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: weq, source, attr } => {
                    let mut b1: bool;
                    let mut weq_1: Arc<BackendDAE::WhenEquation>;
                    let mut eqn = (*eqn).clone();
                    let mut source = (*source).clone();
                    (weq_1, source, b1) = inlineWhenEq(weq.clone(), fns.clone(), source.clone())?;
                    if b1.clone() {
                        eqn = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: weq_1.clone(), source: source.clone(), attr: attr.clone() });
                    }
                    Ok((eqn.clone(), inEqs.clone(), b1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { conditions: explst, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut eqn = (*eqn).clone();
                    let mut explst = (*explst).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut source = (*source).clone();
                    (explst, source, b1) = Inline::inlineExps(explst.clone(), fns.clone(), source.clone());
                    (eqnslst, b2) = inlineEqsLst(eqnslst.clone(), fns.clone(), metamodelica::nil(), false);
                    (eqns, b3) = inlineEqs(eqns.clone(), fns.clone(), metamodelica::nil(), false);
                    b3 = b1.clone() || b2.clone() || b3.clone();
                    if b3.clone() {
                        eqn = Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: attr.clone() });
                    }
                    Ok((eqn.clone(), inEqs.clone(), b3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEquation.clone(), inEqs.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEquation, outEqs, inlined, shared)
}

fn inlineCallsAppend(mut inExp: Arc<DAE::Exp>, mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), mut inSource: Arc<DAE::ElementSource>, mut inEqs: Arc<BackendDAE::EqSystem>, mut iShared: Arc<BackendDAE::Shared>) -> (Arc<DAE::Exp>, Arc<DAE::ElementSource>, Arc<BackendDAE::EqSystem>, bool, Arc<BackendDAE::Shared>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outSource: Arc<DAE::ElementSource>;
    let mut outEqs: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut inlined: bool;
    let mut shared: Arc<BackendDAE::Shared> = iShared.clone();
    (outExp, outSource, outEqs, inlined) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut b: bool;
                    let mut outEqs: Arc<BackendDAE::EqSystem> = outEqs.clone();
                    let (__pa0, (_, __pa1, __pa2, _)) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(inlineCallsWork, Arc<DAE::Exp>, ((Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::EqSystem>, bool, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::EqSystem>, bool, bool)) -> Result<(Arc<DAE::Exp>, ((Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::EqSystem>, bool, bool))> + 'static>), (fns.clone(), inEqs.clone(), false, false))?;
                    e1 = __pa0.clone();
                    outEqs = __pa1.clone();
                    b = __pa2.clone();
                    source = inSource.clone();
                    e2 = e1.clone();
                    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninExp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\noutExp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(((e2.clone(), source.clone(), outEqs.clone(), b.clone()), outEqs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqs = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inSource.clone(), inEqs.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outSource, outEqs, inlined, shared)
}

fn inlineCallsWork(mut inExp: Arc<DAE::Exp>, mut inTuple: ((Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::EqSystem>, bool, bool)) -> (Arc<DAE::Exp>, ((Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::EqSystem>, bool, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: ((Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>), Arc<BackendDAE::EqSystem>, bool, bool);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: p, expLst: args, attr: Deref @ DAE::CallAttributes { inlineType, .. } }, (fns, eqSys, _, false)) => {
                    if !((Inline::checkInlineType(inlineType.clone(), fns.clone()) && Flags::getConfigEnum(Flags::INLINE_METHOD.clone())? == 2)) { bail!("guard") }
                    let mut r#fn: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut newExp: Arc<DAE::Exp>;
                    let mut comment: Option<Arc<SCode::Comment>>;
                    let mut funcname: ArcStr;
                    let mut newEqSys: Arc<BackendDAE::EqSystem>;
                    (r#fn, comment) = Inline::getFunctionBody(p.clone(), fns.clone())?;
                    funcname = (BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone(), false)?).clone();
                    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inline Function ")); __mm_s.push_str(&*funcname.clone()); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::dumpInlineTypeStr(inlineType.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("in : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (outputCrefs, newEqSys) = createEqnSysfromFunction(r#fn.clone(), args.clone(), (funcname.clone()).clone())?;
                    newExp = Expression::makeTuple(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut cr in (outputCrefs.clone()).into_iter().cloned() {
                    let __x = Expression::crefExp(cr.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("out: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(newExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if !(Inline::hasGenerateEventsAnnotation(comment.clone())?) {
                        BackendDAEUtil::traverseBackendDAEExpsEqSystemWithUpdate(newEqSys.clone(), (std::sync::Arc::new(addNoEvent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                    }
                    newEqSys = BackendDAEUtil::mergeEqSystems(newEqSys.clone(), eqSys.clone())?;
                    Ok((newExp.clone(), (fns.clone(), newEqSys.clone(), true, false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: p, expLst: _, attr: Deref @ DAE::CallAttributes { inlineType, .. } }, (fns, eqSys, b, insideIfExp)) => {
                    let mut newExp: Arc<DAE::Exp>;
                    let mut funcname: ArcStr;
                    (newExp, _) = Inline::inlineCall(inExp.clone(), metamodelica::nil(), fns.clone());
                    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                        funcname = (BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone(), false)?).clone();
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBackendInline fallback replace implementation: ")); __mm_s.push_str(&*funcname.clone()); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::dumpInlineTypeStr(inlineType.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("in : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("out: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(newExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok((newExp.clone(), (fns.clone(), eqSys.clone(), b.clone(), insideIfExp.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn addNoEvent(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool = inB.clone();
    outExp = Expression::addNoEventToRelationsAndConds(inExp.clone())?;
    outExp = Expression::addNoEventToEventTriggeringFunctions(outExp.clone())?;
    Ok((outExp, outB))
}

fn createReplacementVariables(mut inCref: Arc<DAE::ComponentRef>, mut funcName: ArcStr, mut inRepls: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut crVar: Arc<DAE::ComponentRef>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outRepls: BackendVarTransform::VariableReplacements = inRepls.clone();
    let mut eVar: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut arrExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut var: BackendDAE::Var;
    var = BackendVariable::createTmpVar(inCref.clone(), (funcName.clone()).clone())?;
    crVar = BackendVariable::varCref(var.clone())?;
    eVar = Expression::crefExp(crVar.clone())?;
    let false = (Expression::isRecord(eVar.clone())) else { bail!("pattern mismatch") };
    outRepls = BackendVarTransform::addReplacement(outRepls.clone(), inCref.clone(), eVar.clone(), None)?;
    crefs = ComponentReference::expandCref(inCref.clone(), false);
    crefs1 = ComponentReference::expandCref(crVar.clone(), false);
    match '__try0: {
        arrExp = unwrap_break_err!(Expression::getArrayOrRangeContents(eVar.clone()), '__try0);
        Ok::<_, anyhow::Error>((arrExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            arrExp = __try0_o0;
        }
        Err(_) => {
            arrExp = list![eVar.clone()];
        }
    }
    if (crefs.clone().len() as i32) != (arrExp.clone().len() as i32) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendInline.createReplacementVariables failed with array handling ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eVar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    for mut c in &*crefs.clone() {
        let mut c = c.clone();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(crefs1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa1.clone();
        crefs1 = __pa2.clone();
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(arrExp.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa3.clone();
        arrExp = __pa4.clone();
        var.varName = cr.clone();
        outVars = metamodelica::cons(var.clone(), outVars.clone());
        outRepls = BackendVarTransform::addReplacement(outRepls.clone(), c.clone(), e.clone(), None)?;
    }
    outVars = outVars.clone().reverse();
    Ok((crVar, outVars, outRepls))
}

fn createEqnSysfromFunction(mut fns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut funcname: ArcStr) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<BackendDAE::EqSystem>)> {
    let mut oOutput: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outEqs: Arc<BackendDAE::EqSystem>;
    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = inArgs.clone();
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut fnInputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut argmap: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>;
    let mut checkcr: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut eqlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ncreate EqnSys from function: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
    }
    outEqs = BackendDAEUtil::createEqSystem(BackendVariable::listVar(metamodelica::nil())?, BackendEquation::listEquation(metamodelica::nil())?, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    repl = BackendVarTransform::emptyReplacements();
    for mut r#fn in &*fns.clone() {
        let mut r#fn = r#fn.clone();
        let () = (::match_deref::match_deref! { match &(r#fn.clone()) {
        Deref @ DAE::Element::VAR { componentRef: __esc_cr, direction: DAE::VarDirection::INPUT { .. }, kind: DAE::VarKind::VARIABLE { .. }, .. } => {
            cr = (*__esc_cr).clone();
            fnInputs = metamodelica::cons(cr.clone(), fnInputs.clone());
            ()
        },
        Deref @ DAE::Element::VAR { componentRef: cr, direction: DAE::VarDirection::OUTPUT { .. }, kind: DAE::VarKind::VARIABLE { .. }, .. } if (!(Expression::isRecordType(ComponentReference::crefTypeFull(cr.clone())?)) && ComponentReference::crefDepth(cr.clone())? > 0) => {
            let mut crVar: Arc<DAE::ComponentRef>;
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
            (crVar, varLst, repl) = createReplacementVariables(cr.clone(), (funcname.clone()).clone(), repl.clone())?;
            outEqs = BackendVariable::addVarsDAE(varLst.clone(), outEqs.clone())?;
            oOutput = metamodelica::cons(crVar.clone(), oOutput.clone());
            ()
        },
        Deref @ DAE::Element::VAR { componentRef: cr, protection: DAE::VarVisibility::PROTECTED { .. }, binding: None, .. } if (!(Expression::isRecordType(ComponentReference::crefTypeFull(cr.clone())?))) => {
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
            (_, varLst, repl) = createReplacementVariables(cr.clone(), (funcname.clone()).clone(), repl.clone())?;
            varLst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut _var in (varLst.clone()).into_iter().cloned() {
            let __x = BackendVariable::setVarTS(_var.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outEqs = BackendVariable::addVarsDAE(varLst.clone(), outEqs.clone())?;
            ()
        },
        Deref @ DAE::Element::VAR { componentRef: cr, protection: DAE::VarVisibility::PROTECTED { .. }, binding: Some(eBind), .. } if (!(Expression::isRecordType(ComponentReference::crefTypeFull(cr.clone())?))) => {
            let mut crVar: Arc<DAE::ComponentRef>;
            let mut eVar: Arc<DAE::Exp>;
            let mut eq: Arc<BackendDAE::Equation>;
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
            (crVar, varLst, repl) = createReplacementVariables(cr.clone(), (funcname.clone()).clone(), repl.clone())?;
            eVar = Expression::crefExp(crVar.clone())?;
            varLst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut _var in (varLst.clone()).into_iter().cloned() {
            let __x = BackendVariable::setVarTS(_var.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outEqs = BackendVariable::addVarsDAE(varLst.clone(), outEqs.clone())?;
            eq = BackendEquation::generateEquation(eVar.clone(), eBind.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
            outEqs = BackendEquation::equationAddDAE(eq.clone(), outEqs.clone())?;
            ()
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: st }, .. } => {
            eqlst = List::map(st.clone(), (std::sync::Arc::new(BackendEquation::statementEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
            outEqs = BackendEquation::equationsAddDAE(eqlst.clone(), outEqs.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    oOutput = oOutput.clone().reverse();
    if BackendDAEUtil::systemSize(outEqs.clone())? != BackendVariable::daenumVariables(outEqs.clone()) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("newBackendInline.createEqnSysfromFunction failed for function ")); __mm_s.push_str(&*funcname.clone()); __mm_s.push_str(&*literal!("with different sizes\n")); ArcStr::from(__mm_s) }).clone())?;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(outEqs.clone())?)); __mm_s.push_str(&*literal!(" <> ")); __mm_s.push_str(&*intString(BackendVariable::daenumVariables(outEqs.clone()))); ArcStr::from(__mm_s) }).clone());
        }
        bail!("fail");
    }
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\noriginal function body of: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
        BackendDump::printEqSystem(outEqs.clone())?;
        metamodelica::print((literal!("\nDump replacements: ")).clone());
        BackendVarTransform::dumpReplacements(repl.clone())?;
    }
    assign_field!(outEqs.orderedEqs = BackendEquation::listEquation((InlineArrayEquations::getScalarArrayEqns(BackendEquation::equationList(outEqs.orderedEqs.clone())?)).0)?);
    outEqs = BackendVarTransform::performReplacementsEqSystem(outEqs.clone(), repl.clone())?;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n replaced protected and output for: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
        BackendDump::printEqSystem(outEqs.clone())?;
    }
    argmap = List::zip(fnInputs.clone().reverse(), args.clone());
    (argmap, checkcr) = Inline::extendCrefRecords(argmap.clone(), HashTableCG::emptyHashTable())?;
    BackendDAEUtil::traverseBackendDAEExpsEqSystemWithUpdate(outEqs.clone(), (std::sync::Arc::new(replaceArgs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))> + 'static>), (argmap.clone(), checkcr.clone(), true))?;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nreplaced input arguments for: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
        BackendDump::printEqSystem(outEqs.clone())?;
    }
    Ok((oOutput, outEqs))
}

fn addReplacement(mut iCr: Arc<DAE::ComponentRef>, mut iExp: Arc<DAE::Exp>, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oRepl: BackendVarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &(iCr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: tp, .. } if (!(Expression::isRecordType(tp.clone())) && !(Expression::isArrayType(tp.clone()))) => {
            BackendVarTransform::addReplacement(iRepl.clone(), iCr.clone(), iExp.clone(), None)?
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: tp, .. } if (Expression::isArrayType(tp.clone())) => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut arrExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut e: Arc<DAE::Exp>;
            crefs = ComponentReference::expandCref(iCr.clone(), false);
            repl = iRepl.clone();
            arrExp = Expression::getArrayOrRangeContents(iExp.clone())?;
            for mut c in &*crefs.clone() {
                let mut c = c.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(arrExp.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                arrExp = __pa1.clone();
                repl = BackendVarTransform::addReplacement(repl.clone(), c.clone(), e.clone(), None)?;
            }
            repl.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRepl)
}

fn replaceArgs(mut inExp: Arc<DAE::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool);
    (outExp, outTuple) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(fnptr!(Inline::replaceArgs, Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))> + 'static>), inTuple.clone())?;
    if !(Util::tuple33(outTuple.clone())) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln((literal!("BackendInline.replaceArgs failed")).clone())?;
        }
        bail!("fail");
    }
    Ok((outExp, outTuple))
}

