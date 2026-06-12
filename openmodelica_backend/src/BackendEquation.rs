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

use crate::AvlSetInt;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendVariable;
use crate::ExpressionSolve;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn emptyEqns() -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = emptyEqnsSized(0);
    equationArray
}

pub(crate) fn emptyEqnsSized(mut size: i32) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ExpandableArray::new(size, openmodelica_backend_types::BackendDAE::Equation::interned_DUMMY_EQUATION());
    outEquationArray
}

pub(crate) fn add(mut inEquation: Arc<BackendDAE::Equation>, mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    ExpandableArray::add(inEquation, equationArray.clone())?;
    Ok(equationArray)
}

pub(crate) fn addList(mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    ExpandableArray::expandToSize(ExpandableArray::getLastUsedIndex(equationArray.clone()) + (eqnlst.clone().len() as i32), equationArray.clone())?;
    for mut e in &*eqnlst {
        let mut e = e.clone();
        equationArray = add(e.clone(), equationArray.clone())?;
    }
    Ok(equationArray)
}

pub(crate) fn delete(mut inPos: i32, mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    ExpandableArray::delete(inPos, equationArray.clone())?;
    Ok(equationArray)
}

pub(crate) fn deleteList(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndices: Arc<metamodelica::List<i32>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    for mut index in &*inIndices {
        let mut index = index.clone();
        ExpandableArray::delete(index.clone(), equationArray.clone())?;
    }
    Ok(equationArray)
}

pub fn merge(mut inEqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inEqns2: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    outEqns = copyEquationArray(inEqns2);
    outEqns = addList(equationList(inEqns1)?, outEqns)?;
    Ok(outEqns)
}

pub fn listEquation(mut inEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    outEquationArray = ExpandableArray::new((inEquationList.clone().len() as i32), openmodelica_backend_types::BackendDAE::Equation::interned_DUMMY_EQUATION());
    for mut eq in &*inEquationList {
        let mut eq = eq.clone();
        ExpandableArray::add(eq.clone(), outEquationArray.clone())?;
    }
    Ok(outEquationArray)
}

pub fn equationList(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = ExpandableArray::toList(equationArray.clone())?;
    Ok(outEquationLst)
}

pub(crate) fn copyEquationArray(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ExpandableArray::copy(inEquationArray.clone(), openmodelica_backend_types::BackendDAE::Equation::interned_DUMMY_EQUATION());
    outEquationArray
}

pub(crate) fn setAtIndex(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPos: i32, mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    if ExpandableArray::occupied(inPos, equationArray.clone()) {
        ExpandableArray::update(inPos, inEquation, equationArray.clone())?;
    } else {
        ExpandableArray::set(inPos, inEquation, equationArray.clone())?;
    }
    Ok(equationArray)
}

pub(crate) fn setAtIndexFirst(mut inPos: i32, mut inEquation: Arc<BackendDAE::Equation>, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = setAtIndex(inEquationArray.clone(), inPos, inEquation.clone())?;
    Ok(outEquationArray)
}

pub fn get(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPos: i32) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEquation: Arc<BackendDAE::Equation> = ExpandableArray::get(inPos, inEquationArray.clone())?;
    Ok(outEquation)
}

pub(crate) fn has(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPos: i32) -> bool {
    let mut b: bool = ExpandableArray::occupied(inPos, inEquationArray.clone());
    b
}

pub(crate) fn getList(mut inIndices: Arc<metamodelica::List<i32>>, mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut index in (inIndices).into_iter().cloned() {
            let __x = get(inEquationArray.clone(), index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEqns)
}

pub fn equationArraySize(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut outSize: i32;
    let mut nfScalarize: bool = Flags::isSet(Flags::NF_SCALARIZE.clone())?;
    outSize = 0;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            if nfScalarize {
                outSize = outSize + equationSize(ExpandableArray::get(i.clone(), equationArray.clone())?)?;
            } else {
                outSize = outSize + 1;
            }
        }
    }
    Ok(outSize)
}

pub(crate) fn getNumberOfEquations(mut inEquationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> i32 {
    let mut outSize: i32 = ExpandableArray::getNumberOfElements(inEquationArray.clone());
    outSize
}

pub fn traverseEquationArray<T: Clone + 'static + metamodelica::gc::MMTrace>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>, mut extraArg: T) -> Result<T> {
    pub type Func<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>;

    let mut extraArg: T = extraArg;
    let mut eqn: Arc<BackendDAE::Equation>;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            eqn = ExpandableArray::get(i.clone(), equationArray.clone())?;
            (_, extraArg) = inFunc(eqn.clone(), extraArg.clone())?;
        }
    }
    Ok(extraArg)
}

pub(crate) fn traverseEquationArray_WithStop<T: Clone + 'static + metamodelica::gc::MMTrace>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFuncWithStop: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, bool, T)> + 'static>, mut extraArg: T) -> Result<T> {
    pub type FuncWithStop<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, bool, T)> + 'static>;

    let mut extraArg: T = extraArg;
    let mut continue_: bool;
    let mut eqn: Arc<BackendDAE::Equation>;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            eqn = ExpandableArray::get(i.clone(), equationArray.clone())?;
            (_, continue_, extraArg) = inFuncWithStop(eqn.clone(), extraArg.clone())?;
            if !(continue_) {
                break;
            }
        }
    }
    Ok(extraArg)
}

pub(crate) fn traverseEquationArray_WithUpdate<T: Clone + 'static + metamodelica::gc::MMTrace>(mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inFuncWithUpdate: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>, mut extraArg: T) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, T)> {
    pub type FuncWithUpdate<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, T) -> Result<(Arc<BackendDAE::Equation>, T)> + 'static>;

    let mut equationArray: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = equationArray;
    let mut extraArg: T = extraArg;
    let mut e: Arc<BackendDAE::Equation>;
    let mut new_e: Arc<BackendDAE::Equation>;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(equationArray.clone()) {
        if ExpandableArray::occupied(i.clone(), equationArray.clone()) {
            e = ExpandableArray::get(i.clone(), equationArray.clone())?;
            (new_e, extraArg) = inFuncWithUpdate(e.clone(), extraArg.clone())?;
            if !(referenceEq(&*(e.clone()),&*(new_e.clone()))) {
                ExpandableArray::update(i.clone(), new_e.clone(), equationArray.clone())?;
            }
        }
    }
    Ok((equationArray, extraArg))
}

