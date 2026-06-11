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
use openmodelica_frontend_dump::Dump;

pub fn rewriteBlockCall(mut inPg: Absyn::Program, mut inDefs: Absyn::Program) -> Result<Absyn::Program> {
    let mut newOut: Absyn::Program;
    newOut = (match inDefs.clone() {
        _ => {
            let mut pg2: Absyn::Program;
            let mut res: ArcStr;
            pg2 = parseProgram(inPg, inDefs)?;
            res = (Dump::unparseStr(pg2.clone(), false, Dump::defaultDumpOptions.clone())?).clone();
            metamodelica::print((res.clone()).clone());
            pg2.clone()
        },
    });
    Ok(newOut)
}

fn parseProgram(mut inPg: Absyn::Program, mut defs: Absyn::Program) -> Result<Absyn::Program> {
    let mut outPg: Absyn::Program = inPg.clone();
    outPg = (match outPg.clone() {
        Absyn::Program { .. } => {
            outPg.classes = parseClasses(outPg.classes.clone(), defs)?;
            outPg
        },
    });
    Ok(outPg)
}

pub(crate) fn parseClasses(mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>, mut defs: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Class>>>> {
    let mut out_classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    out_classes = (::match_deref::match_deref! { match &(classes) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: cls, tail: r_classes } => {
            let mut nr_classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
            let mut n_cls: Arc<Absyn::Class>;
            nr_classes = parseClasses(r_classes.clone(), defs.clone())?;
            n_cls = parseClass(cls.clone(), defs)?;
            metamodelica::cons(n_cls.clone(), nr_classes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_classes)
}

pub(crate) fn parseClass(mut in_class: Arc<Absyn::Class>, mut defs: Absyn::Program) -> Result<Arc<Absyn::Class>> {
    let mut out_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    out_class = (::match_deref::match_deref! { match &(in_class) {
        __esc_out_class @ Deref @ Absyn::Class { body, .. } => {
            out_class = (*__esc_out_class).clone();
            assign_field!(out_class.body = parseClassDef(body.clone(), defs)?);
            out_class.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_class)
}

fn parseClassDef(mut in_def: Arc<Absyn::ClassDef>, mut defs: Absyn::Program) -> Result<Arc<Absyn::ClassDef>> {
    let mut out_def: Arc<Absyn::ClassDef>;
    out_def = (::match_deref::match_deref! { match &(in_def) {
        Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts, ann, comment } => {
            let mut nclsp: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            (nclsp, eqs, elems, _) = parseClassParts(classParts.clone(), defs, metamodelica::nil(), metamodelica::nil(), 0)?;
            Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: metamodelica::cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: elems.clone() }), metamodelica::cons(Arc::new(Absyn::ClassPart::EQUATIONS { contents: eqs.clone() }), nclsp.clone())), ann: ann.clone(), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_def)
}

fn parseClassParts(mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut defs: Absyn::Program, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut instNo: i32) -> Result<(Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut out_classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (out_classes, eqs, elems, newInstNo) = (::match_deref::match_deref! { match &(classes) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), oldEqs, oldElems, instNo)
        },
        Deref @ metamodelica::List::Cons { head: cls, tail: r_classes } => {
            let mut nr_classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut n_cls: Arc<Absyn::ClassPart>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count1: i32;
            (n_cls, eqs2, elems2, count1) = parseClassPart(cls.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nr_classes, eqs1, elems1, count) = parseClassParts(r_classes.clone(), defs, eqs2.clone(), elems2.clone(), count1.clone())?;
            (metamodelica::cons(n_cls.clone(), nr_classes.clone()), eqs1.clone(), elems1.clone(), count.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_classes, eqs, elems, newInstNo))
}

fn parseClassPart(mut in_def: Arc<Absyn::ClassPart>, mut defs: Absyn::Program, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut instNo: i32) -> Result<(Arc<Absyn::ClassPart>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut out_def: Arc<Absyn::ClassPart>;
    let mut reqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut relems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (out_def, reqs, relems, newInstNo) = (::match_deref::match_deref! { match &(in_def) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: elems } => {
            (Arc::new(Absyn::ClassPart::PUBLIC { contents: elems.clone() }), metamodelica::nil(), metamodelica::nil(), instNo)
        },
        Deref @ Absyn::ClassPart::PROTECTED { contents: elems } => {
            (Arc::new(Absyn::ClassPart::PROTECTED { contents: elems.clone() }), metamodelica::nil(), metamodelica::nil(), instNo)
        },
        Deref @ Absyn::ClassPart::CONSTRAINTS { contents: exps } => {
            (Arc::new(Absyn::ClassPart::CONSTRAINTS { contents: exps.clone() }), metamodelica::nil(), metamodelica::nil(), instNo)
        },
        Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs } => {
            let mut neqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (neqs, eqs1, elems1, count) = parseEquations(eqs.clone(), defs, oldEqs, oldElems, instNo)?;
            (Arc::new(Absyn::ClassPart::EQUATIONS { contents: neqs.clone() }), eqs1.clone(), elems1.clone(), count.clone())
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs } => {
            let mut neqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (neqs, eqs1, elems1, count) = parseEquations(eqs.clone(), defs, oldEqs, oldElems, instNo)?;
            (Arc::new(Absyn::ClassPart::INITIALEQUATIONS { contents: neqs.clone() }), eqs1.clone(), elems1.clone(), count.clone())
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs } => {
            (Arc::new(Absyn::ClassPart::ALGORITHMS { contents: algs.clone() }), metamodelica::nil(), metamodelica::nil(), instNo)
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs } => {
            (Arc::new(Absyn::ClassPart::INITIALALGORITHMS { contents: algs.clone() }), metamodelica::nil(), metamodelica::nil(), instNo)
        },
        Deref @ Absyn::ClassPart::EXTERNAL { externalDecl, annotation_ } => {
            (Arc::new(Absyn::ClassPart::EXTERNAL { externalDecl: externalDecl.clone(), annotation_: annotation_.clone() }), metamodelica::nil(), metamodelica::nil(), instNo)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_def, reqs, relems, newInstNo))
}

fn parseEquations(mut classes: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut defs: Absyn::Program, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut instNo: i32) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut out_classes: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (out_classes, eqs, elems, newInstNo) = (::match_deref::match_deref! { match &(classes) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), oldEqs, oldElems, instNo)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, comment: cmt, info }, tail: r_classes } => {
            let mut neq: Arc<Absyn::Equation>;
            let mut nr_classes: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count1: i32;
            (neq, eqs2, elems2, count1) = parseEquation(eq.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nr_classes, eqs1, elems1, count) = parseEquations(r_classes.clone(), defs, eqs2.clone(), elems2.clone(), count1.clone())?;
            (metamodelica::cons(Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: neq.clone(), comment: cmt.clone(), info: info.clone() }), nr_classes.clone()), eqs1.clone(), elems1.clone(), count.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { comment }, tail: r_classes } => {
            let mut nr_classes: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (nr_classes, eqs1, elems1, count) = parseEquations(r_classes.clone(), defs, oldEqs, oldElems, instNo)?;
            (metamodelica::cons(Arc::new(Absyn::EquationItem::EQUATIONITEMCOMMENT { comment: (comment.clone()).clone() }), nr_classes.clone()), eqs1.clone(), elems1.clone(), count.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_classes, eqs, elems, newInstNo))
}

