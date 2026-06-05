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

// ── helpers shared by the LAPACK wrappers ───────────────────────────────────
// MetaModelica matrices are row lists (`List<List<Real>>`); LAPACK operates on
// column-major arrays with leading dimension `lda`, exactly as the C runtime's
// `alloc_real_matrix` / `mk_rml_real_matrix` (lapackimpl.c) convert them
// (`a[j*lda + i]` holds row `i`, column `j`).

type RealMat = Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;

fn mat_to_colmajor(rows: &RealMat, lda: usize, ncol: usize) -> Vec<f64> {
    let mut a = vec![0.0f64; lda * ncol];
    for (i, row) in (&**rows).into_iter().enumerate() {
        if i >= lda { break; }
        for (j, v) in (&**row).into_iter().enumerate() {
            if j >= ncol { break; }
            a[j * lda + i] = v.0;
        }
    }
    a
}

fn colmajor_to_mat(a: &[f64], lda: usize, ncol: usize) -> RealMat {
    let mut res = metamodelica::nil();
    for i in (0..lda).rev() {
        let mut row = metamodelica::nil();
        for j in (0..ncol).rev() {
            row = metamodelica::cons(metamodelica::OrderedFloat(a[j * lda + i]), row);
        }
        res = metamodelica::cons(row, res);
    }
    res
}

fn intlist_to_vec(l: &Arc<metamodelica::List<i32>>) -> Vec<i32> {
    (&**l).into_iter().copied().collect()
}

fn vec_to_intlist(v: &[i32]) -> Arc<metamodelica::List<i32>> {
    let mut res = metamodelica::nil();
    for &x in v.iter().rev() { res = metamodelica::cons(x, res); }
    res
}

fn vec_to_reallist(v: &[f64]) -> Arc<metamodelica::List<metamodelica::Real>> {
    let mut res = metamodelica::nil();
    for &x in v.iter().rev() { res = metamodelica::cons(metamodelica::OrderedFloat(x), res); }
    res
}

/// Reference LAPACK `dgetf2`: partial-pivot LU of an m×n column-major matrix
/// (leading dimension `lda`). Overwrites `a` with L\U; fills 1-based `ipiv`
/// (length min(m,n)); returns INFO (k>0 ⇒ U(k,k)==0).
fn lu_dgetf2(m: usize, n: usize, lda: usize, a: &mut [f64], ipiv: &mut [i32]) -> i32 {
    let mut info = 0i32;
    let mn = m.min(n);
    for j in 0..mn {
        // Pivot: row of largest magnitude in column j, rows j..m.
        let mut jp = j;
        let mut amax = a[j + j * lda].abs();
        for i in (j + 1)..m {
            let v = a[i + j * lda].abs();
            if v > amax { amax = v; jp = i; }
        }
        ipiv[j] = (jp + 1) as i32;
        if a[jp + j * lda] != 0.0 {
            if jp != j {
                for col in 0..n { a.swap(j + col * lda, jp + col * lda); }
            }
            if j + 1 < m {
                let piv = a[j + j * lda];
                for i in (j + 1)..m { a[i + j * lda] /= piv; }
            }
        } else if info == 0 {
            info = (j + 1) as i32;
        }
        // Rank-1 update of the trailing submatrix.
        for jj in (j + 1)..n {
            let ajj = a[j + jj * lda];
            if ajj != 0.0 {
                for i in (j + 1)..m { a[i + jj * lda] -= a[i + j * lda] * ajj; }
            }
        }
    }
    info
}

/// Reference LAPACK `dgetri`: inverse of an n×n matrix from its `dgetf2`
/// factorization (`a` = L\U, 1-based `ipiv`). Overwrites `a` with inv(A);
/// returns INFO (k>0 ⇒ U(k,k)==0, singular).
fn lu_inverse_dgetri(n: usize, lda: usize, a: &mut [f64], ipiv: &[i32]) -> i32 {
    // Invert U in place (dtrtri 'Upper','Non-unit').
    for jj in 0..n {
        let d = a[jj + jj * lda];
        if d == 0.0 { return (jj + 1) as i32; }
        let d = 1.0 / d;
        a[jj + jj * lda] = d;
        let ajj = -d;
        // dtrmv: A(0..jj, jj) := U(0..jj, 0..jj) * A(0..jj, jj)
        for k in 0..jj {
            let temp = a[k + jj * lda];
            if temp != 0.0 {
                for i in 0..k { a[i + jj * lda] += temp * a[i + k * lda]; }
                a[k + jj * lda] = temp * a[k + k * lda];
            }
        }
        for i in 0..jj { a[i + jj * lda] *= ajj; }
    }
    // Solve inv(A) * L = inv(U) for inv(A).
    let mut work = vec![0.0f64; n];
    for j in (0..n).rev() {
        for i in (j + 1)..n {
            work[i] = a[i + j * lda];
            a[i + j * lda] = 0.0;
        }
        for jj in (j + 1)..n {
            let wj = work[jj];
            if wj != 0.0 {
                for i in 0..n { a[i + j * lda] -= a[i + jj * lda] * wj; }
            }
        }
    }
    // Apply column interchanges in reverse order.
    if n >= 2 {
        for j in (0..n - 1).rev() {
            let jp = (ipiv[j] - 1) as usize;
            if jp != j {
                for i in 0..n { a.swap(i + j * lda, i + jp * lda); }
            }
        }
    }
    0
}

