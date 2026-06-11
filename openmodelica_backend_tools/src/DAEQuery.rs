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
use openmodelica_backend::BackendEquation;
use openmodelica_backend::BackendVariable;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub(crate) const matlabStringDelim: &'static str = "'";

pub fn writeAdjacencyMatrix(mut dlow: Arc<BackendDAE::BackendDAE>, mut fileNamePrefix: ArcStr, mut flatModelicaStr: ArcStr) -> Result<ArcStr> {
    let mut fileName: ArcStr;
    fileName = ((match flatModelicaStr {
        mut flatStr => {
            let mut file: ArcStr;
            let mut strIMatrix: ArcStr;
            let mut strVariables: ArcStr;
            let mut strEquations: ArcStr;
            let mut m: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>;
            file = (stringAppend((fileNamePrefix).clone(), (literal!("_imatrix.m")).clone())).clone();
            m = adjacencyMatrix(dlow.clone())?;
            strIMatrix = (getAdjacencyMatrix(m.clone())).clone();
            strVariables = (getVariables(dlow.clone())?).clone();
            strEquations = (getEquations(dlow)?).clone();
            strIMatrix = stringAppendList(list![(strIMatrix).clone(), (literal!("\n")).clone(), (strVariables).clone(), (literal!("\n\n\n")).clone(), (strEquations).clone(), (literal!("\n\n\n")).clone(), (flatStr.clone()).clone()]);
            System::writeFile((file.clone()).clone(), (strIMatrix).clone())?;
            file
        },
    })).clone();
    Ok(fileName)
}

pub(crate) fn getEquations(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<ArcStr> {
    let mut strEqs: ArcStr;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut ls1: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(inBackendDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, shared: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa0.clone();
    ls1 = List::map(BackendEquation::equationList(syst.orderedEqs.clone())?, (std::sync::Arc::new(equationStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?;
    strEqs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EqStr = {")); __mm_s.push_str(&*stringDelimitList(ls1, (literal!(",")).clone())); __mm_s.push_str(&*literal!("};")); ArcStr::from(__mm_s) }).clone();
    Ok(strEqs)
}

pub(crate) fn equationStr(mut inEquation: Arc<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inEquation) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1).clone(), (literal!(" = ")).clone(), (s2).clone(), (literal!(";'")).clone()]);
            res
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1).clone(), (literal!(" = ")).clone(), (s2).clone(), (literal!(";'")).clone()]);
            res
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1).clone(), (literal!(" = ")).clone(), (s2).clone(), (literal!(";'")).clone()]);
            res
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1).clone(), (literal!(" = ")).clone(), (s2).clone(), (literal!(";'")).clone()]);
            res
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition, whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e1, right: e2, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            s3 = (ExpressionBasics::printExpStr(condition.clone())?).clone();
            res = stringAppendList(list![(literal!("'when ")).clone(), (s3).clone(), (literal!(" then ")).clone(), (s1).clone(), (literal!(" = ")).clone(), (s2).clone(), (literal!("; end when;'")).clone()]);
            res
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(literal!("'")).clone(), (s1).clone(), (literal!("= 0")).clone(), (literal!(";'")).clone()]);
            res
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            let mut res: ArcStr;
            res = stringAppendList(list![(literal!("Algorithm\n")).clone()]);
            res
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>) -> ArcStr {
    let mut strIMatrix: ArcStr;
    let mut mlen: i32;
    let mut mlen_str: ArcStr;
    let mut m_1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
    let mut mstr: ArcStr;
    mlen = metamodelica::arrayLength(m.clone());
    mlen_str = (intString(mlen)).clone();
    m_1 = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    mstr = (getAdjacencyMatrix2(m_1, 1)).clone();
    strIMatrix = stringAppendList(list![(literal!("% Adjacency Matrix\n")).clone(), (literal!("% ====================================\n")).clone(), (literal!("% number of rows: ")).clone(), (mlen_str).clone(), (literal!("\n")).clone(), (literal!("IM={")).clone(), (mstr).clone(), (literal!("};")).clone()]);
    strIMatrix
}