fn parseEquation(mut in_eq: Arc<Absyn::Equation>, mut defs: Absyn::Program, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut instNo: i32) -> Result<(Arc<Absyn::Equation>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut out_eq: Arc<Absyn::Equation>;
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (out_eq, eqs, elems, newInstNo) = (::match_deref::match_deref! { match &(in_eq) {
        Deref @ Absyn::Equation::EQ_IF { ifExp: exp1, equationTrueItems: leq1, elseIfBranches: tup1, equationElseItems: leq2 } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nleq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut nleq2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs3: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems3: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count1: i32;
            let mut count2: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nleq1, eqs2, elems2, count1) = parseEquations(leq1.clone(), defs.clone(), eqs1.clone(), elems1.clone(), count.clone())?;
            (nleq2, eqs3, elems3, count2) = parseEquations(leq2.clone(), defs, eqs2.clone(), elems2.clone(), count1.clone())?;
            (Arc::new(Absyn::Equation::EQ_IF { ifExp: nexp1.clone(), equationTrueItems: nleq1.clone(), elseIfBranches: tup1.clone(), equationElseItems: nleq2.clone() }), eqs3.clone(), elems3.clone(), count2.clone())
        },
        Deref @ Absyn::Equation::EQ_EQUALS { leftSide: exp1, rightSide: exp2 } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nexp2: Arc<Absyn::Exp>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count1: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nexp2, eqs2, elems2, count1) = parseExpression(exp2.clone(), defs, eqs1.clone(), elems1.clone(), count.clone())?;
            (Arc::new(Absyn::Equation::EQ_EQUALS { leftSide: nexp1.clone(), rightSide: nexp2.clone() }), eqs2.clone(), elems2.clone(), count1.clone())
        },
        Deref @ Absyn::Equation::EQ_PDE { leftSide: exp1, rightSide: exp2, domain } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nexp2: Arc<Absyn::Exp>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count1: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nexp2, eqs2, elems2, count1) = parseExpression(exp2.clone(), defs, eqs1.clone(), elems1.clone(), count.clone())?;
            (Arc::new(Absyn::Equation::EQ_PDE { leftSide: nexp1.clone(), rightSide: nexp2.clone(), domain: domain.clone() }), eqs2.clone(), elems2.clone(), count1.clone())
        },
        Deref @ Absyn::Equation::EQ_CONNECT { connector1: cr1, connector2: cr2 } => {
            (Arc::new(Absyn::Equation::EQ_CONNECT { connector1: cr1.clone(), connector2: cr2.clone() }), oldEqs, oldElems, instNo)
        },
        Deref @ Absyn::Equation::EQ_FOR { iterators: fi, forEquations: leq1 } => {
            let mut nleq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (nleq1, eqs2, elems2, count) = parseEquations(leq1.clone(), defs, oldEqs, oldElems, instNo)?;
            (Arc::new(Absyn::Equation::EQ_FOR { iterators: fi.clone(), forEquations: nleq1.clone() }), eqs2.clone(), elems2.clone(), count.clone())
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { whenExp: exp1, whenEquations: leq1, elseWhenEquations: tup1 } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nleq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            (nexp1, _, _, _) = parseExpression(exp1.clone(), defs.clone(), oldEqs.clone(), oldElems.clone(), instNo)?;
            (nleq1, _, _, _) = parseEquations(leq1.clone(), defs, oldEqs.clone(), oldElems.clone(), instNo)?;
            (Arc::new(Absyn::Equation::EQ_WHEN_E { whenExp: nexp1.clone(), whenEquations: nleq1.clone(), elseWhenEquations: tup1.clone() }), oldEqs, oldElems, instNo)
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: cr1, functionArgs: farg } => {
            (Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: cr1.clone(), functionArgs: farg.clone() }), oldEqs, oldElems, instNo)
        },
        Deref @ Absyn::Equation::EQ_FAILURE { equ: eqi } => {
            (Arc::new(Absyn::Equation::EQ_FAILURE { equ: eqi.clone() }), oldEqs, oldElems, instNo)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_eq, eqs, elems, newInstNo))
}