pub fn dgeev(mut inJOBVL: ArcStr, mut inJOBVR: ArcStr, mut inN: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inLDVL: i32, mut inLDVR: i32, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outWI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outVL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outVR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgeev"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOBVL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOBVR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDVL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDVR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWI", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outVL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outVR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 67, columnNumberStart: 24, lineNumberEnd: 67, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 67, columnNumberStart: 16, lineNumberEnd: 67, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outWR, outWI, outVL, outVR, outWORK, outINFO)
}

pub fn dgegv(mut inJOBVL: ArcStr, mut inJOBVR: ArcStr, mut inN: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32, mut inLDVL: i32, mut inLDVR: i32, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outALPHAR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outALPHAI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outBETA: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outVL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outVR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgegv"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOBVL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOBVR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDVL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDVR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outALPHAR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outALPHAI", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outBETA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outVL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outVR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 91, columnNumberStart: 49, lineNumberEnd: 91, columnNumberEnd: 75, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 91, columnNumberStart: 41, lineNumberEnd: 91, columnNumberEnd: 75, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outALPHAR, outALPHAI, outBETA, outVL, outVR, outWORK, outINFO)
}

pub fn dgels(mut inTRANS: ArcStr, mut inM: i32, mut inN: i32, mut inNRHS: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgels"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inTRANS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 111, columnNumberStart: 24, lineNumberEnd: 111, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 111, columnNumberStart: 16, lineNumberEnd: 111, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outB, outWORK, outINFO)
}

pub fn dgelsx(mut inM: i32, mut inN: i32, mut inNRHS: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32, mut inJPVT: Arc<metamodelica::List<i32>>, mut inRCOND: metamodelica::Real, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<i32>>, i32, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outJPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outRANK: i32 = 0;
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgelsx"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJPVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inRCOND", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outJPVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outRANK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 132, columnNumberStart: 24, lineNumberEnd: 132, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 132, columnNumberStart: 16, lineNumberEnd: 132, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outB, outJPVT, outRANK, outINFO)
}

pub fn dgelsy(mut inM: i32, mut inN: i32, mut inNRHS: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32, mut inJPVT: Arc<metamodelica::List<i32>>, mut inRCOND: metamodelica::Real, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outJPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outRANK: i32 = 0;
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgelsy"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJPVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inRCOND", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outJPVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outRANK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 155, columnNumberStart: 24, lineNumberEnd: 155, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 155, columnNumberStart: 16, lineNumberEnd: 155, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outB, outJPVT, outRANK, outWORK, outINFO)
}

pub fn dgesv(mut inN: i32, mut inNRHS: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outIPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgesv"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outIPIV", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 170, columnNumberStart: 54, lineNumberEnd: 170, columnNumberEnd: 80, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 170, columnNumberStart: 46, lineNumberEnd: 170, columnNumberEnd: 80, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outIPIV, outB, outINFO)
}

pub fn dgglse(mut inM: i32, mut inN: i32, mut inP: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32, mut inC: Arc<metamodelica::List<metamodelica::Real>>, mut inD: Arc<metamodelica::List<metamodelica::Real>>, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outC: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outD: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outX: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgglse"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inP", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inC", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inD", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outC", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outD", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outX", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 194, columnNumberStart: 24, lineNumberEnd: 194, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 194, columnNumberStart: 16, lineNumberEnd: 194, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outB, outC, outD, outX, outWORK, outINFO)
}

pub fn dgtsv(mut inN: i32, mut inNRHS: i32, mut inDL: Arc<metamodelica::List<metamodelica::Real>>, mut inD: Arc<metamodelica::List<metamodelica::Real>>, mut inDU: Arc<metamodelica::List<metamodelica::Real>>, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32) -> (Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, i32) {
    let mut outDL: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outD: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outDU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgtsv"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inDL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inD", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inDU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outDL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outD", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outDU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 212, columnNumberStart: 24, lineNumberEnd: 212, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 212, columnNumberStart: 16, lineNumberEnd: 212, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outDL, outD, outDU, outB, outINFO)
}

pub fn dgbsv(mut inN: i32, mut inKL: i32, mut inKU: i32, mut inNRHS: i32, mut inAB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDAB: i32, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, i32) {
    let mut outAB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outIPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgbsv"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inKL", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inKU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inAB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDAB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outAB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outIPIV", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 230, columnNumberStart: 24, lineNumberEnd: 230, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 230, columnNumberStart: 16, lineNumberEnd: 230, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outAB, outIPIV, outB, outINFO)
}

