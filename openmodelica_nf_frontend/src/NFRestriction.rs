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

use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFRestriction {
    BLOCK,
    CLASS,
    CLOCK,
    CONNECTOR {
        isExpandable: bool,
    },
    ENUMERATION,
    EXTERNAL_OBJECT,
    FUNCTION,
    MODEL,
    PACKAGE,
    OPERATOR,
    RECORD {
        isOperator: bool,
        usedExternally: bool,
    },
    RECORD_CONSTRUCTOR,
    TYPE,
    UNKNOWN,
}
impl metamodelica::gc::MMTrace for NFRestriction {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFRestriction::BLOCK => Ok(()),
            NFRestriction::CLASS => Ok(()),
            NFRestriction::CLOCK => Ok(()),
            NFRestriction::CONNECTOR { isExpandable } => {
                metamodelica::gc::MMTrace::mm_accept(isExpandable, __mmv)?;
                Ok(())
            }
            NFRestriction::ENUMERATION => Ok(()),
            NFRestriction::EXTERNAL_OBJECT => Ok(()),
            NFRestriction::FUNCTION => Ok(()),
            NFRestriction::MODEL => Ok(()),
            NFRestriction::PACKAGE => Ok(()),
            NFRestriction::OPERATOR => Ok(()),
            NFRestriction::RECORD { isOperator, usedExternally } => {
                metamodelica::gc::MMTrace::mm_accept(isOperator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(usedExternally, __mmv)?;
                Ok(())
            }
            NFRestriction::RECORD_CONSTRUCTOR => Ok(()),
            NFRestriction::TYPE => Ok(()),
            NFRestriction::UNKNOWN => Ok(()),
        }
    }
}
impl NFRestriction {
    pub fn interned_BLOCK() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::BLOCK));
        (*INTERNED).clone()
    }
    pub fn interned_CLASS() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::CLASS));
        (*INTERNED).clone()
    }
    pub fn interned_CLOCK() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::CLOCK));
        (*INTERNED).clone()
    }
    pub fn interned_ENUMERATION() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::ENUMERATION));
        (*INTERNED).clone()
    }
    pub fn interned_EXTERNAL_OBJECT() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::EXTERNAL_OBJECT));
        (*INTERNED).clone()
    }
    pub fn interned_FUNCTION() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::FUNCTION));
        (*INTERNED).clone()
    }
    pub fn interned_MODEL() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::MODEL));
        (*INTERNED).clone()
    }
    pub fn interned_PACKAGE() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::PACKAGE));
        (*INTERNED).clone()
    }
    pub fn interned_OPERATOR() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::OPERATOR));
        (*INTERNED).clone()
    }
    pub fn interned_RECORD_CONSTRUCTOR() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::RECORD_CONSTRUCTOR));
        (*INTERNED).clone()
    }
    pub fn interned_TYPE() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_UNKNOWN() -> Arc<NFRestriction> {
        static INTERNED: std::sync::LazyLock<Arc<NFRestriction>> = std::sync::LazyLock::new(|| Arc::new(NFRestriction::UNKNOWN));
        (*INTERNED).clone()
    }
}
pub fn interned_BLOCK() -> Arc<NFRestriction> { NFRestriction::interned_BLOCK() }
pub fn interned_CLASS() -> Arc<NFRestriction> { NFRestriction::interned_CLASS() }
pub fn interned_CLOCK() -> Arc<NFRestriction> { NFRestriction::interned_CLOCK() }
pub fn interned_ENUMERATION() -> Arc<NFRestriction> { NFRestriction::interned_ENUMERATION() }
pub fn interned_EXTERNAL_OBJECT() -> Arc<NFRestriction> { NFRestriction::interned_EXTERNAL_OBJECT() }
pub fn interned_FUNCTION() -> Arc<NFRestriction> { NFRestriction::interned_FUNCTION() }
pub fn interned_MODEL() -> Arc<NFRestriction> { NFRestriction::interned_MODEL() }
pub fn interned_PACKAGE() -> Arc<NFRestriction> { NFRestriction::interned_PACKAGE() }
pub fn interned_OPERATOR() -> Arc<NFRestriction> { NFRestriction::interned_OPERATOR() }
pub fn interned_RECORD_CONSTRUCTOR() -> Arc<NFRestriction> { NFRestriction::interned_RECORD_CONSTRUCTOR() }
pub fn interned_TYPE() -> Arc<NFRestriction> { NFRestriction::interned_TYPE() }
pub fn interned_UNKNOWN() -> Arc<NFRestriction> { NFRestriction::interned_UNKNOWN() }
impl Default for NFRestriction {
    fn default() -> Self { Self::BLOCK }
}
pub use self::NFRestriction::{BLOCK,CLASS,CLOCK,CONNECTOR,ENUMERATION,EXTERNAL_OBJECT,FUNCTION,MODEL,PACKAGE,OPERATOR,RECORD,RECORD_CONSTRUCTOR,TYPE,UNKNOWN};
pub(crate) fn fromSCode(mut sres: SCode::Restriction) -> Arc<NFRestriction> {
    let mut res: Arc<NFRestriction>;
    res = (match sres.clone() {
        SCode::Restriction::R_BLOCK { .. } => crate::NFRestriction::interned_BLOCK(),
        SCode::Restriction::R_CLASS { .. } => crate::NFRestriction::interned_CLASS(),
        SCode::Restriction::R_PREDEFINED_CLOCK { .. } => crate::NFRestriction::interned_CLOCK(),
        SCode::Restriction::R_CONNECTOR { .. } => Arc::new(NFRestriction::CONNECTOR { isExpandable: var_field!(sres.isExpandable, SCode::Restriction::R_CONNECTOR).clone() }),
        SCode::Restriction::R_ENUMERATION { .. } => crate::NFRestriction::interned_ENUMERATION(),
        SCode::Restriction::R_FUNCTION { .. } => crate::NFRestriction::interned_FUNCTION(),
        SCode::Restriction::R_MODEL { .. } => crate::NFRestriction::interned_MODEL(),
        SCode::Restriction::R_OPERATOR { .. } => crate::NFRestriction::interned_OPERATOR(),
        SCode::Restriction::R_PACKAGE { .. } => crate::NFRestriction::interned_PACKAGE(),
        SCode::Restriction::R_RECORD { .. } => Arc::new(NFRestriction::RECORD { isOperator: var_field!(sres.isOperator, SCode::Restriction::R_RECORD).clone(), usedExternally: false }),
        SCode::Restriction::R_TYPE { .. } => crate::NFRestriction::interned_TYPE(),
        _ => crate::NFRestriction::interned_MODEL(),
    });
    res
}