fn parseExpression(mut in_eq: Arc<Absyn::Exp>, mut defs: Absyn::Program, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut instNo: i32) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut out_eq: Arc<Absyn::Exp>;
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (out_eq, eqs, elems, newInstNo) = (::match_deref::match_deref! { match &(in_eq.clone()) {
        Deref @ Absyn::Exp::BINARY { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nexp2: Arc<Absyn::Exp>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count2: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nexp2, eqs2, elems2, count2) = parseExpression(exp2.clone(), defs, eqs1.clone(), elems1.clone(), count.clone())?;
            (Arc::new(Absyn::Exp::BINARY { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() }), eqs2.clone(), elems2.clone(), count2.clone())
        },
        Deref @ Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nexp2: Arc<Absyn::Exp>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count2: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nexp2, eqs2, elems2, count2) = parseExpression(exp2.clone(), defs, eqs1.clone(), elems1.clone(), count.clone())?;
            (Arc::new(Absyn::Exp::LBINARY { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() }), eqs2.clone(), elems2.clone(), count2.clone())
        },
        Deref @ Absyn::Exp::UNARY { op, exp: exp2 } => {
            let mut nexp2: Arc<Absyn::Exp>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (nexp2, eqs2, elems2, count) = parseExpression(exp2.clone(), defs, oldEqs, oldElems, instNo)?;
            (Arc::new(Absyn::Exp::UNARY { op: op.clone(), exp: nexp2.clone() }), eqs2.clone(), elems2.clone(), count.clone())
        },
        Deref @ Absyn::Exp::LUNARY { op, exp: exp2 } => {
            let mut nexp2: Arc<Absyn::Exp>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (nexp2, eqs2, elems2, count) = parseExpression(exp2.clone(), defs, oldEqs, oldElems, instNo)?;
            (Arc::new(Absyn::Exp::LUNARY { op: op.clone(), exp: nexp2.clone() }), eqs2.clone(), elems2.clone(), count.clone())
        },
        Deref @ Absyn::Exp::IFEXP { ifExp: ife, trueBranch: exp1, elseBranch: exp2, elseIfBranch: elif } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nexp2: Arc<Absyn::Exp>;
            let mut nife: Arc<Absyn::Exp>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs3: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs4: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems3: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems4: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            let mut count2: i32;
            let mut count3: i32;
            let mut count4: i32;
            let mut nelif: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            (nife, eqs1, elems1, count) = parseExpression(ife.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nexp1, eqs2, elems2, count2) = parseExpression(exp1.clone(), defs.clone(), eqs1.clone(), elems1.clone(), count.clone())?;
            (nexp2, eqs3, elems3, count3) = parseExpression(exp2.clone(), defs.clone(), eqs2.clone(), elems2.clone(), count2.clone())?;
            (nelif, eqs4, elems4, count4) = parseExpressionTuple(elif.clone(), defs, eqs3.clone(), elems3.clone(), count3.clone())?;
            (Arc::new(Absyn::Exp::IFEXP { ifExp: nife.clone(), trueBranch: nexp1.clone(), elseBranch: nexp2.clone(), elseIfBranch: nelif.clone() }), eqs4.clone(), elems4.clone(), count4.clone())
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            let mut nexp1: Arc<Absyn::Exp>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count: i32;
            (nexp1, eqs1, elems1, count) = parseCall(in_eq, defs, instNo, oldEqs, oldElems)?;
            (nexp1.clone(), eqs1.clone(), elems1.clone(), count.clone())
        },
        _ => {
            (in_eq, oldEqs, oldElems, instNo)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_eq, eqs, elems, newInstNo))
}

