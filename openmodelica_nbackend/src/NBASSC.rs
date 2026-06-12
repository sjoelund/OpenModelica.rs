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

use crate::NBEquation::EquationPointers;
use crate::NBVariable::VariablePointers;

pub(crate) fn main(mut eqns: Arc<EquationPointers::EquationPointers>, mut vars: Arc<VariablePointers::VariablePointers>) -> () {
    let mut indices: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut values: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    indices = arrayCreate(3, metamodelica::nil());
    values = arrayCreate(3, metamodelica::nil());
    {
        let __cell0 = list![1, 2];
        let __idx0 = 1;
        indices.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
    }
    {
        let __cell1 = list![10, 2];
        let __idx1 = 1;
        values.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
    }
    {
        let __cell2 = list![1];
        let __idx2 = 2;
        indices.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
    }
    {
        let __cell3 = list![5];
        let __idx3 = 2;
        values.clone().borrow_mut()[(__idx3-1) as usize] = __cell3;
    }
    {
        let __cell4 = list![1, 3];
        let __idx4 = 3;
        indices.clone().borrow_mut()[(__idx4-1) as usize] = __cell4;
    }
    {
        let __cell5 = list![8, -2];
        let __idx5 = 3;
        values.clone().borrow_mut()[(__idx5-1) as usize] = __cell5;
    }
    setMatrix(3, 3, 5, indices.clone(), values.clone());
    freeMatrix();
    ()
}

pub(crate) fn setMatrix(mut nv: i32, mut ne: i32, mut nz: i32, mut adj: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut val: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    crate::NBASSCExt::ASSC_setMatrix(nv.clone(), ne.clone(), nz.clone(), adj.clone(), val.clone());
    ()
}

pub(crate) fn freeMatrix() -> () {
    crate::NBASSCExt::ASSC_freeMatrix();
    ()
}

pub(crate) fn printMatrix() -> () {
    crate::NBASSCExt::ASSC_printMatrix();
    ()
}

