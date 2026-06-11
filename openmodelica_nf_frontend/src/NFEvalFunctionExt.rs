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

use crate::NFCeval as Ceval;
use crate::NFEvalFunction as EvalFunction;
use crate::NFEvalFunction::assignVariable;
use crate::NFExpression as Expression;
use crate::NFType as Type;
use openmodelica_util::Lapack;

pub(crate) fn Lapack_dgeev(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut jobvl: Arc<Expression::NFExpression>;
    let mut jobvr: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut ldvl: Arc<Expression::NFExpression>;
    let mut ldvr: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut wr: Arc<Expression::NFExpression>;
    let mut wi: Arc<Expression::NFExpression>;
    let mut vl: Arc<Expression::NFExpression>;
    let mut vr: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut INFO: i32;
    let mut LDA: i32;
    let mut LDVL: i32;
    let mut LDVR: i32;
    let mut LWORK: i32;
    let mut N: i32;
    let mut JOBVL: ArcStr;
    let mut JOBVR: ArcStr;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WR: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WI: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => bail!("pattern mismatch"),
    } };
    jobvl = __pa0.clone();
    jobvr = __pa1.clone();
    n = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    wr = __pa5.clone();
    wi = __pa6.clone();
    vl = __pa7.clone();
    ldvl = __pa8.clone();
    vr = __pa9.clone();
    ldvr = __pa10.clone();
    work = __pa11.clone();
    lwork = __pa12.clone();
    info = __pa13.clone();
    JOBVL = (evaluateExtStringArg(jobvl)?).clone();
    JOBVR = (evaluateExtStringArg(jobvr)?).clone();
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    LDVL = evaluateExtIntArg(ldvl)?;
    LDVR = evaluateExtIntArg(ldvr)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, WR, WI, VL, VR, WORK, INFO) = Lapack::dgeev((JOBVL).clone(), (JOBVR).clone(), N, A, LDA, LDVL, LDVR, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(wr, Expression::makeRealArray(WR)?)?;
    assignVariable(wi, Expression::makeRealArray(WI)?)?;
    assignVariableExt(vl, Expression::makeRealMatrix(VL)?)?;
    assignVariableExt(vr, Expression::makeRealMatrix(VR)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgegv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut jobvl: Arc<Expression::NFExpression>;
    let mut jobvr: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut alphar: Arc<Expression::NFExpression>;
    let mut alphai: Arc<Expression::NFExpression>;
    let mut beta: Arc<Expression::NFExpression>;
    let mut vl: Arc<Expression::NFExpression>;
    let mut ldvl: Arc<Expression::NFExpression>;
    let mut vr: Arc<Expression::NFExpression>;
    let mut ldvr: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut JOBVL: ArcStr;
    let mut JOBVR: ArcStr;
    let mut N: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut LDVL: i32;
    let mut LDVR: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let mut ALPHAR: Arc<metamodelica::List<metamodelica::Real>>;
    let mut ALPHAI: Arc<metamodelica::List<metamodelica::Real>>;
    let mut BETA: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Cons { head: __pa14, tail: Deref @ metamodelica::List::Cons { head: __pa15, tail: Deref @ metamodelica::List::Cons { head: __pa16, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
        _ => bail!("pattern mismatch"),
    } };
    jobvl = __pa0.clone();
    jobvr = __pa1.clone();
    n = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    alphar = __pa7.clone();
    alphai = __pa8.clone();
    beta = __pa9.clone();
    vl = __pa10.clone();
    ldvl = __pa11.clone();
    vr = __pa12.clone();
    ldvr = __pa13.clone();
    work = __pa14.clone();
    lwork = __pa15.clone();
    info = __pa16.clone();
    JOBVL = (evaluateExtStringArg(jobvl)?).clone();
    JOBVR = (evaluateExtStringArg(jobvr)?).clone();
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a)?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b)?;
    LDB = evaluateExtIntArg(ldb)?;
    LDVL = evaluateExtIntArg(ldvl)?;
    LDVR = evaluateExtIntArg(ldvr)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (ALPHAR, ALPHAI, BETA, VL, VR, WORK, INFO) = Lapack::dgegv((JOBVL).clone(), (JOBVR).clone(), N, A, LDA, B, LDB, LDVL, LDVR, WORK, LWORK);
    assignVariable(alphar, Expression::makeRealArray(ALPHAR)?)?;
    assignVariable(alphai, Expression::makeRealArray(ALPHAI)?)?;
    assignVariable(beta, Expression::makeRealArray(BETA)?)?;
    assignVariableExt(vl, Expression::makeRealMatrix(VL)?)?;
    assignVariableExt(vr, Expression::makeRealMatrix(VR)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgels(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut trans: Arc<Expression::NFExpression>;
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut TRANS: ArcStr;
    let mut M: i32;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
        _ => bail!("pattern mismatch"),
    } };
    trans = __pa0.clone();
    m = __pa1.clone();
    n = __pa2.clone();
    nrhs = __pa3.clone();
    a = __pa4.clone();
    lda = __pa5.clone();
    b = __pa6.clone();
    ldb = __pa7.clone();
    work = __pa8.clone();
    lwork = __pa9.clone();
    info = __pa10.clone();
    TRANS = (evaluateExtStringArg(trans)?).clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, B, WORK, INFO) = Lapack::dgels((TRANS).clone(), M, N, NRHS, A, LDA, B, LDB, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgelsx(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut jpvt: Arc<Expression::NFExpression>;
    let mut rcond: Arc<Expression::NFExpression>;
    let mut rank: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut RANK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut JPVT: Arc<metamodelica::List<i32>>;
    let mut RCOND: metamodelica::Real;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    if (args.clone().len() as i32) == 12 {
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(args) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
            _ => bail!("pattern mismatch"),
        } };
        m = __pa0.clone();
        n = __pa1.clone();
        nrhs = __pa2.clone();
        a = __pa3.clone();
        lda = __pa4.clone();
        b = __pa5.clone();
        ldb = __pa6.clone();
        jpvt = __pa7.clone();
        rcond = __pa8.clone();
        rank = __pa9.clone();
        work = __pa10.clone();
        info = __pa11.clone();
    } else {
        let (__pa13, __pa14, __pa15, __pa16, __pa17, __pa18, __pa19, __pa20, __pa21, __pa22, __pa23, __pa24) = ::match_deref::match_deref! { match &(args) {
            Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Cons { head: __pa14, tail: Deref @ metamodelica::List::Cons { head: __pa15, tail: Deref @ metamodelica::List::Cons { head: __pa16, tail: Deref @ metamodelica::List::Cons { head: __pa17, tail: Deref @ metamodelica::List::Cons { head: __pa18, tail: Deref @ metamodelica::List::Cons { head: __pa19, tail: Deref @ metamodelica::List::Cons { head: __pa20, tail: Deref @ metamodelica::List::Cons { head: __pa21, tail: Deref @ metamodelica::List::Cons { head: __pa22, tail: Deref @ metamodelica::List::Cons { head: __pa23, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa24, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } => (__pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone(), __pa23.clone(), __pa24.clone()),
            _ => bail!("pattern mismatch"),
        } };
        m = __pa13.clone();
        n = __pa14.clone();
        nrhs = __pa15.clone();
        a = __pa16.clone();
        lda = __pa17.clone();
        b = __pa18.clone();
        ldb = __pa19.clone();
        jpvt = __pa20.clone();
        rcond = __pa21.clone();
        rank = __pa22.clone();
        work = __pa23.clone();
        info = __pa24.clone();
    }
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    RCOND = evaluateExtRealArg(rcond)?;
    WORK = evaluateExtRealArrayArg(work)?;
    (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(jpvt, Expression::makeIntegerArray(JPVT)?)?;
    assignVariable(rank, Expression::makeInteger(RANK))?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgelsy(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut jpvt: Arc<Expression::NFExpression>;
    let mut rcond: Arc<Expression::NFExpression>;
    let mut rank: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut RANK: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut JPVT: Arc<metamodelica::List<i32>>;
    let mut RCOND: metamodelica::Real;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    nrhs = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    jpvt = __pa7.clone();
    rcond = __pa8.clone();
    rank = __pa9.clone();
    work = __pa10.clone();
    lwork = __pa11.clone();
    info = __pa12.clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    RCOND = evaluateExtRealArg(rcond)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, B, JPVT, RANK, WORK, INFO) = Lapack::dgelsy(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(jpvt, Expression::makeIntegerArray(JPVT)?)?;
    assignVariable(rank, Expression::makeInteger(RANK))?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgesv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut ipiv: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut IPIV: Arc<metamodelica::List<i32>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    nrhs = __pa1.clone();
    a = __pa2.clone();
    lda = __pa3.clone();
    ipiv = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    info = __pa7.clone();
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (A, IPIV, B, INFO) = Lapack::dgesv(N, NRHS, A, LDA, B, LDB);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(ipiv, Expression::makeIntegerArray(IPIV)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgglse(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut p: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut c: Arc<Expression::NFExpression>;
    let mut d: Arc<Expression::NFExpression>;
    let mut x: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut P: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut C: Arc<metamodelica::List<metamodelica::Real>>;
    let mut D: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let mut X: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    p = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    c = __pa7.clone();
    d = __pa8.clone();
    x = __pa9.clone();
    work = __pa10.clone();
    lwork = __pa11.clone();
    info = __pa12.clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    P = evaluateExtIntArg(p)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    C = evaluateExtRealArrayArg(c.clone())?;
    D = evaluateExtRealArrayArg(d.clone())?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, B, C, D, X, WORK, INFO) = Lapack::dgglse(M, N, P, A, LDA, B, LDB, C, D, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(c, Expression::makeRealArray(C)?)?;
    assignVariable(d, Expression::makeRealArray(D)?)?;
    assignVariable(x, Expression::makeRealArray(X)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgtsv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut dl: Arc<Expression::NFExpression>;
    let mut d: Arc<Expression::NFExpression>;
    let mut du: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut DL: Arc<metamodelica::List<metamodelica::Real>>;
    let mut D: Arc<metamodelica::List<metamodelica::Real>>;
    let mut DU: Arc<metamodelica::List<metamodelica::Real>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    nrhs = __pa1.clone();
    dl = __pa2.clone();
    d = __pa3.clone();
    du = __pa4.clone();
    b = __pa5.clone();
    ldb = __pa6.clone();
    info = __pa7.clone();
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    DL = evaluateExtRealArrayArg(dl.clone())?;
    D = evaluateExtRealArrayArg(d.clone())?;
    DU = evaluateExtRealArrayArg(du.clone())?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (DL, D, DU, B, INFO) = Lapack::dgtsv(N, NRHS, DL, D, DU, B, LDB);
    assignVariable(dl, Expression::makeRealArray(DL)?)?;
    assignVariable(d, Expression::makeRealArray(D)?)?;
    assignVariable(du, Expression::makeRealArray(DU)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgbsv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression>;
    let mut kl: Arc<Expression::NFExpression>;
    let mut ku: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut ab: Arc<Expression::NFExpression>;
    let mut ldab: Arc<Expression::NFExpression>;
    let mut ipiv: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut N: i32;
    let mut KL: i32;
    let mut KU: i32;
    let mut NRHS: i32;
    let mut LDAB: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut AB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut IPIV: Arc<metamodelica::List<i32>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    kl = __pa1.clone();
    ku = __pa2.clone();
    nrhs = __pa3.clone();
    ab = __pa4.clone();
    ldab = __pa5.clone();
    ipiv = __pa6.clone();
    b = __pa7.clone();
    ldb = __pa8.clone();
    info = __pa9.clone();
    N = evaluateExtIntArg(n)?;
    KL = evaluateExtIntArg(kl)?;
    KU = evaluateExtIntArg(ku)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    AB = evaluateExtRealMatrixArg(ab.clone())?;
    LDAB = evaluateExtIntArg(ldab)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (AB, IPIV, B, INFO) = Lapack::dgbsv(N, KL, KU, NRHS, AB, LDAB, B, LDB);
    assignVariableExt(ab, Expression::makeRealMatrix(AB)?)?;
    assignVariable(ipiv, Expression::makeIntegerArray(IPIV)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgesvd(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut jobu: Arc<Expression::NFExpression>;
    let mut jobvt: Arc<Expression::NFExpression>;
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut s: Arc<Expression::NFExpression>;
    let mut u: Arc<Expression::NFExpression>;
    let mut ldu: Arc<Expression::NFExpression>;
    let mut vt: Arc<Expression::NFExpression>;
    let mut ldvt: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut JOBU: ArcStr;
    let mut JOBVT: ArcStr;
    let mut M: i32;
    let mut N: i32;
    let mut LDA: i32;
    let mut LDU: i32;
    let mut LDVT: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut U: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut VT: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut S: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => bail!("pattern mismatch"),
    } };
    jobu = __pa0.clone();
    jobvt = __pa1.clone();
    m = __pa2.clone();
    n = __pa3.clone();
    a = __pa4.clone();
    lda = __pa5.clone();
    s = __pa6.clone();
    u = __pa7.clone();
    ldu = __pa8.clone();
    vt = __pa9.clone();
    ldvt = __pa10.clone();
    work = __pa11.clone();
    lwork = __pa12.clone();
    info = __pa13.clone();
    JOBU = (evaluateExtStringArg(jobu)?).clone();
    JOBVT = (evaluateExtStringArg(jobvt)?).clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    LDU = evaluateExtIntArg(ldu)?;
    LDVT = evaluateExtIntArg(ldvt)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, S, U, VT, WORK, INFO) = Lapack::dgesvd((JOBU).clone(), (JOBVT).clone(), M, N, A, LDA, LDU, LDVT, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(s, Expression::makeRealArray(S)?)?;
    assignVariableExt(u, Expression::makeRealMatrix(U)?)?;
    assignVariableExt(vt, Expression::makeRealMatrix(VT)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgetrf(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut ipiv: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut LDA: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut IPIV: Arc<metamodelica::List<i32>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    a = __pa2.clone();
    lda = __pa3.clone();
    ipiv = __pa4.clone();
    info = __pa5.clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    (A, IPIV, INFO) = Lapack::dgetrf(M, N, A, LDA);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(ipiv, Expression::makeIntegerArray(IPIV)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgetrs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut trans: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut nrhs: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut ipiv: Arc<Expression::NFExpression>;
    let mut b: Arc<Expression::NFExpression>;
    let mut ldb: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut TRANS: ArcStr;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut IPIV: Arc<metamodelica::List<i32>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    trans = __pa0.clone();
    n = __pa1.clone();
    nrhs = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    ipiv = __pa5.clone();
    b = __pa6.clone();
    ldb = __pa7.clone();
    info = __pa8.clone();
    TRANS = (evaluateExtStringArg(trans)?).clone();
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a)?;
    LDA = evaluateExtIntArg(lda)?;
    IPIV = evaluateExtIntArrayArg(ipiv)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (B, INFO) = Lapack::dgetrs((TRANS).clone(), N, NRHS, A, LDA, IPIV, B, LDB);
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgetri(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut n: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut ipiv: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut N: i32;
    let mut LDA: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut IPIV: Arc<metamodelica::List<i32>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    a = __pa1.clone();
    lda = __pa2.clone();
    ipiv = __pa3.clone();
    work = __pa4.clone();
    lwork = __pa5.clone();
    info = __pa6.clone();
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    IPIV = evaluateExtIntArrayArg(ipiv)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, WORK, INFO) = Lapack::dgetri(N, A, LDA, IPIV, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dgeqpf(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut jpvt: Arc<Expression::NFExpression>;
    let mut tau: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut LDA: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut JPVT: Arc<metamodelica::List<i32>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let mut TAU: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    a = __pa2.clone();
    lda = __pa3.clone();
    jpvt = __pa4.clone();
    tau = __pa5.clone();
    work = __pa6.clone();
    info = __pa7.clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    WORK = evaluateExtRealArrayArg(work)?;
    (A, JPVT, TAU, INFO) = Lapack::dgeqpf(M, N, A, LDA, JPVT, WORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(jpvt, Expression::makeIntegerArray(JPVT)?)?;
    assignVariable(tau, Expression::makeRealArray(TAU)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dorgqr(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut m: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut k: Arc<Expression::NFExpression>;
    let mut a: Arc<Expression::NFExpression>;
    let mut lda: Arc<Expression::NFExpression>;
    let mut tau: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut K: i32;
    let mut LDA: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut TAU: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    n = __pa1.clone();
    k = __pa2.clone();
    a = __pa3.clone();
    lda = __pa4.clone();
    tau = __pa5.clone();
    work = __pa6.clone();
    lwork = __pa7.clone();
    info = __pa8.clone();
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    K = evaluateExtIntArg(k)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    TAU = evaluateExtRealArrayArg(tau)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, WORK, INFO) = Lapack::dorgqr(M, N, K, A, LDA, TAU, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

pub(crate) fn Lapack_dhseqr(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut job: Arc<Expression::NFExpression>;
    let mut compz: Arc<Expression::NFExpression>;
    let mut n: Arc<Expression::NFExpression>;
    let mut ilo: Arc<Expression::NFExpression>;
    let mut ihi: Arc<Expression::NFExpression>;
    let mut h: Arc<Expression::NFExpression>;
    let mut ldh: Arc<Expression::NFExpression>;
    let mut wr: Arc<Expression::NFExpression>;
    let mut wi: Arc<Expression::NFExpression>;
    let mut z: Arc<Expression::NFExpression>;
    let mut ldz: Arc<Expression::NFExpression>;
    let mut work: Arc<Expression::NFExpression>;
    let mut lwork: Arc<Expression::NFExpression>;
    let mut info: Arc<Expression::NFExpression>;
    let mut N: i32;
    let mut ILO: i32;
    let mut IHI: i32;
    let mut LDH: i32;
    let mut LDZ: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut JOB: ArcStr;
    let mut COMPZ: ArcStr;
    let mut H: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut Z: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut WR: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WI: Arc<metamodelica::List<metamodelica::Real>>;
    let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Cons { head: __pa10, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Cons { head: __pa13, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => bail!("pattern mismatch"),
    } };
    job = __pa0.clone();
    compz = __pa1.clone();
    n = __pa2.clone();
    ilo = __pa3.clone();
    ihi = __pa4.clone();
    h = __pa5.clone();
    ldh = __pa6.clone();
    wr = __pa7.clone();
    wi = __pa8.clone();
    z = __pa9.clone();
    ldz = __pa10.clone();
    work = __pa11.clone();
    lwork = __pa12.clone();
    info = __pa13.clone();
    JOB = (evaluateExtStringArg(job)?).clone();
    COMPZ = (evaluateExtStringArg(compz)?).clone();
    N = evaluateExtIntArg(n)?;
    ILO = evaluateExtIntArg(ilo)?;
    IHI = evaluateExtIntArg(ihi)?;
    H = evaluateExtRealMatrixArg(h.clone())?;
    LDH = evaluateExtIntArg(ldh)?;
    Z = evaluateExtRealMatrixArg(z.clone())?;
    LDZ = evaluateExtIntArg(ldz)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (H, WR, WI, Z, WORK, INFO) = Lapack::dhseqr((JOB).clone(), (COMPZ).clone(), N, ILO, IHI, H, LDH, Z, LDZ, WORK, LWORK);
    assignVariableExt(h, Expression::makeRealMatrix(H)?)?;
    assignVariable(wr, Expression::makeRealArray(WR)?)?;
    assignVariable(wi, Expression::makeRealArray(WI)?)?;
    assignVariableExt(z, Expression::makeRealMatrix(Z)?)?;
    assignVariable(work, Expression::makeRealArray(WORK)?)?;
    assignVariable(info, Expression::makeInteger(INFO))?;
    Ok(())
}

fn evaluateExtIntArg(mut arg: Arc<Expression::NFExpression>) -> Result<i32> {
    let mut value: i32 = getExtIntValue(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?)?;
    Ok(value)
}

fn getExtIntValue(mut exp: Arc<Expression::NFExpression>) -> Result<i32> {
    let mut value: i32;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(),
        Deref @ Expression::EMPTY { .. } => 0,
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

fn evaluateExtRealArg(mut arg: Arc<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real = getExtRealValue(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?)?;
    Ok(value)
}

fn getExtRealValue(mut exp: Arc<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => var_field!((*exp).value, Expression::NFExpression::REAL).clone(),
        Deref @ Expression::EMPTY { .. } => metamodelica::OrderedFloat(0.0_f64),
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

fn evaluateExtStringArg(mut arg: Arc<Expression::NFExpression>) -> Result<ArcStr> {
    let mut value: ArcStr = getExtStringValue(Ceval::evalExp(arg.clone(), Ceval::noTarget().clone())?)?;
    Ok(value)
}

fn getExtStringValue(mut exp: Arc<Expression::NFExpression>) -> Result<ArcStr> {
    let mut value: ArcStr;
    value = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::STRING { .. } => var_field!((*exp).value, Expression::NFExpression::STRING).clone(),
        Deref @ Expression::EMPTY { .. } => literal!(""),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(value)
}

fn evaluateExtIntArrayArg(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut value: Arc<metamodelica::List<i32>>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    expl = Expression::arrayElementList(Ceval::evalExp(arg, Ceval::noTarget().clone())?)?;
    value = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (expl).into_iter().cloned() {
            let __x = getExtIntValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(value)
}

fn evaluateExtRealArrayArg(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<metamodelica::Real>>> {
    let mut value: Arc<metamodelica::List<metamodelica::Real>>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    expl = Expression::arrayElementList(Ceval::evalExp(arg, Ceval::noTarget().clone())?)?;
    value = ({
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut e in (expl).into_iter().cloned() {
            let __x = getExtRealValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(value)
}

fn evaluateExtRealMatrixArg(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>> {
    let mut value: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::evalExp(arg, Ceval::noTarget().clone())?) {
        Deref @ Expression::ARRAY { ty: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    expl = __pa1.clone();
    value = (match Type::dimensionCount(ty) {
        1 => ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
        for mut e in (expl.clone()).borrow().iter() {
            let __x = list![getExtRealValue(e.clone())?];
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        2 => ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
        for mut row in (expl.clone()).borrow().iter() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut e in (Expression::arrayElements(row.clone())?).borrow().iter() {
            let __x = getExtRealValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => bail!("match: no arm matched"),
    });
    Ok(value)
}

fn assignVariableExt(mut variable: Arc<Expression::NFExpression>, mut value: Arc<Expression::NFExpression>) -> Result<()> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((Expression::typeOf(variable.clone()), value.clone())) {
        (Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }) => Expression::makeArray(Type::unliftArray(var_field!((*value).ty, Expression::NFExpression::ARRAY).clone())?, metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*value).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::arrayScalarElement(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()), true),
        _ => value,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assignVariable(variable, exp)?;
    Ok(())
}