fn parseExpressionTuple(mut tuple_list: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, mut defs: Absyn::Program, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut instNo: i32) -> Result<(Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut out_tuple_list: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (out_tuple_list, eqs, elems, newInstNo) = (::match_deref::match_deref! { match &(tuple_list) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), oldEqs, oldElems, instNo)
        },
        Deref @ metamodelica::List::Cons { head: (exp1, exp2), tail: r_tuple_list } => {
            let mut ntuples: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqs3: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut elems3: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut count1: i32;
            let mut count3: i32;
            let mut nexp1: Arc<Absyn::Exp>;
            let mut nexp2: Arc<Absyn::Exp>;
            (nexp1, eqs1, elems1, count1) = parseExpression(exp1.clone(), defs.clone(), oldEqs, oldElems, instNo)?;
            (nexp2, _, _, _) = parseExpression(exp2.clone(), defs.clone(), eqs1.clone(), elems1.clone(), count1.clone())?;
            (ntuples, eqs3, elems3, count3) = parseExpressionTuple(r_tuple_list.clone(), defs, eqs1.clone(), elems1.clone(), count1.clone())?;
            (metamodelica::cons((nexp1.clone(), nexp2.clone()), ntuples.clone()), eqs3.clone(), elems3.clone(), count3.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_tuple_list, eqs, elems, newInstNo))
}

/* *
 When a function call is found, we check if it is in the block definitions, and if it is we replace it
 */
