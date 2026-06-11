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

use crate::BaseModelica;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableKind;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind as ClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction as Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::JSON;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFExpression {
    INTEGER {
        value: i32,
    },
    REAL {
        value: metamodelica::Real,
    },
    STRING {
        value: ArcStr,
    },
    BOOLEAN {
        value: bool,
    },
    ENUM_LITERAL {
        ty: Arc<Type::NFType>,
        name: ArcStr,
        index: i32,
    },
    /// Clock constructors
    CLKCONST {
        /// Clock kinds
        clk: Arc<ClockKind::NFClockKind>,
    },
    CREF {
        ty: Arc<Type::NFType>,
        cref: Arc<ComponentRef::NFComponentRef>,
    },
    /// Represents a type used as a range, e.g. Boolean.
    TYPENAME {
        ty: Arc<Type::NFType>,
    },
    ARRAY {
        ty: Arc<Type::NFType>,
        elements: metamodelica::Array<Arc<NFExpression>>,
        /// True if the array is known to only contain literal expressions.
        literal: bool,
    },
    /// The array concatentation operator [a,b; c,d]; this should be removed during type-checking
    MATRIX {
        elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>>,
    },
    RANGE {
        ty: Arc<Type::NFType>,
        start: Arc<NFExpression>,
        step: Option<Arc<NFExpression>>,
        stop: Arc<NFExpression>,
    },
    TUPLE {
        ty: Arc<Type::NFType>,
        elements: Arc<metamodelica::List<Arc<NFExpression>>>,
    },
    RECORD {
        path: Arc<Path>,
        ty: Arc<Type::NFType>,
        elements: Arc<metamodelica::List<Arc<NFExpression>>>,
    },
    CALL {
        call: Arc<Call::NFCall>,
    },
    SIZE {
        exp: Arc<NFExpression>,
        dimIndex: Option<Arc<NFExpression>>,
    },
    END,
    /// Binary operations, e.g. a+4
    BINARY {
        exp1: Arc<NFExpression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<NFExpression>,
    },
    /// Unary operations, -(4x)
    UNARY {
        operator: Arc<Operator::NFOperator>,
        exp: Arc<NFExpression>,
    },
    /// Logical binary operations: and, or
    LBINARY {
        exp1: Arc<NFExpression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<NFExpression>,
    },
    /// Logical unary operations: not
    LUNARY {
        operator: Arc<Operator::NFOperator>,
        exp: Arc<NFExpression>,
    },
    /// Relation, e.g. a <= 0
    RELATION {
        exp1: Arc<NFExpression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<NFExpression>,
        /// index for event codegen
        index: i32,
    },
    /// Multary expressions with the same operator, e.g. a+b+c
    ///    An empty list has to be interpreted as the neutral element of the operator space
    MULTARY {
        /// arguments that are chained with the operator (+, *)
        arguments: Arc<metamodelica::List<Arc<NFExpression>>>,
        /// arguments that are chained with the inverse operator (-, :)
        inv_arguments: Arc<metamodelica::List<Arc<NFExpression>>>,
        /// Can only be + or * (commutative)
        operator: Arc<Operator::NFOperator>,
    },
    IF {
        ty: Arc<Type::NFType>,
        condition: Arc<NFExpression>,
        trueBranch: Arc<NFExpression>,
        falseBranch: Arc<NFExpression>,
    },
    CAST {
        ty: Arc<Type::NFType>,
        exp: Arc<NFExpression>,
    },
    /// MetaModelica boxed value
    BOX {
        exp: Arc<NFExpression>,
    },
    /// MetaModelica value unboxing (similar to a cast)
    UNBOX {
        exp: Arc<NFExpression>,
        ty: Arc<Type::NFType>,
    },
    SUBSCRIPTED_EXP {
        exp: Arc<NFExpression>,
        subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>,
        ty: Arc<Type::NFType>,
        split: bool,
    },
    TUPLE_ELEMENT {
        tupleExp: Arc<NFExpression>,
        index: i32,
        ty: Arc<Type::NFType>,
    },
    RECORD_ELEMENT {
        recordExp: Arc<NFExpression>,
        index: i32,
        fieldName: ArcStr,
        ty: Arc<Type::NFType>,
    },
    MUTABLE {
        exp: Mutable::Mutable<Arc<NFExpression>>,
    },
    EMPTY {
        ty: Arc<Type::NFType>,
    },
    PARTIAL_FUNCTION_APPLICATION {
        r#fn: Arc<ComponentRef::NFComponentRef>,
        args: Arc<metamodelica::List<Arc<NFExpression>>>,
        argNames: Arc<metamodelica::List<ArcStr>>,
        ty: Arc<Type::NFType>,
    },
    FILENAME {
        filename: ArcStr,
    },
    /// Before code generation, we make a pass that replaces constant literals
    ///    with a SHARED_LITERAL expression. Any immutable type can be shared:
    ///    basic MetaModelica types and Modelica strings are fine. There is no point
    ///    to share Real, Integer, Boolean or Enum though.
    SHARED_LITERAL {
        /// A unique indexing that can be used to point to a single shared literal in generated code
        index: i32,
        /// For printing strings, code generators that do not support this kind of literal, or for getting the type in case the code generator needs that
        exp: Arc<NFExpression>,
    },
    INSTANCE_NAME {
        scope: Arc<InstNode::InstNode>,
    },
}
impl metamodelica::gc::MMTrace for NFExpression {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFExpression::INTEGER { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::REAL { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::STRING { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::BOOLEAN { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::ENUM_LITERAL { ty, name, index } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            NFExpression::CLKCONST { clk } => {
                metamodelica::gc::MMTrace::mm_accept(clk, __mmv)?;
                Ok(())
            }
            NFExpression::CREF { ty, cref } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                Ok(())
            }
            NFExpression::TYPENAME { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::ARRAY { ty, elements, literal } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(literal, __mmv)?;
                Ok(())
            }
            NFExpression::MATRIX { elements } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                Ok(())
            }
            NFExpression::RANGE { ty, start, step, stop } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(step, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stop, __mmv)?;
                Ok(())
            }
            NFExpression::TUPLE { ty, elements } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                Ok(())
            }
            NFExpression::RECORD { path, ty, elements } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                Ok(())
            }
            NFExpression::CALL { call } => {
                metamodelica::gc::MMTrace::mm_accept(call, __mmv)?;
                Ok(())
            }
            NFExpression::SIZE { exp, dimIndex } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimIndex, __mmv)?;
                Ok(())
            }
            NFExpression::END => Ok(()),
            NFExpression::BINARY { exp1, operator, exp2 } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                Ok(())
            }
            NFExpression::UNARY { operator, exp } => {
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::LBINARY { exp1, operator, exp2 } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                Ok(())
            }
            NFExpression::LUNARY { operator, exp } => {
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::RELATION { exp1, operator, exp2, index } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            NFExpression::MULTARY { arguments, inv_arguments, operator } => {
                metamodelica::gc::MMTrace::mm_accept(arguments, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inv_arguments, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                Ok(())
            }
            NFExpression::IF { ty, condition, trueBranch, falseBranch } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(trueBranch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(falseBranch, __mmv)?;
                Ok(())
            }
            NFExpression::CAST { ty, exp } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::BOX { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::UNBOX { exp, ty } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::SUBSCRIPTED_EXP { exp, subscripts, ty, split } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subscripts, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(split, __mmv)?;
                Ok(())
            }
            NFExpression::TUPLE_ELEMENT { tupleExp, index, ty } => {
                metamodelica::gc::MMTrace::mm_accept(tupleExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::RECORD_ELEMENT { recordExp, index, fieldName, ty } => {
                metamodelica::gc::MMTrace::mm_accept(recordExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fieldName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::MUTABLE { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::EMPTY { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::PARTIAL_FUNCTION_APPLICATION { r#fn, args, argNames, ty } => {
                metamodelica::gc::MMTrace::mm_accept(r#fn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(argNames, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::FILENAME { filename } => {
                metamodelica::gc::MMTrace::mm_accept(filename, __mmv)?;
                Ok(())
            }
            NFExpression::SHARED_LITERAL { index, exp } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::INSTANCE_NAME { scope } => {
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                Ok(())
            }
        }
    }
}
impl NFExpression {
    pub fn interned_END() -> Arc<NFExpression> {
        thread_local! {
            static INTERNED: Arc<NFExpression> = Arc::new(NFExpression::END);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_END() -> Arc<NFExpression> { NFExpression::interned_END() }
impl Default for NFExpression {
    fn default() -> Self { Self::END }
}
pub use self::NFExpression::{INTEGER,REAL,STRING,BOOLEAN,ENUM_LITERAL,CLKCONST,CREF,TYPENAME,ARRAY,MATRIX,RANGE,TUPLE,RECORD,CALL,SIZE,END,BINARY,UNARY,LBINARY,LUNARY,RELATION,MULTARY,IF,CAST,BOX,UNBOX,SUBSCRIPTED_EXP,TUPLE_ELEMENT,RECORD_ELEMENT,MUTABLE,EMPTY,PARTIAL_FUNCTION_APPLICATION,FILENAME,SHARED_LITERAL,INSTANCE_NAME};
pub fn isArray(mut exp: Arc<NFExpression>) -> bool {
    let mut isArray: bool;
    isArray = (::match_deref::match_deref! { match &(exp) {
        Deref @ ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

pub(crate) fn isEmptyArray(mut exp: Arc<NFExpression>) -> bool {
    let mut emptyArray: bool;
    emptyArray = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    emptyArray
}

pub(crate) fn isVector(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => Type::isVector(var_field!((*exp).ty, NFExpression::ARRAY).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isCref(mut exp: Arc<NFExpression>) -> bool {
    let mut isCref: bool;
    isCref = (::match_deref::match_deref! { match &(exp) {
        Deref @ CREF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCref
}

pub(crate) fn isFunctionInputCref(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isInput(ComponentRef::last(var_field!((*exp).cref, NFExpression::CREF).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isWildCref(mut exp: Arc<NFExpression>) -> bool {
    let mut wild: bool;
    wild = (::match_deref::match_deref! { match &(exp) {
        Deref @ CREF { cref: Deref @ ComponentRef::WILD, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    wild
}

pub fn isCall(mut exp: Arc<NFExpression>) -> bool {
    let mut isCall: bool;
    isCall = (::match_deref::match_deref! { match &(exp) {
        Deref @ CALL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCall
}

pub(crate) fn isImpureCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isImpure: bool;
    isImpure = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isImpure(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isImpure)
}

pub(crate) fn isExternalCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isExternal(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isCallNamed(mut exp: Arc<NFExpression>, mut name: ArcStr) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isNamed(var_field!((*exp).call, NFExpression::CALL).clone(), (name).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn isConnectionCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isConnection: bool;
    isConnection = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isConnectionsOperator(var_field!((*exp).call, NFExpression::CALL).clone())? || Call::isStreamOperator(var_field!((*exp).call, NFExpression::CALL).clone())? || Call::isCardinality(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isConnection)
}

pub fn isTrue(mut exp: Arc<NFExpression>) -> bool {
    let mut isTrue: bool;
    isTrue = (::match_deref::match_deref! { match &(exp) {
        Deref @ BOOLEAN { value: true } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTrue
}

pub fn isAllTrue(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { value: true } => {
            return Ok(true)
        },
        Deref @ ARRAY { .. } => {
            return Ok(Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isAllTrue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?)
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: e, .. } } => {
            { exp = e.clone(); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isFalse(mut exp: Arc<NFExpression>) -> bool {
    let mut isTrue: bool;
    isTrue = (::match_deref::match_deref! { match &(exp) {
        Deref @ BOOLEAN { value: false } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTrue
}

pub fn isTrivialCref(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp) {
        Deref @ CREF { .. } => true,
        Deref @ UNARY { exp: Deref @ CREF { .. }, .. } => true,
        Deref @ LUNARY { exp: Deref @ CREF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hash(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut hash: i32 = hashContinue(exp.clone(), Util::HASH_SEED.clone())?;
    Ok(hash)
}

pub(crate) fn hashContinue(mut exp: Arc<NFExpression>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            stringHashDjb2Continue((intString(var_field!((*exp).value, NFExpression::INTEGER).clone())).clone(), hash)
        },
        Deref @ REAL { .. } => {
            stringHashDjb2Continue((realString(var_field!((*exp).value, NFExpression::REAL).clone())).clone(), hash)
        },
        Deref @ STRING { .. } => {
            stringHashDjb2Continue((var_field!((*exp).value, NFExpression::STRING).clone()).clone(), hash)
        },
        Deref @ BOOLEAN { .. } => {
            stringHashDjb2Continue((boolString(var_field!((*exp).value, NFExpression::BOOLEAN).clone())).clone(), hash)
        },
        Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { typePath: path, .. }, .. } => {
            hash = AbsynUtil::pathHashContinue(path.clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(".")).clone(), hash);
            hash = stringHashDjb2Continue((var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()).clone(), hash);
            hash
        },
        Deref @ CLKCONST { .. } => {
            ClockKind::hashContinue(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), hash)?
        },
        Deref @ CREF { .. } => {
            ComponentRef::hashContinue(var_field!((*exp).cref, NFExpression::CREF).clone(), false, hash)?
        },
        Deref @ TYPENAME { .. } => {
            Type::hashContinue(Type::arrayElementType(var_field!((*exp).ty, NFExpression::TYPENAME).clone()), hash)?
        },
        Deref @ ARRAY { .. } => {
            hash = stringHashDjb2Continue((literal!("{")).clone(), hash);
            let __range0 = var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                hash = hashContinue(e.clone(), hash)?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!("}")).clone(), hash);
            hash
        },
        Deref @ MATRIX { .. } => {
            hash = stringHashDjb2Continue((literal!("[")).clone(), hash);
            for mut el in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut el = el.clone();
                for mut e in &*el.clone() {
                    let mut e = e.clone();
                    hash = hashContinue(e.clone(), hash)?;
                    hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
                }
                hash = stringHashDjb2Continue((literal!("; ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash);
            hash
        },
        Deref @ RANGE { .. } => {
            hash = hashContinue(var_field!((*exp).start, NFExpression::RANGE).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(":")).clone(), hash);
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                hash = hashContinue(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, hash)?;
                hash = stringHashDjb2Continue((literal!(":")).clone(), hash);
            }
            hash = hashContinue(var_field!((*exp).stop, NFExpression::RANGE).clone(), hash)?;
            hash
        },
        Deref @ TUPLE { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            for mut e in &*var_field!((*exp).elements, NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                hash = hashContinue(e.clone(), hash)?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ RECORD { .. } => {
            hash = AbsynUtil::pathHashContinue(var_field!((*exp).path, NFExpression::RECORD).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            for mut e in &*var_field!((*exp).elements, NFExpression::RECORD).clone() {
                let mut e = e.clone();
                hash = hashContinue(e.clone(), hash)?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ CALL { .. } => {
            stringHashDjb2Continue((Call::toString(var_field!((*exp).call, NFExpression::CALL).clone())?).clone(), hash)
        },
        Deref @ SIZE { .. } => {
            hash = stringHashDjb2Continue((literal!("size(")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::SIZE).clone(), hash)?;
            if isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()) {
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
                hash = hashContinue(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?, hash)?;
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ END { .. } => {
            stringHashDjb2Continue((literal!("end")).clone(), hash)
        },
        Deref @ BINARY { .. } => {
            hash = hashContinue(var_field!((*exp).exp1, NFExpression::BINARY).clone(), hash)?;
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::BINARY).clone(), (literal!(" ")).clone())?).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp2, NFExpression::BINARY).clone(), hash)?;
            hash
        },
        Deref @ UNARY { .. } => {
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::UNARY).clone(), (literal!("")).clone())?).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::UNARY).clone(), hash)?;
            hash
        },
        Deref @ LBINARY { .. } => {
            hash = hashContinue(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), hash)?;
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::LBINARY).clone(), (literal!(" ")).clone())?).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), hash)?;
            hash
        },
        Deref @ LUNARY { .. } => {
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::LUNARY).clone(), (literal!("")).clone())?).clone(), hash);
            hash = stringHashDjb2Continue((literal!(" ")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::LUNARY).clone(), hash)?;
            hash
        },
        Deref @ RELATION { .. } => {
            hash = hashContinue(var_field!((*exp).exp1, NFExpression::RELATION).clone(), hash)?;
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::RELATION).clone(), (literal!(" ")).clone())?).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp2, NFExpression::RELATION).clone(), hash)?;
            hash
        },
        Deref @ MULTARY { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            for mut e in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut e = e.clone();
                hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::MULTARY).clone(), (literal!(" ")).clone())?).clone(), hash);
                hash = hashContinue(e.clone(), hash)?;
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash = stringHashDjb2Continue((Operator::symbol(Operator::invert(var_field!((*exp).operator, NFExpression::MULTARY).clone())?, (literal!(" ")).clone())?).clone(), hash);
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            for mut e in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut e = e.clone();
                hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::MULTARY).clone(), (literal!(" ")).clone())?).clone(), hash);
                hash = hashContinue(e.clone(), hash)?;
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ IF { .. } => {
            hash = stringHashDjb2Continue((literal!("if ")).clone(), hash);
            hash = hashContinue(var_field!((*exp).condition, NFExpression::IF).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(" then ")).clone(), hash);
            hash = hashContinue(var_field!((*exp).trueBranch, NFExpression::IF).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(" else ")).clone(), hash);
            hash = hashContinue(var_field!((*exp).falseBranch, NFExpression::IF).clone(), hash)?;
            hash
        },
        Deref @ CAST { .. } => {
            hash = stringHashDjb2Continue((literal!("CAST(")).clone(), hash);
            hash = Type::hashContinue(var_field!((*exp).ty, NFExpression::CAST).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::CAST).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ BOX { .. } => {
            hash = stringHashDjb2Continue((literal!("BOX(")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::BOX).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ UNBOX { .. } => {
            hash = stringHashDjb2Continue((literal!("UNBOX(")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::UNBOX).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(")[")).clone(), hash);
            for mut sub in &*var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut sub = sub.clone();
                hash = Subscript::hashContinue(sub.clone(), hash)?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash);
            hash
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            hash = hashContinue(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!("[")).clone(), hash);
            hash = stringHashDjb2Continue((intString(var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone())).clone(), hash);
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash);
            hash
        },
        Deref @ RECORD_ELEMENT { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            hash = hashContinue(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(").")).clone(), hash);
            hash = stringHashDjb2Continue((var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), hash);
            hash
        },
        Deref @ MUTABLE { .. } => {
            hashContinue(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), hash)?
        },
        Deref @ EMPTY { .. } => {
            stringHashDjb2Continue((literal!("#EMPTY#")).clone(), hash)
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            hash = stringHashDjb2Continue((literal!("function ")).clone(), hash);
            hash = ComponentRef::hashContinue(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), false, hash)?;
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash);
            for mut n in &*var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut n = n.clone();
                hash = stringHashDjb2Continue((n.clone()).clone(), hash);
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!(" = ")).clone(), hash);
            for mut a in &*var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut a = a.clone();
                hash = hashContinue(a.clone(), hash)?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ FILENAME { .. } => {
            stringHashDjb2Continue((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone(), hash)
        },
        Deref @ SHARED_LITERAL { .. } => {
            hash = stringHashDjb2Continue((literal!("LITERAL(")).clone(), hash);
            hash = stringHashDjb2Continue((intString(var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone())).clone(), hash);
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash);
            hash = hashContinue(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), hash)?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash);
            hash
        },
        Deref @ INSTANCE_NAME { .. } => {
            stringHashDjb2Continue((literal!("getInstanceName()")).clone(), hash)
        },
        _ => {
            hash
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}

pub fn isEqual(mut exp1: Arc<NFExpression>, mut exp2: Arc<NFExpression>) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = 0 == compare(exp1, exp2)?;
    Ok(isEqual)
}

