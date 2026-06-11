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

use crate::NFEnvExtends;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeEnv;
use crate::NFSCodeFlattenImports;
use crate::NFSCodeFlattenRedeclare;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type Frame = Arc<NFSCodeEnv::Frame>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum RedeclareReplaceStrategy {
    INSERT_REDECLARES,
    IGNORE_REDECLARES,
}
impl metamodelica::gc::MMTrace for RedeclareReplaceStrategy {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            RedeclareReplaceStrategy::INSERT_REDECLARES => Ok(()),
            RedeclareReplaceStrategy::IGNORE_REDECLARES => Ok(()),
        }
    }
}
pub(crate) use self::RedeclareReplaceStrategy::{INSERT_REDECLARES,IGNORE_REDECLARES};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum LookupStrategy {
    NO_BUILTIN_TYPES,
    LOOKUP_ANY,
}
impl metamodelica::gc::MMTrace for LookupStrategy {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            LookupStrategy::NO_BUILTIN_TYPES => Ok(()),
            LookupStrategy::LOOKUP_ANY => Ok(()),
        }
    }
}
pub(crate) use self::LookupStrategy::{NO_BUILTIN_TYPES,LOOKUP_ANY};

// Default parts of the declarations for builtin elements and types.
pub(crate) static BUILTIN_PREFIXES: std::sync::LazyLock<Arc<SCode::Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE() }) });

pub(crate) static BUILTIN_ATTRIBUTES: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub(crate) static BUILTIN_CONST_ATTRIBUTES: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| { SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub(crate) static BUILTIN_EMPTY_CLASS: std::sync::LazyLock<Arc<SCode::ClassDef>> = std::sync::LazyLock::new(|| { Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }) });

// Metatypes used to define the builtin types.
pub(crate) static BUILTIN_REALTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$RealType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_INTEGERTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$IntegerType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_BOOLEANTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$BooleanType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_STRINGTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$StringType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ENUMTYPE: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::CLASS { name: (literal!("$EnumType")).clone(), prefixes: BUILTIN_PREFIXES.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION, classDef: BUILTIN_EMPTY_CLASS.clone(), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_REALTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REALTYPE.clone(), isUsed: None }) });

pub(crate) static BUILTIN_INTEGERTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGERTYPE.clone(), isUsed: None }) });

pub(crate) static BUILTIN_BOOLEANTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_BOOLEANTYPE.clone(), isUsed: None }) });

pub(crate) static BUILTIN_STRINGTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STRINGTYPE.clone(), isUsed: None }) });

pub(crate) static BUILTIN_ENUMTYPE_ITEM: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUMTYPE.clone(), isUsed: None }) });

pub(crate) static BUILTIN_REALTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$RealType")).clone() }), arrayDim: None }) });

pub(crate) static BUILTIN_INTEGERTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$IntegerType")).clone() }), arrayDim: None }) });

pub(crate) static BUILTIN_BOOLEANTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$BooleanType")).clone() }), arrayDim: None }) });

pub(crate) static BUILTIN_STRINGTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$StringType")).clone() }), arrayDim: None }) });

pub(crate) static BUILTIN_ENUMTYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$EnumType")).clone() }), arrayDim: None }) });

pub(crate) static BUILTIN_STATESELECT_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("StateSelect")).clone() }), arrayDim: None }) });

// Parts of the builtin types.
// Generic elements:
pub(crate) static BUILTIN_ATTR_QUANTITY: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("quantity")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ATTR_UNIT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("unit")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ATTR_DISPLAYUNIT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("displayUnit")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ATTR_FIXED: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("fixed")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_BOOLEANTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ATTR_STATESELECT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("stateSelect")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STATESELECT_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Real-specific elements:
pub(crate) static BUILTIN_REAL_MIN: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_REAL_MAX: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_REAL_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_REAL_NOMINAL: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("nominal")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_REALTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Integer-specific elements:
pub(crate) static BUILTIN_INTEGER_MIN: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_INTEGER_MAX: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_INTEGER_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Boolean-specific elements:
pub(crate) static BUILTIN_BOOLEAN_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_BOOLEANTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// String-specific elements:
pub(crate) static BUILTIN_STRING_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// StateSelect-specific elements:
pub(crate) static BUILTIN_ENUM_MIN: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("min")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ENUM_MAX: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("max")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_ENUM_START: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("start")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_STATESELECT_NEVER: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("never")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_STATESELECT_AVOID: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("avoid")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_STATESELECT_DEFAULT: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("default")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_STATESELECT_PREFER: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("prefer")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

pub(crate) static BUILTIN_STATESELECT_ALWAYS: std::sync::LazyLock<Arc<SCode::Element>> = std::sync::LazyLock::new(|| { Arc::new(SCode::Element::COMPONENT { name: (literal!("always")).clone(), prefixes: BUILTIN_PREFIXES.clone(), attributes: BUILTIN_CONST_ATTRIBUTES.clone(), typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }) });