pub(crate) fn sortInitialEqns(mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = eqns;
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut init_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut sim_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    eqn_lst = equationList(eqns)?;
    (init_eqns, sim_eqns) = List::splitOnTrue(eqn_lst, (std::sync::Arc::new(isInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>))?;
    eqn_lst = listAppend(init_eqns, sim_eqns);
    eqns = listEquation(eqn_lst)?;
    Ok(eqns)
}

pub(crate) fn getForEquationIterIdent(mut inEquation: Arc<BackendDAE::Equation>) -> Option<ArcStr> {
    let mut forIter: Option<ArcStr>;
    forIter = (::match_deref::match_deref! { match &(inEquation) {
        Deref @ BackendDAE::Equation::FOR_EQUATION { iter: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: iter, .. }, .. }, .. } => {
            Some((iter.clone()).clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    forIter
}

pub fn getWhenEquationExpr(mut inWhenEquation: Arc<BackendDAE::WhenEquation>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    let mut outExp: Arc<DAE::Exp>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inWhenEquation.clone()) {
            Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: __pa1, .. }, right: __pa2, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outComponentRef = __pa1.clone();
        outExp = __pa2.clone();
        Ok::<_, anyhow::Error>((outComponentRef.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outComponentRef = __try0_o0;
            outExp = __try0_o1;
        }
        Err(__try0_err) => {
            Error::addInternalError((literal!("BackendEquation.getWhenEquationExpr failed\n")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            return Err(__try0_err);
        }
    }
    Ok((outComponentRef, outExp))
}

pub(crate) fn setWhenElsePart(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inElseWhenEquation: Arc<BackendDAE::WhenEquation>) -> Result<Arc<BackendDAE::WhenEquation>> {
    let mut outWhenEquation: Arc<BackendDAE::WhenEquation>;
    outWhenEquation = (::match_deref::match_deref! { match &(inWhenEquation) {
        Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: None } => {
            Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: Some(inElseWhenEquation) })
        },
        Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: Some(elsewhenPart) } => {
            Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: Some(setWhenElsePart(elsewhenPart.clone(), inElseWhenEquation)?) })
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outWhenEquation)
}

pub(crate) fn equationsLstVars(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut indexes: Arc<AvlSetInt::Tree>;
    let mut keys: Arc<metamodelica::List<i32>>;
    if inEquationLst.clone().is_empty() {
        outVars = metamodelica::nil();
        return Ok(outVars.clone());
    }
    (_, indexes) = traverseExpsOfEquationList(inEquationLst, (std::sync::Arc::new({ let __pe_b2 = inVars.clone(); move |__pe_a0, __pe_a1| checkEquationsVarsExpTopDownTraverseHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>)> + 'static>), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes, metamodelica::nil());
    outVars = List::map1r(keys, (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVars)?;
    Ok(outVars)
}

pub fn equationsVars(mut inEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut indexes: Arc<AvlSetInt::Tree>;
    let mut keys: Arc<metamodelica::List<i32>>;
    indexes = BackendDAEUtil::traverseBackendDAEExpsEqns(inEquations, (std::sync::Arc::new({ let __pe_b2 = inVars.clone(); move |__pe_a0, __pe_a1| checkEquationsVarsExpTopDownTraverseHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>)> + 'static>), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes, metamodelica::nil());
    outVars = List::map1r(keys, (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVars)?;
    Ok(outVars)
}

pub(crate) fn equationVars(mut inEquation: Arc<BackendDAE::Equation>, mut inVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut indexes: Arc<AvlSetInt::Tree>;
    let mut keys: Arc<metamodelica::List<i32>>;
    (_, indexes) = traverseExpsOfEquation(inEquation, (std::sync::Arc::new({ let __pe_b2 = inVars.clone(); move |__pe_a0, __pe_a1| checkEquationsVarsExpTopDownTraverseHelper(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>)> + 'static>), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes, metamodelica::nil());
    outVars = List::map1r(keys, (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inVars)?;
    Ok(outVars)
}

pub(crate) fn expressionVars(mut inExp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut indexes: Arc<AvlSetInt::Tree>;
    let mut keys: Arc<metamodelica::List<i32>>;
    (_, indexes) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new({ let __pe_b2 = vars.clone(); move |__pe_a0, __pe_a1| Ok(checkEquationsVarsExpTopDown(__pe_a0, __pe_a1, __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>), AvlSetInt::new())?;
    keys = AvlSetInt::listKeys(indexes, metamodelica::nil());
    outVars = List::map1r(keys, (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars)?;
    Ok(outVars)
}

pub(crate) fn expressionVarsIndexes(mut exp: Arc<DAE::Exp>, mut indexes: Arc<AvlSetInt::Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>) -> Result<Arc<AvlSetInt::Tree>> {
    pub type CheckEquationsVarsExpTopDownFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>;

    let mut indexes: Arc<AvlSetInt::Tree> = indexes;
    (_, indexes) = Expression::traverseExpTopDown(exp, func.clone(), indexes)?;
    Ok(indexes)
}

pub(crate) fn checkEquationsVarsExpTopDownTraverseHelper(mut exp: Arc<DAE::Exp>, mut tree: Arc<AvlSetInt::Tree>, mut vars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut tree: Arc<AvlSetInt::Tree> = tree;
    (exp, tree) = Expression::traverseExpTopDown(exp, (std::sync::Arc::new({ let __pe_b2 = vars; move |__pe_a0, __pe_a1| Ok(checkEquationsVarsExpTopDown(__pe_a0, __pe_a1, __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlSetInt::Tree>) -> Result<(Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>)> + 'static>), tree)?;
    Ok((exp, tree))
}

pub(crate) fn checkEquationsVarsExpTopDown(mut exp: Arc<DAE::Exp>, mut tree: Arc<AvlSetInt::Tree>, mut vars: BackendDAE::Variables) -> (Arc<DAE::Exp>, bool, Arc<AvlSetInt::Tree>) {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool;
    let mut tree: Arc<AvlSetInt::Tree> = tree;
    (cont, tree) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. } => {
            (true, tree)
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. } => {
            (true, tree)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (idn.clone() == literal!("pre") || idn.clone() == literal!("previous")) => {
            (false, tree)
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut ilst: Arc<metamodelica::List<i32>>;
            if '__try0: {
                (_, ilst) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars.clone()), '__try0);
                tree = unwrap_break_err!(AvlSetInt::addList(tree.clone(), ilst.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            (true, tree)
        },
        _ => {
            (true, tree)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, cont, tree)
}

pub(crate) fn assertWithCondTrue(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond: Deref @ DAE::Exp::BCONST { bool: true }, .. }, tail: Deref @ metamodelica::List::Nil } }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn equationsParams(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut outParamVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outParamVarsIdc: Arc<metamodelica::List<i32>>;
    let (_, (_, (__pa0, __pa1, _))) = traverseExpsOfEquationList(inEquationLst, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingParamRefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), metamodelica::nil(), inVars)))?;
    outParamVars = __pa0.clone();
    outParamVarsIdc = __pa1.clone();
    Ok((outParamVars, outParamVarsIdc))
}

fn traversingParamRefFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, BackendDAE::Variables);
    outExp = inExp.clone();
    outTpl = (::match_deref::match_deref! { match &((inExp, inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
            inTpl
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, varIdc, allVars)) => {
            let mut foundVars: Arc<metamodelica::List<BackendDAE::Var>>;
            let mut foundVarsIdc: Arc<metamodelica::List<i32>>;
            let mut vars = (*vars).clone();
            let mut varIdc = (*varIdc).clone();
            (foundVars, foundVarsIdc) = BackendVariable::getVar(cr.clone(), allVars.clone())?;
            (vars, varIdc) = traversingParamRefFinder0(foundVars, foundVarsIdc, vars.clone(), varIdc.clone())?;
            (vars.clone(), varIdc.clone(), allVars.clone())
        },
        _ => {
            inTpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn traversingParamRefFinder0(mut iVars: Arc<metamodelica::List<BackendDAE::Var>>, mut iVarIdc: Arc<metamodelica::List<i32>>, mut iParamVarsList: Arc<metamodelica::List<BackendDAE::Var>>, mut iParamVarsIdc: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>)> {
    let mut oParamVarsList: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut oParamVarIdc: Arc<metamodelica::List<i32>>;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = iParamVarsList.clone();
    let mut varIdc: Arc<metamodelica::List<i32>> = iParamVarsIdc.clone();
    let mut varIdx: i32;
    let mut rest: Arc<metamodelica::List<i32>> = iVarIdc.clone();
    for mut var in &*iVars {
        let mut var = var.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        varIdx = __pa0.clone();
        rest = __pa1.clone();
        if BackendVariable::isParam(var.clone()) {
            vars = List::unionEltOnTrue(var.clone(), vars.clone(), (std::sync::Arc::new(BackendVariable::varEqual) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var) -> Result<bool> + 'static>))?;
            varIdc = List::unionEltOnTrue(varIdx, varIdc.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        }
    }
    oParamVarsList = vars;
    oParamVarIdc = varIdc;
    Ok((oParamVarsList, oParamVarIdc))
}

pub(crate) fn iterationVarsinRelations(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables) -> Result<(bool, Arc<metamodelica::List<i32>>)> {
    let mut mixedSystem: bool;
    let mut indexes: Arc<metamodelica::List<i32>>;
    let (_, (_, (__pa0, _))) = traverseExpsOfEquationList(inEquationLst, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingRelationsforIterationVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), inVars)))?;
    indexes = __pa0.clone();
    mixedSystem = !(indexes.clone().is_empty());
    Ok((mixedSystem, indexes))
}

fn traversingRelationsforIterationVars(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<i32>>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<i32>>, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<i32>>, BackendDAE::Variables) = (metamodelica::nil(), <BackendDAE::Variables as ::std::default::Default>::default());
    outExp = inExp.clone();
    outTpl = (::match_deref::match_deref! { match &((inExp, inTpl.clone())) {
        (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, index, .. }, (indexes, vars)) => {
            let mut vlst1: Arc<metamodelica::List<BackendDAE::Var>>;
            let mut vlst2: Arc<metamodelica::List<BackendDAE::Var>>;
            vlst1 = expressionVars(e1.clone(), vars.clone())?;
            vlst2 = expressionVars(e2.clone(), vars.clone())?;
            if !(vlst1.is_empty() && vlst2.is_empty()) {
                outTpl = (metamodelica::cons(index.clone(), indexes.clone()), vars.clone());
            } else {
                outTpl = inTpl;
            }
            outTpl
        },
        _ => {
            inTpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

pub(crate) fn equationsCrefs(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let (_, (_, __pa0)) = traverseExpsOfEquationList(inEquationLst, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::traversingComponentRefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil()))?;
    outExpComponentRefLst = __pa0.clone();
    Ok(outExpComponentRefLst)
}

pub(crate) fn equationCrefs(mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let (_, (_, __pa0)) = traverseExpsOfEquation(inEquation, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::traversingComponentRefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil()))?;
    outExpComponentRefLst = __pa0.clone();
    Ok(outExpComponentRefLst)
}

pub(crate) fn equationCrefsSolved(mut inEquation: Arc<BackendDAE::Equation>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut lhs_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut rhs_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    lhs = getEquationLHS(inEquation.clone())?;
    rhs = getEquationRHS(inEquation)?;
    lhs_lst = Expression::extractCrefsFromExp(lhs)?;
    rhs_lst = Expression::extractCrefsFromExp(rhs)?;
    Ok((lhs_lst, rhs_lst))
}

pub fn getAllCrefFromEquations(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    cr_lst = traverseEquationArray(inEqns, (std::sync::Arc::new(traversingEquationCrefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    Ok(cr_lst)
}

fn traversingEquationCrefFinder(mut inEq: Arc<BackendDAE::Equation>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut e: Arc<BackendDAE::Equation>;
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    e = inEq;
    cr_lst = inCrefs;
    let (_, (_, __pa0)) = traverseExpsOfEquation(e.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::traversingComponentRefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), cr_lst))?;
    cr_lst = __pa0.clone();
    Ok((e, cr_lst))
}

pub(crate) fn getCrefsFromEquations(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inKnVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    ht = HashTable::emptyHashTable();
    (_, _, ht) = traverseEquationArray(inEqns, (std::sync::Arc::new(findUnknownCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))> + 'static>), (inVars, inKnVars, ht))?;
    cr_lst = BaseHashTable::hashTableKeyList(ht)?;
    Ok(cr_lst)
}

pub(crate) fn findUnknownCrefs(mut inEq: Arc<BackendDAE::Equation>, mut extraArgs: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))> {
    let mut inEq: Arc<BackendDAE::Equation> = inEq;
    let mut extraArgs: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) = extraArgs;
    let (_, (_, __pa0)) = traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(checkEquationsUnknownCrefsExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))> + 'static>), extraArgs))?;
    extraArgs = __pa0.clone();
    Ok((inEq, extraArgs))
}

pub(crate) fn equationUnknownCrefs(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inKnVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut cr_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    ht = HashTable::emptyHashTable();
    let (_, (_, (_, _, __pa0))) = traverseExpsOfEquationList(inEquationLst, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(checkEquationsUnknownCrefsExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))> + 'static>), (inVars, inKnVars, ht)))?;
    ht = __pa0.clone();
    cr_lst = BaseHashTable::hashTableKeyList(ht)?;
    Ok(cr_lst)
}

fn checkEquationsUnknownCrefsExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> (Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)));
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, _) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)));
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone())?;
                    (_, outTuple) = Expression::traverseExpList(expl.clone(), (std::sync::Arc::new(fnptr!(checkEquationsUnknownCrefsExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))> + 'static>), inTuple.clone())?;
                    Ok((e.clone(), outTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)));
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, outTuple) = Expression::traverseExpBottomUp(e1.clone(), (std::sync::Arc::new(fnptr!(checkEquationsUnknownCrefsExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))))> + 'static>), inTuple.clone())?;
                    Ok((e.clone(), outTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, _, ht)) => {
                    if !((BaseHashTable::hasKey(cr.clone(), ht.clone())?)) { bail!("guard") }
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _, _)) => {
                    BackendVariable::getVar(cr.clone(), vars.clone())?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, knvars, _)) => {
                    BackendVariable::getVar(cr.clone(), knvars.clone())?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, knvars, ht)) => {
                    let mut ht = (*ht).clone();
                    ht = BaseHashTable::add((cr.clone(), 0), ht.clone())?;
                    Ok((inExp.clone(), (vars.clone(), knvars.clone(), ht.clone())))
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

pub fn traverseExpsOfEquationList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outArg: ArgT = inArg.clone();
    for mut eq in &*inEquations {
        let mut eq = eq.clone();
        (eq, outArg) = traverseExpsOfEquation(eq.clone(), func.clone(), outArg.clone())?;
        outEquations = metamodelica::cons(eq.clone(), outEquations.clone());
    }
    outEquations = metamodelica::Dangerous::listReverseInPlace(outEquations);
    Ok((outEquations, outArg))
}

pub(crate) fn traverseExpsOfEquationList_WithStop<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(bool, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outBoolean: bool = true;
    let mut outTypeA: Type_a = inTypeA.clone();
    for mut eqn in &*inEquations {
        let mut eqn = eqn.clone();
        (outBoolean, outTypeA) = traverseExpsOfEquation_WithStop(eqn.clone(), inFunc.clone(), outTypeA.clone())?;
        if !(outBoolean) {
            break;
        }
    }
    Ok((outBoolean, outTypeA))
}

pub(crate) fn traverseExpsOfEquationList_WithoutChange<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inEquation: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outArg: ArgT = inArg.clone();
    (_, outArg) = traverseExpsOfEquation(inEquation, func.clone(), outArg)?;
    Ok(outArg)
}

fn traverseExpsOfEquationListList_WithStop<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outBoolean: bool = true;
    let mut outTypeA: T = inTypeA.clone();
    for mut eqn in &*inEquations {
        let mut eqn = eqn.clone();
        (outBoolean, outTypeA) = traverseExpsOfEquationList_WithStop(eqn.clone(), func.clone(), outTypeA.clone())?;
        if !(outBoolean) {
            break;
        }
    }
    Ok((outBoolean, outTypeA))
}

pub(crate) fn traverseExpsOfEquation<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inEquation: Arc<BackendDAE::Equation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inTypeA: T) -> Result<(Arc<BackendDAE::Equation>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outEquation: Arc<BackendDAE::Equation>;
    let mut outTypeA: T;
    (outEquation, outTypeA) = (::match_deref::match_deref! { match &(inEquation) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).exp, BackendDAE::Equation::EQUATION).clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; exp = e1);
            (e1, extArg) = inFunc(var_field!((*eqn).scalar, BackendDAE::Equation::EQUATION).clone(), extArg)?;
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; scalar = e1);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).left, BackendDAE::Equation::ARRAY_EQUATION).clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; left = e1);
            (e1, extArg) = inFunc(var_field!((*eqn).right, BackendDAE::Equation::ARRAY_EQUATION).clone(), extArg)?;
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; right = e1);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            e1 = Expression::makeCrefExp(cr.clone(), Expression::r#typeof(var_field!((*eqn).exp, BackendDAE::Equation::SOLVED_EQUATION).clone())?)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inFunc(e1, inTypeA)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr1 = __pa0.clone();
            extArg = __pa1.clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; componentRef = cr1);
            (e1, extArg) = inFunc(var_field!((*eqn).exp, BackendDAE::Equation::SOLVED_EQUATION).clone(), extArg)?;
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; exp = e1);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).exp, BackendDAE::Equation::RESIDUAL_EQUATION).clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; exp = e1);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            let mut whenEquation: Arc<BackendDAE::WhenEquation>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (whenEquation, extArg) = traverseExpsOfWhenEquation(var_field!((*eqn).whenEquation, BackendDAE::Equation::WHEN_EQUATION).clone(), inFunc.clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::WHEN_EQUATION; whenEquation = whenEquation);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            let mut stmts = (*stmts).clone();
            (stmts, extArg) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), inFunc.clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::ALGORITHM; alg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() }));
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (e1, extArg) = inFunc(var_field!((*eqn).left, BackendDAE::Equation::COMPLEX_EQUATION).clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; left = e1);
            (e1, extArg) = inFunc(var_field!((*eqn).right, BackendDAE::Equation::COMPLEX_EQUATION).clone(), extArg)?;
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; right = e1);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            let mut extArg: T;
            let mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqnstrue: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
            let mut eqnsfalse: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqn = (*eqn).clone();
            (conditions, extArg) = traverseExpsOfExpList(var_field!((*eqn).conditions, BackendDAE::Equation::IF_EQUATION).clone(), inFunc.clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; conditions = conditions);
            (eqnstrue, extArg) = List::map1Fold(var_field!((*eqn).eqnstrue, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(traverseExpsOfEquationList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, _, _) -> Result<_> + 'static>), inFunc.clone(), extArg)?;
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; eqnstrue = eqnstrue);
            (eqnsfalse, extArg) = List::map1Fold(var_field!((*eqn).eqnsfalse, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(traverseExpsOfEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), extArg)?;
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; eqnsfalse = eqnsfalse);
            (eqn.clone(), extArg)
        },
        eqn @ Deref @ BackendDAE::Equation::FOR_EQUATION { .. } => {
            let mut eqn1: Arc<BackendDAE::Equation>;
            let mut extArg: T;
            let mut eqn = (*eqn).clone();
            (eqn1, extArg) = traverseExpsOfEquation(var_field!((*eqn).body, BackendDAE::Equation::FOR_EQUATION).clone(), inFunc.clone(), inTypeA)?;
            assign_variant_field!(eqn => BackendDAE::Equation::FOR_EQUATION; body = eqn1);
            (eqn.clone(), extArg)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEquation, outTypeA))
}

