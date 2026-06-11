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

use crate::AbsynUtil;
use crate::BackendInterface;
use crate::Dump;
use crate::MetaUtil;
use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// Constant expression for AssertionLevel.error.
pub(crate) static ASSERTION_LEVEL_ERROR: std::sync::LazyLock<Arc<Absyn::Exp>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("AssertionLevel")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("error")).clone(), subscripts: metamodelica::nil() }) }) }) }) });

pub fn translateAbsyn2SCode(mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outProgram = (match inProgram.clone() {
        _ => {
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut inClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>;
            BackendInterface::initInstHashTable()?;
            let Absyn::PROGRAM { classes: __pa0, .. } = (MetaUtil::createMetaClassesInProgram(inProgram)?) else { bail!("pattern mismatch") };
            inClasses = __pa0.clone();
            System::setHasInnerOuterDefinitions(false);
            System::setHasExpandableConnectors(false);
            System::setHasOverconstrainedConnectors(false);
            System::setHasStreamConnectors(false);
            sp = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (inClasses.clone()).into_iter().cloned() {
            let __x = translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            sp.clone()
        },
    });
    Ok(outProgram)
}

pub fn translateClass(mut inClass: Arc<Absyn::Class>) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = translateClass2(inClass, Error::getNumMessages())?;
    Ok(outClass)
}

