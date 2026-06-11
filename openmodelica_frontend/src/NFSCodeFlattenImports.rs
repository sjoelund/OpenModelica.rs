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

use crate::NFSCodeEnv;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub type Env = Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>;

pub type Item = Arc<NFSCodeEnv::Item>;

pub type Extends = Arc<NFSCodeEnv::Extends>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub(crate) fn flattenProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inEnv: Env) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Env)> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut outEnv: Env;
    (outProgram, outEnv) = List::mapFold(inProgram, (std::sync::Arc::new(flattenClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv)?;
    Ok((outProgram, outEnv))
}

pub(crate) fn flattenClass(mut inClass: Arc<SCode::Element>, mut inEnv: Env) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outClass: Arc<SCode::Element>;
    let mut outEnv: Env;
    (outClass, outEnv) = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: cdef, info, .. } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut cls_env: Arc<NFSCodeEnv::Frame>;
                    let mut cls: Arc<SCode::Element>;
                    let mut cls_ty: NFSCodeEnv::ClassType;
                    let mut cdef = (*cdef).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupInClass((name.clone()).clone(), inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, classType: __pa1, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls_env = __pa0.clone();
                    cls_ty = __pa1.clone();
                    env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv.clone());
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(flattenClassDef(cdef.clone(), env.clone(), info.clone())?) {
                        (__pa3, Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cdef = __pa3.clone();
                    cls_env = __pa4.clone();
                    env = __pa5.clone();
                    cls = SCodeUtil::setClassDef(cdef.clone(), inClass.clone())?;
                    item = NFSCodeEnv::newClassItem(cls.clone(), list![cls_env.clone()], cls_ty.clone());
                    env = NFSCodeEnv::updateItemInEnv(item.clone(), env.clone(), (name.clone()).clone())?;
                    Ok((cls.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenImports.flattenClass failed on ")); __mm_s.push_str(&*SCodeUtil::elementName(inClass.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outClass, outEnv))
}

fn flattenClassDef(mut inClassDef: Arc<SCode::ClassDef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<(Arc<SCode::ClassDef>, Env)> {
    let mut outClassDef: Arc<SCode::ClassDef>;
    let mut outEnv: Env;
    (outClassDef, outEnv) = (::match_deref::match_deref! { match &((inClassDef.clone(), inEnv.clone())) {
        (Deref @ SCode::ClassDef::PARTS { elementLst: el, normalEquationLst: neql, initialEquationLst: ieql, normalAlgorithmLst: nal, initialAlgorithmLst: ial, constraintLst: nco, clsattrs: clats, externalDecl: extdecl }, _) => {
            let mut env: Env;
            let mut el = (*el).clone();
            let mut neql = (*neql).clone();
            let mut ieql = (*ieql).clone();
            let mut nal = (*nal).clone();
            let mut ial = (*ial).clone();
            let mut nco = (*nco).clone();
            el = List::filterOnTrue(el.clone(), (std::sync::Arc::new(fnptr!(isNotImport, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>))?;
            (el, env) = List::mapFold(el.clone(), (std::sync::Arc::new(flattenElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Element>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv)?;
            neql = List::map1(neql.clone(), (std::sync::Arc::new(flattenEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::Equation>> + 'static>), env.clone())?;
            ieql = List::map1(ieql.clone(), (std::sync::Arc::new(flattenEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::Equation>> + 'static>), env.clone())?;
            nal = List::map1(nal.clone(), (std::sync::Arc::new(flattenAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>), env.clone())?;
            ial = List::map1(ial.clone(), (std::sync::Arc::new(flattenAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>), env.clone())?;
            nco = List::map2(nco.clone(), (std::sync::Arc::new(flattenConstraints) as std::sync::Arc<dyn ::std::ops::Fn(SCode::ConstraintSection, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<SCode::ConstraintSection> + 'static>), env.clone(), inInfo)?;
            (Arc::new(SCode::ClassDef::PARTS { elementLst: el.clone(), normalEquationLst: neql.clone(), initialEquationLst: ieql.clone(), normalAlgorithmLst: nal.clone(), initialAlgorithmLst: ial.clone(), constraintLst: nco.clone(), clsattrs: clats.clone(), externalDecl: extdecl.clone() }), env.clone())
        },
        (Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: mods, composition: cdef }, _) => {
            let mut env: Env;
            let mut mods = (*mods).clone();
            let mut cdef = (*cdef).clone();
            (cdef, env) = flattenClassDef(cdef.clone(), inEnv, inInfo.clone())?;
            mods = flattenModifier(mods.clone(), env.clone(), inInfo)?;
            (Arc::new(SCode::ClassDef::CLASS_EXTENDS { modifications: mods.clone(), composition: cdef.clone() }), env.clone())
        },
        (Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, modifications: mods, attributes: attr }, env) => {
            let mut ty = (*ty).clone();
            let mut mods = (*mods).clone();
            let mut env = (*env).clone();
            mods = flattenModifier(mods.clone(), env.clone(), inInfo.clone())?;
            env = NFSCodeEnv::removeExtendsFromLocalScope(env.clone())?;
            ty = flattenTypeSpec(ty.clone(), env.clone(), inInfo)?;
            (Arc::new(SCode::ClassDef::DERIVED { typeSpec: ty.clone(), modifications: mods.clone(), attributes: attr.clone() }), inEnv)
        },
        _ => {
            (inClassDef, inEnv)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDef, outEnv))
}

fn flattenDerivedClassDef(mut inClassDef: Arc<SCode::ClassDef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<SCode::ClassDef>> {
    let mut outClassDef: Arc<SCode::ClassDef>;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut mods: Arc<SCode::Mod>;
    let mut attr: SCode::Attributes;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inClassDef) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa0, modifications: __pa1, attributes: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    mods = __pa1.clone();
    attr = __pa2.clone();
    ty = flattenTypeSpec(ty, inEnv.clone(), inInfo.clone())?;
    mods = flattenModifier(mods, inEnv, inInfo)?;
    outClassDef = Arc::new(SCode::ClassDef::DERIVED { typeSpec: ty, modifications: mods, attributes: attr });
    Ok(outClassDef)
}

fn isNotImport(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inElement) {
        Deref @ SCode::Element::IMPORT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn flattenElement(mut inElement: Arc<SCode::Element>, mut inEnv: Env) -> Result<(Arc<SCode::Element>, Env)> {
    let mut outElement: Arc<SCode::Element>;
    let mut outEnv: Env;
    (outElement, outEnv) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name, .. } => {
            let mut env: Env;
            let mut elem: Arc<SCode::Element>;
            let mut item: Item;
            elem = flattenComponent(inElement, inEnv.clone())?;
            item = NFSCodeEnv::newVarItem(elem.clone(), true);
            env = NFSCodeEnv::updateItemInEnv(item, inEnv, (name.clone()).clone())?;
            (elem, env)
        },
        Deref @ SCode::Element::CLASS { .. } => {
            let mut env: Env;
            let mut elem: Arc<SCode::Element>;
            (elem, env) = flattenClass(inElement, inEnv)?;
            (elem, env)
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            (flattenExtends(inElement, inEnv.clone())?, inEnv)
        },
        _ => {
            (inElement, inEnv)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outElement, outEnv))
}

fn flattenComponent(mut inComponent: Arc<SCode::Element>, mut inEnv: Env) -> Result<Arc<SCode::Element>> {
    let mut outComponent: Arc<SCode::Element>;
    let mut name: ArcStr;
    let mut prefixes: Arc<SCode::Prefixes>;
    let mut attr: SCode::Attributes;
    let mut type_spec: Arc<Absyn::TypeSpec>;
    let mut r#mod: Arc<SCode::Mod>;
    let mut cmt: Arc<SCode::Comment>;
    let mut cond: Option<Arc<Absyn::Exp>>;
    let mut info: SourceInfo;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(inComponent) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: __pa1, attributes: __pa2, typeSpec: __pa3, modifications: __pa4, comment: __pa5, condition: __pa6, info: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    prefixes = __pa1.clone();
    attr = __pa2.clone();
    type_spec = __pa3.clone();
    r#mod = __pa4.clone();
    cmt = __pa5.clone();
    cond = __pa6.clone();
    info = __pa7.clone();
    attr = flattenAttributes(attr, inEnv.clone(), info.clone())?;
    type_spec = flattenTypeSpec(type_spec, inEnv.clone(), info.clone())?;
    r#mod = flattenModifier(r#mod, inEnv.clone(), info.clone())?;
    cond = flattenOptExp(cond, inEnv, info.clone())?;
    outComponent = Arc::new(SCode::Element::COMPONENT { name: (name).clone(), prefixes: prefixes, attributes: attr, typeSpec: type_spec, modifications: r#mod, comment: cmt, condition: cond, info: info });
    Ok(outComponent)
}

fn flattenAttributes(mut inAttributes: SCode::Attributes, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    let mut ct: SCode::ConnectorType;
    let mut prl: SCode::Parallelism;
    let mut var: SCode::Variability;
    let mut dir: Absyn::Direction;
    let mut isf: Absyn::IsField;
    let SCode::ATTR { arrayDims: __pa0, connectorType: __pa1, parallelism: __pa2, variability: __pa3, direction: __pa4, isField: __pa5 } = (inAttributes) else { bail!("pattern mismatch") };
    ad = __pa0.clone();
    ct = __pa1.clone();
    prl = __pa2.clone();
    var = __pa3.clone();
    dir = __pa4.clone();
    isf = __pa5.clone();
    ad = List::map2(ad, (std::sync::Arc::new(flattenSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Subscript>> + 'static>), inEnv, inInfo)?;
    outAttributes = SCode::Attributes { arrayDims: ad, connectorType: ct, parallelism: prl, variability: var, direction: dir, isField: isf };
    Ok(outAttributes)
}

fn flattenTypeSpec(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    outTypeSpec = (::match_deref::match_deref! { match &(inTypeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: ad } => {
            let mut path = (*path).clone();
            (_, path, _) = NFSCodeLookup::lookupClassName(path.clone(), inEnv, inInfo)?;
            Arc::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: ad.clone() })
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. } => {
            inTypeSpec
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path, typeSpecs: tys, arrayDim: ad } => {
            let mut tys = (*tys).clone();
            tys = List::map2(tys.clone(), (std::sync::Arc::new(flattenTypeSpec) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::TypeSpec>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::TypeSpec>> + 'static>), inEnv, inInfo)?;
            Arc::new(Absyn::TypeSpec::TCOMPLEX { path: path.clone(), typeSpecs: tys.clone(), arrayDim: ad.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTypeSpec)
}

fn flattenExtends(mut inExtends: Arc<SCode::Element>, mut inEnv: Env) -> Result<Arc<SCode::Element>> {
    let mut outExtends: Arc<SCode::Element>;
    let mut path: Arc<Absyn::Path>;
    let mut r#mod: Arc<SCode::Mod>;
    let mut ann: Option<Arc<SCode::Annotation>>;
    let mut info: SourceInfo;
    let mut env: Env;
    let mut vis: SCode::Visibility;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inExtends) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: __pa1, modifications: __pa2, ann: __pa3, info: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    vis = __pa1.clone();
    r#mod = __pa2.clone();
    ann = __pa3.clone();
    info = __pa4.clone();
    env = NFSCodeEnv::removeExtendsFromLocalScope(inEnv.clone())?;
    (_, path, _) = NFSCodeLookup::lookupBaseClassName(path, env, info.clone())?;
    r#mod = flattenModifier(r#mod, inEnv, info.clone())?;
    outExtends = Arc::new(SCode::Element::EXTENDS { baseClassPath: path, visibility: vis, modifications: r#mod, ann: ann, info: info });
    Ok(outExtends)
}

fn flattenEquation(mut inEquation: Arc<SCode::Equation>, mut inEnv: Env) -> Result<Arc<SCode::Equation>> {
    let mut outEquation: Arc<SCode::Equation>;
    (outEquation, _) = SCodeUtil::mapFoldEquations(inEquation, (std::sync::Arc::new(flattenEquationTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Equation>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv)?;
    Ok(outEquation)
}

fn flattenEquationTraverser(mut eq: Arc<SCode::Equation>, mut env: Env) -> Result<(Arc<SCode::Equation>, Env)> {
    let mut eq: Arc<SCode::Equation> = eq;
    let mut env: Env = env;
    (eq, env) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { index: iter_name, info, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env)?;
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq, env)
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: crefExp @ Deref @ Absyn::Exp::CREF { componentRef: cref }, expReinit: exp, comment: cmt, info } => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone());
            eq = Arc::new(SCode::Equation::EQ_REINIT { cref: crefExp.clone(), expReinit: exp.clone(), comment: cmt.clone(), info: info.clone() });
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq, env)
        },
        _ => {
            let mut info: SourceInfo;
            info = SCodeUtil::getEquationInfo(eq.clone())?;
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq, env)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, env))
}

fn traverseExp(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = AbsynUtil::traverseExpBidir(inExp, (std::sync::Arc::new(flattenExpTraverserEnter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (std::sync::Arc::new(fnptr!(flattenExpTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), inTuple)?;
    Ok((outExp, outTuple))
}

fn flattenConstraints(mut inConstraints: SCode::ConstraintSection, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<SCode::ConstraintSection> {
    let mut outConstraints: SCode::ConstraintSection;
    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    let SCode::CONSTRAINTS { constraints: __pa0 } = (inConstraints) else { bail!("pattern mismatch") };
    exps = __pa0.clone();
    exps = List::map2(exps, (std::sync::Arc::new(flattenExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Exp>> + 'static>), inEnv, inInfo)?;
    outConstraints = SCode::ConstraintSection { constraints: exps };
    Ok(outConstraints)
}

fn flattenAlgorithm(mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inEnv: Env) -> Result<Arc<SCode::AlgorithmSection>> {
    let mut outAlgorithm: Arc<SCode::AlgorithmSection>;
    let mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm) {
        Deref @ SCode::AlgorithmSection { statements: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    statements = __pa0.clone();
    statements = List::map1(statements, (std::sync::Arc::new(flattenStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<Arc<SCode::Statement>> + 'static>), inEnv)?;
    outAlgorithm = Arc::new(SCode::AlgorithmSection { statements: statements });
    Ok(outAlgorithm)
}

fn flattenStatement(mut inStatement: Arc<SCode::Statement>, mut inEnv: Env) -> Result<Arc<SCode::Statement>> {
    let mut outStatement: Arc<SCode::Statement>;
    (outStatement, _) = SCodeUtil::mapFoldStatements(inStatement, (std::sync::Arc::new(flattenStatementTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<(Arc<SCode::Statement>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>)> + 'static>), inEnv)?;
    Ok(outStatement)
}

fn flattenStatementTraverser(mut stmt: Arc<SCode::Statement>, mut env: Env) -> Result<(Arc<SCode::Statement>, Env)> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut env: Env = env;
    (stmt, env) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_FOR { index: iter_name, info, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env)?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(stmt, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (stmt, env)
        },
        Deref @ SCode::Statement::ALG_PARFOR { index: iter_name, info, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env)?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(stmt, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (stmt, env)
        },
        _ => {
            let mut info: SourceInfo;
            info = SCodeUtil::getStatementInfo(stmt.clone())?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(stmt, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (stmt, env)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, env))
}

fn flattenModifier(mut inMod: Arc<SCode::Mod>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { finalPrefix: fp, eachPrefix: ep, subModLst: sub_mods, binding: opt_exp, comment: cmt, info } => {
            let mut sub_mods = (*sub_mods).clone();
            let mut opt_exp = (*opt_exp).clone();
            opt_exp = flattenModOptExp(opt_exp.clone(), inEnv.clone(), inInfo.clone())?;
            sub_mods = List::map2(sub_mods.clone(), (std::sync::Arc::new(flattenSubMod) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<SCode::SubMod>> + 'static>), inEnv, inInfo)?;
            Arc::new(SCode::Mod::MOD { finalPrefix: fp.clone(), eachPrefix: ep.clone(), subModLst: sub_mods.clone(), binding: opt_exp.clone(), comment: cmt.clone(), info: info.clone() })
        },
        Deref @ SCode::Mod::REDECL { finalPrefix: fp, eachPrefix: ep, element: el } => {
            let mut el = (*el).clone();
            el = flattenRedeclare(el.clone(), inEnv)?;
            Arc::new(SCode::Mod::REDECL { finalPrefix: fp.clone(), eachPrefix: ep.clone(), element: el.clone() })
        },
        Deref @ SCode::Mod::NOMOD { .. } => {
            inMod
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn flattenModOptExp(mut inOptExp: Option<Arc<Absyn::Exp>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outOptExp: Option<Arc<Absyn::Exp>>;
    outOptExp = (::match_deref::match_deref! { match &(inOptExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv, inInfo)?;
            Some(exp.clone())
        },
        _ => {
            inOptExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptExp)
}

fn flattenSubMod(mut inSubMod: Arc<SCode::SubMod>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<SCode::SubMod>> {
    let mut outSubMod: Arc<SCode::SubMod>;
    outSubMod = (::match_deref::match_deref! { match &(inSubMod) {
        Deref @ SCode::SubMod { ident, r#mod } => {
            let mut r#mod = (*r#mod).clone();
            r#mod = flattenModifier(r#mod.clone(), inEnv, inInfo)?;
            Arc::new(SCode::SubMod { ident: (ident.clone()).clone(), r#mod: r#mod.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubMod)
}

fn flattenRedeclare(mut inElement: Arc<SCode::Element>, mut inEnv: Env) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef @ Deref @ SCode::ClassDef::DERIVED { .. }, cmt, info } => {
            let mut cdef2: Arc<SCode::ClassDef>;
            cdef2 = flattenDerivedClassDef(cdef.clone(), inEnv, info.clone())?;
            Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef2, cmt: cmt.clone(), info: info.clone() })
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
            inElement
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut element: Arc<SCode::Element>;
            element = flattenComponent(inElement, inEnv)?;
            element
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unknown redeclare in NFSCodeFlattenImports.flattenRedeclare")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn flattenSubscript(mut inSub: Arc<Absyn::Subscript>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::Subscript>> {
    let mut outSub: Arc<Absyn::Subscript>;
    outSub = (::match_deref::match_deref! { match &(inSub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp } => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv, inInfo)?;
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: exp.clone() })
        },
        Deref @ Absyn::Subscript::NOSUB { .. } => {
            inSub
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSub)
}

fn flattenExp(mut inExp: Arc<Absyn::Exp>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    (outExp, _) = AbsynUtil::traverseExpBidir(inExp, (std::sync::Arc::new(flattenExpTraverserEnter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (std::sync::Arc::new(fnptr!(flattenExpTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> + 'static>), (inEnv, inInfo))?;
    Ok(outExp)
}

fn flattenOptExp(mut inExp: Option<Arc<Absyn::Exp>>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outExp: Option<Arc<Absyn::Exp>>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv, inInfo)?;
            Some(exp.clone())
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn flattenExpTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo))> {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cref }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone());
            (Arc::new(Absyn::Exp::CREF { componentRef: cref.clone() }), tup.clone())
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators: iters }, .. }, (env, info)) => {
            let mut cref = (*cref).clone();
            let mut exp = (*exp).clone();
            let mut env = (*env).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone());
            env = NFSCodeEnv::extendEnvWithIterators(iters.clone(), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone())?;
            exp = flattenExp(exp.clone(), env.clone(), info.clone())?;
            (Arc::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: exp.clone(), iterType: iterType.clone(), iterators: iters.clone() }), typeVars: var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone() }), (env.clone(), info.clone()))
        },
        (Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "SOME", .. }, .. }, _) => {
            (inExp, inTuple)
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, functionArgs: args, .. }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone());
            (Arc::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: args.clone(), typeVars: var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone() }), tup.clone())
        },
        (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, functionArgs: args }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), info.clone());
            (Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: cref.clone(), functionArgs: args.clone() }), tup.clone())
        },
        (exp @ Deref @ Absyn::Exp::MATCHEXP { .. }, (env, info)) => {
            let mut env = (*env).clone();
            env = NFSCodeEnv::extendEnvWithMatch(exp.clone(), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone())?;
            (exp.clone(), (env.clone(), info.clone()))
        },
        _ => {
            (inExp, inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn flattenExpTraverserExit(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) -> (Arc<Absyn::Exp>, (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo)) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outTuple: (Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }, .. }, (Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: env }, info)) => {
            (inExp, (env.clone(), info.clone()))
        },
        (Deref @ Absyn::Exp::MATCHEXP { .. }, (Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: env }, info)) => {
            (inExp, (env.clone(), info.clone()))
        },
        _ => {
            (inExp, inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTuple)
}

pub(crate) fn flattenComponentRefSubs(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
            let mut subs = (*subs).clone();
            subs = List::map2(subs.clone(), (std::sync::Arc::new(flattenSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Subscript>> + 'static>), inEnv, inInfo)?;
            Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subs.clone() })
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts: subs, componentRef: cref } => {
            let mut subs = (*subs).clone();
            let mut cref = (*cref).clone();
            subs = List::map2(subs.clone(), (std::sync::Arc::new(flattenSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, SourceInfo) -> Result<Arc<Absyn::Subscript>> + 'static>), inEnv.clone(), inInfo.clone())?;
            cref = flattenComponentRefSubs(cref.clone(), inEnv, inInfo)?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subs.clone(), componentRef: cref.clone() })
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut cref = (*cref).clone();
            cref = flattenComponentRefSubs(cref.clone(), inEnv, inInfo)?;
            AbsynUtil::crefMakeFullyQualified(cref.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

