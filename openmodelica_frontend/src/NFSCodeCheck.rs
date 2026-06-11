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

use crate::NFInstDump;
use crate::NFInstTypes;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeEnv;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub(crate) fn checkRecursiveShortDefinition(mut inTypeSpec: Arc<Absyn::TypeSpec>, mut inTypeName: ArcStr, mut inTypeEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inTypeEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut ts_path: Arc<Absyn::Path>;
                    let mut ty_path: Arc<Absyn::Path>;
                    ts_path = AbsynUtil::typeSpecPath(inTypeSpec.clone())?;
                    ty_path = NFSCodeEnv::getEnvPath(inTypeEnv.clone())?;
                    let false = (isSelfReference((inTypeName.clone()).clone(), ty_path.clone(), ts_path.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: ArcStr;
                    ty = (Dump::unparseTypeSpec(inTypeSpec.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone(), list![(inTypeName.clone()).clone(), (ty.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn isSelfReference(mut inTypeName: ArcStr, mut inTypePath: Arc<Absyn::Path>, mut inReferencedName: Arc<Absyn::Path>) -> Result<bool> {
    let mut selfRef: bool;
    selfRef = (::match_deref::match_deref! { match &((inTypePath.clone(), inReferencedName)) {
        (p1, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
            AbsynUtil::pathEqual(AbsynUtil::joinPaths(p1.clone(), Arc::new(Absyn::Path::IDENT { name: (inTypeName).clone() }))?, p2.clone())
        },
        (_, p2) => {
            stringEqual((AbsynUtil::pathLastIdent(inTypePath)?).clone(), (AbsynUtil::pathFirstIdent(p2.clone())?).clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(selfRef)
}

pub(crate) fn checkClassExtendsReplaceability(mut inBaseClass: Arc<NFSCodeEnv::Item>, mut inOriginInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inBaseClass) {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { .. }, .. }, .. }, .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn checkRedeclareModifier(mut inModifier: Arc<NFSCodeEnv::Redeclaration>, mut inBaseClass: Arc<Absyn::Path>, mut inEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inModifier) {
        Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: e @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } } => {
            checkRedeclareModifier2(e.clone(), inBaseClass, inEnv)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkRedeclareModifier2(mut inModifier: Arc<SCode::Element>, mut inBaseClass: Arc<Absyn::Path>, mut inEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inModifier;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, .. }, .. } => {
                    let mut ty_path: Arc<Absyn::Path>;
                    ty_path = AbsynUtil::typeSpecPath(ty.clone())?;
                    let false = (isSelfReference((name.clone()).clone(), inBaseClass.clone(), ty_path.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, .. }, info, .. } => {
                    let mut ty_str: ArcStr;
                    ty_str = (Dump::unparseTypeSpec(ty.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone(), list![(name.clone()).clone(), (ty_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn checkModifierIfRedeclare(mut inItem: Arc<NFSCodeEnv::Item>, mut inModifier: Arc<SCode::Mod>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inModifier) {
        Deref @ SCode::Mod::REDECL { element: el, .. } => {
            checkRedeclaredElementPrefix(inItem, el.clone(), inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkRedeclaredElementPrefix(mut inItem: Arc<NFSCodeEnv::Item>, mut inReplacement: Arc<SCode::Element>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inItem, inReplacement)) {
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, prefixes: Deref @ SCode::Prefixes { finalPrefix: fin, replaceablePrefix: repl, .. }, attributes: SCode::Attributes { variability: var, .. }, typeSpec: ty1, info, .. }, .. }, Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { .. }, typeSpec: ty2, .. }) => {
            let mut ty: ArcStr;
            let mut ok: bool;
            ty = (literal!("component")).clone();
            ok = checkCompRedeclarationReplaceable((name.clone()).clone(), repl.clone(), ty1.clone(), ty2.clone(), inInfo.clone(), info.clone())?;
            ok = checkRedeclarationFinal((name.clone()).clone(), (ty.clone()).clone(), fin.clone(), inInfo.clone(), info.clone())? && ok;
            ok = checkRedeclarationVariability((name.clone()).clone(), (ty).clone(), var.clone(), inInfo, info.clone())? && ok;
            let true = (ok) else { bail!("pattern mismatch") };
            ()
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { finalPrefix: fin, replaceablePrefix: repl, .. }, restriction: res, info, .. }, .. }, Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { .. }, .. }) => {
            let mut ty: ArcStr;
            let mut ok: bool;
            ty = (SCodeDump::restrictionStringPP(res.clone())?).clone();
            ok = checkClassRedeclarationReplaceable((name.clone()).clone(), repl.clone(), inInfo.clone(), info.clone())?;
            ok = checkRedeclarationFinal((name.clone()).clone(), (ty).clone(), fin.clone(), inInfo, info.clone())? && ok;
            let true = (ok) else { bail!("pattern mismatch") };
            ()
        },
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, info, .. }, .. }, Deref @ SCode::Element::CLASS { restriction: res, .. }) => {
            let mut ty: ArcStr;
            ty = (SCodeDump::restrictionStringPP(res.clone())?).clone();
            ty = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("a ")); __mm_s.push_str(&*ty); ArcStr::from(__mm_s) }).clone();
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("component")).clone(), (name.clone()).clone(), (ty).clone()], list![inInfo, info.clone()])?;
            bail!("fail")
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, info, .. }, .. }, Deref @ SCode::Element::COMPONENT { name, .. }) => {
            let mut ty: ArcStr;
            ty = (SCodeDump::restrictionStringPP(res.clone())?).clone();
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(ty).clone(), (name.clone()).clone(), (literal!("a component")).clone()], list![inInfo, info.clone()])?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkClassRedeclarationReplaceable(mut inName: ArcStr, mut inReplaceable: Arc<SCode::Replaceable>, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool;
    isValid = (::match_deref::match_deref! { match &(inReplaceable) {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. } if (!(Flags::getConfigBool(Flags::IGNORE_REPLACEABLE.clone())?)) => {
            Error::addMultiSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(inName).clone()], list![inOriginInfo, inInfo])?;
            false
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isValid)
}