// Environments for the builtin types:
pub(crate) static BUILTIN_REAL_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("Real")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("nominal")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_NOMINAL.clone(), isUsed: None }), height: 3, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("max")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_MAX.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("displayUnit")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_DISPLAYUNIT.clone(), isUsed: None }) }), right: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY() }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("min")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_MIN.clone(), isUsed: None }) }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_REAL_START.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("stateSelect")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_STATESELECT.clone(), isUsed: None }), height: 1, left: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY(), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("unit")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_UNIT.clone(), isUsed: None }) }) }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub(crate) static BUILTIN_INTEGER_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("Integer")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("max")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGER_MAX.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("min")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGER_MIN.clone(), isUsed: None }) }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_INTEGER_START.clone(), isUsed: None }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub(crate) static BUILTIN_BOOLEAN_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("Boolean")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_BOOLEAN_START.clone(), isUsed: None }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub(crate) static BUILTIN_STRING_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("String")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 2, left: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY(), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STRING_START.clone(), isUsed: None }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

pub(crate) static BUILTIN_STATESELECT_ENV: std::sync::LazyLock<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>> = std::sync::LazyLock::new(|| { list![Arc::new(NFSCodeEnv::Frame { name: Some((literal!("StateSelect")).clone()), frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE, clsAndVars: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("max")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUM_MAX.clone(), isUsed: None }), height: 3, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("default")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_DEFAULT.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("avoid")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_AVOID.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("always")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_ALWAYS.clone(), isUsed: None }) }), right: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY() }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("fixed")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_FIXED.clone(), isUsed: None }) }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("never")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_NEVER.clone(), isUsed: None }), height: 2, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("min")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUM_MIN.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::NODE { key: (literal!("quantity")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ATTR_QUANTITY.clone(), isUsed: None }), height: 1, left: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("prefer")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_STATESELECT_PREFER.clone(), isUsed: None }) }), right: Arc::new(NFSCodeEnv::EnvTree::Tree::LEAF { key: (literal!("start")).clone(), value: Arc::new(NFSCodeEnv::Item::VAR { var: BUILTIN_ENUM_START.clone(), isUsed: None }) }) }) }) }), extendsTable: Arc::new(NFSCodeEnv::ExtendsTable { baseClasses: metamodelica::nil(), redeclaredElements: metamodelica::nil(), classExtendsInfo: None }), importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: metamodelica::nil(), unqualifiedImports: metamodelica::nil() }, isUsed: None })] });

// The builtin types:
pub(crate) static BUILTIN_REAL: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Real")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_REAL_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) static BUILTIN_INTEGER: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Integer")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_INTEGER_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) static BUILTIN_BOOLEAN: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Boolean")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_BOOLEAN_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) static BUILTIN_STRING: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("String")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_STRING_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) static BUILTIN_STATESELECT: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("StateSelect")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::ENUMERATION { enumLst: list![Arc::new(SCode::Enum { literal: (literal!("never")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("avoid")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("default")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("prefer")).clone(), comment: SCode::noComment.clone() }), Arc::new(SCode::Enum { literal: (literal!("always")).clone(), comment: SCode::noComment.clone() })] }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: BUILTIN_STATESELECT_ENV.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) static BUILTIN_EXTERNALOBJECT: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("ExternalObject")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) static BUILTIN_CLOCK: std::sync::LazyLock<Arc<NFSCodeEnv::Item>> = std::sync::LazyLock::new(|| { Arc::new(NFSCodeEnv::Item::CLASS { cls: Arc::new(SCode::Element::CLASS { name: (literal!("Clock")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() }), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }) });

