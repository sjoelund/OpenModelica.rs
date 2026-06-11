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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::Slot;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum MatchKind {
    /// Exact match
    EXACT = 1,
    /// Matched by casting, e.g. Integer to Real
    CAST = 2,
    /// The expected type was unknown
    UNKNOWN_EXPECTED = 3,
    /// The actual type was unknown
    UNKNOWN_ACTUAL = 4,
    /// Matched with a generic type e.g. function F<T> input T i; end F; F(1)
    GENERIC = 5,
    /// Component by component matching, e.g. class A R r; end A; is plug compatible with class B R r; end B;
    PLUG_COMPATIBLE = 6,
    NOT_COMPATIBLE = 7,
}
impl PartialOrd for MatchKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for MatchKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for MatchKind {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub(crate) fn isCompatibleMatch(mut kind: MatchKind) -> bool {
    let mut isCompatible: bool = kind != MatchKind::NOT_COMPATIBLE.clone();
    isCompatible
}

pub(crate) fn isIncompatibleMatch(mut kind: MatchKind) -> bool {
    let mut isIncompatible: bool = kind == MatchKind::NOT_COMPATIBLE.clone();
    isIncompatible
}

pub(crate) fn isExactMatch(mut kind: MatchKind) -> bool {
    let mut isCompatible: bool = kind == MatchKind::EXACT.clone();
    isCompatible
}

pub(crate) fn isCastMatch(mut kind: MatchKind) -> bool {
    let mut isCast: bool = kind == MatchKind::CAST.clone();
    isCast
}

pub(crate) fn isGenericMatch(mut kind: MatchKind) -> bool {
    let mut isCast: bool = kind == MatchKind::GENERIC.clone();
    isCast
}

pub(crate) fn isValidAssignmentMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind == MatchKind::EXACT.clone() || kind == MatchKind::CAST.clone() || kind == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub(crate) fn isValidArgumentMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind == MatchKind::EXACT.clone() || kind == MatchKind::CAST.clone() || kind == MatchKind::GENERIC.clone() || kind == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub(crate) fn isValidPlugCompatibleMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind == MatchKind::EXACT.clone() || kind == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub type MatchOptions = i32;

pub const DEFAULT_OPTIONS: i32 = 0;

pub(crate) const ALLOW_UNKNOWN: i32 = intBitLShift(1, 0);

pub(crate) const IGNORE_DIMENSIONS: i32 = intBitLShift(1, 1);

pub(crate) const IGNORE_DIMENSIONS_IN_RECORDS: i32 = intBitLShift(1, 2);

pub(crate) fn setOption(mut currentOptions: MatchOptions, mut newOption: MatchOptions) -> MatchOptions {
    let mut newOptions: MatchOptions = intBitOr(currentOptions, newOption);
    newOptions
}

pub(crate) fn getOption(mut options: MatchOptions, mut option: MatchOptions) -> bool {
    let mut isSet: bool = intBitAnd(options, option) > 0;
    isSet
}

pub(crate) fn checkBinaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    if Type::isConditionalArray(type1.clone()) || Type::isConditionalArray(type2.clone()) {
        (binaryExp, resultType) = checkConditionalBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info, retype)?;
    } else if Type::isComplex(Type::arrayElementType(type1.clone())) || Type::isComplex(Type::arrayElementType(type2.clone())) {
        (binaryExp, resultType) = checkOverloadedBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info)?;
    } else if Type::isBoxed(type1.clone()) && Type::isBoxed(type2.clone()) {
        (binaryExp, resultType) = checkBinaryOperationBoxed(exp1, type1, var1, operator, exp2, type2, var2, context, info, retype)?;
    } else {
        (binaryExp, resultType) = (match operator.op.clone() {
        Operator::Op::ADD => checkBinaryOperationAdd(exp1, type1, exp2, type2, info)?,
        Operator::Op::SUB => checkBinaryOperationSub(exp1, type1, exp2, type2, info)?,
        Operator::Op::MUL => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::DIV => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, retype)?,
        Operator::Op::POW => checkBinaryOperationPow(exp1, type1, exp2, type2, info)?,
        Operator::Op::ADD_EW => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::ADD.clone(), info)?,
        Operator::Op::SUB_EW => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::SUB.clone(), info)?,
        Operator::Op::MUL_EW => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::MUL.clone(), info)?,
        Operator::Op::DIV_EW => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, true)?,
        Operator::Op::POW_EW => checkBinaryOperationPowEW(exp1, type1, exp2, type2, info)?,
        Operator::Op::ADD_SCALAR_ARRAY => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::ADD.clone(), info)?,
        Operator::Op::ADD_ARRAY_SCALAR { .. } => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::ADD.clone(), info)?,
        Operator::Op::SUB_SCALAR_ARRAY { .. } => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::SUB.clone(), info)?,
        Operator::Op::SUB_ARRAY_SCALAR => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::SUB.clone(), info)?,
        Operator::Op::MUL_SCALAR_ARRAY => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::MUL_ARRAY_SCALAR { .. } => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::MUL_VECTOR_MATRIX => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::MUL_MATRIX_VECTOR => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::SCALAR_PRODUCT => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::MATRIX_PRODUCT => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, retype)?,
        Operator::Op::DIV_ARRAY_SCALAR { .. } => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, retype)?,
        Operator::Op::POW_SCALAR_ARRAY { .. } => checkBinaryOperationPowEW(exp1, type1, exp2, type2, info)?,
        Operator::Op::POW_ARRAY_SCALAR { .. } => checkBinaryOperationPowEW(exp1, type1, exp2, type2, info)?,
        Operator::Op::POW_MATRIX => checkBinaryOperationPow(exp1, type1, exp2, type2, info)?,
        _ => bail!("match: no arm matched"),
    });
    }
    Ok((binaryExp, resultType))
}

pub(crate) fn checkOverloadedBinaryOperator(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut op_str: ArcStr;
    let mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>;
    let mut ety1: Arc<Type::NFType>;
    let mut ety2: Arc<Type::NFType>;
    op_str = (Operator::symbol(Operator::stripEW(op.clone()), (literal!("'")).clone())?).clone();
    ety1 = Type::arrayElementType(type1.clone());
    ety2 = Type::arrayElementType(type2.clone());
    candidates = OperatorOverloading::lookupOperatorFunctionsInType((op_str.clone()).clone(), ety1.clone())?;
    if !(Type::isEqual(ety1, ety2.clone())?) {
        candidates = listAppend(OperatorOverloading::lookupOperatorFunctionsInType((op_str).clone(), ety2)?, candidates);
    }
    if candidates.clone().is_empty() {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    if Operator::isElementWise(op.clone()) {
        (outExp, outType) = checkOverloadedBinaryArrayEW(exp1, type1, var1, Operator::stripEW(op), exp2, type2, var2, candidates, context, info)?;
    } else {
        (outExp, outType) = matchOverloadedBinaryOperator(exp1, type1, var1, op, exp2, type2, var2, candidates, context, info, true)?;
    }
    outExp = Inline::inlineCallExp(outExp, false)?;
    Ok((outExp, outType))
}

pub(crate) fn matchOverloadedBinaryOperator(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo, mut showErrors: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>>;
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>>;
    let mut exactMatches: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>>;
    let mut r#fn: Arc<Function::Function>;
    args = list![Arc::new(TypedArg { name: None, value: exp1.clone(), ty: type1.clone(), var: var1, purity: Purity::PURE.clone() }), Arc::new(TypedArg { name: None, value: exp2.clone(), ty: type2.clone(), var: var2, purity: Purity::PURE.clone() })];
    matchedFunctions = Function::matchFunctionsSilent(candidates.clone(), args, metamodelica::nil(), context, info.clone(), true)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if exactMatches.clone().is_empty() {
        ErrorExt::setCheckpoint((literal!("NFTypeCheck:implicitConstruction")).clone());
        if '__try0: {
            (outExp, outType) = unwrap_break_err!(implicitConstructAndMatch(candidates.clone(), exp1.clone(), type1.clone(), op.clone(), exp2.clone(), type2.clone(), info.clone()), '__try0);
            if showErrors {
                ErrorExt::delCheckpoint((literal!("NFTypeCheck:implicitConstruction")).clone());
            } else {
                ErrorExt::rollBack((literal!("NFTypeCheck:implicitConstruction")).clone());
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            ErrorExt::rollBack((literal!("NFTypeCheck:implicitConstruction")).clone());
            if Type::isArray(type1.clone()) || Type::isArray(type2.clone()) {
                (outExp, outType) = (match op.op.clone() {
        Operator::Op::ADD => checkOverloadedBinaryArrayAddSub(exp1.clone(), type1.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone())?,
        Operator::Op::SUB => checkOverloadedBinaryArrayAddSub(exp1.clone(), type1.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone())?,
        Operator::Op::MUL => checkOverloadedBinaryArrayMul(exp1.clone(), type1.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone())?,
        Operator::Op::DIV => checkOverloadedBinaryArrayDiv(exp1.clone(), type1.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone())?,
        _ => {
            printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), showErrors)?;
            bail!("fail")
        },
    });
            } else {
                printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), showErrors)?;
            }
        }
    } else if (exactMatches.clone().len() as i32) == 1 {
        let __pa1 = ::match_deref::match_deref! { match &(exactMatches) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        matchedFunc = __pa1.clone();
        r#fn = matchedFunc.func.clone();
        outType = Function::returnType(r#fn);
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(matchedFunc.func.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut a in (matchedFunc.args.clone()).into_iter().cloned() {
            let __x = a.value.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), Prefixes::variabilityMax(var1, var2), Purity::PURE.clone(), outType.clone()) });
    } else {
        if showErrors {
            Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 }))?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut mfn in (matchedFunctions).into_iter().cloned() {
            let __x = mfn.func.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?).clone()], info)?;
        }
        bail!("fail");
    }
    Ok((outExp, outType))
}

pub(crate) fn checkBinaryOperationBoxed(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    (e1, ty1, _) = matchTypes(type1.clone(), Type::unbox(type1), exp1, DEFAULT_OPTIONS.clone())?;
    (e2, ty2, _) = matchTypes(type2.clone(), Type::unbox(type2), exp2, DEFAULT_OPTIONS.clone())?;
    (outExp, outType) = checkBinaryOperation(e1, ty1, var1, op, e2, ty2, var2, context, info, retype)?;
    Ok((outExp, outType))
}