pub fn compare(mut exp1: Arc<NFExpression>, mut exp2: Arc<NFExpression>) -> Result<i32> {
    let mut comp: i32;
    if referenceEq(&*(exp1.clone()),&*(exp2.clone())) {
        comp = 0;
        return Ok(comp.clone());
    }
    comp = Util::intCompare(metamodelica::valueConstructor((&*exp1.clone()))?, metamodelica::valueConstructor((&*exp2.clone()))?);
    if comp != 0 {
        return Ok(comp.clone());
    }
    comp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ INTEGER { .. } => {
            let mut i: i32;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ INTEGER { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            Util::intCompare(var_field!((*exp1).value, NFExpression::INTEGER).clone(), i)
        },
        Deref @ REAL { .. } => {
            let mut r: metamodelica::Real;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ REAL { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            Util::realCompare(var_field!((*exp1).value, NFExpression::REAL).clone(), r)
        },
        Deref @ STRING { .. } => {
            let mut s: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ STRING { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            s = __pa0.clone();
            stringCompare((var_field!((*exp1).value, NFExpression::STRING).clone()).clone(), (s).clone())
        },
        Deref @ BOOLEAN { .. } => {
            let mut b: bool;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ BOOLEAN { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            b = __pa0.clone();
            Util::boolCompare(var_field!((*exp1).value, NFExpression::BOOLEAN).clone(), b)
        },
        Deref @ ENUM_LITERAL { .. } => {
            let mut i: i32;
            let mut ty: Arc<Type::NFType>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ ENUM_LITERAL { ty: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            i = __pa1.clone();
            comp = AbsynUtil::pathCompare(Type::enumName(var_field!((*exp1).ty, NFExpression::ENUM_LITERAL).clone())?, Type::enumName(ty)?)?;
            if comp == 0 {
                comp = Util::intCompare(var_field!((*exp1).index, NFExpression::ENUM_LITERAL).clone(), i);
            }
            comp
        },
        Deref @ CLKCONST { .. } => {
            let mut clk: Arc<ClockKind::NFClockKind>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ CLKCONST { clk: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            clk = __pa0.clone();
            ClockKind::compare(var_field!((*exp1).clk, NFExpression::CLKCONST).clone(), clk)?
        },
        Deref @ CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            ComponentRef::compare(var_field!((*exp1).cref, NFExpression::CREF).clone(), cr)?
        },
        Deref @ TYPENAME { .. } => {
            let mut ty: Arc<Type::NFType>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ TYPENAME { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            valueCompare(var_field!((*exp1).ty, NFExpression::TYPENAME).clone(), ty)
        },
        Deref @ ARRAY { .. } => {
            let mut ty: Arc<Type::NFType>;
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ ARRAY { ty: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            arr = __pa1.clone();
            comp = valueCompare(ty, var_field!((*exp1).ty, NFExpression::ARRAY).clone());
            if (comp == 0) {Array::compare(var_field!((*exp1).elements, NFExpression::ARRAY).clone(), arr.clone(), (std::sync::Arc::new(compare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<i32> + 'static>))?} else {comp}
        },
        Deref @ MATRIX { .. } => {
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ MATRIX { elements: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            mat = __pa0.clone();
            List::compare(var_field!((*exp1).elements, NFExpression::MATRIX).clone(), mat, (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static> = (std::sync::Arc::new(compare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<i32> + 'static>); move |__pe_a0, __pe_a1| List::compare(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>))?
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut oe: Option<Arc<NFExpression>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            oe = __pa1.clone();
            e2 = __pa2.clone();
            comp = compare(var_field!((*exp1).start, NFExpression::RANGE).clone(), e1)?;
            if comp == 0 {
                comp = compare(var_field!((*exp1).stop, NFExpression::RANGE).clone(), e2)?;
                if comp == 0 {
                    comp = compareOpt(var_field!((*exp1).step, NFExpression::RANGE).clone(), oe)?;
                }
            }
            comp
        },
        Deref @ TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ TUPLE { elements: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            List::compare(var_field!((*exp1).elements, NFExpression::TUPLE).clone(), expl, (std::sync::Arc::new(compare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<i32> + 'static>))?
        },
        Deref @ RECORD { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            let mut p: Arc<Path>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RECORD { path: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            expl = __pa1.clone();
            comp = AbsynUtil::pathCompare(var_field!((*exp1).path, NFExpression::RECORD).clone(), p)?;
            if (comp == 0) {List::compare(var_field!((*exp1).elements, NFExpression::RECORD).clone(), expl, (std::sync::Arc::new(compare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<i32> + 'static>))?} else {comp}
        },
        Deref @ CALL { .. } => {
            let mut c: Arc<Call::NFCall>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ CALL { call: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            c = __pa0.clone();
            Call::compare(var_field!((*exp1).call, NFExpression::CALL).clone(), c)?
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut oe: Option<Arc<NFExpression>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ SIZE { exp: __pa0, dimIndex: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            oe = __pa1.clone();
            comp = compareOpt(var_field!((*exp1).dimIndex, NFExpression::SIZE).clone(), oe)?;
            if (comp == 0) {compare(var_field!((*exp1).exp, NFExpression::SIZE).clone(), e1)?} else {comp}
        },
        Deref @ END { .. } => {
            0
        },
        Deref @ MULTARY { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            let mut inv_expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ MULTARY { arguments: __pa0, inv_arguments: __pa1, operator: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            inv_expl = __pa1.clone();
            op = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::MULTARY).clone(), op);
            if comp == 0 {
                comp = compareList(var_field!((*exp1).arguments, NFExpression::MULTARY).clone(), expl)?;
            }
            if comp == 0 {
                comp = compareList(var_field!((*exp1).inv_arguments, NFExpression::MULTARY).clone(), inv_expl)?;
            }
            comp
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            op = __pa1.clone();
            e2 = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::BINARY).clone(), op);
            if comp == 0 {
                comp = compare(var_field!((*exp1).exp1, NFExpression::BINARY).clone(), e1)?;
                if comp == 0 {
                    comp = compare(var_field!((*exp1).exp2, NFExpression::BINARY).clone(), e2)?;
                }
            }
            comp
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ UNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            e1 = __pa1.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::UNARY).clone(), op);
            if (comp == 0) {compare(var_field!((*exp1).exp, NFExpression::UNARY).clone(), e1)?} else {comp}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            op = __pa1.clone();
            e2 = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::LBINARY).clone(), op);
            if comp == 0 {
                comp = compare(var_field!((*exp1).exp1, NFExpression::LBINARY).clone(), e1)?;
                if comp == 0 {
                    comp = compare(var_field!((*exp1).exp2, NFExpression::LBINARY).clone(), e2)?;
                }
            }
            comp
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ LUNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            e1 = __pa1.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::LUNARY).clone(), op);
            if (comp == 0) {compare(var_field!((*exp1).exp, NFExpression::LUNARY).clone(), e1)?} else {comp}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            op = __pa1.clone();
            e2 = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::RELATION).clone(), op);
            if comp == 0 {
                comp = compare(var_field!((*exp1).exp1, NFExpression::RELATION).clone(), e1)?;
                if comp == 0 {
                    comp = compare(var_field!((*exp1).exp2, NFExpression::RELATION).clone(), e2)?;
                }
            }
            comp
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ IF { condition: __pa0, trueBranch: __pa1, falseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            e2 = __pa1.clone();
            e3 = __pa2.clone();
            comp = compare(var_field!((*exp1).condition, NFExpression::IF).clone(), e1)?;
            if comp == 0 {
                comp = compare(var_field!((*exp1).trueBranch, NFExpression::IF).clone(), e2)?;
                if comp == 0 {
                    comp = compare(var_field!((*exp1).falseBranch, NFExpression::IF).clone(), e3)?;
                }
            }
            comp
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = (::match_deref::match_deref! { match &(exp2) {
        Deref @ CAST { exp: __esc_e1, .. } => {
            e1 = (*__esc_e1).clone();
            e1.clone()
        },
        __esc_e1 => {
            e1 = (*__esc_e1).clone();
            e1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            compare(var_field!((*exp1).exp, NFExpression::CAST).clone(), e1)?
        },
        Deref @ BOX { .. } => {
            let mut e2: Arc<NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ BOX { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            compare(var_field!((*exp1).exp, NFExpression::BOX).clone(), e2)?
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            compare(var_field!((*exp1).exp, NFExpression::UNBOX).clone(), e1)?
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            subs = __pa1.clone();
            comp = compare(var_field!((*exp1).exp, NFExpression::SUBSCRIPTED_EXP).clone(), e1)?;
            if comp == 0 {
                comp = Subscript::compareList(var_field!((*exp1).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), subs)?;
            }
            comp
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut i: i32;
            let mut e1: Arc<NFExpression>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ TUPLE_ELEMENT { tupleExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            i = __pa1.clone();
            comp = Util::intCompare(var_field!((*exp1).index, NFExpression::TUPLE_ELEMENT).clone(), i);
            if comp == 0 {
                comp = compare(var_field!((*exp1).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), e1)?;
            }
            comp
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut i: i32;
            let mut e1: Arc<NFExpression>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RECORD_ELEMENT { recordExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            i = __pa1.clone();
            comp = Util::intCompare(var_field!((*exp1).index, NFExpression::RECORD_ELEMENT).clone(), i);
            if comp == 0 {
                comp = compare(var_field!((*exp1).recordExp, NFExpression::RECORD_ELEMENT).clone(), e1)?;
            }
            comp
        },
        Deref @ MUTABLE { .. } => {
            let mut me: Mutable::Mutable<Arc<NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            me = __pa0.clone();
            compare(Mutable::access(var_field!((*exp1).exp, NFExpression::MUTABLE).clone()), Mutable::access(me))?
        },
        Deref @ SHARED_LITERAL { .. } => {
            let mut e1: Arc<NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ SHARED_LITERAL { exp: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            compare(var_field!((*exp1).exp, NFExpression::SHARED_LITERAL).clone(), e1)?
        },
        Deref @ EMPTY { .. } => {
            let mut ty: Arc<Type::NFType>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ EMPTY { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            valueCompare(var_field!((*exp1).ty, NFExpression::EMPTY).clone(), ty)
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef>;
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ PARTIAL_FUNCTION_APPLICATION { r#fn: __pa0, args: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            expl = __pa1.clone();
            comp = ComponentRef::compare(var_field!((*exp1).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), cr)?;
            if comp == 0 {
                comp = List::compare(var_field!((*exp1).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), expl, (std::sync::Arc::new(compare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<i32> + 'static>))?;
            }
            comp
        },
        Deref @ FILENAME { .. } => {
            let mut s: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ FILENAME { filename: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            s = __pa0.clone();
            stringCompare((var_field!((*exp1).filename, NFExpression::FILENAME).clone()).clone(), (s).clone())
        },
        Deref @ INSTANCE_NAME { .. } => {
            let mut node: Arc<InstNode::InstNode>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ INSTANCE_NAME { scope: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            InstNode::refCompare(var_field!((*exp1).scope, NFExpression::INSTANCE_NAME).clone(), node)?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.compare")); __mm_s.push_str(&*literal!(" got unknown expression.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub(crate) fn compareOpt(mut expl1: Option<Arc<NFExpression>>, mut expl2: Option<Arc<NFExpression>>) -> Result<i32> {
    let mut comp: i32;
    let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
    comp = (::match_deref::match_deref! { match &((expl1, expl2)) {
        (None, None) => 0,
        (None, _) => -1,
        (_, None) => 1,
        (Some(__esc_e1), Some(__esc_e2)) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            compare(e1.clone(), e2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(comp)
}

pub(crate) fn compareList(mut expl1: Arc<metamodelica::List<Arc<NFExpression>>>, mut expl2: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<i32> {
    let mut comp: i32 = List::compare(expl1.clone(), expl2.clone(), (std::sync::Arc::new(compare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<i32> + 'static>))?;
    Ok(comp)
}

pub fn typeOf(mut exp: Arc<NFExpression>) -> Arc<Type::NFType> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return crate::NFType::interned_INTEGER(),
        Deref @ REAL { .. } => return crate::NFType::interned_REAL(),
        Deref @ STRING { .. } => return crate::NFType::interned_STRING(),
        Deref @ BOOLEAN { .. } => return crate::NFType::interned_BOOLEAN(),
        Deref @ ENUM_LITERAL { .. } => return var_field!((*exp).ty, NFExpression::ENUM_LITERAL).clone(),
        Deref @ CLKCONST { .. } => return crate::NFType::interned_CLOCK(),
        Deref @ CREF { .. } => return var_field!((*exp).ty, NFExpression::CREF).clone(),
        Deref @ TYPENAME { .. } => return var_field!((*exp).ty, NFExpression::TYPENAME).clone(),
        Deref @ ARRAY { .. } => return var_field!((*exp).ty, NFExpression::ARRAY).clone(),
        Deref @ RANGE { .. } => return var_field!((*exp).ty, NFExpression::RANGE).clone(),
        Deref @ TUPLE { .. } => return var_field!((*exp).ty, NFExpression::TUPLE).clone(),
        Deref @ RECORD { .. } => return var_field!((*exp).ty, NFExpression::RECORD).clone(),
        Deref @ CALL { .. } => return Call::typeOf(var_field!((*exp).call, NFExpression::CALL).clone()),
        Deref @ SIZE { .. } => if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {return crate::NFType::interned_INTEGER()} else {return Type::sizeType(typeOf(var_field!((*exp).exp, NFExpression::SIZE).clone()))},
        Deref @ END { .. } => return crate::NFType::interned_INTEGER(),
        Deref @ MULTARY { .. } => return Operator::typeOf(var_field!((*exp).operator, NFExpression::MULTARY).clone()),
        Deref @ BINARY { .. } => return Operator::typeOf(var_field!((*exp).operator, NFExpression::BINARY).clone()),
        Deref @ UNARY { .. } => return Operator::typeOf(var_field!((*exp).operator, NFExpression::UNARY).clone()),
        Deref @ LBINARY { .. } => return Operator::typeOf(var_field!((*exp).operator, NFExpression::LBINARY).clone()),
        Deref @ LUNARY { .. } => return Operator::typeOf(var_field!((*exp).operator, NFExpression::LUNARY).clone()),
        Deref @ RELATION { .. } => return Type::copyDims(Operator::typeOf(var_field!((*exp).operator, NFExpression::RELATION).clone()), crate::NFType::interned_BOOLEAN()),
        Deref @ IF { .. } => return var_field!((*exp).ty, NFExpression::IF).clone(),
        Deref @ CAST { .. } => return var_field!((*exp).ty, NFExpression::CAST).clone(),
        Deref @ BOX { .. } => return Arc::new(Type::NFType::METABOXED { ty: typeOf(var_field!((*exp).exp, NFExpression::BOX).clone()) }),
        Deref @ UNBOX { .. } => return var_field!((*exp).ty, NFExpression::UNBOX).clone(),
        Deref @ SUBSCRIPTED_EXP { .. } => return var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(),
        Deref @ TUPLE_ELEMENT { .. } => return var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone(),
        Deref @ RECORD_ELEMENT { .. } => return var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone(),
        Deref @ MUTABLE { .. } => { exp = Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()); continue '__tco; },
        Deref @ SHARED_LITERAL { .. } => { exp = var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(); continue '__tco; },
        Deref @ EMPTY { .. } => return var_field!((*exp).ty, NFExpression::EMPTY).clone(),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => return var_field!((*exp).ty, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(),
        Deref @ FILENAME { .. } => return crate::NFType::interned_STRING(),
        Deref @ INSTANCE_NAME { .. } => return crate::NFType::interned_STRING(),
        _ => return crate::NFType::interned_UNKNOWN(),
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn sizeOf(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut sz: i32 = Type::sizeOf(typeOf(exp.clone()), false)?;
    Ok(sz)
}

pub(crate) fn sizeZero(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool;
    match '__try0: {
        b = 0 == unwrap_break_err!(sizeOf(exp.clone()), '__try0);
        Ok::<_, anyhow::Error>((b.clone(),))
    } {
        Ok((__try0_o0,)) => {
            b = __try0_o0;
        }
        Err(_) => {
            b = false;
        }
    }
    b
}

pub(crate) fn setType(mut ty: Arc<Type::NFType>, mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ENUM_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::ENUM_LITERAL; ty = ty);
            exp
        },
        Deref @ CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF; ty = ty);
            exp
        },
        Deref @ TYPENAME { .. } => {
            assign_variant_field!(exp => NFExpression::TYPENAME; ty = ty);
            exp
        },
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; ty = ty);
            exp
        },
        Deref @ RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = ty);
            exp
        },
        Deref @ TUPLE { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE; ty = ty);
            exp
        },
        Deref @ RECORD { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD; ty = ty);
            exp
        },
        Deref @ CALL { .. } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::setType(var_field!((*exp).call, NFExpression::CALL).clone(), ty)?);
            exp
        },
        Deref @ BINARY { .. } => {
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::setType(ty, var_field!((*exp).operator, NFExpression::BINARY).clone()));
            exp
        },
        Deref @ UNARY { .. } => {
            assign_variant_field!(exp => NFExpression::UNARY; operator = Operator::setType(ty, var_field!((*exp).operator, NFExpression::UNARY).clone()));
            exp
        },
        Deref @ LBINARY { .. } => {
            assign_variant_field!(exp => NFExpression::LBINARY; operator = Operator::setType(ty, var_field!((*exp).operator, NFExpression::LBINARY).clone()));
            exp
        },
        Deref @ LUNARY { .. } => {
            assign_variant_field!(exp => NFExpression::LUNARY; operator = Operator::setType(ty, var_field!((*exp).operator, NFExpression::LUNARY).clone()));
            exp
        },
        Deref @ RELATION { .. } => {
            assign_variant_field!(exp => NFExpression::RELATION; operator = Operator::setType(ty, var_field!((*exp).operator, NFExpression::RELATION).clone()));
            exp
        },
        Deref @ IF { .. } => {
            assign_variant_field!(exp => NFExpression::IF; ty = ty);
            exp
        },
        Deref @ CAST { .. } => {
            assign_variant_field!(exp => NFExpression::CAST; ty = ty);
            exp
        },
        Deref @ UNBOX { .. } => {
            assign_variant_field!(exp => NFExpression::UNBOX; ty = ty);
            exp
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            assign_variant_field!(exp => NFExpression::SUBSCRIPTED_EXP; ty = ty);
            exp
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE_ELEMENT; ty = ty);
            exp
        },
        Deref @ RECORD_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD_ELEMENT; ty = ty);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; ty = ty);
            exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn applyToType(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type typeFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;

    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ENUM_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::ENUM_LITERAL; ty = func(var_field!((*exp).ty, NFExpression::ENUM_LITERAL).clone())?);
            exp
        },
        Deref @ CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF;
                ty = func(var_field!((*exp).ty, NFExpression::CREF).clone())?,
                cref = ComponentRef::applyToType(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?
            );
            exp
        },
        Deref @ TYPENAME { .. } => {
            assign_variant_field!(exp => NFExpression::TYPENAME; ty = func(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?);
            exp
        },
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; ty = func(var_field!((*exp).ty, NFExpression::ARRAY).clone())?);
            exp
        },
        Deref @ RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = func(var_field!((*exp).ty, NFExpression::RANGE).clone())?);
            exp
        },
        Deref @ TUPLE { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE; ty = func(var_field!((*exp).ty, NFExpression::TUPLE).clone())?);
            exp
        },
        Deref @ RECORD { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD; ty = func(var_field!((*exp).ty, NFExpression::RECORD).clone())?);
            exp
        },
        Deref @ CALL { .. } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::setType(var_field!((*exp).call, NFExpression::CALL).clone(), func(Call::typeOf(var_field!((*exp).call, NFExpression::CALL).clone()))?)?);
            exp
        },
        Deref @ SIZE { .. } => {
            assign_variant_field!(exp => NFExpression::SIZE; exp = applyToType(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?);
            exp
        },
        Deref @ MULTARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::MULTARY; operator = o.clone());
            exp
        },
        Deref @ BINARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::BINARY; operator = o.clone());
            exp
        },
        Deref @ UNARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::UNARY; operator = o.clone());
            exp
        },
        Deref @ LBINARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::LBINARY; operator = o.clone());
            exp
        },
        Deref @ LUNARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::LUNARY; operator = o.clone());
            exp
        },
        Deref @ RELATION { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::RELATION; operator = o.clone());
            exp
        },
        Deref @ IF { .. } => {
            assign_variant_field!(exp => NFExpression::IF; ty = func(var_field!((*exp).ty, NFExpression::IF).clone())?);
            exp
        },
        Deref @ CAST { .. } => {
            assign_variant_field!(exp => NFExpression::CAST; ty = func(var_field!((*exp).ty, NFExpression::CAST).clone())?);
            exp
        },
        Deref @ BOX { .. } => {
            assign_variant_field!(exp => NFExpression::BOX; exp = applyToType(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?);
            exp
        },
        Deref @ UNBOX { .. } => {
            assign_variant_field!(exp => NFExpression::UNBOX; ty = func(var_field!((*exp).ty, NFExpression::UNBOX).clone())?);
            exp
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            assign_variant_field!(exp => NFExpression::SUBSCRIPTED_EXP; ty = func(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone())?);
            exp
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE_ELEMENT; ty = func(var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone())?);
            exp
        },
        Deref @ RECORD_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD_ELEMENT; ty = func(var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone())?);
            exp
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), applyToType(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?);
            exp
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = applyToType(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?);
            exp
        },
        Deref @ EMPTY { .. } => {
            assign_variant_field!(exp => NFExpression::EMPTY; ty = func(var_field!((*exp).ty, NFExpression::EMPTY).clone())?);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; ty = func(var_field!((*exp).ty, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?);
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn typeCastOpt(mut exp: Option<Arc<NFExpression>>, mut ty: Arc<Type::NFType>) -> Result<Option<Arc<NFExpression>>> {
    let mut outExp: Option<Arc<NFExpression>> = Util::applyOption(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = ty.clone(); move |__pe_a0| typeCast(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
    Ok(outExp)
}

pub(crate) fn typeCast(mut exp: Arc<NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ety: Arc<Type::NFType>;
    let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut arr: metamodelica::Array<Arc<NFExpression>> = Default::default();
    ety = Type::arrayElementType(ty.clone());
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => if (Type::isReal(ety.clone())?) {Arc::new(NFExpression::REAL { value: intReal(var_field!((*exp).value, NFExpression::INTEGER).clone()) })} else if (Type::isEnumeration(ety.clone()) && Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdIntegersAsEnumeration")).clone())?) {Arc::new(NFExpression::ENUM_LITERAL { ty: ety.clone(), name: (Type::nthEnumLiteral(ety, var_field!((*exp).value, NFExpression::INTEGER).clone())?).clone(), index: var_field!((*exp).value, NFExpression::INTEGER).clone() })} else {typeCastGeneric(exp, ety)?},
        Deref @ ENUM_LITERAL { .. } if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdEnumerationAsIntegers")).clone())?) => if (Type::isInteger(ety.clone())?) {Arc::new(NFExpression::INTEGER { value: toInteger(exp)? })} else {typeCastGeneric(exp, ety)?},
        Deref @ BOOLEAN { .. } => if (Type::isReal(ety.clone())? && Flags::isSet(Flags::NF_API.clone())?) {Arc::new(NFExpression::REAL { value: if (var_field!((*exp).value, NFExpression::BOOLEAN).clone()) {metamodelica::OrderedFloat(1.0_f64)} else {metamodelica::OrderedFloat(0.0_f64)} })} else {typeCastGeneric(exp, ety)?},
        Deref @ REAL { .. } => if (Type::isReal(ety.clone())?) {exp} else {typeCastGeneric(exp, ety)?},
        Deref @ ARRAY { ty: __esc_t, elements: __esc_arr, .. } => {
            t = (*__esc_t).clone();
            arr = (*__esc_arr).clone();
            arr = Array::map(arr.clone(), (std::sync::Arc::new({ let __pe_b1 = ety.clone(); move |__pe_a0| typeCast(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
            t = Type::setArrayElementType(t.clone(), ety);
            makeArray(t.clone(), arr.clone(), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ RANGE { ty: __esc_t, .. } => {
            t = (*__esc_t).clone();
            t = Type::setArrayElementType(t.clone(), ety.clone());
            Arc::new(NFExpression::RANGE { ty: t.clone(), start: typeCast(var_field!((*exp).start, NFExpression::RANGE).clone(), ety.clone())?, step: typeCastOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), ety.clone())?, stop: typeCast(var_field!((*exp).stop, NFExpression::RANGE).clone(), ety)? })
        },
        Deref @ UNARY { .. } => {
            t = Type::setArrayElementType(Operator::typeOf(var_field!((*exp).operator, NFExpression::UNARY).clone()), ety.clone());
            Arc::new(NFExpression::UNARY { operator: Operator::setType(t.clone(), var_field!((*exp).operator, NFExpression::UNARY).clone()), exp: typeCast(var_field!((*exp).exp, NFExpression::UNARY).clone(), ety)? })
        },
        Deref @ IF { .. } => {
            e1 = typeCast(var_field!((*exp).trueBranch, NFExpression::IF).clone(), ety.clone())?;
            e2 = typeCast(var_field!((*exp).falseBranch, NFExpression::IF).clone(), ety)?;
            t = if (Type::isConditionalArray(ty.clone())) {Type::setConditionalArrayTypes(ty, typeOf(e1.clone()), typeOf(e2.clone()))?} else {typeOf(e1.clone())};
            Arc::new(NFExpression::IF { ty: t.clone(), condition: var_field!((*exp).condition, NFExpression::IF).clone(), trueBranch: e1, falseBranch: e2 })
        },
        Deref @ CALL { .. } => Call::typeCast(exp, ety)?,
        Deref @ CAST { .. } => typeCast(var_field!((*exp).exp, NFExpression::CAST).clone(), ty)?,
        Deref @ SUBSCRIPTED_EXP { .. } => {
            e1 = typeCast(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), ety.clone())?;
            t = Type::setArrayElementType(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), ety);
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e1, subscripts: var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), ty: t.clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        _ => typeCastGeneric(exp, ety)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn typeCastGeneric(mut exp: Arc<NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut exp_ty: Arc<Type::NFType> = typeOf(exp.clone());
    if !(Type::isEqual(ty.clone(), Type::arrayElementType(exp_ty.clone()))?) {
        exp = Arc::new(NFExpression::CAST { ty: Type::setArrayElementType(exp_ty, ty), exp: exp });
    }
    Ok(exp)
}

pub fn realValue(mut exp: Arc<NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone(),
        Deref @ INTEGER { .. } => intReal(var_field!((*exp).value, NFExpression::INTEGER).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

pub(crate) fn makeReal(mut value: metamodelica::Real) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::REAL { value: value });
    exp
}

pub fn integerValue(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut value: i32;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ INTEGER { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        value = __pa1.clone();
        Ok::<_, anyhow::Error>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(__try0_err) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.integerValue")); __mm_s.push_str(&*literal!(" failed because expression is not an integer:\n")); __mm_s.push_str(&*toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            return Err(__try0_err);
        }
    }
    Ok(value)
}

pub fn integerValueOrDefault(mut exp: Arc<NFExpression>, mut value: i32) -> i32 {
    let mut value: i32 = value;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone(),
        _ => value,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub(crate) fn makeInteger(mut value: i32) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::INTEGER { value: value });
    exp
}

pub fn stringValue(mut exp: Arc<NFExpression>) -> ArcStr {
    let mut value: ArcStr;
    value = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ STRING { .. } => var_field!((*exp).value, NFExpression::STRING).clone(),
        Deref @ FILENAME { .. } => var_field!((*exp).filename, NFExpression::FILENAME).clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    value
}

pub fn booleanValue(mut exp: Arc<NFExpression>) -> bool {
    let mut value: bool;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ BOOLEAN { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        value = __pa1.clone();
        Ok::<_, anyhow::Error>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            value = false;
        }
    }
    value
}

pub fn makeArray(mut ty: Arc<Type::NFType>, mut expl: metamodelica::Array<Arc<NFExpression>>, mut literal: bool) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression>;
    outExp = Arc::new(NFExpression::ARRAY { ty: ty, elements: expl.clone(), literal: literal });
    outExp
}

pub fn makeArrayCheckLiteral(mut ty: Arc<Type::NFType>, mut expl: metamodelica::Array<Arc<NFExpression>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    outExp = Arc::new(NFExpression::ARRAY { ty: ty, elements: expl.clone(), literal: Array::all(expl.clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))? });
    Ok(outExp)
}

pub(crate) fn makeEmptyArray(mut ty: Arc<Type::NFType>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression>;
    outExp = Arc::new(NFExpression::ARRAY { ty: ty, elements: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), literal: true });
    outExp
}

pub(crate) fn makeIntegerArray(mut values: Arc<metamodelica::List<i32>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    exp = makeArray(Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![Dimension::fromInteger((values.clone().len() as i32), Prefixes::Variability::CONSTANT.clone())] }), Array::mapList(values, (std::sync::Arc::new(fnptr!(makeInteger, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<NFExpression>> + 'static>))?, true);
    Ok(exp)
}

pub(crate) fn makeRealArray(mut values: Arc<metamodelica::List<metamodelica::Real>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    exp = makeArray(Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_REAL(), dimensions: list![Dimension::fromInteger((values.clone().len() as i32), Prefixes::Variability::CONSTANT.clone())] }), Array::mapList(values, (std::sync::Arc::new(fnptr!(makeReal, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<Arc<NFExpression>> + 'static>))?, true);
    Ok(exp)
}

pub(crate) fn makeRealMatrix(mut values: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
    if values.clone().is_empty() {
        ty = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_REAL(), dimensions: list![Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone()), crate::NFDimension::interned_UNKNOWN()] });
        exp = makeEmptyArray(ty);
    } else {
        ty = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_REAL(), dimensions: list![Dimension::fromInteger((listHead(values.clone())?.len() as i32), Prefixes::Variability::CONSTANT.clone())] });
        expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut row in (values).into_iter().cloned() {
            let __x = makeArray(ty.clone(), metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut v in (row.clone()).into_iter().cloned() {
            let __x = Arc::new(NFExpression::REAL { value: v.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        ty = Type::liftArrayLeft(ty, Dimension::fromInteger((expl.clone().len() as i32), Prefixes::Variability::CONSTANT.clone()));
        exp = makeArray(ty, metamodelica::arrayFromVec(expl.into_iter().cloned().collect()), true);
    }
    Ok(exp)
}

pub fn makeExpArray(mut elements: metamodelica::Array<Arc<NFExpression>>, mut elementType: Arc<Type::NFType>, mut isLiteral: bool) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression>;
    let mut ty: Arc<Type::NFType>;
    ty = Type::liftArrayLeft(elementType, Dimension::fromInteger(metamodelica::arrayLength(elements.clone()), Prefixes::Variability::CONSTANT.clone()));
    exp = makeArray(ty, elements.clone(), isLiteral);
    exp
}

pub(crate) fn makeRecord(mut recordName: Arc<Path>, mut recordType: Arc<Type::NFType>, mut fields: Arc<metamodelica::List<Arc<NFExpression>>>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression>;
    exp = Arc::new(NFExpression::RECORD { path: recordName, ty: recordType, elements: fields });
    exp
}

pub fn makeRange(mut start: Arc<NFExpression>, mut step: Option<Arc<NFExpression>>, mut stop: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut rangeExp: Arc<NFExpression>;
    rangeExp = Arc::new(NFExpression::RANGE { ty: TypeCheck::getRangeType(start.clone(), step.clone(), stop.clone(), typeOf(start.clone()), Absyn::dummyInfo.clone())?, start: start, step: step, stop: stop });
    Ok(rangeExp)
}

pub(crate) fn makeIntegerRange(mut start: i32, mut step: i32, mut stop: i32) -> Result<Arc<NFExpression>> {
    let mut rangeExp: Arc<NFExpression>;
    let mut start_exp: Arc<NFExpression>;
    let mut stop_exp: Arc<NFExpression>;
    let mut step_exp: Option<Arc<NFExpression>>;
    start_exp = Arc::new(NFExpression::INTEGER { value: start });
    stop_exp = Arc::new(NFExpression::INTEGER { value: stop });
    if start == stop || step == 1 && start <= stop || step == -1 && start >= stop {
        step_exp = None;
    } else {
        step_exp = Some(Arc::new(NFExpression::INTEGER { value: step }));
    }
    rangeExp = makeRange(start_exp, step_exp, stop_exp)?;
    Ok(rangeExp)
}

pub fn getIntegerRange(mut range: Arc<NFExpression>, mut resize: bool) -> Result<(i32, i32, i32)> {
    let mut start: i32 = 0;
    let mut step: i32 = 0;
    let mut stop: i32 = 0;
    (start, step, stop) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ RANGE { .. } => {
            match '__try0: {
                start = unwrap_break_err!(getInteger(var_field!((*range).start, NFExpression::RANGE).clone(), resize), '__try0);
                stop = unwrap_break_err!(getInteger(var_field!((*range).stop, NFExpression::RANGE).clone(), resize), '__try0);
                if isSome(var_field!((*range).step, NFExpression::RANGE).clone()) {
                    step = unwrap_break_err!(getInteger(unwrap_break_err!(Util::getOption(var_field!((*range).step, NFExpression::RANGE).clone()), '__try0), resize), '__try0);
                } else {
                    step = if (start > stop) {-1} else {1};
                }
                Ok::<_, anyhow::Error>((start.clone(), step.clone(), stop.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    start = __try0_o0;
                    step = __try0_o1;
                    stop = __try0_o2;
                }
                Err(__try0_err) => {
                    Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.getIntegerRange")); __mm_s.push_str(&*literal!(" range could not be parsed to integer values: ")); __mm_s.push_str(&*toString(range.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
                    return Err(__try0_err);
                }
            }
            (start, step, stop)
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.getIntegerRange")); __mm_s.push_str(&*literal!(" expression not RANGE(): ")); __mm_s.push_str(&*toString(range)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start, step, stop))
}

pub fn getInteger(mut exp: Arc<NFExpression>, mut resize: bool) -> Result<i32> {
    let mut i: i32 = 0;
    let mut e: Arc<NFExpression>;
    if resize {
        e = map(exp.clone(), (std::sync::Arc::new(replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
    } else {
        e = map(exp.clone(), (std::sync::Arc::new(replaceResizableParameterWithOriginal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
    }
    i = (::match_deref::match_deref! { match &(SimplifyExp::simplify(e, false)?) {
        Deref @ INTEGER { value: __esc_i } => {
            i = (*__esc_i).clone();
            i.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.getInteger")); __mm_s.push_str(&*literal!(" cannot be parsed to an integer: ")); __mm_s.push_str(&*toString(exp)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub(crate) fn makeTuple(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<NFExpression>> {
    let mut tupleExp: Arc<NFExpression>;
    let mut tyl: Arc<metamodelica::List<Arc<Type::NFType>>>;
    if (expl.clone().len() as i32) == 1 {
        tupleExp = listHead(expl)?;
    } else {
        tyl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = typeOf(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        tupleExp = Arc::new(NFExpression::TUPLE { ty: Arc::new(Type::NFType::TUPLE { types: tyl, names: None }), elements: expl });
    }
    Ok(tupleExp)
}

pub fn rangeSize(mut range: Arc<NFExpression>, mut resize: bool) -> Result<i32> {
    let mut size: i32 = Dimension::size(Type::nthDimension(typeOf(range.clone()), 1)?, resize)?;
    Ok(size)
}

pub fn rangeSizeExp(mut range: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut size: Arc<NFExpression> = Dimension::sizeExp(Type::nthDimension(typeOf(range.clone()), 1)?)?;
    Ok(size)
}

pub fn applySubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut exp: Arc<NFExpression>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    if subscripts.clone().is_empty() {
        outExp = exp;
    } else {
        outExp = applySubscript(listHead(subscripts.clone())?, exp, listRest(subscripts)?, applyToScope)?;
    }
    Ok(outExp)
}

pub(crate) fn applySubscript(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => applySubscriptCref(subscript, var_field!((*exp).cref, NFExpression::CREF).clone(), restSubscripts.clone(), applyToScope)?,
        Deref @ TYPENAME { .. } if (restSubscripts.clone().is_empty()) => applySubscriptTypename(subscript, var_field!((*exp).ty, NFExpression::TYPENAME).clone())?,
        Deref @ ARRAY { .. } => applySubscriptArray(subscript, exp, restSubscripts.clone(), applyToScope)?,
        Deref @ RANGE { .. } if (restSubscripts.clone().is_empty()) => applySubscriptRange(subscript, exp)?,
        Deref @ CALL { .. } => applySubscriptCall(subscript, exp, restSubscripts.clone(), applyToScope)?,
        Deref @ IF { .. } => applySubscriptIf(subscript, exp, restSubscripts.clone(), applyToScope)?,
        Deref @ UNBOX { .. } => {
            outExp = applySubscript(subscript, var_field!((*exp).exp, NFExpression::UNBOX).clone(), restSubscripts.clone(), applyToScope)?;
            unbox(outExp)
        },
        Deref @ BOX { .. } => r#box(applySubscript(subscript, var_field!((*exp).exp, NFExpression::BOX).clone(), restSubscripts.clone(), applyToScope)?),
        Deref @ CAST { .. } => {
            outExp = applySubscript(subscript, var_field!((*exp).exp, NFExpression::CAST).clone(), restSubscripts.clone(), applyToScope)?;
            Arc::new(NFExpression::CAST { ty: Type::copyElementType(typeOf(outExp.clone()), var_field!((*exp).ty, NFExpression::CAST).clone()), exp: outExp })
        },
        _ => makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts.clone()), exp, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn applySubscriptCref(mut subscript: Arc<Subscript::NFSubscript>, mut cref: Arc<ComponentRef::NFComponentRef>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut cr: Arc<ComponentRef::NFComponentRef>;
    let mut ty: Arc<Type::NFType>;
    cr = ComponentRef::mergeSubscripts(metamodelica::cons(subscript, restSubscripts), cref, applyToScope, false, false)?;
    ty = ComponentRef::getSubscriptedType(cr.clone(), false)?;
    outExp = Arc::new(NFExpression::CREF { ty: ty, cref: cr });
    Ok(outExp)
}

pub(crate) fn applySubscriptTypename(mut subscript: Arc<Subscript::NFSubscript>, mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut sub: Arc<Subscript::NFSubscript>;
    let mut expl: metamodelica::Array<Arc<NFExpression>> = Default::default();
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => applyIndexSubscriptTypename(ty, sub)?,
        Deref @ Subscript::SLICE { .. } => Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: Arc::new(NFExpression::TYPENAME { ty: ty.clone() }), subscripts: list![subscript], ty: Arc::new(Type::NFType::ARRAY { elementType: ty, dimensions: list![Subscript::toDimension(sub)?] }), split: false }),
        Deref @ Subscript::WHOLE => Arc::new(NFExpression::TYPENAME { ty: ty }),
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            expl = Array::mapList(var_field!((*sub).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone(), (std::sync::Arc::new({ let __pe_b0 = ty.clone(); move |__pe_a1| applyIndexSubscriptTypename(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> + 'static>))?;
            makeArray(Type::liftArrayLeft(ty, Dimension::fromInteger(metamodelica::arrayLength(expl.clone()), Prefixes::Variability::CONSTANT.clone())), expl.clone(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptTypename(mut ty: Arc<Type::NFType>, mut index: Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> {
    let mut subscriptedExp: Arc<NFExpression>;
    let mut idx_exp: Arc<NFExpression>;
    let mut idx: i32;
    idx_exp = Subscript::toExp(index.clone())?;
    if isScalarLiteral(idx_exp.clone()) {
        idx = toInteger(idx_exp)?;
        subscriptedExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::BOOLEAN if (idx <= 2) => if (idx == 1) {Arc::new(NFExpression::BOOLEAN { value: false })} else {Arc::new(NFExpression::BOOLEAN { value: true })},
        Deref @ Type::ENUMERATION { .. } => nthEnumLiteral(ty, idx)?,
        _ => bail!("match: no arm matched"),
    } });
    } else {
        subscriptedExp = Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: Arc::new(NFExpression::TYPENAME { ty: ty.clone() }), subscripts: list![index], ty: ty, split: false });
    }
    Ok(subscriptedExp)
}

pub(crate) fn applySubscriptArray(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut sub: Arc<Subscript::NFSubscript>;
    let mut s: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut rest_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut expl: metamodelica::Array<Arc<NFExpression>> = Default::default();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut literal: bool = false;
    if isEmptyArray(exp.clone()) {
        outExp = makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?;
        return Ok(outExp.clone());
    }
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => applyIndexSubscriptArray(exp, sub, restSubscripts)?,
        Deref @ Subscript::SLICE { .. } => makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?,
        Deref @ Subscript::WHOLE => {
            if restSubscripts.clone().is_empty() {
                outExp = exp;
            } else {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp) {
                    Deref @ ARRAY { ty: __pa0, elements: __pa1, literal: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                ty = __pa0.clone();
                expl = __pa1.clone();
                literal = __pa2.clone();
                let (__pa3, __pa4) = ::match_deref::match_deref! { match &(restSubscripts.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                s = __pa3.clone();
                rest_subs = __pa4.clone();
                expl = Array::map(expl.clone(), (std::sync::Arc::new({ let __pe_b0 = s; let __pe_b2 = rest_subs; let __pe_b3 = applyToScope; move |__pe_a1| applySubscript(__pe_b0.clone(), __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
                (ty, literal) = typeSubscriptedArray(expl.clone(), restSubscripts, ty, literal)?;
                outExp = makeArray(ty, expl.clone(), literal);
            }
            outExp
        },
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ ARRAY { ty: __pa0, literal: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            literal = __pa1.clone();
            expl = Array::mapList(var_field!((*sub).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone(), (std::sync::Arc::new({ let __pe_b0 = exp; let __pe_b2 = restSubscripts.clone(); move |__pe_a1| applyIndexSubscriptArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> + 'static>))?;
            (ty, literal) = typeSubscriptedArray(expl.clone(), restSubscripts, ty, literal)?;
            makeArray(ty, expl.clone(), literal)
        },
        Deref @ Subscript::SPLIT_INDEX { .. } => makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn typeSubscriptedArray(mut elements: metamodelica::Array<Arc<NFExpression>>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut ty: Arc<Type::NFType>, mut literal: bool) -> Result<(Arc<Type::NFType>, bool)> {
    let mut ty: Arc<Type::NFType> = ty;
    let mut literal: bool = literal;
    let mut count: i32;
    let mut e: Arc<NFExpression>;
    count = metamodelica::arrayLength(elements.clone());
    if count > 0 {
        e = ({let __elt = elements.borrow()[(1-1) as usize].clone(); __elt});
        ty = typeOf(e.clone());
        literal = literal && isLiteral(e)?;
    } else {
        ty = Type::subscript(Type::unliftArray(ty)?, subscripts, true)?;
    }
    ty = Type::liftArrayLeft(ty, Dimension::fromInteger(count, Prefixes::Variability::CONSTANT.clone()));
    Ok((ty, literal))
}

pub(crate) fn applyIndexSubscriptArray(mut exp: Arc<NFExpression>, mut index: Arc<Subscript::NFSubscript>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    outExp = applyIndexExpArray(exp, Subscript::toExp(index)?, restSubscripts)?;
    Ok(outExp)
}

pub(crate) fn applyIndexExpArray(mut exp: Arc<NFExpression>, mut index: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    let mut idx: i32;
    if isScalarLiteral(index.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ ARRAY { elements: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expl = __pa0.clone();
        idx = toInteger(index.clone())?;
        if idx > 0 && idx <= metamodelica::arrayLength(expl.clone()) {
            outExp = applySubscripts(restSubscripts, ({let __elt = expl.borrow()[(idx-1) as usize].clone(); __elt}), false)?;
            return Ok(outExp.clone());
        }
    }
    outExp = makeSubscriptedExp(metamodelica::cons(Arc::new(Subscript::NFSubscript::INDEX { index: index }), restSubscripts), exp, false)?;
    Ok(outExp)
}

pub(crate) fn applySubscriptRange(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut sub: Arc<Subscript::NFSubscript>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expl: metamodelica::Array<Arc<NFExpression>> = Default::default();
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => applyIndexSubscriptRange(exp, sub)?,
        Deref @ Subscript::SLICE { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            ty = Arc::new(Type::NFType::ARRAY { elementType: Type::unliftArray(ty)?, dimensions: list![Subscript::toDimension(sub)?] });
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: exp, subscripts: list![subscript], ty: ty, split: false })
        },
        Deref @ Subscript::WHOLE => exp,
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            expl = Array::mapList(var_field!((*sub).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone(), (std::sync::Arc::new({ let __pe_b0 = exp.clone(); move |__pe_a1| applyIndexSubscriptRange(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> + 'static>))?;
            let __pa0 = ::match_deref::match_deref! { match &(exp) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            makeArray(Type::liftArrayLeft(ty, Dimension::fromInteger(metamodelica::arrayLength(expl.clone()), Prefixes::Variability::CONSTANT.clone())), expl.clone(), false)
        },
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            ty = Type::unliftArray(ty)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: exp, subscripts: list![sub], ty: ty, split: true })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.applySubscriptRange")); __mm_s.push_str(&*literal!(" got unknown subscript '")); __mm_s.push_str(&*Subscript::toString(sub)?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptRange(mut rangeExp: Arc<NFExpression>, mut index: Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut index_exp: Arc<NFExpression>;
    let mut start_exp: Arc<NFExpression>;
    let mut stop_exp: Arc<NFExpression>;
    let mut step_exp: Option<Arc<NFExpression>>;
    let mut ty: Arc<Type::NFType>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let __pa0 = ::match_deref::match_deref! { match &(index.clone()) {
        Deref @ Subscript::INDEX { index: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    index_exp = __pa0.clone();
    if isScalarLiteral(index_exp.clone()) {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(rangeExp) {
            Deref @ RANGE { start: __pa1, step: __pa2, stop: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        start_exp = __pa1.clone();
        step_exp = __pa2.clone();
        stop_exp = __pa3.clone();
        outExp = applyIndexSubscriptRange2(start_exp, step_exp, stop_exp, toInteger(index_exp)?)?;
    } else {
        let __pa4 = ::match_deref::match_deref! { match &(rangeExp.clone()) {
            Deref @ RANGE { ty: __pa4, .. } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa4.clone();
        subs = list![index];
        ty = Type::subscript(ty, subs.clone(), true)?;
        outExp = Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: rangeExp, subscripts: subs, ty: ty, split: false });
    }
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptRange2(mut startExp: Arc<NFExpression>, mut stepExp: Option<Arc<NFExpression>>, mut stopExp: Arc<NFExpression>, mut index: i32) -> Result<Arc<NFExpression>> {
    let mut subscriptedExp: Arc<NFExpression>;
    let mut iidx: i32 = 0;
    let mut ridx: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    subscriptedExp = (::match_deref::match_deref! { match &((startExp.clone(), stepExp)) {
        (Deref @ INTEGER { .. }, Some(Deref @ INTEGER { value: __esc_iidx })) => {
            iidx = (*__esc_iidx).clone();
            Arc::new(NFExpression::INTEGER { value: var_field!((*startExp).value, NFExpression::INTEGER).clone() + (index - 1) * iidx.clone() })
        },
        (Deref @ INTEGER { .. }, _) => Arc::new(NFExpression::INTEGER { value: var_field!((*startExp).value, NFExpression::INTEGER).clone() + index - 1 }),
        (Deref @ REAL { .. }, Some(Deref @ REAL { value: __esc_ridx })) => {
            ridx = (*__esc_ridx).clone();
            Arc::new(NFExpression::REAL { value: var_field!((*startExp).value, NFExpression::REAL).clone() + (metamodelica::OrderedFloat((index - 1) as f64)) * ridx.clone() })
        },
        (Deref @ REAL { .. }, _) => Arc::new(NFExpression::REAL { value: var_field!((*startExp).value, NFExpression::REAL).clone() + metamodelica::OrderedFloat((index) as f64) - metamodelica::OrderedFloat(1.0_f64) }),
        (Deref @ BOOLEAN { .. }, _) => if (index == 1) {startExp} else {stopExp},
        (Deref @ ENUM_LITERAL { index: __esc_iidx, .. }, _) => {
            iidx = (*__esc_iidx).clone();
            iidx = iidx.clone() + index - 1;
            nthEnumLiteral(var_field!((*startExp).ty, NFExpression::ENUM_LITERAL).clone(), iidx.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscriptedExp)
}

pub(crate) fn applySubscriptCall(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut call: Arc<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    outExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, .. } if (Function::Function::isSubscriptableBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone())?) => {
            let mut ty: Arc<Type::NFType>;
            let mut arg = (*arg).clone();
            arg = applySubscript(subscript, arg.clone(), restSubscripts, applyToScope)?;
            ty = Type::copyDims(typeOf(arg.clone()), var_field!((*call).ty, Call::NFCall::TYPED_CALL).clone());
            Arc::new(NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_CALL { r#fn: var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone(), ty: ty, var: var_field!((*call).var, Call::NFCall::TYPED_CALL).clone(), purity: var_field!((*call).purity, Call::NFCall::TYPED_CALL).clone(), arguments: list![arg.clone()], attributes: var_field!((*call).attributes, Call::NFCall::TYPED_CALL).clone() }) })
        },
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
            applySubscriptArrayConstructor(subscript, call.clone(), restSubscripts)?
        },
        _ => {
            makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn applySubscriptArrayConstructor(mut subscript: Arc<Subscript::NFSubscript>, mut call: Arc<Call::NFCall>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    if Subscript::isIndex(subscript.clone()) && restSubscripts.clone().is_empty() {
        outExp = applyIndexSubscriptArrayConstructor(call, subscript)?;
    } else {
        outExp = makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), Arc::new(NFExpression::CALL { call: call }), false)?;
    }
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptArrayConstructor(mut call: Arc<Call::NFCall>, mut index: Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> {
    let mut subscriptedExp: Arc<NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut var: Variability;
    let mut pur: Purity;
    let mut exp: Arc<NFExpression>;
    let mut iter_exp: Arc<NFExpression>;
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<NFExpression>)>>;
    let mut iter: Arc<InstNode::InstNode>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(call) {
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { ty: __pa0, var: __pa1, purity: __pa2, exp: __pa3, iters: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    var = __pa1.clone();
    pur = __pa2.clone();
    exp = __pa3.clone();
    iters = __pa4.clone();
    let ((__pa5, __pa6), __pa7) = List::splitLast(iters)?;
    iter = __pa5.clone();
    iter_exp = __pa6.clone();
    iters = __pa7.clone();
    iter_exp = applySubscript(index, iter_exp, metamodelica::nil(), false)?;
    subscriptedExp = replaceIterator(exp, iter, iter_exp)?;
    if !(iters.clone().is_empty()) {
        subscriptedExp = Arc::new(NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: Type::unliftArray(ty)?, var: var, purity: pur, exp: subscriptedExp, iters: iters }) });
    }
    Ok(subscriptedExp)
}

pub(crate) fn applySubscriptIf(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut cond: Arc<NFExpression>;
    let mut tb: Arc<NFExpression>;
    let mut fb: Arc<NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    tb = __pa2.clone();
    fb = __pa3.clone();
    if Type::isConditionalArray(ty.clone()) {
        match '__try4: {
            tb = unwrap_break_err!(applySubscript(subscript.clone(), tb.clone(), restSubscripts.clone(), applyToScope), '__try4);
            fb = unwrap_break_err!(applySubscript(subscript.clone(), fb.clone(), restSubscripts.clone(), applyToScope), '__try4);
            ty = unwrap_break_err!(Type::setConditionalArrayTypes(ty.clone(), typeOf(tb.clone()), typeOf(fb.clone())), '__try4);
            outExp = Arc::new(NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb.clone(), falseBranch: fb.clone() });
            Ok::<_, anyhow::Error>((outExp.clone(),))
        } {
            Ok((__try4_o0,)) => {
                outExp = __try4_o0;
            }
            Err(_) => {
                outExp = makeSubscriptedExp(metamodelica::cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?;
            }
        }
    } else {
        tb = applySubscript(subscript.clone(), tb, restSubscripts.clone(), applyToScope)?;
        fb = applySubscript(subscript, fb, restSubscripts, applyToScope)?;
        ty = typeOf(tb.clone());
        outExp = Arc::new(NFExpression::IF { ty: ty, condition: cond, trueBranch: tb, falseBranch: fb });
    }
    Ok(outExp)
}

pub(crate) fn makeSubscriptedExp(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut exp: Arc<NFExpression>, mut backend: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut e: Arc<NFExpression>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut extra_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut ty: Arc<Type::NFType>;
    let mut dim_count: i32;
    let mut split: bool;
    (e, subs, ty, split) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { .. } => (var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), typeOf(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone()), var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone()),
        _ => (exp.clone(), metamodelica::nil(), typeOf(exp.clone()), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(split) {
        split = List::any(subscripts.clone(), (std::sync::Arc::new(fnptr!(Subscript::isSplitIndex, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>))?;
    }
    dim_count = Type::dimensionCount(ty.clone());
    (subs, extra_subs) = Subscript::mergeList(subscripts.clone(), subs, dim_count, backend)?;
    if !(extra_subs.is_empty()) {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeSubscriptedExp")); __mm_s.push_str(&*literal!(": too few dimensions in ")); __mm_s.push_str(&*toString(exp)?); __mm_s.push_str(&*literal!(" to apply subscripts ")); __mm_s.push_str(&*Subscript::toStringList(subscripts)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
    }
    ty = Type::subscript(ty, subs.clone(), true)?;
    outExp = Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e, subscripts: subs, ty: ty, split: split });
    Ok(outExp)
}

pub fn replaceIterator(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut iteratorValue: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = map(exp, (std::sync::Arc::new({ let __pe_b1 = iterator; let __pe_b2 = iteratorValue; move |__pe_a0| replaceIterator2(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
    Ok(exp)
}

pub(crate) fn replaceIterator2(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut iteratorValue: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } if (ComponentRef::isSimple(var_field!((*exp).cref, NFExpression::CREF).clone())) => {
            if (InstNode::refEqual(iterator, node.clone())) {iteratorValue} else {exp.clone()}
        },
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            let mut node: Arc<InstNode::InstNode>;
            let mut fields: Arc<metamodelica::List<ArcStr>>;
            node = ComponentRef::node(ComponentRef::last(var_field!((*exp).cref, NFExpression::CREF).clone()))?;
            if InstNode::refEqual(iterator, node.clone()) {
                outExp = iteratorValue;
                fields = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (listRest(ComponentRef::nodes(var_field!((*exp).cref, NFExpression::CREF).clone(), metamodelica::nil())?)?).into_iter().cloned() {
            let __x = InstNode::name(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                for mut f in &*fields {
                    let mut f = f.clone();
                    outExp = recordElement((f.clone()).clone(), outExp.clone())?;
                }
            } else {
                outExp = exp.clone();
            }
            outExp
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn containsIterator(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>) -> Result<bool> {
    fn containsIterator2(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>) -> bool {
        let mut res: bool;
        res = (::match_deref::match_deref! { match &(exp) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } => {
            InstNode::refEqual(node.clone(), iterator)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    let mut res: bool;
    res = contains(exp, (std::sync::Arc::new({ let __pe_b1 = iterator; move |__pe_a0| Ok(containsIterator2(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?;
    Ok(res)
}

pub(crate) fn arrayFromList(mut inExps: Arc<metamodelica::List<Arc<NFExpression>>>, mut elemTy: Arc<Type::NFType>, mut inDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    outExp = arrayFromList_impl(inExps, elemTy, inDims.reverse())?;
    Ok(outExp)
}

pub(crate) fn arrayFromList_impl(mut inExps: Arc<metamodelica::List<Arc<NFExpression>>>, mut elemTy: Arc<Type::NFType>, mut inDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut ldim: Arc<Dimension::NFDimension>;
    let mut restdims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut ty: Arc<Type::NFType>;
    let mut newlst: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut partexps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>>;
    let mut dimsize: i32;
    Error::assertion(!(inDims.clone().is_empty()), (literal!("Empty dimension list given in arrayFromList.")).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDims.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ldim = __pa0.clone();
    restdims = __pa1.clone();
    dimsize = Dimension::size(ldim.clone(), false)?;
    ty = Type::liftArrayLeft(elemTy, ldim);
    if List::hasOneElement(inDims) {
        Error::assertion(dimsize == (inExps.clone().len() as i32), (literal!("Length mismatch in arrayFromList.")).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
        outExp = makeArray(ty, metamodelica::arrayFromVec(inExps.into_iter().cloned().collect()), false);
        return Ok(outExp.clone());
    }
    partexps = List::partition(inExps, dimsize)?;
    newlst = metamodelica::nil();
    for mut arrexp in &*partexps {
        let mut arrexp = arrexp.clone();
        newlst = metamodelica::cons(makeArray(ty.clone(), metamodelica::arrayFromVec(arrexp.clone().into_iter().cloned().collect()), false), newlst.clone());
    }
    newlst = newlst.reverse();
    outExp = arrayFromList_impl(newlst, ty, restdims)?;
    Ok(outExp)
}

pub(crate) fn makeEnumLiteral(mut enumType: Arc<Type::NFType>, mut index: i32) -> Result<Arc<NFExpression>> {
    let mut literal: Arc<NFExpression>;
    let mut literals: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(enumType.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    literals = __pa0.clone();
    literal = Arc::new(NFExpression::ENUM_LITERAL { ty: enumType, name: ((literals).get(index)?).clone(), index: index });
    Ok(literal)
}

pub(crate) fn makeEnumLiterals(mut enumType: Arc<Type::NFType>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut literals: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut lits: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(enumType.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lits = __pa0.clone();
    literals = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        let __thr_src0 = lits.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let mut __thr_it1 = (1..=(lits.len() as i32)).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(l), Some(i)) => {
                    let __x = Arc::new(NFExpression::ENUM_LITERAL { ty: enumType.clone(), name: (l.clone()).clone(), index: i.clone() });
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(literals)
}

pub(crate) fn isIntegerValue(mut exp: Arc<NFExpression>, mut value: i32) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() == value,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn toInteger(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut i: i32;
    i = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone(),
        Deref @ BOOLEAN { .. } => if (var_field!((*exp).value, NFExpression::BOOLEAN).clone()) {2} else {1},
        Deref @ ENUM_LITERAL { .. } => var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(i)
}

pub(crate) fn toStringTyped(mut exp: Arc<NFExpression>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/*")); __mm_s.push_str(&*Type::toString(typeOf(exp.clone()))?); __mm_s.push_str(&*literal!("*/ ")); __mm_s.push_str(&*toString(exp)?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn toString(mut exp: Arc<NFExpression>) -> Result<ArcStr> {
    '__tco: loop {
        let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(intString(var_field!((*exp).value, NFExpression::INTEGER).clone())),
        Deref @ REAL { .. } => return Ok(realString(var_field!((*exp).value, NFExpression::REAL).clone())),
        Deref @ STRING { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*System::escapedString((var_field!((*exp).value, NFExpression::STRING).clone()).clone(), false)); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
        Deref @ BOOLEAN { .. } => return Ok(boolString(var_field!((*exp).value, NFExpression::BOOLEAN).clone())),
        Deref @ ENUM_LITERAL { ty: __esc_t @ Deref @ Type::ENUMERATION { .. }, .. } => {
            t = (*__esc_t).clone();
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*t).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()); ArcStr::from(__mm_s) })
        },
        Deref @ CLKCONST { .. } => return Ok(ClockKind::toString(var_field!((*exp).clk, NFExpression::CLKCONST).clone())?),
        Deref @ CREF { .. } => return Ok(ComponentRef::toString(var_field!((*exp).cref, NFExpression::CREF).clone())?),
        Deref @ TYPENAME { .. } => return Ok(Type::typenameString(Type::arrayElementType(var_field!((*exp).ty, NFExpression::TYPENAME).clone()))?),
        Deref @ ARRAY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }),
        Deref @ MATRIX { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut el in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("; ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
        Deref @ RANGE { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).start, NFExpression::RANGE).clone(), exp.clone(), false)?); __mm_s.push_str(&*if (isSome(var_field!((*exp).step, NFExpression::RANGE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandString(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, exp.clone(), false)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandString(var_field!((*exp).stop, NFExpression::RANGE).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) }),
        Deref @ TUPLE { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ RECORD { .. } => return Ok(List::toString(var_field!((*exp).elements, NFExpression::RECORD).clone(), (std::sync::Arc::new(toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<ArcStr> + 'static>), (AbsynUtil::pathString(var_field!((*exp).path, NFExpression::RECORD).clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?),
        Deref @ CALL { .. } => return Ok(Call::toString(var_field!((*exp).call, NFExpression::CALL).clone())?),
        Deref @ SIZE { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SIZE).clone())?); __mm_s.push_str(&*if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ END { .. } => return Ok(literal!("end")),
        Deref @ MULTARY { .. } if (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone().is_empty()) => return Ok(multaryString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), false)?),
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty() && Operator::isDashClassification(Operator::getMathClassification(var_field!((*exp).operator, NFExpression::MULTARY).clone())?)) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*multaryString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); ArcStr::from(__mm_s) }),
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty()) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1/")); __mm_s.push_str(&*multaryString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); ArcStr::from(__mm_s) }),
        Deref @ MULTARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*multaryString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); __mm_s.push_str(&*Operator::symbol(Operator::invert(var_field!((*exp).operator, NFExpression::MULTARY).clone())?, (literal!(" ")).clone())?); __mm_s.push_str(&*multaryString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); ArcStr::from(__mm_s) }),
        Deref @ BINARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).exp1, NFExpression::BINARY).clone(), exp.clone(), true)?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::BINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp2, NFExpression::BINARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) }),
        Deref @ UNARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::UNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp, NFExpression::UNARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) }),
        Deref @ LBINARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), exp.clone(), true)?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LBINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) }),
        Deref @ LUNARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LUNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*operandString(var_field!((*exp).exp, NFExpression::LUNARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) }),
        Deref @ RELATION { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).exp1, NFExpression::RELATION).clone(), exp.clone(), true)?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::RELATION).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp2, NFExpression::RELATION).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) }),
        Deref @ IF { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*toString(var_field!((*exp).condition, NFExpression::IF).clone())?); __mm_s.push_str(&*literal!(" then ")); __mm_s.push_str(&*toString(var_field!((*exp).trueBranch, NFExpression::IF).clone())?); __mm_s.push_str(&*literal!(" else ")); __mm_s.push_str(&*toString(var_field!((*exp).falseBranch, NFExpression::IF).clone())?); ArcStr::from(__mm_s) }),
        Deref @ CAST { .. } => if (Flags::isSet(Flags::NF_API.clone())?) {{ exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; }} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CAST(")); __mm_s.push_str(&*Type::toString(var_field!((*exp).ty, NFExpression::CAST).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::CAST).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })},
        Deref @ BOX { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BOX(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::BOX).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ UNBOX { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNBOX(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::UNBOX).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ SUBSCRIPTED_EXP { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*Subscript::toStringList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?); ArcStr::from(__mm_s) }),
        Deref @ TUPLE_ELEMENT { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
        Deref @ RECORD_ELEMENT { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toString(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?); __mm_s.push_str(&*literal!(").")); __mm_s.push_str(&*var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()); ArcStr::from(__mm_s) }),
        Deref @ MUTABLE { .. } => { exp = Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()); continue '__tco; },
        Deref @ SHARED_LITERAL { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("LITERAL(")); __mm_s.push_str(&*intString(var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ EMPTY { .. } => return Ok(literal!("#EMPTY#")),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let __thr_src0 = var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(a), Some(n)) => {
                    let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*toString(a.clone())?); ArcStr::from(__mm_s) };
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ FILENAME { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*System::escapedString((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone(), false)); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
        Deref @ INSTANCE_NAME { .. } => return Ok(literal!("getInstanceName()")),
        _ => return Ok(anyString(exp.clone())),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn toFlatString(mut exp: Arc<NFExpression>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    '__tco: loop {
        let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(intString(var_field!((*exp).value, NFExpression::INTEGER).clone())),
        Deref @ REAL { .. } => return Ok(realString(var_field!((*exp).value, NFExpression::REAL).clone())),
        Deref @ STRING { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Util::escapeModelicaStringToCString((var_field!((*exp).value, NFExpression::STRING).clone()).clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
        Deref @ BOOLEAN { .. } => return Ok(boolString(var_field!((*exp).value, NFExpression::BOOLEAN).clone())),
        Deref @ ENUM_LITERAL { ty: __esc_t @ Deref @ Type::ENUMERATION { .. }, .. } => {
            t = (*__esc_t).clone();
            if (Type::isBuiltinEnumeration(t.clone())) {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*t).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()); ArcStr::from(__mm_s) })} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::makeQuotedIdentifier((AbsynUtil::pathString(var_field!((*t).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?).clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Util::makeQuotedIdentifier((var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()).clone())?); ArcStr::from(__mm_s) })}
        },
        Deref @ CLKCONST { .. } => return Ok(ClockKind::toFlatString(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), format)?),
        Deref @ CREF { .. } => return Ok(ComponentRef::toFlatString(var_field!((*exp).cref, NFExpression::CREF).clone(), format)?),
        Deref @ TYPENAME { .. } => return Ok(Type::typenameString(Type::arrayElementType(var_field!((*exp).ty, NFExpression::TYPENAME).clone()))?),
        Deref @ ARRAY { .. } => if (var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("fill(")); __mm_s.push_str(&*toFlatString(makeDefaultValue(Type::elementType(var_field!((*exp).ty, NFExpression::ARRAY).clone()), None, None)?, format.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Type::dimensionsToFlatString(var_field!((*exp).ty, NFExpression::ARRAY).clone(), format)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) })},
        Deref @ MATRIX { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut el in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("; ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
        Deref @ RANGE { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).start, NFExpression::RANGE).clone(), exp.clone(), false, format.clone())?); __mm_s.push_str(&*if (isSome(var_field!((*exp).step, NFExpression::RANGE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandFlatString(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandFlatString(var_field!((*exp).stop, NFExpression::RANGE).clone(), exp.clone(), false, format)?); ArcStr::from(__mm_s) }),
        Deref @ TUPLE { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ RECORD { .. } => return Ok(List::toString(var_field!((*exp).elements, NFExpression::RECORD).clone(), (std::sync::Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<ArcStr> + 'static>), (Type::toFlatString(var_field!((*exp).ty, NFExpression::RECORD).clone(), format)?).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?),
        Deref @ CALL { .. } => return Ok(Call::toFlatString(var_field!((*exp).call, NFExpression::CALL).clone(), format)?),
        Deref @ SIZE { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*toFlatString(var_field!((*exp).exp, NFExpression::SIZE).clone(), format.clone())?); __mm_s.push_str(&*if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toFlatString(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?, format)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ END { .. } => return Ok(literal!("end")),
        Deref @ MULTARY { .. } if (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone().is_empty()) => return Ok(multaryFlatString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format, false)?),
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty() && Operator::isDashClassification(Operator::getMathClassification(var_field!((*exp).operator, NFExpression::MULTARY).clone())?)) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format, true)?); ArcStr::from(__mm_s) }),
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty()) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1/")); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format, true)?); ArcStr::from(__mm_s) }),
        Deref @ MULTARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format.clone(), true)?); __mm_s.push_str(&*Operator::symbol(Operator::invert(var_field!((*exp).operator, NFExpression::MULTARY).clone())?, (literal!(" ")).clone())?); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format, true)?); ArcStr::from(__mm_s) }),
        Deref @ BINARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp1, NFExpression::BINARY).clone(), exp.clone(), true, format.clone())?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::BINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp2, NFExpression::BINARY).clone(), exp.clone(), false, format)?); ArcStr::from(__mm_s) }),
        Deref @ UNARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::UNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp, NFExpression::UNARY).clone(), exp.clone(), false, format)?); ArcStr::from(__mm_s) }),
        Deref @ LBINARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), exp.clone(), true, format.clone())?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LBINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), exp.clone(), false, format)?); ArcStr::from(__mm_s) }),
        Deref @ LUNARY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LUNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp, NFExpression::LUNARY).clone(), exp.clone(), false, format)?); ArcStr::from(__mm_s) }),
        Deref @ RELATION { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp1, NFExpression::RELATION).clone(), exp.clone(), true, format.clone())?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::RELATION).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp2, NFExpression::RELATION).clone(), exp.clone(), false, format)?); ArcStr::from(__mm_s) }),
        Deref @ IF { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*toFlatString(var_field!((*exp).condition, NFExpression::IF).clone(), format.clone())?); __mm_s.push_str(&*literal!(" then ")); __mm_s.push_str(&*toFlatString(var_field!((*exp).trueBranch, NFExpression::IF).clone(), format.clone())?); __mm_s.push_str(&*literal!(" else ")); __mm_s.push_str(&*toFlatString(var_field!((*exp).falseBranch, NFExpression::IF).clone(), format)?); ArcStr::from(__mm_s) }),
        Deref @ CAST { .. } => { (exp, format) = (var_field!((*exp).exp, NFExpression::CAST).clone(), format); continue '__tco; },
        Deref @ UNBOX { .. } => { (exp, format) = (var_field!((*exp).exp, NFExpression::UNBOX).clone(), format); continue '__tco; },
        Deref @ BOX { .. } => { (exp, format) = (var_field!((*exp).exp, NFExpression::BOX).clone(), format); continue '__tco; },
        Deref @ SUBSCRIPTED_EXP { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toFlatString(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), format.clone())?); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*Subscript::toFlatStringList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), format, false)?); ArcStr::from(__mm_s) }),
        Deref @ TUPLE_ELEMENT { .. } => { (exp, format) = (var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), format); continue '__tco; },
        Deref @ RECORD_ELEMENT { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toFlatString(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), format)?); __mm_s.push_str(&*literal!(").")); __mm_s.push_str(&*var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()); ArcStr::from(__mm_s) }),
        Deref @ MUTABLE { .. } => { (exp, format) = (Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), format); continue '__tco; },
        Deref @ SHARED_LITERAL { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[literal: ")); __mm_s.push_str(&*intString(var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
        Deref @ EMPTY { .. } => return Ok(literal!("#EMPTY#")),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ")); __mm_s.push_str(&*ComponentRef::toFlatString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), format.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let __thr_src0 = var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(a), Some(n)) => {
                    let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*toFlatString(a.clone(), format.clone())?); ArcStr::from(__mm_s) };
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
        Deref @ FILENAME { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Util::escapeModelicaStringToCString((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
        Deref @ INSTANCE_NAME { .. } => return Ok(literal!("getInstanceName()")),
        _ => return Ok(anyString(exp.clone())),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn operandString(mut operand: Arc<NFExpression>, mut operator: Arc<NFExpression>, mut lhs: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut operand_prio: i32;
    let mut operator_prio: i32;
    let mut parenthesize: bool = false;
    r#str = (toString(operand.clone())?).clone();
    operand_prio = priority(operand.clone(), lhs)?;
    if operand_prio == 4 {
        parenthesize = true;
    } else {
        operator_prio = priority(operator, lhs)?;
        if operand_prio > operator_prio {
            parenthesize = true;
        } else if operand_prio == operator_prio {
            parenthesize = if (lhs) {isNonAssociativeExp(operand)} else {!(isAssociativeExp(operand))};
        }
    }
    if parenthesize {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn operandFlatString(mut operand: Arc<NFExpression>, mut operator: Arc<NFExpression>, mut lhs: bool, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut operand_prio: i32;
    let mut operator_prio: i32;
    let mut parenthesize: bool = false;
    r#str = (toFlatString(operand.clone(), format)?).clone();
    operand_prio = priority(operand.clone(), lhs)?;
    if operand_prio == 4 {
        parenthesize = true;
    } else {
        operator_prio = priority(operator, lhs)?;
        if operand_prio > operator_prio {
            parenthesize = true;
        } else if operand_prio == operator_prio {
            parenthesize = if (lhs) {isNonAssociativeExp(operand)} else {!(isAssociativeExp(operand))};
        }
    }
    if parenthesize {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn multaryString(mut arguments: Arc<metamodelica::List<Arc<NFExpression>>>, mut exp: Arc<NFExpression>, mut operator: Arc<Operator::NFOperator>, mut parenthesize: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (arguments.clone()).into_iter().cloned() {
            let __x = operandString(e.clone(), exp.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (Operator::symbol(operator, (literal!(" ")).clone())?).clone());
    if parenthesize && (arguments.len() as i32) > 1 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn multaryFlatString(mut arguments: Arc<metamodelica::List<Arc<NFExpression>>>, mut exp: Arc<NFExpression>, mut operator: Arc<Operator::NFOperator>, mut format: BaseModelica::OutputFormat, mut parenthesize: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (arguments.clone()).into_iter().cloned() {
            let __x = operandFlatString(e.clone(), exp.clone(), false, format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (Operator::symbol(operator, (literal!(" ")).clone())?).clone());
    if parenthesize && (arguments.len() as i32) > 1 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn priority(mut exp: Arc<NFExpression>, mut lhs: bool) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => if (var_field!((*exp).value, NFExpression::INTEGER).clone() < 0) {return Ok(4)} else {return Ok(0)},
        Deref @ REAL { .. } => if (var_field!((*exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat(0.0_f64)) {return Ok(4)} else {return Ok(0)},
        Deref @ MULTARY { .. } => return Ok(Operator::priority(var_field!((*exp).operator, NFExpression::MULTARY).clone(), lhs)),
        Deref @ BINARY { .. } => return Ok(Operator::priority(var_field!((*exp).operator, NFExpression::BINARY).clone(), lhs)),
        Deref @ UNARY { .. } => return Ok(4),
        Deref @ LBINARY { .. } => return Ok(Operator::priority(var_field!((*exp).operator, NFExpression::LBINARY).clone(), lhs)),
        Deref @ LUNARY { .. } => return Ok(7),
        Deref @ RELATION { .. } => return Ok(6),
        Deref @ RANGE { .. } => return Ok(10),
        Deref @ IF { .. } => return Ok(11),
        Deref @ CAST { .. } => { (exp, lhs) = (var_field!((*exp).exp, NFExpression::CAST).clone(), lhs); continue '__tco; },
        Deref @ BOX { .. } => { (exp, lhs) = (var_field!((*exp).exp, NFExpression::BOX).clone(), lhs); continue '__tco; },
        Deref @ UNBOX { .. } => { (exp, lhs) = (var_field!((*exp).exp, NFExpression::UNBOX).clone(), lhs); continue '__tco; },
        _ => return Ok(0),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isAssociativeExp(mut exp: Arc<NFExpression>) -> bool {
    let mut isAssociative: bool;
    isAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { .. } => Operator::isAssociative(var_field!((*exp).operator, NFExpression::BINARY).clone()),
        Deref @ LBINARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAssociative
}

pub(crate) fn isNonAssociativeExp(mut exp: Arc<NFExpression>) -> bool {
    let mut isAssociative: bool;
    isAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { .. } => Operator::isNonAssociative(var_field!((*exp).operator, NFExpression::BINARY).clone()),
        Deref @ LBINARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAssociative
}

pub(crate) fn getName(mut exp: Arc<NFExpression>) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RECORD { .. } => return Ok(AbsynUtil::pathString(var_field!((*exp).path, NFExpression::RECORD).clone(), (literal!(".")).clone(), true, false)?),
        Deref @ CALL { .. } => return Ok(AbsynUtil::pathString(Call::functionName(var_field!((*exp).call, NFExpression::CALL).clone())?, (literal!(".")).clone(), true, false)?),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ BOX { .. } => { exp = var_field!((*exp).exp, NFExpression::BOX).clone(); continue '__tco; },
        Deref @ UNBOX { .. } => { exp = var_field!((*exp).exp, NFExpression::UNBOX).clone(); continue '__tco; },
        Deref @ MUTABLE { .. } => { exp = Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()); continue '__tco; },
        Deref @ SHARED_LITERAL { .. } => { exp = var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(); continue '__tco; },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => return Ok(ComponentRef::toString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?),
        Deref @ INSTANCE_NAME { .. } => return Ok(literal!("getInstanceName")),
        _ => return Ok(toString(exp)?),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn enumLiteralPath(mut exp: Arc<NFExpression>) -> Result<Arc<Path>> {
    let mut path: Arc<Path>;
    let mut name: ArcStr;
    let mut ty_path: Arc<Path>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp) {
        Deref @ ENUM_LITERAL { name: __pa0, ty: Deref @ Type::ENUMERATION { typePath: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty_path = __pa1.clone();
    path = AbsynUtil::suffixPath(ty_path, (name).clone())?;
    Ok(path)
}

pub fn getNominal(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = map(exp, (std::sync::Arc::new(computeNominal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
    exp = SimplifyExp::simplify(exp, false)?;
    Ok(exp)
}

pub(crate) fn computeNominal(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { varPointer, .. }, .. }, .. } => {
            let mut nominal: Option<Arc<NFExpression>>;
            nominal = Variable::getNominal(Pointer::access(varPointer.clone()));
            Util::getOptionOrDefault(nominal, exp)
        },
        Deref @ INTEGER { .. } => {
            Arc::new(NFExpression::INTEGER { value: var_field!((*exp).value, NFExpression::INTEGER).clone().abs() })
        },
        Deref @ REAL { .. } => {
            Arc::new(NFExpression::REAL { value: var_field!((*exp).value, NFExpression::REAL).clone().abs() })
        },
        Deref @ UNARY { .. } => {
            var_field!((*exp).exp, NFExpression::UNARY).clone()
        },
        Deref @ BINARY { operator, .. } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::SUBTRACTION.clone()) => {
            let mut sizeClass: Operator::SizeClassification;
            (_, sizeClass) = Operator::classify(operator.clone())?;
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass), operator.ty.clone())?);
            exp
        },
        Deref @ MULTARY { operator, .. } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = listAppend(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()),
                inv_arguments = metamodelica::nil()
            );
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn toAbsyn(mut exp: Arc<NFExpression>) -> Result<Arc<Absyn::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            return Ok(Arc::new(Absyn::Exp::INTEGER { value: var_field!((*exp).value, NFExpression::INTEGER).clone() }))
        },
        Deref @ REAL { .. } => {
            return Ok(Arc::new(Absyn::Exp::REAL { value: ArcStr::from(::std::format!("{}", var_field!((*exp).value, NFExpression::REAL).clone())) }))
        },
        Deref @ STRING { .. } => {
            return Ok(Arc::new(Absyn::Exp::STRING { value: (var_field!((*exp).value, NFExpression::STRING).clone()).clone() }))
        },
        Deref @ BOOLEAN { .. } => {
            return Ok(Arc::new(Absyn::Exp::BOOL { value: var_field!((*exp).value, NFExpression::BOOLEAN).clone() }))
        },
        Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { .. }, .. } => {
            return Ok(Arc::new(Absyn::Exp::CREF { componentRef: AbsynUtil::pathToCref(enumLiteralPath(exp)?)? }))
        },
        Deref @ CLKCONST { .. } => {
            return Ok(ClockKind::toAbsyn(var_field!((*exp).clk, NFExpression::CLKCONST).clone())?)
        },
        Deref @ CREF { .. } => {
            return Ok(Arc::new(Absyn::Exp::CREF { componentRef: ComponentRef::toAbsyn(var_field!((*exp).cref, NFExpression::CREF).clone())? }))
        },
        Deref @ TYPENAME { .. } => {
            return Ok(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (Type::toString(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?).clone(), subscripts: metamodelica::nil() }) }))
        },
        Deref @ ARRAY { .. } => {
            return Ok(Arc::new(Absyn::Exp::ARRAY { arrayExp: ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
        },
        Deref @ MATRIX { .. } => {
            return Ok(Arc::new(Absyn::Exp::MATRIX { matrix: ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
        for mut l in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (l.clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
        },
        Deref @ RANGE { .. } => {
            return Ok(Arc::new(Absyn::Exp::RANGE { start: toAbsyn(var_field!((*exp).start, NFExpression::RANGE).clone())?, step: Util::applyOption(var_field!((*exp).step, NFExpression::RANGE).clone(), (std::sync::Arc::new(toAbsyn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<Absyn::Exp>> + 'static>))?, stop: toAbsyn(var_field!((*exp).stop, NFExpression::RANGE).clone())? }))
        },
        Deref @ TUPLE { .. } => {
            return Ok(Arc::new(Absyn::Exp::TUPLE { expressions: ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
        },
        Deref @ RECORD { .. } => {
            return Ok(AbsynUtil::makeCall(AbsynUtil::pathToCref(var_field!((*exp).path, NFExpression::RECORD).clone())?, ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), metamodelica::nil()))
        },
        Deref @ CALL { .. } => {
            return Ok(Call::toAbsyn(var_field!((*exp).call, NFExpression::CALL).clone())?)
        },
        Deref @ SIZE { .. } => {
            return Ok(AbsynUtil::makeCall(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("size")).clone(), subscripts: metamodelica::nil() }), if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {list![toAbsyn(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?]} else {metamodelica::nil()}, metamodelica::nil()))
        },
        Deref @ END { .. } => {
            return Ok(openmodelica_ast::Absyn::Exp::interned_END())
        },
        Deref @ BINARY { .. } => {
            return Ok(Arc::new(Absyn::Exp::BINARY { exp1: toAbsyn(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::BINARY).clone())?, exp2: toAbsyn(var_field!((*exp).exp2, NFExpression::BINARY).clone())? }))
        },
        Deref @ UNARY { .. } => {
            return Ok(Arc::new(Absyn::Exp::UNARY { op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::UNARY).clone())?, exp: toAbsyn(var_field!((*exp).exp, NFExpression::UNARY).clone())? }))
        },
        Deref @ LBINARY { .. } => {
            return Ok(Arc::new(Absyn::Exp::LBINARY { exp1: toAbsyn(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::LBINARY).clone())?, exp2: toAbsyn(var_field!((*exp).exp2, NFExpression::LBINARY).clone())? }))
        },
        Deref @ LUNARY { .. } => {
            return Ok(Arc::new(Absyn::Exp::LUNARY { op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::LUNARY).clone())?, exp: toAbsyn(var_field!((*exp).exp, NFExpression::LUNARY).clone())? }))
        },
        Deref @ RELATION { .. } => {
            return Ok(Arc::new(Absyn::Exp::RELATION { exp1: toAbsyn(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::RELATION).clone())?, exp2: toAbsyn(var_field!((*exp).exp2, NFExpression::RELATION).clone())? }))
        },
        Deref @ IF { .. } => {
            return Ok(Arc::new(Absyn::Exp::IFEXP { ifExp: toAbsyn(var_field!((*exp).condition, NFExpression::IF).clone())?, trueBranch: toAbsyn(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, elseBranch: toAbsyn(var_field!((*exp).falseBranch, NFExpression::IF).clone())?, elseIfBranch: metamodelica::nil() }))
        },
        Deref @ CAST { .. } => {
            { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; }
        },
        Deref @ BOX { .. } => {
            { exp = var_field!((*exp).exp, NFExpression::BOX).clone(); continue '__tco; }
        },
        Deref @ UNBOX { .. } => {
            { exp = var_field!((*exp).exp, NFExpression::UNBOX).clone(); continue '__tco; }
        },
        Deref @ MUTABLE { .. } => {
            { exp = Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()); continue '__tco; }
        },
        Deref @ SHARED_LITERAL { .. } => {
            { exp = var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(); continue '__tco; }
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            return Ok(Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: ComponentRef::toAbsyn(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?, functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), argNames: metamodelica::nil() }) }))
        },
        Deref @ FILENAME { .. } => {
            return Ok(Arc::new(Absyn::Exp::STRING { value: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() }))
        },
        Deref @ INSTANCE_NAME { .. } => {
            return Ok(AbsynUtil::makeCall(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("getInstanceName")).clone(), subscripts: metamodelica::nil() }), metamodelica::nil(), metamodelica::nil()))
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toAbsyn")); __mm_s.push_str(&*literal!(" got unknown expression '")); __mm_s.push_str(&*toString(exp)?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn toDAE(mut exp: Arc<NFExpression>, mut allowEmpty: bool) -> Result<Arc<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            return Ok(Arc::new(DAE::Exp::ICONST { integer: var_field!((*exp).value, NFExpression::INTEGER).clone() }))
        },
        Deref @ REAL { .. } => {
            return Ok(Arc::new(DAE::Exp::RCONST { real: var_field!((*exp).value, NFExpression::REAL).clone() }))
        },
        Deref @ STRING { .. } => {
            return Ok(Arc::new(DAE::Exp::SCONST { string: (var_field!((*exp).value, NFExpression::STRING).clone()).clone() }))
        },
        Deref @ BOOLEAN { .. } => {
            return Ok(Arc::new(DAE::Exp::BCONST { bool: var_field!((*exp).value, NFExpression::BOOLEAN).clone() }))
        },
        Deref @ ENUM_LITERAL { .. } => {
            return Ok(Arc::new(DAE::Exp::ENUM_LITERAL { name: enumLiteralPath(exp.clone())?, index: var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone() }))
        },
        Deref @ CLKCONST { .. } => {
            return Ok(Arc::new(DAE::Exp::CLKCONST { clk: ClockKind::toDAE(var_field!((*exp).clk, NFExpression::CLKCONST).clone())? }))
        },
        Deref @ CREF { .. } => {
            return Ok(Arc::new(DAE::Exp::CREF { componentRef: ComponentRef::toDAE(var_field!((*exp).cref, NFExpression::CREF).clone())?, ty: Type::toDAE(var_field!((*exp).ty, NFExpression::CREF).clone(), true)? }))
        },
        Deref @ TYPENAME { .. } => {
            { (exp, allowEmpty) = (ExpandExp::expandTypename(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?, false); continue '__tco; }
        },
        Deref @ ARRAY { .. } => {
            return Ok(Arc::new(DAE::Exp::ARRAY { ty: Type::toDAE(var_field!((*exp).ty, NFExpression::ARRAY).clone(), true)?, scalar: Type::isVector(var_field!((*exp).ty, NFExpression::ARRAY).clone())?, array: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toDAE(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
        },
        Deref @ RECORD { .. } => {
            return Ok(toDAERecord(var_field!((*exp).ty, NFExpression::RECORD).clone(), var_field!((*exp).path, NFExpression::RECORD).clone(), var_field!((*exp).elements, NFExpression::RECORD).clone())?)
        },
        Deref @ RANGE { .. } => {
            return Ok(Arc::new(DAE::Exp::RANGE { ty: Type::toDAE(var_field!((*exp).ty, NFExpression::RANGE).clone(), true)?, start: toDAE(var_field!((*exp).start, NFExpression::RANGE).clone(), false)?, step: if (isSome(var_field!((*exp).step, NFExpression::RANGE).clone())) {Some(toDAE(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, false)?)} else {None}, stop: toDAE(var_field!((*exp).stop, NFExpression::RANGE).clone(), false)? }))
        },
        Deref @ TUPLE { .. } => {
            return Ok(Arc::new(DAE::Exp::TUPLE { PR: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toDAE(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
        },
        Deref @ CALL { .. } => {
            return Ok(Call::toDAE(var_field!((*exp).call, NFExpression::CALL).clone())?)
        },
        Deref @ SIZE { .. } => {
            return Ok(Arc::new(DAE::Exp::SIZE { exp: toDAE(var_field!((*exp).exp, NFExpression::SIZE).clone(), false)?, sz: if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {Some(toDAE(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?, false)?)} else {None} }))
        },
        Deref @ MULTARY { .. } => {
            { (exp, allowEmpty) = (SimplifyExp::splitMultary(exp)?, false); continue '__tco; }
        },
        Deref @ BINARY { .. } => {
            let mut daeOp: DAE::Operator;
            let mut swap: bool;
            let mut negate: bool;
            let mut dae1: Arc<DAE::Exp>;
            let mut dae2: Arc<DAE::Exp>;
            (daeOp, swap, negate) = Operator::toDAE(var_field!((*exp).operator, NFExpression::BINARY).clone())?;
            dae1 = toDAE(var_field!((*exp).exp1, NFExpression::BINARY).clone(), false)?;
            dae2 = toDAE(if (negate) {self::negate(var_field!((*exp).exp2, NFExpression::BINARY).clone())} else {var_field!((*exp).exp2, NFExpression::BINARY).clone()}, false)?;
            return Ok(Arc::new(DAE::Exp::BINARY { exp1: if (swap) {dae2.clone()} else {dae1.clone()}, operator: daeOp, exp2: if (swap) {dae1} else {dae2} }))
        },
        Deref @ UNARY { .. } => {
            return Ok(Arc::new(DAE::Exp::UNARY { operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::UNARY).clone())?.0, exp: toDAE(var_field!((*exp).exp, NFExpression::UNARY).clone(), false)? }))
        },
        Deref @ LBINARY { .. } => {
            return Ok(Arc::new(DAE::Exp::LBINARY { exp1: toDAE(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), false)?, operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::LBINARY).clone())?.0, exp2: toDAE(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), false)? }))
        },
        Deref @ LUNARY { .. } => {
            return Ok(Arc::new(DAE::Exp::LUNARY { operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::LUNARY).clone())?.0, exp: toDAE(var_field!((*exp).exp, NFExpression::LUNARY).clone(), false)? }))
        },
        Deref @ RELATION { .. } => {
            return Ok(Arc::new(DAE::Exp::RELATION { exp1: toDAE(var_field!((*exp).exp1, NFExpression::RELATION).clone(), false)?, operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::RELATION).clone())?.0, exp2: toDAE(var_field!((*exp).exp2, NFExpression::RELATION).clone(), false)?, index: var_field!((*exp).index, NFExpression::RELATION).clone(), optionExpisASUB: None }))
        },
        Deref @ IF { .. } => {
            return Ok(Arc::new(DAE::Exp::IFEXP { expCond: toDAE(var_field!((*exp).condition, NFExpression::IF).clone(), false)?, expThen: toDAE(var_field!((*exp).trueBranch, NFExpression::IF).clone(), false)?, expElse: toDAE(var_field!((*exp).falseBranch, NFExpression::IF).clone(), false)? }))
        },
        Deref @ CAST { .. } => {
            return Ok(Arc::new(DAE::Exp::CAST { ty: Type::toDAE(var_field!((*exp).ty, NFExpression::CAST).clone(), true)?, exp: toDAE(var_field!((*exp).exp, NFExpression::CAST).clone(), false)? }))
        },
        Deref @ BOX { .. } => {
            return Ok(Arc::new(DAE::Exp::BOX { exp: toDAE(var_field!((*exp).exp, NFExpression::BOX).clone(), false)? }))
        },
        Deref @ UNBOX { .. } => {
            return Ok(Arc::new(DAE::Exp::UNBOX { exp: toDAE(var_field!((*exp).exp, NFExpression::UNBOX).clone(), false)?, ty: Type::toDAE(var_field!((*exp).ty, NFExpression::UNBOX).clone(), true)? }))
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            return Ok(Arc::new(DAE::Exp::ASUB { exp: toDAE(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?, sub: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::toDAE(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            return Ok(Arc::new(DAE::Exp::TSUB { exp: toDAE(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), false)?, ix: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: Type::toDAE(var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone(), true)? }))
        },
        Deref @ RECORD_ELEMENT { .. } => {
            return Ok(Arc::new(DAE::Exp::RSUB { exp: toDAE(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), false)?, ix: -1, fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: Type::toDAE(var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone(), true)? }))
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut r#fn: Arc<Function::Function::Function>;
            let __pa0 = ::match_deref::match_deref! { match &(Function::Function::typeRefCache(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            return Ok(Arc::new(DAE::Exp::PARTEVALFUNCTION { path: Function::Function::nameConsiderBuiltin(r#fn.clone())?, expList: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = toDAE(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: Type::toDAE(var_field!((*exp).ty, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), true)?, origType: Type::toDAE(Arc::new(Type::NFType::FUNCTION { r#fn: r#fn, fnType: Type::FunctionType::FUNCTIONAL_VARIABLE.clone() }), true)? }))
        },
        Deref @ MUTABLE { .. } => {
            { (exp, allowEmpty) = (Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), false); continue '__tco; }
        },
        Deref @ EMPTY { .. } if (allowEmpty) => {
            let mut dty: Arc<DAE::Type>;
            dty = Type::toDAE(var_field!((*exp).ty, NFExpression::EMPTY).clone(), true)?;
            return Ok(Arc::new(DAE::Exp::EMPTY { scope: (literal!("")).clone(), name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$dummy")).clone(), identType: dty.clone(), subscriptLst: metamodelica::nil() }), ty: dty, tyStr: (Type::toString(var_field!((*exp).ty, NFExpression::EMPTY).clone())?).clone() }))
        },
        Deref @ SHARED_LITERAL { .. } => {
            return Ok(Arc::new(DAE::Exp::SHARED_LITERAL { index: var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone(), exp: toDAE(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), false)? }))
        },
        Deref @ FILENAME { .. } => {
            if (Flags::getConfigBool(Flags::BUILDING_FMU.clone())?) {return Ok(Arc::new(DAE::Exp::CALL { path: Arc::new(Path::IDENT { name: (literal!("OpenModelica_fmuLoadResource")).clone() }), expLst: list![Arc::new(DAE::Exp::SCONST { string: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() })], attr: DAE::callAttrBuiltinImpureString().clone() }))} else {return Ok(Arc::new(DAE::Exp::SCONST { string: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() }))}
        },
        Deref @ INSTANCE_NAME { .. } => {
            return Ok(Arc::new(DAE::Exp::CALL { path: Arc::new(Path::IDENT { name: (literal!("getInstanceName")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinString().clone() }))
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toDAE")); __mm_s.push_str(&*literal!(" got unknown expression '")); __mm_s.push_str(&*toString(exp)?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn toDAERecord(mut ty: Arc<Type::NFType>, mut path: Arc<Path>, mut args: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut field_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut arg: Arc<NFExpression>;
    let mut rest_args: Arc<metamodelica::List<Arc<NFExpression>>> = args.clone();
    let mut dargs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    for mut field in &*Type::recordFields(Type::unbox(ty.clone())) {
        let mut field = field.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(field.clone()) {
        Deref @ Record::Field::INPUT { .. } => {
            field_names = metamodelica::cons((var_field!((*field).name, Record::Field::Field::INPUT).clone()).clone(), field_names.clone());
            dargs = metamodelica::cons(toDAE(arg.clone(), true)?, dargs.clone());
            ()
        },
        Deref @ Record::Field::LOCAL { .. } => {
            field_names = metamodelica::cons((var_field!((*field).name, Record::Field::Field::LOCAL).clone()).clone(), field_names.clone());
            dargs = metamodelica::cons(toDAE(arg.clone(), true)?, dargs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    field_names = metamodelica::Dangerous::listReverseInPlace(field_names);
    dargs = metamodelica::Dangerous::listReverseInPlace(dargs);
    exp = if (Type::isBoxed(ty.clone())) {Arc::new(DAE::Exp::METARECORDCALL { path: path, args: dargs, fieldNames: field_names, index: -1, typeVars: metamodelica::nil() })} else {Arc::new(DAE::Exp::RECORD { path: path, exps: dargs, comp: field_names, ty: Type::toDAE(ty, true)? })};
    Ok(exp)
}

pub(crate) fn toDAEValue(mut exp: Arc<NFExpression>) -> Result<Arc<Values::Value>> {
    let mut value: Arc<Values::Value>;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Values::Value::INTEGER { integer: var_field!((*exp).value, NFExpression::INTEGER).clone() })
        },
        Deref @ REAL { .. } => {
            Arc::new(Values::Value::REAL { real: var_field!((*exp).value, NFExpression::REAL).clone() })
        },
        Deref @ STRING { .. } => {
            Arc::new(Values::Value::STRING { string: (var_field!((*exp).value, NFExpression::STRING).clone()).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Values::Value::BOOL { boolean: var_field!((*exp).value, NFExpression::BOOLEAN).clone() })
        },
        Deref @ ENUM_LITERAL { ty: ty @ Deref @ Type::ENUMERATION { .. }, .. } => {
            Arc::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), (var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()).clone())?, index: var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone() })
        },
        Deref @ ARRAY { .. } => {
            ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toDAEValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))
        },
        Deref @ RECORD { .. } => {
            toDAEValueRecord(var_field!((*exp).ty, NFExpression::RECORD).clone(), var_field!((*exp).path, NFExpression::RECORD).clone(), var_field!((*exp).elements, NFExpression::RECORD).clone())?
        },
        Deref @ FILENAME { .. } => {
            Arc::new(Values::Value::STRING { string: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toDAEValue")); __mm_s.push_str(&*literal!(" got unhandled expression ")); __mm_s.push_str(&*toString(exp)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub(crate) fn toDAEValueRecord(mut ty: Arc<Type::NFType>, mut path: Arc<Path>, mut args: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<Values::Value>> {
    let mut value: Arc<Values::Value>;
    let mut field_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut arg: Arc<NFExpression>;
    let mut rest_args: Arc<metamodelica::List<Arc<NFExpression>>> = args.clone();
    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    for mut field in &*Type::recordFields(ty) {
        let mut field = field.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(field.clone()) {
        Deref @ Record::Field::INPUT { .. } => {
            field_names = metamodelica::cons((var_field!((*field).name, Record::Field::Field::INPUT).clone()).clone(), field_names.clone());
            values = metamodelica::cons(toDAEValue(arg.clone())?, values.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    field_names = metamodelica::Dangerous::listReverseInPlace(field_names);
    values = metamodelica::Dangerous::listReverseInPlace(values);
    value = Arc::new(Values::Value::RECORD { record_: path, orderd: values, comp: field_names, index: -1 });
    Ok(value)
}

pub(crate) fn dimensionCount(mut exp: Arc<NFExpression>) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { ty: Deref @ Type::UNKNOWN, .. } => return Ok(1 + dimensionCount(metamodelica::arrayGet(var_field!((*exp).elements, NFExpression::ARRAY).clone(), 1)?)?),
        Deref @ ARRAY { .. } => return Ok(Type::dimensionCount(var_field!((*exp).ty, NFExpression::ARRAY).clone())),
        Deref @ RANGE { .. } => return Ok(Type::dimensionCount(var_field!((*exp).ty, NFExpression::RANGE).clone())),
        Deref @ SIZE { dimIndex: None, .. } => { exp = var_field!((*exp).exp, NFExpression::SIZE).clone(); continue '__tco; },
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ SUBSCRIPTED_EXP { .. } => return Ok(Type::dimensionCount(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone())),
        Deref @ TUPLE_ELEMENT { .. } => return Ok(Type::dimensionCount(var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone())),
        _ => return Ok(0),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn dimensions(mut exp: Arc<NFExpression>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    dims = Type::arrayDims(typeOf(exp));
    dims
}

pub fn map(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            Arc::new(NFExpression::CLKCONST { clk: ClockKind::mapExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())? })
        },
        Deref @ CREF { .. } => {
            Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: ComponentRef::mapExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())? })
        },
        Deref @ ARRAY { .. } if (!(var_field!((*exp).literal, NFExpression::ARRAY).clone())) => {
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static> = func.clone(); move |__pe_a0| map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?, var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            Arc::new(NFExpression::MATRIX { elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            let mut e4: Arc<NFExpression>;
            e1 = map(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e4 = map(e2.clone(), func.clone())?;
            e3 = map(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e4.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = map(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e3 = map(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { .. } => {
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RECORD { .. } => {
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ CALL { .. } => {
            Arc::new(NFExpression::CALL { call: Call::mapExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            e3 = map(e2.clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())?;
            e2 = map(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::BINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::BINARY).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNARY).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1 })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())?;
            e2 = map(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::LBINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::LBINARY).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::LUNARY).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1 })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())?;
            e2 = map(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::RELATION).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::RELATION).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1, operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = map(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())?;
            e2 = map(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())?;
            e3 = map(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).condition, NFExpression::IF).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).trueBranch, NFExpression::IF).clone()),&*(e2.clone())) && referenceEq(&*(var_field!((*exp).falseBranch, NFExpression::IF).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1, trueBranch: e2.clone(), falseBranch: e3 })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::CAST).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1 })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::BOX).clone()),&*(e1.clone()))) {exp.clone()} else {r#box(e1)}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNBOX).clone()),&*(e1.clone()))) {exp.clone()} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: map(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?, subscripts: ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = map(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), map(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = map(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp = func(outExp)?;
    Ok(outExp)
}

pub fn fakeMap(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression> = func(exp.clone())?;
    Ok(outExp)
}

pub(crate) fn mapOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Option<Arc<NFExpression>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Option<Arc<NFExpression>>;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            Some(map(e.clone(), func.clone())?)
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn mapReverse(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut exp: Arc<NFExpression> = exp;
    exp = func(exp)?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            Arc::new(NFExpression::CLKCONST { clk: ClockKind::mapExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())? })
        },
        Deref @ CREF { .. } => {
            Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: ComponentRef::mapExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())? })
        },
        Deref @ ARRAY { .. } if (!(var_field!((*exp).literal, NFExpression::ARRAY).clone())) => {
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static> = func.clone(); move |__pe_a0| mapReverse(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?, var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            Arc::new(NFExpression::MATRIX { elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            let mut e4: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e4 = mapReverse(e2.clone(), func.clone())?;
            e3 = mapReverse(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e4.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e3 = mapReverse(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { .. } => {
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RECORD { .. } => {
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ CALL { .. } => {
            Arc::new(NFExpression::CALL { call: Call::mapExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            e3 = mapReverse(e2.clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::BINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::BINARY).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = mapReverse(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = mapReverse(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNARY).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1 })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::LBINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::LBINARY).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::LUNARY).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1 })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::RELATION).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::RELATION).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1, operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())?;
            e3 = mapReverse(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).condition, NFExpression::IF).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).trueBranch, NFExpression::IF).clone()),&*(e2.clone())) && referenceEq(&*(var_field!((*exp).falseBranch, NFExpression::IF).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1, trueBranch: e2.clone(), falseBranch: e3 })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::CAST).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1 })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::BOX).clone()),&*(e1.clone()))) {exp.clone()} else {r#box(e1)}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNBOX).clone()),&*(e1.clone()))) {exp.clone()} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: mapReverse(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?, subscripts: ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = mapReverse(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&*(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), mapReverse(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = mapReverse(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn mapShallow(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            Arc::new(NFExpression::CLKCONST { clk: ClockKind::mapExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())? })
        },
        Deref @ CREF { .. } => {
            Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: ComponentRef::mapExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())? })
        },
        Deref @ ARRAY { .. } if (!(var_field!((*exp).literal, NFExpression::ARRAY).clone())) => {
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?, var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            Arc::new(NFExpression::MATRIX { elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            let mut e4: Arc<NFExpression>;
            e1 = func(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            e4 = func(e2.clone())?;
            e3 = func(var_field!((*exp).stop, NFExpression::RANGE).clone())?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e4.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = func(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            e3 = func(var_field!((*exp).stop, NFExpression::RANGE).clone())?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { .. } => {
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RECORD { .. } => {
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ CALL { .. } => {
            Arc::new(NFExpression::CALL { call: Call::mapExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::SIZE).clone())?;
            e3 = func(e2.clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::SIZE).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp1, NFExpression::BINARY).clone())?;
            e2 = func(var_field!((*exp).exp2, NFExpression::BINARY).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::BINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::BINARY).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::UNARY).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNARY).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1 })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?;
            e2 = func(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::LBINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::LBINARY).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::LUNARY).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::LUNARY).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1 })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp1, NFExpression::RELATION).clone())?;
            e2 = func(var_field!((*exp).exp2, NFExpression::RELATION).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::RELATION).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::RELATION).clone()),&*(e2.clone()))) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1, operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            e1 = func(var_field!((*exp).condition, NFExpression::IF).clone())?;
            e2 = func(var_field!((*exp).trueBranch, NFExpression::IF).clone())?;
            e3 = func(var_field!((*exp).falseBranch, NFExpression::IF).clone())?;
            if (referenceEq(&*(var_field!((*exp).condition, NFExpression::IF).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).trueBranch, NFExpression::IF).clone()),&*(e2.clone())) && referenceEq(&*(var_field!((*exp).falseBranch, NFExpression::IF).clone()),&*(e3.clone()))) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1, trueBranch: e2.clone(), falseBranch: e3 })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::CAST).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::CAST).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1 })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::BOX).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::BOX).clone()),&*(e1.clone()))) {exp.clone()} else {r#box(e1)}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).exp, NFExpression::UNBOX).clone())?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNBOX).clone()),&*(e1.clone()))) {exp.clone()} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, subscripts: ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?;
            if (referenceEq(&*(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            e1 = func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?;
            if (referenceEq(&*(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone()),&*(e1.clone()))) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn mapShallowOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Option<Arc<NFExpression>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Option<Arc<NFExpression>>;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            Some(func(e.clone())?)
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn mapArrayElements(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static> = func.clone(); move |__pe_a0| mapArrayElements(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?);
            assign_variant_field!(exp => NFExpression::ARRAY; literal = Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?);
            exp
        },
        _ => func(exp)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn foldArray<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT = arg.clone();
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        result = fold(e.clone(), func.clone(), result.clone())?;
    }
    Ok(result)
}

pub(crate) fn foldList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT = arg.clone();
    for mut e in &*expl {
        let mut e = e.clone();
        result = fold(e.clone(), func.clone(), result.clone())?;
    }
    Ok(result)
}

pub(crate) fn foldOpt<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(exp) {
        Some(e) => {
            func(e.clone(), arg)?
        },
        _ => {
            arg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn fold<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::foldExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone(), arg)?
        },
        Deref @ CREF { .. } => {
            ComponentRef::foldExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone(), arg)?
        },
        Deref @ ARRAY { .. } => {
            foldArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone(), arg)?
        },
        Deref @ MATRIX { .. } => {
            result = arg;
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                result = foldList(row.clone(), func.clone(), result.clone())?;
            }
            result
        },
        Deref @ RANGE { .. } => {
            result = fold(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone(), arg)?;
            result = foldOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone(), result)?;
            fold(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone(), result)?
        },
        Deref @ TUPLE { .. } => {
            foldList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone(), arg)?
        },
        Deref @ RECORD { .. } => {
            foldList(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone(), arg)?
        },
        Deref @ CALL { .. } => {
            Call::foldExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone(), arg)?
        },
        Deref @ SIZE { dimIndex: Some(e), .. } => {
            result = fold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg)?;
            fold(e.clone(), func.clone(), result)?
        },
        Deref @ SIZE { .. } => {
            fold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg)?
        },
        Deref @ BINARY { .. } => {
            result = fold(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone(), arg)?;
            fold(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone(), result)?
        },
        Deref @ MULTARY { .. } => {
            result = arg;
            for mut argument in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                result = fold(argument.clone(), func.clone(), result.clone())?;
            }
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                result = fold(argument.clone(), func.clone(), result.clone())?;
            }
            result
        },
        Deref @ UNARY { .. } => {
            fold(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone(), arg)?
        },
        Deref @ LBINARY { .. } => {
            result = fold(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone(), arg)?;
            fold(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone(), result)?
        },
        Deref @ LUNARY { .. } => {
            fold(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone(), arg)?
        },
        Deref @ RELATION { .. } => {
            result = fold(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone(), arg)?;
            fold(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone(), result)?
        },
        Deref @ IF { .. } => {
            result = fold(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone(), arg)?;
            result = fold(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone(), result)?;
            fold(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone(), result)?
        },
        Deref @ CAST { .. } => {
            fold(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone(), arg)?
        },
        Deref @ BOX { .. } => {
            fold(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone(), arg)?
        },
        Deref @ UNBOX { .. } => {
            fold(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone(), arg)?
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            result = fold(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone(), arg)?;
            List::fold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::foldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>, _) -> Result<_> + 'static>), result)?
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            fold(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone(), arg)?
        },
        Deref @ RECORD_ELEMENT { .. } => {
            fold(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone(), arg)?
        },
        Deref @ MUTABLE { .. } => {
            fold(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone(), arg)?
        },
        Deref @ SHARED_LITERAL { .. } => {
            fold(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone(), arg)?
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            foldList(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone(), arg)?
        },
        _ => {
            arg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = func(exp, result)?;
    Ok(result)
}