pub(crate) fn lookupSimpleName(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupSimpleName2((inName).clone(), inEnv, metamodelica::nil())?) {
        (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outItem = __pa0.clone();
    outPath = __pa1.clone();
    outEnv = __pa2.clone();
    Ok((outItem, outPath, outEnv))
}

fn lookupSimpleName2(mut inName: ArcStr, mut inEnv: Env, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    (opt_item, opt_path, opt_env) = lookupInLocalScope((inName.clone()).clone(), inEnv.clone(), inVisitedScopes.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(scope_name), frameType: frame_type, .. }, tail: rest_env } => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    frameNotEncapsulated(frame_type.clone())?;
                    (opt_item, opt_path, opt_env) = lookupSimpleName2((inName.clone()).clone(), rest_env.clone(), metamodelica::cons((scope_name.clone()).clone(), inVisitedScopes.clone()))?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE { .. }, .. }, tail: rest_env } => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    let mut rest_env = (*rest_env).clone();
                    rest_env = NFSCodeEnv::getEnvTopScope(rest_env.clone())?;
                    (opt_item, opt_path, opt_env) = lookupSimpleName2((inName.clone()).clone(), rest_env.clone(), metamodelica::nil())?;
                    checkBuiltinItem(opt_item.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn frameNotEncapsulated(mut frameType: FrameType) -> Result<()> {
    let () = (match frameType {
        NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE { .. } => bail!("fail"),
        _ => (),
    });
    Ok(())
}

fn checkBuiltinItem(mut inItem: Option<Arc<NFSCodeEnv::Item>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inItem) {
        Some(Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BUILTIN { .. }, .. }) => (),
        None => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn lookupInLocalScope(mut inName: ArcStr, mut inEnv: Env, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: Env;
                    let mut item: Item;
                    (item, env) = lookupInClass((inName.clone()).clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() })), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    (opt_item, opt_path, opt_env) = lookupInBaseClasses((inName.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES, inVisitedScopes.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: imps, .. }, .. }, tail: _ } => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports((inName.clone()).clone(), imps.clone(), inEnv.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { hidden: false, unqualifiedImports: imps, .. }, .. }, tail: _ } => {
                    let mut env: Env;
                    let mut item: Item;
                    let mut path: Arc<Absyn::Path>;
                    (item, path, env) = lookupInUnqualifiedImports((inName.clone()).clone(), imps.clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: rest_env } => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    (opt_item, opt_path, opt_env) = lookupInLocalScope((inName.clone()).clone(), rest_env.clone(), inVisitedScopes.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInClass(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    let mut tree: Arc<NFSCodeEnv::EnvTree::Tree>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tree = __pa0.clone();
    outItem = NFSCodeEnv::EnvTree::get(tree, (inName).clone())?;
    (outItem, outEnv) = resolveAlias(outItem, inEnv)?;
    Ok((outItem, outEnv))
}

pub(crate) fn resolveAlias(mut inItem: Item, mut inEnv: Env) -> Result<(Item, Env)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
        (Deref @ NFSCodeEnv::Item::ALIAS { name, path: None, .. }, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ }) => {
            let mut item: Item;
            let mut env: Env;
            item = NFSCodeEnv::EnvTree::get(tree.clone(), (name.clone()).clone())?;
            { (inItem, inEnv) = (item.clone(), inEnv); continue '__tco; }
        },
        (Deref @ NFSCodeEnv::Item::ALIAS { name, path: Some(path), .. }, _) => {
            let mut item: Item;
            let mut env: Env;
            let mut tree: Arc<NFSCodeEnv::EnvTree::Tree>;
            env = NFSCodeEnv::getEnvTopScope(inEnv)?;
            env = NFSCodeEnv::enterScopePath(env.clone(), path.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            tree = __pa0.clone();
            item = NFSCodeEnv::EnvTree::get(tree.clone(), (name.clone()).clone())?;
            { (inItem, inEnv) = (item.clone(), env.clone()); continue '__tco; }
        },
        _ => {
            return Ok((inItem, inEnv))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lookupInBaseClasses(mut inName: ArcStr, mut inEnv: Env, mut inReplaceRedeclares: RedeclareReplaceStrategy, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    let mut env: Env;
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    env = NFSCodeEnv::removeExtendsFromLocalScope(inEnv.clone())?;
    env = NFSCodeEnv::setImportTableHidden(env, false)?;
    (outItem, outPath, outEnv) = lookupInBaseClasses2((inName).clone(), bcl, env, inEnv, inReplaceRedeclares, inVisitedScopes)?;
    Ok((outItem, outPath, outEnv))
}

fn lookupInBaseClasses2(mut inName: ArcStr, mut inBaseClasses: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>, mut inEnv: Env, mut inEnvWithExtends: Env, mut inReplaceRedeclares: RedeclareReplaceStrategy, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = inBaseClasses;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: ext, tail: _ } => {
                    let mut item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut path: Option<Arc<Absyn::Path>>;
                    let mut env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    (item, path, env) = lookupInBaseClasses3((inName.clone()).clone(), ext.clone(), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_ext } => {
                    let mut item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut path: Option<Arc<Absyn::Path>>;
                    let mut env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    (item, path, env) = lookupInBaseClasses2((inName.clone()).clone(), rest_ext.clone(), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares.clone(), inVisitedScopes.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInBaseClasses3(mut inName: ArcStr, mut inBaseClass: Extends, mut inEnv: Env, mut inEnvWithExtends: Env, mut inReplaceRedeclares: RedeclareReplaceStrategy, mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &(inBaseClass) {
        Deref @ NFSCodeEnv::Extends { baseClass: bc @ Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }, info, .. } => {
            NFEnvExtends::printExtendsError(bc.clone(), inEnvWithExtends, info.clone())?;
            (None, None, None)
        },
        Deref @ NFSCodeEnv::Extends { baseClass: bc, redeclareModifiers: redecls, info, .. } => {
            let mut path: Arc<Absyn::Path>;
            let mut item: Item;
            let mut env: Env;
            let mut opt_path: Option<Arc<Absyn::Path>>;
            let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
            let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
            (item, path, env) = lookupBaseClassName(bc.clone(), inEnv.clone(), info.clone())?;
            let true = (checkVisitedScopes(inVisitedScopes, inEnv, path.clone())) else { bail!("pattern mismatch") };
            item = NFSCodeEnv::setImportsInItemHidden(item.clone(), true)?;
            (opt_item, opt_env) = NFSCodeFlattenRedeclare::replaceRedeclares(redecls.clone(), item.clone(), env.clone(), inEnvWithExtends, inReplaceRedeclares);
            (opt_item, opt_path, opt_env) = lookupInBaseClasses4(Arc::new(Absyn::Path::IDENT { name: (inName).clone() }), opt_item.clone(), opt_env.clone())?;
            (opt_item.clone(), opt_path.clone(), opt_env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outItem, outPath, outEnv))
}

fn checkVisitedScopes(mut inVisitedScopes: Arc<metamodelica::List<ArcStr>>, mut inEnv: Env, mut inBaseClass: Arc<Absyn::Path>) -> bool {
    let mut outRes: bool;
    outRes = 'mc: {
        let __mc_input = inVisitedScopes.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut visited_path: Arc<Absyn::Path>;
                    let mut bc_path: Arc<Absyn::Path>;
                    env_path = NFSCodeEnv::getEnvPath(inEnv.clone())?;
                    bc_path = AbsynUtil::removePrefix(env_path.clone(), inBaseClass.clone())?;
                    visited_path = AbsynUtil::stringListPath(inVisitedScopes.clone())?;
                    let true = (AbsynUtil::pathPrefixOf(visited_path.clone(), bc_path.clone())) else { bail!("pattern mismatch") };
                    Ok(false)
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
    outRes
}

fn lookupInBaseClasses4(mut inName: Arc<Absyn::Path>, mut inItem: Option<Arc<NFSCodeEnv::Item>>, mut inEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inItem, inEnv)) {
        (None, None) => {
            (None, None, None)
        },
        (Some(item), Some(env)) => {
            let mut path: Arc<Absyn::Path>;
            let mut item = (*item).clone();
            let mut env = (*env).clone();
            (item, path, env) = lookupNameInItem(inName, item.clone(), env.clone())?;
            (Some(item.clone()), Some(path.clone()), Some(env.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInQualifiedImports(mut inName: ArcStr, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: Env) -> Result<(Option<Arc<NFSCodeEnv::Item>>, Option<Arc<Absyn::Path>>, Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>)> {
    let mut outItem: Option<Arc<NFSCodeEnv::Item>>;
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut outEnv: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps } => {
                    let mut opt_item: Option<Arc<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<Arc<Absyn::Path>>;
                    let mut opt_env: Option<Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>>;
                    let false = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports((inName.clone()).clone(), rest_imps.clone(), inEnv.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, path }, tail: _ } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (item, path, env) = lookupFullyQualified(path.clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ } => {
                    let true = (stringEqual((inName.clone()).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((None, None, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInUnqualifiedImports(mut inName: ArcStr, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ } => {
                    let mut item: Item;
                    let mut path2: Arc<Absyn::Path>;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, path, env) = lookupFullyQualified(path.clone(), inEnv.clone())?;
                    (item, path2, env) = lookupNameInItem(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), item.clone(), env.clone())?;
                    path = joinPaths(path.clone(), path2.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_imps } => {
                    let mut item: Item;
                    let mut path: Arc<Absyn::Path>;
                    let mut env: Env;
                    (item, path, env) = lookupInUnqualifiedImports((inName.clone()).clone(), rest_imps.clone(), inEnv.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupFullyQualified(mut inName: Arc<Absyn::Path>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env;
    let mut env: Env;
    env = NFSCodeEnv::getEnvTopScope(inEnv)?;
    (outItem, outPath, outEnv) = lookupNameInPackage(inName, env)?;
    outPath = AbsynUtil::makeFullyQualified(outPath);
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupNameInPackage(mut inName: Arc<Absyn::Path>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inName, inEnv.clone())) {
        (Deref @ Absyn::Path::IDENT { name }, _) => {
            let mut path: Arc<Absyn::Path>;
            let mut env: Env;
            let mut item: Item;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv, metamodelica::nil())?) {
                (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            item = __pa0.clone();
            path = __pa1.clone();
            env = __pa2.clone();
            env = NFSCodeEnv::setImportTableHidden(env.clone(), false)?;
            (item.clone(), path.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { name, path }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut new_path: Arc<Absyn::Path>;
            let mut env: Env;
            let mut item: Item;
            let mut path = (*path).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv, metamodelica::nil())?) {
                (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            item = __pa0.clone();
            new_path = __pa1.clone();
            env = __pa2.clone();
            env = NFSCodeEnv::setImportTableHidden(env.clone(), false)?;
            (item, path, env) = lookupNameInItem(path.clone(), item.clone(), env.clone())?;
            path = joinPaths(new_path.clone(), path.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupCrefInPackage(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<(Item, Arc<Absyn::ComponentRef>)> {
    let mut outItem: Item;
    let mut outCref: Arc<Absyn::ComponentRef>;
    (outItem, outCref) = 'mc: {
        let __mc_input = inCref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
                    let mut new_path: Arc<Absyn::Path>;
                    let mut cref: Arc<Absyn::ComponentRef>;
                    let mut item: Item;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                        (Some(__pa0), Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    new_path = __pa1.clone();
                    cref = AbsynUtil::pathToCrefWithSubs(new_path.clone(), subs.clone())?;
                    Ok((item.clone(), cref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts: subs, componentRef: cref_rest } => {
                    let mut new_path: Arc<Absyn::Path>;
                    let mut cref: Arc<Absyn::ComponentRef>;
                    let mut item: Item;
                    let mut env: Env;
                    let mut cref_rest = (*cref_rest).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                        (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    new_path = __pa1.clone();
                    env = __pa2.clone();
                    (item, cref_rest) = lookupCrefInItem(cref_rest.clone(), item.clone(), env.clone())?;
                    if '__try3: {
                        ::match_deref::match_deref! { match &(cref_rest.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: _ } => (),
                            _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    cref = AbsynUtil::pathToCrefWithSubs(new_path.clone(), subs.clone())?;
                    cref = AbsynUtil::joinCrefs(cref.clone(), cref_rest.clone())?;
                    Ok((item.clone(), cref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name, componentRef: cref_rest, .. } => {
                    let mut cref: Arc<Absyn::ComponentRef>;
                    let mut item: Item;
                    let mut env: Env;
                    let mut cref_rest = (*cref_rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope((name.clone()).clone(), inEnv.clone(), metamodelica::nil())?) {
                        (Some(__pa0), Some(_), Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    env = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(lookupCrefInItem(cref_rest.clone(), item.clone(), env.clone())?) {
                        (__pa2, __pa3 @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: _ }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa2.clone();
                    cref_rest = __pa3.clone();
                    cref = cref_rest.clone();
                    Ok((item.clone(), cref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outCref))
}

pub(crate) fn lookupNameInItem(mut inName: Arc<Absyn::Path>, mut inItem: Item, mut inEnv: Env) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inItem, inEnv.clone())) {
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { typeSpec: type_spec, modifications: mods, info, .. }, .. }, env) => {
            let mut item: Item;
            let mut path: Arc<Absyn::Path>;
            let mut type_env: Env;
            let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>;
            let mut env = (*env).clone();
            (item, _, type_env) = lookupTypeSpec(type_spec.clone(), env.clone(), info.clone())?;
            redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone())?;
            (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares.clone(), item.clone(), type_env.clone(), inEnv, NFInstPrefix::emptyPrefix().clone())?;
            { (inName, inItem, inEnv) = (inName, item.clone(), type_env.clone()); continue '__tco; }
        },
        (Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: class_env, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut item: Item;
            let mut path: Arc<Absyn::Path>;
            let mut env: Env;
            env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv);
            return Ok(lookupNameInPackage(inName, env.clone())?)
        },
        (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env }, _) => {
            let mut path: Arc<Absyn::Path>;
            let mut item = (*item).clone();
            let mut env = (*env).clone();
            { (inName, inItem, inEnv) = (inName, item.clone(), env.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn lookupCrefInItem(mut inCref: Arc<Absyn::ComponentRef>, mut inItem: Item, mut inEnv: Env) -> Result<(Item, Arc<Absyn::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inItem) {
        Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { typeSpec: type_spec, modifications: mods, info, .. }, .. } => {
            let mut item: Item;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut type_env: Env;
            let mut redeclares: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>;
            (item, _, type_env) = lookupTypeSpec(type_spec.clone(), inEnv.clone(), info.clone())?;
            redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(mods.clone())?;
            (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares.clone(), item.clone(), type_env.clone(), inEnv, NFInstPrefix::emptyPrefix().clone())?;
            { (inCref, inItem, inEnv) = (inCref, item.clone(), type_env.clone()); continue '__tco; }
        },
        Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: class_env, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut item: Item;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut env: Env;
            env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv);
            return Ok(lookupCrefInPackage(inCref, env.clone())?)
        },
        Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env } => {
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut item = (*item).clone();
            { (inCref, inItem, inEnv) = (inCref, item.clone(), env.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn lookupBaseClasses(mut inName: ArcStr, mut inEnv: Env) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    (_, outBaseClasses) = List::fold22(bcl, (std::sync::Arc::new(fnptr!(lookupBaseClasses2, Arc<NFSCodeEnv::Extends>, ArcStr, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Extends>, ArcStr, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), (inName).clone(), inEnv, metamodelica::nil(), metamodelica::nil())?;
    let false = (outBaseClasses.clone().is_empty()) else { bail!("pattern mismatch") };
    outBaseClasses = outBaseClasses.reverse();
    Ok(outBaseClasses)
}

fn lookupBaseClasses2(mut inBaseClass: Extends, mut inName: ArcStr, mut inEnv: Env, mut items: Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, mut bcl: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> (Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) {
    let mut items: Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>> = items;
    let mut bcl: Arc<metamodelica::List<Arc<Absyn::Path>>> = bcl;
    (items, bcl) = 'mc: {
        let __mc_input = inBaseClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Extends { baseClass: bc, info, .. } => {
                    let mut env: Env;
                    let mut item: Item;
                    (item, _, env) = lookupBaseClassName(bc.clone(), inEnv.clone(), info.clone())?;
                    item = NFSCodeEnv::setImportsInItemHidden(item.clone(), true)?;
                    (item, _, _) = lookupNameInItem(Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), item.clone(), env.clone())?;
                    Ok((metamodelica::cons(item.clone(), items.clone()), metamodelica::cons(bc.clone(), bcl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((items.clone(), bcl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (items, bcl)
}

pub(crate) fn lookupInheritedName(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses((inName).clone(), inEnv, crate::NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES, metamodelica::nil())?) {
        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outItem = __pa0.clone();
    outEnv = __pa1.clone();
    Ok((outItem, outEnv))
}

pub(crate) fn lookupInheritedNameAndBC(mut inName: ArcStr, mut inEnv: Env) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> {
    let mut outItems: Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>;
    let mut outBaseClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    let mut bcl: Arc<metamodelica::List<Arc<NFSCodeEnv::Extends>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    bcl = __pa0.clone();
    (outItems, outBaseClasses) = List::fold22(bcl, (std::sync::Arc::new(fnptr!(lookupBaseClasses2, Arc<NFSCodeEnv::Extends>, ArcStr, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSCodeEnv::Extends>, ArcStr, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<metamodelica::List<Arc<NFSCodeEnv::Item>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), (inName).clone(), inEnv, metamodelica::nil(), metamodelica::nil())?;
    outBaseClasses = outBaseClasses.reverse();
    outItems = outItems.reverse();
    Ok((outItems, outBaseClasses))
}

pub(crate) fn lookupRedeclaredClassByItem(mut inItem: Item, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = 'mc: {
        let __mc_input = inItem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut rdp: SCode::Redeclare;
                    let mut rpp: Arc<SCode::Replaceable>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses((name.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, metamodelica::nil())?) {
                        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    env = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemPrefixes(item.clone())?) {
                        Deref @ SCode::Prefixes { redeclarePrefix: __pa2, replaceablePrefix: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rdp = __pa2.clone();
                    rpp = __pa3.clone();
                    (item, env) = lookupRedeclaredClass2(item.clone(), rdp.clone(), rpp.clone(), env.clone(), inInfo.clone())?;
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.lookupRedeclaredClassByItem failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

fn lookupRedeclaredClass2(mut inItem: Item, mut inRedeclarePrefix: SCode::Redeclare, mut inReplaceablePrefix: Arc<SCode::Replaceable>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = 'mc: {
        let __mc_input = (inItem.clone(), inRedeclarePrefix.clone(), inReplaceablePrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Redeclare::NOT_REDECLARE { .. }, Deref @ SCode::Replaceable::REPLACEABLE { .. }) => {
                    Ok((inItem.clone(), inEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. }, SCode::Redeclare::REDECLARE { .. }, Deref @ SCode::Replaceable::REPLACEABLE { .. }) => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut rdp: SCode::Redeclare;
                    let mut rpp: Arc<SCode::Replaceable>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses((name.clone()).clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, metamodelica::nil())?) {
                        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    item = __pa0.clone();
                    env = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemPrefixes(item.clone())?) {
                        Deref @ SCode::Prefixes { redeclarePrefix: __pa2, replaceablePrefix: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rdp = __pa2.clone();
                    rpp = __pa3.clone();
                    (item, env) = lookupRedeclaredClass2(item.clone(), rdp.clone(), rpp.clone(), env.clone(), inInfo.clone())?;
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env }, _, _) => {
                    let mut item = (*item).clone();
                    let mut env = (*env).clone();
                    (item, env) = lookupRedeclaredClass2(item.clone(), inRedeclarePrefix.clone(), inReplaceablePrefix.clone(), env.clone(), inInfo.clone())?;
                    Ok((item.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, info, .. }, .. }, _, Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }) => {
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), inInfo.clone())?;
                    Error::addSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(name.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, info, .. }, .. }, _, _) => {
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), inInfo.clone())?;
                    Error::addSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("component")).clone(), (name.clone()).clone(), (literal!("a class")).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.lookupRedeclaredClass2 failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outEnv))
}

pub(crate) fn lookupBuiltinType(mut inName: ArcStr) -> Result<Item> {
    let mut outItem: Item;
    outItem = (::match_deref::match_deref! { match &(inName) {
        Deref @ "Real" => BUILTIN_REAL.clone(),
        Deref @ "Integer" => BUILTIN_INTEGER.clone(),
        Deref @ "Boolean" => BUILTIN_BOOLEAN.clone(),
        Deref @ "String" => BUILTIN_STRING.clone(),
        Deref @ "StateSelect" => BUILTIN_STATESELECT.clone(),
        Deref @ "ExternalObject" => BUILTIN_EXTERNALOBJECT.clone(),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { bail!("pattern mismatch") };
            BUILTIN_CLOCK.clone()
        },
        Deref @ "$RealType" => BUILTIN_REALTYPE_ITEM.clone(),
        Deref @ "$IntegerType" => BUILTIN_INTEGERTYPE_ITEM.clone(),
        Deref @ "$BooleanType" => BUILTIN_BOOLEANTYPE_ITEM.clone(),
        Deref @ "$StringType" => BUILTIN_STRINGTYPE_ITEM.clone(),
        Deref @ "$EnumType" => BUILTIN_ENUMTYPE_ITEM.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outItem)
}

fn lookupBuiltinName(mut inName: Arc<Absyn::Path>) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = (::match_deref::match_deref! { match &(inName) {
        Deref @ Absyn::Path::IDENT { name: id } => {
            let mut item: Item;
            item = lookupBuiltinType((id.clone()).clone())?;
            (item.clone(), NFSCodeEnv::emptyEnv.clone())
        },
        Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: id } } => {
            let mut item: Item;
            (item, _) = lookupInClass((id.clone()).clone(), BUILTIN_STATESELECT_ENV.clone())?;
            (item.clone(), BUILTIN_STATESELECT_ENV.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn lookupName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inLookupStrategy: LookupStrategy, mut inInfo: SourceInfo, mut inErrorType: Option<ErrorTypes::Message>) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inLookupStrategy, inErrorType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, LookupStrategy::LOOKUP_ANY { .. }, _) => {
                    let mut item: Item;
                    let mut env: Env;
                    (item, env) = lookupBuiltinName(inName.clone())?;
                    Ok((item.clone(), inName.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: id }, _, _) => {
                    let mut item: Item;
                    let mut new_path: Arc<Absyn::Path>;
                    let mut env: Env;
                    (item, new_path, env) = lookupSimpleName((id.clone()).clone(), inEnv.clone())?;
                    Ok((item.clone(), new_path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { name: id, path }, _, _) => {
                    let mut item: Item;
                    let mut new_path: Arc<Absyn::Path>;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, new_path, env) = lookupSimpleName((id.clone()).clone(), inEnv.clone())?;
                    (item, path, env) = lookupNameInItem(path.clone(), item.clone(), env.clone())?;
                    path = joinPaths(new_path.clone(), path.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _, _) => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, path, env) = lookupFullyQualified(path.clone(), inEnv.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(error_id)) => {
                    let mut name_str: ArcStr;
                    let mut env_str: ArcStr;
                    name_str = (AbsynUtil::pathString(inName.clone(), (literal!(".")).clone(), true, false)?).clone();
                    env_str = (NFSCodeEnv::getEnvName(inEnv.clone())).clone();
                    Error::addSourceMessage(error_id.clone(), list![(name_str.clone()).clone(), (env_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItem, outName, outEnv))
}

fn joinPaths(mut inPath1: Arc<Absyn::Path>, mut inPath2: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath1, inPath2.clone())) {
        (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => {
            inPath2
        },
        (Deref @ Absyn::Path::IDENT { name: id }, _) => {
            Arc::new(Absyn::Path::QUALIFIED { name: (id.clone()).clone(), path: inPath2 })
        },
        (Deref @ Absyn::Path::QUALIFIED { name: id, path }, _) => {
            let mut path = (*path).clone();
            path = joinPaths(path.clone(), inPath2)?;
            Arc::new(Absyn::Path::QUALIFIED { name: (id.clone()).clone(), path: path.clone() })
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _) => {
            let mut path = (*path).clone();
            path = joinPaths(path.clone(), inPath2)?;
            AbsynUtil::makeFullyQualified(path.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub(crate) fn lookupNameSilent(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo, None)?;
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupNameSilentNoBuiltin(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo, None)?;
    Ok((outItem, outName, outEnv))
}

pub fn lookupClassName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo, Some(Error::LOOKUP_ERROR.clone()))?;
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupBaseClassName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inEnv.clone())) {
        (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$ce", path: path @ Deref @ Absyn::Path::IDENT { name: id } }, Deref @ metamodelica::List::Cons { head: _, tail: env }) => {
            let mut item: Item;
            let mut env = (*env).clone();
            (item, env) = lookupInheritedName((id.clone()).clone(), env.clone())?;
            (item.clone(), path.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }, _) => {
            NFEnvExtends::printExtendsError(inName, inEnv, inInfo)?;
            bail!("fail")
        },
        _ => {
            let mut env: Env;
            let mut item: Item;
            let mut path: Arc<Absyn::Path>;
            (item, path, env) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo, Some(Error::LOOKUP_BASECLASS_ERROR.clone()))?;
            (item.clone(), path.clone(), env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupVariableName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo, Some(Error::LOOKUP_VARIABLE_ERROR.clone()))?;
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupFunctionName(mut inName: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: Arc<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo, Some(Error::LOOKUP_FUNCTION_ERROR.clone()))?;
    Ok((outItem, outName, outEnv))
}

fn crefStripEnvPrefix(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Arc<Absyn::ComponentRef> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env_path: Arc<Absyn::Path>;
                    let mut cref1: Arc<Absyn::ComponentRef>;
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    let false = (Flags::isSet(Flags::SCODE_INST.clone())?) else { bail!("pattern mismatch") };
                    env_path = NFSCodeEnv::getEnvPath(inEnv.clone())?;
                    cref1 = AbsynUtil::unqualifyCref(inCref.clone());
                    cref2 = crefStripEnvPrefix2(cref1.clone(), env_path.clone())?;
                    let false = (AbsynUtil::crefEqual(cref1.clone(), cref2.clone())?) else { bail!("pattern mismatch") };
                    Ok(cref2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn crefStripEnvPrefix2(mut inCref: Arc<Absyn::ComponentRef>, mut inEnvPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnvPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::List::Nil, componentRef: cref }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: env_path }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(crefStripEnvPrefix2(cref.clone(), env_path.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::List::Nil, componentRef: cref }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::List::Nil, .. }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let false = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

pub(crate) fn lookupComponentRef(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Arc<Absyn::ComponentRef> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = inCref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "StateSelect", subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { .. } } => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::WILD { .. } => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref: Arc<Absyn::ComponentRef>;
                    cref = NFSCodeFlattenImports::flattenComponentRefSubs(inCref.clone(), inEnv.clone(), inInfo.clone())?;
                    (cref, _) = lookupComponentRef2(cref.clone(), inEnv.clone())?;
                    cref = crefStripEnvPrefix(cref.clone(), inEnv.clone());
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn lookupComponentRef2(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<(Arc<Absyn::ComponentRef>, Env)> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    let mut outEnv: Env;
    (outCref, outEnv) = (::match_deref::match_deref! { match &(inCref) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut path: Arc<Absyn::Path>;
            let mut env: Env;
            (_, path, env) = lookupSimpleName((name.clone()).clone(), inEnv)?;
            cref = AbsynUtil::pathToCrefWithSubs(path.clone(), subs.clone())?;
            (cref.clone(), env.clone())
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts: subs, componentRef: rest_cref } => {
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut new_path: Arc<Absyn::Path>;
            let mut env: Env;
            let mut item: Item;
            let mut rest_cref = (*rest_cref).clone();
            (item, new_path, env) = lookupSimpleName((name.clone()).clone(), inEnv)?;
            cref = AbsynUtil::pathToCrefWithSubs(new_path.clone(), subs.clone())?;
            (item, rest_cref) = lookupCrefInItem(rest_cref.clone(), item.clone(), env.clone())?;
            cref = joinCrefs(cref.clone(), rest_cref.clone())?;
            (cref.clone(), env.clone())
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut env: Env;
            let mut cref = (*cref).clone();
            cref = lookupCrefFullyQualified(cref.clone(), inEnv.clone())?;
            env = NFSCodeEnv::getEnvTopScope(inEnv)?;
            (cref.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCref, outEnv))
}

pub(crate) fn lookupCrefFullyQualified(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    let mut env: Env;
    env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
    (_, outCref) = lookupCrefInPackage(inCref, inEnv)?;
    outCref = AbsynUtil::crefMakeFullyQualified(outCref);
    Ok(outCref)
}

pub(crate) fn joinCrefs(mut inCref1: Arc<Absyn::ComponentRef>, mut inCref2: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref2.clone()) {
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => inCref2,
        _ => AbsynUtil::joinCrefs(inCref1, inCref2)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub(crate) fn lookupTypeSpec(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Item, Arc<Absyn::TypeSpec>, Env)> {
    let mut outItem: Item;
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    let mut outTypeEnv: Env;
    (outItem, outTypeSpec, outTypeEnv) = (::match_deref::match_deref! { match &(inTypeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: ad } => {
            let mut newpath: Arc<Absyn::Path>;
            let mut item: Item;
            let mut env: Env;
            (item, newpath, env) = lookupClassName(path.clone(), inEnv, inInfo)?;
            (item.clone(), Arc::new(Absyn::TypeSpec::TPATH { path: newpath.clone(), arrayDim: ad.clone() }), env.clone())
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name }, .. } => {
            let mut cls: Arc<SCode::Element>;
            cls = makeDummyMetaType((name.clone()).clone());
            (Arc::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }), inTypeSpec, NFSCodeEnv::emptyEnv.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outItem, outTypeSpec, outTypeEnv))
}

fn makeDummyMetaType(mut inTypeName: ArcStr) -> Arc<SCode::Element> {
    let mut outClass: Arc<SCode::Element>;
    outClass = Arc::new(SCode::Element::CLASS { name: (inTypeName).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: metamodelica::nil(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: Absyn::dummyInfo.clone() });
    outClass
}

pub(crate) fn qualifyPath(mut inPath: Arc<Absyn::Path>, mut inEnv: Env, mut inInfo: SourceInfo, mut inErrorType: Option<ErrorTypes::Message>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: id } => {
                    lookupBuiltinType((id.clone()).clone())?;
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: Arc<Absyn::Path>;
                    let mut env: Env;
                    (_, path, env) = lookupName(inPath.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo.clone(), inErrorType.clone())?;
                    path = NFSCodeEnv::mergePathWithEnvPath(path.clone(), env.clone());
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.qualifyPath failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