pub(crate) fn traverseExpsOfEquation_WithStop<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inEquation: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outBoolean: bool;
    let mut outTypeA: T;
    (outBoolean, outTypeA) = (::match_deref::match_deref! { match &(inEquation) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut ext_arg: T;
            let mut b: bool;
            (_, b, ext_arg) = func(e1.clone(), inTypeA)?;
            if b {
                (_, b, ext_arg) = func(e2.clone(), ext_arg)?;
            }
            (b, ext_arg)
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            let mut ext_arg: T;
            let mut b: bool;
            (_, b, ext_arg) = func(e1.clone(), inTypeA)?;
            if b {
                (_, b, ext_arg) = func(e2.clone(), ext_arg)?;
            }
            (b, ext_arg)
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            let mut ext_arg: T;
            let mut b: bool;
            tp = Expression::r#typeof(e2.clone())?;
            e1 = Expression::makeCrefExp(cr.clone(), tp)?;
            (_, b, ext_arg) = func(e1.clone(), inTypeA)?;
            if b {
                (_, b, ext_arg) = func(e2.clone(), ext_arg)?;
            }
            (b, ext_arg)
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. } => {
            let mut ext_arg: T;
            let mut b: bool;
            (_, b, ext_arg) = func(e1.clone(), inTypeA)?;
            (b, ext_arg)
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: we, .. } => {
            let mut ext_arg: T;
            let mut b: bool;
            (b, ext_arg) = traverseExpsOfWhenEquation_WithStop(we.clone(), func.clone(), inTypeA)?;
            (b, ext_arg)
        },
        Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { .. }, .. } => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("not implemented error - BackendDAE.ALGORITHM - BackendEquation.traverseExpsOfEquation_WithStop\n")).clone())?;
            bail!("fail")
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            let mut ext_arg: T;
            let mut b: bool;
            (_, b, ext_arg) = func(e1.clone(), inTypeA)?;
            if b {
                (_, b, ext_arg) = func(e2.clone(), ext_arg)?;
            }
            (b, ext_arg)
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: expl, eqnstrue: eqnslst, eqnsfalse: eqns, .. } => {
            let mut ext_arg: T;
            let mut b: bool;
            (b, ext_arg) = traverseExpsOfExpList_WithStop(expl.clone(), func.clone(), inTypeA)?;
            if b {
                (b, ext_arg) = traverseExpsOfEquationListList_WithStop(eqnslst.clone(), func.clone(), ext_arg)?;
            }
            if b {
                (b, ext_arg) = traverseExpsOfEquationList_WithStop(eqns.clone(), func.clone(), ext_arg)?;
            }
            (b, ext_arg)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBoolean, outTypeA))
}

fn traverseExpsOfWhenEquation<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inTypeA: T) -> Result<(Arc<BackendDAE::WhenEquation>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outWhenEquation: Arc<BackendDAE::WhenEquation>;
    let mut outTypeA: T;
    (outWhenEquation, outTypeA) = (::match_deref::match_deref! { match &(inWhenEquation) {
        Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: oelsewe } => {
            let mut elsewe: Arc<BackendDAE::WhenEquation>;
            let mut extArg: T;
            let mut cond = (*cond).clone();
            let mut whenStmtLst = (*whenStmtLst).clone();
            let mut oelsewe = (*oelsewe).clone();
            (cond, extArg) = inFunc(cond.clone(), inTypeA)?;
            (whenStmtLst, extArg) = traverseExpsOfWhenOps(whenStmtLst.clone(), inFunc.clone(), extArg, metamodelica::nil())?;
            if isSome(oelsewe.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                elsewe = __pa0.clone();
                (elsewe, extArg) = traverseExpsOfWhenEquation(elsewe, inFunc.clone(), extArg)?;
                oelsewe = Some(elsewe);
            } else {
                oelsewe = None;
            }
            (Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oelsewe.clone() }), extArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outWhenEquation, outTypeA))
}

fn traverseExpsOfWhenOps<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inTypeA: T, mut inAccum: Arc<metamodelica::List<BackendDAE::WhenOperator>>) -> Result<(Arc<metamodelica::List<BackendDAE::WhenOperator>>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut outTypeA: T;
    (outWhenOps, outTypeA) = (::match_deref::match_deref! { match &(inWhenOps) {
        Deref @ metamodelica::List::Nil => {
            (inAccum.reverse(), inTypeA)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e1, right: e2, source }, tail: rest } => {
            let mut extArg: T;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA)?;
            (e2, extArg) = inFunc(e2.clone(), extArg)?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg, metamodelica::cons(BackendDAE::WhenOperator::ASSIGN { left: e1.clone(), right: e2.clone(), source: source.clone() }, inAccum))?;
            (outWhenOps, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { stateVar: cr, value: e2, source }, tail: rest } => {
            let mut e1: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut extArg: T;
            let mut e2 = (*e2).clone();
            e1 = Expression::crefExp(cr.clone())?;
            (e1, extArg) = inFunc(e1, inTypeA)?;
            if Expression::isCref(e1.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(e1.clone()) {
                    Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                cr1 = __pa0.clone();
            } else {
                cr1 = cr.clone();
            }
            (e2, extArg) = inFunc(e2.clone(), extArg)?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg, metamodelica::cons(BackendDAE::WhenOperator::REINIT { stateVar: cr1, value: e2.clone(), source: source.clone() }, inAccum))?;
            (outWhenOps, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { condition: e1, message: e2, level, source }, tail: rest } => {
            let mut extArg: T;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA)?;
            (e2, extArg) = inFunc(e2.clone(), extArg)?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg, metamodelica::cons(BackendDAE::WhenOperator::ASSERT { condition: e1.clone(), message: e2.clone(), level: level.clone(), source: source.clone() }, inAccum))?;
            (outWhenOps, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: e1, source }, tail: rest } => {
            let mut extArg: T;
            let mut e1 = (*e1).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA)?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg, metamodelica::cons(BackendDAE::WhenOperator::TERMINATE { message: e1.clone(), source: source.clone() }, inAccum))?;
            (outWhenOps, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: e1, source }, tail: rest } => {
            let mut extArg: T;
            let mut e1 = (*e1).clone();
            (e1, extArg) = inFunc(e1.clone(), inTypeA)?;
            (outWhenOps, extArg) = traverseExpsOfWhenOps(rest.clone(), inFunc.clone(), extArg, metamodelica::cons(BackendDAE::WhenOperator::NORETCALL { exp: e1.clone(), source: source.clone() }, inAccum))?;
            (outWhenOps, extArg)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outWhenOps, outTypeA))
}

