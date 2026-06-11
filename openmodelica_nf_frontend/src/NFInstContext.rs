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

use crate::NFInstNode::InstNode;
use crate::NFRestriction as Restriction;

pub type Type = i32;

// Flag values:
pub const NO_CONTEXT: i32 = 0;

// Global flags:
pub const RELAXED: i32 = intBitLShift(1, 0);

// Relaxed instantiation, used by e.g. checkModel.
pub const INSTANCE_API: i32 = intBitLShift(1, 1);

// Instantiation for the model instance API.
pub const FAST_LOOKUP: i32 = intBitLShift(1, 2);

// Only expand packages when doing lookup.
pub(crate) const GLOBAL_FLAGS: i32 = intBitOr(RELAXED, intBitOr(INSTANCE_API, FAST_LOOKUP));

// Scope flags:
pub const CLASS: i32 = intBitLShift(1, 3);

// In class.
pub const FUNCTION: i32 = intBitLShift(1, 4);

// In function.
pub(crate) const REDECLARED: i32 = intBitLShift(1, 5);

// In an element that will be replaced with a redeclare.
pub(crate) const ALGORITHM: i32 = intBitLShift(1, 6);

// In algorithm section.
pub(crate) const EQUATION: i32 = intBitLShift(1, 7);

// In equation section.
pub(crate) const INITIAL: i32 = intBitLShift(1, 8);

// In initial section.
pub(crate) const LHS: i32 = intBitLShift(1, 9);

// On left hand side of equality/assignment.
pub const RHS: i32 = intBitLShift(1, 10);

// On right hand side of equality/assignment.
pub(crate) const WHEN: i32 = intBitLShift(1, 11);

// In when equation/statement.
pub(crate) const CLOCKED: i32 = intBitLShift(1, 12);

// Part of a clocked when equation.
pub(crate) const FOR: i32 = intBitLShift(1, 13);

// In a for loop.
pub(crate) const IF: i32 = intBitLShift(1, 14);

// In an if equation/statement.
pub(crate) const WHILE: i32 = intBitLShift(1, 15);

// In a while loop.
pub(crate) const NONEXPANDABLE: i32 = intBitLShift(1, 16);

// In non-parameter if/for.
pub(crate) const ITERATION_RANGE: i32 = intBitLShift(1, 17);

// In range used for iteration.
pub(crate) const DIMENSION: i32 = intBitLShift(1, 18);

// In dimension.
pub(crate) const BINDING: i32 = intBitLShift(1, 19);

// In binding.
pub(crate) const CONDITION: i32 = intBitLShift(1, 20);

// In conditional expression.
pub(crate) const SUBSCRIPT: i32 = intBitLShift(1, 21);

// In subscript.
pub(crate) const SUBEXPRESSION: i32 = intBitLShift(1, 22);

// Part of a larger expression.
pub(crate) const CONNECT: i32 = intBitLShift(1, 23);

// Part of connect argument.
pub(crate) const NOEVENT: i32 = intBitLShift(1, 24);

// Part of noEvent argument.
pub(crate) const ASSERT: i32 = intBitLShift(1, 25);

// Part of assert argument.
pub const ANNOTATION: i32 = intBitLShift(1, 26);

// Part of an annotation.
// Combined flags:
pub(crate) const EQ_SUBEXPRESSION: i32 = intBitOr(EQUATION, SUBEXPRESSION);

pub(crate) const VALID_TYPENAME_SCOPE: i32 = intBitOr(ITERATION_RANGE, DIMENSION);

pub(crate) const DISCRETE_SCOPE: i32 = intBitOr(WHEN, intBitOr(INITIAL, FUNCTION));

pub(crate) const NON_EXP_FLAGS: i32 = intBitOr(GLOBAL_FLAGS, intBitOr(CLASS, FUNCTION));

pub fn set(mut context: Type, mut flag: Type) -> Type {
    let mut newOrigin: Type;
    newOrigin = intBitOr(context, flag);
    newOrigin
}

pub fn unset(mut context: Type, mut flag: Type) -> Type {
    let mut newOrigin: Type;
    newOrigin = intBitAnd(context, intBitNot(flag));
    newOrigin
}

pub(crate) fn isSet(mut context: Type, mut flag: Type) -> bool {
    let mut set: bool;
    set = intBitAnd(context, flag) > 0;
    set
}

pub(crate) fn isNotSet(mut context: Type, mut flag: Type) -> bool {
    let mut notSet: bool;
    notSet = intBitAnd(context, flag) == 0;
    notSet
}