fn getAdjacencyMatrix2(mut inStringLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut rowIndex: i32) -> ArcStr {
    let mut strIMatrix: ArcStr;
    strIMatrix = ((::match_deref::match_deref! { match &(inStringLstLst) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: row, tail: Deref @ metamodelica::List::Nil } => {
            let mut r#str: ArcStr;
            let mut str1: ArcStr;
            str1 = (getAdjacencyRow(row.clone())).clone();
            r#str = stringAppendList(list![(literal!("{")).clone(), (str1).clone(), (literal!("}")).clone()]);
            r#str
        },
        Deref @ metamodelica::List::Cons { head: row, tail: rows } => {
            let mut r#str: ArcStr;
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = (getAdjacencyRow(row.clone())).clone();
            str2 = (getAdjacencyMatrix2(rows.clone(), rowIndex + 1)).clone();
            r#str = stringAppendList(list![(literal!("{")).clone(), (str1).clone(), (literal!("},")).clone(), (str2).clone()]);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    strIMatrix
}

fn getAdjacencyRow(mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut strRow: ArcStr;
    strRow = ((::match_deref::match_deref! { match &(inStringLst) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Nil } => {
            x.clone()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut s: ArcStr;
            let mut s2: ArcStr;
            s2 = (getAdjacencyRow(xs.clone())).clone();
            s = stringAppendList(list![(x.clone()).clone(), (literal!(",")).clone(), (s2).clone()]);
            s
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    strRow
}

pub(crate) fn getVariables(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<ArcStr> {
    let mut strVars: ArcStr;
    strVars = ((::match_deref::match_deref! { match &(inBackendDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars1, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
            let mut s: ArcStr;
            vars = BackendVariable::varList(vars1.clone())?;
            s = (dumpVars(vars)?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VL = {")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("};")); ArcStr::from(__mm_s) }).clone();
            s
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(strVars)
}

pub(crate) fn dumpVars(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut strVars: ArcStr;
    strVars = (dumpVars2(vars, 1)?).clone();
    Ok(strVars)
}

fn dumpVars2(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut strVars: ArcStr;
    strVars = ('mc: {
        let __mc_input = (inVarLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr, .. }, tail: Deref @ metamodelica::List::Nil }, _) => {
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    str1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("'")).clone(), (str1.clone()).clone(), (literal!("'")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: cr, .. }, tail: xs }, varno) => {
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut varno_1: i32;
                    str1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    varno_1 = varno.clone() + 1;
                    str2 = (dumpVars2(xs.clone(), varno_1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("'")).clone(), (str1.clone()).clone(), (literal!("',")).clone(), (str2.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(strVars)
}

pub(crate) fn adjacencyMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<metamodelica::Array<Arc<metamodelica::List<ArcStr>>>> {
    let mut outAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>;
    outAdjacencyMatrix = 'mc: {
        let __mc_input = inBackendDAE;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut eqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut lstlst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    let mut arr: metamodelica::Array<Arc<metamodelica::List<ArcStr>>>;
                    eqnsl = BackendEquation::equationList(eqns.clone())?;
                    lstlst = adjacencyMatrix2(vars.clone(), eqnsl.clone())?;
                    arr = metamodelica::arrayFromVec(lstlst.clone().into_iter().cloned().collect());
                    Ok(arr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("DAEQuery.adjacencyMatrix failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAdjacencyMatrix)
}

fn adjacencyMatrix2(mut inVariables: BackendDAE::Variables, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outStringLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
    outStringLstLst = 'mc: {
        let __mc_input = (inVariables, inEquationLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ metamodelica::List::Cons { head: e, tail: eqns }) => {
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    let mut row: Arc<metamodelica::List<ArcStr>>;
                    lst = adjacencyMatrix2(vars.clone(), eqns.clone())?;
                    row = adjacencyRow(vars.clone(), e.clone())?;
                    Ok(metamodelica::cons(row.clone(), lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    metamodelica::print((literal!("adjacency_matrix2 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLstLst)
}

fn adjacencyRow(mut inVariables: BackendDAE::Variables, mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outIntegerLst: Arc<metamodelica::List<ArcStr>>;
    outIntegerLst = 'mc: {
        let __mc_input = (inVariables, inEquation);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst2: Arc<metamodelica::List<ArcStr>>;
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone());
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst2: Arc<metamodelica::List<ArcStr>>;
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone());
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst2: Arc<metamodelica::List<ArcStr>>;
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone());
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst2: Arc<metamodelica::List<ArcStr>>;
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    lst1 = adjacencyRowExp(Expression::crefExp(cr.clone())?, vars.clone());
                    lst2 = adjacencyRowExp(e.clone(), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst2: Arc<metamodelica::List<ArcStr>>;
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    lst1 = adjacencyRowExp(Expression::crefExp(cr.clone())?, vars.clone());
                    lst2 = adjacencyRowExp(e.clone(), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }) => {
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    res = adjacencyRowExp(e.clone(), vars.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: we, .. }) => {
                    let mut lst1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst2: Arc<metamodelica::List<ArcStr>>;
                    let mut res: Arc<metamodelica::List<ArcStr>>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut cr: Arc<DAE::ComponentRef>;
                    (cr, e2) = BackendEquation::getWhenEquationExpr(we.clone())?;
                    e1 = Expression::crefExp(cr.clone())?;
                    lst1 = adjacencyRowExp(e1.clone(), vars.clone());
                    lst2 = adjacencyRowExp(e2.clone(), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }) => {
                    let mut res_1: Arc<metamodelica::List<ArcStr>>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut lstres: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    expl = Algorithm::getAllExps(alg.clone())?;
                    lstres = List::map1(expl.clone(), (std::sync::Arc::new(fnptr!(adjacencyRowExp, Arc<DAE::Exp>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone())?;
                    res_1 = List::flatten(lstres.clone())?;
                    Ok(res_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    metamodelica::print((literal!("- DAEQuery.adjacencyRow failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

// protected function adjacencyRowStmts "author: PA
//   Helper function to adjacencyRow, investigates statements for
//   variables, returning variable indexes."
//   input list<DAE.Statement> inAlgorithmStatementLst;
//   input BackendDAE.Variables inVariables;
//   output list<String> outStringLst;
// algorithm
//   outStringLst := matchcontinue (inAlgorithmStatementLst,inVariables)
//     local
//       list<String> lst1,lst2,lst3,res,lst3_1;
//       DAE.Type tp;
//       DAE.ComponentRef cr;
//       DAE.Exp e, e1;
//       list<DAE.Statement> rest,stmts;
//       BackendDAE.Variables vars;
//       list<DAE.Exp> expl;
//       DAE.Else else_;
//       list<list<String>> lstlst;
//
//     case ({},_) then {};
//
//     case ((DAE.STMT_ASSIGN(type_ = tp,exp1 = e1,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lst3 = adjacencyRowExp(e1, vars);
//         res = List.flatten({lst1,lst2,lst3});
//       then
//         res;
//
//     case ((DAE.STMT_TUPLE_ASSIGN(type_ = tp,expExpLst = expl,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lstlst = List.map1(expl, adjacencyRowExp, vars);
//         lst3_1 = List.flatten(lstlst);
//         res = List.flatten({lst1,lst2,lst3_1});
//       then
//         res;
//
//     case ((DAE.STMT_ASSIGN_ARR(type_ = tp,componentRef = cr,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lst3 = adjacencyRowExp(Expression.crefExp(cr), vars);
//         res = List.flatten({lst1,lst2,lst3});
//       then
//         res;
//
//     case ((DAE.STMT_IF(exp = e,statementLst = stmts,else_ = else_) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on IF not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_FOR(type_ = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on FOR not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_PARFOR(type_ = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on PARFOR not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_WHILE(exp = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on WHILE not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_WHEN(exp = e) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on WHEN not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_ASSERT(cond = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on ASSERT not implemented\n");
//       then
//         {};
//   end matchcontinue;
// end adjacencyRowStmts;
fn adjacencyRowExp(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>>;
    outStringLst = 'mc: {
        let __mc_input = (inExp, inVariables);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut p_1: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    p_1 = List::map1r(p.clone(), (std::sync::Arc::new(fnptr!(intSub, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)?;
                    pStr = List::map(p_1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, exp2: e2, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    pStr = adjacencyRowExp(e.clone(), vars.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    pStr = adjacencyRowExp(e.clone(), vars.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1 @ Deref @ DAE::Exp::RELATION { operator: op1, exp2: ee2, .. }, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    let mut s3: Arc<metamodelica::List<ArcStr>>;
                    let mut s: ArcStr;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut opStr: ArcStr;
                    opStr = (ExpressionDump::relopSymbol(op1.clone())?).clone();
                    s = (printExpStr(ee2.clone())).clone();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone());
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (s.clone()).clone(), (literal!(",'")).clone(), (opStr.clone()).clone(), (literal!("' {")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1 @ Deref @ DAE::Exp::LBINARY { .. }, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    let mut s3: Arc<metamodelica::List<ArcStr>>;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut sb: ArcStr;
                    printExpStr(e1.clone());
                    sb = stringAppendList(list![(literal!("'true',")).clone(), (literal!("'=='")).clone()]);
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone());
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (sb.clone()).clone(), (literal!(",")).clone(), (literal!("{")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1 @ Deref @ DAE::Exp::CREF { .. }, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    let mut s3: Arc<metamodelica::List<ArcStr>>;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut sb: ArcStr;
                    sb = stringAppendList(list![(literal!("'true',")).clone(), (literal!("'=='")).clone()]);
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone());
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (sb.clone()).clone(), (literal!(" {")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut s2: Arc<metamodelica::List<ArcStr>>;
                    let mut s3: Arc<metamodelica::List<ArcStr>>;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut sb: ArcStr;
                    sb = (printExpStr(e1.clone())).clone();
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    ss1 = (getAdjacencyRow(s1.clone())).clone();
                    s2 = adjacencyRowExp(e2.clone(), vars.clone());
                    ss2 = (getAdjacencyRow(s2.clone())).clone();
                    s3 = adjacencyRowExp(e3.clone(), vars.clone());
                    ss3 = (getAdjacencyRow(s3.clone())).clone();
                    ss = stringAppendList(list![(literal!("{'if', ")).clone(), (literal!("'")).clone(), (sb.clone()).clone(), (literal!("' {")).clone(), (ss1.clone()).clone(), (literal!("}")).clone(), (literal!(",{")).clone(), (ss2.clone()).clone(), (literal!("},")).clone(), (ss3.clone()).clone(), (literal!("}")).clone()]);
                    pStr = list![(ss.clone()).clone()];
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, vars) => {
                    let mut p: Arc<metamodelica::List<i32>>;
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    (_, p) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    pStr = List::map(p.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: expl, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    lst = List::map1(expl.clone(), (std::sync::Arc::new(fnptr!(adjacencyRowExp, Arc<DAE::Exp>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone())?;
                    pStr = List::flatten(lst.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    lst = List::map1(expl.clone(), (std::sync::Arc::new(fnptr!(adjacencyRowExp, Arc<DAE::Exp>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone())?;
                    pStr = List::flatten(lst.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: explTpl, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    pStr = adjacencyRowMatrixExp(explTpl.clone(), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { .. }, _) => {
                    metamodelica::print((literal!("- DAEQuery.adjacency_row_exp TUPLE not impl. yet.")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    pStr = adjacencyRowExp(e.clone(), vars.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    pStr = adjacencyRowExp(e.clone(), vars.clone());
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { expr: e1, iterators: iters, .. }, vars) => {
                    let mut pStr: Arc<metamodelica::List<ArcStr>>;
                    let mut s1: Arc<metamodelica::List<ArcStr>>;
                    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    s1 = adjacencyRowExp(e1.clone(), vars.clone());
                    lst = List::map1(iters.clone(), (std::sync::Arc::new(adjacencyRowIter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone())?;
                    pStr = List::flatten(metamodelica::cons(s1.clone(), lst.clone()))?;
                    Ok(pStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outStringLst
}

fn adjacencyRowIter(mut iter: Arc<DAE::ReductionIterator>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strs: Arc<metamodelica::List<ArcStr>>;
    strs = (::match_deref::match_deref! { match &(iter) {
        Deref @ DAE::ReductionIterator { guardExp: Some(e1), exp: e2, .. } => {
            let mut s1: Arc<metamodelica::List<ArcStr>>;
            let mut s2: Arc<metamodelica::List<ArcStr>>;
            s1 = adjacencyRowExp(e1.clone(), vars.clone());
            s2 = adjacencyRowExp(e2.clone(), vars);
            listAppend(s1, s2)
        },
        Deref @ DAE::ReductionIterator { exp: e1, .. } => {
            adjacencyRowExp(e1.clone(), vars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(strs)
}

fn adjacencyRowMatrixExp(mut inTplExpExpBooleanLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>>;
    outStringLst = (::match_deref::match_deref! { match &((inTplExpExpBooleanLstLst, inVariables)) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: expl_1, tail: es }, vars) => {
            let mut res1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
            let mut pStr: Arc<metamodelica::List<ArcStr>>;
            let mut res1_1: Arc<metamodelica::List<ArcStr>>;
            let mut res2: Arc<metamodelica::List<ArcStr>>;
            res1 = List::map1(expl_1.clone(), (std::sync::Arc::new(fnptr!(adjacencyRowExp, Arc<DAE::Exp>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), vars.clone())?;
            res2 = adjacencyRowMatrixExp(es.clone(), vars.clone())?;
            res1_1 = List::flatten(res1)?;
            pStr = listAppend(res1_1, res2);
            pStr
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStringLst)
}

fn printExpStr(mut e: Arc<DAE::Exp>) -> ArcStr {
    let mut s: ArcStr;
    s = (ExpressionDump::printExp2Str::<()>(e, (literal!("'")).clone(), None, None)).clone();
    s
}