fn traverseExpsOfWhenEquation_WithStop<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outCont: bool;
    let mut outTypeA: T;
    (outCont, outTypeA) = (::match_deref::match_deref! { match &(inWhenEquation) {
        Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: oelsewe } => {
            let mut elsewe: Arc<BackendDAE::WhenEquation>;
            let mut extArg: T;
            let mut b: bool;
            (_, b, extArg) = inFunc(cond.clone(), inTypeA)?;
            if b {
                (b, extArg) = traverseExpsOfWhenOps_WithStop(whenStmtLst.clone(), inFunc.clone(), extArg, b)?;
            }
            if b {
                if isSome(oelsewe.clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    elsewe = __pa0.clone();
                    (b, extArg) = traverseExpsOfWhenEquation_WithStop(elsewe, inFunc.clone(), extArg)?;
                }
            }
            (b, extArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCont, outTypeA))
}

pub(crate) fn statementEq(mut iStmts: Arc<DAE::Statement>) -> Result<Arc<BackendDAE::Equation>> {
    let mut oEq: Arc<BackendDAE::Equation>;
    oEq = (::match_deref::match_deref! { match &(iStmts) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp, .. } => {
            generateEquation(Expression::crefExp(cr.clone())?, exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp, .. } => {
            generateEquation(Expression::crefExp(cr.clone())?, exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: explst, exp, .. } => {
            generateEquation(Expression::makeTuple(explst.clone())?, exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oEq)
}

fn traverseExpsOfWhenOps_WithStop<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inWhenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inTypeA: T, mut inCont: bool) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outCont: bool;
    let mut extArg: T = inTypeA.clone();
    (outCont, extArg) = ({
        let mut b: bool = false;
        (::match_deref::match_deref! { match &(inWhenOps) {
        Deref @ metamodelica::List::Nil => {
            (inCont, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e1, right: e2, .. }, tail: rest } => {
            if inCont {
                (_, b, extArg) = inFunc(e1.clone(), extArg)?;
            }
            if b {
                (_, b, extArg) = inFunc(e2.clone(), extArg)?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg, b)?;
            (b, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { stateVar: cr, value: e2, .. }, tail: rest } => {
            let mut tp: Arc<DAE::Type>;
            let mut e1: Arc<DAE::Exp>;
            tp = Expression::r#typeof(e2.clone())?;
            e1 = Expression::makeCrefExp(cr.clone(), tp)?;
            if inCont {
                (_, b, extArg) = inFunc(e1.clone(), extArg)?;
            }
            if b {
                (_, b, extArg) = inFunc(e2.clone(), extArg)?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg, b)?;
            (b, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { condition: e1, message: e2, .. }, tail: rest } => {
            if inCont {
                (_, b, extArg) = inFunc(e1.clone(), extArg)?;
            }
            if b {
                (_, b, extArg) = inFunc(e2.clone(), extArg)?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg, b)?;
            (b, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: e1, .. }, tail: rest } => {
            if inCont {
                (_, b, extArg) = inFunc(e1.clone(), extArg)?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg, b)?;
            (b, extArg)
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: e1, .. }, tail: rest } => {
            if inCont {
                (_, b, extArg) = inFunc(e1.clone(), extArg)?;
            }
            (b, extArg) = traverseExpsOfWhenOps_WithStop(rest.clone(), inFunc.clone(), extArg, b)?;
            (b, extArg)
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok((outCont, extArg))
}

fn traverseExpsOfExpList<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inExtArg: T) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeA: T = inExtArg.clone();
    for mut e in &*inExpl {
        let mut e = e.clone();
        (e, outTypeA) = rel(e.clone(), outTypeA.clone())?;
        outExpl = metamodelica::cons(e.clone(), outExpl.clone());
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl);
    Ok((outExpl, outTypeA))
}

fn traverseExpsOfExpList_WithStop<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>, mut inExtArg: T) -> Result<(bool, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, bool, T)> + 'static>;

    let mut outBoolean: bool = true;
    let mut outTypeA: T = inExtArg.clone();
    for mut e in &*inExpl {
        let mut e = e.clone();
        (_, outBoolean, outTypeA) = rel(e.clone(), outTypeA.clone())?;
        if !(outBoolean) {
            break;
        }
    }
    Ok((outBoolean, outTypeA))
}

pub(crate) fn equationEqual(mut e1: Arc<BackendDAE::Equation>, mut e2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut res: bool = true;
    if referenceEq(&*(e1.clone()),&*(e2.clone())) {
        return Ok(res.clone());
    }
    res = (::match_deref::match_deref! { match &((e1, e2)) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e11, scalar: e12, .. }, Deref @ BackendDAE::Equation::EQUATION { exp: e21, scalar: e22, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e11, right: e12, .. }, Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e21, right: e22, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e11, right: e12, .. }, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e21, right: e22, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1, exp: exp1, .. }, Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr2, exp: exp2, .. }) => {
            res = boolAnd(ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?, ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?);
            res
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp1, .. }, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp2, .. }) => {
            res = ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?;
            res
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg: alg1, .. }, Deref @ BackendDAE::Equation::ALGORITHM { alg: alg2, .. }) => {
            let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            explst1 = Algorithm::getAllExps(alg1.clone())?;
            explst2 = Algorithm::getAllExps(alg2.clone())?;
            res = List::isEqualOnTrue(explst1, explst2, (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
            res
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e11, right: e12, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e21, right: e22, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }) => {
            res = boolAnd(ExpressionBasics::expEqual(e11.clone(), e21.clone())?, ExpressionBasics::expEqual(e12.clone(), e22.clone())?);
            res
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn equationAddDAE(mut inEquation: Arc<BackendDAE::Equation>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), add(inEquation, inEqSystem.orderedEqs.clone())?);
    assign_field!(outEqSystem.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING());
    Ok(outEqSystem)
}

pub(crate) fn equationsAddDAE(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = inEqSystem.clone();
    assign_field!(
        outEqSystem.orderedEqs = addList(inEquations, outEqSystem.orderedEqs.clone())?,
        outEqSystem.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING()
    );
    Ok(outEqSystem)
}

pub(crate) fn requationsAddDAE(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inSyst: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem>;
    outSyst = if (inEquations.clone().is_empty()) {inSyst} else {BackendDAEUtil::setEqSystRemovedEqns(inSyst.clone(), addList(inEquations, inSyst.removedEqs.clone())?)};
    Ok(outSyst)
}

pub(crate) fn removeRemovedEqs(mut eqSystem: Arc<BackendDAE::EqSystem>) -> Arc<BackendDAE::EqSystem> {
    let mut eqSystem: Arc<BackendDAE::EqSystem> = eqSystem;
    ExpandableArray::clear(eqSystem.removedEqs.clone());
    eqSystem
}

pub(crate) fn equationToScalarResidualForm(mut inEquation: Arc<BackendDAE::Equation>, mut funcTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEquations = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: explst }, scalar: e2, source, attr } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (_, eqns) = List::fold3(explst.clone(), (std::sync::Arc::new(equationTupleToScalarResidualForm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes, (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), e2.clone(), source.clone(), attr.clone(), (1, metamodelica::nil()))?;
            eqns
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { real: __rlit_0 }, scalar: e2, source, attr } if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            let mut e: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (e, _) = ExpressionSimplify::simplify(e2.clone())?;
            eqns = list![Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source: source.clone(), attr: attr.clone() })];
            eqns
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, source, attr } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            let mut e: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (e, _) = ExpressionSimplify::simplify(e1.clone())?;
            eqns = list![Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source: source.clone(), attr: attr.clone() })];
            eqns
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
            list![Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp, source: source.clone(), attr: attr.clone() })]
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, source, attr } => {
            let mut e1: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            e1 = Expression::crefExp(cr.clone())?;
            exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
            list![Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp, source: source.clone(), attr: attr.clone() })]
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, left: e1, right: e2, source, attr, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>;
            exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
            subslst = Expression::dimensionSizesSubscripts(ds.clone())?;
            subslst = Expression::rangesToSubscripts(subslst)?;
            explst = List::map1r(subslst, (std::sync::Arc::new(Expression::applyExpSubscripts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Exp>> + 'static>), exp)?;
            explst = ExpressionSimplify::simplifyList(explst)?;
            eqns = List::map2(explst.clone(), (std::sync::Arc::new(fnptr!(generateRESIDUAL_EQUATION, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), source.clone(), attr.clone())?;
            eqns
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1 @ Deref @ DAE::Exp::CALL { expLst: explst, .. }, right: e2, source, attr, .. } if (Expression::isRecordCall(e1.clone(), funcTree.clone())? && Expression::isCref(e2.clone())) => {
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut explst = (*explst).clone();
            crlst = ComponentReference::expandCref(Expression::expCref(e2.clone())?, true)?;
            explst2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (crlst).into_iter().cloned() {
            let __x = Expression::crefExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            explst = List::threadMap(explst.clone(), explst2, (std::sync::Arc::new(Expression::createResidualExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            eqns = List::map2(explst.clone(), (std::sync::Arc::new(fnptr!(generateRESIDUAL_EQUATION, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), source.clone(), attr.clone())?;
            eqns
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e1 @ Deref @ DAE::Exp::CALL { expLst: explst, .. }, left: e2, source, attr, .. } if (Expression::isRecordCall(e1.clone(), funcTree.clone())? && Expression::isCref(e2.clone())) => {
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut explst = (*explst).clone();
            crlst = ComponentReference::expandCref(Expression::expCref(e2.clone())?, true)?;
            explst2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (crlst).into_iter().cloned() {
            let __x = Expression::crefExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            explst = List::threadMap(explst.clone(), explst2, (std::sync::Arc::new(Expression::createResidualExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            eqns = List::map2(explst.clone(), (std::sync::Arc::new(fnptr!(generateRESIDUAL_EQUATION, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), source.clone(), attr.clone())?;
            eqns
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr, .. } if (Expression::isCref(e1.clone()) && Expression::isCref(e2.clone())) => {
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut crlst2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crlst = ComponentReference::expandCref(Expression::expCref(e1.clone())?, true)?;
            crlst2 = ComponentReference::expandCref(Expression::expCref(e2.clone())?, true)?;
            explst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (crlst).into_iter().cloned() {
            let __x = Expression::crefExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            explst2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (crlst2).into_iter().cloned() {
            let __x = Expression::crefExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            explst = List::threadMap(explst, explst2, (std::sync::Arc::new(Expression::createResidualExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            eqns = List::map2(explst.clone(), (std::sync::Arc::new(fnptr!(generateRESIDUAL_EQUATION, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), source.clone(), attr.clone())?;
            eqns
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr, .. } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
            list![Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp, source: source.clone(), attr: attr.clone() })]
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: condExps, eqnstrue, eqnsfalse, source, attr } => {
            let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
            let mut e2: Arc<DAE::Exp>;
            let mut cond: Arc<DAE::Exp>;
            let mut i: i32;
            let mut branches: i32;
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut expA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
            branches = (condExps.clone().len() as i32);
            expA = arrayCreate(branches, metamodelica::nil());
            for mut eqLst in &*eqnstrue.clone() {
                let mut eqLst = eqLst.clone();
                i = 1;
                for mut eq in &*eqLst.clone() {
                    let mut eq = eq.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(equationToScalarResidualForm(eq.clone(), funcTree.clone())?) {
                        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa0, source: _, attr: _ }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    expA = Array::consToElement(i, e1.clone(), expA.clone())?;
                    i = i + 1;
                }
            }
            i = 1;
            for mut eq in &*eqnsfalse.clone() {
                let mut eq = eq.clone();
                let __pa2 = ::match_deref::match_deref! { match &(equationToScalarResidualForm(eq.clone(), funcTree.clone())?) {
                    Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa2, source: _, attr: _ }, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa2.clone();
                expA = Array::consToElement(i, e1.clone(), expA.clone())?;
                i = i + 1;
            }
            eqns = metamodelica::nil();
            for mut i in 1..=branches {
                explst = metamodelica::arrayGet(expA.clone(), i)?;
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(explst.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e2 = __pa4.clone();
                explst = __pa5.clone();
                explst2 = condExps.clone();
                for mut e1 in &*explst.clone() {
                    let mut e1 = e1.clone();
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(explst2.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cond = __pa6.clone();
                    explst2 = __pa7.clone();
                    e2 = Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1.clone(), expElse: e2.clone() });
                }
                eqns = metamodelica::cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e2.clone(), source: source.clone(), attr: attr.clone() }), eqns.clone());
            }
            eqns
        },
        backendEq @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            list![backendEq.clone()]
        },
        backendEq @ Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            list![backendEq.clone()]
        },
        backendEq @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            list![backendEq.clone()]
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            BackendDump::printEquation(inEquation)?;
            Debug::trace((literal!("- BackendDAE.equationToScalarResidualForm failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEquations)
}

fn equationTupleToScalarResidualForm(mut cr: Arc<DAE::Exp>, mut exp: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes, mut inTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    outTpl = (::match_deref::match_deref! { match &((cr.clone(), inTpl)) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, (i, eqs)) => {
            (i.clone() + 1, eqs.clone())
        },
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, (i, eqs)) => {
            (i.clone() + 1, eqs.clone())
        },
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, (i, eqs)) => {
            let mut eqs = (*eqs).clone();
            eqs = metamodelica::cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: Arc::new(DAE::Exp::TSUB { exp: exp, ix: i.clone(), ty: DAE::T_REAL_DEFAULT().clone() }), source: inSource, attr: inEqAttr }), eqs.clone());
            (i.clone() + 1, eqs.clone())
        },
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, .. }, (i, eqs)) => {
            let mut e: Arc<DAE::Exp>;
            let mut eqs = (*eqs).clone();
            e = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![Arc::new(DAE::Exp::TSUB { exp: exp, ix: i.clone(), ty: DAE::T_REAL_DEFAULT().clone() })], DAE::T_REAL_DEFAULT().clone());
            eqs = metamodelica::cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source: inSource, attr: inEqAttr }), eqs.clone());
            (i.clone() + 1, eqs.clone())
        },
        (_, (i, _)) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.equationTupleToScalarResidualForm failed: ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(cr)?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str).clone()], ElementSource::getElementSourceFileInfo(inSource))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

pub(crate) fn equationToResidualForm(mut inEquation: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEquation: Arc<BackendDAE::Equation>;
    outEquation = 'mc: {
        let __mc_input = inEquation;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, source, attr: eqAttr } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, source, attr: eqAttr, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr: eqAttr, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    exp = Expression::createResidualExp(e1.clone(), e2.clone())?;
                    (e, _) = ExpressionSimplify::simplify(exp.clone())?;
                    Ok(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                backendEq @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
                    Ok(backendEq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                backendEq @ Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
                    Ok(backendEq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                backendEq @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
                    Ok(backendEq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- BackendDAE.equationToResidualForm failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEquation)
}

pub fn traverseEquationToScalarResidualForm(mut inEq: Arc<BackendDAE::Equation>, mut inEqs: (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))> {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outEqs: (Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    (outEq, outEqs) = (::match_deref::match_deref! { match &((inEq, inEqs)) {
        (eqn, (funcs, eqns)) => {
            let mut reqn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            reqn = equationToScalarResidualForm(eqn.clone(), funcs.clone())?;
            eqns = listAppend(reqn, eqns.clone());
            (eqn.clone(), (funcs.clone(), eqns.clone()))
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.traverseEquationToScalarResidualForm")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outEqs))
}

pub fn convertResidualsIntoSolvedEquations(mut inResidualList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inName: ArcStr, mut inIndex: i32, mut isResidual: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outEquationList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outVariableList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outVarIndex: i32 = inIndex;
    for mut eq in &*inResidualList {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp, source, attr: eqAttr } => {
            let mut componentRef: Arc<DAE::ComponentRef>;
            let mut currEquation: Arc<BackendDAE::Equation>;
            let mut currVariable: BackendDAE::Var;
            componentRef = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*intString(outVarIndex)); ArcStr::from(__mm_s) }).clone(), identType: Expression::r#typeof(exp.clone())?, subscriptLst: metamodelica::nil() });
            currEquation = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: componentRef.clone(), exp: exp.clone(), source: source.clone(), attr: eqAttr.clone() });
            currVariable = BackendVariable::makeVar(componentRef.clone())?;
            if isResidual {
                currVariable = BackendVariable::setVarKind(currVariable.clone(), openmodelica_backend_types::BackendDAE::VarKind::DAE_RESIDUAL_VAR)?;
            }
            outVarIndex = outVarIndex + 1;
            outEquationList = metamodelica::cons(currEquation.clone(), outEquationList.clone());
            outVariableList = metamodelica::cons(currVariable.clone(), outVariableList.clone());
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.convertResidualsIntoSolvedEquations")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outEquationList = metamodelica::Dangerous::listReverseInPlace(outEquationList);
    outVariableList = metamodelica::Dangerous::listReverseInPlace(outVariableList);
    Ok((outEquationList, outVariableList, outVarIndex))
}

pub(crate) fn equationInfo(mut eq: Arc<BackendDAE::Equation>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = ElementSource::getElementSourceFileInfo(equationSource(eq)?);
    Ok(info)
}

pub(crate) fn markedEquationSource(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inPos: i32) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource>;
    outSource = equationSource(get(inEqSystem.orderedEqs.clone(), inPos)?)?;
    Ok(outSource)
}

pub fn equationSource(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    source = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { source: __esc_source, .. } => {
            source = (*__esc_source).clone();
            source.clone()
        },
        _ => {
            Error::addInternalError((literal!("BackendEquation.equationSource failed!")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub(crate) fn equationSizeKeepAlgorithmAsOne(mut eq: Arc<BackendDAE::Equation>) -> Result<i32> {
    let mut osize: i32;
    osize = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { size: _, .. } => 1,
        _ => equationSize(eq)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osize)
}

pub(crate) fn equationSize(mut eq: Arc<BackendDAE::Equation>) -> Result<i32> {
    let mut osize: i32;
    osize = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { .. } => {
            1
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, recordSize: Some(recordSize), .. } => {
            let mut size: i32;
            size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)? * recordSize.clone();
            size
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, recordSize: None, .. } => {
            let mut size: i32;
            size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
            size
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            1
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            1
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size, .. } => {
            size.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { size, .. } => {
            size.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, .. } => {
            size.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse, .. } => {
            let mut size: i32;
            size = equationLstSize(eqnsfalse.clone())?;
            size
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { start: Deref @ DAE::Exp::ICONST { integer: start }, stop: Deref @ DAE::Exp::ICONST { integer: stop }, .. } => {
            let mut size: i32;
            size = (stop.clone() - start.clone() + 1) * equationSize(var_field!((*eq).body, BackendDAE::Equation::FOR_EQUATION).clone())?;
            size
        },
        _ => {
            Error::addInternalError((literal!("BackendEquation.equationSize failed!")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osize)
}

pub(crate) fn isInitialEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool;
    let mut eqKind: BackendDAE::EquationKind;
    eqKind = equationKind(inEquation)?;
    outBool = isInitialEqKind(eqKind);
    Ok(outBool)
}

pub(crate) fn isInitialEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool;
    outBool = (match inEqKind {
        BackendDAE::EquationKind::INITIAL_EQUATION { .. } => true,
        _ => false,
    });
    outBool
}

pub(crate) fn isDynamicEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool;
    outBool = isDynamicEqKind(equationKind(inEquation)?);
    Ok(outBool)
}

pub(crate) fn isDynamicEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool;
    outBool = (match inEqKind {
        BackendDAE::EquationKind::DYNAMIC_EQUATION { .. } => true,
        _ => false,
    });
    outBool
}

pub(crate) fn isBindingEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool;
    outBool = isBindingEqKind(equationKind(inEquation)?);
    Ok(outBool)
}

pub(crate) fn isBindingEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool;
    outBool = (match inEqKind {
        BackendDAE::EquationKind::BINDING_EQUATION { .. } => true,
        _ => false,
    });
    outBool
}

pub(crate) fn isDiscreteEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool;
    outBool = isDiscreteEqKind(equationKind(inEquation)?);
    Ok(outBool)
}

pub(crate) fn isDiscreteEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool;
    outBool = (match inEqKind {
        BackendDAE::EquationKind::DISCRETE_EQUATION { .. } => true,
        _ => false,
    });
    outBool
}

pub(crate) fn isAuxEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outBool: bool;
    outBool = isAuxEqKind(equationKind(inEquation)?);
    Ok(outBool)
}

pub(crate) fn isAuxEqKind(mut inEqKind: BackendDAE::EquationKind) -> bool {
    let mut outBool: bool;
    outBool = (match inEqKind {
        BackendDAE::EquationKind::AUX_EQUATION { .. } => true,
        _ => false,
    });
    outBool
}

pub(crate) fn defaultClockedEqAttr(mut clockIndex: i32) -> BackendDAE::EquationAttributes {
    let mut outEqAttr: BackendDAE::EquationAttributes;
    outEqAttr = BackendDAE::EquationAttributes { differentiated: false, kind: BackendDAE::EquationKind::CLOCKED_EQUATION { clk: clockIndex }, evalStages: BackendDAE::defaultEvalStages.clone() };
    outEqAttr
}

pub(crate) fn equationKind(mut inEquation: Arc<BackendDAE::Equation>) -> Result<BackendDAE::EquationKind> {
    let mut outEqKind: BackendDAE::EquationKind;
    outEqKind = (::match_deref::match_deref! { match &(inEquation) {
        Deref @ BackendDAE::Equation::EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: BackendDAE::EquationAttributes { kind, .. }, .. } => {
            kind.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.equationKind")); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqKind)
}

pub(crate) fn setEquationKind(mut eq: Arc<BackendDAE::Equation>, mut k: BackendDAE::EquationKind) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::EquationKind)> {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut k: BackendDAE::EquationKind = k;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::ARRAY_EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::FOR_EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::SOLVED_EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::RESIDUAL_EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::WHEN_EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::ALGORITHM; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::COMPLEX_EQUATION; attr = a.clone());
            eq
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: a, .. } => {
            let mut a = (*a).clone();
            a.kind = k;
            assign_variant_field!(eq => BackendDAE::Equation::IF_EQUATION; attr = a.clone());
            eq
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendEquation.setEquationKind")); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, k))
}

pub(crate) fn setEvalStageDynamic(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.dynamicEval = true;
    evalStage
}

pub(crate) fn setEvalStageAlgebraic(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.algebraicEval = true;
    evalStage
}

pub(crate) fn setEvalStageZeroCross(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.zerocrossEval = true;
    evalStage
}

pub(crate) fn setEvalStageDiscrete(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.discreteEval = true;
    evalStage
}

pub(crate) fn setEvalStageOnlyDiscrete(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage = setEvalStage(evalStage, false, false, false, true);
    evalStage
}

pub(crate) fn setEvalStageAll(mut evalStage: BackendDAE::EvaluationStages) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage = setEvalStage(evalStage, true, true, true, true);
    evalStage
}

pub(crate) fn setEvalStage(mut evalStage: BackendDAE::EvaluationStages, mut dynamicEval: bool, mut algebraicEval: bool, mut zerocrossEval: bool, mut discreteEval: bool) -> BackendDAE::EvaluationStages {
    let mut evalStage: BackendDAE::EvaluationStages = evalStage;
    evalStage.dynamicEval = dynamicEval;
    evalStage.algebraicEval = algebraicEval;
    evalStage.zerocrossEval = zerocrossEval;
    evalStage.discreteEval = discreteEval;
    evalStage
}

pub(crate) fn setEquationEvalStage(mut eqn: Arc<BackendDAE::Equation>, mut func: Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>) -> Result<Arc<BackendDAE::Equation>> {
    pub type setEvalStage = std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::EvaluationStages) -> Result<BackendDAE::EvaluationStages> + 'static>;

    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    let mut attr: BackendDAE::EquationAttributes;
    attr = getEquationAttributes(eqn.clone())?;
    attr.evalStages = func(attr.evalStages.clone())?;
    eqn = setEquationAttributes(eqn, attr)?;
    Ok(eqn)
}

pub(crate) fn equationLstSize(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut size: i32 = 0;
    for mut eqn in &*inEqns {
        let mut eqn = eqn.clone();
        size = size + equationSize(eqn.clone())?;
    }
    Ok(size)
}

pub(crate) fn equationLstSizeKeepAlgorithmAsOne(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut size: i32 = 0;
    for mut eqn in &*inEqns {
        let mut eqn = eqn.clone();
        size = size + equationSizeKeepAlgorithmAsOne(eqn.clone())?;
    }
    Ok(size)
}

pub(crate) fn generateEquation(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    let mut ty: Arc<DAE::Type>;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Expression::r#typeof(lhs.clone())?;
    outEqn = (match () {
        () if (DAEUtil::expTypeComplex(ty.clone()) || DAEUtil::expTypeTuple(ty.clone())) => {
            let mut size: i32;
            size = Expression::sizeOf(ty.clone());
            Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: lhs, right: rhs, source: source, attr: inEqAttr })
        },
        () if (DAEUtil::expTypeArray(ty.clone())) => {
            let mut recordSize: Option<i32>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut ds: Arc<metamodelica::List<i32>>;
            tp = Expression::r#typeof(lhs.clone())?;
            tp = DAEUtil::expTypeElementType(tp);
            if DAEUtil::expTypeComplex(tp.clone()) {
                recordSize = Some(Expression::sizeOf(tp));
            } else {
                recordSize = None;
            }
            dims = Expression::arrayDimension(ty.clone());
            ds = Expression::dimensionsSizes(dims)?;
            Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, left: lhs, right: rhs, source: source, attr: inEqAttr, recordSize: recordSize })
        },
        () if (!(DAEUtil::expTypeComplex(ty.clone())) && !(DAEUtil::expTypeArray(ty.clone()))) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, source: source, attr: inEqAttr })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendEquation.generateEquation failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    });
    Ok(outEqn)
}