pub(crate) fn clearScopeFlags(mut context: Type) -> Type {
    let mut outContext: Type;
    outContext = intBitAnd(context, GLOBAL_FLAGS.clone());
    outContext
}

pub(crate) fn clearExpFlags(mut context: Type) -> Type {
    let mut outContext: Type;
    outContext = intBitAnd(context, NON_EXP_FLAGS.clone());
    outContext
}

pub(crate) fn inRelaxed(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, RELAXED.clone()) > 0;
    res
}

pub(crate) fn inInstanceAPI(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, INSTANCE_API.clone()) > 0;
    res
}

pub(crate) fn inFastLookup(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, FAST_LOOKUP.clone()) > 0;
    res
}

pub(crate) fn inClass(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, CLASS.clone()) > 0;
    res
}

pub(crate) fn inFunction(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, FUNCTION.clone()) > 0;
    res
}

pub(crate) fn inRedeclared(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, REDECLARED.clone()) > 0;
    res
}

pub(crate) fn inAlgorithm(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, ALGORITHM.clone()) > 0;
    res
}

pub(crate) fn inEquation(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, EQUATION.clone()) > 0;
    res
}

pub(crate) fn inInitial(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, INITIAL.clone()) > 0;
    res
}

pub(crate) fn onLHS(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, LHS.clone()) > 0;
    res
}

pub(crate) fn onRHS(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, RHS.clone()) > 0;
    res
}

pub(crate) fn inWhen(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, WHEN.clone()) > 0;
    res
}

pub(crate) fn inClocked(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, CLOCKED.clone()) > 0;
    res
}

pub(crate) fn inFor(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, FOR.clone()) > 0;
    res
}

pub(crate) fn inIf(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, IF.clone()) > 0;
    res
}

pub(crate) fn inWhile(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, WHILE.clone()) > 0;
    res
}

pub(crate) fn inNonexpandable(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, NONEXPANDABLE.clone()) > 0;
    res
}

pub(crate) fn inIterationRange(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, ITERATION_RANGE.clone()) > 0;
    res
}

pub(crate) fn inDimension(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, DIMENSION.clone()) > 0;
    res
}

pub(crate) fn inBinding(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, BINDING.clone()) > 0;
    res
}

pub(crate) fn inCondition(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, CONDITION.clone()) > 0;
    res
}

pub(crate) fn inSubscript(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, SUBSCRIPT.clone()) > 0;
    res
}

pub(crate) fn inSubexpression(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, SUBEXPRESSION.clone()) > 0;
    res
}

pub(crate) fn inConnect(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, CONNECT.clone()) > 0;
    res
}

pub(crate) fn inNoEvent(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, NOEVENT.clone()) > 0;
    res
}

pub(crate) fn inAssert(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, ASSERT.clone()) > 0;
    res
}

pub(crate) fn inAnnotation(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, ANNOTATION.clone()) > 0;
    res
}

pub(crate) fn inValidTypenameScope(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, intBitOr(ITERATION_RANGE.clone(), DIMENSION.clone())) > 0;
    res
}

pub(crate) fn inDiscreteScope(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, intBitOr(WHEN.clone(), intBitOr(INITIAL.clone(), FUNCTION.clone()))) > 0;
    res
}

pub(crate) fn inLoop(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, intBitOr(FOR.clone(), WHILE.clone())) > 0;
    res
}

pub(crate) fn inValidWhenScope(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context, intBitOr(intBitOr(FUNCTION.clone(), WHILE.clone()), intBitOr(IF.clone(), intBitOr(FOR.clone(), WHEN.clone())))) == 0;
    res
}

pub(crate) fn isSingleExpression(mut context: Type) -> bool {
    let mut isSingle: bool = context < ITERATION_RANGE.clone() - 1;
    isSingle
}

pub(crate) fn nodeContext(mut node: Arc<InstNode::InstNode>, mut currentContext: Type) -> Type {
    let mut nodeContext: Type;
    let mut parent: Arc<InstNode::InstNode>;
    let mut parent_res: Arc<Restriction::NFRestriction>;
    nodeContext = clearScopeFlags(currentContext);
    parent = InstNode::explicitParent(node);
    if !(InstNode::isRootClass(parent.clone())) {
        nodeContext = set(nodeContext, CLASS.clone());
        return nodeContext.clone();
    }
    parent_res = InstNode::restriction(parent);
    nodeContext = if (Restriction::isFunction(parent_res.clone()) || Restriction::isRecord(parent_res)) {set(nodeContext, FUNCTION.clone())} else {set(nodeContext, CLASS.clone())};
    nodeContext
}