fn checkConditionalBinaryOperator(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tty1: Arc<Type::NFType>;
    let mut fty1: Arc<Type::NFType>;
    let mut tty2: Arc<Type::NFType>;
    let mut fty2: Arc<Type::NFType>;
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut valid1: bool;
    let mut valid2: bool;
    let mut branch: Type::Branch;
    (tty1, fty1, tty2, fty2, branch) = (::match_deref::match_deref! { match &((type1.clone(), type2.clone())) {
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => (var_field!((*type1).trueType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type1).falseType, Type::NFType::CONDITIONAL_ARRAY).clone(), type2.clone(), type2.clone(), var_field!((*type1).matchedBranch, Type::NFType::CONDITIONAL_ARRAY).clone()),
        (_, Deref @ Type::CONDITIONAL_ARRAY { .. }) => (type1.clone(), type1.clone(), var_field!((*type2).trueType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type2).falseType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type2).matchedBranch, Type::NFType::CONDITIONAL_ARRAY).clone()),
        _ => bail!("match: no arm matched"),
    } });
    ErrorExt::setCheckpoint(literal!("NFTypeCheck.checkConditionalBinaryOperator"));
    match '__try0: {
        (e1, ty1) = unwrap_break_err!(checkBinaryOperation(exp1.clone(), tty1.clone(), var1, op.clone(), exp2.clone(), tty2.clone(), var2, context, info.clone(), retype), '__try0);
        valid1 = true;
        Ok::<_, anyhow::Error>((valid1.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valid1 = __try0_o0;
        }
        Err(_) => {
            valid1 = false;
        }
    }
    match '__try1: {
        (e2, ty2) = unwrap_break_err!(checkBinaryOperation(exp1.clone(), fty1.clone(), var1, op.clone(), exp2.clone(), fty2.clone(), var2, context, info.clone(), retype), '__try1);
        valid2 = true;
        Ok::<_, anyhow::Error>((valid2.clone(),))
    } {
        Ok((__try1_o0,)) => {
            valid2 = __try1_o0;
        }
        Err(_) => {
            valid2 = false;
        }
    }
    ErrorExt::rollBack(literal!("NFTypeCheck.checkConditionalBinaryOperator"));
    if valid1 && valid2 {
        outType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: ty1, falseType: ty2, matchedBranch: branch });
        outExp = e1;
    } else if valid1 {
        outType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: ty1, falseType: crate::NFType::interned_UNKNOWN(), matchedBranch: Type::Branch::TRUE.clone() });
        outExp = e1;
    } else if valid2 {
        outType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: crate::NFType::interned_UNKNOWN(), falseType: ty2, matchedBranch: Type::Branch::FALSE.clone() });
        outExp = e2;
    } else {
        printUnresolvableTypeError(exp1, list![type1, type2], info, true)?;
    }
    outExp = Expression::setType(outType.clone(), outExp)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayAddSub(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    (e1, e2, _, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    if !(isCompatibleMatch(mk)) {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    (e1, _) = ExpandExp::expand(e1, false, false)?;
    (e2, _) = ExpandExp::expand(e2, false, false)?;
    (outExp, outType) = checkOverloadedBinaryArrayAddSub2(e1, type1, var1, op, e2, type2, var2, candidates, context, info)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayAddSub2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    (outExp, outType) = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::ARRAY { elements: arr1, .. }, Deref @ Expression::ARRAY { elements: arr2, .. }) => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty1: Arc<Type::NFType>;
            let mut ty2: Arc<Type::NFType>;
            let mut e: Arc<Expression::NFExpression>;
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            if arr1.clone().borrow().is_empty() {
                ty1 = Type::arrayElementType(type1.clone());
                ty2 = Type::arrayElementType(type2.clone());
                arr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                if '__try0: {
                    (_, ty) = unwrap_break_err!(matchOverloadedBinaryOperator(Arc::new(Expression::NFExpression::EMPTY { ty: ty1.clone() }), ty1.clone(), var1, op.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: ty2.clone() }), ty2.clone(), var2, candidates.clone(), context, info.clone(), false), '__try0);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
                }
            } else {
                ty1 = Type::unliftArray(type1.clone())?;
                ty2 = Type::unliftArray(type2)?;
                arr = metamodelica::arrayCreateDefault(metamodelica::arrayLength(arr1.clone()));
                for mut i in 1..=metamodelica::arrayLength(arr1.clone()) {
                    e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i.clone());
                    e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i.clone());
                    (e, ty) = checkOverloadedBinaryArrayAddSub2(e1.clone(), ty1.clone(), var1, op.clone(), e2.clone(), ty2.clone(), var2, candidates.clone(), context, info.clone())?;
                    unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), e.clone()) };
                }
            }
            outType = Type::setArrayElementType(type1, ty.clone());
            outExp = Expression::makeArray(outType.clone(), arr.clone(), false);
            (outExp, outType)
        },
        _ => {
            matchOverloadedBinaryOperator(exp1, type1, var1, op, exp2, type2, var2, candidates, context, info, true)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayMul(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType>;
    let mut valid: bool = false;
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dim11: Arc<Dimension::NFDimension>;
    let mut dim12: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim21: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dims1 = Type::arrayDims(type1.clone());
    dims2 = Type::arrayDims(type2.clone());
    (valid, outExp) = (::match_deref::match_deref! { match &((dims1, dims2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }) => {
            (outExp, _) = checkOverloadedBinaryScalarArray(exp1, type1.clone(), var1, op, exp2, type2.clone(), var2, candidates, context, info.clone())?;
            (true, outExp)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => {
            (outExp, _) = checkOverloadedBinaryArrayScalar(exp1, type1.clone(), var1, op, exp2, type2.clone(), var2, candidates, context, info.clone())?;
            (true, outExp)
        },
        (Deref @ metamodelica::List::Cons { head: __esc_dim11, tail: Deref @ metamodelica::List::Cons { head: __esc_dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: __esc_dim21, tail: Deref @ metamodelica::List::Nil }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 });
            valid = false;
            (valid, outExp)
        },
        (Deref @ metamodelica::List::Cons { head: __esc_dim11, tail: Deref @ metamodelica::List::Cons { head: __esc_dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: __esc_dim21, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 });
            valid = false;
            (valid, outExp)
        },
        _ => (false, Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(valid) {
        printUnresolvableTypeError(outExp.clone(), list![type1, type2], info, true)?;
    }
    outType = Expression::typeOf(outExp.clone());
    Ok((outExp, outType))
}

fn checkOverloadedBinaryScalarArray(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    (outExp, outType) = checkOverloadedBinaryScalarArray2(exp1, type1, var1, op, (ExpandExp::expand(exp2, false, false)?).0, type2, var2, candidates, context, info)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryScalarArray2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (outExp, outType) = (::match_deref::match_deref! { match &(exp2.clone()) {
        Deref @ Expression::ARRAY { .. } if (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty()) => {
            if '__try0: {
                ty = unwrap_break_err!(Type::unliftArray(type2.clone()), '__try0);
                (_, outType) = unwrap_break_err!(matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1, op.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: type2.clone() }), ty.clone(), var2, candidates.clone(), context, info.clone(), false), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone()], info.clone(), true)?;
            }
            outType = Type::setArrayElementType(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), outType);
            (Expression::makeEmptyArray(outType.clone()), outType)
        },
        Deref @ Expression::ARRAY { .. } => {
            ty = Type::unliftArray(type2)?;
            arr = metamodelica::arrayCreate(metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone()), exp2.clone());
            for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
                e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), i.clone());
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), (checkOverloadedBinaryScalarArray2(exp1.clone(), type1.clone(), var1, op.clone(), e2.clone(), ty.clone(), var2, candidates.clone(), context, info.clone())?).0) };
            }
            outType = Type::setArrayElementType(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Expression::typeOf(({let __elt = arr.borrow()[(1-1) as usize].clone(); __elt})));
            (Expression::makeArray(outType.clone(), arr.clone(), false), outType)
        },
        _ => matchOverloadedBinaryOperator(exp1, type1, var1, op, exp2.clone(), type2, var2, candidates, context, info, true)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayScalar(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    (outExp, outType) = checkOverloadedBinaryArrayScalar2((ExpandExp::expand(exp1, false, false)?).0, type1, var1, op, exp2, type2, var2, candidates, context, info)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayScalar2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    (outExp, outType) = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::ARRAY { .. } if (var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty()) => {
            if '__try0: {
                ty = unwrap_break_err!(Type::unliftArray(type1.clone()), '__try0);
                (_, outType) = unwrap_break_err!(matchOverloadedBinaryOperator(Arc::new(Expression::NFExpression::EMPTY { ty: type1.clone() }), ty.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone(), false), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone()], info.clone(), true)?;
            }
            outType = Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), outType);
            (Expression::makeEmptyArray(outType.clone()), outType)
        },
        Deref @ Expression::ARRAY { .. } => {
            ty = Type::unliftArray(type1)?;
            arr = metamodelica::arrayCreate(metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()), exp1.clone());
            for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
                e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), i.clone());
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), (checkOverloadedBinaryArrayScalar2(e1.clone(), ty.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone())?).0) };
            }
            outType = Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Expression::typeOf(({let __elt = arr.borrow()[(1-1) as usize].clone(); __elt})));
            (Expression::makeArray(outType.clone(), arr.clone(), false), outType)
        },
        _ => matchOverloadedBinaryOperator(exp1.clone(), type1, var1, op, exp2, type2, var2, candidates, context, info, true)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayDiv(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    if Type::isArray(type1.clone()) && Type::isScalar(type2.clone()) {
        (outExp, outType) = checkOverloadedBinaryArrayScalar(exp1, type1, var1, op, exp2, type2, var2, candidates, context, info)?;
    } else {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 }), list![type1, type2], info, true)?;
    }
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayEW(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    if Type::isArray(type1.clone()) && Type::isArray(type2.clone()) {
        (e1, e2, _, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    } else {
        (e1, e2, _, mk) = matchExpressions(exp1.clone(), Type::arrayElementType(type1.clone()), exp2.clone(), Type::arrayElementType(type2.clone()), ALLOW_UNKNOWN.clone())?;
    }
    if !(isCompatibleMatch(mk)) {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: op.clone(), exp2: e2 }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    (e1, _) = ExpandExp::expand(exp1, false, false)?;
    (e2, _) = ExpandExp::expand(exp2, false, false)?;
    (outExp, outType) = checkOverloadedBinaryArrayEW2(e1, type1, var1, op, e2, type2, var2, candidates, context, info)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayEW2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut expl1: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut expl2: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut is_array1: bool;
    let mut is_array2: bool;
    is_array1 = Type::isArray(type1.clone());
    is_array2 = Type::isArray(type2.clone());
    if is_array1 || is_array2 {
        expl = metamodelica::nil();
        if Expression::isEmptyArray(exp1.clone()) || Expression::isEmptyArray(exp2.clone()) {
            ty1 = Type::arrayElementType(type1.clone());
            ty2 = Type::arrayElementType(type2.clone());
            if '__try0: {
                (_, ty) = unwrap_break_err!(matchOverloadedBinaryOperator(Arc::new(Expression::NFExpression::EMPTY { ty: ty1.clone() }), ty1.clone(), var1, op.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: ty2.clone() }), ty2.clone(), var2, candidates.clone(), context, info.clone(), true), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
            }
        } else if is_array1 && is_array2 {
            ty1 = Type::unliftArray(type1.clone())?;
            ty2 = Type::unliftArray(type2)?;
            expl1 = Expression::arrayElements(exp1)?;
            expl2 = Expression::arrayElements(exp2)?;
            if metamodelica::arrayLength(expl1.clone()) > metamodelica::arrayLength(expl2.clone()) {
                bail!("fail");
            }
            for mut i in 1..=metamodelica::arrayLength(expl1.clone()) {
                e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(expl1.clone(), i.clone());
                e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(expl2.clone(), i.clone());
                (e1, ty) = checkOverloadedBinaryArrayEW2(e1.clone(), ty1.clone(), var1, op.clone(), e2.clone(), ty2.clone(), var2, candidates.clone(), context, info.clone())?;
                expl = metamodelica::cons(e1.clone(), expl.clone());
            }
        } else if is_array1 {
            ty1 = Type::unliftArray(type1.clone())?;
            expl1 = Expression::arrayElements(exp1)?;
            let __range1 = expl1.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range1 {
                (e, ty) = checkOverloadedBinaryArrayEW2(e.clone(), ty1.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates.clone(), context, info.clone())?;
                expl = metamodelica::cons(e.clone(), expl.clone());
            }
        } else if is_array2 {
            ty2 = Type::unliftArray(type2)?;
            expl2 = Expression::arrayElements(exp2)?;
            let __range2 = expl2.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range2 {
                (e, ty) = checkOverloadedBinaryArrayEW2(exp1.clone(), type1.clone(), var1, op.clone(), e.clone(), ty2.clone(), var2, candidates.clone(), context, info.clone())?;
                expl = metamodelica::cons(e.clone(), expl.clone());
            }
        }
        outType = Type::setArrayElementType(type1, ty);
        outExp = Expression::makeArray(outType.clone(), metamodelica::arrayFromVec(metamodelica::Dangerous::listReverseInPlace(expl).into_iter().cloned().collect()), false);
    } else {
        (outExp, outType) = matchOverloadedBinaryOperator(exp1, type1, var1, op, exp2, type2, var2, candidates, context, info, true)?;
    }
    Ok((outExp, outType))
}

fn implicitConstructAndMatch(mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut inExp1: Arc<Expression::NFExpression>, mut inType1: Arc<Type::NFType>, mut op: Arc<Operator::NFOperator>, mut inExp2: Arc<Expression::NFExpression>, mut inType2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    let mut in1: Arc<InstNode::InstNode>;
    let mut in2: Arc<InstNode::InstNode>;
    let mut operfn: Arc<Function::Function>;
    let mut matchedfuncs: Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>> = metamodelica::nil();
    let mut exp1: Arc<Expression::NFExpression>;
    let mut exp2: Arc<Expression::NFExpression>;
    let mut arg1_ty: Arc<Type::NFType>;
    let mut arg2_ty: Arc<Type::NFType>;
    let mut var: Variability;
    let mut matched: bool;
    let mut arg1_info: SourceInfo;
    let mut arg2_info: SourceInfo;
    exp1 = inExp1.clone();
    exp2 = inExp2.clone();
    for mut r#fn in &*candidates {
        let mut r#fn = r#fn.clone();
        if (r#fn.inputs.clone().len() as i32) != 2 {
            continue;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#fn.inputs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        in1 = __pa0.clone();
        in2 = __pa1.clone();
        arg1_ty = InstNode::getType(in1.clone())?;
        arg2_ty = InstNode::getType(in2.clone())?;
        arg1_info = InstNode::info(in1.clone());
        arg2_info = InstNode::info(in2.clone());
        (matchedfuncs, matched) = implicitConstructAndMatch2(inExp1.clone(), inType1.clone(), inExp2.clone(), arg1_ty.clone(), arg1_info.clone(), arg2_ty.clone(), arg2_info.clone(), InstNode::classScope(in2.clone()), r#fn.clone(), false, matchedfuncs.clone())?;
        if matched {
            continue;
        }
        (matchedfuncs, matched) = implicitConstructAndMatch2(inExp2.clone(), inType2.clone(), inExp1.clone(), arg2_ty.clone(), arg2_info.clone(), arg1_ty.clone(), arg1_info.clone(), InstNode::classScope(in1.clone()), r#fn.clone(), true, matchedfuncs.clone())?;
    }
    if (matchedfuncs.clone().len() as i32) == 1 {
        let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(matchedfuncs) {
            Deref @ metamodelica::List::Cons { head: (__pa3, Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } }, __pa6), tail: _ } => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        operfn = __pa3.clone();
        exp1 = __pa4.clone();
        exp2 = __pa5.clone();
        var = __pa6.clone();
        outType = Function::returnType(operfn.clone());
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(operfn, list![exp1, exp2], var, Purity::PURE.clone(), outType.clone()) });
    } else {
        Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 }))?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (matchedfuncs).into_iter().cloned() {
            let __x = Util::tuple31(r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?).clone()], info)?;
        bail!("fail");
    }
    Ok((outExp, outType))
}