pub(crate) fn getEquationArraySubsetLst(mut eqnArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iLst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut subset: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut i in &*iLst {
        let mut i = i.clone();
        subset = metamodelica::cons(ExpandableArray::get(i.clone(), eqnArr.clone())?, subset.clone());
    }
    Ok(subset)
}

pub(crate) fn getEquationAttributes(mut inEqn: Arc<BackendDAE::Equation>) -> Result<BackendDAE::EquationAttributes> {
    let mut outAttr: BackendDAE::EquationAttributes;
    outAttr = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr, .. } => {
            attr.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { attr, .. } => {
            attr.clone()
        },
        _ => {
            Error::addInternalError((literal!("function getEquationAttributes failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttr)
}

pub(crate) fn setEquationAttributes(mut inEqn: Arc<BackendDAE::Equation>, mut inAttr: BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, source, .. } => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: lhs, right: rhs, source, recordSize, .. } => {
            Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: inAttr, recordSize: recordSize.clone() })
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { .. } => {
            Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: var_field!((*inEqn).iter, BackendDAE::Equation::FOR_EQUATION).clone(), start: var_field!((*inEqn).start, BackendDAE::Equation::FOR_EQUATION).clone(), stop: var_field!((*inEqn).stop, BackendDAE::Equation::FOR_EQUATION).clone(), body: var_field!((*inEqn).body, BackendDAE::Equation::FOR_EQUATION).clone(), source: var_field!((*inEqn).source, BackendDAE::Equation::FOR_EQUATION).clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef, exp: rhs, source, .. } => {
            Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: componentRef.clone(), exp: rhs.clone(), source: source.clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: rhs, source, .. } => {
            Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: rhs.clone(), source: source.clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::ALGORITHM { size, alg, source, expand, .. } => {
            Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: expand.clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation, source, .. } => {
            Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEquation.clone(), source: source.clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: lhs, right: rhs, source, .. } => {
            Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: inAttr })
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions, eqnstrue, eqnsfalse, source, .. } => {
            Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: conditions.clone(), eqnstrue: eqnstrue.clone(), eqnsfalse: eqnsfalse.clone(), source: source.clone(), attr: inAttr })
        },
        _ => {
            Error::addInternalError((literal!("function setEquationAttributes failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub(crate) fn setEquationLHS(mut inEqn: Arc<BackendDAE::Equation>, mut lhs: Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = (::match_deref::match_deref! { match &(inEqn) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; exp = lhs);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; left = lhs);
            eqn.clone()
        },
        _ => {
            Error::addInternalError((literal!("function setEquationLHS failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub(crate) fn setEquationRHS(mut inEqn: Arc<BackendDAE::Equation>, mut rhs: Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = (::match_deref::match_deref! { match &(inEqn) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; scalar = rhs);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; right = rhs);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; exp = rhs);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; exp = rhs);
            eqn.clone()
        },
        _ => {
            Error::addInternalError((literal!("function setEquationRHS failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub(crate) fn generateSolvedEqnsfromOption(mut inLhs: Arc<DAE::ComponentRef>, mut inRhs: Option<Arc<DAE::Exp>>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEqn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEqn = (::match_deref::match_deref! { match &(inRhs) {
        Some(rhs) => {
            list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: inLhs, exp: rhs.clone(), source: inSource, attr: inEqAttr })]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqn
}

pub(crate) fn generateResidualFromRelation(mut conCrefName: ArcStr, mut iRhs: Arc<DAE::Exp>, mut Source: Arc<DAE::ElementSource>, mut inVars: BackendDAE::Variables, mut knvars: BackendDAE::Variables, mut conKind: BackendDAE::VarKind) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Var)> {
    let mut outEqn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut vout: BackendDAE::Var;
    (outEqn, vout) = (::match_deref::match_deref! { match &(iRhs) {
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::LESS { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ } => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef>;
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e1.clone(), e2.clone())?;
            (rhs, _) = ExpressionSimplify::simplify1(rhs)?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar, Some(lowBound), Some(expNull))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs, exp: rhs, source: Source, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar)
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::LESSEQ { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ } => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef>;
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e1.clone(), e2.clone())?;
            (rhs, _) = ExpressionSimplify::simplify1(rhs)?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar, Some(lowBound), Some(expNull))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs, exp: rhs, source: Source, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar)
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::GREATER { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ } => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef>;
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e2.clone(), e1.clone())?;
            (rhs, _) = ExpressionSimplify::simplify1(rhs)?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar, Some(lowBound), Some(expNull))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs, exp: rhs, source: Source, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar)
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::GREATEREQ { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ } => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lowBound: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef>;
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e2.clone(), e1.clone())?;
            (rhs, _) = ExpressionSimplify::simplify(rhs)?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            lowBound = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e21_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar, Some(lowBound), Some(expNull))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs, exp: rhs, source: Source, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar)
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: DAE::Operator::EQUAL { ty: _ }, exp2: e2, index: _, optionExpisASUB: _ } => {
            let mut rhs: Arc<DAE::Exp>;
            let mut expNull: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::ComponentRef>;
            let mut dummyVar: BackendDAE::Var;
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            rhs = Expression::expSub(e2.clone(), e1.clone())?;
            (rhs, _) = ExpressionSimplify::simplify(rhs)?;
            expNull = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
            dummyVar = BackendVariable::setVarMinMax(dummyVar, Some(expNull.clone()), Some(expNull))?;
            (list![Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs, exp: rhs, source: Source, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() })], dummyVar)
        },
        e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut lhs: Arc<DAE::ComponentRef>;
            let mut dummyVar: BackendDAE::Var;
            let mut v: BackendDAE::Var;
            let mut eqn: Arc<BackendDAE::Equation>;
            match '__try0: {
                (v, _) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), inVars.clone()), '__try0);
                Ok::<_, anyhow::Error>((v.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    v = __try0_o0;
                }
                Err(_) => {
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), knvars.clone())?;
                }
            }
            lhs = ComponentReferenceBasics::makeCrefIdent((conCrefName).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
            dummyVar = BackendDAE::Var { varName: lhs.clone(), varKind: conKind, varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            dummyVar = BackendVariable::mergeAliasVars(dummyVar, v, false, knvars)?;
            eqn = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: lhs, exp: e1.clone(), source: Source, attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
            (list![eqn], dummyVar)
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEqn, vout))
}