pub(crate) fn applyArray(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        apply(e.clone(), func.clone())?;
    }
    Ok(())
}

pub(crate) fn applyList(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    for mut e in &*expl {
        let mut e = e.clone();
        apply(e.clone(), func.clone())?;
    }
    Ok(())
}

pub(crate) fn applyOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    let mut e: Arc<NFExpression>;
    if isSome(exp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(exp) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        apply(e, func.clone())?;
    }
    Ok(())
}

pub(crate) fn apply(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::applyExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())?;
            ()
        },
        Deref @ CREF { .. } => {
            ComponentRef::applyExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?;
            ()
        },
        Deref @ ARRAY { .. } => {
            applyArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?;
            ()
        },
        Deref @ MATRIX { .. } => {
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                applyList(row.clone(), func.clone())?;
            }
            ()
        },
        Deref @ RANGE { .. } => {
            apply(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            applyOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone())?;
            apply(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            ()
        },
        Deref @ TUPLE { .. } => {
            applyList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone())?;
            ()
        },
        Deref @ RECORD { .. } => {
            applyList(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone())?;
            ()
        },
        Deref @ CALL { .. } => {
            Call::applyExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?;
            ()
        },
        Deref @ SIZE { .. } => {
            apply(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            applyOpt(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone())?;
            ()
        },
        Deref @ BINARY { .. } => {
            apply(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())?;
            apply(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?;
            ()
        },
        Deref @ MULTARY { .. } => {
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                apply(arg.clone(), func.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                apply(arg.clone(), func.clone())?;
            }
            ()
        },
        Deref @ UNARY { .. } => {
            apply(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?;
            ()
        },
        Deref @ LBINARY { .. } => {
            apply(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())?;
            apply(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?;
            ()
        },
        Deref @ LUNARY { .. } => {
            apply(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?;
            ()
        },
        Deref @ RELATION { .. } => {
            apply(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())?;
            apply(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?;
            ()
        },
        Deref @ IF { .. } => {
            apply(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())?;
            apply(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())?;
            apply(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?;
            ()
        },
        Deref @ CAST { .. } => {
            apply(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?;
            ()
        },
        Deref @ BOX { .. } => {
            apply(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?;
            ()
        },
        Deref @ UNBOX { .. } => {
            apply(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?;
            ()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            apply(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?;
            for mut s in &*var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut s = s.clone();
                Subscript::applyExp(s.clone(), func.clone())?;
            }
            ()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            apply(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?;
            ()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            apply(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?;
            ()
        },
        Deref @ MUTABLE { .. } => {
            apply(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?;
            ()
        },
        Deref @ SHARED_LITERAL { .. } => {
            apply(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?;
            ()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            applyList(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    func(exp)?;
    Ok(())
}

pub(crate) fn applyArrayShallow(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        func(e.clone())?;
    }
    Ok(())
}

pub(crate) fn applyListShallow(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    for mut e in &*expl {
        let mut e = e.clone();
        func(e.clone())?;
    }
    Ok(())
}

pub(crate) fn applyShallow(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::applyExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())?;
            ()
        },
        Deref @ CREF { .. } => {
            ComponentRef::applyExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?;
            ()
        },
        Deref @ ARRAY { .. } => {
            applyArrayShallow(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?;
            ()
        },
        Deref @ MATRIX { .. } => {
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                applyListShallow(row.clone(), func.clone())?;
            }
            ()
        },
        Deref @ RANGE { .. } => {
            func(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            applyShallowOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone())?;
            func(var_field!((*exp).stop, NFExpression::RANGE).clone())?;
            ()
        },
        Deref @ TUPLE { .. } => {
            applyListShallow(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone())?;
            ()
        },
        Deref @ RECORD { .. } => {
            applyListShallow(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone())?;
            ()
        },
        Deref @ CALL { .. } => {
            Call::applyExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?;
            ()
        },
        Deref @ SIZE { .. } => {
            func(var_field!((*exp).exp, NFExpression::SIZE).clone())?;
            applyShallowOpt(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone())?;
            ()
        },
        Deref @ BINARY { .. } => {
            func(var_field!((*exp).exp1, NFExpression::BINARY).clone())?;
            func(var_field!((*exp).exp2, NFExpression::BINARY).clone())?;
            ()
        },
        Deref @ MULTARY { .. } => {
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                func(arg.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                func(arg.clone())?;
            }
            ()
        },
        Deref @ UNARY { .. } => {
            func(var_field!((*exp).exp, NFExpression::UNARY).clone())?;
            ()
        },
        Deref @ LBINARY { .. } => {
            func(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?;
            func(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?;
            ()
        },
        Deref @ LUNARY { .. } => {
            func(var_field!((*exp).exp, NFExpression::LUNARY).clone())?;
            ()
        },
        Deref @ RELATION { .. } => {
            func(var_field!((*exp).exp1, NFExpression::RELATION).clone())?;
            func(var_field!((*exp).exp2, NFExpression::RELATION).clone())?;
            ()
        },
        Deref @ IF { .. } => {
            func(var_field!((*exp).condition, NFExpression::IF).clone())?;
            func(var_field!((*exp).trueBranch, NFExpression::IF).clone())?;
            func(var_field!((*exp).falseBranch, NFExpression::IF).clone())?;
            ()
        },
        Deref @ CAST { .. } => {
            func(var_field!((*exp).exp, NFExpression::CAST).clone())?;
            ()
        },
        Deref @ BOX { .. } => {
            func(var_field!((*exp).exp, NFExpression::BOX).clone())?;
            ()
        },
        Deref @ UNBOX { .. } => {
            func(var_field!((*exp).exp, NFExpression::UNBOX).clone())?;
            ()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?;
            for mut s in &*var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut s = s.clone();
                Subscript::applyExpShallow(s.clone(), func.clone())?;
            }
            ()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?;
            ()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?;
            ()
        },
        Deref @ MUTABLE { .. } => {
            func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?;
            ()
        },
        Deref @ SHARED_LITERAL { .. } => {
            func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?;
            ()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            applyListShallow(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn applyShallowOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>;

    let mut e: Arc<NFExpression>;
    if isSome(exp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(exp) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        func(e)?;
    }
    Ok(())
}

pub fn mapFold<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFExpression>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>;

    let mut outExp: Arc<NFExpression>;
    let mut arg: ArgT = arg;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            let mut ck: Arc<ClockKind::NFClockKind>;
            (ck, arg) = ClockKind::mapFoldExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).clk, NFExpression::CLKCONST).clone()),&*(ck.clone()))) {exp} else {Arc::new(NFExpression::CLKCONST { clk: ck })}
        },
        Deref @ CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef>;
            (cr, arg) = ComponentRef::mapFoldExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).cref, NFExpression::CREF).clone()),&*(cr.clone()))) {exp} else {Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: cr })}
        },
        Deref @ ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            (arr, arg) = Array::mapFold(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| mapFold(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static>), arg)?;
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), arr.clone(), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>>;
            (mat, arg) = List::mapFoldList(var_field!((*exp).elements, NFExpression::MATRIX).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| mapFold(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static>), arg)?;
            Arc::new(NFExpression::MATRIX { elements: mat })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            let mut e4: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone(), arg)?;
            (e4, arg) = mapFold(e2.clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e4.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).start, NFExpression::RANGE).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).stop, NFExpression::RANGE).clone()),&*(e3.clone()))) {exp} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            (expl, arg) = List::map1Fold(var_field!((*exp).elements, NFExpression::TUPLE).clone(), (std::sync::Arc::new(mapFold) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _, _) -> Result<_> + 'static>), func.clone(), arg)?;
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: expl })
        },
        Deref @ RECORD { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            (expl, arg) = List::map1Fold(var_field!((*exp).elements, NFExpression::RECORD).clone(), (std::sync::Arc::new(mapFold) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _, _) -> Result<_> + 'static>), func.clone(), arg)?;
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: expl })
        },
        Deref @ CALL { .. } => {
            let mut call: Arc<Call::NFCall>;
            (call, arg) = Call::mapFoldExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).call, NFExpression::CALL).clone()),&*(call.clone()))) {exp} else {Arc::new(NFExpression::CALL { call: call })}
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone())) && referenceEq(&*(e2.clone()),&*(e3.clone()))) {exp} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::BINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::BINARY).clone()),&*(e2.clone()))) {exp} else {Arc::new(NFExpression::BINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = mapFold(argument.clone(), func.clone(), arg.clone())?;
                expl = metamodelica::cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; arguments = expl.reverse());
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = mapFold(argument.clone(), func.clone(), arg.clone())?;
                expl = metamodelica::cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; inv_arguments = expl.reverse());
            exp
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNARY).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1 })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::LBINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::LBINARY).clone()),&*(e2.clone()))) {exp} else {Arc::new(NFExpression::LBINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::LUNARY).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1 })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::RELATION).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::RELATION).clone()),&*(e2.clone()))) {exp} else {Arc::new(NFExpression::RELATION { exp1: e1, operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).condition, NFExpression::IF).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).trueBranch, NFExpression::IF).clone()),&*(e2.clone())) && referenceEq(&*(var_field!((*exp).falseBranch, NFExpression::IF).clone()),&*(e3.clone()))) {exp} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1, trueBranch: e2.clone(), falseBranch: e3 })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::CAST).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1 })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::BOX).clone()),&*(e1.clone()))) {exp} else {r#box(e1)}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNBOX).clone()),&*(e1.clone()))) {exp} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone(), arg)?;
            (subs, arg) = List::mapFold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::mapFoldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>, _) -> Result<_> + 'static>), arg)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e1, subscripts: subs, ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone(), arg)?;
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), e1);
            exp
        },
        Deref @ SHARED_LITERAL { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone(), arg)?;
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = e1);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            (expl, arg) = List::map1Fold(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), (std::sync::Arc::new(mapFold) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _, _) -> Result<_> + 'static>), func.clone(), arg)?;
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = expl);
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, arg) = func(outExp, arg)?;
    Ok((outExp, arg))
}