fn implicitConstructAndMatch2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut paramType1: Arc<Type::NFType>, mut paramInfo1: SourceInfo, mut paramType2: Arc<Type::NFType>, mut paramInfo2: SourceInfo, mut scope: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>, mut reverseArgs: bool, mut matchedFns: Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>>) -> Result<(Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>>, bool)> {
    let mut matchedFns: Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>> = matchedFns;
    let mut matched: bool;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut var: Variability;
    let mut ty: Arc<Type::NFType>;
    (e1, _, mk) = matchTypes(paramType1, type1, exp1, DEFAULT_OPTIONS.clone())?;
    if mk == MatchKind::EXACT.clone() {
        (fn_ref, _, _) = Function::instFunction(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("'constructor'")).clone(), subscripts: metamodelica::nil() }), scope.clone(), InstContext::NO_CONTEXT.clone(), paramInfo2)?;
        e2 = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: fn_ref, arguments: list![exp2], named_args: metamodelica::nil(), call_scope: scope }) });
        (e2, ty, var, _) = Call::typeCall(e2, 0, paramInfo1, false)?;
        (_, _, mk) = matchTypes(paramType2, ty, e2.clone(), DEFAULT_OPTIONS.clone())?;
        if mk == MatchKind::EXACT.clone() {
            matchedFns = metamodelica::cons((r#fn, if (reverseArgs) {list![e2, e1]} else {list![e1, e2]}, var), matchedFns);
            matched = true;
        } else {
            matched = false;
        }
    } else {
        matched = false;
    }
    Ok((matchedFns, matched))
}