pub fn dgesvd(mut inJOBU: ArcStr, mut inJOBVT: ArcStr, mut inM: i32, mut inN: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inLDU: i32, mut inLDVT: i32, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outS: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outU: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outVT: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgesvd"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOBU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOBVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 252, columnNumberStart: 24, lineNumberEnd: 252, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 252, columnNumberStart: 16, lineNumberEnd: 252, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outS, outU, outVT, outWORK, outINFO)
}

pub fn dgetrf(mut inM: i32, mut inN: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<i32>>, i32) {
    // Mirror LapackImpl__dgetrf (lapackimpl.c): partial-pivot LU.
    let m = inM.max(0) as usize;
    let n = inN.max(0) as usize;
    let lda = inLDA.max(1) as usize;
    let mut a = mat_to_colmajor(&inA, lda, n);
    let mut ipiv = vec![0i32; m.min(n)];
    let outINFO = lu_dgetf2(m, n, lda, &mut a, &mut ipiv);
    let outA = colmajor_to_mat(&a, lda, n);
    let outIPIV = vec_to_intlist(&ipiv);
    (outA, outIPIV, outINFO)
}

pub fn dgetrs(mut inTRANS: ArcStr, mut inN: i32, mut inNRHS: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inIPIV: Arc<metamodelica::List<i32>>, mut inB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDB: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, i32) {
    let mut outB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgetrs"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inTRANS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inNRHS", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inIPIV", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 280, columnNumberStart: 24, lineNumberEnd: 280, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 280, columnNumberStart: 16, lineNumberEnd: 280, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outB, outINFO)
}

pub fn dgetri(mut inN: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inIPIV: Arc<metamodelica::List<i32>>, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    // Mirror LapackImpl__dgetri (lapackimpl.c): inverse from the LU factors.
    let n = inN.max(0) as usize;
    let lda = inLDA.max(1) as usize;
    let lwork = inLWORK.max(0) as usize;
    let mut a = mat_to_colmajor(&inA, lda, n);
    let ipiv = intlist_to_vec(&inIPIV);
    let outINFO = lu_inverse_dgetri(n, lda, &mut a, &ipiv);
    let outA = colmajor_to_mat(&a, lda, n);
    // The work array's contents are unused by callers; return a zeroed buffer
    // of the requested length (LAPACK would leave the optimal lwork in WORK(1)).
    let outWORK = vec_to_reallist(&vec![0.0f64; lwork]);
    (outA, outWORK, outINFO)
}

pub fn dgeqpf(mut inM: i32, mut inN: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inJPVT: Arc<metamodelica::List<i32>>, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outJPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outTAU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dgeqpf"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJPVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outJPVT", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outTAU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 309, columnNumberStart: 50, lineNumberEnd: 309, columnNumberEnd: 76, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 309, columnNumberStart: 42, lineNumberEnd: 309, columnNumberEnd: 76, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outJPVT, outTAU, outINFO)
}

pub fn dorgqr(mut inM: i32, mut inN: i32, mut inK: i32, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDA: i32, mut inTAU: Arc<metamodelica::List<metamodelica::Real>>, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dorgqr"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inM", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inTAU", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outA", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 325, columnNumberStart: 48, lineNumberEnd: 325, columnNumberEnd: 74, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 325, columnNumberStart: 40, lineNumberEnd: 325, columnNumberEnd: 74, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outA, outWORK, outINFO)
}

pub fn dhseqr(mut inJOB: ArcStr, mut inCOMPZ: ArcStr, mut inN: i32, mut inILO: i32, mut inIHI: i32, mut inH: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDH: i32, mut inZ: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut inLDZ: i32, mut inWORK: Arc<metamodelica::List<metamodelica::Real>>, mut inLWORK: i32) -> (Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, Arc<metamodelica::List<metamodelica::Real>>, i32) {
    let mut outH: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outWI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outZ: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outWORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outINFO: i32 = 0;
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("LapackImpl__dhseqr"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "inJOB", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inCOMPZ", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inN", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inILO", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inIHI", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inH", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDH", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inZ", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLDZ", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "inLWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outH", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWR", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWI", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outZ", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outWORK", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "outINFO", subscripts: Nil } }, tail: Nil } } } } } } } } } } } } } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Library" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: ARRAY { arrayExp: Cons { head: STRING { value: "omcruntime" }, tail: Cons { head: STRING { value: "Lapack" }, tail: Nil } } }, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 348, columnNumberStart: 24, lineNumberEnd: 348, columnNumberEnd: 50, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/projects/OpenModelica/OMCompiler/Compiler/Util/Lapack.mo", isReadOnly: false, lineNumberStart: 348, columnNumberStart: 16, lineNumberEnd: 348, columnNumberEnd: 50, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    (outH, outWR, outWI, outZ, outWORK, outINFO)
}