fn checkCompRedeclarationReplaceable(mut inName: ArcStr, mut inReplaceable: Arc<SCode::Replaceable>, mut inType1: Arc<Absyn::TypeSpec>, mut inType2: Arc<Absyn::TypeSpec>, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool;
    isValid = (::match_deref::match_deref! { match &(inReplaceable) {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. } if (AbsynUtil::pathEqual(AbsynUtil::typeSpecPath(inType1.clone())?, AbsynUtil::typeSpecPath(inType2.clone())?)) => true,
        Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. } if (!(Flags::getConfigBool(Flags::IGNORE_REPLACEABLE.clone())?)) => {
            Error::addMultiSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(inName).clone()], list![inOriginInfo, inInfo])?;
            bail!("fail")
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isValid)
}

fn checkRedeclarationFinal(mut inName: ArcStr, mut inType: ArcStr, mut inFinal: SCode::Final, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match inFinal {
        SCode::Final::NOT_FINAL { .. } => true,
        SCode::Final::FINAL { .. } => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE.clone(), list![(literal!("final")).clone(), (inType).clone(), (inName).clone()], list![inOriginInfo, inInfo])?;
            false
        },
    });
    Ok(isValid)
}

fn checkRedeclarationVariability(mut inName: ArcStr, mut inType: ArcStr, mut inVariability: SCode::Variability, mut inOriginInfo: SourceInfo, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match inVariability {
        SCode::Variability::CONST { .. } => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE.clone(), list![(literal!("constant")).clone(), (inType).clone(), (inName).clone()], list![inOriginInfo, inInfo])?;
            false
        },
        _ => true,
    });
    Ok(isValid)
}

fn checkRedeclarationVisibility(mut inName: ArcStr, mut inType: ArcStr, mut inOriginalVisibility: SCode::Visibility, mut inNewVisibility: SCode::Visibility, mut inOriginInfo: SourceInfo, mut inNewInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match (inOriginalVisibility, inNewVisibility) {
        (SCode::Visibility::PUBLIC { .. }, SCode::Visibility::PROTECTED { .. }) => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("public element")).clone(), (inName).clone(), (literal!("protected")).clone()], list![inNewInfo, inOriginInfo])?;
            false
        },
        (SCode::Visibility::PROTECTED { .. }, SCode::Visibility::PUBLIC { .. }) => {
            Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(literal!("protected element")).clone(), (inName).clone(), (literal!("public")).clone()], list![inNewInfo, inOriginInfo])?;
            false
        },
        _ => true,
    });
    Ok(isValid)
}