fn checkBinaryOperationAdd(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut valid: bool;
    (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    valid = (::match_deref::match_deref! { match &(Type::arrayElementType(resultType.clone())) {
        Deref @ Type::INTEGER => valid,
        Deref @ Type::REAL => valid,
        Deref @ Type::STRING => valid,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: Operator::makeAdd(resultType.clone()), exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationSub(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut valid: bool;
    (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    valid = (::match_deref::match_deref! { match &(Type::arrayElementType(resultType.clone())) {
        Deref @ Type::INTEGER => valid,
        Deref @ Type::REAL => valid,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: Operator::makeSub(resultType.clone()), exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationMul(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dim11: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim12: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim21: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim22: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut mk: MatchKind;
    let mut op: Op;
    let mut valid: bool;
    ty1 = Type::arrayElementType(type1.clone());
    ty2 = Type::arrayElementType(type2.clone());
    (e1, e2, resultType, mk) = matchExpressions(exp1, ty1, exp2, ty2, ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    valid = (::match_deref::match_deref! { match &(resultType.clone()) {
        Deref @ Type::INTEGER => valid,
        Deref @ Type::REAL => valid,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dims1 = Type::arrayDims(type1.clone());
    dims2 = Type::arrayDims(type2.clone());
    (resultType, op) = (::match_deref::match_deref! { match &((dims1.clone(), dims2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => (resultType, Op::MUL.clone()),
        (Deref @ metamodelica::List::Nil, _) => (Arc::new(Type::NFType::ARRAY { elementType: resultType, dimensions: dims2 }), Op::MUL_SCALAR_ARRAY.clone()),
        (_, Deref @ metamodelica::List::Nil) => (Arc::new(Type::NFType::ARRAY { elementType: resultType, dimensions: dims1 }), Op::MUL_ARRAY_SCALAR.clone()),
        (Deref @ metamodelica::List::Cons { head: __esc_dim11, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: __esc_dim21, tail: Deref @ metamodelica::List::Nil }) => {
            dim11 = (*__esc_dim11).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(dim11.clone(), dim21.clone())?;
            (resultType, Op::SCALAR_PRODUCT.clone())
        },
        (Deref @ metamodelica::List::Cons { head: __esc_dim11, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: __esc_dim21, tail: Deref @ metamodelica::List::Cons { head: __esc_dim22, tail: Deref @ metamodelica::List::Nil } }) => {
            dim11 = (*__esc_dim11).clone();
            dim21 = (*__esc_dim21).clone();
            dim22 = (*__esc_dim22).clone();
            valid = Dimension::isEqual(dim11.clone(), dim21.clone())?;
            (Arc::new(Type::NFType::ARRAY { elementType: resultType, dimensions: list![dim22.clone()] }), Op::MUL_VECTOR_MATRIX.clone())
        },
        (Deref @ metamodelica::List::Cons { head: __esc_dim11, tail: Deref @ metamodelica::List::Cons { head: __esc_dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: __esc_dim21, tail: Deref @ metamodelica::List::Nil }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            (Arc::new(Type::NFType::ARRAY { elementType: resultType, dimensions: list![dim11.clone()] }), Op::MUL_MATRIX_VECTOR.clone())
        },
        (Deref @ metamodelica::List::Cons { head: __esc_dim11, tail: Deref @ metamodelica::List::Cons { head: __esc_dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: __esc_dim21, tail: Deref @ metamodelica::List::Cons { head: __esc_dim22, tail: Deref @ metamodelica::List::Nil } }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            dim22 = (*__esc_dim22).clone();
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            (Arc::new(Type::NFType::ARRAY { elementType: resultType, dimensions: list![dim11.clone(), dim22.clone()] }), Op::MATRIX_PRODUCT.clone())
        },
        _ => {
            valid = false;
            (resultType, Op::MUL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: Arc::new(Operator::NFOperator { ty: resultType.clone(), op: op }), exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationDiv(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo, mut isElementWise: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut op: Arc<Operator::NFOperator>;
    (e1, ty1, mk) = matchTypes(type1.clone(), Type::setArrayElementType(type1.clone(), crate::NFType::interned_REAL()), exp1, ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    (e2, ty2, mk) = matchTypes(type2.clone(), Type::setArrayElementType(type2.clone(), crate::NFType::interned_REAL()), exp2, ALLOW_UNKNOWN.clone())?;
    valid = valid && isCompatibleMatch(mk);
    (resultType, op) = (match (Type::isArray(ty1.clone()), Type::isArray(ty2.clone()), isElementWise) {
        (false, false, _) => (ty1.clone(), Operator::makeDiv(ty1)),
        (_, false, _) => (ty1.clone(), Arc::new(Operator::NFOperator { ty: ty1, op: Op::DIV_ARRAY_SCALAR.clone() })),
        (false, _, true) => (ty2.clone(), Arc::new(Operator::NFOperator { ty: ty2, op: Op::DIV_SCALAR_ARRAY.clone() })),
        (true, _, true) => {
            (_, _, mk) = matchArrayTypes(ty1.clone(), ty2, e1.clone(), ALLOW_UNKNOWN.clone())?;
            valid = valid && isCompatibleMatch(mk);
            (ty1.clone(), Operator::makeDiv(ty1))
        },
        _ => {
            valid = false;
            (ty1.clone(), Operator::makeDiv(ty1))
        },
    });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: op, exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationPow(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut op: Arc<Operator::NFOperator>;
    (e1, resultType, mk) = matchTypes(type1.clone(), Type::setArrayElementType(type1.clone(), crate::NFType::interned_REAL()), exp1, ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    if Type::isArray(resultType.clone()) {
        valid = valid && Type::isSquareMatrix(resultType.clone())?;
        valid = valid && Type::isInteger(type2.clone())?;
        valid = valid && !(Expression::isNegative(exp2.clone())?);
        op = Arc::new(Operator::NFOperator { ty: resultType.clone(), op: Op::POW_MATRIX.clone() });
        e2 = exp2;
    } else {
        (e2, _, mk) = matchTypes(type2.clone(), crate::NFType::interned_REAL(), exp2, ALLOW_UNKNOWN.clone())?;
        valid = valid && isCompatibleMatch(mk);
        op = Arc::new(Operator::NFOperator { ty: resultType.clone(), op: Op::POW.clone() });
    }
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: op, exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationPowEW(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut op: Arc<Operator::NFOperator>;
    (e1, ty1, mk) = matchTypes(type1.clone(), Type::setArrayElementType(type1.clone(), crate::NFType::interned_REAL()), exp1, ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    (e2, ty2, mk) = matchTypes(type2.clone(), Type::setArrayElementType(type2.clone(), crate::NFType::interned_REAL()), exp2, ALLOW_UNKNOWN.clone())?;
    valid = valid && isCompatibleMatch(mk);
    (resultType, op) = (match (Type::isArray(ty1.clone()), Type::isArray(ty2.clone())) {
        (false, false) => (ty1.clone(), Operator::makePow(ty1)),
        (_, false) => (ty1.clone(), Arc::new(Operator::NFOperator { ty: ty1, op: Op::POW_ARRAY_SCALAR.clone() })),
        (false, _) => (ty2.clone(), Arc::new(Operator::NFOperator { ty: ty2, op: Op::POW_SCALAR_ARRAY.clone() })),
        _ => {
            (_, _, mk) = matchArrayTypes(ty1.clone(), ty2, e1.clone(), ALLOW_UNKNOWN.clone())?;
            valid = valid && isCompatibleMatch(mk);
            (ty1.clone(), Operator::makePow(ty1))
        },
    });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: op, exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationEW(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut elemOp: Op, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut is_arr1: bool;
    let mut is_arr2: bool;
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    is_arr1 = Type::isArray(type1.clone());
    is_arr2 = Type::isArray(type2.clone());
    if is_arr1 && is_arr2 {
        (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    } else {
        ty1 = Type::arrayElementType(type1.clone());
        ty2 = Type::arrayElementType(type2.clone());
        (e1, e2, resultType, mk) = matchExpressions(exp1, ty1, exp2, ty2, ALLOW_UNKNOWN.clone())?;
    }
    valid = isCompatibleMatch(mk);
    valid = (::match_deref::match_deref! { match &((Type::arrayElementType(resultType.clone()), elemOp)) {
        (Deref @ Type::INTEGER, _) => valid,
        (Deref @ Type::REAL, _) => valid,
        (Deref @ Type::STRING, Operator::Op::ADD) => valid,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (resultType, op) = (match (is_arr1, is_arr2) {
        (true, false) => {
            resultType = Type::copyDims(type1.clone(), resultType);
            op = Operator::makeArrayScalar(resultType.clone(), elemOp)?;
            (resultType, op)
        },
        (false, true) => {
            resultType = Type::copyDims(type2.clone(), resultType);
            op = Operator::makeScalarArray(resultType.clone(), elemOp)?;
            (resultType, op)
        },
        (true, true) => (resultType.clone(), Operator::makeEW(Arc::new(Operator::NFOperator { ty: resultType, op: elemOp }))),
        _ => (resultType.clone(), Arc::new(Operator::NFOperator { ty: resultType, op: elemOp })),
    });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1, operator: op, exp2: e2 });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

pub(crate) fn checkUnaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut unaryExp: Arc<Expression::NFExpression>;
    let mut unaryType: Arc<Type::NFType>;
    let mut op: Arc<Operator::NFOperator>;
    if Type::isComplex(Type::arrayElementType(type1.clone())) {
        (unaryExp, unaryType) = checkOverloadedUnaryOperator(exp1, type1, var1, operator, context, info)?;
        return Ok((unaryExp.clone(), unaryType.clone()));
    }
    unaryType = type1.clone();
    op = Operator::setType(unaryType.clone(), operator.clone());
    unaryExp = (match operator.op.clone() {
        Operator::Op::ADD => exp1,
        _ => Arc::new(Expression::NFExpression::UNARY { operator: op, exp: exp1 }),
    });
    if !(Type::isNumeric(type1.clone())?) {
        printUnresolvableTypeError(unaryExp.clone(), list![type1], info, true)?;
    }
    Ok((unaryExp, unaryType))
}

pub(crate) fn checkOverloadedUnaryOperator(mut inExp1: Arc<Expression::NFExpression>, mut inType1: Arc<Type::NFType>, mut var: Variability, mut inOp: Arc<Operator::NFOperator>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut outType: Arc<Type::NFType>;
    let mut opstr: ArcStr;
    let mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>;
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>>;
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut exactMatches: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>>;
    opstr = (Operator::symbol(inOp.clone(), (literal!("'")).clone())?).clone();
    candidates = OperatorOverloading::lookupOperatorFunctionsInType((opstr).clone(), inType1.clone())?;
    args = list![Arc::new(TypedArg { name: None, value: inExp1.clone(), ty: inType1.clone(), var: var, purity: Purity::PURE.clone() })];
    matchedFunctions = Function::matchFunctionsSilent(candidates, args, metamodelica::nil(), context, info.clone(), false)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if exactMatches.clone().is_empty() {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::UNARY { operator: inOp.clone(), exp: inExp1.clone() }), list![inType1], info.clone(), true)?;
        bail!("fail");
    }
    if (exactMatches.clone().len() as i32) == 1 {
        let __pa0 = ::match_deref::match_deref! { match &(exactMatches) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        matchedFunc = __pa0.clone();
        outType = Function::returnType(matchedFunc.func.clone());
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(matchedFunc.func.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut a in (matchedFunc.args.clone()).into_iter().cloned() {
            let __x = a.value.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), var, Purity::PURE.clone(), outType.clone()) });
    } else {
        Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::UNARY { operator: inOp, exp: inExp1 }))?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut mfn in (matchedFunctions).into_iter().cloned() {
            let __x = mfn.func.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?).clone()], info)?;
        bail!("fail");
    }
    outExp = Inline::inlineCallExp(outExp, false)?;
    Ok((outExp, outType))
}

pub(crate) fn checkLogicalBinaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    if Type::isComplex(Type::arrayElementType(type1.clone())) || Type::isComplex(Type::arrayElementType(type2.clone())) {
        (outExp, resultType) = checkOverloadedBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info)?;
        return Ok((outExp.clone(), resultType.clone()));
    }
    (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    outExp = Arc::new(Expression::NFExpression::LBINARY { exp1: e1, operator: Operator::setType(resultType.clone(), operator), exp2: e2 });
    if !(isCompatibleMatch(mk)) || !(Type::isBoolean(Type::arrayElementType(resultType.clone()))) {
        printUnresolvableTypeError(outExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((outExp, resultType))
}

pub(crate) fn checkLogicalUnaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType> = type1.clone();
    if Type::isComplex(Type::arrayElementType(type1.clone())) {
        (outExp, resultType) = checkOverloadedUnaryOperator(exp1, type1, var1, operator, context, info)?;
        return Ok((outExp.clone(), resultType.clone()));
    }
    outExp = Arc::new(Expression::NFExpression::LUNARY { operator: Operator::setType(type1.clone(), operator), exp: exp1 });
    if !(Type::isBoolean(Type::arrayElementType(type1.clone()))) {
        printUnresolvableTypeError(outExp.clone(), list![type1], info, true)?;
    }
    Ok((outExp, resultType))
}

pub(crate) fn checkRelationOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut index: i32, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut resultType: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut o: Op = Op::ADD;
    if Type::isComplex(Type::arrayElementType(type1.clone())) || Type::isComplex(Type::arrayElementType(type2.clone())) {
        (outExp, resultType) = checkOverloadedBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info)?;
        return Ok((outExp.clone(), resultType.clone()));
    }
    (e1, e2, ty, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), DEFAULT_OPTIONS.clone())?;
    valid = isCompatibleMatch(mk);
    resultType = crate::NFType::interned_BOOLEAN();
    outExp = Arc::new(Expression::NFExpression::RELATION { exp1: e1, operator: Operator::setType(ty.clone(), operator.clone()), exp2: e2, index: index });
    valid = (::match_deref::match_deref! { match &(ty) {
        Deref @ Type::INTEGER => valid,
        Deref @ Type::REAL => {
            o = operator.op.clone();
            if !(InstContext::inFunction(context)) && (o == Op::EQUAL.clone() || o == Op::NEQUAL.clone()) {
                Error::addStrictMessage(Error::WARNING_RELATION_ON_REAL.clone(), list![(Expression::toString(outExp.clone())?).clone(), (Operator::symbol(operator, (literal!("")).clone())?).clone()], info.clone())?;
            }
            valid
        },
        Deref @ Type::STRING => valid,
        Deref @ Type::BOOLEAN => valid,
        Deref @ Type::ENUMERATION { .. } => valid,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(valid) {
        printUnresolvableTypeError(outExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((outExp, resultType))
}

pub(crate) fn printUnresolvableTypeError(mut exp: Arc<Expression::NFExpression>, mut types: Arc<metamodelica::List<Arc<Type::NFType>>>, mut info: SourceInfo, mut printError: bool) -> Result<()> {
    let mut exp_str: ArcStr;
    let mut ty_str: ArcStr;
    if printError {
        exp_str = (Expression::toString(exp)?).clone();
        ty_str = (List::toString(types, (std::sync::Arc::new(Type::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), false, 0)?).clone();
        Error::addSourceMessage(Error::UNRESOLVABLE_TYPE.clone(), list![(exp_str).clone(), (ty_str).clone(), (literal!("<NO_COMPONENT>")).clone()], info)?;
    }
    bail!("fail");
    Ok(())
}

pub(crate) fn matchExpressions(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind;
    if referenceEq(&*(type1.clone()),&*(type2.clone())) {
        compatibleType = type1;
        matchKind = MatchKind::EXACT.clone();
        return Ok((exp1.clone(), exp2.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if metamodelica::valueConstructor((&*type1.clone()))? != metamodelica::valueConstructor((&*type2.clone()))? {
        (exp1, exp2, compatibleType, matchKind) = matchExpressions_cast(exp1, type1, exp2, type2, options)?;
        return Ok((exp1.clone(), exp2.clone(), compatibleType.clone(), matchKind.clone()));
    }
    matchKind = MatchKind::EXACT.clone();
    compatibleType = (::match_deref::match_deref! { match &(type1.clone()) {
        Deref @ Type::INTEGER => type1,
        Deref @ Type::REAL => type1,
        Deref @ Type::STRING => type1,
        Deref @ Type::BOOLEAN => type1,
        Deref @ Type::CLOCK => type1,
        Deref @ Type::ENUMERATION { .. } => {
            matchKind = matchEnumerationTypes(type1.clone(), type2)?;
            type1
        },
        Deref @ Type::ARRAY { .. } => {
            (exp1, exp2, compatibleType, matchKind) = matchArrayExpressions(exp1, type1, exp2, type2, options)?;
            compatibleType
        },
        Deref @ Type::TUPLE { .. } => {
            (exp1, compatibleType, matchKind) = matchTupleTypes(type1, type2, exp1, options)?;
            compatibleType
        },
        Deref @ Type::UNKNOWN => {
            matchKind = if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            type1
        },
        Deref @ Type::COMPLEX { .. } => {
            (exp1, compatibleType, matchKind) = matchComplexTypes(type1, type2, exp1, options)?;
            compatibleType
        },
        Deref @ Type::METABOXED { .. } => {
            (exp1, exp2, compatibleType, matchKind) = matchBoxedExpressions(exp1, type1, exp2, type2, options)?;
            compatibleType
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTypeCheck.matchExpressions")); __mm_s.push_str(&*literal!(" got unknown type.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTypeCheck.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub fn matchTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind;
    if referenceEq(&*(actualType.clone()),&*(expectedType.clone())) {
        compatibleType = actualType;
        matchKind = MatchKind::EXACT.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if metamodelica::valueConstructor((&*actualType.clone()))? != metamodelica::valueConstructor((&*expectedType.clone()))? {
        (expression, compatibleType, matchKind) = matchTypes_cast(actualType, expectedType, expression, options)?;
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    matchKind = MatchKind::EXACT.clone();
    compatibleType = (::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::INTEGER => actualType,
        Deref @ Type::REAL => actualType,
        Deref @ Type::STRING => actualType,
        Deref @ Type::BOOLEAN => actualType,
        Deref @ Type::CLOCK => actualType,
        Deref @ Type::ENUMERATION { .. } => {
            if Type::isUnspecifiedEnumeration(expectedType.clone()) {
                matchKind = MatchKind::EXACT.clone();
            } else {
                matchKind = matchEnumerationTypes(actualType.clone(), expectedType)?;
            }
            actualType
        },
        Deref @ Type::ARRAY { .. } => {
            (expression, compatibleType, matchKind) = matchArrayTypes(actualType, expectedType, expression, options)?;
            compatibleType
        },
        Deref @ Type::TUPLE { .. } => {
            (expression, compatibleType, matchKind) = matchTupleTypes(actualType, expectedType, expression, options)?;
            compatibleType
        },
        Deref @ Type::UNKNOWN => {
            matchKind = if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            actualType
        },
        Deref @ Type::COMPLEX { .. } => {
            (expression, compatibleType, matchKind) = matchComplexTypes(actualType, expectedType, expression, options)?;
            compatibleType
        },
        Deref @ Type::FUNCTION { .. } => {
            (expression, compatibleType, matchKind) = matchFunctionTypes(actualType, expectedType, expression, options)?;
            compatibleType
        },
        Deref @ Type::METABOXED { .. } => {
            (expression, compatibleType, matchKind) = matchTypes(var_field!((*actualType).ty, Type::NFType::METABOXED).clone(), Type::unbox(expectedType), Expression::unbox(expression), options)?;
            expression = Expression::r#box(expression);
            compatibleType = Type::r#box(compatibleType);
            compatibleType
        },
        Deref @ Type::CONDITIONAL_ARRAY { .. } => {
            (expression, compatibleType, matchKind) = matchConditionalArrayTypes(actualType, expectedType, expression, options)?;
            compatibleType
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTypeCheck.matchTypes")); __mm_s.push_str(&*literal!(" got unknown type.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTypeCheck.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchExpressions_cast(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut before: Arc<Expression::NFExpression> = exp1.clone();
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((type1.clone(), type2.clone())) {
        (Deref @ Type::INTEGER, Deref @ Type::REAL) => {
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::ENUMERATION { .. }, Deref @ Type::INTEGER) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdEnumerationAsIntegers")).clone())?) => {
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of enumeration expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" to Integer: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use Integer(")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(") instead!")); ArcStr::from(__mm_s) }).clone())?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::INTEGER, Deref @ Type::ENUMERATION { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdIntegersAsEnumeration")).clone())?) => {
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Integer expression: ")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(" to enumeration: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use the actual enumeration instead!")); ArcStr::from(__mm_s) }).clone())?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::REAL, Deref @ Type::INTEGER) => {
            exp2 = Expression::typeCast(exp2, type1.clone())?;
            (type1, MatchKind::CAST.clone())
        },
        (Deref @ Type::BOOLEAN, Deref @ Type::REAL) if (Flags::isSet(Flags::NF_API.clone())?) => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Boolean expression: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(" to Real.")); ArcStr::from(__mm_s) }).clone())?;
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::REAL, Deref @ Type::BOOLEAN) if (Flags::isSet(Flags::NF_API.clone())?) => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Boolean expression: ")); __mm_s.push_str(&*Expression::toString(exp2.clone())?); __mm_s.push_str(&*literal!(" to Real.")); ArcStr::from(__mm_s) }).clone())?;
            exp2 = Expression::typeCast(exp2, type1.clone())?;
            (type1, MatchKind::CAST.clone())
        },
        (Deref @ Type::TUPLE { types: Deref @ metamodelica::List::Cons { head: __esc_compatibleType, tail: _ }, .. }, _) => {
            compatibleType = (*__esc_compatibleType).clone();
            exp1 = Expression::tupleElement(exp1, compatibleType.clone(), 1)?;
            (exp1, compatibleType, matchKind) = matchTypes(compatibleType.clone(), type2, exp1, options)?;
            if isCompatibleMatch(matchKind) {
                matchKind = MatchKind::CAST.clone();
            }
            (compatibleType.clone(), matchKind)
        },
        (Deref @ Type::UNKNOWN, _) => (type2, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (_, Deref @ Type::UNKNOWN) => (type1, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (Deref @ Type::METABOXED { .. }, _) => {
            (exp1, exp2, compatibleType, matchKind) = matchExpressions(Expression::unbox(exp1), var_field!((*type1).ty, Type::NFType::METABOXED).clone(), exp2, type2, options)?;
            (compatibleType.clone(), matchKind)
        },
        (_, Deref @ Type::METABOXED { .. }) => {
            (exp1, exp2, compatibleType, matchKind) = matchExpressions(exp1, type1, Expression::unbox(exp2), var_field!((*type2).ty, Type::NFType::METABOXED).clone(), options)?;
            (compatibleType.clone(), matchKind)
        },
        (_, Deref @ Type::POLYMORPHIC { .. }) => {
            exp1 = Expression::r#box(exp1);
            (Type::r#box(type1), MatchKind::GENERIC.clone())
        },
        (Deref @ Type::POLYMORPHIC { .. }, _) => {
            exp2 = Expression::r#box(exp2);
            (Type::r#box(type2), MatchKind::GENERIC.clone())
        },
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => {
            (exp1, exp2, compatibleType, matchKind) = matchConditionalArrayExp(exp1, type1, exp2, type2, options)?;
            (compatibleType.clone(), matchKind)
        },
        (_, Deref @ Type::CONDITIONAL_ARRAY { .. }) => {
            (exp2, exp1, compatibleType, matchKind) = matchConditionalArrayExp(exp2, type2, exp1, type1, options)?;
            (compatibleType.clone(), matchKind)
        },
        _ => (crate::NFType::interned_UNKNOWN(), MatchKind::NOT_COMPATIBLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub(crate) fn matchComplexTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = actualType.clone();
    let mut matchKind: MatchKind = MatchKind::NOT_COMPATIBLE.clone();
    let mut cls1: Arc<Class::NFClass>;
    let mut cls2: Arc<Class::NFClass>;
    let mut ctree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut anode: Arc<InstNode::InstNode>;
    let mut enode: Arc<InstNode::InstNode>;
    let mut comps1: metamodelica::Array<Arc<InstNode::InstNode>> = Default::default();
    let mut comps2: metamodelica::Array<Arc<InstNode::InstNode>> = Default::default();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cty1: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut cty2: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut matched_elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut elem_arr: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    let mut opt: MatchOptions = options;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    anode = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(expectedType.clone()) {
        Deref @ Type::COMPLEX { cls: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    enode = __pa1.clone();
    if InstNode::isSame(anode.clone(), enode.clone()) {
        matchKind = MatchKind::EXACT.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    cls1 = InstNode::getClass(anode)?;
    cls2 = InstNode::getClass(enode.clone())?;
    if getOption(opt, IGNORE_DIMENSIONS_IN_RECORDS.clone()) {
        opt = setOption(opt, IGNORE_DIMENSIONS.clone());
    }
    let () = (::match_deref::match_deref! { match &((cls1, actualType, cls2, expectedType.clone())) {
        (_, Deref @ Type::COMPLEX { complexTy: __esc_cty1 @ Deref @ ComplexType::CONNECTOR { .. }, .. }, _, Deref @ Type::COMPLEX { complexTy: __esc_cty2 @ Deref @ ComplexType::CONNECTOR { .. }, .. }) => {
            cty1 = (*__esc_cty1).clone();
            cty2 = (*__esc_cty2).clone();
            matchKind = matchComponentList(var_field!((*cty1).potentials, ComplexType::NFComplexType::CONNECTOR).clone(), var_field!((*cty2).potentials, ComplexType::NFComplexType::CONNECTOR).clone(), options)?;
            if matchKind != MatchKind::NOT_COMPATIBLE.clone() {
                matchKind = matchComponentList(var_field!((*cty1).flows, ComplexType::NFComplexType::CONNECTOR).clone(), var_field!((*cty2).flows, ComplexType::NFComplexType::CONNECTOR).clone(), options)?;
                if matchKind != MatchKind::NOT_COMPATIBLE.clone() {
                    matchKind = matchComponentList(var_field!((*cty1).streams, ComplexType::NFComplexType::CONNECTOR).clone(), var_field!((*cty2).streams, ComplexType::NFComplexType::CONNECTOR).clone(), options)?;
                }
            }
            if matchKind != MatchKind::NOT_COMPATIBLE.clone() {
                matchKind = MatchKind::PLUG_COMPATIBLE.clone();
            }
            ()
        },
        (Deref @ Class::INSTANCED_CLASS { elements: __esc_ctree @ Deref @ ClassTree::FLAT_TREE { components: __esc_comps1, .. }, .. }, _, Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_comps2, .. }, .. }, _) => {
            ctree = (*__esc_ctree).clone();
            comps1 = (*__esc_comps1).clone();
            comps2 = (*__esc_comps2).clone();
            if metamodelica::arrayLength(comps1.clone()) != metamodelica::arrayLength(comps2.clone()) {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
            }
            matchKind = MatchKind::PLUG_COMPATIBLE.clone();
            elem_arr = (::match_deref::match_deref! { match &(expression.clone()) {
        Deref @ Expression::RECORD { .. } => metamodelica::arrayFromVec(var_field!((*expression).elements, Expression::NFExpression::RECORD).clone().into_iter().cloned().collect()),
        _ => {
            elem_arr = metamodelica::arrayCreate(metamodelica::arrayLength(comps1.clone()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
            dims = Type::arrayDims(Expression::typeOf(expression.clone()));
            for mut i in ({let __s=metamodelica::arrayLength(comps1.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                ty = Component::getType(InstNode::component(({let __elt = comps1.borrow()[(i.clone()-1) as usize].clone(); __elt}))?)?;
                ty = Type::liftArrayRightList(ty.clone(), dims.clone());
                {
                    let __cell0 = Arc::new(Expression::NFExpression::RECORD_ELEMENT { recordExp: expression.clone(), index: i.clone(), fieldName: (InstNode::name(({let __elt = comps1.borrow()[(i.clone()-1) as usize].clone(); __elt}))?).clone(), ty: ty.clone() });
                    let __idx0 = i.clone();
                    unsafe { metamodelica::Dangerous::arrayInitSlot(elem_arr.clone().clone(), __idx0, __cell0); }
                }
            }
            elem_arr.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (matched_elements, matchKind) = matchComplexComponents(comps1.clone(), comps2.clone(), elem_arr.clone(), ctree.clone(), opt)?;
            if matchKind == MatchKind::CAST.clone() {
                expression = typeCastRecord(matched_elements, enode, expectedType, expression)?;
            }
            ()
        },
        _ => {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchComplexComponents(mut actualComponents: metamodelica::Array<Arc<InstNode::InstNode>>, mut expectedComponents: metamodelica::Array<Arc<InstNode::InstNode>>, mut expressions: metamodelica::Array<Arc<Expression::NFExpression>>, mut classTree: Arc<ClassTree::ClassTree>, mut options: MatchOptions) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, MatchKind)> {
    let mut matchedExpressions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut matchKind: MatchKind = MatchKind::PLUG_COMPATIBLE.clone();
    let mut anode: Arc<InstNode::InstNode>;
    let mut enode: Arc<InstNode::InstNode>;
    let mut acomp: Arc<Component::NFComponent>;
    let mut ecomp: Arc<Component::NFComponent>;
    let mut idx: i32;
    let mut e: Arc<Expression::NFExpression>;
    let mut mk: MatchKind;
    if metamodelica::arrayLength(actualComponents.clone()) != metamodelica::arrayLength(expectedComponents.clone()) || metamodelica::arrayLength(actualComponents.clone()) != metamodelica::arrayLength(expressions.clone()) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((matchedExpressions.clone(), matchKind.clone()));
    }
    for mut i in 1..=metamodelica::arrayLength(actualComponents.clone()) {
        enode = ({let __elt = expectedComponents.borrow()[(i.clone()-1) as usize].clone(); __elt});
        ecomp = InstNode::component(enode.clone())?;
        anode = ({let __elt = actualComponents.borrow()[(i.clone()-1) as usize].clone(); __elt});
        if InstNode::name(anode.clone())? == InstNode::name(enode.clone())? {
            idx = i.clone();
        } else {
            if let Ok(__iflet0) = ClassTree::lookupComponentIndex((InstNode::name(enode.clone())?).clone(), classTree.clone()) {
                idx = __iflet0;
            } else {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok((matchedExpressions.clone(), matchKind.clone()));
            }
            anode = ({let __elt = actualComponents.borrow()[(idx-1) as usize].clone(); __elt});
        }
        if i.clone() != idx {
            matchKind = MatchKind::CAST.clone();
        }
        acomp = InstNode::component(anode.clone())?;
        e = ({let __elt = expressions.borrow()[(idx-1) as usize].clone(); __elt});
        (e, _, mk) = matchTypes(Component::getType(acomp.clone())?, Component::getType(ecomp.clone())?, e.clone(), options)?;
        matchedExpressions = metamodelica::cons(e.clone(), matchedExpressions.clone());
        if mk == MatchKind::CAST.clone() {
            matchKind = mk;
        } else if !(isValidPlugCompatibleMatch(mk)) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            break;
        }
    }
    matchedExpressions = metamodelica::Dangerous::listReverseInPlace(matchedExpressions);
    Ok((matchedExpressions, matchKind))
}

pub(crate) fn typeCastRecord(mut expressions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut node: Arc<InstNode::InstNode>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut ty: Arc<Type::NFType>;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut iter: Arc<InstNode::InstNode>;
    let mut iters: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    let mut sub: Arc<Subscript::NFSubscript>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut i: i32;
    ty = Expression::typeOf(expression.clone());
    if Type::isArray(ty.clone()) {
        dims = Type::arrayDims(ty.clone());
        ranges = metamodelica::nil();
        iters = metamodelica::nil();
        subs = metamodelica::nil();
        i = 1;
        for mut d in &*dims.reverse() {
            let mut d = d.clone();
            if Dimension::isUnknown(d.clone()) {
                ranges = metamodelica::cons(Arc::new(Expression::NFExpression::RANGE { ty: crate::NFType::interned_INTEGER(), start: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), step: None, stop: Arc::new(Expression::NFExpression::SIZE { exp: expression.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i })) }) }), ranges.clone());
            } else {
                ranges = metamodelica::cons(Dimension::toRange(d.clone())?, ranges.clone());
            }
            iter = InstNode::newUniqueIterator(InstNode::info(node.clone()), crate::NFType::interned_INTEGER());
            iters = metamodelica::cons(iter.clone(), iters.clone());
            sub = Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::CREF { ty: crate::NFType::interned_INTEGER(), cref: ComponentRef::makeIterator(iter.clone(), crate::NFType::interned_INTEGER())? }) });
            subs = metamodelica::cons(sub.clone(), subs.clone());
            i = i + 1;
        }
        expression = Arc::new(Expression::NFExpression::RECORD { path: InstNode::scopePath(node, InstNode::ScopeType::RELATIVE.clone(), false)?, ty: expectedType, elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (expressions).into_iter().cloned() {
            let __x = Expression::applySubscripts(subs.clone(), e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) });
        expression = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty, var: Expression::variability(expression.clone())?, purity: Expression::purity(expression.clone())?, exp: expression, iters: ({
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        let __thr_src0 = iters;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = ranges;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(i), Some(r)) => {
                    let __x = (i.clone(), r.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    }) }) });
    } else {
        expression = Arc::new(Expression::NFExpression::RECORD { path: InstNode::scopePath(node, InstNode::ScopeType::RELATIVE.clone(), false)?, ty: expectedType, elements: expressions });
    }
    Ok(expression)
}

pub(crate) fn matchComponentList(mut comps1: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut comps2: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut options: MatchOptions) -> Result<MatchKind> {
    let mut matchKind: MatchKind;
    let mut c2: Arc<InstNode::InstNode>;
    let mut rest_c2: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = comps2.clone();
    let mut dummy: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    if (comps1.clone().len() as i32) != (comps2.len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
    } else {
        for mut c1 in &*comps1 {
            let mut c1 = c1.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_c2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            c2 = __pa0.clone();
            rest_c2 = __pa1.clone();
            if InstNode::name(c1.clone())? != InstNode::name(c2.clone())? {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok(matchKind.clone());
            }
            (_, _, matchKind) = matchTypes(InstNode::getType(c1.clone())?, InstNode::getType(c2.clone())?, dummy.clone(), options)?;
            if matchKind == MatchKind::NOT_COMPATIBLE.clone() {
                return Ok(matchKind.clone());
            }
        }
    }
    matchKind = MatchKind::PLUG_COMPATIBLE.clone();
    Ok(matchKind)
}

pub(crate) fn matchFunctionTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = actualType.clone();
    let mut matchKind: MatchKind = MatchKind::EXACT.clone();
    let mut inputs1: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    let mut inputs2: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    let mut outputs1: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    let mut outputs2: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    let mut slots1: Arc<metamodelica::List<Arc<Slot::Slot>>>;
    let mut slots2: Arc<metamodelica::List<Arc<Slot::Slot>>>;
    let mut slot1: Arc<Slot::Slot>;
    let mut slot2: Arc<Slot::Slot>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(actualType) {
        Deref @ Type::FUNCTION { r#fn: Deref @ Function::FUNCTION { inputs: __pa0, outputs: __pa1, slots: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    inputs1 = __pa0.clone();
    outputs1 = __pa1.clone();
    slots1 = __pa2.clone();
    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(expectedType) {
        Deref @ Type::FUNCTION { r#fn: Deref @ Function::FUNCTION { inputs: __pa4, outputs: __pa5, slots: __pa6, .. }, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    inputs2 = __pa4.clone();
    outputs2 = __pa5.clone();
    slots2 = __pa6.clone();
    if (outputs1.clone().len() as i32) != (outputs2.clone().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if !(matchFunctionParameters(outputs1, outputs2, options)?) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if !(matchFunctionParameters(inputs1, inputs2.clone(), options)?) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    for mut i in &*inputs2 {
        let mut i = i.clone();
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(slots1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
            _ => bail!("pattern mismatch"),
        } };
        slot1 = __pa8.clone();
        slots1 = __pa9.clone();
        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(slots2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
            _ => bail!("pattern mismatch"),
        } };
        slot2 = __pa10.clone();
        slots2 = __pa11.clone();
        if isSome(slot2.default.clone()) && isNone(slot1.default.clone()) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
        }
    }
    for mut slot in &*slots1 {
        let mut slot = slot.clone();
        if isNone(slot.default.clone()) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
        }
    }
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchFunctionParameters(mut params1: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut params2: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut options: MatchOptions) -> Result<bool> {
    let mut matching: bool = true;
    let mut pl1: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = params1.clone();
    let mut pl2: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = params2.clone();
    let mut p1: Arc<InstNode::InstNode>;
    let mut dummy: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    let mut mk: MatchKind;
    for mut p2 in &*pl2 {
        let mut p2 = p2.clone();
        if pl1.clone().is_empty() {
            matching = false;
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pl1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        p1 = __pa0.clone();
        pl1 = __pa1.clone();
        if InstNode::name(p1.clone())? != InstNode::name(p2.clone())? {
            matching = false;
            break;
        }
        (_, _, mk) = matchTypes(Type::unbox(InstNode::getType(p1.clone())?), Type::unbox(InstNode::getType(p2.clone())?), dummy.clone(), options)?;
        if mk != MatchKind::EXACT.clone() {
            matching = false;
            break;
        }
    }
    Ok(matching)
}

pub(crate) fn matchEnumerationTypes(mut type1: Arc<Type::NFType>, mut type2: Arc<Type::NFType>) -> Result<MatchKind> {
    let mut matchKind: MatchKind;
    let mut lits1: Arc<metamodelica::List<ArcStr>>;
    let mut lits2: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(type1) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lits1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(type2) {
        Deref @ Type::ENUMERATION { literals: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lits2 = __pa1.clone();
    matchKind = if (List::isEqualOnTrue(lits1, lits2, (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
    Ok(matchKind)
}

pub(crate) fn matchArrayExpressions(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut ety1: Arc<Type::NFType>;
    let mut ety2: Arc<Type::NFType>;
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(type1) {
        Deref @ Type::ARRAY { elementType: __pa0, dimensions: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ety1 = __pa0.clone();
    dims1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(type2) {
        Deref @ Type::ARRAY { elementType: __pa2, dimensions: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ety2 = __pa2.clone();
    dims2 = __pa3.clone();
    (exp1, exp2, compatibleType, matchKind) = matchExpressions(exp1, ety1, exp2, ety2, options)?;
    (compatibleType, matchKind) = matchArrayDims(dims1, dims2, compatibleType, matchKind, options)?;
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub(crate) fn matchArrayTypes(mut arrayType1: Arc<Type::NFType>, mut arrayType2: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut ety1: Arc<Type::NFType>;
    let mut ety2: Arc<Type::NFType>;
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(arrayType1) {
        Deref @ Type::ARRAY { elementType: __pa0, dimensions: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ety1 = __pa0.clone();
    dims1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(arrayType2) {
        Deref @ Type::ARRAY { elementType: __pa2, dimensions: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ety2 = __pa2.clone();
    dims2 = __pa3.clone();
    (expression, compatibleType, matchKind) = matchTypes(ety1, ety2, expression, options)?;
    (compatibleType, matchKind) = matchArrayDims(dims1, dims2, compatibleType, matchKind, options)?;
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchArrayDims(mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut ty: Arc<Type::NFType>, mut matchKind: MatchKind, mut options: MatchOptions) -> Result<(Arc<Type::NFType>, MatchKind)> {
    let mut ty: Arc<Type::NFType> = ty;
    let mut matchKind: MatchKind = matchKind;
    let mut rest_dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = dims2.clone();
    let mut cdims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim2: Arc<Dimension::NFDimension>;
    let mut compat: bool;
    if !(isCompatibleMatch(matchKind)) {
        return Ok((ty.clone(), matchKind.clone()));
    }
    if (dims1.clone().len() as i32) != (dims2.len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((ty.clone(), matchKind.clone()));
    }
    for mut dim1 in &*dims1 {
        let mut dim1 = dim1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim2 = __pa0.clone();
        rest_dims2 = __pa1.clone();
        (dim1, compat) = matchDimensions(dim1.clone(), dim2.clone())?;
        if !(compat) && !(getOption(options, IGNORE_DIMENSIONS.clone())) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            break;
        }
        cdims = metamodelica::cons(dim1.clone(), cdims.clone());
    }
    ty = Arc::new(Type::NFType::ARRAY { elementType: ty, dimensions: metamodelica::Dangerous::listReverseInPlace(cdims) });
    Ok((ty, matchKind))
}

pub(crate) fn matchDimensions(mut dim1: Arc<Dimension::NFDimension>, mut dim2: Arc<Dimension::NFDimension>) -> Result<(Arc<Dimension::NFDimension>, bool)> {
    let mut compatibleDim: Arc<Dimension::NFDimension>;
    let mut compatible: bool;
    if Dimension::isEqualKnown(dim1.clone(), dim2.clone())? {
        compatibleDim = dim1;
        compatible = true;
    } else {
        if !(Dimension::isKnown(dim1.clone(), false)) {
            compatibleDim = dim2;
            compatible = true;
        } else if !(Dimension::isKnown(dim2.clone(), false)) {
            compatibleDim = dim1;
            compatible = true;
        } else if Dimension::isResizable(dim1.clone()) && Dimension::isResizable(dim2.clone()) {
            compatibleDim = dim1;
            compatible = true;
        } else {
            compatibleDim = dim1;
            compatible = false;
        }
    }
    Ok((compatibleDim, compatible))
}

pub(crate) fn matchTupleTypes(mut tupleType1: Arc<Type::NFType>, mut tupleType2: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = tupleType1.clone();
    let mut matchKind: MatchKind = MatchKind::EXACT.clone();
    let mut tyl1: Arc<metamodelica::List<Arc<Type::NFType>>>;
    let mut tyl2: Arc<metamodelica::List<Arc<Type::NFType>>>;
    let mut ty1: Arc<Type::NFType>;
    let __pa0 = ::match_deref::match_deref! { match &(tupleType1) {
        Deref @ Type::TUPLE { types: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tyl1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(tupleType2) {
        Deref @ Type::TUPLE { types: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tyl2 = __pa1.clone();
    if (tyl1.clone().len() as i32) < (tyl2.clone().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    for mut ty2 in &*tyl2 {
        let mut ty2 = ty2.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tyl1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty1 = __pa2.clone();
        tyl1 = __pa3.clone();
        if Type::isUnknown(ty2.clone()) {
            continue;
        }
        (_, _, matchKind) = matchTypes(ty1.clone(), ty2.clone(), expression.clone(), options)?;
        if matchKind != MatchKind::EXACT.clone() {
            break;
        }
    }
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchBoxedExpressions(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    e1 = Expression::unbox(exp1.clone());
    e2 = Expression::unbox(exp2.clone());
    (e1, e2, compatibleType, matchKind) = matchExpressions(e1, Type::unbox(type1), e2, Type::unbox(type2), options)?;
    if isCastMatch(matchKind) {
        exp1 = Expression::r#box(e1);
        exp2 = Expression::r#box(e2);
    }
    compatibleType = Type::r#box(compatibleType);
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub(crate) fn matchConditionalArrayExp(mut condExp: Arc<Expression::NFExpression>, mut condType: Arc<Type::NFType>, mut otherExp: Arc<Expression::NFExpression>, mut otherType: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut condExp: Arc<Expression::NFExpression> = condExp;
    let mut otherExp: Arc<Expression::NFExpression> = otherExp;
    let mut compatibleType: Arc<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut true_ty: Arc<Type::NFType>;
    let mut false_ty: Arc<Type::NFType>;
    let mut cond_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp_ty1: Arc<Type::NFType>;
    let mut comp_ty2: Arc<Type::NFType>;
    let mut e1_1: Arc<Expression::NFExpression>;
    let mut e2_1: Arc<Expression::NFExpression>;
    let mut e1_2: Arc<Expression::NFExpression>;
    let mut e2_2: Arc<Expression::NFExpression>;
    let mut branch: Type::Branch;
    let mut mk1: MatchKind;
    let mut mk2: MatchKind;
    let mut compat1: bool;
    let mut compat2: bool;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa0, falseType: __pa1, matchedBranch: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    true_ty = __pa0.clone();
    false_ty = __pa1.clone();
    branch = __pa2.clone();
    if branch == Type::Branch::NONE.clone() {
        (e1_1, e2_1, comp_ty1, mk1) = matchExpressions(condExp.clone(), true_ty, otherExp.clone(), otherType.clone(), options)?;
        (e1_2, e2_2, comp_ty2, mk2) = matchExpressions(condExp.clone(), false_ty, otherExp.clone(), otherType, options)?;
        compat1 = isCompatibleMatch(mk1);
        compat2 = isCompatibleMatch(mk2);
        (compatibleType, otherExp, matchKind) = (match (isCompatibleMatch(mk1), isCompatibleMatch(mk2)) {
        (true, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2, matchedBranch: Type::Branch::NONE.clone() });
            condExp = Expression::typeCast(condExp, cond_ty)?;
            (comp_ty1, otherExp, mk1)
        },
        (true, _) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2, matchedBranch: Type::Branch::TRUE.clone() });
            condExp = Expression::typeCast(e1_1, cond_ty)?;
            (comp_ty1, e2_1, mk1)
        },
        (_, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1, falseType: comp_ty2.clone(), matchedBranch: Type::Branch::FALSE.clone() });
            condExp = Expression::typeCast(e1_2, cond_ty)?;
            (comp_ty2, e2_2, mk2)
        },
        _ => (condType, condExp.clone(), mk1),
    });
    } else {
        if branch == Type::Branch::TRUE.clone() {
            (condExp, otherExp, compatibleType, matchKind) = matchExpressions(condExp, true_ty, otherExp, otherType, options)?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: compatibleType.clone(), falseType: false_ty, matchedBranch: branch });
        } else {
            (condExp, otherExp, compatibleType, matchKind) = matchExpressions(condExp, false_ty, otherExp, otherType, options)?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty, falseType: compatibleType.clone(), matchedBranch: branch });
        }
        if isCompatibleMatch(matchKind) {
            condExp = Expression::typeCast(condExp, cond_ty)?;
        }
    }
    Ok((condExp, otherExp, compatibleType, matchKind))
}

pub(crate) fn matchConditionalArrayTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut actual_true_ty: Arc<Type::NFType>;
    let mut actual_false_ty: Arc<Type::NFType>;
    let mut expected_true_ty: Arc<Type::NFType>;
    let mut expected_false_ty: Arc<Type::NFType>;
    let mut true_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut false_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut true_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut false_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa0, falseType: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    actual_true_ty = __pa0.clone();
    actual_false_ty = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(expectedType) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa2, falseType: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expected_true_ty = __pa2.clone();
    expected_false_ty = __pa3.clone();
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { .. } => {
            (true_exp, true_ty, matchKind) = matchTypes(actual_true_ty, expected_true_ty, var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), options)?;
            if !(isCompatibleMatch(matchKind)) {
                compatibleType = actualType;
                return Ok((exp.clone(), compatibleType.clone(), matchKind.clone()));
            }
            (false_exp, false_ty, matchKind) = matchTypes(actual_false_ty, expected_false_ty, var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), options)?;
            if !(isCompatibleMatch(matchKind)) {
                compatibleType = actualType;
                return Ok((exp.clone(), compatibleType.clone(), matchKind.clone()));
            }
            compatibleType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty, falseType: false_ty, matchedBranch: Type::Branch::NONE.clone() });
            exp = Arc::new(Expression::NFExpression::IF { ty: compatibleType.clone(), condition: var_field!((*exp).condition, Expression::NFExpression::IF).clone(), trueBranch: true_exp, falseBranch: false_exp });
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((exp, compatibleType, matchKind))
}

pub(crate) fn matchConditionalArrayTypes_cast(mut condType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut compatibleType: Arc<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut true_ty: Arc<Type::NFType>;
    let mut false_ty: Arc<Type::NFType>;
    let mut cond_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp_ty1: Arc<Type::NFType>;
    let mut comp_ty2: Arc<Type::NFType>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut branch: Type::Branch;
    let mut mk1: MatchKind;
    let mut mk2: MatchKind;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa0, falseType: __pa1, matchedBranch: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    true_ty = __pa0.clone();
    false_ty = __pa1.clone();
    branch = __pa2.clone();
    if branch == Type::Branch::NONE.clone() {
        (e1, comp_ty1, mk1) = matchTypes(true_ty.clone(), expectedType.clone(), exp.clone(), options)?;
        (e2, comp_ty2, mk2) = matchTypes(false_ty.clone(), expectedType, exp.clone(), options)?;
        (compatibleType, matchKind) = (match (isCompatibleMatch(mk1), isCompatibleMatch(mk2)) {
        (true, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2, matchedBranch: Type::Branch::NONE.clone() });
            exp = Expression::typeCast(exp, cond_ty)?;
            (comp_ty1, mk1)
        },
        (true, _) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: false_ty, matchedBranch: Type::Branch::TRUE.clone() });
            exp = Expression::typeCast(e1, cond_ty)?;
            (comp_ty1, mk1)
        },
        (_, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty, falseType: comp_ty2.clone(), matchedBranch: Type::Branch::FALSE.clone() });
            exp = Expression::typeCast(e2, cond_ty)?;
            (comp_ty2, mk2)
        },
        _ => (condType, mk1),
    });
    } else {
        if branch == Type::Branch::TRUE.clone() {
            (exp, compatibleType, matchKind) = matchTypes(true_ty, expectedType, exp, options)?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: compatibleType.clone(), falseType: false_ty, matchedBranch: branch });
        } else {
            (exp, compatibleType, matchKind) = matchTypes(false_ty, expectedType, exp, options)?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty, falseType: compatibleType.clone(), matchedBranch: branch });
        }
        if isCompatibleMatch(matchKind) {
            exp = Expression::typeCast(exp, cond_ty)?;
        }
    }
    Ok((exp, compatibleType, matchKind))
}

pub(crate) fn matchTypes_cast(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut before: Arc<Expression::NFExpression> = expression.clone();
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((actualType.clone(), expectedType.clone())) {
        (Deref @ Type::INTEGER, Deref @ Type::REAL) => {
            expression = Expression::typeCast(expression, expectedType.clone())?;
            (expectedType, MatchKind::CAST.clone())
        },
        (Deref @ Type::ENUMERATION { .. }, Deref @ Type::INTEGER) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdEnumerationAsIntegers")).clone())?) => {
            expression = Expression::typeCast(expression, expectedType.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing usage of enumeration expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" as Integer: ")); __mm_s.push_str(&*Expression::toString(expression.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use Integer(")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(") instead!")); ArcStr::from(__mm_s) }).clone())?;
            (expectedType, MatchKind::CAST.clone())
        },
        (Deref @ Type::INTEGER, Deref @ Type::ENUMERATION { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdIntegersAsEnumeration")).clone())?) => {
            expression = Expression::typeCast(expression, expectedType.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing usage of Integer expression: ")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(" as enumeration: ")); __mm_s.push_str(&*Expression::toString(expression.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use the actual enumeration instead!")); ArcStr::from(__mm_s) }).clone())?;
            (expectedType, MatchKind::CAST.clone())
        },
        (Deref @ Type::TUPLE { types: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, _) => {
            (expression, compatibleType, matchKind) = matchTypes(listHead(var_field!((*actualType).types, Type::NFType::TUPLE).clone())?, expectedType, expression, options)?;
            if isCompatibleMatch(matchKind) {
                expression = (::match_deref::match_deref! { match &(expression.clone()) {
        Deref @ Expression::TUPLE { .. } => listHead(var_field!((*expression).elements, Expression::NFExpression::TUPLE).clone())?,
        _ => Arc::new(Expression::NFExpression::TUPLE_ELEMENT { tupleExp: expression.clone(), index: 1, ty: Type::setArrayElementType(Expression::typeOf(expression), compatibleType.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                matchKind = MatchKind::CAST.clone();
            }
            (compatibleType, matchKind)
        },
        (Deref @ Type::UNKNOWN, _) => (expectedType, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::UNKNOWN_ACTUAL.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (_, Deref @ Type::UNKNOWN) => (actualType, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::UNKNOWN_EXPECTED.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (Deref @ Type::METABOXED { .. }, _) => {
            expression = Expression::unbox(expression);
            (expression, compatibleType, matchKind) = matchTypes(var_field!((*actualType).ty, Type::NFType::METABOXED).clone(), expectedType, expression, options)?;
            (compatibleType, if (isCompatibleMatch(matchKind)) {MatchKind::CAST.clone()} else {matchKind})
        },
        (_, Deref @ Type::METABOXED { .. }) => {
            (expression, compatibleType, matchKind) = matchTypes(actualType, var_field!((*expectedType).ty, Type::NFType::METABOXED).clone(), expression, options)?;
            expression = Expression::r#box(expression);
            compatibleType = Type::r#box(compatibleType);
            (compatibleType, if (isCompatibleMatch(matchKind)) {MatchKind::CAST.clone()} else {matchKind})
        },
        (_, Deref @ Type::POLYMORPHIC { .. }) => {
            (expression, compatibleType, matchKind) = matchPolymorphic((var_field!((*expectedType).name, Type::NFType::POLYMORPHIC).clone()).clone(), actualType, expression)?;
            (compatibleType, matchKind)
        },
        (Deref @ Type::POLYMORPHIC { .. }, _) => (expectedType, MatchKind::GENERIC.clone()),
        (_, Deref @ Type::ANY) => (expectedType, MatchKind::EXACT.clone()),
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => {
            (expression, compatibleType, matchKind) = matchConditionalArrayTypes_cast(actualType, expectedType, expression, options)?;
            (compatibleType, matchKind)
        },
        _ => (crate::NFType::interned_UNKNOWN(), MatchKind::NOT_COMPATIBLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchPolymorphic(mut polymorphicName: ArcStr, mut actualType: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut compatibleType: Arc<Type::NFType>;
    let mut matchKind: MatchKind = MatchKind::EXACT;
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &(polymorphicName) {
        Deref @ "__Any" => (actualType, MatchKind::GENERIC.clone()),
        Deref @ "__Scalar" => {
            matchKind = if (Type::isScalar(actualType.clone())) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__Array" => {
            matchKind = if (Type::isArray(actualType.clone())) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__Connector" => {
            matchKind = if (Type::isScalar(actualType.clone()) && Expression::isConnector(exp.clone())?) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__ComponentExpression" => {
            matchKind = if (Type::isScalar(actualType.clone()) && Expression::isComponentExpression(exp.clone())?) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__Block" => {
            matchKind = if (Type::isComplex(actualType.clone())) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        _ => {
            exp = Expression::r#box(exp);
            (Arc::new(Type::NFType::METABOXED { ty: actualType }), MatchKind::GENERIC.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, compatibleType, matchKind))
}

pub(crate) fn getRangeType(mut startExp: Arc<Expression::NFExpression>, mut stepExp: Option<Arc<Expression::NFExpression>>, mut stopExp: Arc<Expression::NFExpression>, mut rangeElemType: Arc<Type::NFType>, mut info: SourceInfo) -> Result<Arc<Type::NFType>> {
    let mut rangeType: Arc<Type::NFType>;
    let mut dim: Arc<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &(rangeElemType.clone()) {
        Deref @ Type::INTEGER => getRangeTypeInt(startExp, stepExp, stopExp, info)?,
        Deref @ Type::REAL => getRangeTypeReal(startExp, stepExp, stopExp, info)?,
        Deref @ Type::BOOLEAN => {
            if isSome(stepExp) {
                Error::addSourceMessageAndFail(Error::RANGE_INVALID_STEP.clone(), list![(Type::toString(rangeElemType.clone())?).clone()], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            getRangeTypeBool(startExp, stopExp)?
        },
        Deref @ Type::ENUMERATION { .. } => {
            if isSome(stepExp) {
                Error::addSourceMessageAndFail(Error::RANGE_INVALID_STEP.clone(), list![(Type::toString(rangeElemType.clone())?).clone()], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            getRangeTypeEnum(startExp, stopExp)?
        },
        _ => {
            Error::addSourceMessage(Error::RANGE_INVALID_TYPE.clone(), list![(Type::toString(rangeElemType.clone())?).clone()], info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    rangeType = Arc::new(Type::NFType::ARRAY { elementType: rangeElemType, dimensions: list![dim] });
    Ok(rangeType)
}

pub(crate) fn getRangeTypeInt(mut startExp: Arc<Expression::NFExpression>, mut stepExp: Option<Arc<Expression::NFExpression>>, mut stopExp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone(), stopExp.clone())) {
        (Deref @ Expression::INTEGER { .. }, None, Deref @ Expression::INTEGER { .. }) => {
            Dimension::fromInteger(std::cmp::max(var_field!((*stopExp).value, Expression::NFExpression::INTEGER).clone() - var_field!((*startExp).value, Expression::NFExpression::INTEGER).clone() + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::INTEGER { .. }, Some(Deref @ Expression::INTEGER { value: step }), Deref @ Expression::INTEGER { .. }) => {
            if step.clone() == 0 {
                Error::addSourceMessageAndFail(Error::RANGE_TOO_SMALL_STEP.clone(), list![ArcStr::from(::std::format!("{}", step.clone()))], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            Dimension::fromInteger(std::cmp::max(intDiv(var_field!((*stopExp).value, Expression::NFExpression::INTEGER).clone() - var_field!((*startExp).value, Expression::NFExpression::INTEGER).clone(), step.clone()) + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::INTEGER { value: 1 }, None, _) => {
            let mut dim_exp: Arc<Expression::NFExpression>;
            dim_exp = SimplifyExp::simplify(stopExp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), Expression::variability(dim_exp.clone())?)?
        },
        (_, None, _) if (Expression::isEqual(startExp.clone(), stopExp.clone())?) => {
            Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut step_exp: Arc<Expression::NFExpression>;
            let mut dim_exp: Arc<Expression::NFExpression>;
            let mut var: Variability;
            let mut pur: Purity;
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: stopExp.clone(), operator: Operator::makeSub(crate::NFType::interned_INTEGER()), exp2: startExp.clone() });
            var = Prefixes::variabilityMax(Expression::variability(stopExp.clone())?, Expression::variability(startExp.clone())?);
            pur = Prefixes::purityMin(Expression::purity(stopExp.clone())?, Expression::purity(startExp.clone())?);
            if isSome(stepExp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(stepExp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                step_exp = __pa0.clone();
                var = Prefixes::variabilityMax(var.clone(), Expression::variability(step_exp.clone())?);
                pur = Prefixes::purityMin(pur.clone(), Expression::purity(step_exp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::DIV_INT().clone(), list![dim_exp.clone(), step_exp.clone()], var.clone(), pur.clone(), NFBuiltinFuncs::DIV_INT().returnType.clone()) });
            }
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
            dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_INT().clone(), list![dim_exp.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 })], var.clone(), pur.clone(), NFBuiltinFuncs::MAX_INT().returnType.clone()) });
            dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), var.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn getRangeTypeReal(mut startExp: Arc<Expression::NFExpression>, mut stepExp: Option<Arc<Expression::NFExpression>>, mut stopExp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone(), stopExp.clone())) {
        (Deref @ Expression::REAL { .. }, None, Deref @ Expression::REAL { .. }) => {
            Dimension::fromInteger(Util::realRangeSize(var_field!((*startExp).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat(1.0_f64), var_field!((*stopExp).value, Expression::NFExpression::REAL).clone()), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::REAL { value: start }, Some(Deref @ Expression::REAL { value: step }), Deref @ Expression::REAL { .. }) => {
            if start.clone() == start.clone() + step.clone() {
                Error::addSourceMessageAndFail(Error::RANGE_TOO_SMALL_STEP.clone(), list![ArcStr::from(::std::format!("{}", step.clone()))], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            Dimension::fromInteger(Util::realRangeSize(var_field!((*startExp).value, Expression::NFExpression::REAL).clone(), step.clone(), var_field!((*stopExp).value, Expression::NFExpression::REAL).clone()), Prefixes::Variability::CONSTANT.clone())
        },
        (_, None, _) if (Expression::isEqual(startExp.clone(), stopExp.clone())?) => {
            Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut dim_exp: Arc<Expression::NFExpression>;
            let mut step_exp: Arc<Expression::NFExpression>;
            let mut var: Variability;
            let mut pur: Purity;
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: stopExp.clone(), operator: Operator::makeSub(crate::NFType::interned_REAL()), exp2: startExp.clone() });
            var = Prefixes::variabilityMax(Expression::variability(stopExp.clone())?, Expression::variability(startExp.clone())?);
            pur = Prefixes::purityMin(Expression::purity(stopExp.clone())?, Expression::purity(startExp.clone())?);
            if isSome(stepExp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(stepExp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                step_exp = __pa0.clone();
                var = Prefixes::variabilityMax(var.clone(), Expression::variability(step_exp.clone())?);
                pur = Prefixes::purityMin(pur.clone(), Expression::purity(step_exp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeDiv(crate::NFType::interned_REAL()), exp2: step_exp.clone() });
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(crate::NFType::interned_REAL()), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(5e-15_f64) }) });
            }
            dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FLOOR().clone(), list![dim_exp.clone()], var.clone(), pur.clone(), NFBuiltinFuncs::FLOOR().returnType.clone()) });
            dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INTEGER_REAL().clone(), list![dim_exp.clone()], var.clone(), pur.clone(), NFBuiltinFuncs::INTEGER_REAL().returnType.clone()) });
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
            dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), var.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn getRangeTypeBool(mut startExp: Arc<Expression::NFExpression>, mut stopExp: Arc<Expression::NFExpression>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stopExp.clone())) {
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => {
            let mut sz: i32;
            sz = if (var_field!((*startExp).value, Expression::NFExpression::BOOLEAN).clone() == var_field!((*stopExp).value, Expression::NFExpression::BOOLEAN).clone()) {1} else if (var_field!((*startExp).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*stopExp).value, Expression::NFExpression::BOOLEAN).clone()) {2} else {0};
            Dimension::fromInteger(sz.clone(), Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut dim_exp: Arc<Expression::NFExpression>;
            let mut var: Variability;
            if Expression::isEqual(startExp.clone(), stopExp.clone())? {
                dim = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            } else {
                var = Prefixes::variabilityMax(Expression::variability(startExp.clone())?, Expression::variability(stopExp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::IF { ty: crate::NFType::interned_INTEGER(), condition: Arc::new(Expression::NFExpression::RELATION { exp1: startExp.clone(), operator: Operator::makeEqual(crate::NFType::interned_BOOLEAN()), exp2: stopExp.clone(), index: -1 }), trueBranch: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), falseBranch: Arc::new(Expression::NFExpression::IF { ty: crate::NFType::interned_INTEGER(), condition: Arc::new(Expression::NFExpression::RELATION { exp1: startExp, operator: Operator::makeLess(crate::NFType::interned_BOOLEAN()), exp2: stopExp, index: -1 }), trueBranch: Arc::new(Expression::NFExpression::INTEGER { value: 2 }), falseBranch: Arc::new(Expression::NFExpression::INTEGER { value: 0 }) }) });
                dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
                dim = Dimension::fromExp(dim_exp.clone(), var.clone())?;
            }
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn getRangeTypeEnum(mut startExp: Arc<Expression::NFExpression>, mut stopExp: Arc<Expression::NFExpression>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stopExp.clone())) {
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => {
            Dimension::fromInteger(std::cmp::max(var_field!((*stopExp).index, Expression::NFExpression::ENUM_LITERAL).clone() - var_field!((*startExp).index, Expression::NFExpression::ENUM_LITERAL).clone() + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::ENUM_LITERAL { index: 1, .. }, _) => {
            Dimension::fromExp(stopExp.clone(), Expression::variability(stopExp)?)?
        },
        _ => {
            let mut dim_exp: Arc<Expression::NFExpression>;
            let mut var: Variability;
            if Expression::isEqual(startExp.clone(), stopExp.clone())? {
                dim = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            } else {
                var = Prefixes::variabilityMax(Expression::variability(startExp.clone())?, Expression::variability(stopExp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: Expression::enumIndexExp(startExp)?, operator: Operator::makeSub(crate::NFType::interned_INTEGER()), exp2: Expression::enumIndexExp(stopExp)? });
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
                dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
                dim = Dimension::fromExp(dim_exp.clone(), var.clone())?;
            }
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn matchBinding(mut binding: Arc<Binding::NFBinding>, mut componentType: Arc<Type::NFType>, mut name: ArcStr, mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { bindingExp: exp, .. } => {
            let mut ty_match: MatchKind;
            let mut ty: Arc<Type::NFType>;
            let mut bind_ty: Arc<Type::NFType>;
            let mut comp_ty: Arc<Type::NFType>;
            let mut exp = (*exp).clone();
            (bind_ty, comp_ty) = elaborateBindingType(exp.clone(), component.clone(), var_field!((*binding).bindingType, Binding::NFBinding::TYPED_BINDING).clone(), componentType)?;
            (exp, ty, ty_match) = matchTypes(bind_ty.clone(), comp_ty.clone(), exp.clone(), ALLOW_UNKNOWN.clone())?;
            if !(isValidAssignmentMatch(ty_match.clone())) {
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = Expression::expandSplitIndices(exp.clone())?);
                printBindingTypeError((name).clone(), binding.clone(), comp_ty.clone(), bind_ty.clone(), component, context)?;
                if !(InstContext::inInstanceAPI(context)) {
                    bail!("fail");
                }
            } else if isCastMatch(ty_match.clone()) {
                binding = Arc::new(Binding::NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty.clone(), variability: var_field!((*binding).variability, Binding::NFBinding::TYPED_BINDING).clone(), purity: var_field!((*binding).purity, Binding::NFBinding::TYPED_BINDING).clone(), eachType: var_field!((*binding).eachType, Binding::NFBinding::TYPED_BINDING).clone(), evalState: var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), isFlattened: var_field!((*binding).isFlattened, Binding::NFBinding::TYPED_BINDING).clone(), source: var_field!((*binding).source, Binding::NFBinding::TYPED_BINDING).clone(), confidence: var_field!((*binding).confidence, Binding::NFBinding::TYPED_BINDING).clone(), info: var_field!((*binding).info, Binding::NFBinding::TYPED_BINDING).clone() });
            }
            ()
        },
        Deref @ Binding::UNBOUND => {
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTypeCheck.matchBinding")); __mm_s.push_str(&*literal!(" got untyped binding ")); __mm_s.push_str(&*Binding::toString(binding.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTypeCheck.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub(crate) fn elaborateBindingType(mut bindingExp: Arc<Expression::NFExpression>, mut component: Arc<InstNode::InstNode>, mut bindingType: Arc<Type::NFType>, mut componentType: Arc<Type::NFType>) -> Result<(Arc<Type::NFType>, Arc<Type::NFType>)> {
    fn isParent(mut parent: Arc<InstNode::InstNode>, mut node: Arc<InstNode::InstNode>) -> bool {
        let mut res: bool;
        let mut n: Arc<InstNode::InstNode> = InstNode::getDerivedNode(node.clone(), true);
        let mut p: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        res = (::match_deref::match_deref! { match &(n.clone()) {
        Deref @ InstNode::COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent: __esc_p }, .. } => {
            p = (*__esc_p).clone();
            InstNode::refEqual(parent.clone(), n) || isParent(parent, p.clone())
        },
        Deref @ InstNode::COMPONENT_NODE { .. } => InstNode::refEqual(parent.clone(), n.clone()) || isParent(parent, var_field!((*n).parent, InstNode::InstNode::COMPONENT_NODE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    let mut bindingType: Arc<Type::NFType> = bindingType;
    let mut componentType: Arc<Type::NFType> = componentType;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(bindingExp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            bindingType = Expression::typeOf(var_field!((*bindingExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone());
            dims = metamodelica::nil();
            for mut s in &*var_field!((*bindingExp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut s = s.clone();
                dims = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            if isParent(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), component.clone()) {
                dims = metamodelica::cons(Type::nthDimension(InstNode::getType(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone())?, var_field!((*s).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone())?, dims.clone());
            }
            dims.clone()
        },
        _ => metamodelica::cons(crate::NFDimension::interned_UNKNOWN(), dims.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            dims = metamodelica::Dangerous::listReverseInPlace(dims.clone());
            componentType = Type::liftArrayLeftList(componentType.clone(), dims.clone());
            ()
        },
        Deref @ Expression::CREF { .. } => {
            bindingType = ComponentRef::getSubscriptedType(ComponentRef::expandSplitSubscripts(var_field!((*bindingExp).cref, Expression::NFExpression::CREF).clone())?, false)?;
            dims = metamodelica::nil();
            for mut s in &*ComponentRef::subscriptsAllFlat(var_field!((*bindingExp).cref, Expression::NFExpression::CREF).clone())? {
                let mut s = s.clone();
                dims = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            if isParent(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), component.clone()) {
                dims = metamodelica::cons(Type::nthDimension(InstNode::getType(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone())?, var_field!((*s).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone())?, dims.clone());
            }
            dims.clone()
        },
        _ => dims.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            dims = metamodelica::Dangerous::listReverseInPlace(dims.clone());
            componentType = Type::liftArrayLeftList(componentType.clone(), dims.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((bindingType, componentType))
}

pub(crate) fn printBindingTypeError(mut name: ArcStr, mut binding: Arc<Binding::NFBinding>, mut componentType: Arc<Type::NFType>, mut bindingType: Arc<Type::NFType>, mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut binding_info: SourceInfo;
    let mut comp_info: SourceInfo;
    let mut mk: MatchKind;
    binding_info = Binding::getInfo(binding.clone());
    comp_info = InstNode::info(component);
    if Type::isScalar(bindingType.clone()) && Type::isArray(componentType.clone()) {
        Error::addMultiSourceMessage(Error::MODIFIER_NON_ARRAY_TYPE_ERROR.clone(), list![(Binding::toString(binding, (literal!("")).clone())?).clone(), (name).clone()], list![binding_info, comp_info])?;
    } else {
        (_, _, mk) = matchTypes(Type::arrayElementType(bindingType.clone()), Type::arrayElementType(componentType.clone()), Arc::new(Expression::NFExpression::EMPTY { ty: bindingType.clone() }), ALLOW_UNKNOWN.clone())?;
        if !(InstContext::inAnnotation(context)) {
            if isValidAssignmentMatch(mk) {
                Error::addMultiSourceMessage(Error::VARIABLE_BINDING_DIMS_MISMATCH.clone(), list![(name).clone(), (Binding::toString(binding, (literal!("")).clone())?).clone(), (Dimension::toStringList(Type::arrayDims(componentType), true)?).clone(), (Dimension::toStringList(Type::arrayDims(bindingType), true)?).clone()], list![binding_info, comp_info])?;
            } else {
                Error::addMultiSourceMessage(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone(), list![(name).clone(), (Binding::toString(binding, (literal!("")).clone())?).clone(), (Type::toString(componentType)?).clone(), (Type::toString(bindingType)?).clone()], list![binding_info, comp_info])?;
            }
        }
    }
    Ok(())
}

pub(crate) fn checkDimensionType(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut info: SourceInfo) -> Result<()> {
    if !(Type::isInteger(ty.clone())?) {
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::BOOLEAN, .. } } => (),
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::ENUMERATION { .. }, .. } } => (),
        _ => {
            Error::addSourceMessage(Error::INVALID_DIMENSION_TYPE.clone(), list![(Expression::toString(exp)?).clone(), (Type::toString(ty)?).clone()], info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub(crate) fn checkReductionType(mut ty: Arc<Type::NFType>, mut name: Arc<Absyn::Path>, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
    let mut err: ArcStr;
    err = ((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } => (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        Deref @ Type::COMPLEX { .. } if (checkSumComplexType(ty.clone(), exp.clone(), info.clone())?) => literal!(""),
        _ => literal!("Integer or Real, or operator record"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "product" } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        _ => literal!("scalar Integer or Real"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "min" } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        Deref @ Type::BOOLEAN => literal!(""),
        Deref @ Type::ENUMERATION { .. } => literal!(""),
        _ => literal!("scalar enumeration, Boolean, Integer, or Real"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "max" } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        Deref @ Type::BOOLEAN => literal!(""),
        Deref @ Type::ENUMERATION { .. } => literal!(""),
        _ => literal!("scalar enumeration, Boolean, Integer, or Real"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    if !(stringEmpty((err.clone()).clone())) {
        Error::addSourceMessageAndFail(Error::INVALID_REDUCTION_TYPE.clone(), list![(Expression::toString(exp)?).clone(), (Type::toString(ty)?).clone(), (AbsynUtil::pathString(name, (literal!(".")).clone(), true, false)?).clone(), (err).clone()], info)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok(())
}

pub(crate) fn checkSumComplexType(mut ty: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<bool> {
    let mut valid: bool = true;
    let mut cls_node: Arc<InstNode::InstNode>;
    let mut cls: Arc<Class::NFClass>;
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls_node = __pa0.clone();
    cls = InstNode::getClass(cls_node)?;
    for mut op in &*list![(literal!("'+'")).clone(), (literal!("'0'")).clone()] {
        let mut op = op.clone();
        if !(Class::hasOperator((op.clone()).clone(), cls.clone())) {
            Error::addSourceMessage(Error::OPERATOR_RECORD_MISSING_OPERATOR.clone(), list![(Type::toString(ty.clone())?).clone(), (Expression::toString(exp.clone())?).clone(), (literal!("sum")).clone(), (op.clone()).clone()], info.clone())?;
            valid = false;
        }
    }
    Ok(valid)
}

pub(crate) fn matchIfBranches(mut trueBranch: Arc<Expression::NFExpression>, mut trueType: Arc<Type::NFType>, mut falseBranch: Arc<Expression::NFExpression>, mut falseType: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut trueBranch: Arc<Expression::NFExpression> = trueBranch;
    let mut falseBranch: Arc<Expression::NFExpression> = falseBranch;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((trueType.clone(), falseType.clone())) {
        (Deref @ Type::ARRAY { .. }, Deref @ Type::ARRAY { .. }) => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch, var_field!((*trueType).elementType, Type::NFType::ARRAY).clone(), falseBranch, var_field!((*falseType).elementType, Type::NFType::ARRAY).clone(), options)?;
            if isIncompatibleMatch(matchKind) {
                return Ok((trueBranch.clone(), falseBranch.clone(), compatibleType.clone(), matchKind.clone()));
            }
            (compatibleType, matchKind) = matchArrayDims(var_field!((*trueType).dimensions, Type::NFType::ARRAY).clone(), var_field!((*falseType).dimensions, Type::NFType::ARRAY).clone(), compatibleType, matchKind, options)?;
            if isIncompatibleMatch(matchKind) && (var_field!((*trueType).dimensions, Type::NFType::ARRAY).clone().len() as i32) == (var_field!((*falseType).dimensions, Type::NFType::ARRAY).clone().len() as i32) {
                compatibleType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Type::copyElementType(trueType.clone(), compatibleType.clone()), falseType: Type::copyElementType(falseType.clone(), compatibleType), matchedBranch: Type::Branch::NONE.clone() });
                matchKind = MatchKind::EXACT.clone();
            }
            (compatibleType, matchKind)
        },
        (_, _) if (Type::isConditionalArray(trueType.clone()) || Type::isConditionalArray(falseType.clone())) => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch, Type::arrayElementType(trueType.clone()), falseBranch, Type::arrayElementType(falseType.clone()), options)?;
            if isIncompatibleMatch(matchKind) {
                return Ok((trueBranch.clone(), falseBranch.clone(), compatibleType.clone(), matchKind.clone()));
            }
            compatibleType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Type::copyElementType(trueType.clone(), compatibleType.clone()), falseType: Type::copyElementType(falseType.clone(), compatibleType), matchedBranch: Type::Branch::NONE.clone() });
            (compatibleType, matchKind)
        },
        _ => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch, trueType.clone(), falseBranch, falseType.clone(), options)?;
            (compatibleType, matchKind)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((trueBranch, falseBranch, compatibleType, matchKind))
}