fn parseCall(mut in_eq: Arc<Absyn::Exp>, mut defs: Absyn::Program, mut instNo: i32, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32)> {
    let mut res_expr: Arc<Absyn::Exp>;
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newElems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut newInstNo: i32;
    (res_expr, newEqs, newElems, newInstNo) = 'mc: {
        let __mc_input = in_eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: _ }, functionArgs: fargs, .. } => {
                    let mut elName: ArcStr;
                    let mut elem: Arc<Absyn::ElementItem>;
                    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let mut count: i32;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(getDefinition((id.clone()).clone(), instNo, defs.clone(), fargs.clone(), oldEqs.clone(), metamodelica::nil())?) {
                        (__pa0, __pa1, true, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqs = __pa0.clone();
                    mods = __pa1.clone();
                    count = __pa2.clone();
                    elName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_autogen_")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*intString(instNo)); ArcStr::from(__mm_s) }).clone();
                    elem = Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), arrayDim: None }), components: list![Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (elName.clone()).clone(), arrayDim: metamodelica::nil(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: mods.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })) }, condition: None, comment: None })] }), info: Absyn::dummyInfo.clone(), constrainClass: None }) });
                    Ok((Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (elName.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("out")).clone(), subscripts: metamodelica::nil() }) }) }), eqs.clone(), metamodelica::cons(elem.clone(), oldElems.clone()), count.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { .. } => {
                    Ok((in_eq.clone(), oldEqs.clone(), metamodelica::nil(), instNo))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((res_expr, newEqs, newElems, newInstNo))
}

fn getDefinition(mut id: ArcStr, mut instNo: i32, mut defs: Absyn::Program, mut fargs: Arc<Absyn::FunctionArgs>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, bool, i32)> {
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut found: bool;
    let mut newInstNo: i32;
    (newEqs, newModif, found, newInstNo) = (match defs.clone() {
        Absyn::Program { .. } => parseClassesDefs((id).clone(), instNo, defs.classes.clone(), fargs, oldEqs, oldModif),
    });
    Ok((newEqs, newModif, found, newInstNo))
}

/* *
 Get the block definitions, go through all packages
 */