pub(crate) fn makeTmpEqnForExp(mut iExp: Arc<DAE::Exp>, mut name: ArcStr, mut offset: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>, mut noPara: bool) -> Result<(Arc<DAE::Exp>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>, bool, bool)> {
    let mut oExp: Arc<DAE::Exp>;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut update: bool;
    let mut para: bool = false;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut tmpvar: BackendDAE::Var;
    let mut name_: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("__OMC__")); __mm_s.push_str(&*intString(offset)); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) };
    let mut y: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut eqnVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut eqnKnVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut inputsKnVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut knowVars: BackendDAE::Variables;
    let mut b: bool;
    (y, _) = ExpressionSimplify::simplify(iExp.clone())?;
    if makeTmpEqnForExp_rule(y.clone())? {
        update = true;
        cr = ComponentReferenceBasics::makeCrefIdent((name_).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
        oExp = Expression::crefExp(cr.clone())?;
        tmpvar = BackendVariable::makeVar(cr)?;
        tmpvar = BackendVariable::setVarTS(tmpvar, Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: oExp.clone(), scalar: y.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!(" -- new eqn--\n")); ArcStr::from(__mm_s) }).clone());
        }
        eqnVars = equationVars(eqn.clone(), ivars)?;
        b = eqnVars.is_empty() && !(Expression::expHasCref(y.clone(), DAE::crefTime().clone())?);
        if b {
            knowVars = BackendVariable::daeGlobalKnownVars(oshared.clone());
            eqnKnVars = equationVars(eqn.clone(), knowVars)?;
            (inputsKnVars, _) = List::splitOnTrue(eqnKnVars, (std::sync::Arc::new(fnptr!(BackendVariable::isInput, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
            b = inputsKnVars.is_empty();
        }
        b = false;
        if b {
            if noPara {
                (oExp, _) = ExpressionSimplify::simplify(iExp)?;
                update = false;
            } else {
                tmpvar = BackendVariable::setBindExp(tmpvar, Some(y));
                tmpvar = BackendVariable::setVarKind(tmpvar, openmodelica_backend_types::BackendDAE::VarKind::PARAM)?;
                oshared = BackendVariable::addGlobalKnownVarDAE(tmpvar, oshared)?;
                para = true;
            }
        } else {
            oeqns = add(eqn, oeqns)?;
            ovars = BackendVariable::addVar(tmpvar, ovars)?;
        }
    } else {
        oExp = y;
        update = false;
    }
    Ok((oExp, oeqns, ovars, oshared, update, para))
}

fn makeTmpEqnForExp_rule(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut allowed: bool;
    if Expression::isCref(inExp.clone()) || Expression::isConst(inExp.clone())? || Expression::isUnaryCref(inExp.clone()) {
        allowed = false;
        return Ok(allowed.clone());
    }
    allowed = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } if ((Expression::isOne(e1.clone()) || Expression::isConstMinusOne(e1.clone())) && (Expression::isCref(e2.clone()) || Expression::isUnaryCref(e2.clone()))) => {
            false
        },
        Deref @ DAE::Exp::CAST { exp: e1, .. } => {
            makeTmpEqnForExp_rule(e1.clone())?
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(allowed)
}

pub(crate) fn normalizationVec(mut vec: metamodelica::Array<Arc<DAE::Exp>>, mut name: ArcStr, mut offset: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut nvec: metamodelica::Array<Arc<DAE::Exp>>;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut ovars: BackendDAE::Variables;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut len: Arc<DAE::Exp> = Expression::lenVec(vec.clone())?;
    (len, oeqns, ovars, oshared, _, _) = makeTmpEqnForExp(len, (name).clone(), offset, ieqns, ivars, ishared, false)?;
    if Expression::isZero(len.clone())? {
        bail!("fail");
    }
    nvec = Array::map1(vec.clone(), (std::sync::Arc::new(Expression::makeDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), len)?;
    Ok((nvec, oeqns, ovars, oshared))
}

pub(crate) fn solveEquation(mut eqn: Arc<BackendDAE::Equation>, mut crefExp: Arc<DAE::Exp>, mut functions: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = 'mc: {
        let __mc_input = eqn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
                    let mut res: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve2(e1.clone(), e2.clone(), crefExp.clone(), functions.clone(), None, true, false)?) {
                        (__pa0, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    res = __pa0.clone();
                    Ok(Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, source, attr: eqAttr, .. } => {
                    let mut res: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve2(e1.clone(), e2.clone(), crefExp.clone(), functions.clone(), None, true, false)?) {
                        (__pa0, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    res = __pa0.clone();
                    Ok(Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e2, source, attr: eqAttr } => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    cr = Expression::expCref(crefExp.clone())?;
                    let true = (ComponentReferenceBasics::crefEqual(cref.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: e2.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e2, source, attr: eqAttr } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cref.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve2(e1.clone(), e2.clone(), crefExp.clone(), functions.clone(), None, true, false)?) {
                        (__pa0, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    res = __pa0.clone();
                    Ok(Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e2, source, attr: eqAttr } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    e1 = Expression::makeConstZero(Expression::r#typeof(e2.clone())?);
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve2(e2.clone(), e1.clone(), crefExp.clone(), functions.clone(), None, true, false)?) {
                        (__pa0, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    res = __pa0.clone();
                    Ok(Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr: eqAttr, .. } => {
                    let mut res: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve2(e1.clone(), e2.clone(), crefExp.clone(), functions.clone(), None, true, false)?) {
                        (__pa0, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    res = __pa0.clone();
                    Ok(Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: res.clone(), source: source.clone(), attr: eqAttr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    BackendDump::dumpBackendDAEEqnList(list![eqn.clone()], ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function BackendEquation.solveEquation failed w.r.t ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(crefExp.clone())?); ArcStr::from(__mm_s) }).clone(), true)?;
                    Error::addInternalError((literal!("function solveEquation failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEqn)
}

pub(crate) fn generateRESIDUAL_EQUATION(mut inExp: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Arc<BackendDAE::Equation> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: inExp, source: inSource, attr: inEqAttr });
    outEqn
}

pub(crate) fn generateRESIDUAL_EQUATION1(mut inTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>), mut source: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    (e1, e2) = inTpl;
    e = Expression::createResidualExp(e1, e2)?;
    outEqn = Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source: source, attr: inEqAttr });
    Ok(outEqn)
}

pub(crate) fn equationSystemsEqnsLst(mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut eq: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    for mut es in &*systs {
        let mut es = es.clone();
        let __pa0 = ::match_deref::match_deref! { match &(es.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        eq = __pa0.clone();
        eqns = equationList(eq.clone())?;
        outEqns = List::append_reverse(eqns.clone(), outEqns.clone());
    }
    outEqns = metamodelica::Dangerous::listReverseInPlace(outEqns);
    Ok(outEqns)
}

pub(crate) fn getEqnsFromEqSystems(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outOrderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    outOrderedEqs = listEquation(equationSystemsEqnsLst(inEqSystems)?)?;
    Ok(outOrderedEqs)
}

pub fn getEqnsFromEqSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outOrderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqSystem.orderedEqs.clone();
    outOrderedEqs
}

pub(crate) fn getInitialEqnsFromShared(mut inShared: Arc<BackendDAE::Shared>) -> Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> {
    let mut outInitialEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inShared.initialEqs.clone();
    outInitialEqs
}

pub(crate) fn aliasEquation(mut inEqn: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>;
    outTpls = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            aliasEquation1(e1.clone(), e2.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            aliasEquation1(e1.clone(), e2.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::crefExp(cr.clone())?;
            aliasEquation1(e, e2.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. } => {
            aliasExpression(e1.clone(), metamodelica::nil())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            aliasEquation1(e1.clone(), e2.clone(), metamodelica::nil())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

fn aliasEquation1(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>;
    outTpls = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), lhs, rhs, false), inTpls)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs }), rhs, true), inTpls)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs }), rhs, true), inTpls)
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), lhs, Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs }), true), inTpls)
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), lhs, Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs }), true), inTpls)
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: lhs }), rhs, true), inTpls)
        },
        (Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), lhs, Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: rhs }), true), inTpls)
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, Deref @ DAE::Exp::ARRAY { array: elst2, .. }) => {
            List::threadFold(elst1.clone(), elst2.clone(), (std::sync::Arc::new(aliasEquation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls)?
        },
        (Deref @ DAE::Exp::MATRIX { matrix: elstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: elstlst2, .. }) => {
            List::threadFold(elstlst1.clone(), elstlst2.clone(), (std::sync::Arc::new(aliasEquationLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls)?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CALL { path: pathb, expLst: elst2, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst: varLst2, complexClassType: ClassInf::State::RECORD { path: pathb1 }, .. }, .. } }) if (AbsynUtil::pathEqual(pathb.clone(), pathb1.clone())) => {
            aliasRecord(cr1.clone(), varLst2.clone(), elst2.clone(), inTpls)?
        },
        (Deref @ DAE::Exp::CALL { path: patha, expLst: elst1, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst: varLst1, complexClassType: ClassInf::State::RECORD { path: patha1 }, .. }, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) if (AbsynUtil::pathEqual(patha.clone(), patha1.clone())) => {
            aliasRecord(cr2.clone(), varLst1.clone(), elst1.clone(), inTpls)?
        },
        (Deref @ DAE::Exp::CALL { path: patha, expLst: elst1, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: patha1 }, .. }, .. } }, Deref @ DAE::Exp::CALL { path: pathb, expLst: elst2, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: pathb1 }, .. }, .. } }) if (AbsynUtil::pathEqual(patha.clone(), patha1.clone()) && AbsynUtil::pathEqual(pathb.clone(), pathb1.clone())) => {
            List::threadFold(elst1.clone(), elst2.clone(), (std::sync::Arc::new(aliasEquation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls)?
        },
        _ => {
            aliasEquation2(lhs, rhs, inTpls)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpls)
}

fn aliasEquationLst(mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut elst2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>;
    outTpls = List::threadFold(elst1, elst2, (std::sync::Arc::new(aliasEquation1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls)?;
    Ok(outTpls)
}

fn aliasEquation2(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>;
    outTpls = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, _) if (Expression::isZero(rhs.clone())?) => {
            List::fold(elst1.clone(), (std::sync::Arc::new(aliasExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls)?
        },
        (_, Deref @ DAE::Exp::ARRAY { array: elst2, .. }) if (Expression::isZero(lhs.clone())?) => {
            List::fold(elst2.clone(), (std::sync::Arc::new(aliasExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> + 'static>), inTpls)?
        },
        (_, _) if (Expression::isZero(rhs.clone())?) => {
            aliasExpression(lhs.clone(), inTpls)?
        },
        (_, _) if (Expression::isZero(lhs.clone())?) => {
            aliasExpression(rhs.clone(), inTpls)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

fn aliasRecord(mut cr: Arc<DAE::ComponentRef>, mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((varLst, explst)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(inTpls)
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: ident, ty, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, tail: elst }) => {
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() });
            { (cr, varLst, explst, inTpls) = (cr, vlst.clone(), elst.clone(), metamodelica::cons((cr1, cr2.clone(), e1, e2.clone(), false), inTpls)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: ident, ty, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, tail: elst }) => {
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() }) });
            { (cr, varLst, explst, inTpls) = (cr, vlst.clone(), elst.clone(), metamodelica::cons((cr1, cr2.clone(), e1, e2.clone(), true), inTpls)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: ident, ty, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, tail: elst }) => {
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() }) });
            { (cr, varLst, explst, inTpls) = (cr, vlst.clone(), elst.clone(), metamodelica::cons((cr1, cr2.clone(), e1, e2.clone(), true), inTpls)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: ident, ty, .. }, tail: vlst }, Deref @ metamodelica::List::Cons { head: e2 @ Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }, tail: elst }) => {
            let mut cr1: Arc<DAE::ComponentRef>;
            let mut e1: Arc<DAE::Exp>;
            cr1 = ComponentReference::crefPrependIdent(cr.clone(), (ident.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e1 = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: cr1.clone(), ty: ty.clone() }) });
            { (cr, varLst, explst, inTpls) = (cr, vlst.clone(), elst.clone(), metamodelica::cons((cr1, cr2.clone(), e1, e2.clone(), true), inTpls)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn aliasExpression(mut exp: Arc<DAE::Exp>, mut inTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>> {
    let mut outTpls: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)>>;
    outTpls = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() }), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }), true), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e1.clone() }), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), true), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), e2.clone(), false), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }), true), inTpls)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            metamodelica::cons((cr1.clone(), cr2.clone(), e1.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), true), inTpls)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpls)
}