fn translateClass2(mut inClass: Arc<Absyn::Class>, mut inNumMessages: i32) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = 'mc: {
        let __mc_input = inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                c @ Deref @ Absyn::Class { name: n, partialPrefix: p, finalPrefix: f, encapsulatedPrefix: e, restriction: r, body: d, info: file_info, .. } => {
                    let mut d_1: Arc<SCode::ClassDef>;
                    let mut r_1: SCode::Restriction;
                    let mut scodeClass: Arc<SCode::Element>;
                    let mut sFin: SCode::Final;
                    let mut sEnc: SCode::Encapsulated;
                    let mut sPar: SCode::Partial;
                    let mut cmt: Arc<SCode::Comment>;
                    r_1 = translateRestriction(c.clone(), r.clone())?;
                    (d_1, cmt) = translateClassdef(d.clone(), file_info.clone(), r_1.clone())?;
                    sFin = SCodeUtil::boolFinal(f.clone());
                    sEnc = SCodeUtil::boolEncapsulated(e.clone());
                    sPar = SCodeUtil::boolPartial(p.clone());
                    scodeClass = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: sFin.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE() }), encapsulatedPrefix: sEnc.clone(), partialPrefix: sPar.clone(), restriction: r_1.clone(), classDef: d_1.clone(), cmt: cmt.clone(), info: file_info.clone() });
                    Ok(scodeClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { name: n, info: file_info, .. } => {
                    let mut n = (*n).clone();
                    let true = (intEq(Error::getNumMessages(), inNumMessages)) else { bail!("pattern mismatch") };
                    n = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynToSCode.translateClass2 failed: ")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(n.clone()).clone()], file_info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

//mahge: FIX HERE. Check for proper input and output
//declarations in operators according to the specifications.
pub(crate) fn translateOperatorDef(mut inClassDef: Arc<Absyn::ClassDef>, mut operatorName: ArcStr, mut info: SourceInfo) -> Result<(Arc<SCode::ClassDef>, Arc<SCode::Comment>)> {
    let mut outOperDef: Arc<SCode::ClassDef>;
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    (outOperDef, cmt) = (::match_deref::match_deref! { match &(inClassDef) {
        Deref @ Absyn::ClassDef::PARTS { classParts: parts, ann: aann, comment: cmtString, .. } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            els = translateClassdefElements(parts.clone())?;
            cmt = translateCommentList(aann.clone(), cmtString.clone())?;
            (Arc::new(SCode::ClassDef::PARTS { elementLst: els.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt)
        },
        _ => {
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Could not translate operator to SCode because it is not using class parts.")).clone()], info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outOperDef, cmt))
}

pub(crate) fn getOperatorGivenName(mut inOperatorFunction: Arc<SCode::Element>) -> Result<Arc<Absyn::Path>> {
    let mut outName: Arc<Absyn::Path>;
    outName = (::match_deref::match_deref! { match &(inOperatorFunction) {
        Deref @ SCode::Element::CLASS { name, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, classDef: _, cmt: _, info: _ } => {
            Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outName)
}

pub(crate) fn getOperatorQualName(mut inOperatorFunction: Arc<SCode::Element>, mut operName: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut outName: Arc<Absyn::Path>;
    outName = (::match_deref::match_deref! { match &((inOperatorFunction, operName)) {
        (Deref @ SCode::Element::CLASS { name, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, classDef: _, cmt: _, info: _ }, opname) => {
            AbsynUtil::joinPaths(Arc::new(Absyn::Path::IDENT { name: (opname.clone()).clone() }), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outName)
}

pub fn getListofQualOperatorFuncsfromOperator(mut inOperator: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    outNames = (::match_deref::match_deref! { match &(inOperator) {
        Deref @ SCode::Element::CLASS { name: opername, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_OPERATOR { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, cmt: _, info: _ } => {
            let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            names = List::map1(els.clone(), (std::sync::Arc::new(getOperatorQualName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, ArcStr) -> Result<Arc<Absyn::Path>> + 'static>), (opername.clone()).clone())?;
            names.clone()
        },
        Deref @ SCode::Element::CLASS { name: opername, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, classDef: _, cmt: _, info: _ } => {
            let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            names = list![Arc::new(Absyn::Path::IDENT { name: (opername.clone()).clone() })];
            names.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNames)
}

pub(crate) fn translateRestriction(mut inClass: Arc<Absyn::Class>, mut inRestriction: Absyn::Restriction) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    outRestriction = (::match_deref::match_deref! { match &((inClass, inRestriction)) {
        (d, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity } }) => {
            if (containsExternalFuncDecl(d.clone())?) {SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: purity.clone() } }} else {SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: purity.clone() } }}
        },
        (_, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }) => {
            SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }
        },
        (_, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } }) => {
            SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_PARALLEL_FUNCTION }
        },
        (_, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_KERNEL_FUNCTION { .. } }) => {
            SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_KERNEL_FUNCTION }
        },
        (_, Absyn::Restriction::R_CLASS { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_CLASS
        },
        (_, Absyn::Restriction::R_OPTIMIZATION { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_OPTIMIZATION
        },
        (_, Absyn::Restriction::R_MODEL { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_MODEL
        },
        (_, Absyn::Restriction::R_RECORD { .. }) => {
            SCode::Restriction::R_RECORD { isOperator: false }
        },
        (_, Absyn::Restriction::R_OPERATOR_RECORD { .. }) => {
            SCode::Restriction::R_RECORD { isOperator: true }
        },
        (_, Absyn::Restriction::R_BLOCK { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_BLOCK
        },
        (_, Absyn::Restriction::R_CONNECTOR { .. }) => {
            SCode::Restriction::R_CONNECTOR { isExpandable: false }
        },
        (_, Absyn::Restriction::R_EXP_CONNECTOR { .. }) => {
            System::setHasExpandableConnectors(true);
            SCode::Restriction::R_CONNECTOR { isExpandable: true }
        },
        (_, Absyn::Restriction::R_OPERATOR { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_OPERATOR
        },
        (_, Absyn::Restriction::R_TYPE { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_TYPE
        },
        (_, Absyn::Restriction::R_PACKAGE { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PACKAGE
        },
        (_, Absyn::Restriction::R_ENUMERATION { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION
        },
        (_, Absyn::Restriction::R_PREDEFINED_INTEGER { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER
        },
        (_, Absyn::Restriction::R_PREDEFINED_REAL { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL
        },
        (_, Absyn::Restriction::R_PREDEFINED_STRING { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING
        },
        (_, Absyn::Restriction::R_PREDEFINED_BOOLEAN { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN
        },
        (_, Absyn::Restriction::R_PREDEFINED_CLOCK { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK
        },
        (_, Absyn::Restriction::R_PREDEFINED_ENUMERATION { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION
        },
        (_, Absyn::Restriction::R_METARECORD { name, index, singleton, moved, typeVars }) => {
            SCode::Restriction::R_METARECORD { name: name.clone(), index: index.clone(), singleton: singleton.clone(), moved: moved.clone(), typeVars: typeVars.clone() }
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, .. }, .. }, Absyn::Restriction::R_UNIONTYPE { .. }) => {
            SCode::Restriction::R_UNIONTYPE { typeVars: typeVars.clone() }
        },
        (_, Absyn::Restriction::R_UNIONTYPE { .. }) => {
            SCode::Restriction::R_UNIONTYPE { typeVars: metamodelica::nil() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRestriction)
}

fn containsExternalFuncDecl(mut inClass: Arc<Absyn::Class>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            List::any(parts.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isExternalPart, Arc<Absyn::ClassPart>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>) -> Result<bool> + 'static>))?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            List::any(parts.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isExternalPart, Arc<Absyn::ClassPart>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>) -> Result<bool> + 'static>))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

fn translateAttributes(mut inEA: Absyn::ElementAttributes, mut extraArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<SCode::Attributes> {
    let mut outA: SCode::Attributes;
    outA = (::match_deref::match_deref! { match &((inEA, extraArrayDim)) {
        (Absyn::ElementAttributes { flowPrefix: f, streamPrefix: s, parallelism: p, variability: v, direction: dir, isField: fi, arrayDim: adim }, extraADim) => {
            let mut ct: SCode::ConnectorType;
            let mut sp: SCode::Parallelism;
            let mut sv: SCode::Variability;
            let mut adim = (*adim).clone();
            ct = translateConnectorType(f.clone(), s.clone())?;
            sv = translateVariability(v.clone())?;
            sp = translateParallelism(p.clone())?;
            adim = listAppend(extraADim.clone(), adim.clone());
            SCode::Attributes { arrayDims: adim.clone(), connectorType: ct.clone(), parallelism: sp.clone(), variability: sv.clone(), direction: dir.clone(), isField: fi.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outA)
}

fn translateConnectorType(mut inFlow: bool, mut inStream: bool) -> Result<SCode::ConnectorType> {
    let mut outType: SCode::ConnectorType;
    outType = (match (inFlow, inStream) {
        (false, false) => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        (true, false) => openmodelica_frontend_types::SCode::ConnectorType::FLOW,
        (false, true) => openmodelica_frontend_types::SCode::ConnectorType::STREAM,
        (true, true) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("AbsynToSCode.translateConnectorType got both flow and stream prefix.")).clone()])?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outType)
}

fn translateClassdef(mut inClassDef: Arc<Absyn::ClassDef>, mut info: SourceInfo, mut re: SCode::Restriction) -> Result<(Arc<SCode::ClassDef>, Arc<SCode::Comment>)> {
    let mut outClassDef: Arc<SCode::ClassDef>;
    let mut outComment: Arc<SCode::Comment>;
    (outClassDef, outComment) = (::match_deref::match_deref! { match &(inClassDef) {
        Deref @ Absyn::ClassDef::DERIVED { typeSpec: t, attributes: attr, arguments: a, comment: cmt } => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut scodeAttr: SCode::Attributes;
            checkTypeSpec(t.clone(), info.clone())?;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: a.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info, false)?;
            scodeAttr = translateAttributes(attr.clone(), metamodelica::nil())?;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::DERIVED { typeSpec: t.clone(), modifications: r#mod.clone(), attributes: scodeAttr.clone() }), scodeCmt.clone())
        },
        Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmtString } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut typeVars = (*typeVars).clone();
            typeVars = (match re.clone() {
        SCode::Restriction::R_METARECORD { .. } => List::union(typeVars.clone(), var_field!(re.typeVars, SCode::Restriction::R_METARECORD).clone()),
        SCode::Restriction::R_UNIONTYPE { .. } => List::union(typeVars.clone(), var_field!(re.typeVars, SCode::Restriction::R_UNIONTYPE).clone()),
        _ => typeVars.clone(),
    });
            tvels = List::map1(typeVars.clone(), (std::sync::Arc::new(fnptr!(makeTypeVarElement, ArcStr, SourceInfo)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, SourceInfo) -> Result<Arc<SCode::Element>> + 'static>), info.clone())?;
            els = translateClassdefElements(parts.clone())?;
            els = listAppend(tvels.clone(), els.clone());
            eqs = translateClassdefEquations(parts.clone())?;
            initeqs = translateClassdefInitialequations(parts.clone())?;
            als = translateClassdefAlgorithms(parts.clone())?;
            initals = translateClassdefInitialalgorithms(parts.clone())?;
            cos = translateClassdefConstraints(parts.clone())?;
            scodeCmt = translateCommentList(ann.clone(), cmtString.clone())?;
            decl = translateClassdefExternaldecls(parts.clone())?;
            decl = translateAlternativeExternalAnnotation(decl.clone(), scodeCmt.clone(), info)?;
            (Arc::new(SCode::ClassDef::PARTS { elementLst: els.clone(), normalEquationLst: eqs.clone(), initialEquationLst: initeqs.clone(), normalAlgorithmLst: als.clone(), initialAlgorithmLst: initals.clone(), constraintLst: cos.clone(), clsattrs: classAttrs.clone(), externalDecl: decl.clone() }), scodeCmt.clone())
        },
        Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: Deref @ Absyn::EnumDef::ENUMLITERALS { enumLiterals: lst }, comment: cmt } => {
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            lst_1 = translateEnumlist(lst.clone())?;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::ENUMERATION { enumLst: lst_1.clone() }), scodeCmt.clone())
        },
        Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: Deref @ Absyn::EnumDef::ENUM_COLON { .. }, comment: cmt } => {
            let mut scodeCmt: Arc<SCode::Comment>;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::ENUMERATION { enumLst: metamodelica::nil() }), scodeCmt.clone())
        },
        Deref @ Absyn::ClassDef::OVERLOAD { functionNames: pathLst, comment: cmt } => {
            let mut scodeCmt: Arc<SCode::Comment>;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::OVERLOAD { pathLst: pathLst.clone() }), scodeCmt.clone())
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { modifications: cmod, ann, comment: cmtString, parts, .. } => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            els = translateClassdefElements(parts.clone())?;
            eqs = translateClassdefEquations(parts.clone())?;
            initeqs = translateClassdefInitialequations(parts.clone())?;
            als = translateClassdefAlgorithms(parts.clone())?;
            initals = translateClassdefInitialalgorithms(parts.clone())?;
            cos = translateClassdefConstraints(parts.clone())?;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: cmod.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            scodeCmt = translateCommentList(ann.clone(), cmtString.clone())?;
            decl = translateClassdefExternaldecls(parts.clone())?;
            decl = translateAlternativeExternalAnnotation(decl.clone(), scodeCmt.clone(), info)?;
            (Arc::new(SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod.clone(), composition: Arc::new(SCode::ClassDef::PARTS { elementLst: els.clone(), normalEquationLst: eqs.clone(), initialEquationLst: initeqs.clone(), normalAlgorithmLst: als.clone(), initialAlgorithmLst: initals.clone(), constraintLst: cos.clone(), clsattrs: metamodelica::nil(), externalDecl: decl.clone() }) }), scodeCmt.clone())
        },
        Deref @ Absyn::ClassDef::PDER { functionName: path, vars, comment: cmt } => {
            let mut scodeCmt: Arc<SCode::Comment>;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::PDER { functionPath: path.clone(), derivedVariables: vars.clone() }), scodeCmt.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("AbsynToSCode.translateClassdef failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDef, outComment))
}

fn translateAlternativeExternalAnnotation(mut decl: Option<Arc<SCode::ExternalDecl>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<Option<Arc<SCode::ExternalDecl>>> {
    fn whitelist_mod(mut submod: Arc<SCode::SubMod>) -> bool {
        let mut keep: bool;
        keep = (::match_deref::match_deref! { match &(submod.ident.clone()) {
        Deref @ "Library" => true,
        Deref @ "Include" => true,
        Deref @ "LibraryDirectory" => true,
        Deref @ "SourceDirectory" => true,
        Deref @ "License" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        keep
    }

    let mut outDecl: Option<Arc<SCode::ExternalDecl>>;
    let mut ext_decl: Arc<SCode::ExternalDecl> = Arc::new(<SCode::ExternalDecl as ::std::default::Default>::default());
    let mut ann: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    outDecl = (::match_deref::match_deref! { match &((decl.clone(), comment.clone())) {
        (Some(__esc_ext_decl @ Deref @ SCode::ExternalDecl { annotation_: None, .. }), Deref @ SCode::Comment { annotation_: Some(__esc_ann), .. }) => {
            ext_decl = (*__esc_ext_decl).clone();
            ann = (*__esc_ann).clone();
            assign_field!(ann.modification = SCodeUtil::filterSubMods(ann.modification.clone(), (std::sync::Arc::new(fnptr!(whitelist_mod, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>))?);
            if !(SCodeUtil::isEmptyMod(ann.modification.clone())) {
                if Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())? {
                    Error::addSourceMessage(Error::MISPLACED_EXTERNAL_ANNOTATION.clone(), metamodelica::nil(), info.clone())?;
                }
                assign_field!(ext_decl.annotation_ = Some(ann.clone()));
            }
            Some(ext_decl.clone())
        },
        _ => decl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDecl)
}

fn translateEnumlist(mut inAbsynEnumLiteralLst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Enum>>>> {
    let mut outEnumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
    outEnumLst = (::match_deref::match_deref! { match &(inAbsynEnumLiteralLst) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EnumLiteral { literal: id, comment: cmtOpt }, tail: rest } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut cmt: Arc<SCode::Comment>;
            cmt = translateComment(cmtOpt.clone())?;
            res = translateEnumlist(rest.clone())?;
            metamodelica::cons(Arc::new(SCode::Enum { literal: (id.clone()).clone(), comment: cmt.clone() }), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnumLst)
}

pub fn translateClassdefElements(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: es }, tail: rest } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut es_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            es_1 = translateEitemlist(es.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?;
            els = translateClassdefElements(rest.clone())?;
            return Ok(listAppend(es_1.clone(), els.clone()))
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: es }, tail: rest } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut es_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            es_1 = translateEitemlist(es.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
            els = translateClassdefElements(rest.clone())?;
            return Ok(listAppend(es_1.clone(), els.clone()))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateClassdefEquations(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eql }, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            eql_1 = translateEquations(eql.clone(), false)?;
            eqs = translateClassdefEquations(rest.clone())?;
            return Ok(listAppend(eqs.clone(), eql_1.clone()))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateClassdefInitialequations(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eql }, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            eql_1 = translateEquations(eql.clone(), true)?;
            eqs = translateClassdefInitialequations(rest.clone())?;
            return Ok(listAppend(eqs.clone(), eql_1.clone()))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateClassdefAlgorithms(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: al }, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut al_1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            al_1 = translateClassdefAlgorithmitems(al.clone())?;
            als = translateClassdefAlgorithms(rest.clone())?;
            return Ok(metamodelica::cons(Arc::new(SCode::AlgorithmSection { statements: al_1.clone() }), als.clone()))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- AbsynToSCode.translateClassdefAlgorithms failed\n")).clone())?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateClassdefConstraints(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<SCode::ConstraintSection>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::CONSTRAINTS { contents: consts }, tail: rest } => {
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut cos_1: Arc<metamodelica::List<SCode::ConstraintSection>>;
            cos = translateClassdefConstraints(rest.clone())?;
            return Ok(metamodelica::cons(SCode::ConstraintSection { constraints: consts.clone() }, cos.clone()))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- AbsynToSCode.translateClassdefConstraints failed\n")).clone())?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateClassdefInitialalgorithms(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: al }, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            stmts = translateClassdefAlgorithmitems(al.clone())?;
            als = translateClassdefInitialalgorithms(rest.clone())?;
            return Ok(metamodelica::cons(Arc::new(SCode::AlgorithmSection { statements: stmts.clone() }), als.clone()))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn translateClassdefAlgorithmitems(mut inStatements: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Statement>>>> {
    let mut outStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    outStatements = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut stmt in (inStatements).into_iter().cloned() {
            if !(AbsynUtil::isAlgorithmItem(stmt.clone())) { continue; }
            let __x = translateClassdefAlgorithmItem(stmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outStatements)
}

fn translateClassdefAlgorithmItem(mut inAlgorithm: Arc<Absyn::AlgorithmItem>) -> Result<Arc<SCode::Statement>> {
    let mut outStatement: Arc<SCode::Statement>;
    let mut absynComment: Option<Arc<Absyn::Comment>>;
    let mut comment: Arc<SCode::Comment>;
    let mut info: SourceInfo;
    let mut alg: Arc<Absyn::Algorithm>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inAlgorithm) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa0, comment: __pa1, info: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    alg = __pa0.clone();
    absynComment = __pa1.clone();
    info = __pa2.clone();
    (comment, info) = translateCommentWithLineInfoChanges(absynComment, info)?;
    outStatement = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { .. } => {
            Arc::new(SCode::Statement::ALG_ASSIGN { assignComponent: var_field!((*alg).assignComponent, Absyn::Algorithm::ALG_ASSIGN).clone(), value: var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_IF { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone())?;
            else_body = translateClassdefAlgorithmitems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone())?;
            branches = translateAlgBranches(var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone())?;
            Arc::new(SCode::Statement::ALG_IF { boolExpr: var_field!((*alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), trueBranch: body.clone(), elseIfBranch: branches.clone(), elseBranch: else_body.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_FOR { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone())?;
            for mut i in &*var_field!((*alg).iterators, Absyn::Algorithm::ALG_FOR).clone().reverse() {
                let mut i = i.clone();
                (iter_name, iter_range) = translateIterator(i.clone(), info.clone())?;
                body = list![Arc::new(SCode::Statement::ALG_FOR { index: (iter_name.clone()).clone(), range: iter_range.clone(), forBody: body.clone(), comment: comment.clone(), info: info.clone() })];
            }
            listHead(body.clone())?
        },
        Deref @ Absyn::Algorithm::ALG_PARFOR { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone())?;
            for mut i in &*var_field!((*alg).iterators, Absyn::Algorithm::ALG_PARFOR).clone().reverse() {
                let mut i = i.clone();
                (iter_name, iter_range) = translateIterator(i.clone(), info.clone())?;
                body = list![Arc::new(SCode::Statement::ALG_PARFOR { index: (iter_name.clone()).clone(), range: iter_range.clone(), parforBody: body.clone(), comment: comment.clone(), info: info.clone() })];
            }
            listHead(body.clone())?
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone())?;
            Arc::new(SCode::Statement::ALG_WHILE { boolExpr: var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), whileBody: body.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { .. } => {
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            branches = translateAlgBranches(metamodelica::cons((var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHEN_A).clone(), var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone()), var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone()))?;
            Arc::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: ASSERTION_LEVEL_ERROR.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "level", argValue: e3 }, tail: Deref @ metamodelica::List::Nil } } } => {
            Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "terminate", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Statement::ALG_TERMINATE { message: e1.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "reinit", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Statement::ALG_REINIT { cref: e1.clone(), newValue: e2.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            e1 = Arc::new(Absyn::Exp::CALL { function_: var_field!((*alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), functionArgs: var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), typeVars: metamodelica::nil() });
            Arc::new(SCode::Statement::ALG_NORETCALL { exp: e1.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_FAILURE { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).equ, Absyn::Algorithm::ALG_FAILURE).clone())?;
            Arc::new(SCode::Statement::ALG_FAILURE { stmts: body.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_TRY { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).body, Absyn::Algorithm::ALG_TRY).clone())?;
            else_body = translateClassdefAlgorithmitems(var_field!((*alg).elseBody, Absyn::Algorithm::ALG_TRY).clone())?;
            Arc::new(SCode::Statement::ALG_TRY { body: body.clone(), elseBody: else_body.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_RETURN { .. } => {
            Arc::new(SCode::Statement::ALG_RETURN { comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_BREAK { .. } => {
            Arc::new(SCode::Statement::ALG_BREAK { comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_CONTINUE { .. } => {
            Arc::new(SCode::Statement::ALG_CONTINUE { comment: comment, info: info })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatement)
}

fn translateAlgBranches(mut inBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>> {
    let mut outBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
    let mut condition: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    outBranches = ({
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut branch in (inBranches).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(branch.clone()) {
        (__esc_condition, __esc_body) => {
            condition = (*__esc_condition).clone();
            body = (*__esc_body).clone();
            (condition.clone(), translateClassdefAlgorithmitems(body.clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outBranches)
}

fn translateClassdefExternaldecls(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Option<Arc<SCode::ExternalDecl>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynClassPartLst) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: Deref @ Absyn::ExternalDecl { funcName: fn_name, lang, output_, args, annotation_: aann }, .. }, tail: _ } => {
            let mut sann: Option<Arc<SCode::Annotation>>;
            sann = translateAnnotationOpt(aann.clone())?;
            return Ok(Some(Arc::new(SCode::ExternalDecl { funcName: fn_name.clone(), lang: lang.clone(), output_: output_.clone(), args: args.clone(), annotation_: sann.clone() })))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut res: Option<Arc<SCode::ExternalDecl>>;
            { inAbsynClassPartLst = rest.clone(); continue '__tco; }
        },
        Deref @ metamodelica::List::Nil => {
            return Ok(None)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn translateEitemlist(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inVisibility: SCode::Visibility) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut l: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut es: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = inAbsynElementItemLst.clone();
    let mut ei: Arc<Absyn::ElementItem> = Arc::new(<Absyn::ElementItem as ::std::default::Default>::default());
    let mut e: Arc<Absyn::Element> = Arc::new(<Absyn::Element as ::std::default::Default>::default());
    for mut ei in &*es {
        let mut ei = ei.clone();
        let () = (::match_deref::match_deref! { match &(ei.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: __esc_e } => {
            e = (*__esc_e).clone();
            let mut e_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            e_1 = translateElement(e.clone(), inVisibility.clone())?;
            l = List::append_reverse(e_1.clone(), l.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outElementLst = Dangerous::listReverseInPlace(l);
    Ok(outElementLst)
}

// stefan
pub(crate) fn translateAnnotation(mut inAnnotation: Arc<Absyn::Annotation>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut outAnnotation: Option<Arc<SCode::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(inAnnotation) {
        Deref @ Absyn::Annotation { elementArgs: Deref @ metamodelica::List::Nil } => {
            None
        },
        Deref @ Absyn::Annotation { elementArgs: args } => {
            let mut m: Arc<SCode::Mod>;
            m = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), true)?;
            if (SCodeUtil::isEmptyMod(m.clone())) {None} else {Some(Arc::new(SCode::Annotation { modification: m.clone() }))}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

pub(crate) fn translateAnnotationOpt(mut absynAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut scodeAnnotation: Option<Arc<SCode::Annotation>>;
    scodeAnnotation = (::match_deref::match_deref! { match &(absynAnnotation) {
        Some(ann) => {
            translateAnnotation(ann.clone())?
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(scodeAnnotation)
}

pub fn translateElement(mut inElement: Arc<Absyn::Element>, mut inVisibility: SCode::Visibility) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElementLst = (::match_deref::match_deref! { match &((inElement, inVisibility)) {
        (Deref @ Absyn::Element::ELEMENT { constrainClass: cc, finalPrefix: f, innerOuter: io, redeclareKeywords: repl, specification: s, info }, vis) => {
            let mut es: Arc<metamodelica::List<Arc<SCode::Element>>>;
            es = translateElementspec(cc.clone(), f.clone(), io.clone(), repl.clone(), vis.clone(), s.clone(), info.clone())?;
            es.clone()
        },
        (Deref @ Absyn::Element::DEFINEUNIT { name, args, info }, vis) => {
            let mut expOpt: Option<ArcStr>;
            let mut weightOpt: Option<metamodelica::Real>;
            expOpt = translateDefineunitParam(args.clone(), (literal!("exp")).clone())?;
            weightOpt = translateDefineunitParam2(args.clone(), (literal!("weight")).clone())?;
            list![Arc::new(SCode::Element::DEFINEUNIT { name: (name.clone()).clone(), visibility: vis.clone(), exp: expOpt.clone(), weight: weightOpt.clone(), info: info.clone() })]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementLst)
}

fn translateDefineunitParam(mut inArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inArg: ArcStr) -> Result<Option<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs, inArg)) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: name, argValue: Deref @ Absyn::Exp::STRING { value: r#str } }, tail: _ }, arg) if (name.clone() == arg.clone()) => {
            return Ok(Some((r#str.clone()).clone()))
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(None)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: args }, arg) => {
            { (inArgs, inArg) = (args.clone(), (arg.clone()).clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateDefineunitParam2(mut inArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inArg: ArcStr) -> Result<Option<metamodelica::Real>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs, inArg)) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: name, argValue: Deref @ Absyn::Exp::REAL { value: s } }, tail: _ }, arg) if (name.clone() == arg.clone()) => {
            let mut r: metamodelica::Real;
            r = stringReal((s.clone()).clone())?;
            return Ok(Some(r.clone()))
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(None)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: args }, arg) => {
            { (inArgs, inArg) = (args.clone(), (arg.clone()).clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateElementspec(mut cc: Option<Arc<Absyn::ConstrainClass>>, mut finalPrefix: bool, mut io: Absyn::InnerOuter, mut inRedeclareKeywords: Option<Absyn::RedeclareKeywords>, mut inVisibility: SCode::Visibility, mut inElementSpec4: Arc<Absyn::ElementSpec>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElementLst = (::match_deref::match_deref! { match &((inRedeclareKeywords, inVisibility, inElementSpec4.clone(), inInfo)) {
        (repl, vis, Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: rp, class_: Deref @ Absyn::Class { name: n, partialPrefix: pa, encapsulatedPrefix: e, restriction: Absyn::Restriction::R_OPERATOR { .. }, body: de, info: i, .. } }, _) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut redecl: bool;
            let mut cmt: Arc<SCode::Comment>;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            (de_1, cmt) = translateOperatorDef(de.clone(), (n.clone()).clone(), i.clone())?;
            (_, redecl) = translateRedeclarekeywords(repl.clone());
            sRed = SCodeUtil::boolRedeclare(redecl.clone());
            sFin = SCodeUtil::boolFinal(finalPrefix);
            scc = translateConstrainClass(cc)?;
            sRep = if (rp.clone()) {Arc::new(SCode::Replaceable::REPLACEABLE { cc: scc.clone() })} else {openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()};
            sEnc = SCodeUtil::boolEncapsulated(e.clone());
            sPar = SCodeUtil::boolPartial(pa.clone());
            cls = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: sRed.clone(), finalPrefix: sFin.clone(), innerOuter: io, replaceablePrefix: sRep.clone() }), encapsulatedPrefix: sEnc.clone(), partialPrefix: sPar.clone(), restriction: openmodelica_frontend_types::SCode::Restriction::R_OPERATOR, classDef: de_1.clone(), cmt: cmt.clone(), info: i.clone() });
            list![cls.clone()]
        },
        (repl, vis, Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: rp, class_: cl @ Deref @ Absyn::Class { name: n, partialPrefix: pa, encapsulatedPrefix: e, restriction: re, body: de, info: i, .. } }, _) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut redecl: bool;
            let mut cmt: Arc<SCode::Comment>;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            re_1 = translateRestriction(cl.clone(), re.clone())?;
            (de_1, cmt) = translateClassdef(de.clone(), i.clone(), re_1.clone())?;
            (_, redecl) = translateRedeclarekeywords(repl.clone());
            sRed = SCodeUtil::boolRedeclare(redecl.clone());
            sFin = SCodeUtil::boolFinal(finalPrefix);
            scc = translateConstrainClass(cc)?;
            sRep = if (rp.clone()) {Arc::new(SCode::Replaceable::REPLACEABLE { cc: scc.clone() })} else {openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()};
            sEnc = SCodeUtil::boolEncapsulated(e.clone());
            sPar = SCodeUtil::boolPartial(pa.clone());
            cls = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: sRed.clone(), finalPrefix: sFin.clone(), innerOuter: io, replaceablePrefix: sRep.clone() }), encapsulatedPrefix: sEnc.clone(), partialPrefix: sPar.clone(), restriction: re_1.clone(), classDef: de_1.clone(), cmt: cmt.clone(), info: i.clone() });
            list![cls.clone()]
        },
        (_, vis, Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg: args, annotationOpt: None }, info) => {
            let mut r#mod: Arc<SCode::Mod>;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            list![Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: None, info: info.clone() })]
        },
        (_, vis, Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg: args, annotationOpt: Some(absann) }, info) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            ann = translateAnnotation(absann.clone())?;
            list![Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() })]
        },
        (_, _, Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::List::Nil, .. }, _) => {
            metamodelica::nil()
        },
        (repl, vis, Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: fl, streamPrefix: st, parallelism, variability, direction: di, isField: isf, arrayDim: ad }, typeSpec: t, .. }, info) => {
            let mut repl_1: bool;
            let mut redecl: bool;
            let mut n: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            let mut info = (*info).clone();
            xs_1 = metamodelica::nil();
            for mut comp in &*var_field!((*inElementSpec4).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let mut comp = comp.clone();
                let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ Absyn::ComponentItem { component: Absyn::Component { name: __pa0, arrayDim: __pa1, modification: __pa2 }, comment: __pa3, condition: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                n = __pa0.clone();
                d = __pa1.clone();
                m = __pa2.clone();
                comment = __pa3.clone();
                cond = __pa4.clone();
                checkTypeSpec(t.clone(), info.clone())?;
                setHasInnerOuterDefinitionsHandler(io.clone());
                setHasStreamConnectorsHandler(st.clone())?;
                r#mod = translateMod(m.clone(), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                prl1 = translateParallelism(parallelism.clone())?;
                var1 = translateVariability(variability.clone())?;
                tot_dim = listAppend(d.clone(), ad.clone());
                (repl_1, redecl) = translateRedeclarekeywords(repl.clone());
                (cmt, info) = translateCommentWithLineInfoChanges(comment.clone(), info.clone())?;
                sFin = SCodeUtil::boolFinal(finalPrefix);
                sRed = SCodeUtil::boolRedeclare(redecl.clone());
                scc = translateConstrainClass(cc.clone())?;
                sRep = if (repl_1.clone()) {Arc::new(SCode::Replaceable::REPLACEABLE { cc: scc.clone() })} else {openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()};
                ct = translateConnectorType(fl.clone(), st.clone())?;
                prefixes = Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: sRed.clone(), finalPrefix: sFin.clone(), innerOuter: io.clone(), replaceablePrefix: sRep.clone() });
                xs_1 = (match di.clone() {
        Absyn::Direction::INPUT_OUTPUT { .. } if (!(Flags::isSet(Flags::SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR.clone())?)) => {
            let mut attr1: SCode::Attributes;
            let mut attr2: SCode::Attributes;
            let mut mod2: Arc<SCode::Mod>;
            let mut inName: ArcStr;
            inName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$in_")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
            attr1 = SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct.clone(), parallelism: prl1.clone(), variability: var1.clone(), direction: openmodelica_ast::Absyn::Direction::INPUT, isField: isf.clone() };
            attr2 = SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct.clone(), parallelism: prl1.clone(), variability: var1.clone(), direction: openmodelica_ast::Absyn::Direction::OUTPUT, isField: isf.clone() };
            mod2 = Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (inName.clone()).clone(), subscripts: metamodelica::nil() }) })), comment: None, info: info.clone() });
            metamodelica::cons(Arc::new(SCode::Element::COMPONENT { name: (n.clone()).clone(), prefixes: prefixes.clone(), attributes: attr2.clone(), typeSpec: t.clone(), modifications: mod2.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), metamodelica::cons(Arc::new(SCode::Element::COMPONENT { name: (inName.clone()).clone(), prefixes: prefixes.clone(), attributes: attr1.clone(), typeSpec: t.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), xs_1.clone()))
        },
        _ => {
            metamodelica::cons(Arc::new(SCode::Element::COMPONENT { name: (n.clone()).clone(), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct.clone(), parallelism: prl1.clone(), variability: var1.clone(), direction: di.clone(), isField: isf.clone() }, typeSpec: t.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), xs_1.clone())
        },
    });
            }
            xs_1 = Dangerous::listReverseInPlace(xs_1.clone());
            xs_1.clone()
        },
        (_, vis, Deref @ Absyn::ElementSpec::IMPORT { import_: imp, info, .. }, _) => {
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            xs_1 = translateImports(imp.clone(), vis.clone(), info.clone())?;
            xs_1.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("AbsynToSCode.translateElementspec failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElementLst)
}

fn translateImports(mut imp: Absyn::Import, mut visibility: SCode::Visibility, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(imp.clone()) {
        Absyn::Import::NAMED_IMPORT { name, path: Deref @ Absyn::Path::FULLYQUALIFIED { path: p } } => {
            { (imp, visibility, info) = (Absyn::Import::NAMED_IMPORT { name: (name.clone()).clone(), path: p.clone() }, visibility, info); continue '__tco; }
        },
        Absyn::Import::QUAL_IMPORT { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: p } } => {
            { (imp, visibility, info) = (Absyn::Import::QUAL_IMPORT { path: p.clone() }, visibility, info); continue '__tco; }
        },
        Absyn::Import::UNQUAL_IMPORT { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: p } } => {
            { (imp, visibility, info) = (Absyn::Import::UNQUAL_IMPORT { path: p.clone() }, visibility, info); continue '__tco; }
        },
        Absyn::Import::GROUP_IMPORT { prefix: p, groups } => {
            return Ok(List::map3(groups.clone(), (std::sync::Arc::new(translateGroupImport) as std::sync::Arc<dyn ::std::ops::Fn(Absyn::GroupImport, Arc<Absyn::Path>, SCode::Visibility, SourceInfo) -> Result<Arc<SCode::Element>> + 'static>), p.clone(), visibility, info)?)
        },
        _ => {
            return Ok(list![Arc::new(SCode::Element::IMPORT { imp: imp, visibility: visibility, info: info })])
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn translateGroupImport(mut gimp: Absyn::GroupImport, mut prefix: Arc<Absyn::Path>, mut visibility: SCode::Visibility, mut info: SourceInfo) -> Result<Arc<SCode::Element>> {
    let mut elt: Arc<SCode::Element>;
    elt = (match (gimp, visibility) {
        (Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut name }, mut vis) => {
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::joinPaths(prefix, Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            Arc::new(SCode::Element::IMPORT { imp: Absyn::Import::QUAL_IMPORT { path: path.clone() }, visibility: vis.clone(), info: info })
        },
        (Absyn::GroupImport::GROUP_IMPORT_RENAME { rename: mut rename, name: mut name }, mut vis) => {
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::joinPaths(prefix, Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            Arc::new(SCode::Element::IMPORT { imp: Absyn::Import::NAMED_IMPORT { name: (rename.clone()).clone(), path: path.clone() }, visibility: vis.clone(), info: info })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(elt)
}

fn setHasInnerOuterDefinitionsHandler(mut io: Absyn::InnerOuter) -> () {
    let () = (match io {
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => (),
        _ => {
            System::setHasInnerOuterDefinitions(true);
            ()
        },
    });
    ()
}

fn setHasStreamConnectorsHandler(mut streamPrefix: bool) -> Result<()> {
    let () = (match streamPrefix {
        false => (),
        true => {
            System::setHasStreamConnectors(true);
            ()
        },
    });
    Ok(())
}

fn translateRedeclarekeywords(mut inRedeclKeywords: Option<Absyn::RedeclareKeywords>) -> (bool, bool) {
    let mut outIsReplaceable: bool;
    let mut outIsRedeclared: bool;
    (outIsReplaceable, outIsRedeclared) = (match inRedeclKeywords {
        Some(Absyn::RedeclareKeywords::REDECLARE { .. }) => (false, true),
        Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }) => (true, false),
        Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }) => (true, true),
        _ => (false, false),
    });
    (outIsReplaceable, outIsRedeclared)
}

fn translateConstrainClass(mut inConstrainClass: Option<Arc<Absyn::ConstrainClass>>) -> Result<Option<Arc<SCode::ConstrainClass>>> {
    let mut outConstrainClass: Option<Arc<SCode::ConstrainClass>>;
    outConstrainClass = (::match_deref::match_deref! { match &(inConstrainClass) {
        Some(Deref @ Absyn::ConstrainClass { elementSpec: Deref @ Absyn::ElementSpec::EXTENDS { path: cc_path, elementArg: eltargs, .. }, comment: cmt }) => {
            let mut cc_cmt: Arc<SCode::Comment>;
            let mut r#mod: Arc<Absyn::Modification>;
            let mut cc_mod: Arc<SCode::Mod>;
            r#mod = Arc::new(Absyn::Modification { elementArgLst: eltargs.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() });
            cc_mod = translateMod(Some(r#mod.clone()), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            cc_cmt = translateComment(cmt.clone())?;
            Some(Arc::new(SCode::ConstrainClass { constrainingClass: cc_path.clone(), modifier: cc_mod.clone(), comment: cc_cmt.clone() }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outConstrainClass)
}

fn translateParallelism(mut inParallelism: Absyn::Parallelism) -> Result<SCode::Parallelism> {
    let mut outParallelism: SCode::Parallelism;
    outParallelism = (match inParallelism {
        Absyn::Parallelism::PARGLOBAL { .. } => openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL,
        Absyn::Parallelism::PARLOCAL { .. } => openmodelica_frontend_types::SCode::Parallelism::PARLOCAL,
        Absyn::Parallelism::NON_PARALLEL { .. } => openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
    });
    Ok(outParallelism)
}

fn translateVariability(mut inVariability: Absyn::Variability) -> Result<SCode::Variability> {
    let mut outVariability: SCode::Variability;
    outVariability = (match inVariability {
        Absyn::Variability::VAR { .. } => openmodelica_frontend_types::SCode::Variability::VAR,
        Absyn::Variability::DISCRETE { .. } => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        Absyn::Variability::PARAM { .. } => openmodelica_frontend_types::SCode::Variability::PARAM,
        Absyn::Variability::CONST { .. } => openmodelica_frontend_types::SCode::Variability::CONST,
    });
    Ok(outVariability)
}

fn translateEquations(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inIsInitial: bool) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    outEquationLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut eq in (inAbsynEquationItemLst).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { .. } => {
            let mut com: Arc<SCode::Comment>;
            let mut info: SourceInfo;
            (com, info) = translateCommentWithLineInfoChanges(var_field!((*eq).comment, Absyn::EquationItem::EQUATIONITEM).clone(), var_field!((*eq).info, Absyn::EquationItem::EQUATIONITEM).clone())?;
            translateEquation(var_field!((*eq).equation_, Absyn::EquationItem::EQUATIONITEM).clone(), com.clone(), info.clone(), inIsInitial)?
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEquationLst)
}

fn translateCommentWithLineInfoChanges(mut inComment: Option<Arc<Absyn::Comment>>, mut inInfo: SourceInfo) -> Result<(Arc<SCode::Comment>, SourceInfo)> {
    let mut outComment: Arc<SCode::Comment>;
    let mut outInfo: SourceInfo;
    outComment = translateComment(inComment)?;
    outInfo = getInfoAnnotationOrDefault(outComment.clone(), inInfo);
    Ok((outComment, outInfo))
}

fn getInfoAnnotationOrDefault(mut comment: Arc<SCode::Comment>, mut default: SourceInfo) -> SourceInfo {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match &(comment) {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: lst, .. } }), .. } => {
            getInfoAnnotationOrDefault2(lst.clone(), default)
        },
        _ => {
            default
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    info
}

fn getInfoAnnotationOrDefault2(mut lst: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut default: SourceInfo) -> SourceInfo {
    '__tco: loop {
        ::match_deref::match_deref! { match &(lst) {
        Deref @ metamodelica::List::Nil => {
            return default
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_FileInfo", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::STRING { value: fileName }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::INTEGER { value: line }, tail: Deref @ metamodelica::List::Nil } } }), .. } }, tail: _ } => {
            return SourceInfo { fileName: (fileName.clone()).clone(), isReadOnly: false, lineNumberStart: line.clone(), columnNumberStart: 0, lineNumberEnd: line.clone(), columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) }
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            { (lst, default) = (rest.clone(), default); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn translateComment(mut inComment: Option<Arc<Absyn::Comment>>) -> Result<Arc<SCode::Comment>> {
    let mut outComment: Arc<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match &(inComment) {
        None => {
            SCode::noComment.clone()
        },
        Some(Deref @ Absyn::Comment { annotation_: absann, comment: ostr }) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut ostr = (*ostr).clone();
            ann = translateAnnotationOpt(absann.clone())?;
            ostr = Util::applyOption(ostr.clone(), (std::sync::Arc::new(fnptr!(System::unescapedString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))?;
            Arc::new(SCode::Comment { annotation_: ann.clone(), comment: ostr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComment)
}

fn translateCommentList(mut inAnns: Arc<metamodelica::List<Arc<Absyn::Annotation>>>, mut inString: Option<ArcStr>) -> Result<Arc<SCode::Comment>> {
    let mut outComment: Arc<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match &(inAnns) {
        Deref @ metamodelica::List::Nil => {
            Arc::new(SCode::Comment { annotation_: None, comment: inString })
        },
        Deref @ metamodelica::List::Cons { head: absann, tail: Deref @ metamodelica::List::Nil } => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut ostr: Option<ArcStr>;
            ann = translateAnnotation(absann.clone())?;
            ostr = Util::applyOption(inString, (std::sync::Arc::new(fnptr!(System::unescapedString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))?;
            Arc::new(SCode::Comment { annotation_: ann.clone(), comment: ostr.clone() })
        },
        Deref @ metamodelica::List::Cons { head: absann, tail: anns } => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut ostr: Option<ArcStr>;
            let mut absann = (*absann).clone();
            absann = AbsynUtil::mergeAnnotationsList(absann.clone(), anns.clone())?;
            ann = translateAnnotation(absann.clone())?;
            ostr = Util::applyOption(inString, (std::sync::Arc::new(fnptr!(System::unescapedString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))?;
            Arc::new(SCode::Comment { annotation_: ann.clone(), comment: ostr.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

fn translateCommentSeparate(mut inComment: Option<Arc<Absyn::Comment>>) -> Result<(Option<Arc<SCode::Annotation>>, Option<ArcStr>)> {
    let mut outAnn: Option<Arc<SCode::Annotation>>;
    let mut outStr: Option<ArcStr>;
    (outAnn, outStr) = (::match_deref::match_deref! { match &(inComment) {
        None => {
            (None, None)
        },
        Some(Deref @ Absyn::Comment { annotation_: None, comment: None }) => {
            (None, None)
        },
        Some(Deref @ Absyn::Comment { annotation_: None, comment: Some(r#str) }) => {
            (None, Some((r#str.clone()).clone()))
        },
        Some(Deref @ Absyn::Comment { annotation_: Some(absann), comment: None }) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            ann = translateAnnotation(absann.clone())?;
            (ann.clone(), None)
        },
        Some(Deref @ Absyn::Comment { annotation_: Some(absann), comment: Some(r#str) }) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            ann = translateAnnotation(absann.clone())?;
            (ann.clone(), Some((r#str.clone()).clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outAnn, outStr))
}

fn translateEquation(mut inEquation: Arc<Absyn::Equation>, mut inComment: Arc<SCode::Comment>, mut inInfo: SourceInfo, mut inIsInitial: bool) -> Result<Arc<SCode::Equation>> {
    let mut outEquation: Arc<SCode::Equation>;
    outEquation = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ Absyn::Equation::EQ_IF { .. } => {
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut bodies: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            body = translateEquations(var_field!((*inEquation).equationTrueItems, Absyn::Equation::EQ_IF).clone(), inIsInitial)?;
            (conditions, bodies) = List::map1_2(var_field!((*inEquation).elseIfBranches, Absyn::Equation::EQ_IF).clone(), (std::sync::Arc::new(translateEqBranch) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)> + 'static>), inIsInitial)?;
            conditions = metamodelica::cons(var_field!((*inEquation).ifExp, Absyn::Equation::EQ_IF).clone(), conditions.clone());
            else_branch = translateEquations(var_field!((*inEquation).equationElseItems, Absyn::Equation::EQ_IF).clone(), inIsInitial)?;
            Arc::new(SCode::Equation::EQ_IF { condition: conditions.clone(), thenBranch: metamodelica::cons(body.clone(), bodies.clone()), elseBranch: else_branch.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut bodies: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            body = translateEquations(var_field!((*inEquation).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), inIsInitial)?;
            (conditions, bodies) = List::map1_2(var_field!((*inEquation).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone(), (std::sync::Arc::new(translateEqBranch) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)> + 'static>), inIsInitial)?;
            branches = ({
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        let __thr_src0 = conditions.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = bodies.clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(c), Some(b)) => {
                    let __x = (c.clone(), b.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
            Arc::new(SCode::Equation::EQ_WHEN { condition: var_field!((*inEquation).whenExp, Absyn::Equation::EQ_WHEN_E).clone(), eEquationLst: body.clone(), elseBranches: branches.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_EQUALS { .. } => {
            Arc::new(SCode::Equation::EQ_EQUALS { expLeft: var_field!((*inEquation).leftSide, Absyn::Equation::EQ_EQUALS).clone(), expRight: var_field!((*inEquation).rightSide, Absyn::Equation::EQ_EQUALS).clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_PDE { .. } => {
            Arc::new(SCode::Equation::EQ_PDE { expLeft: var_field!((*inEquation).leftSide, Absyn::Equation::EQ_PDE).clone(), expRight: var_field!((*inEquation).rightSide, Absyn::Equation::EQ_PDE).clone(), domain: var_field!((*inEquation).domain, Absyn::Equation::EQ_PDE).clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_CONNECT { .. } => {
            Arc::new(SCode::Equation::EQ_CONNECT { crefLeft: var_field!((*inEquation).connector1, Absyn::Equation::EQ_CONNECT).clone(), crefRight: var_field!((*inEquation).connector2, Absyn::Equation::EQ_CONNECT).clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_FOR { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            body = translateEquations(var_field!((*inEquation).forEquations, Absyn::Equation::EQ_FOR).clone(), inIsInitial)?;
            for mut i in &*var_field!((*inEquation).iterators, Absyn::Equation::EQ_FOR).clone().reverse() {
                let mut i = i.clone();
                (iter_name, iter_range) = translateIterator(i.clone(), inInfo.clone())?;
                body = list![Arc::new(SCode::Equation::EQ_FOR { index: (iter_name.clone()).clone(), range: iter_range.clone(), eEquationLst: body.clone(), comment: inComment.clone(), info: inInfo.clone() })];
            }
            listHead(body.clone())?
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: ASSERTION_LEVEL_ERROR.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "level", argValue: e3 }, tail: Deref @ metamodelica::List::Nil } } } => {
            Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "terminate", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Equation::EQ_TERMINATE { message: e1.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "reinit", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil } } => {
            Arc::new(SCode::Equation::EQ_REINIT { cref: e1.clone(), expReinit: e2.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { .. } => {
            Arc::new(SCode::Equation::EQ_NORETCALL { exp: Arc::new(Absyn::Exp::CALL { function_: var_field!((*inEquation).functionName, Absyn::Equation::EQ_NORETCALL).clone(), functionArgs: var_field!((*inEquation).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), typeVars: metamodelica::nil() }), comment: inComment, info: inInfo })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEquation)
}

fn translateEqBranch(mut inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), mut inIsInitial: bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)> {
    let mut outCondition: Arc<Absyn::Exp>;
    let mut outBody: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut body: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    (outCondition, body) = inBranch;
    outBody = translateEquations(body, inIsInitial)?;
    Ok((outCondition, outBody))
}

fn translateIterator(mut inIterator: Arc<Absyn::ForIterator>, mut inInfo: SourceInfo) -> Result<(ArcStr, Option<Arc<Absyn::Exp>>)> {
    let mut outName: ArcStr;
    let mut outRange: Option<Arc<Absyn::Exp>>;
    let mut guard_exp: Option<Arc<Absyn::Exp>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inIterator) {
        Deref @ Absyn::ForIterator { name: __pa0, guardExp: __pa1, range: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    guard_exp = __pa1.clone();
    outRange = __pa2.clone();
    if isSome(guard_exp) {
        Error::addSourceMessageAndFail(Error::INTERNAL_ERROR.clone(), list![(literal!("For loops with guards not yet implemented")).clone()], inInfo)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok((outName, outRange))
}

fn translateElementAddinfo(mut elem: Arc<SCode::Element>, mut nfo: SourceInfo) -> Arc<SCode::Element> {
    let mut oelem: Arc<SCode::Element>;
    oelem = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::COMPONENT { name: a1, prefixes: p, attributes: a6, typeSpec: a7, modifications: a8, comment: a10, condition: a11, info: _ } => {
            Arc::new(SCode::Element::COMPONENT { name: (a1.clone()).clone(), prefixes: p.clone(), attributes: a6.clone(), typeSpec: a7.clone(), modifications: a8.clone(), comment: a10.clone(), condition: a11.clone(), info: nfo })
        },
        _ => {
            elem
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oelem
}

/* Modification management */
pub fn translateMod(mut inMod: Option<Arc<Absyn::Modification>>, mut finalPrefix: SCode::Final, mut eachPrefix: SCode::Each, mut comment: Option<ArcStr>, mut info: SourceInfo, mut keepEmpty: bool) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut eqmod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut subs: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    let mut binding: Option<Arc<Absyn::Exp>>;
    (args, eqmod) = (::match_deref::match_deref! { match &(inMod) {
        Some(Deref @ Absyn::Modification { elementArgLst: __esc_args, eqMod: __esc_eqmod }) => {
            args = (*__esc_args).clone();
            eqmod = (*__esc_eqmod).clone();
            (args.clone(), eqmod.clone())
        },
        _ => (metamodelica::nil(), openmodelica_ast::Absyn::EqMod::interned_NOMOD()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subs = if (args.clone().is_empty()) {metamodelica::nil()} else {translateArgs(args, keepEmpty)?};
    binding = (::match_deref::match_deref! { match &(eqmod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => Some(var_field!((*eqmod).exp, Absyn::EqMod::EQMOD).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod = (::match_deref::match_deref! { match &((subs.clone(), binding.clone(), finalPrefix.clone(), eachPrefix.clone())) {
        (Deref @ metamodelica::List::Nil, None, SCode::Final::NOT_FINAL { .. }, SCode::Each::NOT_EACH { .. }) => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        _ => Arc::new(SCode::Mod::MOD { finalPrefix: finalPrefix, eachPrefix: eachPrefix, subModLst: subs, binding: binding, comment: comment, info: info }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn translateArgs(mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut keepEmpty: bool) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut elem: Arc<SCode::Element> = Arc::new(<SCode::Element as ::std::default::Default>::default());
    let mut sub: Arc<SCode::SubMod> = Arc::new(<SCode::SubMod as ::std::default::Default>::default());
    let mut cr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut cr2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut name: ArcStr = arcstr::literal!("");
    for mut arg in &*args {
        let mut arg = arg.clone();
        subMods = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            smod = translateMod(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone(), SCodeUtil::boolFinal(var_field!((*arg).finalPrefix, Absyn::ElementArg::MODIFICATION).clone()), translateEach(var_field!((*arg).eachPrefix, Absyn::ElementArg::MODIFICATION).clone())?, var_field!((*arg).comment, Absyn::ElementArg::MODIFICATION).clone(), var_field!((*arg).info, Absyn::ElementArg::MODIFICATION).clone(), false)?;
            if !(SCodeUtil::isEmptyMod(smod.clone())) || keepEmpty {
                sub = translateSub(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), smod.clone(), var_field!((*arg).info, Absyn::ElementArg::MODIFICATION).clone())?;
                subMods = metamodelica::cons(sub.clone(), subMods.clone());
            }
            subMods.clone()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(translateElementspec(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), var_field!((*arg).finalPrefix, Absyn::ElementArg::REDECLARATION).clone(), openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, Some(var_field!((*arg).redeclareKeywords, Absyn::ElementArg::REDECLARATION).clone()), openmodelica_frontend_types::SCode::Visibility::PUBLIC, var_field!((*arg).elementSpec, Absyn::ElementArg::REDECLARATION).clone(), var_field!((*arg).info, Absyn::ElementArg::REDECLARATION).clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elem = __pa0.clone();
            sub = Arc::new(SCode::SubMod { ident: (AbsynUtil::elementSpecName(var_field!((*arg).elementSpec, Absyn::ElementArg::REDECLARATION).clone())?).clone(), r#mod: Arc::new(SCode::Mod::REDECL { finalPrefix: SCodeUtil::boolFinal(var_field!((*arg).finalPrefix, Absyn::ElementArg::REDECLARATION).clone()), eachPrefix: translateEach(var_field!((*arg).eachPrefix, Absyn::ElementArg::REDECLARATION).clone())?, element: elem.clone() }) });
            metamodelica::cons(sub.clone(), subMods.clone())
        },
        Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => subMods.clone(),
        Deref @ Absyn::ElementArg::INHERITANCEBREAK { cnct: Deref @ Absyn::Equation::EQ_CONNECT { connector1: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "break", .. }, connector2: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, .. } }, .. } => {
            name = (*__esc_name).clone();
            metamodelica::cons(Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: Arc::new(SCode::Mod::BREAK_COMPONENT { info: var_field!((*arg).info, Absyn::ElementArg::INHERITANCEBREAK).clone() }) }), subMods.clone())
        },
        Deref @ Absyn::ElementArg::INHERITANCEBREAK { cnct: Deref @ Absyn::Equation::EQ_CONNECT { connector1: __esc_cr1, connector2: __esc_cr2 }, .. } => {
            cr1 = (*__esc_cr1).clone();
            cr2 = (*__esc_cr2).clone();
            metamodelica::cons(Arc::new(SCode::SubMod { ident: (literal!("")).clone(), r#mod: Arc::new(SCode::Mod::BREAK_CONNECT { lhs: cr1.clone(), rhs: cr2.clone(), info: var_field!((*arg).info, Absyn::ElementArg::INHERITANCEBREAK).clone() }) }), subMods.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    subMods = subMods.reverse();
    Ok(subMods)
}

fn translateSub(mut inPath: Arc<Absyn::Path>, mut inMod: Arc<SCode::Mod>, mut info: SourceInfo) -> Result<Arc<SCode::SubMod>> {
    let mut outSubMod: Arc<SCode::SubMod>;
    outSubMod = (::match_deref::match_deref! { match &(inPath) {
        Deref @ Absyn::Path::IDENT { name: i } => {
            Arc::new(SCode::SubMod { ident: (i.clone()).clone(), r#mod: inMod })
        },
        Deref @ Absyn::Path::QUALIFIED { name: i, path } => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut sub: Arc<SCode::SubMod>;
            sub = translateSub(path.clone(), inMod, info.clone())?;
            r#mod = Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![sub.clone()], binding: None, comment: None, info: info });
            Arc::new(SCode::SubMod { ident: (i.clone()).clone(), r#mod: r#mod.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMod)
}

fn makeTypeVarElement(mut r#str: ArcStr, mut info: SourceInfo) -> Arc<SCode::Element> {
    let mut elt: Arc<SCode::Element>;
    let mut cd: Arc<SCode::ClassDef>;
    let mut ts: Arc<Absyn::TypeSpec>;
    ts = Arc::new(Absyn::TypeSpec::TCOMPLEX { path: Arc::new(Absyn::Path::IDENT { name: (literal!("polymorphic")).clone() }), typeSpecs: list![Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Any")).clone() }), arrayDim: None })], arrayDim: None });
    cd = Arc::new(SCode::ClassDef::DERIVED { typeSpec: ts, modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });
    elt = Arc::new(SCode::Element::CLASS { name: (r#str).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE() }), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE, classDef: cd, cmt: SCode::noComment.clone(), info: info });
    elt
}

fn translateEach(mut inAEach: Absyn::Each) -> Result<SCode::Each> {
    let mut outSEach: SCode::Each;
    outSEach = (match inAEach {
        Absyn::Each::EACH { .. } => openmodelica_frontend_types::SCode::Each::EACH,
        Absyn::Each::NON_EACH { .. } => openmodelica_frontend_types::SCode::Each::NOT_EACH,
    });
    Ok(outSEach)
}

fn checkTypeSpec(mut ts: Arc<Absyn::TypeSpec>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ts.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tuple" }, typeSpecs: Deref @ metamodelica::List::Cons { head: ts2, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut r#str: ArcStr;
            r#str = (AbsynUtil::typeSpecString(ts)?).clone();
            Error::addSourceMessage(Error::TCOMPLEX_TUPLE_ONE_NAME.clone(), list![(r#str.clone()).clone()], info.clone())?;
            checkTypeSpec(ts2.clone(), info)?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tuple" }, typeSpecs: tss @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. } => {
            List::map1_0(tss.clone(), (std::sync::Arc::new(checkTypeSpec) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::TypeSpec>, SourceInfo) -> Result<()> + 'static>), info)?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: Deref @ metamodelica::List::Cons { head: ts2, tail: Deref @ metamodelica::List::Nil }, .. } => {
            checkTypeSpec(ts2.clone(), info)?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: tss, .. } => {
            let mut r#str: ArcStr;
            if listMember(var_field!((*ts).path, Absyn::TypeSpec::TCOMPLEX).clone(), list![Arc::new(Absyn::Path::IDENT { name: (literal!("list")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("List")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Array")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("polymorphic")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Option")).clone() })]) {
                r#str = (AbsynUtil::typeSpecString(ts)?).clone();
                Error::addSourceMessage(Error::TCOMPLEX_MULTIPLE_NAMES.clone(), list![(r#str.clone()).clone()], info.clone())?;
                List::map1_0(tss.clone(), (std::sync::Arc::new(checkTypeSpec) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::TypeSpec>, SourceInfo) -> Result<()> + 'static>), info)?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