pub(crate) fn mapFoldOpt<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Option<Arc<NFExpression>>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>;

    let mut outExp: Option<Arc<NFExpression>>;
    let mut arg: ArgT = arg;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            (e, arg) = mapFold(e.clone(), func.clone(), arg)?;
            Some(e.clone())
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub fn mapFoldShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFExpression>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>;

    let mut outExp: Arc<NFExpression>;
    let mut arg: ArgT = arg;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            let mut ck: Arc<ClockKind::NFClockKind>;
            (ck, arg) = ClockKind::mapFoldExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).clk, NFExpression::CLKCONST).clone()),&*(ck.clone()))) {exp} else {Arc::new(NFExpression::CLKCONST { clk: ck })}
        },
        Deref @ CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef>;
            (cr, arg) = ComponentRef::mapFoldExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).cref, NFExpression::CREF).clone()),&*(cr.clone()))) {exp} else {Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: cr })}
        },
        Deref @ ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            (arr, arg) = Array::mapFold(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone(), arg)?;
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), arr.clone(), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>>;
            (mat, arg) = List::mapFoldList(var_field!((*exp).elements, NFExpression::MATRIX).clone(), func.clone(), arg)?;
            Arc::new(NFExpression::MATRIX { elements: mat })
        },
        Deref @ RANGE { step: oe, .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            let mut oe = (*oe).clone();
            (e1, arg) = func(var_field!((*exp).start, NFExpression::RANGE).clone(), arg)?;
            (oe, arg) = mapFoldOptShallow(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone(), arg)?;
            (e3, arg) = func(var_field!((*exp).stop, NFExpression::RANGE).clone(), arg)?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*exp).start, NFExpression::RANGE).clone())) && (match (&(oe.clone()), &(var_field!((*exp).step, NFExpression::RANGE).clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && referenceEq(&*(e3.clone()),&*(var_field!((*exp).stop, NFExpression::RANGE).clone()))) {exp} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1, step: oe.clone(), stop: e3 })}
        },
        Deref @ TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            (expl, arg) = List::mapFold(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone(), arg)?;
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: expl })
        },
        Deref @ RECORD { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            (expl, arg) = List::mapFold(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone(), arg)?;
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: expl })
        },
        Deref @ CALL { .. } => {
            let mut call: Arc<Call::NFCall>;
            (call, arg) = Call::mapFoldExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).call, NFExpression::CALL).clone()),&*(call.clone()))) {exp} else {Arc::new(NFExpression::CALL { call: call })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut oe: Option<Arc<NFExpression>>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::SIZE).clone(), arg)?;
            (oe, arg) = mapFoldOptShallow(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::SIZE).clone()),&*(e1.clone())) && (match (&(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()), &(oe.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {exp} else {Arc::new(NFExpression::SIZE { exp: e1, dimIndex: oe.clone() })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp1, NFExpression::BINARY).clone(), arg)?;
            (e2, arg) = func(var_field!((*exp).exp2, NFExpression::BINARY).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::BINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::BINARY).clone()),&*(e2.clone()))) {exp} else {Arc::new(NFExpression::BINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2 })}
        },
        Deref @ MULTARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = func(argument.clone(), arg.clone())?;
                expl = metamodelica::cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; arguments = expl.reverse());
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = func(argument.clone(), arg.clone())?;
                expl = metamodelica::cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; inv_arguments = expl.reverse());
            exp
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::UNARY).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNARY).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1 })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), arg)?;
            (e2, arg) = func(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::LBINARY).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::LBINARY).clone()),&*(e2.clone()))) {exp} else {Arc::new(NFExpression::LBINARY { exp1: e1, operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2 })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::LUNARY).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::LUNARY).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1 })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp1, NFExpression::RELATION).clone(), arg)?;
            (e2, arg) = func(var_field!((*exp).exp2, NFExpression::RELATION).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp1, NFExpression::RELATION).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).exp2, NFExpression::RELATION).clone()),&*(e2.clone()))) {exp} else {Arc::new(NFExpression::RELATION { exp1: e1, operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2, index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut e2: Arc<NFExpression>;
            let mut e3: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).condition, NFExpression::IF).clone(), arg)?;
            (e2, arg) = func(var_field!((*exp).trueBranch, NFExpression::IF).clone(), arg)?;
            (e3, arg) = func(var_field!((*exp).falseBranch, NFExpression::IF).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).condition, NFExpression::IF).clone()),&*(e1.clone())) && referenceEq(&*(var_field!((*exp).trueBranch, NFExpression::IF).clone()),&*(e2.clone())) && referenceEq(&*(var_field!((*exp).falseBranch, NFExpression::IF).clone()),&*(e3.clone()))) {exp} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1, trueBranch: e2, falseBranch: e3 })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::CAST).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::CAST).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1 })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::BOX).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::BOX).clone()),&*(e1.clone()))) {exp} else {r#box(e1)}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::UNBOX).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).exp, NFExpression::UNBOX).clone()),&*(e1.clone()))) {exp} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<NFExpression>;
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), arg)?;
            (subs, arg) = List::mapFold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::mapFoldExpShallow(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>, _) -> Result<_> + 'static>), arg)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e1, subscripts: subs, ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), arg)?;
            if (referenceEq(&*(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone()),&*(e1.clone()))) {exp} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), arg)?;
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), e1);
            exp
        },
        Deref @ SHARED_LITERAL { .. } => {
            let mut e1: Arc<NFExpression>;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), arg)?;
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = e1);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
            (expl, arg) = List::mapFold(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone(), arg)?;
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = expl);
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub(crate) fn mapFoldOptShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Option<Arc<NFExpression>>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>;

    let mut outExp: Option<Arc<NFExpression>>;
    let mut arg: ArgT = arg;
    let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e1) => {
            e1 = (*__esc_e1).clone();
            (e2, arg) = func(e1.clone(), arg)?;
            if (referenceEq(&*(e1.clone()),&*(e2.clone()))) {exp} else {Some(e2)}
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub(crate) fn containsOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    res = (::match_deref::match_deref! { match &(exp) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            contains(e.clone(), func.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn contains(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    if func(exp.clone())? {
        res = true;
        return Ok(res.clone());
    }
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => ClockKind::containsExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())?,
        Deref @ CREF { .. } => ComponentRef::containsExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?,
        Deref @ ARRAY { .. } => arrayContains(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?,
        Deref @ MATRIX { .. } => {
            res = false;
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                if listContains(row.clone(), func.clone())? {
                    res = true;
                    break;
                }
            }
            res
        },
        Deref @ RANGE { .. } => contains(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())? || containsOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone())? || contains(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?,
        Deref @ TUPLE { .. } => listContains(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone())?,
        Deref @ RECORD { .. } => listContains(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone())?,
        Deref @ CALL { .. } => Call::containsExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?,
        Deref @ SIZE { .. } => containsOpt(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone())? || contains(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?,
        Deref @ BINARY { .. } => contains(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())? || contains(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?,
        Deref @ MULTARY { .. } => {
            res = false;
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res {
                    break;
                }
                res = contains(arg.clone(), func.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res {
                    break;
                }
                res = contains(arg.clone(), func.clone())?;
            }
            res
        },
        Deref @ UNARY { .. } => contains(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?,
        Deref @ LBINARY { .. } => contains(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())? || contains(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?,
        Deref @ LUNARY { .. } => contains(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?,
        Deref @ RELATION { .. } => contains(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())? || contains(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?,
        Deref @ IF { .. } => contains(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())? || contains(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())? || contains(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?,
        Deref @ CAST { .. } => contains(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?,
        Deref @ BOX { .. } => contains(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?,
        Deref @ UNBOX { .. } => contains(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => contains(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())? || Subscript::listContainsExp(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?,
        Deref @ TUPLE_ELEMENT { .. } => contains(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?,
        Deref @ RECORD_ELEMENT { .. } => contains(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?,
        Deref @ MUTABLE { .. } => contains(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?,
        Deref @ SHARED_LITERAL { .. } => contains(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => listContains(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn arrayContains(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if contains(e.clone(), func.clone())? {
            res = true;
            return Ok(res.clone());
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn listContains(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut e in &*expl {
        let mut e = e.clone();
        if contains(e.clone(), func.clone())? {
            res = true;
            return Ok(res.clone());
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn containsShallow(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => ClockKind::containsExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())?,
        Deref @ CREF { .. } => ComponentRef::containsExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?,
        Deref @ ARRAY { .. } => Array::any(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?,
        Deref @ MATRIX { .. } => {
            res = false;
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                if List::any(row.clone(), func.clone())? {
                    res = true;
                    break;
                }
            }
            res
        },
        Deref @ RANGE { .. } => func(var_field!((*exp).start, NFExpression::RANGE).clone())? || Util::applyOptionOrDefault(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone(), false)? || func(var_field!((*exp).stop, NFExpression::RANGE).clone())?,
        Deref @ TUPLE { .. } => List::any(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone())?,
        Deref @ RECORD { .. } => List::any(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone())?,
        Deref @ CALL { .. } => Call::containsExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?,
        Deref @ SIZE { .. } => Util::applyOptionOrDefault(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone(), false)? || func(var_field!((*exp).exp, NFExpression::SIZE).clone())?,
        Deref @ BINARY { .. } => func(var_field!((*exp).exp1, NFExpression::BINARY).clone())? || func(var_field!((*exp).exp2, NFExpression::BINARY).clone())?,
        Deref @ MULTARY { .. } => {
            res = false;
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res {
                    break;
                }
                res = func(arg.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res {
                    break;
                }
                res = func(arg.clone())?;
            }
            res
        },
        Deref @ UNARY { .. } => func(var_field!((*exp).exp, NFExpression::UNARY).clone())?,
        Deref @ LBINARY { .. } => func(var_field!((*exp).exp1, NFExpression::LBINARY).clone())? || func(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?,
        Deref @ LUNARY { .. } => func(var_field!((*exp).exp, NFExpression::LUNARY).clone())?,
        Deref @ RELATION { .. } => func(var_field!((*exp).exp1, NFExpression::RELATION).clone())? || func(var_field!((*exp).exp2, NFExpression::RELATION).clone())?,
        Deref @ IF { .. } => func(var_field!((*exp).condition, NFExpression::IF).clone())? || func(var_field!((*exp).trueBranch, NFExpression::IF).clone())? || func(var_field!((*exp).falseBranch, NFExpression::IF).clone())?,
        Deref @ CAST { .. } => func(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => func(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => func(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())? || Subscript::listContainsExpShallow(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?,
        Deref @ TUPLE_ELEMENT { .. } => func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?,
        Deref @ RECORD_ELEMENT { .. } => func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?,
        Deref @ MUTABLE { .. } => func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => listContains(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn arrayFirstScalar(mut arrayExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { .. } => { arrayExp = metamodelica::arrayGet(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), 1)?; continue '__tco; },
        _ => return Ok(arrayExp),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn arrayAllEqual(mut arrayExp: Arc<NFExpression>) -> bool {
    let mut allEqual: bool;
    allEqual = 'mc: {
        let __mc_input = arrayExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ ARRAY { .. } => {
                    Ok(arrayAllEqual2(arrayExp.clone(), arrayFirstScalar(arrayExp.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    allEqual
}

pub(crate) fn arrayAllEqual2(mut arrayExp: Arc<NFExpression>, mut element: Arc<NFExpression>) -> Result<bool> {
    let mut allEqual: bool;
    allEqual = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { .. } if (!(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) && isArray(metamodelica::arrayGet(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), 1)?)) => Array::all(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = element; move |__pe_a0| arrayAllEqual2(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?,
        Deref @ ARRAY { .. } => Array::all(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = element; move |__pe_a0| isEqual(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(allEqual)
}

pub fn fromCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut includeScope: bool) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    exp = Arc::new(NFExpression::CREF { ty: ComponentRef::getSubscriptedType(cref.clone(), includeScope)?, cref: cref });
    Ok(exp)
}

pub(crate) fn fromTypedCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::CREF { ty: ty.clone(), cref: cref.clone() });
    exp
}

pub fn toCref(mut exp: Arc<NFExpression>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(exp) {
        Deref @ CREF { cref: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cref = __pa0.clone();
    Ok(cref)
}

pub fn extractCrefs(mut exp: Arc<NFExpression>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = fold(exp.clone(), (std::sync::Arc::new(extractCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13))?;
    Ok(crefs)
}

pub(crate) fn extractCref(mut exp: Arc<NFExpression>, mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = crefs;
    crefs = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => {
            UnorderedSet::add(var_field!((*exp).cref, NFExpression::CREF).clone(), crefs.clone())?;
            crefs
        },
        _ => crefs,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

pub(crate) fn isResizableCref(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isIterator(mut exp: Arc<NFExpression>) -> bool {
    let mut isIterator: bool;
    isIterator = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isIterator(var_field!((*exp).cref, NFExpression::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isIterator
}

pub(crate) fn containsAnyIterator(mut exp: Arc<NFExpression>, mut context: i32) -> Result<bool> {
    let mut iter: bool;
    if InstContext::inFor(context) {
        iter = contains(exp, (std::sync::Arc::new(fnptr!(isIterator, Arc<NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?;
    } else {
        iter = false;
    }
    Ok(iter)
}

pub fn isTime(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isTime(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isSubstitute(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isSubstitute(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isZero(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() == 0),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => { exp = var_field!((*exp).exp, NFExpression::UNARY).clone(); continue '__tco; },
        Deref @ ARRAY { .. } => return Ok(Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isNonZero(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool = isPositive(exp.clone())? || isNegative(exp.clone())?;
    Ok(res)
}

pub fn isOne(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() == 1),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(1.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return Ok(isMinusOne(var_field!((*exp).exp, NFExpression::UNARY).clone())?),
        Deref @ ARRAY { .. } => return Ok(Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isOne) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isMinusOne(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() == -1),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(-1.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return Ok(self::isOne(var_field!((*exp).exp, NFExpression::UNARY).clone())?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isNaN(mut nan: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(nan.clone()) {
        Deref @ BINARY { .. } => Operator::getMathClassification(var_field!((*nan).operator, NFExpression::BINARY).clone())? == Operator::MathClassification::DIVISION.clone() && isZero(var_field!((*nan).exp1, NFExpression::BINARY).clone())? && isZero(var_field!((*nan).exp2, NFExpression::BINARY).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isPositive(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() > 0),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() > metamodelica::OrderedFloat(0.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return Ok(isNegative(var_field!((*exp).exp, NFExpression::UNARY).clone())?),
        Deref @ CREF { .. } => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("min")).clone()), (std::sync::Arc::new(isPositive) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), false)?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isNegative(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() < 0),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat(0.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return Ok(isPositive(var_field!((*exp).exp, NFExpression::UNARY).clone())?),
        Deref @ CREF { .. } => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("max")).clone()), (std::sync::Arc::new(isNegative) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), false)?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isNonPositive(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() <= 0),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() <= metamodelica::OrderedFloat(0.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return Ok(isNonNegative(var_field!((*exp).exp, NFExpression::UNARY).clone())?),
        Deref @ CREF { .. } => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("max")).clone()), (std::sync::Arc::new(isNonPositive) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), false)?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isNonNegative(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return Ok(var_field!((*exp).value, NFExpression::INTEGER).clone() >= 0),
        Deref @ REAL { .. } => return Ok(var_field!((*exp).value, NFExpression::REAL).clone() >= metamodelica::OrderedFloat(0.0_f64)),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return Ok(isNonPositive(var_field!((*exp).exp, NFExpression::UNARY).clone())?),
        Deref @ CREF { .. } => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("min")).clone()), (std::sync::Arc::new(isNonNegative) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), false)?),
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isGreaterOrEqual(mut lhs: Arc<NFExpression>, mut rhs: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ REAL { .. }, Deref @ REAL { .. }) => return Ok(var_field!((*lhs).value, NFExpression::REAL).clone() >= var_field!((*rhs).value, NFExpression::REAL).clone()),
        (Deref @ CREF { .. }, _) => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*lhs).cref, NFExpression::CREF).clone(), (literal!("min")).clone()), (std::sync::Arc::new({ let __pe_b1 = rhs; move |__pe_a0| isGreaterOrEqual(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), false)?),
        (_, Deref @ CREF { .. }) => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*rhs).cref, NFExpression::CREF).clone(), (literal!("max")).clone()), (std::sync::Arc::new({ let __pe_b0 = lhs; move |__pe_a1| isGreaterOrEqual(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), false)?),
        (Deref @ UNARY { exp: Deref @ CREF { .. }, .. }, _) => { (lhs, rhs) = (negate(rhs), var_field!((*lhs).exp, NFExpression::UNARY).clone()); continue '__tco; },
        (_, Deref @ UNARY { exp: Deref @ CREF { .. }, .. }) => { (lhs, rhs) = (var_field!((*rhs).exp, NFExpression::UNARY).clone(), negate(lhs)); continue '__tco; },
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn hasArrayType(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = Type::isArray(typeOf(exp.clone()));
    b
}

pub fn isScalar(mut exp: Arc<NFExpression>) -> bool {
    let mut scalar: bool = Type::isScalar(typeOf(exp.clone()));
    scalar
}

pub(crate) fn isScalarLiteral(mut exp: Arc<NFExpression>) -> bool {
    let mut literal: bool;
    literal = (::match_deref::match_deref! { match &(exp) {
        Deref @ INTEGER { .. } => true,
        Deref @ REAL { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUM_LITERAL { .. } => true,
        Deref @ FILENAME { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

pub fn isLiteral(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut literal: bool;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ REAL { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUM_LITERAL { .. } => true,
        Deref @ ARRAY { .. } => var_field!((*exp).literal, NFExpression::ARRAY).clone() || Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?,
        Deref @ RECORD { .. } => List::all(var_field!((*exp).elements, NFExpression::RECORD).clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?,
        Deref @ RANGE { .. } => isLiteral(var_field!((*exp).start, NFExpression::RANGE).clone())? && isLiteral(var_field!((*exp).stop, NFExpression::RANGE).clone())? && Util::applyOptionOrDefault(var_field!((*exp).step, NFExpression::RANGE).clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), true)?,
        Deref @ FILENAME { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(literal)
}

pub fn isLiteralXML(mut exp: Arc<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            return Ok(true)
        },
        Deref @ REAL { .. } => {
            return Ok(true)
        },
        Deref @ STRING { .. } => {
            return Ok(true)
        },
        Deref @ BOOLEAN { .. } => {
            return Ok(true)
        },
        Deref @ ENUM_LITERAL { .. } => {
            return Ok(true)
        },
        Deref @ ARRAY { .. } => {
            return Ok(Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isLiteralXML) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?)
        },
        Deref @ RECORD { .. } => {
            return Ok(List::all(var_field!((*exp).elements, NFExpression::RECORD).clone(), (std::sync::Arc::new(isLiteralXML) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?)
        },
        Deref @ RANGE { .. } => {
            return Ok(isLiteralXML(var_field!((*exp).start, NFExpression::RANGE).clone())? && isLiteralXML(var_field!((*exp).stop, NFExpression::RANGE).clone())? && Util::applyOptionOrDefault(var_field!((*exp).step, NFExpression::RANGE).clone(), (std::sync::Arc::new(isLiteralXML) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>), true)?)
        },
        Deref @ FILENAME { .. } => {
            return Ok(true)
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: call_exp, .. } } => {
            { exp = call_exp.clone(); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isLiteralReplace(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ STRING { .. } => true,
        Deref @ BOX { exp: Deref @ STRING { .. } } => true,
        Deref @ RECORD { .. } => isLiteral(exp)?,
        Deref @ ARRAY { .. } => isLiteral(exp)?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isKnownSizeFill(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut literal: bool;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isKnownSizeFill(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(literal)
}

pub(crate) fn isInteger(mut exp: Arc<NFExpression>) -> bool {
    let mut isInteger: bool;
    isInteger = (::match_deref::match_deref! { match &(exp) {
        Deref @ INTEGER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInteger
}

pub fn isReal(mut exp: Arc<NFExpression>) -> bool {
    let mut isReal: bool;
    isReal = (::match_deref::match_deref! { match &(exp) {
        Deref @ REAL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isReal
}

pub fn isConstNumber(mut exp: Arc<NFExpression>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return true,
        Deref @ REAL { .. } => return true,
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => { exp = var_field!((*exp).exp, NFExpression::UNARY).clone(); continue '__tco; },
        _ => return false,
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isBoolean(mut exp: Arc<NFExpression>) -> bool {
    let mut isBool: bool;
    isBool = (::match_deref::match_deref! { match &(exp) {
        Deref @ BOOLEAN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBool
}

pub(crate) fn isRecord(mut exp: Arc<NFExpression>) -> bool {
    let mut isRecord: bool;
    isRecord = (::match_deref::match_deref! { match &(exp) {
        Deref @ RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

pub(crate) fn isRecordOrRecordArray(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isRecord: bool;
    isRecord = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RECORD { .. } => true,
        Deref @ ARRAY { .. } => Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new(isRecordOrRecordArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isRecord)
}

pub(crate) fn fillType(mut ty: Arc<Type::NFType>, mut fillExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = fillExp.clone();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = Type::arrayDims(ty.clone());
    let mut arr_ty: Arc<Type::NFType> = Type::arrayElementType(ty.clone());
    let mut is_literal: bool = isLiteral(exp.clone())?;
    for mut dim in &*dims.reverse() {
        let mut dim = dim.clone();
        (exp, arr_ty) = fillArray_impl(Dimension::size(dim.clone(), false)?, exp.clone(), arr_ty.clone(), is_literal)?;
    }
    Ok(exp)
}

pub(crate) fn fillArgs(mut fillExp: Arc<NFExpression>, mut dims: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<NFExpression>> {
    let mut result: Arc<NFExpression> = fillExp.clone();
    let mut arr_ty: Arc<Type::NFType> = typeOf(result.clone());
    let mut is_literal: bool = isLiteral(fillExp.clone())?;
    let mut d_resizable: Arc<NFExpression>;
    for mut d in &*dims.reverse() {
        let mut d = d.clone();
        d_resizable = map(d.clone(), (std::sync::Arc::new(move |__pe_a0| replaceResizableParameter(__pe_a0)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
        (result, arr_ty) = fillArray_impl(toInteger(d_resizable.clone())?, result.clone(), arr_ty.clone(), is_literal)?;
    }
    Ok(result)
}

pub(crate) fn fillArray(mut n: i32, mut fillExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut result: Arc<NFExpression>;
    (result, _) = fillArray_impl(n, fillExp.clone(), typeOf(fillExp.clone()), isLiteral(fillExp)?)?;
    Ok(result)
}

pub(crate) fn fillArray_impl(mut n: i32, mut fillExp: Arc<NFExpression>, mut ty: Arc<Type::NFType>, mut isLiteral: bool) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut result: Arc<NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut arr: metamodelica::Array<Arc<NFExpression>>;
    arr = Array::generate(n, (std::sync::Arc::new({ let __pe_b0 = fillExp; move || Ok(clone(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<Arc<NFExpression>> + 'static>))?;
    resultType = Type::liftArrayLeft(ty, Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone()));
    result = makeArray(resultType.clone(), arr.clone(), isLiteral);
    Ok((result, resultType))
}

pub(crate) fn liftArray(mut dim: Arc<Dimension::NFDimension>, mut exp: Arc<NFExpression>) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut arrayType: Arc<Type::NFType> = typeOf(exp.clone());
    (exp, arrayType) = fillArray_impl(Dimension::size(dim, false)?, exp.clone(), arrayType, isLiteral(exp)?)?;
    Ok((exp, arrayType))
}

pub(crate) fn liftArrayList(mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut exp: Arc<NFExpression>) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut arrayType: Arc<Type::NFType> = typeOf(exp.clone());
    let mut is_literal: bool = isLiteral(exp.clone())?;
    for mut dim in &*dims.reverse() {
        let mut dim = dim.clone();
        (exp, arrayType) = fillArray_impl(Dimension::size(dim.clone(), false)?, exp.clone(), arrayType.clone(), is_literal)?;
    }
    Ok((exp, arrayType))
}

pub fn makeZero(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut zeroExp: Arc<NFExpression>;
    zeroExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: 0 }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: false }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeZero(Type::arrayElementType(ty))?)?,
        Deref @ Type::COMPLEX { .. } => makeOperatorRecordZero(var_field!((*ty).cls, Type::NFType::COMPLEX).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeZero")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Type::toString(ty)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(zeroExp)
}

pub(crate) fn makeOperatorRecordZero(mut recordNode: Arc<InstNode::InstNode>) -> Result<Arc<NFExpression>> {
    let mut zeroExp: Arc<NFExpression>;
    let mut op_node: Arc<InstNode::InstNode>;
    let mut r#fn: Arc<Function::Function::Function>;
    match '__try0: {
        (op_node, _) = unwrap_break_err!(Class::lookupElement((literal!("'0'")).clone(), unwrap_break_err!(InstNode::getClass(recordNode.clone()), '__try0)), '__try0);
        unwrap_break_err!(Function::Function::instFunctionNode(op_node.clone(), InstContext::NO_CONTEXT.clone(), InstNode::info(InstNode::parent(op_node.clone()))), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Function::Function::typeNodeCache(op_node.clone(), InstContext::FUNCTION.clone()), '__try0)) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        r#fn = __pa1.clone();
        zeroExp = Arc::new(NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), metamodelica::nil(), Variability::CONSTANT.clone(), Purity::PURE.clone(), r#fn.returnType.clone()) });
        zeroExp = unwrap_break_err!(Ceval::evalExp(zeroExp.clone(), Ceval::noTarget().clone()), '__try0);
        Ok::<_, anyhow::Error>((r#fn.clone(), op_node.clone(), zeroExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            r#fn = __try0_o0;
            op_node = __try0_o1;
            zeroExp = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeOperatorRecordZero")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*InstNode::toString(recordNode.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            return Err(__try0_err);
        }
    }
    Ok(zeroExp)
}

pub fn makeOne(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut oneExp: Arc<NFExpression>;
    oneExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: 1 }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeOne(Type::arrayElementType(ty))?)?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeOne")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Type::toString(ty)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oneExp)
}

pub(crate) fn makeMinusOne(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut oneExp: Arc<NFExpression>;
    oneExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(-1.0_f64) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: -1 }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeMinusOne(Type::arrayElementType(ty))?)?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeMinusOne")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Type::toString(ty)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oneExp)
}

pub(crate) fn makeNaN(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut nan: Arc<NFExpression>;
    let mut zero: Arc<NFExpression> = makeZero(ty.clone())?;
    nan = Arc::new(NFExpression::BINARY { exp1: zero.clone(), operator: Operator::makeDiv(ty), exp2: zero });
    Ok(nan)
}

pub fn makeMaxValue(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    exp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: System::realMaxLit() }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: System::intMaxLit() }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: true }),
        Deref @ Type::ENUMERATION { .. } => Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (List::last(var_field!((*ty).literals, Type::NFType::ENUMERATION).clone())?).clone(), index: (var_field!((*ty).literals, Type::NFType::ENUMERATION).clone().len() as i32) }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeMaxValue(Type::arrayElementType(ty))?)?,
        _ => Arc::new(NFExpression::REAL { value: System::realMaxLit() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn makeMinValue(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    exp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: -(System::realMaxLit()) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: -(System::intMaxLit()) }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: false }),
        Deref @ Type::ENUMERATION { .. } => Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (listHead(var_field!((*ty).literals, Type::NFType::ENUMERATION).clone())?).clone(), index: 1 }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeMinValue(Type::arrayElementType(ty))?)?,
        _ => Arc::new(NFExpression::REAL { value: -(System::realMaxLit()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn makeDefaultValue(mut ty: Arc<Type::NFType>, mut min: Option<Arc<NFExpression>>, mut max: Option<Arc<NFExpression>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => {
            if isSome(min.clone()) && isNonNegative(Util::getOption(min.clone())?)? {
                let __pa0 = ::match_deref::match_deref! { match &(min) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
            } else if isSome(max.clone()) && isNonPositive(Util::getOption(max.clone())?)? {
                let __pa1 = ::match_deref::match_deref! { match &(max) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa1.clone();
            } else {
                exp = Arc::new(NFExpression::INTEGER { value: 0 });
            }
            exp
        },
        Deref @ Type::REAL => {
            if isSome(min.clone()) && isNonNegative(Util::getOption(min.clone())?)? {
                let __pa0 = ::match_deref::match_deref! { match &(min) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
            } else if isSome(max.clone()) && isNonPositive(Util::getOption(max.clone())?)? {
                let __pa1 = ::match_deref::match_deref! { match &(max) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa1.clone();
            } else {
                exp = Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) });
            }
            exp
        },
        Deref @ Type::STRING => Arc::new(NFExpression::STRING { value: (literal!("")).clone() }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: false }),
        Deref @ Type::ENUMERATION { .. } => {
            if isSome(min.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(min) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
            } else {
                exp = Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (listHead(var_field!((*ty).literals, Type::NFType::ENUMERATION).clone())?).clone(), index: 1 });
            }
            exp
        },
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeDefaultValue(Type::arrayElementType(ty), None, None)?)?,
        Deref @ Type::TUPLE { .. } => Arc::new(NFExpression::TUPLE { ty: ty.clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).types, Type::NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = makeDefaultValue(t.clone(), None, None)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub(crate) fn r#box(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut boxedExp: Arc<NFExpression>;
    boxedExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ STRING { .. } => exp,
        Deref @ RECORD { .. } => Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: Type::r#box(var_field!((*exp).ty, NFExpression::RECORD).clone()), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = r#box(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }),
        Deref @ BOX { .. } => exp,
        Deref @ FILENAME { .. } => exp,
        _ => Arc::new(NFExpression::BOX { exp: exp }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    boxedExp
}

pub(crate) fn unbox(mut boxedExp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression>;
    exp = (::match_deref::match_deref! { match &(boxedExp.clone()) {
        Deref @ BOX { .. } => {
            var_field!((*boxedExp).exp, NFExpression::BOX).clone()
        },
        _ => {
            let mut ty: Arc<Type::NFType>;
            ty = typeOf(boxedExp.clone());
            if (Type::isBoxed(ty.clone())) {Arc::new(NFExpression::UNBOX { exp: boxedExp, ty: Type::unbox(ty) })} else {boxedExp}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub(crate) fn isNegated(mut exp: Arc<NFExpression>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => return var_field!((*exp).value, NFExpression::INTEGER).clone() < 0,
        Deref @ REAL { .. } => return var_field!((*exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64),
        Deref @ CAST { .. } => { exp = var_field!((*exp).exp, NFExpression::CAST).clone(); continue '__tco; },
        Deref @ UNARY { .. } => return true,
        _ => return false,
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn negate(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Arc::new(NFExpression::INTEGER { value: -(var_field!((*exp).value, NFExpression::INTEGER).clone()) }),
        Deref @ REAL { .. } => Arc::new(NFExpression::REAL { value: -(var_field!((*exp).value, NFExpression::REAL).clone()) }),
        Deref @ CAST { .. } => Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: negate(var_field!((*exp).exp, NFExpression::CAST).clone()) }),
        Deref @ UNARY { .. } => var_field!((*exp).exp, NFExpression::UNARY).clone(),
        _ => Arc::new(NFExpression::UNARY { operator: Operator::makeUMinus(typeOf(exp.clone())), exp: exp }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn logicNegate(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { .. } => Arc::new(NFExpression::BOOLEAN { value: !(var_field!((*exp).value, NFExpression::BOOLEAN).clone()) }),
        Deref @ LUNARY { .. } => var_field!((*exp).exp, NFExpression::LUNARY).clone(),
        _ => Arc::new(NFExpression::LUNARY { operator: Operator::makeNot(typeOf(exp.clone())), exp: exp }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn revertRange(mut range: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut range: Arc<NFExpression> = range;
    range = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ RANGE { step: Some(step), .. } => {
            Arc::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: var_field!((*range).stop, NFExpression::RANGE).clone(), step: Some(negate(step.clone())), stop: var_field!((*range).start, NFExpression::RANGE).clone() })
        },
        Deref @ RANGE { .. } => {
            Arc::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: var_field!((*range).stop, NFExpression::RANGE).clone(), step: Some(Arc::new(NFExpression::INTEGER { value: -1 })), stop: var_field!((*range).start, NFExpression::RANGE).clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.revertRange")); __mm_s.push_str(&*literal!(" failed because expression is not a range:\n")); __mm_s.push_str(&*toString(range)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(range)
}

pub fn sliceRange(mut range: Arc<NFExpression>, mut slice: (i32, i32, i32)) -> Result<Arc<NFExpression>> {
    let mut range: Arc<NFExpression> = range;
    range = (::match_deref::match_deref! { match &((range.clone(), slice)) {
        (Deref @ RANGE { .. }, (slice_start, slice_step, slice_stop)) => {
            let mut start: i32;
            let mut step: i32;
            let mut stop: i32;
            step = Util::applyOptionOrDefault(var_field!((*range).step, NFExpression::RANGE).clone(), (std::sync::Arc::new(integerValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<i32> + 'static>), 1)?;
            start = integerValue(var_field!((*range).start, NFExpression::RANGE).clone())?;
            stop = start + slice_stop.clone() * step;
            start = start + slice_start.clone() * step;
            step = slice_step.clone() * step;
            range = Arc::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: Arc::new(NFExpression::INTEGER { value: start }), step: Some(Arc::new(NFExpression::INTEGER { value: step })), stop: Arc::new(NFExpression::INTEGER { value: stop }) });
            retype(range)?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.sliceRange")); __mm_s.push_str(&*literal!(" failed because expression is not a range:\n")); __mm_s.push_str(&*toString(range)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(range)
}

pub(crate) fn arrayElements(mut array: Arc<NFExpression>) -> Result<metamodelica::Array<Arc<NFExpression>>> {
    let mut elements: metamodelica::Array<Arc<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &(array) {
        Deref @ ARRAY { elements: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    elements = __pa0.clone();
    Ok(elements)
}

pub(crate) fn arrayElementList(mut array: Arc<NFExpression>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>>;
    elements = (::match_deref::match_deref! { match &(array.clone()) {
        Deref @ ARRAY { .. } => Arc::new(var_field!((*array).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(elements)
}

pub(crate) fn arrayScalarElements(mut exp: Arc<NFExpression>) -> Arc<metamodelica::List<Arc<NFExpression>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>>;
    elements = metamodelica::Dangerous::listReverseInPlace(arrayScalarElements_impl(exp, metamodelica::nil()));
    elements
}

pub(crate) fn arrayScalarElements_impl(mut exp: Arc<NFExpression>, mut elements: Arc<metamodelica::List<Arc<NFExpression>>>) -> Arc<metamodelica::List<Arc<NFExpression>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>> = elements;
    elements = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            let __range0 = var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                elements = arrayScalarElements_impl(e.clone(), elements.clone());
            }
            elements
        },
        _ => metamodelica::cons(exp, elements),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elements
}

pub(crate) fn arrayScalarElement(mut arrayExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut scalarExp: Arc<NFExpression>;
    scalarExp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { .. } if (metamodelica::arrayLength(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone()) == 1) => metamodelica::arrayGet(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), 1)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(scalarExp)
}

pub(crate) fn hasArrayCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut hasArrayCall: bool;
    hasArrayCall = contains(exp, (std::sync::Arc::new(hasArrayCall2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?;
    Ok(hasArrayCall)
}

pub(crate) fn hasArrayCall2(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut hasArrayCall: bool;
    let mut call: Arc<Call::NFCall> = Arc::new(<Call::NFCall as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    hasArrayCall = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { call: __esc_call } => {
            call = (*__esc_call).clone();
            ty = Call::typeOf(call.clone());
            Type::isArray(ty) && Call::isVectorizeable(call.clone())
        },
        Deref @ TUPLE_ELEMENT { tupleExp: Deref @ CALL { call: __esc_call }, .. } => {
            call = (*__esc_call).clone();
            ty = Type::nthTupleType(Call::typeOf(call.clone()), var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone())?;
            Type::isArray(ty) && Call::isVectorizeable(call.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasArrayCall)
}

pub(crate) fn transposeArray(mut arrayExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut literal: bool = false;
    let mut arr: metamodelica::Array<Arc<NFExpression>> = Default::default();
    let mut matrix_arr: metamodelica::Array<metamodelica::Array<Arc<NFExpression>>> = Default::default();
    outExp = (::match_deref::match_deref! { match &(arrayExp) {
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_ty, dimensions: Deref @ metamodelica::List::Cons { head: __esc_dim1, tail: Deref @ metamodelica::List::Cons { head: __esc_dim2, tail: __esc_rest_dims } } }, elements: __esc_arr, literal: __esc_literal } => {
            ty = (*__esc_ty).clone();
            dim1 = (*__esc_dim1).clone();
            dim2 = (*__esc_dim2).clone();
            rest_dims = (*__esc_rest_dims).clone();
            arr = (*__esc_arr).clone();
            literal = (*__esc_literal).clone();
            if !(arr.clone().borrow().is_empty()) {
                row_ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: metamodelica::cons(dim1.clone(), rest_dims.clone()) });
                matrix_arr = Array::map(arr.clone(), (std::sync::Arc::new(arrayElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<metamodelica::Array<Arc<NFExpression>>> + 'static>))?;
                matrix_arr = Array::transpose(matrix_arr.clone());
                arr = Array::map(matrix_arr.clone(), (std::sync::Arc::new({ let __pe_b0 = row_ty; let __pe_b2 = literal.clone(); move |__pe_a1| Ok(makeArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<NFExpression>>) -> Result<Arc<NFExpression>> + 'static>))?;
            }
            makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) }), arr.clone(), literal.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn makeIdentityMatrix(mut n: i32, mut elementType: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut matrix: Arc<NFExpression>;
    let mut row: metamodelica::Array<Arc<NFExpression>>;
    let mut rows: metamodelica::Array<Arc<NFExpression>>;
    let mut zero: Arc<NFExpression>;
    let mut one: Arc<NFExpression>;
    let mut row_ty: Arc<Type::NFType>;
    zero = makeZero(elementType.clone())?;
    one = makeOne(elementType.clone())?;
    rows = metamodelica::arrayCreate(n, zero.clone());
    row_ty = Arc::new(Type::NFType::ARRAY { elementType: elementType, dimensions: list![Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())] });
    for mut i in 1..=n {
        row = metamodelica::arrayCreate(n, zero.clone());
        for mut j in 1..=n {
            unsafe { metamodelica::Dangerous::arrayInitSlot(row.clone(), j.clone(), if (i.clone() == j.clone()) {one.clone()} else {zero.clone()}) };
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(rows.clone(), i.clone(), makeArray(row_ty.clone(), row.clone(), true)) };
    }
    matrix = makeExpArray(rows.clone(), row_ty, true);
    Ok(matrix)
}

pub fn makeTriuMask(mut n: i32, mut elTy: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut mask: Arc<NFExpression>;
    let mut row: metamodelica::Array<Arc<NFExpression>>;
    let mut rows: metamodelica::Array<Arc<NFExpression>>;
    let mut zero: Arc<NFExpression>;
    let mut one: Arc<NFExpression>;
    let mut row_ty: Arc<Type::NFType>;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    zero = makeZero(elTy.clone())?;
    one = makeOne(elTy.clone())?;
    rows = metamodelica::arrayCreate(n, zero.clone());
    row_ty = Arc::new(Type::NFType::ARRAY { elementType: elTy, dimensions: list![Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())] });
    for mut i in 1..=n {
        row = metamodelica::arrayCreate(n, zero.clone());
        for mut j in 1..=n {
            unsafe { metamodelica::Dangerous::arrayInitSlot(row.clone(), j, if (i <= j) {one.clone()} else {zero.clone()}) };
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(rows.clone(), i, makeArray(row_ty.clone(), row.clone(), true)) };
    }
    mask = makeExpArray(rows.clone(), row_ty, true);
    Ok(mask)
}

pub(crate) fn promote(mut e: Arc<NFExpression>, mut ty: Arc<Type::NFType>, mut n: i32) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut e: Arc<NFExpression> = e;
    let mut ty: Arc<Type::NFType> = ty;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut ety: Arc<Type::NFType>;
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut is_array: bool;
    dims = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut i in (Type::dimensionCount(ty.clone())..=n - 1).into_iter() {
            let __x = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !(dims.clone().is_empty()) {
        dims = listAppend(Type::arrayDims(ty.clone()), dims);
        is_array = Type::isArray(ty.clone());
        ety = Type::arrayElementType(ty);
        ty = Type::liftArrayLeftList(ety.clone(), dims.clone());
        while !(dims.clone().is_empty()) {
            tys = metamodelica::cons(Type::liftArrayLeftList(ety.clone(), dims.clone()), tys.clone());
            dims = listRest(dims.clone())?;
        }
        e = promote2(e, is_array, n, tys.reverse())?;
    }
    Ok((e, ty))
}

pub(crate) fn promote2(mut exp: Arc<NFExpression>, mut isArray: bool, mut dims: i32, mut types: Arc<metamodelica::List<Arc<Type::NFType>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &((exp.clone(), types.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            exp
        },
        (Deref @ ARRAY { .. }, Deref @ metamodelica::List::Cons { head: ty, tail: rest_ty }) => {
            makeArray(ty.clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = false; let __pe_b2 = dims; let __pe_b3 = rest_ty.clone(); move |__pe_a0| promote2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?, false)
        },
        (_, _) if (isArray) => {
            let mut expanded: bool;
            if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? && !(isLiteral(exp.clone())?) {
                expanded = false;
            } else {
                (outExp, expanded) = ExpandExp::expand(exp.clone(), false, false)?;
            }
            if expanded {
                outExp = promote2(outExp, true, dims, types)?;
            } else {
                outExp = Arc::new(NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::PROMOTE().clone(), list![exp.clone(), Arc::new(NFExpression::INTEGER { value: dims })], variability(exp.clone())?, purity(exp)?, listHead(types)?) });
            }
            outExp
        },
        _ => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            outExp = exp;
            for mut ty in &*types.reverse() {
                let mut ty = ty.clone();
                outExp = makeArray(ty.clone(), arrayCreate(1, outExp.clone()), false);
            }
            outExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn variability(mut exp: Arc<NFExpression>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Variability::CONSTANT.clone(),
        Deref @ REAL { .. } => Variability::CONSTANT.clone(),
        Deref @ STRING { .. } => Variability::CONSTANT.clone(),
        Deref @ BOOLEAN { .. } => Variability::CONSTANT.clone(),
        Deref @ ENUM_LITERAL { .. } => Variability::CONSTANT.clone(),
        Deref @ CLKCONST { .. } => Variability::DISCRETE.clone(),
        Deref @ CREF { .. } => ComponentRef::variability(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        Deref @ TYPENAME { .. } => Variability::CONSTANT.clone(),
        Deref @ ARRAY { .. } => variabilityArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Prefixes::Variability::CONSTANT.clone())?,
        Deref @ MATRIX { .. } => List::fold(var_field!((*exp).elements, NFExpression::MATRIX).clone(), (std::sync::Arc::new(variabilityList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<NFExpression>>>, Variability) -> Result<Variability> + 'static>), Variability::CONSTANT.clone())?,
        Deref @ RANGE { .. } => {
            var = variability(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            var = Prefixes::variabilityMax(var, variability(var_field!((*exp).stop, NFExpression::RANGE).clone())?);
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                var = Prefixes::variabilityMax(var, variability(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?)?);
            }
            var
        },
        Deref @ TUPLE { .. } => variabilityList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), Prefixes::Variability::CONSTANT.clone())?,
        Deref @ RECORD { .. } => variabilityList(var_field!((*exp).elements, NFExpression::RECORD).clone(), Prefixes::Variability::CONSTANT.clone())?,
        Deref @ CALL { .. } => Call::variability(var_field!((*exp).call, NFExpression::CALL).clone())?,
        Deref @ SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()) {
                var = Prefixes::variabilityMax(Variability::PARAMETER.clone(), variability(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?);
            } else {
                var = Variability::PARAMETER.clone();
            }
            var
        },
        Deref @ END { .. } => Variability::PARAMETER.clone(),
        Deref @ MULTARY { .. } => Prefixes::variabilityMax(variabilityList(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), Prefixes::Variability::CONSTANT.clone())?, variabilityList(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), Prefixes::Variability::CONSTANT.clone())?),
        Deref @ BINARY { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, variability(var_field!((*exp).exp2, NFExpression::BINARY).clone())?),
        Deref @ UNARY { .. } => variability(var_field!((*exp).exp, NFExpression::UNARY).clone())?,
        Deref @ LBINARY { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, variability(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?),
        Deref @ LUNARY { .. } => variability(var_field!((*exp).exp, NFExpression::LUNARY).clone())?,
        Deref @ RELATION { .. } => Prefixes::variabilityMin(Prefixes::variabilityMax(variability(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, variability(var_field!((*exp).exp2, NFExpression::RELATION).clone())?), Variability::DISCRETE.clone()),
        Deref @ IF { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).condition, NFExpression::IF).clone())?, Prefixes::variabilityMax(variability(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, variability(var_field!((*exp).falseBranch, NFExpression::IF).clone())?)),
        Deref @ CAST { .. } => variability(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => variability(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => variability(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, Subscript::variabilityList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?),
        Deref @ TUPLE_ELEMENT { .. } => variability(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?,
        Deref @ RECORD_ELEMENT { .. } => variability(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?,
        Deref @ MUTABLE { .. } => variability(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => variability(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ EMPTY { .. } => Variability::CONSTANT.clone(),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => Variability::CONTINUOUS.clone(),
        Deref @ FILENAME { .. } => Variability::CONSTANT.clone(),
        Deref @ INSTANCE_NAME { .. } => Variability::CONSTANT.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.variability")); __mm_s.push_str(&*literal!(" got unknown expression.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn variabilityArray(mut expl: metamodelica::Array<Arc<NFExpression>>, mut var: Variability) -> Result<Variability> {
    let mut var: Variability = var;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        var = Prefixes::variabilityMax(var, variability(e.clone())?);
    }
    Ok(var)
}

pub(crate) fn variabilityList(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut var: Variability) -> Result<Variability> {
    let mut var: Variability = var;
    for mut e in &*expl {
        let mut e = e.clone();
        var = Prefixes::variabilityMax(var, variability(e.clone())?);
    }
    Ok(var)
}

pub fn purity(mut exp: Arc<NFExpression>) -> Result<Purity> {
    let mut pur: Purity = Purity::PURE;
    pur = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Purity::PURE.clone(),
        Deref @ REAL { .. } => Purity::PURE.clone(),
        Deref @ STRING { .. } => Purity::PURE.clone(),
        Deref @ BOOLEAN { .. } => Purity::PURE.clone(),
        Deref @ ENUM_LITERAL { .. } => Purity::PURE.clone(),
        Deref @ CLKCONST { .. } => Purity::PURE.clone(),
        Deref @ CREF { .. } => ComponentRef::purity(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        Deref @ TYPENAME { .. } => Purity::PURE.clone(),
        Deref @ ARRAY { .. } => purityArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Prefixes::Purity::PURE.clone())?,
        Deref @ MATRIX { .. } => List::fold(var_field!((*exp).elements, NFExpression::MATRIX).clone(), (std::sync::Arc::new(purityList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<NFExpression>>>, Purity) -> Result<Purity> + 'static>), Purity::PURE.clone())?,
        Deref @ RANGE { .. } => {
            pur = purity(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            pur = Prefixes::purityMin(pur, purity(var_field!((*exp).stop, NFExpression::RANGE).clone())?);
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                pur = Prefixes::purityMin(pur, purity(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?)?);
            }
            pur
        },
        Deref @ TUPLE { .. } => purityList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), Prefixes::Purity::PURE.clone())?,
        Deref @ RECORD { .. } => purityList(var_field!((*exp).elements, NFExpression::RECORD).clone(), Prefixes::Purity::PURE.clone())?,
        Deref @ CALL { .. } => Call::purity(var_field!((*exp).call, NFExpression::CALL).clone()),
        Deref @ SIZE { .. } => if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {purity(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?} else {Purity::PURE.clone()},
        Deref @ END { .. } => Purity::PURE.clone(),
        Deref @ BINARY { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, purity(var_field!((*exp).exp2, NFExpression::BINARY).clone())?),
        Deref @ UNARY { .. } => purity(var_field!((*exp).exp, NFExpression::UNARY).clone())?,
        Deref @ LBINARY { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, purity(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?),
        Deref @ LUNARY { .. } => purity(var_field!((*exp).exp, NFExpression::LUNARY).clone())?,
        Deref @ RELATION { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, purity(var_field!((*exp).exp2, NFExpression::RELATION).clone())?),
        Deref @ MULTARY { .. } => Prefixes::purityMin(purityList(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), Prefixes::Purity::PURE.clone())?, purityList(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), Prefixes::Purity::PURE.clone())?),
        Deref @ IF { .. } => Prefixes::purityMin(purity(var_field!((*exp).condition, NFExpression::IF).clone())?, Prefixes::purityMin(purity(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, purity(var_field!((*exp).falseBranch, NFExpression::IF).clone())?)),
        Deref @ CAST { .. } => purity(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => purity(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => purity(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, Subscript::purityList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?),
        Deref @ TUPLE_ELEMENT { .. } => purity(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?,
        Deref @ RECORD_ELEMENT { .. } => purity(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?,
        Deref @ MUTABLE { .. } => purity(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => purity(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ EMPTY { .. } => Purity::PURE.clone(),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => Purity::PURE.clone(),
        Deref @ FILENAME { .. } => Purity::PURE.clone(),
        Deref @ INSTANCE_NAME { .. } => Purity::PURE.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.purity")); __mm_s.push_str(&*literal!(" got unknown expression.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(pur)
}

pub(crate) fn purityArray(mut expl: metamodelica::Array<Arc<NFExpression>>, mut pur: Purity) -> Result<Purity> {
    let mut pur: Purity = pur;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        pur = Prefixes::purityMin(pur, purity(e.clone())?);
    }
    Ok(pur)
}

pub(crate) fn purityList(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut pur: Purity) -> Result<Purity> {
    let mut pur: Purity = pur;
    for mut e in &*expl {
        let mut e = e.clone();
        pur = Prefixes::purityMin(pur, purity(e.clone())?);
    }
    Ok(pur)
}

pub(crate) fn makeMutable(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression>;
    outExp = Arc::new(NFExpression::MUTABLE { exp: Mutable::create(exp) });
    outExp
}

pub(crate) fn makeImmutable(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MUTABLE { .. } => Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()),
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub(crate) fn isMutable(mut exp: Arc<NFExpression>) -> bool {
    let mut isMutable: bool;
    isMutable = (::match_deref::match_deref! { match &(exp) {
        Deref @ MUTABLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isMutable
}

pub(crate) fn updateMutable(mut mutableExp: Arc<NFExpression>, mut value: Arc<NFExpression>) -> Result<()> {
    let mut exp_ptr: Mutable::Mutable<Arc<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &(mutableExp) {
        Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp_ptr = __pa0.clone();
    Mutable::update(exp_ptr, value);
    Ok(())
}

pub(crate) fn applyMutable(mut mutableExp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<()> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut exp_ptr: Mutable::Mutable<Arc<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &(mutableExp) {
        Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp_ptr = __pa0.clone();
    Mutable::update(exp_ptr.clone(), func(Mutable::access(exp_ptr))?);
    Ok(())
}

pub fn isEmpty(mut exp: Arc<NFExpression>) -> bool {
    let mut empty: bool;
    empty = (::match_deref::match_deref! { match &(exp) {
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    empty
}

pub fn isEnd(mut exp: Arc<NFExpression>) -> bool {
    let mut isend: bool;
    isend = (::match_deref::match_deref! { match &(exp) {
        Deref @ END { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isend
}

pub(crate) fn enumIndexExp(mut enumExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut indexExp: Arc<NFExpression>;
    indexExp = (::match_deref::match_deref! { match &(enumExp.clone()) {
        Deref @ ENUM_LITERAL { .. } => Arc::new(NFExpression::INTEGER { value: var_field!((*enumExp).index, NFExpression::ENUM_LITERAL).clone() }),
        _ => Arc::new(NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INTEGER_ENUM().clone(), list![enumExp.clone()], variability(enumExp)?, Purity::PURE.clone(), NFBuiltinFuncs::INTEGER_ENUM().returnType.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(indexExp)
}

pub(crate) fn toScalar(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } if (metamodelica::arrayLength(var_field!((*exp).elements, NFExpression::ARRAY).clone()) == 1) => { exp = metamodelica::arrayGet(var_field!((*exp).elements, NFExpression::ARRAY).clone(), 1)?; continue '__tco; },
        _ => return Ok(exp.clone()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn tupleElement(mut exp: Arc<NFExpression>, mut ty: Arc<Type::NFType>, mut index: i32) -> Result<Arc<NFExpression>> {
    let mut tupleElem: Arc<NFExpression>;
    tupleElem = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ TUPLE { .. } => {
            (var_field!((*exp).elements, NFExpression::TUPLE).clone()).get(index)?
        },
        Deref @ ARRAY { .. } => {
            let mut ety: Arc<Type::NFType>;
            ety = Type::unliftArray(ty)?;
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = ety; let __pe_b2 = index; move |__pe_a0| tupleElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?);
            exp
        },
        Deref @ SUBSCRIPTED_EXP { split: true, .. } => {
            mapSplitExpressions(exp, (std::sync::Arc::new({ let __pe_b1 = ty; let __pe_b2 = index; move |__pe_a0| tupleElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?
        },
        _ => {
            Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: exp, index: index, ty: ty })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tupleElem)
}

pub(crate) fn recordElement(mut elementName: ArcStr, mut recordExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(recordExp.clone()) {
        Deref @ RECORD { ty: Deref @ Type::COMPLEX { cls: node, .. }, .. } => {
            let mut cls: Arc<Class::NFClass>;
            let mut index: i32;
            cls = InstNode::getClass(node.clone())?;
            index = Class::lookupComponentIndex((elementName).clone(), cls)?;
            (var_field!((*recordExp).elements, NFExpression::RECORD).clone()).get(index)?
        },
        Deref @ CREF { .. } => {
            let mut node: Arc<InstNode::InstNode>;
            let mut cls_tree: Arc<ClassTree::ClassTree>;
            let mut ty: Arc<Type::NFType>;
            let mut cref: Arc<ComponentRef::NFComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(Type::arrayElementType(var_field!((*recordExp).ty, NFExpression::CREF).clone())) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            cls_tree = Class::classTree(InstNode::getClass(node)?)?;
            let __pa1 = ::match_deref::match_deref! { match &(ClassTree::lookupElement((elementName).clone(), cls_tree)?) {
                (__pa1, false) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa1.clone();
            ty = InstNode::getType(node.clone())?;
            cref = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), var_field!((*recordExp).cref, NFExpression::CREF).clone());
            ty = Type::liftArrayLeftList(ty, Type::arrayDims(var_field!((*recordExp).ty, NFExpression::CREF).clone()));
            Arc::new(NFExpression::CREF { ty: ty, cref: cref })
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } if (var_field!((*recordExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) => {
            let mut cls: Arc<Class::NFClass>;
            let mut ty: Arc<Type::NFType>;
            let mut index: i32;
            cls = InstNode::getClass(node.clone())?;
            index = Class::lookupComponentIndex((elementName).clone(), cls.clone())?;
            ty = InstNode::getType(Class::nthComponent(index, cls)?)?;
            ty = Type::liftArrayLeftList(ty, Type::arrayDims(var_field!((*recordExp).ty, NFExpression::ARRAY).clone()));
            makeEmptyArray(ty)
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } => {
            let mut ty: Arc<Type::NFType>;
            let mut index: i32;
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            index = Class::lookupComponentIndex((elementName).clone(), InstNode::getClass(node.clone())?)?;
            arr = Array::map(var_field!((*recordExp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = index; move |__pe_a1| nthRecordElement(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
            ty = Type::liftArrayLeft(typeOf(metamodelica::arrayGet(arr.clone(), 1)?), Dimension::fromInteger(metamodelica::arrayLength(arr.clone()), Prefixes::Variability::CONSTANT.clone()));
            makeArray(ty, arr.clone(), var_field!((*recordExp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut ty: Arc<Type::NFType>;
            outExp = recordElement((elementName).clone(), var_field!((*recordExp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?;
            ty = Type::subscript(typeOf(outExp.clone()), var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), true)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: outExp, subscripts: var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), ty: ty, split: var_field!((*recordExp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ EMPTY { .. } => {
            bail!("fail")
        },
        _ => {
            let mut node: Arc<InstNode::InstNode>;
            let mut cls: Arc<Class::NFClass>;
            let mut ty: Arc<Type::NFType>;
            let mut index: i32;
            ty = typeOf(recordExp.clone());
            let __pa0 = ::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            cls = InstNode::getClass(node.clone())?;
            index = Class::lookupComponentIndex((elementName.clone()).clone(), cls.clone())?;
            ty = Type::liftArrayLeftList(InstNode::getType(Class::nthComponent(index, cls)?)?, Type::arrayDims(ty));
            Arc::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index, fieldName: (elementName).clone(), ty: ty })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn nthRecordElement(mut index: i32, mut recordExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(recordExp.clone()) {
        Deref @ RECORD { .. } => {
            (var_field!((*recordExp).elements, NFExpression::RECORD).clone()).get(index)?
        },
        Deref @ CREF { .. } => {
            let mut node: Arc<InstNode::InstNode>;
            let __pa0 = ::match_deref::match_deref! { match &(Type::arrayElementType(typeOf(recordExp.clone()))) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            node = Class::nthComponent(index, InstNode::getClass(node)?)?;
            fromCref(ComponentRef::prefixCref(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), var_field!((*recordExp).cref, NFExpression::CREF).clone()), false)?
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } if (var_field!((*recordExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) => {
            makeEmptyArray(InstNode::getType(Class::nthComponent(index, InstNode::getClass(node.clone())?)?)?)
        },
        Deref @ ARRAY { .. } => {
            let mut ty: Arc<Type::NFType>;
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            arr = Array::map(var_field!((*recordExp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = index; move |__pe_a1| nthRecordElement(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
            ty = Type::liftArrayLeft(typeOf(metamodelica::arrayGet(arr.clone(), 1)?), listHead(Type::arrayDims(var_field!((*recordExp).ty, NFExpression::ARRAY).clone()))?);
            makeArray(ty, arr.clone(), false)
        },
        Deref @ RECORD_ELEMENT { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } => {
            let mut node = (*node).clone();
            node = Class::nthComponent(index, InstNode::getClass(node.clone())?)?;
            Arc::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index, fieldName: (InstNode::name(node.clone())?).clone(), ty: Type::liftArrayLeftList(InstNode::getType(node.clone())?, Type::arrayDims(var_field!((*recordExp).ty, NFExpression::RECORD_ELEMENT).clone())) })
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut ty: Arc<Type::NFType>;
            outExp = nthRecordElement(index, var_field!((*recordExp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?;
            ty = Type::subscript(typeOf(outExp.clone()), var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), true)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: outExp, subscripts: var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), ty: ty, split: var_field!((*recordExp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ IF { .. } => {
            let mut trueBranch: Arc<NFExpression>;
            let mut falseBranch: Arc<NFExpression>;
            trueBranch = nthRecordElement(index, var_field!((*recordExp).trueBranch, NFExpression::IF).clone())?;
            falseBranch = nthRecordElement(index, var_field!((*recordExp).falseBranch, NFExpression::IF).clone())?;
            Arc::new(NFExpression::IF { ty: typeOf(trueBranch.clone()), condition: var_field!((*recordExp).condition, NFExpression::IF).clone(), trueBranch: trueBranch, falseBranch: falseBranch })
        },
        _ => {
            let mut node: Arc<InstNode::InstNode>;
            let __pa0 = ::match_deref::match_deref! { match &(typeOf(recordExp.clone())) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            node = Class::nthComponent(index, InstNode::getClass(node)?)?;
            Arc::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index, fieldName: (InstNode::name(node.clone())?).clone(), ty: InstNode::getType(node.clone())? })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn getRecordElements(mut exp: Arc<NFExpression>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Type::arrayElementType(typeOf(exp.clone()));
    elements = (::match_deref::match_deref! { match &(ty) {
        Deref @ Type::COMPLEX { complexTy: complexTy @ Deref @ ComplexType::RECORD { .. }, .. } => {
            for mut i in ({let __s=metamodelica::arrayLength(var_field!((**complexTy).fields, ComplexType::NFComplexType::RECORD).clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                elements = metamodelica::cons(recordElement((Record::Field::name(({let __elt = var_field!((**complexTy).fields, ComplexType::NFComplexType::RECORD).borrow()[(i.clone()-1) as usize].clone(); __elt}))?).clone(), exp.clone())?, elements.clone());
            }
            elements
        },
        _ => {
            elements
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

pub(crate) fn retype(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = TypeCheck::getRangeType(var_field!((*exp).start, NFExpression::RANGE).clone(), var_field!((*exp).step, NFExpression::RANGE).clone(), var_field!((*exp).stop, NFExpression::RANGE).clone(), typeOf(var_field!((*exp).start, NFExpression::RANGE).clone()), Absyn::dummyInfo.clone())?);
            ()
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::retype(var_field!((*exp).call, NFExpression::CALL).clone()));
            ()
        },
        _ => {
            let mut ty: Arc<Type::NFType>;
            ty = typeOf(exp.clone());
            if Type::isConditionalArray(ty.clone()) {
                ty = Type::simplifyConditionalArray(ty);
                exp = setType(ty, exp)?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn nthEnumLiteral(mut ty: Arc<Type::NFType>, mut n: i32) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression>;
    exp = Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (Type::nthEnumLiteral(ty, n)?).clone(), index: n });
    Ok(exp)
}

pub(crate) fn createIterationRanges(mut exp: Arc<NFExpression>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<NFExpression>)>>) -> Result<(Arc<NFExpression>, Arc<metamodelica::List<Arc<NFExpression>>>, Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut ranges: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut iters: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode>;
    let mut range: Arc<NFExpression>;
    let mut iter: Mutable::Mutable<Arc<NFExpression>>;
    for mut i in &*iterators {
        let mut i = i.clone();
        (node, range) = i.clone();
        iter = Mutable::create(Arc::new(NFExpression::INTEGER { value: 0 }));
        ranges = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut r in (ranges.clone()).into_iter().cloned() {
            let __x = replaceIterator(r.clone(), node.clone(), Arc::new(NFExpression::MUTABLE { exp: iter.clone() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        exp = replaceIterator(exp.clone(), node.clone(), Arc::new(NFExpression::MUTABLE { exp: iter.clone() }))?;
        iters = metamodelica::cons(iter.clone(), iters.clone());
        ranges = metamodelica::cons(range.clone(), ranges.clone());
    }
    Ok((exp, ranges, iters))
}

pub(crate) fn foldReduction(mut exp: Arc<NFExpression>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<NFExpression>)>>, mut foldExp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    pub type FoldFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut result: Arc<NFExpression>;
    let mut e: Arc<NFExpression>;
    let mut ranges: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut iters: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>> = metamodelica::nil();
    (e, ranges, iters) = createIterationRanges(exp, iterators)?;
    result = foldReduction2(e, ranges, iters, foldExp, mapFn.clone(), foldFn.clone())?;
    Ok(result)
}

pub(crate) fn foldReduction2(mut exp: Arc<NFExpression>, mut ranges: Arc<metamodelica::List<Arc<NFExpression>>>, mut iterators: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>>, mut foldExp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    pub type FoldFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut result: Arc<NFExpression>;
    let mut range: Arc<NFExpression>;
    let mut value: Arc<NFExpression>;
    let mut ranges_rest: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut iter: Mutable::Mutable<Arc<NFExpression>>;
    let mut iters_rest: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>>;
    let mut range_iter: Arc<ExpressionIterator::NFExpressionIterator>;
    if ranges.clone().is_empty() {
        result = foldFn(foldExp, mapFn(exp)?)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ranges) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        ranges_rest = __pa1.clone();
        range = Ceval::evalExp(range, Ceval::noTarget().clone())?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(iterators) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter = __pa2.clone();
        iters_rest = __pa3.clone();
        range_iter = ExpressionIterator::fromExp(range, false, false)?;
        result = foldExp;
        while ExpressionIterator::hasNext(range_iter.clone())? {
            (range_iter, value) = ExpressionIterator::next(range_iter.clone())?;
            Mutable::update(iter.clone(), value.clone());
            result = foldReduction2(exp.clone(), ranges_rest.clone(), iters_rest.clone(), result.clone(), mapFn.clone(), foldFn.clone())?;
        }
    }
    Ok(result)
}

pub(crate) fn isPure(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isPure: bool;
    isPure = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => !(ComponentRef::isIterator(var_field!((*exp).cref, NFExpression::CREF).clone())),
        Deref @ CALL { .. } => (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Call::functionName(var_field!((*exp).call, NFExpression::CALL).clone())?)?) {
        Deref @ "Connections" => false,
        Deref @ "cardinality" => false,
        _ => !(Call::isImpure(var_field!((*exp).call, NFExpression::CALL).clone())?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isPure)
}

pub fn containsCref(mut exp: Arc<NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool;
    b = fold(exp, (std::sync::Arc::new({ let __pe_b2 = cref; move |__pe_a0, __pe_a1| isCrefEqual(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, bool) -> Result<bool> + 'static>), false)?;
    Ok(b)
}

pub(crate) fn isCrefEqual(mut exp: Arc<NFExpression>, mut b: bool, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b, exp.clone())) {
        (false, Deref @ CREF { .. }) => ComponentRef::isEqual(var_field!((*exp).cref, NFExpression::CREF).clone(), cref)?,
        _ => b,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn containsCrefSet(mut exp: Arc<NFExpression>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut b: bool;
    b = fold(exp, (std::sync::Arc::new({ let __pe_b2 = set; move |__pe_a0, __pe_a1| isCrefEqualSet(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, bool) -> Result<bool> + 'static>), false)?;
    Ok(b)
}

pub(crate) fn isCrefEqualSet(mut exp: Arc<NFExpression>, mut b: bool, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b, exp.clone())) {
        (false, Deref @ CREF { .. }) => UnorderedSet::contains(var_field!((*exp).cref, NFExpression::CREF).clone(), set)?,
        _ => b,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn filterSplitIndices(mut exp: Arc<NFExpression>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { exp: _, subscripts: __esc_subs, .. } => {
            subs = (*__esc_subs).clone();
            subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            if !(!(filterSplitIndices2(s.clone(), node.clone()))) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if (subs.clone().is_empty()) {var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone()} else if (Type::isUnknown(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone())) {Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), subscripts: subs.clone(), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: List::any(subs.clone(), (std::sync::Arc::new(fnptr!(Subscript::isSplit, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>))? })} else {applySubscripts(subs.clone(), var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?}
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn filterSplitIndices2(mut sub: Arc<Subscript::NFSubscript>, mut node: Arc<InstNode::InstNode>) -> bool {
    let mut matching: bool;
    matching = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => InstNode::refEqual(var_field!((*sub).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), node),
        Deref @ Subscript::SPLIT_PROXY { .. } => InstNode::refEqual(var_field!((*sub).parent, Subscript::NFSubscript::SPLIT_PROXY).clone(), node),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    matching
}

pub fn expandSplitIndices(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { .. } => applySubscripts(Subscript::expandSplitIndices(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), metamodelica::nil())?, var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?,
        Deref @ CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF; cref = ComponentRef::expandSplitSubscripts(var_field!((*exp).cref, NFExpression::CREF).clone())?);
            exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn expandNonListedSplitIndices(mut exp: Arc<NFExpression>, mut indicesToKeep: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { split: true, .. } => applySubscripts(Subscript::expandSplitIndices(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), indicesToKeep)?, var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?,
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn isSplitSubscriptedExp(mut exp: Arc<NFExpression>) -> bool {
    let mut split: bool = false;
    split = (::match_deref::match_deref! { match &(exp) {
        Deref @ SUBSCRIPTED_EXP { split: __esc_split, .. } => {
            split = (*__esc_split).clone();
            split.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    split
}

pub(crate) fn mapSplitExpressions(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    let mut osub_repls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>;
    let mut sub_repls: Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut sub_exps: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut dim_sizes: Arc<metamodelica::List<Arc<NFExpression>>>;
    (outExp, osub_repls) = mapFold(exp.clone(), (std::sync::Arc::new(replaceSplitSubscripts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>) -> Result<(Arc<NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>)> + 'static>), None)?;
    if isNone(osub_repls.clone()) {
        outExp = func(exp)?;
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(osub_repls) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        sub_repls = __pa0.clone();
        subs = UnorderedMap::keyList(sub_repls.clone());
        sub_exps = UnorderedMap::valueList(sub_repls.clone());
        dim_sizes = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Subscript::splitIndexDimExp(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        dim_sizes = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut d in (dim_sizes).into_iter().cloned() {
            let __x = (replaceSplitSubscripts(d.clone(), Some(sub_repls.clone()))?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        outExp = mapSplitExpressions2(outExp, dim_sizes, sub_exps, func.clone())?;
        outExp = applySubscripts(subs, outExp, false)?;
    }
    Ok(outExp)
}

pub(crate) fn replaceSplitSubscripts(mut exp: Arc<NFExpression>, mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>) -> Result<(Arc<NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>> = subRepls;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { split: true, .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
            (subs, subRepls) = List::mapFold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), (std::sync::Arc::new(replaceSplitSubscripts2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>) -> Result<(Arc<Subscript::NFSubscript>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>)> + 'static>), subRepls)?;
            applySubscripts(subs, var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, subRepls))
}

pub(crate) fn replaceSplitSubscripts2(mut subscript: Arc<Subscript::NFSubscript>, mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>) -> Result<(Arc<Subscript::NFSubscript>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>)> {
    let mut subscript: Arc<Subscript::NFSubscript> = subscript;
    let mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>> = subRepls;
    let mut sub_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut sub_repls: Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>> = <Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>> as ::std::default::Default>::default();
    subscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            if isSome(subRepls.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(subRepls.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                sub_repls = __pa0.clone();
            } else {
                sub_repls = UnorderedMap::new((std::sync::Arc::new(Subscript::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<i32> + 'static>), (std::sync::Arc::new(Subscript::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>, Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>), 1);
                subRepls = Some(sub_repls.clone());
            }
            sub_exp = makeMutable(Arc::new(NFExpression::INTEGER { value: 0 }));
            sub_exp = UnorderedMap::tryAdd(subscript, sub_exp, sub_repls)?;
            Arc::new(Subscript::NFSubscript::INDEX { index: sub_exp })
        },
        _ => subscript,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((subscript, subRepls))
}

pub(crate) fn mapSplitExpressions2(mut exp: Arc<NFExpression>, mut dimSizes: Arc<metamodelica::List<Arc<NFExpression>>>, mut subExps: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    let mut dim_size: Arc<NFExpression>;
    let mut rest_dims: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut dim_size_int: i32;
    let mut sub_exp: Arc<NFExpression>;
    let mut rest_subs: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    let mut ty: Arc<Type::NFType>;
    if dimSizes.clone().is_empty() {
        outExp = map(exp, (std::sync::Arc::new(mapSplitExpressions3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
        outExp = func(outExp)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dimSizes) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim_size = __pa0.clone();
        rest_dims = __pa1.clone();
        dim_size_int = toInteger(Ceval::evalExp(dim_size, Ceval::noTarget().clone())?)?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(subExps) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        sub_exp = __pa2.clone();
        rest_subs = __pa3.clone();
        expl = metamodelica::arrayCreate(dim_size_int, exp.clone());
        for mut i in 1..=dim_size_int {
            updateMutable(sub_exp.clone(), Arc::new(NFExpression::INTEGER { value: i.clone() }))?;
            unsafe { metamodelica::Dangerous::arrayInitSlot(expl.clone(), i.clone(), mapSplitExpressions2(exp.clone(), rest_dims.clone(), rest_subs.clone(), func.clone())?) };
        }
        ty = typeOf(if (expl.clone().borrow().is_empty()) {exp} else {metamodelica::arrayGet(expl.clone(), 1)?});
        outExp = makeExpArray(expl.clone(), ty, Array::all(expl.clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?);
    }
    Ok(outExp)
}

pub(crate) fn mapSplitExpressions3(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MUTABLE { .. } => Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()),
        Deref @ SUBSCRIPTED_EXP { subscripts: __esc_subs, .. } => {
            subs = (*__esc_subs).clone();
            applySubscripts(subs.clone(), var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn mapCrefScalars(mut crefExp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    (outExp, _) = ExpandExp::expand(crefExp, false, false)?;
    outExp = mapCrefScalars2(outExp, mapFn.clone())?;
    Ok(outExp)
}

pub(crate) fn mapCrefScalars2(mut exp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static>;

    let mut outExp: Arc<NFExpression>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut literal: bool = false;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut arr: metamodelica::Array<Arc<NFExpression>> = Default::default();
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } if (!(var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty())) => {
            arr = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static> = mapFn.clone(); move |__pe_a0| mapCrefScalars2(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?;
            ty = typeOf(metamodelica::arrayGet(arr.clone(), 1)?);
            literal = Array::all(arr.clone(), (std::sync::Arc::new(isLiteral) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>))?;
            makeExpArray(arr.clone(), ty, literal)
        },
        Deref @ CREF { .. } => mapFn(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn isFunctionPointer(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(exp) {
        Deref @ CREF { ty: Deref @ Type::FUNCTION { .. }, .. } => true,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isClockOrSampleFunction(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp) {
        Deref @ CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: _ }, .. } } => {
            (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "sample" => !(isLiteral(arg.clone())?),
        Deref @ "subSample" => true,
        Deref @ "superSample" => true,
        Deref @ "shiftSample" => true,
        Deref @ "backSample" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ CLKCONST { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isConnector(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => {
            node = ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF).clone())?;
            InstNode::isComponent(node.clone())? && InstNode::isConnector(node)?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn isComponentExpression(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isCref(var_field!((*exp).cref, NFExpression::CREF).clone()) && InstNode::isComponent(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF).clone())?)?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn clone(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = metamodelica::arrayFromVec(var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn toJSON(mut exp: Arc<NFExpression>) -> Result<Arc<JSON::JSON>> {
    fn dump_arg(mut name: ArcStr, mut arg: Arc<NFExpression>) -> Result<Arc<JSON::JSON>> {
        let mut json: Arc<JSON::JSON> = JSON::emptyListObject();
        json = JSON::addPair((literal!("name")).clone(), JSON::makeString((name).clone()), json)?;
        json = JSON::addPair((literal!("value")).clone(), toJSON(arg)?, json)?;
        Ok(json)
    }

    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => JSON::makeInteger(var_field!((*exp).value, NFExpression::INTEGER).clone()),
        Deref @ REAL { .. } => JSON::makeNumber(var_field!((*exp).value, NFExpression::REAL).clone()),
        Deref @ STRING { .. } => JSON::makeString((var_field!((*exp).value, NFExpression::STRING).clone()).clone()),
        Deref @ BOOLEAN { .. } => JSON::makeBoolean(var_field!((*exp).value, NFExpression::BOOLEAN).clone()),
        Deref @ ENUM_LITERAL { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("enum")).clone() }), json)?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((toString(exp.clone())?).clone()), json)?;
            json = JSON::addPair((literal!("index")).clone(), JSON::makeInteger(var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone()), json)?;
            json
        },
        Deref @ CLKCONST { .. } => ClockKind::toJSON(var_field!((*exp).clk, NFExpression::CLKCONST).clone())?,
        Deref @ CREF { .. } => ComponentRef::toJSON(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        Deref @ TYPENAME { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("typename")).clone() }), json)?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((Type::toString(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?).clone()), json)?;
            json
        },
        Deref @ ARRAY { .. } => JSON::makeList(({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toJSON(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })),
        Deref @ RANGE { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("range")).clone() }), json)?;
            json = JSON::addPair((literal!("start")).clone(), toJSON(var_field!((*exp).start, NFExpression::RANGE).clone())?, json)?;
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                json = JSON::addPair((literal!("step")).clone(), toJSON(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?)?, json)?;
            }
            json = JSON::addPair((literal!("stop")).clone(), toJSON(var_field!((*exp).stop, NFExpression::RANGE).clone())?, json)?;
            json
        },
        Deref @ TUPLE { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("tuple")).clone() }), json)?;
            json = JSON::addPair((literal!("elements")).clone(), JSON::makeList(({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toJSON(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), json)?;
            json
        },
        Deref @ RECORD { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("record")).clone() }), json)?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((AbsynUtil::pathString(var_field!((*exp).path, NFExpression::RECORD).clone(), (literal!(".")).clone(), true, false)?).clone()), json)?;
            json = JSON::addPair((literal!("elements")).clone(), JSON::makeList(({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = toJSON(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), json)?;
            json
        },
        Deref @ CALL { .. } => Call::toJSON(var_field!((*exp).call, NFExpression::CALL).clone())?,
        Deref @ SIZE { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("call")).clone() }), json)?;
            json = JSON::addPair((literal!("name")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("size")).clone() }), json)?;
            if isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()) {
                json = JSON::addPair((literal!("arguments")).clone(), JSON::makeList(list![toJSON(var_field!((*exp).exp, NFExpression::SIZE).clone())?, toJSON(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?]), json)?;
            } else {
                json = JSON::addPair((literal!("arguments")).clone(), JSON::makeArray(list![toJSON(var_field!((*exp).exp, NFExpression::SIZE).clone())?]), json)?;
            }
            json
        },
        Deref @ BINARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("binary_op")).clone() }), json)?;
            json = JSON::addPair((literal!("lhs")).clone(), toJSON(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, json)?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::BINARY).clone()), json)?;
            json = JSON::addPair((literal!("rhs")).clone(), toJSON(var_field!((*exp).exp2, NFExpression::BINARY).clone())?, json)?;
            json
        },
        Deref @ UNARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("unary_op")).clone() }), json)?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::UNARY).clone()), json)?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).exp, NFExpression::UNARY).clone())?, json)?;
            json
        },
        Deref @ LBINARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("binary_op")).clone() }), json)?;
            json = JSON::addPair((literal!("lhs")).clone(), toJSON(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, json)?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::LBINARY).clone()), json)?;
            json = JSON::addPair((literal!("rhs")).clone(), toJSON(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?, json)?;
            json
        },
        Deref @ LUNARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("unary_op")).clone() }), json)?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::LUNARY).clone()), json)?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).exp, NFExpression::LUNARY).clone())?, json)?;
            json
        },
        Deref @ RELATION { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("binary_op")).clone() }), json)?;
            json = JSON::addPair((literal!("lhs")).clone(), toJSON(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, json)?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::RELATION).clone()), json)?;
            json = JSON::addPair((literal!("rhs")).clone(), toJSON(var_field!((*exp).exp2, NFExpression::RELATION).clone())?, json)?;
            json
        },
        Deref @ MULTARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("multary_op")).clone() }), json)?;
            json = JSON::addPair((literal!("args")).clone(), JSON::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut a in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = toJSON(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), json)?;
            json = JSON::addPair((literal!("inv_args")).clone(), JSON::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut a in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = toJSON(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), json)?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::MULTARY).clone()), json)?;
            json
        },
        Deref @ IF { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("if")).clone() }), json)?;
            json = JSON::addPair((literal!("condition")).clone(), toJSON(var_field!((*exp).condition, NFExpression::IF).clone())?, json)?;
            json = JSON::addPair((literal!("true")).clone(), toJSON(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, json)?;
            json = JSON::addPair((literal!("false")).clone(), toJSON(var_field!((*exp).falseBranch, NFExpression::IF).clone())?, json)?;
            json
        },
        Deref @ CAST { .. } => toJSON(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => toJSON(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => toJSON(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("sub")).clone() }), json)?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, json)?;
            json = JSON::addPair((literal!("subscripts")).clone(), Subscript::toJSONList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?, json)?;
            json
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("tuple_element")).clone() }), json)?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?, json)?;
            json = JSON::addPair((literal!("index")).clone(), JSON::makeInteger(var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone()), json)?;
            json
        },
        Deref @ RECORD_ELEMENT { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("record_element")).clone() }), json)?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?, json)?;
            json = JSON::addPair((literal!("index")).clone(), JSON::makeInteger(var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone()), json)?;
            json = JSON::addPair((literal!("field")).clone(), JSON::makeString((var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone()), json)?;
            json
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("function")).clone() }), json)?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((ComponentRef::toString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?).clone()), json)?;
            json = JSON::addPair((literal!("arguments")).clone(), JSON::makeList(({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        let __thr_src0 = var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(arg), Some(name)) => {
                    let __x = dump_arg((name.clone()).clone(), arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    })), json)?;
            json
        },
        Deref @ FILENAME { .. } => JSON::makeString((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone()),
        _ => JSON::makeString((toString(exp.clone())?).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn tupleElements(mut exp: Arc<NFExpression>) -> Arc<metamodelica::List<Arc<NFExpression>>> {
    let mut expl: Arc<metamodelica::List<Arc<NFExpression>>>;
    expl = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ TUPLE { .. } => var_field!((*exp).elements, NFExpression::TUPLE).clone(),
        _ => list![exp],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    expl
}

pub(crate) fn wrapCall(mut exp: Arc<NFExpression>, mut fun: Arc<dyn ::std::ops::Fn(Arc<Call::NFCall>) -> Result<Arc<Call::NFCall>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type callFun = std::sync::Arc<dyn ::std::ops::Fn(Arc<Call::NFCall>) -> Result<Arc<Call::NFCall>> + 'static>;

    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => {
            assign_variant_field!(exp => NFExpression::CALL; call = fun(var_field!((*exp).call, NFExpression::CALL).clone())?);
            exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn repairOperator(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { .. } => {
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::repairBinary(var_field!((*exp).operator, NFExpression::BINARY).clone(), typeOf(var_field!((*exp).exp1, NFExpression::BINARY).clone()), typeOf(var_field!((*exp).exp2, NFExpression::BINARY).clone()))?);
            exp
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY; operator = Operator::repairMultary(var_field!((*exp).operator, NFExpression::MULTARY).clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut e in (listAppend(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone())).into_iter().cloned() {
            let __x = typeOf(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?);
            exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn makeUnary(mut op: Arc<Operator::NFOperator>, mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut unaryExp: Arc<NFExpression>;
    if op.op.clone() == Operator::Op::ADD.clone() {
        unaryExp = exp;
    } else if op.op.clone() == Operator::Op::UMINUS.clone() {
        unaryExp = negate(exp);
    } else {
        unaryExp = Arc::new(NFExpression::UNARY { operator: op, exp: exp });
    }
    unaryExp
}

pub fn replaceLiteral(mut exp: Arc<NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<NFExpression>, i32>>, mut idx_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFExpression>> {
    fn replace(mut exp: Arc<NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<NFExpression>, i32>>, mut idx_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFExpression>> {
        let mut exp: Arc<NFExpression> = exp;
        let mut idx: i32;
        let mut idx_opt: Option<i32>;
        idx_opt = UnorderedMap::get(exp.clone(), map.clone())?;
        if isSome(idx_opt.clone()) {
            idx = Util::getOption(idx_opt)?;
        } else {
            idx = Pointer::access(idx_ptr.clone());
            Pointer::update(idx_ptr, idx + 1);
            UnorderedMap::add(exp.clone(), idx, map)?;
        }
        exp = Arc::new(NFExpression::SHARED_LITERAL { index: idx, exp: exp });
        Ok(exp)
    }

    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SHARED_LITERAL { .. } => exp.clone(),
        Deref @ ARRAY { .. } if (isLiteralReplace(exp.clone())?) => replace(replaceLiteralArrayElements(exp.clone(), map.clone(), idx_ptr.clone())?, map, idx_ptr)?,
        Deref @ RECORD { .. } if (isLiteralReplace(exp.clone())?) => {
            assign_variant_field!(exp => NFExpression::RECORD; elements = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut elem in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = replaceLiteral(elem.clone(), map.clone(), idx_ptr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            replace(exp.clone(), map, idx_ptr)?
        },
        _ if (isLiteralReplace(exp.clone())?) => replace(exp.clone(), map, idx_ptr)?,
        _ => mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = map; let __pe_b2 = idx_ptr; move |__pe_a0| replaceLiteral(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn replaceLiteralArrayElements(mut exp: Arc<NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<NFExpression>, i32>>, mut idx_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = map; let __pe_b2 = idx_ptr; move |__pe_a0| replaceLiteralArrayElements(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?);
            exp
        },
        _ => replaceLiteral(exp, map, idx_ptr)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn replaceCrefWithBinding(mut cref: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type recurse = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>;

    let mut exp: Arc<NFExpression> = exp;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(InstNode::getBindingExpOpt(ComponentRef::node(cref)?)?) {
        Some(__esc_e @ Deref @ INTEGER { .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Some(__esc_e @ Deref @ CREF { .. }) => {
            e = (*__esc_e).clone();
            replaceCrefWithBinding(var_field!((*e).cref, NFExpression::CREF).clone(), e.clone(), func.clone())?
        },
        Some(Deref @ SUBSCRIPTED_EXP { exp: __esc_e @ Deref @ INTEGER { .. }, .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Some(Deref @ SUBSCRIPTED_EXP { exp: __esc_e @ Deref @ CREF { .. }, .. }) => {
            e = (*__esc_e).clone();
            replaceCrefWithBinding(var_field!((*e).cref, NFExpression::CREF).clone(), e.clone(), func.clone())?
        },
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            e = map(e.clone(), func.clone())?;
            e.clone()
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceResizableParameterWithOriginal(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } if (ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?) => replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), (std::sync::Arc::new(replaceResizableParameterWithOriginal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceResizableParameter(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { varPointer: var, .. }, .. }, .. } if (ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?) => {
            let mut v: i32 = 0;
            (::match_deref::match_deref! { match &(Pointer::access(var.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { resize_value: Some(__esc_v) }, .. }, .. } => {
            v = (*__esc_v).clone();
            Arc::new(NFExpression::INTEGER { value: v.clone() })
        },
        _ => replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), (std::sync::Arc::new(replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ CREF { .. } if (ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?) => {
            replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), (std::sync::Arc::new(replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>))?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn mulResultType(mut tl: Arc<Type::NFType>, mut tr: Arc<Type::NFType>) -> Arc<Type::NFType> {
    let mut tres: Arc<Type::NFType>;
    if Type::isArray(tl.clone()) && Type::isArray(tr.clone()) {
        tres = tl;
    } else if Type::isArray(tl.clone()) {
        tres = tl;
    } else if Type::isArray(tr.clone()) {
        tres = tr;
    } else {
        tres = tl;
    }
    tres
}

pub(crate) fn mmul(mut lhs: Arc<NFExpression>, mut rhs: Arc<NFExpression>, mut baseOp: Arc<Operator::NFOperator>) -> Result<Arc<NFExpression>> {
    let mut prod: Arc<NFExpression>;
    let mut tl: Arc<Type::NFType> = typeOf(lhs.clone());
    let mut tr: Arc<Type::NFType> = typeOf(rhs.clone());
    let mut lArr: bool = Type::isArray(tl.clone());
    let mut rArr: bool = Type::isArray(tr.clone());
    let mut sizeClass: Operator::SizeClassification;
    let mut resTy: Arc<Type::NFType>;
    let mut op: Arc<Operator::NFOperator>;
    if !(lArr) && !(rArr) {
        sizeClass = Operator::SizeClassification::SCALAR.clone();
    } else if !(lArr) && rArr {
        sizeClass = Operator::SizeClassification::SCALAR_ARRAY.clone();
    } else if lArr && !(rArr) {
        sizeClass = Operator::SizeClassification::ARRAY_SCALAR.clone();
    } else {
        sizeClass = Operator::SizeClassification::ELEMENT_WISE.clone();
    }
    resTy = mulResultType(tl, tr);
    op = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass), resTy)?;
    prod = Arc::new(NFExpression::BINARY { exp1: lhs, operator: op, exp2: rhs });
    Ok(prod)
}

pub fn productOfListExceptSelf(mut arguments: Arc<metamodelica::List<Arc<NFExpression>>>, mut mulOp: Arc<Operator::NFOperator>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut products: Arc<metamodelica::List<Arc<NFExpression>>>;
    let mut n: i32 = (arguments.clone().len() as i32);
    let mut argsArr: metamodelica::Array<Arc<NFExpression>>;
    let mut pref: metamodelica::Array<Arc<NFExpression>>;
    let mut res: metamodelica::Array<Arc<NFExpression>>;
    let mut i: i32;
    let mut rightProd: Arc<NFExpression>;
    let mut baseTy: Arc<Type::NFType> = mulOp.ty.clone();
    let mut elTy: Arc<Type::NFType>;
    if n == 0 {
        products = metamodelica::nil();
        return Ok(products.clone());
    }
    elTy = if (Type::isArray(baseTy.clone())) {Type::arrayElementType(baseTy)} else {baseTy};
    argsArr = arrayCreate(n, makeOne(elTy.clone())?);
    i = 1;
    for mut a in &*arguments {
        let mut a = a.clone();
        {
            let __cell0 = a.clone();
            let __idx0 = i;
            argsArr.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
        i = i + 1;
    }
    pref = arrayCreate(n, makeOne(elTy.clone())?);
    res = arrayCreate(n, makeOne(elTy.clone())?);
    for mut i in 2..=n {
        {
            let __cell1 = mmul(({let __elt = pref.borrow()[(i - 1-1) as usize].clone(); __elt}), ({let __elt = argsArr.borrow()[(i - 1-1) as usize].clone(); __elt}), mulOp.clone())?;
            let __idx1 = i;
            pref.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
        }
    }
    rightProd = makeOne(elTy)?;
    for mut i in ({let __s=n; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        {
            let __cell2 = mmul(({let __elt = pref.borrow()[(i-1) as usize].clone(); __elt}), rightProd.clone(), mulOp.clone())?;
            let __idx2 = i;
            res.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
        }
        rightProd = mmul(rightProd.clone(), ({let __elt = argsArr.borrow()[(i-1) as usize].clone(); __elt}), mulOp.clone())?;
    }
    products = metamodelica::nil();
    for mut i in ({let __s=n; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        products = metamodelica::cons(SimplifyExp::simplify(({let __elt = res.borrow()[(i-1) as usize].clone(); __elt}), false)?, products.clone());
    }
    Ok(products)
}