pub(crate) fn derivativeEquation(mut eqn: Arc<BackendDAE::Equation>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<DAE::Exp>, Arc<DAE::Exp>, bool)> {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut de: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut negate: bool;
    (cr, dcr, e, de, negate) = (::match_deref::match_deref! { match &(eqn) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. }, scalar: __esc_de @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            (cr.clone(), dcr.clone(), e.clone(), de.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, scalar: __esc_e @ Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            (cr.clone(), dcr.clone(), e.clone(), de.clone(), false)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. }, scalar: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne, de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. }, scalar: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne, de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, scalar: __esc_e @ Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne, de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, scalar: __esc_e @ Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            (cr.clone(), dcr.clone(), ne, de.clone(), true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, scalar: __esc_de @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne, true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, scalar: __esc_de @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne, true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, scalar: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne, true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, scalar: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            ne = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), e.clone(), ne, true)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, scalar: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne, ne2, false)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, scalar: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne, ne2, false)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, scalar: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne, ne2, false)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: __esc_de @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, scalar: __esc_e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: __esc_dcr, .. } }, .. } => {
            cr = (*__esc_cr).clone();
            dcr = (*__esc_dcr).clone();
            e = (*__esc_e).clone();
            de = (*__esc_de).clone();
            let mut ne: Arc<DAE::Exp>;
            let mut ne2: Arc<DAE::Exp>;
            ne = Expression::negate(e.clone())?;
            ne2 = Expression::negate(de.clone())?;
            (cr.clone(), dcr.clone(), ne, ne2, false)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cr, dcr, e, de, negate))
}