pub(crate) fn toDAE(mut res: Arc<NFRestriction>, mut path: Arc<Absyn::Path>) -> ClassInf::State {
    let mut state: ClassInf::State;
    state = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ BLOCK { .. } => ClassInf::State::BLOCK { path: path },
        Deref @ CLOCK { .. } => ClassInf::State::TYPE_CLOCK { path: path },
        Deref @ CONNECTOR { .. } => ClassInf::State::CONNECTOR { path: path, isExpandable: var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone() },
        Deref @ ENUMERATION { .. } => ClassInf::State::ENUMERATION { path: path },
        Deref @ EXTERNAL_OBJECT { .. } => ClassInf::State::EXTERNAL_OBJ { path: path },
        Deref @ FUNCTION { .. } => ClassInf::State::FUNCTION { path: path, isImpure: false },
        Deref @ MODEL { .. } => ClassInf::State::MODEL { path: path },
        Deref @ OPERATOR { .. } => ClassInf::State::FUNCTION { path: path, isImpure: false },
        Deref @ PACKAGE { .. } => ClassInf::State::PACKAGE { path: path },
        Deref @ RECORD { .. } => ClassInf::State::RECORD { path: path },
        Deref @ RECORD_CONSTRUCTOR { .. } => ClassInf::State::RECORD { path: path },
        Deref @ TYPE { .. } => ClassInf::State::TYPE { path: path },
        _ => ClassInf::State::UNKNOWN { path: path },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    state
}