pub(crate) fn checkDuplicateRedeclarations(mut inRedeclare: Arc<NFSCodeEnv::Redeclaration>, mut inRedeclarations: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> Result<()> {
    let mut el_name: ArcStr;
    let mut el_info: SourceInfo;
    (el_name, el_info) = NFSCodeEnv::getRedeclarationNameInfo(inRedeclare)?;
    let false = (checkDuplicateRedeclarations2((el_name).clone(), el_info, inRedeclarations)) else { bail!("pattern mismatch") };
    Ok(())
}

fn checkDuplicateRedeclarations2(mut inRedeclareName: ArcStr, mut inRedeclareInfo: SourceInfo, mut inRedeclarations: Arc<metamodelica::List<Arc<NFSCodeEnv::Redeclaration>>>) -> bool {
    let mut outIsDuplicate: bool;
    outIsDuplicate = 'mc: {
        let __mc_input = inRedeclarations;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: redecl, tail: _ } => {
                    let mut el_name: ArcStr;
                    let mut el_info: SourceInfo;
                    (el_name, el_info) = NFSCodeEnv::getRedeclarationNameInfo(redecl.clone())?;
                    let true = (stringEqual((inRedeclareName.clone()).clone(), (el_name.clone()).clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), el_info.clone())?;
                    Error::addSourceMessage(Error::DUPLICATE_REDECLARATION.clone(), list![(inRedeclareName.clone()).clone()], inRedeclareInfo.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_redecls } => {
                    Ok(checkDuplicateRedeclarations2((inRedeclareName.clone()).clone(), inRedeclareInfo.clone(), rest_redecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outIsDuplicate
}

pub(crate) fn checkRecursiveComponentDeclaration(mut inComponentName: ArcStr, mut inComponentInfo: SourceInfo, mut inTypeEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>, mut inTypeItem: Arc<NFSCodeEnv::Item>, mut inComponentEnv: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inTypeEnv.clone(), inComponentEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let false = (NFSCodeEnv::envPrefixOf(inTypeEnv.clone(), inComponentEnv.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(cls_name), .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ } }) => {
                    let mut el: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(tree.clone(), (cls_name.clone()).clone())?) {
                        Deref @ NFSCodeEnv::Item::CLASS { cls: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    el = __pa0.clone();
                    let true = (SCodeUtil::isFunction(el.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(cls_name), .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ } }) => {
                    let mut el: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(tree.clone(), (cls_name.clone()).clone())?) {
                        Deref @ NFSCodeEnv::Item::CLASS { cls: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    el = __pa0.clone();
                    let true = (SCodeUtil::isUniontype(el.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty_name: ArcStr;
                    ty_name = (NFSCodeEnv::getItemName(inTypeItem.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSIVE_DEFINITION.clone(), list![(inComponentName.clone()).clone(), (ty_name.clone()).clone()], inComponentInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn checkIdentNotEqTypeName(mut inIdent: ArcStr, mut inTypeName: Arc<Absyn::TypeSpec>, mut inInfo: SourceInfo) -> bool {
    let mut outIsNotEq: bool;
    outIsNotEq = 'mc: {
        let __mc_input = (inIdent, inTypeName);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id, Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: ty }, .. }) => {
                    let true = (stringEq((id.clone()).clone(), (ty.clone()).clone())) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::LOOKUP_TYPE_FOUND_COMP.clone(), list![(id.clone()).clone()], inInfo.clone())?;
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
    outIsNotEq
}

pub(crate) fn checkComponentsEqual(mut inComponent1: Arc<NFInstTypes::Component>, mut inComponent2: Arc<NFInstTypes::Component>) -> () {
    let () = (::match_deref::match_deref! { match &(inComponent2) {
        _ => {
            metamodelica::print((literal!("Found duplicate component\n")).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub(crate) fn checkInstanceRestriction(mut inItem: Arc<NFSCodeEnv::Item>, mut inPrefix: Arc<NFInstPrefix::Prefix>, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inItem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, .. }, .. } => {
                    let true = (SCodeUtil::isInstantiableClassRestriction(res.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, .. }, .. } => {
                    let mut pre_str: ArcStr;
                    let mut res_str: ArcStr;
                    res_str = (SCodeDump::restrictionStringPP(res.clone())?).clone();
                    pre_str = (NFInstDump::prefixStr(inPrefix.clone())?).clone();
                    Error::addSourceMessage(Error::INVALID_CLASS_RESTRICTION.clone(), list![(res_str.clone()).clone(), (pre_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- NFSCodeCheck.checkInstanceRestriction failed on unknown item.")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn checkPartialInstance(mut inItem: Arc<NFSCodeEnv::Item>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inItem) {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, partialPrefix: SCode::Partial::PARTIAL { .. }, .. }, .. } => {
            Error::addSourceMessage(Error::INST_PARTIAL_CLASS.clone(), list![(name.clone()).clone()], inInfo)?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