pub(crate) fn addOperation(mut inEqn: Arc<BackendDAE::Equation>, mut inSymOp: Arc<DAE::SymbolicOperation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = (::match_deref::match_deref! { match &(inEqn) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::ARRAY_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::SOLVED_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::RESIDUAL_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ALGORITHM; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::ALGORITHM).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::WHEN_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::WHEN_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::COMPLEX_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::IF_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::FOR_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::FOR_EQUATION; source = ElementSource::addSymbolicTransformation(var_field!((*eqn).source, BackendDAE::Equation::FOR_EQUATION).clone(), inSymOp)?);
            eqn.clone()
        },
        _ => {
            Error::addInternalError((literal!("BackendEquation.addOperation failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendEquation.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

pub(crate) fn isEquationsSystem(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn isTornSystem(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn isWhenEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isWhenEquationOrDiscreteAlgorithm(mut inEqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = inEqn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    let mut b1: bool;
                    let mut lhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    b1 = true;
                    for mut s in &*stmts.clone() {
                        let mut s = s.clone();
                        (lhsCrefs, _) = Expression::extractCrefsStatment(s.clone())?;
                        for mut c in &*lhsCrefs.clone() {
                            let mut c = c.clone();
                            b1 = b1.clone() && BackendVariable::isDiscrete(c.clone(), vars.clone())?;
                        }
                    }
                    Ok(b1.clone())
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
    b
}

pub(crate) fn isArrayEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isAlgorithm(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isComplexEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isEquation(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isNotAlgorithm(mut inEqn: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = !(isAlgorithm(inEqn));
    b
}

pub(crate) fn markDifferentiated(mut inEqn: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut outEqn: Arc<BackendDAE::Equation>;
    outEqn = (::match_deref::match_deref! { match &(inEqn.clone()) {
        eqn @ Deref @ BackendDAE::Equation::EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::ARRAY_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::ARRAY_EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::SOLVED_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::SOLVED_EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::RESIDUAL_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::RESIDUAL_EQUATION).clone()));
            eqn.clone()
        },
        eqn @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::COMPLEX_EQUATION; attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::COMPLEX_EQUATION).clone()));
            eqn.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            inEqn
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            inEqn
        },
        eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            let mut eqn = (*eqn).clone();
            assign_variant_field!(eqn => BackendDAE::Equation::IF_EQUATION;
                attr = markDifferentiated2(var_field!((*eqn).attr, BackendDAE::Equation::IF_EQUATION).clone()),
                eqnstrue = List::mapList(var_field!((*eqn).eqnstrue, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(markDifferentiated) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?,
                eqnsfalse = List::map(var_field!((*eqn).eqnsfalse, BackendDAE::Equation::IF_EQUATION).clone(), (std::sync::Arc::new(markDifferentiated) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?
            );
            eqn.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqn)
}

fn markDifferentiated2(mut attr: BackendDAE::EquationAttributes) -> BackendDAE::EquationAttributes {
    let mut attr: BackendDAE::EquationAttributes = attr;
    attr.differentiated = true;
    attr
}

pub(crate) fn isDifferentiated(mut inEqn: Arc<BackendDAE::Equation>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inEqn) {
        Deref @ BackendDAE::Equation::EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::ALGORITHM { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: BackendDAE::EquationAttributes { differentiated: b, .. }, .. } => {
            return Ok(b.clone())
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: Deref @ metamodelica::List::Cons { head: eqn, tail: _ }, .. } => {
            { inEqn = eqn.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn replaceDerOpInEquationList(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (outEqns, _) = traverseExpsOfEquationList(inEqns, (std::sync::Arc::new(Expression::replaceDerOpInExpCond) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)> + 'static>), None)?;
    Ok(outEqns)
}

pub(crate) fn getEquationRHS(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut rhs: Arc<DAE::Exp>;
    rhs = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition: Deref @ DAE::Exp::BCONST { bool: true }, whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: exp1, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            exp1.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rhs)
}

pub(crate) fn getEquationLHS(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut lhs: Arc<DAE::Exp>;
    lhs = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, .. } => {
            Expression::crefExp(cref.clone())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp1, .. } => {
            exp1.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition: Deref @ DAE::Exp::BCONST { bool: true }, whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: exp1, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            exp1.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lhs)
}

pub(crate) fn scalarComplexEquations(mut inEquation: Arc<BackendDAE::Equation>, mut funcTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEquations = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::TUPLE { PR: explst }, right: Deref @ DAE::Exp::TUPLE { PR: explst2 }, source, attr, .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let true = ((explst.clone().len() as i32) == (explst2.clone().len() as i32)) else { bail!("pattern mismatch") };
            eqns = List::threadMap2(explst.clone(), explst2.clone(), (std::sync::Arc::new(generateEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), source.clone(), attr.clone())?;
            eqns
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr, .. } if ((Expression::isRecordCall(e1.clone(), funcTree.clone())? || Expression::isRecord(e1.clone())) && (Expression::isRecordCall(e2.clone(), funcTree.clone())? || Expression::isRecord(e2.clone()))) => {
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            explst = Expression::splitRecord(e1.clone(), Expression::r#typeof(e1.clone())?)?;
            explst2 = Expression::splitRecord(e2.clone(), Expression::r#typeof(e2.clone())?)?;
            let true = ((explst.clone().len() as i32) == (explst2.clone().len() as i32)) else { bail!("pattern mismatch") };
            eqns = List::threadMap2(explst.clone(), explst2.clone(), (std::sync::Arc::new(generateEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), source.clone(), attr.clone())?;
            eqns
        },
        _ => {
            list![inEquation]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEquations)
}

pub(crate) fn allAlgorithmsLst(mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eqn_lst) {
        Deref @ metamodelica::List::Nil => {
            return true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { .. }, tail: Deref @ metamodelica::List::Nil } => {
            return true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { .. }, tail: rest } => {
            { eqn_lst = rest.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn createResidualExp(mut eqn: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = (::match_deref::match_deref! { match &(eqn) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            Expression::createResidualExp(e1.clone(), e2.clone())?
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            Expression::createResidualExp(e1.clone(), e2.clone())?
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            Expression::createResidualExp(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ComponentReference::crefTypeFull(cr.clone())? }), e2.clone())?
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. } => {
            e1.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            Expression::createResidualExp(e1.clone(), e2.clone())?
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn hasAnyUnknown(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut b: bool;
    b = !(equationVars(eqn, vars)?.is_empty());
    Ok(b)
}