pub(crate) fn isConnector(mut res: Arc<NFRestriction>) -> bool {
    let mut isConnector: bool;
    isConnector = (::match_deref::match_deref! { match &(res) {
        Deref @ CONNECTOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

pub(crate) fn isExpandableConnector(mut res: Arc<NFRestriction>) -> bool {
    let mut isConnector: bool;
    isConnector = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ CONNECTOR { .. } => var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

pub(crate) fn isNonexpandableConnector(mut res: Arc<NFRestriction>) -> bool {
    let mut isNonexpandable: bool;
    isNonexpandable = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ CONNECTOR { .. } => !(var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNonexpandable
}

pub(crate) fn isExternalObject(mut res: Arc<NFRestriction>) -> bool {
    let mut isExternalObject: bool;
    isExternalObject = (::match_deref::match_deref! { match &(res) {
        Deref @ EXTERNAL_OBJECT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExternalObject
}

pub(crate) fn isFunction(mut res: Arc<NFRestriction>) -> bool {
    let mut isFunction: bool;
    isFunction = (::match_deref::match_deref! { match &(res) {
        Deref @ FUNCTION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isFunction
}

pub(crate) fn isRecordConstructor(mut res: Arc<NFRestriction>) -> bool {
    let mut isConstructor: bool;
    isConstructor = (::match_deref::match_deref! { match &(res) {
        Deref @ RECORD_CONSTRUCTOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConstructor
}

pub(crate) fn isRecord(mut res: Arc<NFRestriction>) -> bool {
    let mut isRecord: bool;
    isRecord = (::match_deref::match_deref! { match &(res) {
        Deref @ RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

pub(crate) fn isExternalRecord(mut res: Arc<NFRestriction>) -> bool {
    let mut isExtRecord: bool;
    isExtRecord = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { .. } => var_field!((*res).usedExternally, NFRestriction::RECORD).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtRecord
}

pub(crate) fn setExternalRecord(mut res: Arc<NFRestriction>) -> Arc<NFRestriction> {
    let mut res: Arc<NFRestriction> = res;
    let () = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { usedExternally: false, .. } => {
            assign_variant_field!(res => NFRestriction::RECORD; usedExternally = true);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn isOperatorRecord(mut res: Arc<NFRestriction>) -> bool {
    let mut isOpRecord: bool;
    isOpRecord = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ RECORD { .. } => var_field!((*res).isOperator, NFRestriction::RECORD).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOpRecord
}

pub(crate) fn isOperator(mut res: Arc<NFRestriction>) -> bool {
    let mut isOperator: bool;
    isOperator = (::match_deref::match_deref! { match &(res) {
        Deref @ OPERATOR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOperator
}

pub(crate) fn isType(mut res: Arc<NFRestriction>) -> bool {
    let mut isType: bool;
    isType = (::match_deref::match_deref! { match &(res) {
        Deref @ TYPE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isType
}

pub(crate) fn isClock(mut res: Arc<NFRestriction>) -> bool {
    let mut isClock: bool;
    isClock = (::match_deref::match_deref! { match &(res) {
        Deref @ CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isClock
}

pub(crate) fn isModel(mut res: Arc<NFRestriction>) -> bool {
    let mut isModel: bool;
    isModel = (::match_deref::match_deref! { match &(res) {
        Deref @ MODEL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isModel
}

pub fn toString(mut res: Arc<NFRestriction>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(res.clone()) {
        Deref @ BLOCK { .. } => literal!("block"),
        Deref @ CLASS { .. } => literal!("class"),
        Deref @ CLOCK { .. } => literal!("clock"),
        Deref @ CONNECTOR { .. } => if (var_field!((*res).isExpandable, NFRestriction::CONNECTOR).clone()) {literal!("expandable connector")} else {literal!("connector")},
        Deref @ ENUMERATION { .. } => literal!("enumeration"),
        Deref @ EXTERNAL_OBJECT { .. } => literal!("ExternalObject"),
        Deref @ FUNCTION { .. } => literal!("function"),
        Deref @ MODEL { .. } => literal!("model"),
        Deref @ OPERATOR { .. } => literal!("operator"),
        Deref @ PACKAGE { .. } => literal!("package"),
        Deref @ RECORD { .. } => literal!("record"),
        Deref @ RECORD_CONSTRUCTOR { .. } => literal!("record"),
        Deref @ TYPE { .. } => literal!("type"),
        _ => literal!("unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    r#str
}

pub(crate) fn assertNoEquations(mut equations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut initialEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut res: Arc<NFRestriction>, mut onlyDeprecated: bool) -> Result<()> {
    let mut eq: Arc<SCode::Equation>;
    if equations.clone().is_empty() && initialEquations.clone().is_empty() {
        return Ok(());
    }
    eq = listHead(if (equations.clone().is_empty()) {initialEquations} else {equations})?;
    if onlyDeprecated {
        Error::addSourceMessage(Error::DEPRECATED_TRANSITION_FAILURE.clone(), list![(literal!("Equation sections")).clone(), (toString(res)).clone()], SCodeUtil::getEquationInfo(eq)?)?;
    } else {
        Error::addSourceMessage(Error::EQUATION_TRANSITION_FAILURE.clone(), list![(toString(res)).clone()], SCodeUtil::getEquationInfo(eq)?)?;
        bail!("fail");
    }
    Ok(())
}

pub(crate) fn assertNoAlgorithms(mut algorithms: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut initialAlgorithms: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut res: Arc<NFRestriction>, mut onlyDeprecated: bool) -> Result<()> {
    let mut alg_opt: Option<Arc<SCode::AlgorithmSection>> = None;
    let mut alg: Arc<SCode::AlgorithmSection>;
    let mut info: SourceInfo;
    alg_opt = List::findOption(algorithms, (std::sync::Arc::new(fnptr!(SCodeUtil::isNonEmptyAlgorithm, Arc<SCode::AlgorithmSection>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>) -> Result<bool> + 'static>))?;
    if isNone(alg_opt.clone()) {
        alg_opt = List::findOption(initialAlgorithms, (std::sync::Arc::new(fnptr!(SCodeUtil::isNonEmptyAlgorithm, Arc<SCode::AlgorithmSection>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>) -> Result<bool> + 'static>))?;
    }
    if isSome(alg_opt.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(alg_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        alg = __pa0.clone();
        info = SCodeUtil::getStatementInfo(listHead(alg.statements.clone())?)?;
        if onlyDeprecated {
            Error::addSourceMessage(Error::DEPRECATED_TRANSITION_FAILURE.clone(), list![(literal!("Algorithm sections")).clone(), (toString(res)).clone()], info)?;
            return Ok(());
        } else {
            Error::addSourceMessage(Error::ALGORITHM_TRANSITION_FAILURE.clone(), list![(toString(res)).clone()], info)?;
            bail!("fail");
        }
    }
    Ok(())
}

pub(crate) fn assertNoInitialAlgorithms(mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut res: Arc<NFRestriction>) -> Result<()> {
    for mut alg in &*algs {
        let mut alg = alg.clone();
        if !(alg.statements.clone().is_empty()) {
            Error::addSourceMessage(Error::INITIAL_ALGORITHM_TRANSITION_FAILURE.clone(), list![(toString(res.clone())).clone()], SCodeUtil::getStatementInfo(listHead(alg.statements.clone())?)?)?;
            bail!("fail");
        }
    }
    Ok(())
}

pub(crate) fn assertNoProtected(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut res: Arc<NFRestriction>) -> Result<()> {
    for mut e in &*elements {
        let mut e = e.clone();
        if SCodeUtil::isElementProtected(e.clone())? {
            Error::addSourceMessage(Error::PROTECTED_TRANSITION_FAILURE.clone(), list![(toString(res.clone())).clone()], SCodeUtil::elementInfo(e.clone()))?;
            bail!("fail");
        }
    }
    Ok(())
}

pub(crate) fn assertNoComponents(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut res: Arc<NFRestriction>) -> Result<()> {
    for mut e in &*elements {
        let mut e = e.clone();
        if SCodeUtil::isComponent(e.clone()) {
            Error::addSourceMessage(Error::DEPRECATED_TRANSITION_FAILURE.clone(), list![(literal!("Components")).clone(), (toString(res.clone())).clone()], SCodeUtil::elementInfo(e.clone()))?;
        }
    }
    Ok(())
}

pub(crate) fn assertOnlyConstantComponents(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut clsNode: Arc<InstNode::InstNode>) -> Result<()> {
    for mut e in &*elements {
        let mut e = e.clone();
        let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } if (!(SCodeUtil::isConstant(SCodeUtil::attrVariability(var_field!((*e).attributes, SCode::Element::COMPONENT).clone())?))) => {
            Error::addSourceMessage(Error::PACKAGE_VARIABLE_NOT_CONSTANT.clone(), list![(var_field!((*e).name, SCode::Element::COMPONENT).clone()).clone(), (InstNode::name(clsNode.clone())?).clone()], var_field!((*e).info, SCode::Element::COMPONENT).clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub(crate) fn assertOnlyFunctions(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut res: Arc<NFRestriction>) -> () {
    for mut e in &*elements {
        let mut e = e.clone();
        if !(SCodeUtil::isFunction(e.clone())) {
        }
    }
    ()
}

pub(crate) fn checkClass(mut node: Arc<InstNode::InstNode>, mut restriction: Arc<NFRestriction>, mut context: i32) -> Result<()> {
    let mut cdef: Arc<SCode::ClassDef>;
    if InstContext::inRelaxed(context) {
        return Ok(());
    }
    cdef = SCodeUtil::getClassBody(InstNode::definition(node.clone())?)?;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            let () = (::match_deref::match_deref! { match &(restriction.clone()) {
        Deref @ CLASS => {
            assertNoComponents(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), true)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction, true)?;
            ()
        },
        Deref @ RECORD { .. } => {
            assertNoProtected(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction, false)?;
            ()
        },
        Deref @ TYPE => {
            assertNoProtected(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction, false)?;
            ()
        },
        Deref @ BLOCK => (),
        Deref @ FUNCTION => {
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoInitialAlgorithms(var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction)?;
            ()
        },
        Deref @ CONNECTOR { .. } => {
            assertNoProtected(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), restriction.clone())?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction, false)?;
            ()
        },
        Deref @ PACKAGE => {
            assertOnlyConstantComponents(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), node)?;
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction, false)?;
            ()
        },
        Deref @ OPERATOR => {
            assertNoEquations(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), restriction.clone(), false)?;
            assertNoAlgorithms(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), restriction, false)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}