fn parseClassesDefs(mut id: ArcStr, mut instNo: i32, mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>, mut fargs: Arc<Absyn::FunctionArgs>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, bool, i32) {
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut found: bool;
    let mut newInstNo: i32;
    (newEqs, newModif, found, newInstNo) = 'mc: {
        let __mc_input = classes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), false, instNo))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Class { name: _, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_PACKAGE { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. }, tail: _ } => {
                    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookThroughClasses((id.clone()).clone(), instNo, fargs.clone(), classParts.clone(), oldEqs.clone(), oldModif.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqs = __pa0.clone();
                    mods = __pa1.clone();
                    Ok((eqs.clone(), mods.clone(), true, instNo + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Class { name: id2, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_BLOCK { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. }, tail: _ } => {
                    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let true = (id2.clone() == id.clone()) else { bail!("pattern mismatch") };
                    (eqs, mods) = parseArgs(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_autogen_")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*intString(instNo)); ArcStr::from(__mm_s) }).clone(), classParts.clone(), fargs.clone(), oldEqs.clone(), oldModif.clone())?;
                    Ok((eqs.clone(), mods.clone(), true, instNo + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: r_classes } => {
                    Ok(parseClassesDefs((id.clone()).clone(), instNo, r_classes.clone(), fargs.clone(), oldEqs.clone(), oldModif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found, newInstNo)
}

fn lookThroughClasses(mut id: ArcStr, mut instNo: i32, mut fargs: Arc<Absyn::FunctionArgs>, mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, bool) {
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut found: bool;
    (newEqs, newModif, found) = 'mc: {
        let __mc_input = classes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elems1 }, tail: _ } => {
                    let mut eq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let mut modif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookThroughElems((id.clone()).clone(), instNo, fargs.clone(), elems1.clone(), oldEqs.clone(), oldModif.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eq1 = __pa0.clone();
                    modif = __pa1.clone();
                    Ok((eq1.clone(), modif.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: r_classes } => {
                    Ok(lookThroughClasses((id.clone()).clone(), instNo, fargs.clone(), r_classes.clone(), oldEqs.clone(), oldModif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found)
}

fn lookThroughElems(mut id: ArcStr, mut instNo: i32, mut fargs: Arc<Absyn::FunctionArgs>, mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, bool) {
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut found: bool;
    (newEqs, newModif, found) = 'mc: {
        let __mc_input = elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: _, class_: Deref @ Absyn::Class { name: id2, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_BLOCK { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. } }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let true = (id2.clone() == id.clone()) else { bail!("pattern mismatch") };
                    (eqs, mods) = parseArgs(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_autogen_")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*intString(instNo)); ArcStr::from(__mm_s) }).clone(), classParts.clone(), fargs.clone(), oldEqs.clone(), oldModif.clone())?;
                    Ok((eqs.clone(), mods.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: _, class_: Deref @ Absyn::Class { name: _, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_PACKAGE { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. } }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookThroughClasses((id.clone()).clone(), instNo, fargs.clone(), classParts.clone(), oldEqs.clone(), oldModif.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqs = __pa0.clone();
                    mods = __pa1.clone();
                    Ok((eqs.clone(), mods.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: r_elems } => {
                    Ok(lookThroughElems((id.clone()).clone(), instNo, fargs.clone(), r_elems.clone(), oldEqs.clone(), oldModif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found)
}

fn parseArgs(mut elemId: ArcStr, mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut fargs: Arc<Absyn::FunctionArgs>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>)> {
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut mods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    (eqs, mods) = (::match_deref::match_deref! { match &(fargs) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
            let mut eqs1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut mods1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            (eqs1, mods1) = matchArgsClass((elemId.clone()).clone(), args.clone(), classes.clone(), oldEqs, oldModif)?;
            matchNamedArgsClass((elemId).clone(), argNames.clone(), classes, eqs1.clone(), mods1.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((eqs, mods))
}

/* *
uniontype NamedArg "The NamedArg uniontype consist of an Identifier for the argument and an expression
  giving the value of the argument"
  record NAMEDARG
    Ident argName "argName" ;
    Exp argValue "argValue" ;
  end NAMEDARG;

end NamedArg;
*/
fn matchArgsClass(mut elemId: ArcStr, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((classes, args.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            return Ok((oldEqs, oldModif))
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((oldEqs, oldModif))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elems1 }, tail: r_classes }, _) => {
            let mut eq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut r_args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut modif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            (eq1, modif, r_args) = matchArgsElems((elemId.clone()).clone(), args, elems1.clone(), oldEqs, oldModif)?;
            { (elemId, args, classes, oldEqs, oldModif) = ((elemId).clone(), r_args.clone(), r_classes.clone(), eq1.clone(), modif.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: r_classes }, _) => {
            { (elemId, args, classes, oldEqs, oldModif) = ((elemId).clone(), args, r_classes.clone(), oldEqs, oldModif); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn matchArgsElems(mut elemId: ArcStr, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((args.clone(), elems)) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((oldEqs, oldModif, args))
        },
        (_, Deref @ metamodelica::List::Nil) => {
            return Ok((oldEqs, oldModif, args))
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: r_args }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::PARAM { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: r_elems }) => {
            let mut modif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut r_args = (*r_args).clone();
            (modif, r_args) = matchParamArgs(args, comps.clone(), oldModif)?;
            { (elemId, args, elems, oldEqs, oldModif) = ((elemId).clone(), r_args.clone(), r_elems.clone(), oldEqs, modif.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: r_args }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::VAR { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: r_elems }) => {
            let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut r_args = (*r_args).clone();
            (eqs, r_args) = matchVarArgs((elemId.clone()).clone(), args, comps.clone(), oldEqs)?;
            { (elemId, args, elems, oldEqs, oldModif) = ((elemId).clone(), r_args.clone(), r_elems.clone(), eqs.clone(), oldModif); continue '__tco; }
        },
        (_, Deref @ metamodelica::List::Cons { head: _, tail: r_elems }) => {
            { (elemId, args, elems, oldEqs, oldModif) = ((elemId).clone(), args, r_elems.clone(), oldEqs, oldModif); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn matchParamArgs(mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((comps, args.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((oldModif, args))
        },
        (_, Deref @ metamodelica::List::Nil) => {
            return Ok((oldModif, args))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: r_comps }, Deref @ metamodelica::List::Cons { head: arg, tail: r_args }) => {
            let mut modif: Arc<Absyn::ElementArg>;
            modif = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (cName.clone()).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: arg.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
            { (args, comps, oldModif) = (r_args.clone(), r_comps.clone(), metamodelica::cons(modif.clone(), oldModif)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn matchVarArgs(mut elemId: ArcStr, mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((comps, args.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((oldEqs, args))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: r_comps }, Deref @ metamodelica::List::Cons { head: arg, tail: r_args }) => {
            let mut eq: Arc<Absyn::EquationItem>;
            eq = Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_EQUALS { leftSide: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (elemId.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (cName.clone()).clone(), subscripts: metamodelica::nil() }) }) }), rightSide: arg.clone() }), comment: None, info: Absyn::dummyInfo.clone() });
            { (elemId, args, comps, oldEqs) = ((elemId).clone(), r_args.clone(), r_comps.clone(), metamodelica::cons(eq.clone(), oldEqs)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn matchNamedArgsClass(mut elemId: ArcStr, mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((classes.clone(), nargs)) {
        (_, Deref @ metamodelica::List::Nil) => {
            return Ok((oldEqs, oldModif))
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((oldEqs, oldModif))
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName, argValue }, tail: r_nargs }) => {
            let mut eq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut modif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            (eq1, modif) = matchNamedArgClass((elemId.clone()).clone(), (argName.clone()).clone(), argValue.clone(), classes.clone(), oldEqs, oldModif);
            { (elemId, nargs, classes, oldEqs, oldModif) = ((elemId).clone(), r_nargs.clone(), classes, eq1.clone(), modif.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn matchNamedArgClass(mut elemId: ArcStr, mut argName: ArcStr, mut argValue: Arc<Absyn::Exp>, mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) {
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    (newEqs, newModif) = 'mc: {
        let __mc_input = classes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elems1 }, tail: _ } => {
                    let mut eq1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let mut modif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matchNamedArgElems((elemId.clone()).clone(), (argName.clone()).clone(), argValue.clone(), elems1.clone(), oldEqs.clone(), oldModif.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eq1 = __pa0.clone();
                    modif = __pa1.clone();
                    Ok((eq1.clone(), modif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: r_classes } => {
                    Ok(matchNamedArgClass((elemId.clone()).clone(), (argName.clone()).clone(), argValue.clone(), r_classes.clone(), oldEqs.clone(), oldModif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif)
}

fn matchNamedArgElems(mut elemId: ArcStr, mut argName: ArcStr, mut argValue: Arc<Absyn::Exp>, mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, bool) {
    let mut newEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut newModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut found: bool;
    (newEqs, newModif, found) = 'mc: {
        let __mc_input = elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::PARAM { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut modif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(matchParamNamedArg((argName.clone()).clone(), argValue.clone(), comps.clone(), oldModif.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    modif = __pa0.clone();
                    Ok((oldEqs.clone(), modif.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::VAR { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(matchVarNamedArg((elemId.clone()).clone(), (argName.clone()).clone(), argValue.clone(), comps.clone(), oldEqs.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqs = __pa0.clone();
                    Ok((eqs.clone(), oldModif.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: r_elems } => {
                    Ok(matchNamedArgElems((elemId.clone()).clone(), (argName.clone()).clone(), argValue.clone(), r_elems.clone(), oldEqs.clone(), oldModif.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found)
}

fn matchParamNamedArg(mut argName: ArcStr, mut argValue: Arc<Absyn::Exp>, mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut oldModif: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> (Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(comps) {
        Deref @ metamodelica::List::Nil => {
            return (oldModif, false)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: _ } if (cName.clone() == argName.clone()) => {
            let mut modif: Arc<Absyn::ElementArg>;
            modif = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (cName.clone()).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: argValue, info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
            return (metamodelica::cons(modif.clone(), oldModif), true)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: r_comps } => {
            { (argName, argValue, comps, oldModif) = ((argName.clone()).clone(), argValue, r_comps.clone(), oldModif); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn matchVarNamedArg(mut elemId: ArcStr, mut argName: ArcStr, mut argValue: Arc<Absyn::Exp>, mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut oldEqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(comps) {
        Deref @ metamodelica::List::Nil => {
            return (oldEqs, false)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: _ } if (cName.clone() == argName.clone()) => {
            let mut eq: Arc<Absyn::EquationItem>;
            eq = Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_EQUALS { leftSide: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (elemId).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (cName.clone()).clone(), subscripts: metamodelica::nil() }) }) }), rightSide: argValue }), comment: None, info: Absyn::dummyInfo.clone() });
            return (metamodelica::cons(eq.clone(), oldEqs), true)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: r_comps } => {
            { (elemId, argName, argValue, comps, oldEqs) = ((elemId).clone(), (argName.clone()).clone(), argValue, r_comps.clone(), oldEqs); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

