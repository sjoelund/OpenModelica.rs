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

use crate::Ceval;
use crate::FGraph;
use crate::FNode;
use crate::HashSet;
use crate::InnerOuter;
use crate::Inst;
use crate::InstExtends;
use crate::InstFunction;
use crate::Lookup;
use crate::Mod;
use crate::NFSCodeFlatten;
use crate::Patternm;
use crate::PrefixUtil;
use crate::Static;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use crate::UnitChecker;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTable5;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

pub(crate) fn newIdent() -> Arc<DAE::ComponentRef> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    let mut i: i32;
    let mut is: ArcStr;
    let mut s: ArcStr;
    i = tick();
    is = (intString(i)).clone();
    s = (stringAppend((literal!("__TMP__")).clone(), (is).clone())).clone();
    outComponentRef = ComponentReferenceBasics::makeCrefIdent((s).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    outComponentRef
}

fn isNotFunction(mut cls: Arc<SCode::Element>) -> bool {
    let mut res: bool;
    res = SCodeUtil::isFunction(cls);
    res = boolNot(res);
    res
}

pub(crate) fn scodeFlatten(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Flags::isSet(Flags::DO_SCODE_DEP.clone())?) else { bail!("pattern mismatch") };
                    Ok(inProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "" } => {
                    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = outProgram.clone();
                    outProgram = scodeFlattenProgram(inProgram.clone());
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outProgram = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = outProgram.clone();
                    (outProgram, _) = NFSCodeFlatten::flattenClassInProgram(inPath.clone(), inProgram.clone())?;
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outProgram = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProgram)
}

fn scodeFlattenProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = outProgram.clone();
                    ErrorExt::setCheckpoint((literal!("scodeFlattenProgram")).clone());
                    outProgram = NFSCodeFlatten::flattenCompleteProgram(inProgram.clone())?;
                    ErrorExt::delCheckpoint((literal!("scodeFlattenProgram")).clone());
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outProgram = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("scodeFlattenProgram")).clone());
                    Ok(inProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outProgram
}

pub(crate) fn reEvaluateInitialIfEqns(mut cache: FCore::Cache, mut env: FCore::Graph, mut dae: DAE::DAElist, mut isTopCall: bool) -> Result<DAE::DAElist> {
    let mut odae: DAE::DAElist;
    odae = (match (dae.clone(), isTopCall) {
        (DAE::DAElist { elementLst: ref elems }, true) => {
            let mut elems = elems.clone();
            elems = List::fold2r(elems.clone(), (std::sync::Arc::new(fnptr!(reEvaluateInitialIfEqns2, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Element>, FCore::Cache, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Element>, FCore::Cache, FCore::Graph) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> + 'static>), cache, env, metamodelica::nil())?.reverse();
            DAE::DAElist { elementLst: elems.clone() }
        },
        (_, false) => {
            dae
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(odae)
}

fn reEvaluateInitialIfEqns2(mut acc: Arc<metamodelica::List<Arc<DAE::Element>>>, mut elem: Arc<DAE::Element>, mut inCache: FCore::Cache, mut env: FCore::Graph) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut oelems: Arc<metamodelica::List<Arc<DAE::Element>>>;
    oelems = 'mc: {
        let __mc_input = (elem.clone(), inCache);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: tbs, equations3: fb, .. }, cache) => {
                    let mut valList: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut selectedBranch: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut blist: Arc<metamodelica::List<bool>>;
                    (_, valList) = Ceval::cevalList(cache.clone(), env.clone(), conds.clone(), true, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    blist = List::map(valList.clone(), (std::sync::Arc::new(ValuesUtil::valueBool) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<bool> + 'static>))?;
                    selectedBranch = List::findBoolList(blist.clone(), tbs.clone(), fb.clone())?;
                    selectedBranch = makeDAEElementInitial(selectedBranch.clone())?;
                    Ok(listAppend(selectedBranch.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(elem.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oelems
}

fn makeDAEElementInitial(mut inElems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outElems = (::match_deref::match_deref! { match &(inElems) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e1, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(Arc::new(DAE::Element::INITIALDEFINE { componentRef: cr.clone(), exp: e1.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: dims.clone(), exp: e1.clone(), array: e2.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(Arc::new(DAE::Element::INITIALEQUATION { exp1: e1.clone(), exp2: e2.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: tbs, equations3: fb, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: expl.clone(), equations2: tbs.clone(), equations3: fb.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: al, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: al.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1.clone(), rhs: e2.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            metamodelica::cons(elem.clone(), outElems)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElems)
}

pub(crate) fn lookupTopLevelClass(mut inName: ArcStr, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPrintError: bool) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = 'mc: {
        let __mc_input = inPrintError;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut cls: Arc<SCode::Element>;
            cls = List::getMemberOnTrue((inName.clone()).clone(), inProgram.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isClassNamed, ArcStr, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SCode::Element>) -> Result<bool> + 'static>))?;
            Ok(cls.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else { bail!("nomatch") };
            Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![(inName.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

pub(crate) fn fixInstClassType(mut ty: Arc<DAE::Type>, mut isPartialFn: bool) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (ty.clone(), isPartialFn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::TYPE { path: path1 }, .. }, _) => {
                    let mut name: ArcStr;
                    let mut path2: Arc<Absyn::Path>;
                    name = (AbsynUtil::pathLastIdent(path1.clone())?).clone();
                    path2 = AbsynUtil::stripLast(path1.clone())?;
                    ::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path2.clone())?) {
                        Deref @ "$Code" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    path2 = AbsynUtil::stripLast(path2.clone())?;
                    ::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path2.clone())?) {
                        Deref @ "OpenModelica" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(Util::assoc((name.clone()).clone(), list![(literal!("Expression"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION })), (literal!("ExpressionOrModification"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION_OR_MODIFICATION })), (literal!("TypeName"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME })), (literal!("VariableName"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME })), (literal!("VariableNames"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAMES }))])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true) => {
                    Ok(Types::makeFunctionPolymorphicReference(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub(crate) fn updateEnumerationEnvironment(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inClass: Arc<SCode::Element>, mut inCi_State: ClassInf::State) -> (FCore::Cache, FCore::Graph) {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, inType, inCi_State);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Type::T_ENUMERATION { names, literalVarLst: vars, path: p, .. }, ClassInf::State::ENUMERATION { path: pname }) => {
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_1) = updateEnumerationEnvironment1(cache.clone(), env.clone(), (AbsynUtil::pathString(pname.clone(), (literal!(".")).clone(), true, false)?).clone(), names.clone(), vars.clone(), p.clone())?;
                    Ok((cache.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outEnv)
}

fn updateEnumerationEnvironment1(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inName: ArcStr, mut inNames: Arc<metamodelica::List<ArcStr>>, mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, inNames, inVars, inPath)) {
        (cache, env, Deref @ metamodelica::List::Cons { head: nn, tail: names }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, .. }, tail: vars }, p) => {
            let mut env_1: FCore::Graph;
            let mut env_2: FCore::Graph;
            let mut compenv: FCore::Graph;
            let mut var: Arc<DAE::Var>;
            let mut cache = (*cache).clone();
            (cache, var, _, _, _, compenv) = Lookup::lookupIdentLocal(cache.clone(), env.clone(), (nn.clone()).clone())?;
            assign_field!(var.ty = ty.clone());
            env_1 = FGraph::updateComp(env.clone(), var.clone(), openmodelica_frontend_dump::FCore::Status::VAR_DAE, compenv.clone());
            { (inCache, inEnv, inName, inNames, inVars, inPath) = (cache.clone(), env_1.clone(), (var.name.clone()).clone(), names.clone(), vars.clone(), p.clone()); continue '__tco; }
        },
        (cache, env, Deref @ metamodelica::List::Nil, _, _) => {
            return Ok((cache.clone(), env.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn updateDeducedUnits(mut callScope: bool, mut store: UnitAbsyn::InstStore, mut dae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (match (callScope, store, dae.clone()) {
        (true, UnitAbsyn::InstStore::INSTSTORE { store: UnitAbsyn::Store { storeVector: mut vec, numElts: _ }, ht: mut ht, checkResult: _ }, DAE::DAElist { elementLst: ref elts }) => {
            let mut elts = elts.clone();
            elts = List::map2(elts.clone(), (std::sync::Arc::new(fnptr!(updateDeducedUnits2, Arc<DAE::Element>, metamodelica::Array<Option<UnitAbsyn::Unit>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, metamodelica::Array<Option<UnitAbsyn::Unit>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))) -> Result<Arc<DAE::Element>> + 'static>), vec.clone(), ht.clone())?;
            DAE::DAElist { elementLst: elts.clone() }
        },
        _ => {
            dae
        },
    });
    Ok(outDae)
}

fn updateDeducedUnits2(mut elt: Arc<DAE::Element>, mut vec: metamodelica::Array<Option<UnitAbsyn::Unit>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))) -> Arc<DAE::Element> {
    let mut oelt: Arc<DAE::Element>;
    oelt = 'mc: {
        let __mc_input = elt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cr, variableAttributesOption: varOpt @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: None, .. }), .. } => {
                    let mut indx: i32;
                    let mut unitStr: ArcStr;
                    let mut unit: UnitAbsyn::Unit;
                    let mut varOpt = (*varOpt).clone();
                    indx = BaseHashTable::get(cr.clone(), ht.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(({let __elt = vec.borrow()[(indx.clone()-1) as usize].clone(); __elt})) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit = __pa0.clone();
                    unitStr = (UnitAbsynBuilder::unit2str(unit.clone())?).clone();
                    varOpt = DAEUtil::setUnitAttr(varOpt.clone(), Arc::new(DAE::Exp::SCONST { string: (unitStr.clone()).clone() }))?;
                    Ok(DAEUtil::setVariableAttributes(elt.clone(), varOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oelt
}

pub(crate) fn reportUnitConsistency(mut topScope: bool, mut store: UnitAbsyn::InstStore) -> () {
    let () = 'mc: {
        let __mc_input = (topScope, store);
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (true, UnitAbsyn::InstStore::INSTSTORE { store: mut st, ht: _, checkResult: Some(UnitAbsyn::UnitCheckResult::CONSISTENT { .. }) }) = __mc_input.clone() else { bail!("nomatch") };
            let mut complete: bool;
            (complete, _) = UnitChecker::isComplete(st.clone())?;
            Error::addMessage(if (complete.clone()) {Error::CONSISTENT_UNITS.clone()} else {Error::INCOMPLETE_UNITS.clone()}, metamodelica::nil())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn extractConnectorPrefix(mut connectorRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut prefixCon: Arc<DAE::ComponentRef>;
    prefixCon = 'mc: {
        let __mc_input = connectorRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, subscriptLst: subs, componentRef: _ } => {
                    Ok(ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: child } => {
                    let mut child = (*child).clone();
                    child = extractConnectorPrefix(child.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), ty.clone(), subs.clone(), child.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(prefixCon)
}

fn updateCrefTypesWithConnectorPrefix(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = 'mc: {
        let __mc_input = (cr1.clone(), cr2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: name2, identType: _, subscriptLst: _, componentRef: child2 }) => {
                    let true = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), ty.clone(), subs.clone(), child2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: child }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: name2, identType: _, subscriptLst: _, componentRef: child2 }) => {
                    let mut outCref: Arc<DAE::ComponentRef> = outCref.clone();
                    let true = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    outCref = updateCrefTypesWithConnectorPrefix(child.clone(), child2.clone())?;
                    Ok((ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), ty.clone(), subs.clone(), outCref.clone()), outCref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outCref = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ***** FAILURE with ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); __mm_s.push_str(&*literal!(" _and_ ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

fn checkClassEqual(mut c1: Arc<SCode::Element>, mut c2: Arc<SCode::Element>) -> bool {
    let mut areEqual: bool;
    areEqual = 'mc: {
        let __mc_input = (c1.clone(), c2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((Config::acceptMetaModelicaGrammar()? && !(c1.clone() == c2.clone()))) { bail!("guard") }
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, .. }, _) => {
                    if !((!(c1.clone() == c2.clone()))) { bail!("guard") }
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: r, .. }, _) => {
                    let false = (SCodeUtil::isFunctionRestriction(r.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { normalAlgorithmLst: normalAlgorithmLst1, initialAlgorithmLst: initialAlgorithmLst1, .. }, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { normalAlgorithmLst: normalAlgorithmLst2, initialAlgorithmLst: initialAlgorithmLst2, .. }, .. }) => {
                    let true = (intEq((normalAlgorithmLst1.clone().len() as i32), (normalAlgorithmLst2.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let true = (intEq((initialAlgorithmLst1.clone().len() as i32), (initialAlgorithmLst2.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: cd1 @ Deref @ SCode::ClassDef::DERIVED { .. }, .. }, Deref @ SCode::Element::CLASS { classDef: cd2 @ Deref @ SCode::ClassDef::DERIVED { .. }, .. }) => {
                    Ok(cd1.clone() == cd2.clone())
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
    areEqual
}

pub(crate) fn prefixEqualUnlessBasicType(mut pre1: DAE::Prefix, mut pre2: DAE::Prefix, mut cls: Arc<SCode::Element>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = cls;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_ENUMERATION { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_INTEGER { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_REAL { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_STRING { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_BOOLEAN { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_CLOCK { .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: idn, .. } => {
                    if !((idn.clone() == literal!("Real") || idn.clone() == literal!("Integer") || idn.clone() == literal!("String") || idn.clone() == literal!("Boolean"))) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. } => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (pre1.clone() == pre2.clone()) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn isBuiltInClass(mut className: ArcStr) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(className) {
        Deref @ "Real" => true,
        Deref @ "Integer" => true,
        Deref @ "String" => true,
        Deref @ "Boolean" => true,
        Deref @ "Clock" => Config::synchronousFeaturesAllowed()?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn equalityConstraintOutputDimension(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElements) {
        Deref @ metamodelica::List::Nil => {
            return 0
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, arrayDims: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::INTEGER { value: dim } }, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, tail: _ } => {
            return dim.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: tail } => {
            let mut dim: i32;
            { inElements = tail.clone(); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn equalityConstraint(mut inEnv: FCore::Graph, mut inCdefelts: Arc<metamodelica::List<Arc<SCode::Element>>>, mut info: SourceInfo) -> Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> {
    let mut outResult: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut path: Arc<Absyn::Path>;
    let mut dimension: i32;
    let mut inlineType: DAE::InlineType;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(FGraph::getScopePath(inEnv.clone()), '__try0)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        path = __pa1.clone();
        path = unwrap_break_err!(AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("equalityConstraint")).clone() })), '__try0);
        path = AbsynUtil::makeFullyQualified(path.clone());
        Ok::<_, anyhow::Error>((path.clone(),))
    } {
        Ok((__try0_o0,)) => {
            path = __try0_o0;
        }
        Err(_) => {
            return outResult.clone();
        }
    }
    for mut el in &*inCdefelts {
        let mut el = el.clone();
        if '__try2: {
            let __pa3 = ::match_deref::match_deref! { match &(el.clone()) {
                Deref @ SCode::Element::CLASS { name: Deref @ "equalityConstraint", restriction: SCode::Restriction::R_FUNCTION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa3, .. }, .. } => __pa3.clone(),
                _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            els = __pa3.clone();
            dimension = equalityConstraintOutputDimension(els.clone());
            inlineType = classIsInlineFunc(el.clone());
            outResult = Some((path.clone(), dimension, inlineType.clone()));
            return outResult.clone();
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outResult
}

pub(crate) fn handleUnitChecking(mut cache: FCore::Cache, mut env: FCore::Graph, mut inStore: UnitAbsyn::InstStore, mut pre: DAE::Prefix, mut compDAE: DAE::DAElist, mut daes: Arc<metamodelica::List<DAE::DAElist>>, mut className: ArcStr) -> Result<(FCore::Cache, FCore::Graph, UnitAbsyn::InstStore)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outStore: UnitAbsyn::InstStore;
    (outCache, outEnv, outStore) = (match inStore {
        mut store => {
            (cache, env, store.clone())
        },
        mut store => {
            let mut daetemp: DAE::DAElist;
            let mut ut: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>>;
            daetemp = DAEUtil::joinDaeLst(daes)?;
            (store, ut) = UnitAbsynBuilder::instBuildUnitTerms(env.clone(), daetemp.clone(), compDAE.clone(), store.clone())?;
            UnitAbsynBuilder::registerUnitWeights(cache.clone(), env.clone(), compDAE);
            store = UnitChecker::check(ut.clone(), store.clone());
            (cache, env, store.clone())
        },
    });
    Ok((outCache, outEnv, outStore))
}

fn checkExtendsRestrictionMatch(mut r1: SCode::Restriction, mut r2: SCode::Restriction) -> Result<()> {
    let () = (match (r1, r2) {
        (SCode::Restriction::R_PACKAGE { .. }, SCode::Restriction::R_PACKAGE { .. }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ } }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }) => (),
        (SCode::Restriction::R_TYPE { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (SCode::Restriction::R_RECORD { isOperator: _ }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_TYPE { .. }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (SCode::Restriction::R_BLOCK { .. }, SCode::Restriction::R_RECORD { isOperator: false }) => (),
        (SCode::Restriction::R_BLOCK { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_RECORD { isOperator: false }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_CLASS { .. }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_CLASS { .. }) => (),
        (SCode::Restriction::R_OPERATOR { .. }, SCode::Restriction::R_OPERATOR { .. }) => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn checkExtendsForTypeRestiction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inRestriction: SCode::Restriction, mut inSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inRestriction, inSCodeElementLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let true = (listMember(r.clone(), list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: true }])) else { bail!("pattern mismatch") };
                    let true = (listMember((id.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone()])) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { bail!("pattern mismatch") };
                    let true = (listMember(r.clone(), list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: true }])) else { bail!("pattern mismatch") };
                    let true = (listMember((id.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone(), (literal!("Clock")).clone()])) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClass(inCache.clone(), inEnv.clone(), p.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r1, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r2: SCode::Restriction;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache.clone(), inEnv.clone(), p.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { restriction: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    checkExtendsRestrictionMatch(r1.clone(), r2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r1, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r2: SCode::Restriction;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache.clone(), inEnv.clone(), p.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { restriction: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error!: ")); __mm_s.push_str(&*SCodeDump::restrString(r1.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())); __mm_s.push_str(&*literal!(" cannot be extended by ")); __mm_s.push_str(&*SCodeDump::restrString(r2.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" due to derived/base class restrictions.\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn checkDerivedRestriction(mut parentRestriction: SCode::Restriction, mut childRestriction: SCode::Restriction, mut childName: ArcStr) -> Result<bool> {
    let mut b: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut b4: bool;
    let mut strLst: Arc<metamodelica::List<ArcStr>>;
    let mut rstLst: Arc<metamodelica::List<SCode::Restriction>>;
    strLst = if (Config::synchronousFeaturesAllowed()?) {list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("String")).clone(), (literal!("Boolean")).clone(), (literal!("Clock")).clone()]} else {list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("String")).clone(), (literal!("Boolean")).clone()]};
    b1 = listMember((childName).clone(), strLst);
    rstLst = if (Config::synchronousFeaturesAllowed()?) {list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK]} else {list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN]};
    b2 = listMember(childRestriction, rstLst);
    b3 = parentRestriction.clone() == openmodelica_frontend_types::SCode::Restriction::R_TYPE;
    b4 = parentRestriction.clone() == SCode::Restriction::R_CONNECTOR { isExpandable: false } || parentRestriction == SCode::Restriction::R_CONNECTOR { isExpandable: true };
    b = boolOr(b1, boolOr(b2, boolOr(b3, boolAnd(boolOr(b1, b2), b4))));
    Ok(b)
}

pub(crate) fn matchModificationToComponents(mut inElems: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inmod: Arc<DAE::Mod>, mut callingScope: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inElems, inmod.clone())) {
        (_, Deref @ DAE::Mod::NOMOD { .. }) => {
            ()
        },
        (_, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. }) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = (Mod::prettyPrintMod(inmod, 0)?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" not found in <")); __mm_s.push_str(&*callingScope); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::UNUSED_MODIFIER.clone(), list![(s2.clone()).clone()])?;
            bail!("fail")
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: cn, .. }, tail: elems }, r#mod) => {
            let mut r#mod = (*r#mod).clone();
            r#mod = Mod::removeMod(r#mod.clone(), (cn.clone()).clone())?;
            matchModificationToComponents(elems.clone(), r#mod.clone(), (callingScope).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: elems }, _) => {
            matchModificationToComponents(elems.clone(), inmod, (callingScope).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { name: cn, prefixes: Deref @ SCode::Prefixes { .. }, .. }, tail: elems }, r#mod) => {
            let mut r#mod = (*r#mod).clone();
            r#mod = Mod::removeMod(r#mod.clone(), (cn.clone()).clone())?;
            matchModificationToComponents(elems.clone(), r#mod.clone(), (callingScope).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::IMPORT { .. }, tail: elems }, _) => {
            matchModificationToComponents(elems.clone(), inmod, (callingScope).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }, .. }, tail: elems }, _) => {
            matchModificationToComponents(elems.clone(), inmod, (callingScope).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn elementNameMember(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut els: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut isNamed: bool;
    isNamed = listMember(Util::tuple21(inElement), els);
    isNamed
}

pub(crate) fn extractConstantPlusDepsTpl(mut inComps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut ocr: Option<Arc<DAE::ComponentRef>>, mut allComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr, mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut iieql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut ialgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut iialgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>) -> Result<(Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>)> {
    let mut oel: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut oeql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut oieql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut oalgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
    let mut oialgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
    (oel, oeql, oieql, oalgs, oialgs) = 'mc: {
        let __mc_input = (inComps.clone(), ocr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::nil(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None) => {
                    Ok((inComps.clone(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_)) => {
                    let mut lst: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut oel: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = oel.clone();
                    lst = List::map(inComps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
                    lst = extractConstantPlusDeps2(lst.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), metamodelica::nil())?;
                    let false = (lst.clone().is_empty()) else { bail!("pattern mismatch") };
                    lst = lst.clone().reverse();
                    oel = List::filter1OnTrue(inComps.clone(), (std::sync::Arc::new(fnptr!(elementNameMember, (Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<Arc<SCode::Element>>>)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<bool> + 'static>), lst.clone())?;
                    Ok(((oel.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), oel.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oel = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(cr)) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps failure to find ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(", returning \n")); ArcStr::from(__mm_s) }).clone())?;
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps elements to instantiate:")); __mm_s.push_str(&*intString((inComps.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oel, oeql, oieql, oalgs, oialgs))
}

pub(crate) fn extractConstantPlusDeps(mut inComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut ocr: Option<Arc<DAE::ComponentRef>>, mut allComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = ocr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(inComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(_) => {
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    outComps = extractConstantPlusDeps2(inComps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), metamodelica::nil())?;
                    let false = (outComps.clone().is_empty()) else { bail!("pattern mismatch") };
                    outComps = outComps.clone().reverse();
                    Ok((outComps.clone(), outComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outComps = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(cr) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps failure to find ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(", returning \n")); ArcStr::from(__mm_s) }).clone())?;
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps elements to instantiate:")); __mm_s.push_str(&*intString((inComps.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComps)
}

fn extractConstantPlusDeps2(mut inComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut ocr: Option<Arc<DAE::ComponentRef>>, mut inAllComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr, mut inExisting: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (inComps.clone(), ocr.clone(), inAllComps, inExisting);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Some(_), _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    Ok(inComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: selem @ Deref @ SCode::Element::CLASS { name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut allComps = (*allComps).clone();
                    let mut existing = (*existing).clone();
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    allComps = metamodelica::cons(selem.clone(), allComps.clone());
                    existing = metamodelica::cons((name2.clone()).clone(), existing.clone());
                    outComps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok((metamodelica::cons(selem.clone(), outComps.clone()), outComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outComps = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: selem @ Deref @ SCode::Element::COMPONENT { name: name2, modifications: scmod, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }), allComps, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    let mut allComps = (*allComps).clone();
                    let mut existing = (*existing).clone();
                    let true = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    crefs = getCrefFromMod(scmod.clone())?;
                    allComps = listAppend(comps.clone(), allComps.clone());
                    existing = metamodelica::cons((name2.clone()).clone(), existing.clone());
                    recDeps = extractConstantPlusDeps3(crefs.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(metamodelica::cons(selem.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: selem @ Deref @ SCode::Element::COMPONENT { name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }), allComps, existing) => {
                    let mut allComps = (*allComps).clone();
                    let false = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    allComps = metamodelica::cons(selem.clone(), allComps.clone());
                    Ok(extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compMod @ Deref @ SCode::Element::EXTENDS { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut allComps = (*allComps).clone();
                    allComps = metamodelica::cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(metamodelica::cons(compMod.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compMod @ Deref @ SCode::Element::IMPORT { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut allComps = (*allComps).clone();
                    allComps = metamodelica::cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(metamodelica::cons(compMod.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compMod @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut allComps = (*allComps).clone();
                    allComps = metamodelica::cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(metamodelica::cons(compMod.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!(" failure in get_Constant_PlusDeps \n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComps)
}

fn extractConstantPlusDeps3(mut inAcrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut remainingComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr, mut inExisting: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (inAcrefs, inExisting);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: acr }, tail: acrefs }, existing) => {
                    Ok(extractConstantPlusDeps3(metamodelica::cons(acr.clone(), acrefs.clone()), remainingComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: s1, subscripts: _, componentRef: acr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: _, subscripts: _ } }, tail: acrefs }, existing) => {
                    if !((stringEq((className.clone()).clone(), (s1.clone()).clone()))) { bail!("guard") }
                    Ok(extractConstantPlusDeps3(metamodelica::cons(acr.clone(), acrefs.clone()), remainingComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: _, subscripts: _, componentRef: _ }, tail: acrefs }, existing) => {
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    outComps = extractConstantPlusDeps3(acrefs.clone(), remainingComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok((outComps.clone(), outComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outComps = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s1, subscripts: _ }, tail: acrefs }, existing) => {
                    if !((List::isMemberOnTrue((s1.clone()).clone(), existing.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?)) { bail!("guard") }
                    Ok(extractConstantPlusDeps3(acrefs.clone(), remainingComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s1, subscripts: _ }, tail: acrefs }, existing) => {
                    let mut localComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut names: Arc<metamodelica::List<ArcStr>>;
                    let mut cref_: Arc<DAE::ComponentRef>;
                    let mut existing = (*existing).clone();
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent((s1.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    localComps = extractConstantPlusDeps2(remainingComps.clone(), Some(cref_.clone()), metamodelica::nil(), (className.clone()).clone(), existing.clone())?;
                    names = SCodeUtil::componentNamesFromElts(localComps.clone());
                    existing = listAppend(names.clone(), existing.clone());
                    outComps = extractConstantPlusDeps3(acrefs.clone(), remainingComps.clone(), (className.clone()).clone(), existing.clone())?;
                    outComps = listAppend(localComps.clone(), outComps.clone());
                    Ok((outComps.clone(), outComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outComps = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComps)
}

pub fn removeSelfReference(mut className: ArcStr, mut path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = if (stringEq((className.clone()).clone(), (AbsynUtil::pathFirstIdent(path.clone())?).clone())) {AbsynUtil::removePrefix(Arc::new(Absyn::Path::IDENT { name: (className).clone() }), path)?} else {path};
    Ok(outPath)
}

pub(crate) fn printExtcomps(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inElements) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (el, r#mod), tail: els } => {
            let mut s: ArcStr;
            s = (SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?).clone();
            metamodelica::print((s.clone()).clone());
            metamodelica::print((literal!(", ")).clone());
            metamodelica::print((Mod::printModStr(r#mod.clone())?).clone());
            metamodelica::print((literal!("\n")).clone());
            printExtcomps(els.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn constantEls(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    outElements = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut el in (elements).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: __esc_attr, .. } => {
            attr = (*__esc_attr).clone();
            SCodeUtil::isConstant(SCodeUtil::attrVariability(attr.clone())?)
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = el.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outElements)
}

pub(crate) fn constantAndParameterEls(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    outElements = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut el in (elements).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: __esc_attr, .. } => {
            attr = (*__esc_attr).clone();
            SCodeUtil::isParameterOrConst(SCodeUtil::attrVariability(attr.clone())?)
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = el.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outElements)
}

fn removeBindings(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElements = (::match_deref::match_deref! { match &(elements) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name, prefixes, attributes, typeSpec, modifications: _, comment, condition, info }, tail: els } => {
            let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            els1 = removeBindings(els.clone());
            metamodelica::cons(Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attributes.clone(), typeSpec: typeSpec.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), els1.clone())
        },
        Deref @ metamodelica::List::Cons { head: el, tail: els } => {
            let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            els1 = removeBindings(els.clone());
            metamodelica::cons(el.clone(), els1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

fn removeExtBindings(mut elements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    outElements = (::match_deref::match_deref! { match &(elements) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name, prefixes, attributes, typeSpec, modifications: _, comment, condition, info }, _), tail: els } => {
            let mut els1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
            els1 = removeExtBindings(els.clone());
            metamodelica::cons((Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attributes.clone(), typeSpec: typeSpec.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()), els1.clone())
        },
        Deref @ metamodelica::List::Cons { head: el, tail: els } => {
            let mut els1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
            els1 = removeExtBindings(els.clone());
            metamodelica::cons(el.clone(), els1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub(crate) fn getModsForDep(mut inDepCref: Arc<Absyn::ComponentRef>, mut inElems: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<DAE::Mod>> {
    let mut omods: Arc<DAE::Mod>;
    omods = 'mc: {
        let __mc_input = (inDepCref, inElems);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { .. }, Deref @ DAE::Mod::NOMOD { .. }), tail: elems }) => {
                    Ok(getModsForDep(dep.clone(), elems.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name: name1, .. }, cmod), tail: _ }) => {
                    let mut name2: ArcStr;
                    let mut cmod = (*cmod).clone();
                    name2 = (Dump::printComponentRefStr(dep.clone())?).clone();
                    let true = (stringEq((name2.clone()).clone(), (name1.clone()).clone())) else { bail!("pattern mismatch") };
                    cmod = Arc::new(DAE::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(DAE::SubMod { ident: (name2.clone()).clone(), r#mod: cmod.clone() })], binding: None, info: Absyn::dummyInfo.clone() });
                    Ok(cmod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::List::Cons { head: _, tail: elems }) => {
                    let mut cmod: Arc<DAE::Mod>;
                    cmod = getModsForDep(dep.clone(), elems.clone())?;
                    Ok(cmod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omods)
}

fn getOptionArraydim(mut inAbsynArrayDimOption: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    outArrayDim = (::match_deref::match_deref! { match &(inAbsynArrayDimOption) {
        Some(dim) => {
            dim.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArrayDim
}

pub(crate) fn addNomod(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    outElements = ({
        let mut __acc: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
        for mut x in (inElements).into_iter().cloned() {
            let __x = (x.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElements
}

pub(crate) fn sortElementList(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inEnv: FCore::Graph, mut isFunctionScope: bool) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    pub(crate) type Element = (Arc<SCode::Element>, Arc<DAE::Mod>);

    let mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = inElements;
    let mut outE: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    let mut cycles: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>;
    let mut g: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>;
    g = Graph::buildGraph(inElements.clone(), (std::sync::Arc::new(fnptr!(getElementDependencies, (Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> + 'static>), (inElements.clone(), isFunctionScope))?;
    (outE, cycles) = Graph::topologicalSort(g, (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?;
    if !(Config::acceptMetaModelicaGrammar()?) {
        inElements = listAppend(outE, List::map(cycles.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?);
    }
    checkCyclicalComponents(cycles, inEnv)?;
    Ok(inElements)
}

fn printGraph(mut env: FCore::Graph, mut g: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>, mut order: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut cycles: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>) -> Result<()> {
    pub(crate) type Element = (Arc<SCode::Element>, Arc<DAE::Mod>);

    let () = 'mc: {
        let __mc_input = g.clone();
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
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Graph for env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Graph::printGraph(g.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Element order:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>))?, (literal!("\n\t")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cycles:\n")); __mm_s.push_str(&*Graph::printGraph(cycles.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getDepsFromExps(mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAllElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut isFunction: bool) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExps, inDependencies.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(inDependencies)
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, deps) => {
            let mut deps = (*deps).clone();
            let (_, (_, _, __pa0, _)) = AbsynUtil::traverseExpBidir(e.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (inAllElements.clone(), metamodelica::nil(), deps.clone(), isFunction))?;
            deps = __pa0.clone();
            { (inExps, inAllElements, inDependencies, isFunction) = (rest.clone(), inAllElements, deps.clone(), isFunction); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn removeCurrentElementFromArrayDimDeps(mut name: ArcStr, mut inDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    outDependencies = ({
        let mut __acc: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
        for mut dep in (inDependencies).into_iter().cloned() {
            if !(!(stringEq((name.clone()).clone(), (SCodeUtil::elementName(Util::tuple21(dep.clone()))?).clone()))) { continue; }
            let __x = dep.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outDependencies)
}

pub(crate) fn getExpsFromConstrainClass(mut inRP: Arc<SCode::Replaceable>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut outBindingExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    let mut outSubsExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    (outBindingExp, outSubsExps) = (::match_deref::match_deref! { match &(inRP) {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: None } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m, .. }) } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            (l1, l2) = getExpsFromMod(m.clone())?;
            (l1.clone(), l2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBindingExp, outSubsExps))
}

fn getExpsFromSubMods(mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outSubsExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    outSubsExps = (::match_deref::match_deref! { match &(inSubMods) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { r#mod, .. }, tail: rest } => {
            let mut e: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut sm: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            (e, sm) = getExpsFromMod(r#mod.clone())?;
            exps = getExpsFromSubMods(rest.clone())?;
            exps = listAppend(e.clone(), listAppend(sm.clone(), exps.clone()));
            exps.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubsExps)
}

pub(crate) fn getCrefFromMod(mut inMod: Arc<SCode::Mod>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outCrefs = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = outCrefs.clone();
                    (l1, l2) = getExpsFromMod(inMod.clone())?;
                    outCrefs = List::flatten(List::map2(listAppend(l1.clone(), l2.clone()), (std::sync::Arc::new(AbsynUtil::getCrefFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool, bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> + 'static>), true, true)?)?;
                    Ok((outCrefs.clone(), outCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outCrefs = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.getCrefFromMod")); __mm_s.push_str(&*literal!(": could not retrieve crefs from SCode.Mod: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefs)
}

pub(crate) fn getExpsFromMod(mut inMod: Arc<SCode::Mod>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut outBindingExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    let mut outSubsExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    (outBindingExp, outSubsExps) = (::match_deref::match_deref! { match &(inMod) {
        Deref @ SCode::Mod::NOMOD { .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, binding: None, .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Mod::MOD { subModLst: subs, binding: Some(e), .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            se = getExpsFromSubMods(subs.clone())?;
            (list![e.clone()], se.clone())
        },
        Deref @ SCode::Mod::MOD { subModLst: subs, binding: None, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            se = getExpsFromSubMods(subs.clone())?;
            (metamodelica::nil(), se.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: ado }, modifications: m, attributes: _ }, .. }, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l3: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l4: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (_, se) = AbsynUtil::getExpsFromArrayDimOpt(ado.clone())?;
            (l3, l4) = getExpsFromMod(m.clone())?;
            l1 = listAppend(se.clone(), listAppend(l1.clone(), l3.clone()));
            l4 = listAppend(l2.clone(), l4.clone());
            (l1.clone(), l4.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: m, .. }, .. }, .. } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l3: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l4: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (l3, l4) = getExpsFromMod(m.clone())?;
            l3 = listAppend(l1.clone(), l3.clone());
            l4 = listAppend(l2.clone(), l4.clone());
            (l3.clone(), l4.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, .. } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (l1.clone(), l2.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, modifications: m, attributes: SCode::Attributes { arrayDims: ad, .. }, .. }, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l3: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l4: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (_, se) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
            (l3, l4) = getExpsFromMod(m.clone())?;
            l1 = listAppend(se.clone(), listAppend(l1.clone(), l3.clone()));
            l4 = listAppend(l2.clone(), l4.clone());
            (l1.clone(), l4.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBindingExp, outSubsExps))
}

pub(crate) fn getCrefFromDim(mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = inArrayDim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    l1 = getCrefFromDim(rest.clone())?;
                    l2 = AbsynUtil::getCrefFromExp(exp.clone(), true, true)?;
                    res = List::union(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: rest } => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    res = getCrefFromDim(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.getCrefFromDim failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynComponentRefLst)
}

pub(crate) fn getElementDependencies(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inAllElementsAndIsFunctionScope: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    outDependencies = 'mc: {
        let __mc_input = (inElement, inAllElementsAndIsFunctionScope);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { name, condition: cExpOpt, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, attributes: SCode::Attributes { arrayDims: ad, variability: var, .. }, modifications: r#mod, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let true = (SCodeUtil::isParameterOrConst(var.clone())) else { bail!("pattern mismatch") };
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (bexps, sexps) = getExpsFromMod(r#mod.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(Mod::unelabMod(daeMod.clone())?)?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = getDepsFromExps(List::fromOption(cExpOpt.clone()), inAllElements.clone(), deps.clone(), isFunction.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction, .. }, .. }, _), (_, true)) => {
                    let true = (AbsynUtil::isInputOrOutput(direction.clone())?) else { bail!("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { name, condition: cExpOpt, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, attributes: SCode::Attributes { arrayDims: ad, .. }, modifications: r#mod, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (bexps, sexps) = getExpsFromMod(r#mod.clone())?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    exps = listAppend(bexps.clone(), exps.clone());
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(Mod::unelabMod(daeMod.clone())?)?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    exps = listAppend(bexps.clone(), exps.clone());
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = getDepsFromExps(List::fromOption(cExpOpt.clone()), inAllElements.clone(), deps.clone(), isFunction.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, classDef: Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, attributes: SCode::Attributes { arrayDims: ad, .. }, .. }, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (_, sexps) = getExpsFromMod(r#mod.clone())?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (_, sexps) = getExpsFromMod(Mod::unelabMod(daeMod.clone())?)?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { .. }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl, .. }, .. }, _), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    exps = getExpsFromExternalDecl(externalDecl.clone())?;
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outDependencies
}

fn getExpsFromExternalDecl(mut inExternalDecl: Option<Arc<SCode::ExternalDecl>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    outExps = (::match_deref::match_deref! { match &(inExternalDecl) {
        None => {
            metamodelica::nil()
        },
        Some(Deref @ SCode::ExternalDecl { args: exps, .. }) => {
            exps.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExps)
}

fn getExpsFromDefaults(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inAcc: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Arc<metamodelica::List<Arc<Absyn::Exp>>> {
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    outExps = 'mc: {
        let __mc_input = inEls;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, modifications: m, .. }, tail: rest } => {
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    exps = inAcc.clone();
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(m.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    exps = getExpsFromDefaults(rest.clone(), exps.clone());
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    exps = getExpsFromDefaults(rest.clone(), inAcc.clone());
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outExps
}

fn getElementDependenciesTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> (Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) {
    pub(crate) type ElementList = Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;

    let mut outExp: Arc<Absyn::Exp>;
    let mut outTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::CREF { componentRef: cref }, (all_el, stack, accum_el, b)) => {
                    let mut id: ArcStr;
                    let mut e: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    e = List::find1(all_el.clone(), (std::sync::Arc::new(fnptr!(isElementNamed, (Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr) -> Result<bool> + 'static>), (id.clone()).clone())?;
                    Ok((exp.clone(), (all_el.clone(), stack.clone(), metamodelica::cons(e.clone(), accum_el.clone()), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::CALL { function_: cref, .. }, (all_el, stack, accum_el, b)) => {
                    let mut id: ArcStr;
                    let mut e: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    e = List::find1(all_el.clone(), (std::sync::Arc::new(fnptr!(isElementNamed, (Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr) -> Result<bool> + 'static>), (id.clone()).clone())?;
                    Ok((exp.clone(), (all_el.clone(), stack.clone(), metamodelica::cons(e.clone(), accum_el.clone()), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::IFEXP { .. }, (all_el, stack, accum_el, false)) => {
                    Ok((exp.clone(), (all_el.clone(), metamodelica::cons(accum_el.clone(), stack.clone()), metamodelica::nil(), false)))
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

fn getElementDependenciesTraverserExit(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> (Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) {
    pub(crate) type ElementList = Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;

    let mut outExp: Arc<Absyn::Exp>;
    let mut outTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::IFEXP { ifExp, .. }, (all_el, Deref @ metamodelica::List::Cons { head: stack_el, tail: rest_stack }, _, false)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let (_, (_, _, __pa0, _)) = AbsynUtil::traverseExpBidir(ifExp.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserExit, Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (all_el.clone(), metamodelica::nil(), metamodelica::nil(), false))?;
                    deps = __pa0.clone();
                    Ok((exp.clone(), (all_el.clone(), rest_stack.clone(), listAppend(deps.clone(), stack_el.clone()), false)))
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

fn isElementNamed(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inName: ArcStr) -> bool {
    let mut isNamed: bool;
    isNamed = (::match_deref::match_deref! { match &(inElement) {
        (Deref @ SCode::Element::COMPONENT { name, .. }, _) => {
            name.clone() == inName
        },
        (Deref @ SCode::Element::CLASS { name, .. }, _) => {
            name.clone() == inName
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

fn isElementEqual(mut inElement1: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inElement2: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((inElement1.clone(), inElement2.clone())) {
        ((Deref @ SCode::Element::COMPONENT { name: id1, .. }, _), (Deref @ SCode::Element::COMPONENT { name: id2, .. }, _)) => {
            stringEqual((id1.clone()).clone(), (id2.clone()).clone())
        },
        ((Deref @ SCode::Element::CLASS { name: id1, .. }, _), (Deref @ SCode::Element::CLASS { name: id2, .. }, _)) => {
            stringEqual((id1.clone()).clone(), (id2.clone()).clone())
        },
        _ => {
            stringEq((elementName(inElement1)?).clone(), (elementName(inElement2)?).clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

fn checkCyclicalComponents(mut inCycles: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>, mut inEnv: FCore::Graph) -> Result<()> {
    pub(crate) type Element = (Arc<SCode::Element>, Arc<DAE::Mod>);

    let () = 'mc: {
        let __mc_input = inCycles.clone();
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
                _ => {
                    let mut graph: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>;
                    graph = Graph::filterGraph(inCycles.clone(), (std::sync::Arc::new(fnptr!(isElementParamOrConst, (Arc<SCode::Element>, Arc<DAE::Mod>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?;
                    ::match_deref::match_deref! { match &(Graph::findCycles(graph.clone(), (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cycles: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>;
                    let mut names: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    let mut cycles_strs: Arc<metamodelica::List<ArcStr>>;
                    let mut cycles_str: ArcStr;
                    let mut scope_str: ArcStr;
                    cycles = Graph::findCycles(inCycles.clone(), (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?;
                    names = List::mapList(cycles.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>))?;
                    cycles_strs = List::map1(names.clone(), (std::sync::Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!(",")).clone())?;
                    cycles_str = stringDelimitList(cycles_strs.clone(), (literal!("}, {")).clone());
                    cycles_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*cycles_str.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    scope_str = (FGraph::printGraphPathStr(inEnv.clone())).clone();
                    Error::addMessage(Error::CIRCULAR_COMPONENTS.clone(), list![(scope_str.clone()).clone(), (cycles_str.clone()).clone()])?;
                    if !(Flags::isSet(Flags::IGNORE_CYCLES.clone())?) {
                        bail!("fail");
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn isElementParamOrConst(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> bool {
    let mut outIsParamOrConst: bool;
    outIsParamOrConst = (::match_deref::match_deref! { match &(inElement) {
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: var, .. }, .. }, _) => {
            SCodeUtil::isParameterOrConst(var.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsParamOrConst
}

fn elementName(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    let mut elem: Arc<SCode::Element> = Arc::new(<SCode::Element as ::std::default::Default>::default());
    outName = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elem: Arc<SCode::Element> = elem.clone();
                    let mut outName: ArcStr = outName.clone();
                    (elem, _) = inElement.clone();
                    outName = (SCodeUtil::elementName(elem.clone())?).clone();
                    Ok((outName.clone(), elem.clone(), outName.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { elem = __wb0; outName = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = (SCodeDump::shortElementStr(Util::tuple21(inElement.clone()))?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outName)
}

pub(crate) fn classdefElts2(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut partialPrefix: SCode::Partial) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outClassDefs: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut outConstEls: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    (outClassDefs, outConstEls) = 'mc: {
        let __mc_input = (inElements, partialPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, .. }, _), tail: xs }, SCode::Partial::PARTIAL { .. }) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((metamodelica::cons(cdef.clone(), cdefs.clone()), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cdef @ Deref @ SCode::Element::CLASS { .. }, _), tail: xs }, SCode::Partial::NOT_PARTIAL { .. }) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((metamodelica::cons(cdef.clone(), cdefs.clone()), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: el @ (Deref @ SCode::Element::COMPONENT { attributes: attr, .. }, _), tail: xs }, SCode::Partial::NOT_PARTIAL { .. }) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let SCode::CONST { .. } = (SCodeUtil::attrVariability(attr.clone())?) else { bail!("pattern mismatch") };
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((cdefs.clone(), metamodelica::cons(el.clone(), els.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((cdefs.clone(), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outClassDefs, outConstEls))
}

pub(crate) fn classdefAndImpElts(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>) {
    let mut cdefElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut restElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (cdefElts, restElts) = (::match_deref::match_deref! { match &(elts) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            (_, restElts) = classdefAndImpElts(xs.clone());
            (metamodelica::cons(cdef.clone(), restElts.clone()), restElts)
        },
        Deref @ metamodelica::List::Cons { head: imp @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            (cdefElts, restElts) = classdefAndImpElts(xs.clone());
            (metamodelica::cons(imp.clone(), cdefElts), restElts)
        },
        Deref @ metamodelica::List::Cons { head: e, tail: xs } => {
            (cdefElts, restElts) = classdefAndImpElts(xs.clone());
            (cdefElts, metamodelica::cons(e.clone(), restElts))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (cdefElts, restElts)
}

/*
protected function extendsElts
"author: PA
  This function filters out the extends Element in an Element list"
  input list<SCode.Element> inSCodeElementLst;
  output list<SCode.Element> outSCodeElementLst;
algorithm
  outSCodeElementLst := match (inSCodeElementLst)
    local
      list<SCode.Element> res,xs;
      SCode.Element cdef;
    case ({}) then {};
    case (((cdef as SCode.EXTENDS(baseClassPath = _)) :: xs))
      algorithm
        res = extendsElts(xs);
      then
        (cdef :: res);
    case ((_ :: xs))
      algorithm
        res = extendsElts(xs);
      then
        res;
  end match;
end extendsElts;
*/
pub(crate) fn componentElts(mut inSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inSCodeElementLst) {
        Deref @ metamodelica::List::Nil => {
            return metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::COMPONENT { .. }, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>>;
            res = componentElts(xs.clone());
            return metamodelica::cons(cdef.clone(), res.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>>;
            { inSCodeElementLst = xs.clone(); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn addClassdefsToEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inClasses: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inImpl: bool, mut inRedeclareMod: Option<Arc<DAE::Mod>>, mut checkDuplicates: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    for mut c in &*inClasses {
        let mut c = c.clone();
        (outCache, outEnv, outIH) = addClassdefToEnv(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), c.clone(), inImpl, inRedeclareMod.clone(), checkDuplicates)?;
    }
    Ok((outCache, outEnv, outIH))
}

fn addClassdefToEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSCodeElement: Arc<SCode::Element>, mut inBoolean: bool, mut redeclareMod: Option<Arc<DAE::Mod>>, mut checkDuplicates: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>>;
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPrefix, inSCodeElement, redeclareMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, sel1 @ Deref @ SCode::Element::CLASS { .. }, Some(m)) => {
                    let mut env_1: FCore::Graph;
                    let mut cl2: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut m = (*m).clone();
                    m = Mod::lookupCompModification(m.clone(), (var_field!((**sel1).name, SCode::Element::CLASS).clone()).clone())?;
                    let false = (m.clone() == openmodelica_frontend_types::DAE::Mod::interned_NOMOD()) else { bail!("pattern mismatch") };
                    env_1 = FGraph::mkClassNode(env.clone(), sel1.clone(), pre.clone(), m.clone(), false)?;
                    (cache, env_1, ih, cl2) = addClassdefsToEnv3(cache.clone(), env_1.clone(), ih.clone(), pre.clone(), redeclareMod.clone(), sel1.clone())?;
                    ih = InnerOuter::addClassIfInner(cl2.clone(), pre.clone(), env_1.clone(), ih.clone());
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, sel1 @ Deref @ SCode::Element::CLASS { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    let mut ih = (*ih).clone();
                    env_1 = FGraph::mkClassNode(env.clone(), sel1.clone(), pre.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), checkDuplicates)?;
                    ih = InnerOuter::addClassIfInner(sel1.clone(), pre.clone(), env_1.clone(), ih.clone());
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, imp @ Deref @ SCode::Element::IMPORT { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    env_1 = FGraph::mkImportNode(env.clone(), imp.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, elt @ Deref @ SCode::Element::DEFINEUNIT { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    env_1 = FGraph::mkDefunitNode(env.clone(), elt.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.addClassdefToEnv2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH))
}

fn checkCompEnvPathVsCompTypePath(mut inCompEnvPath: Option<Arc<Absyn::Path>>, mut inCompTypePath: Arc<Absyn::Path>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inCompEnvPath, inCompTypePath)) {
        (_, Deref @ Absyn::Path::IDENT { name: _ }) => {
            ()
        },
        (Some(ep), tp) => {
            let mut tp = (*tp).clone();
            tp = AbsynUtil::stripLast(tp.clone())?;
            let true = (AbsynUtil::pathPrefixOf(tp.clone(), ep.clone())) else { bail!("pattern mismatch") };
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn addComponentsToEnv(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut r#mod: Arc<DAE::Mod>, mut prefix: DAE::Prefix, mut state: ClassInf::State, mut components: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut r#impl: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = ih;
    let mut comp: Arc<SCode::Element>;
    let mut comp2: Arc<SCode::Element> = Arc::new(<SCode::Element as ::std::default::Default>::default());
    let mut cmod: Arc<DAE::Mod>;
    let mut local_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut comp_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut ty_path: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut prefs: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut dattr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let mut error: bool = false;
    let mut err_msg: ArcStr = arcstr::literal!("");
    for mut compmod in &*components {
        let mut compmod = compmod.clone();
        (comp, cmod) = compmod.clone();
        error = 'mc: {
        let __mc_input = comp.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: ty_path, .. }, .. } => {
                    if !((var_field!((*comp).name, SCode::Element::COMPONENT).clone() == AbsynUtil::pathLastIdent(ty_path.clone())?)) { bail!("guard") }
                    let mut err_msg: ArcStr = err_msg.clone();
                    checkCompEnvPathVsCompTypePath(FGraph::getScopePath(env.clone())?, ty_path.clone())?;
                    err_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*comp).name, SCode::Element::COMPONENT).clone()); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::COMPONENT_NAME_SAME_AS_TYPE_NAME.clone(), list![(err_msg.clone()).clone(), (AbsynUtil::pathString(ty_path.clone(), (literal!(".")).clone(), true, false)?).clone()], var_field!((*comp).info, SCode::Element::COMPONENT).clone())?;
                    Ok((true, err_msg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { err_msg = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { prefixes: prefs @ Deref @ SCode::Prefixes { .. }, attributes: attr @ SCode::Attributes { .. }, .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut cmod: Arc<DAE::Mod> = cmod.clone();
                    let mut comp: Arc<SCode::Element> = comp.clone();
                    let mut comp2: Arc<SCode::Element> = comp2.clone();
                    let mut comp_mod: Arc<DAE::Mod> = comp_mod.clone();
                    let mut dattr: Arc<DAE::Attributes> = dattr.clone();
                    let mut env: FCore::Graph = env.clone();
                    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = ih.clone();
                    let mut local_mod: Arc<DAE::Mod> = local_mod.clone();
                    let mut mod2: Arc<DAE::Mod> = mod2.clone();
                    let mut ty_path: Arc<Absyn::Path> = ty_path.clone();
                    ty_path = AbsynUtil::typeSpecPath(var_field!((*comp).typeSpec, SCode::Element::COMPONENT).clone())?;
                    local_mod = Mod::lookupModificationP(r#mod.clone(), ty_path.clone())?;
                    if SCodeUtil::finalBool(SCodeUtil::prefixesFinal(prefs.clone())?)? {
                        assign_variant_field!(comp => SCode::Element::COMPONENT; modifications = traverseModAddFinal(var_field!((*comp).modifications, SCode::Element::COMPONENT).clone())?);
                    }
                    (cache, env, ih, comp2, mod2) = Inst::redeclareType(cache.clone(), env.clone(), ih.clone(), local_mod.clone(), comp.clone(), prefix.clone(), state.clone(), r#impl, cmod.clone())?;
                    comp_mod = Mod::lookupCompModification(r#mod.clone(), (var_field!((*comp).name, SCode::Element::COMPONENT).clone()).clone())?;
                    cmod = Mod::merge(comp_mod.clone(), cmod.clone(), (literal!("")).clone(), true)?;
                    dattr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), prefs.clone())?;
                    env = FGraph::mkComponentNode(env.clone(), Arc::new(DAE::Var { name: (var_field!((*comp).name, SCode::Element::COMPONENT).clone()).clone(), attributes: dattr.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), comp.clone(), cmod.clone(), openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
                    Ok((false, cache.clone(), cmod.clone(), comp.clone(), comp2.clone(), comp_mod.clone(), dattr.clone(), env.clone(), ih.clone(), local_mod.clone(), mod2.clone(), ty_path.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { cache = __wb0; cmod = __wb1; comp = __wb2; comp2 = __wb3; comp_mod = __wb4; dattr = __wb5; env = __wb6; ih = __wb7; local_mod = __wb8; mod2 = __wb9; ty_path = __wb10; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
                    Ok(true)
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
        bail!("matchcontinue: no arm matched")
    };
        if error {
            bail!("fail");
        }
    }
    Ok((cache, env, ih))
}

fn getCrefsFromCompdims(mut inComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outCrefs = 'mc: {
        let __mc_input = inComponents;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: arraydim, .. }, .. }, _), tail: xs } => {
                    let mut crefs1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    let mut crefs2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    crefs1 = getCrefFromDim(arraydim.clone())?;
                    crefs2 = getCrefsFromCompdims(xs.clone());
                    crefs = listAppend(crefs1.clone(), crefs2.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    crefs = getCrefsFromCompdims(xs.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outCrefs
}

fn memberCrefs(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inComponentRefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> Result<bool> {
    let mut outIsMember: bool;
    outIsMember = List::isMemberOnTrue(inComponentRef, inComponentRefs, (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqualNoSubs, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))?;
    Ok(outIsMember)
}

pub(crate) fn chainRedeclares(mut inModOuter: Arc<DAE::Mod>, mut inModInner: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut b: bool = false;
    outMod = (::match_deref::match_deref! { match &(inModInner.clone()) {
        _ => {
            (outMod, b) = chainRedeclare_dispatch(inModOuter, inModInner.clone());
            if (b) {outMod} else {inModInner}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub(crate) fn chainRedeclare_dispatch(mut inModOuter: Arc<DAE::Mod>, mut inModInner: Arc<SCode::Mod>) -> (Arc<SCode::Mod>, bool) {
    let mut outMod: Arc<SCode::Mod>;
    let mut change: bool = false;
    (outMod, change) = 'mc: {
        let __mc_input = inModInner.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: Deref @ SCode::Element::CLASS { name: nInner, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: nDerivedInner }, .. }, .. }, .. } } => {
                    let mut cls: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inModOuter.clone(), (nDerivedInner.clone()).clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls = __pa0.clone();
                    cls = SCodeUtil::setClassName((nInner.clone()).clone(), cls.clone())?;
                    Ok((Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: cls.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: Deref @ SCode::Element::CLASS { name: nInner, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: _ }, .. }, .. }, .. } } => {
                    let mut cls: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inModOuter.clone(), (nInner.clone()).clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls = __pa0.clone();
                    Ok((Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: cls.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: name, r#mod: m @ Deref @ SCode::Mod::REDECL { .. } }, tail: rest }, binding: b, comment: cmt, info } => {
                    let mut subs: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut m2: Arc<SCode::Mod>;
                    (m2, _) = chainRedeclare_dispatch(inModOuter.clone(), m.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chainRedeclare_dispatch(inModOuter.clone(), Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: rest.clone(), binding: b.clone(), comment: cmt.clone(), info: info.clone() }))) {
                        (Deref @ SCode::Mod::MOD { subModLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    Ok((Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: metamodelica::cons(Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: m2.clone() }), subs.clone()), binding: b.clone(), comment: cmt.clone(), info: info.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: Deref @ metamodelica::List::Cons { head: sm, tail: rest }, binding: b, comment: cmt, info } => {
                    let mut subs: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut change: bool = change.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chainRedeclare_dispatch(inModOuter.clone(), Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: rest.clone(), binding: b.clone(), comment: cmt.clone(), info: info.clone() }))) {
                        (Deref @ SCode::Mod::MOD { subModLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    change = __pa1.clone();
                    Ok(((Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: metamodelica::cons(sm.clone(), subs.clone()), binding: b.clone(), comment: cmt.clone(), info: info.clone() }), change), change.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { change = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inModInner.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outMod, change)
}

fn addRecordConstructorsToTheCache(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inDirection: Absyn::Direction, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>) {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>>;
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inState, inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ClassInf::State::FUNCTION { path, .. }, Deref @ SCode::Element::CLASS { name, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }) => {
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut ih: InstanceHierarchy;
                    metamodelica::print((literal!("Depreciated record constructor used: Inst.addRecordConstructorsToTheCache")).clone());
                    let true = (AbsynUtil::isInputOrOutput(inDirection.clone())?) else { bail!("pattern mismatch") };
                    let false = (stringEq((AbsynUtil::pathLastIdent(path.clone())?).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outEnv, outIH)
}

pub(crate) fn checkMultiplyDeclared(mut cache: FCore::Cache, mut env: FCore::Graph, mut r#mod: Arc<DAE::Mod>, mut prefix: DAE::Prefix, mut ciState: ClassInf::State, mut compTuple: (Arc<SCode::Element>, Arc<DAE::Mod>), mut instDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut r#impl: bool) -> Result<bool> {
    let mut alreadyDeclared: bool = false;
    alreadyDeclared = 'mc: {
        let __mc_input = compTuple;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::setCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }, _) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { .. }, Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                newComp @ (Deref @ SCode::Element::COMPONENT { name: n, .. }, _) => {
                    let mut oldElt: Arc<SCode::Element>;
                    let mut oldMod: Arc<DAE::Mod>;
                    let mut instStatus: FCore::Status;
                    let mut alreadyDeclared: bool = alreadyDeclared.clone();
                    (_, _, oldElt, oldMod, instStatus, _) = Lookup::lookupIdentLocal(cache.clone(), env.clone(), (n.clone()).clone())?;
                    checkMultipleElementsIdentical(cache.clone(), env.clone(), (oldElt.clone(), oldMod.clone()), newComp.clone())?;
                    alreadyDeclared = instStatusToBool(instStatus.clone())?;
                    ErrorExt::delCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    Ok((alreadyDeclared, alreadyDeclared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { alreadyDeclared = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { name: n, .. }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdentLocal(cache.clone(), env.clone(), (n.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }, _) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { name: n, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: n2 }, .. }, tail: _ }, .. }, .. }, _) => {
                    let mut n = (*n).clone();
                    n = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$parent")); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
                    let 0 = (System::stringFind((n.clone()).clone(), (n2.clone()).clone())?) else { bail!("pattern mismatch") };
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (newClass @ Deref @ SCode::Element::CLASS { name: n, .. }, _) => {
                    let mut oldClass: Arc<SCode::Element>;
                    (oldClass, _) = Lookup::lookupClassLocal(env.clone(), (n.clone()).clone())?;
                    checkMultipleClassesEquivalent(oldClass.clone(), newClass.clone())?;
                    ErrorExt::delCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { name: n, .. }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClassLocal(env.clone(), (n.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::delCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("-Inst.checkMultiplyDeclared failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(alreadyDeclared)
}

fn instStatusToBool(mut instStatus: FCore::Status) -> Result<bool> {
    let mut alreadyDeclared: bool;
    alreadyDeclared = (match instStatus {
        FCore::Status::VAR_DAE { .. } => true,
        FCore::Status::VAR_UNTYPED { .. } => false,
        FCore::Status::VAR_TYPED { .. } => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(alreadyDeclared)
}

fn checkMultipleElementsIdentical(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut oldComponent: (Arc<SCode::Element>, Arc<DAE::Mod>), mut newComponent: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inCache, inEnv, oldComponent, newComponent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (oldElt, _), (newElt, _)) => {
                    let true = (SCodeUtil::elementEqual(oldElt.clone(), newElt.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, (Deref @ SCode::Element::COMPONENT { name: n1, prefixes: prefixes1, attributes: attr1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath1, arrayDim: ad1 }, modifications: smod1, comment: _, condition: cond1, info: _ }, _), (Deref @ SCode::Element::COMPONENT { name: n2, prefixes: prefixes2, attributes: attr2, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath2, arrayDim: ad2 }, modifications: smod2, comment: _, condition: cond2, info: _ }, _)) => {
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut c1: Arc<SCode::Element>;
                    let mut c2: Arc<SCode::Element>;
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::prefixesEqual(prefixes1.clone(), prefixes2.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::attributesEqual(attr1.clone(), attr2.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::modEqual(smod1.clone(), smod2.clone())) else { bail!("pattern mismatch") };
                    let true = (ad1.clone() == ad2.clone()) else { bail!("pattern mismatch") };
                    let true = (cond1.clone() == cond2.clone()) else { bail!("pattern mismatch") };
                    (_, c1, env1) = Lookup::lookupClass(cache.clone(), env.clone(), tpath1.clone(), None)?;
                    (_, c2, env2) = Lookup::lookupClass(cache.clone(), env.clone(), tpath2.clone(), None)?;
                    let true = (stringEq((FGraph::printGraphPathStr(env1.clone())).clone(), (FGraph::printGraphPathStr(env2.clone())).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(c1.clone(), c2.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, (oldElt @ Deref @ SCode::Element::COMPONENT { name: n1, prefixes: prefixes1, attributes: attr1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath1, arrayDim: ad1 }, modifications: smod1, comment: _, condition: cond1, info: old_info }, _), (newElt @ Deref @ SCode::Element::COMPONENT { name: n2, prefixes: prefixes2, attributes: attr2, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath2, arrayDim: ad2 }, modifications: smod2, comment: _, condition: cond2, info: new_info }, _)) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s: ArcStr;
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut c1: Arc<SCode::Element>;
                    let mut c2: Arc<SCode::Element>;
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (stringEq((n1.clone()).clone(), (literal!("m_flow")).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::prefixesEqual(prefixes1.clone(), prefixes2.clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::attributesEqual(attr1.clone(), attr2.clone())) else { bail!("pattern mismatch") };
                    let false = (SCodeUtil::modEqual(smod1.clone(), smod2.clone())) else { bail!("pattern mismatch") };
                    let true = (ad1.clone() == ad2.clone()) else { bail!("pattern mismatch") };
                    let true = (cond1.clone() == cond2.clone()) else { bail!("pattern mismatch") };
                    (_, c1, env1) = Lookup::lookupClass(cache.clone(), env.clone(), tpath1.clone(), None)?;
                    (_, c2, env2) = Lookup::lookupClass(cache.clone(), env.clone(), tpath2.clone(), None)?;
                    let true = (stringEq((FGraph::printGraphPathStr(env1.clone())).clone(), (FGraph::printGraphPathStr(env2.clone())).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(c1.clone(), c2.clone())) else { bail!("pattern mismatch") };
                    s1 = (SCodeDump::unparseElementStr(oldElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (SCodeDump::unparseElementStr(newElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inherited elements are not identical: bug: https://trac.modelica.org/Modelica/ticket/627\n\tfirst:  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\n\tsecond: ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\nContinue ....")); ArcStr::from(__mm_s) }).clone();
                    Error::addMultiSourceMessage(Error::COMPILER_WARNING.clone(), list![(s.clone()).clone()], list![old_info.clone(), new_info.clone()])?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (oldElt @ Deref @ SCode::Element::COMPONENT { info: old_info, .. }, _), (newElt @ Deref @ SCode::Element::COMPONENT { info: new_info, .. }, _)) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s1 = (SCodeDump::unparseElementStr(oldElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (SCodeDump::unparseElementStr(newElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Error::addMultiSourceMessage(Error::DUPLICATE_ELEMENTS_NOT_IDENTICAL.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], list![old_info.clone(), new_info.clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn checkMultipleClassesEquivalent(mut oldClass: Arc<SCode::Element>, mut newClass: Arc<SCode::Element>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (oldClass, newClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, .. }, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst, .. }, .. }) => {
                    let mut sl1: Arc<metamodelica::List<ArcStr>>;
                    let mut sl2: Arc<metamodelica::List<ArcStr>>;
                    sl1 = List::map(enumLst.clone(), (std::sync::Arc::new(SCodeUtil::enumName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Enum>) -> Result<ArcStr> + 'static>))?;
                    sl2 = List::map(elementLst.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>))?;
                    let true = (List::isEqualOnTrue(sl1.clone(), sl2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst, .. }, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, .. }) => {
                    let mut sl1: Arc<metamodelica::List<ArcStr>>;
                    let mut sl2: Arc<metamodelica::List<ArcStr>>;
                    sl1 = List::map(enumLst.clone(), (std::sync::Arc::new(SCodeUtil::enumName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Enum>) -> Result<ArcStr> + 'static>))?;
                    sl2 = List::map(elementLst.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>))?;
                    let true = (List::isEqualOnTrue(sl1.clone(), sl2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oldCl, newCl) => {
                    let true = (SCodeUtil::elementEqual(oldCl.clone(), newCl.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oldCl, newCl) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut info1: SourceInfo;
                    let mut info2: SourceInfo;
                    s1 = (SCodeDump::unparseElementStr(oldCl.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (SCodeDump::unparseElementStr(newCl.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    info1 = SCodeUtil::elementInfo(oldCl.clone());
                    info2 = SCodeUtil::elementInfo(newCl.clone());
                    Error::addMultiSourceMessage(Error::DUPLICATE_CLASSES_NOT_EQUIVALENT.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], list![info1.clone(), info2.clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn removeOptCrefFromCrefs(mut inCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inCref: Option<Arc<Absyn::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outCrefs = (::match_deref::match_deref! { match &(inCref) {
        Some(cref) => {
            removeCrefFromCrefs(inCrefs, cref.clone())?
        },
        _ => {
            inCrefs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefs)
}

pub(crate) fn removeCrefFromCrefs(mut inAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynComponentRefLst, inComponentRef)) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: n1, subscripts: Deref @ metamodelica::List::Nil }, tail: rest }, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n2, subscripts: Deref @ metamodelica::List::Nil }) if (stringEq((n1.clone()).clone(), (n2.clone()).clone())) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            { (inAbsynComponentRefLst, inComponentRef) = (rest.clone(), cr2.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: n1, .. }, tail: rest }, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n2, .. }) if (stringEq((n1.clone()).clone(), (n2.clone()).clone())) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            { (inAbsynComponentRefLst, inComponentRef) = (rest.clone(), cr2.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: cr1, tail: rest }, cr2) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            rest_1 = removeCrefFromCrefs(rest.clone(), cr2.clone())?;
            return Ok(metamodelica::cons(cr1.clone(), rest_1.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn keepConstrainingTypeModifersOnly(mut inMod: Arc<DAE::Mod>, mut elems: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<DAE::Mod>> {
    let mut filteredMod: Arc<DAE::Mod>;
    filteredMod = (::match_deref::match_deref! { match &((inMod.clone(), elems.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inMod
        },
        (Deref @ DAE::Mod::NOMOD { .. }, _) => {
            openmodelica_frontend_types::DAE::Mod::interned_NOMOD()
        },
        (Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }, _) => {
            inMod
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: subs, binding: oe, info }, _) => {
            let mut compNames: Arc<metamodelica::List<ArcStr>>;
            let mut subs = (*subs).clone();
            compNames = List::map(elems, (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>))?;
            subs = keepConstrainingTypeModifersOnly2(subs.clone(), compNames.clone())?;
            Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: subs.clone(), binding: oe.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(filteredMod)
}

fn keepConstrainingTypeModifersOnly2(mut isubs: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut elems: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((isubs, elems.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(metamodelica::nil())
        },
        (subs, Deref @ metamodelica::List::Nil) => {
            return Ok(subs.clone())
        },
        (Deref @ metamodelica::List::Cons { head: sub @ Deref @ DAE::SubMod { ident: n, .. }, tail: subs }, _) if (List::isMemberOnTrue((n.clone()).clone(), elems.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) => {
            return Ok(metamodelica::cons(sub.clone(), keepConstrainingTypeModifersOnly2(subs.clone(), elems.clone())?))
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: subs }, _) => {
            { (isubs, elems) = (subs.clone(), elems.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn extractConstrainingComps(mut cc: Option<Arc<SCode::ConstrainClass>>, mut env: FCore::Graph, mut pre: DAE::Prefix) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut elems: Arc<metamodelica::List<Arc<SCode::Element>>>;
    elems = 'mc: {
        let __mc_input = cc;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ SCode::ConstrainClass { constrainingClass: path, .. }) => {
                    let mut name: ArcStr;
                    let mut selems: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut extendselts: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut compelts: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut extcompelts: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut classextendselts: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut classes: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut extcomps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { name: __pa0, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa1, .. }, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    selems = __pa1.clone();
                    (classes, classextendselts, extendselts, compelts) = splitElts(selems.clone())?;
                    (_, _, _, _, extcomps, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(FCore::emptyCache(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), extendselts.clone(), classextendselts.clone(), selems.clone(), ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, (name.clone()).clone(), true, false)?;
                    extcompelts = List::map(extcomps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
                    compelts = listAppend(classes.clone(), listAppend(compelts.clone(), extcompelts.clone()));
                    Ok(compelts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ SCode::ConstrainClass { constrainingClass: path, modifier: r#mod, comment: cmt }) => {
                    let mut compelts: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let mut path = (*path).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, .. }, .. }, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    compelts = extractConstrainingComps(Some(Arc::new(SCode::ConstrainClass { constrainingClass: path.clone(), modifier: r#mod.clone(), comment: cmt.clone() })), env.clone(), pre.clone())?;
                    Ok(compelts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(elems)
}

pub(crate) fn moveBindings(mut inEquations: DAE::DAElist, mut inVariables: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outVariables: DAE::DAElist;
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    if Config::getGraphicsExpMode()? {
        outVariables = inVariables;
        return Ok(outVariables.clone());
    }
    let DAE::DAE { elementLst: __pa0 } = (inEquations) else { bail!("pattern mismatch") };
    eqs = __pa0.clone();
    let DAE::DAE { elementLst: __pa1 } = (inVariables) else { bail!("pattern mismatch") };
    vars = __pa1.clone();
    Error::assertion(intEq((eqs.clone().len() as i32), (vars.clone().len() as i32)), (literal!("- InstUtil.moveBindings: Mismatched number of equations and variables.")).clone(), Absyn::dummyInfo.clone())?;
    vars = List::threadMap(eqs, vars, (std::sync::Arc::new(moveBindings2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<Arc<DAE::Element>> + 'static>))?;
    outVariables = DAE::DAElist { elementLst: vars };
    Ok(outVariables)
}

fn moveBindings2(mut inEquation: Arc<DAE::Element>, mut inVariable: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outVariable: Arc<DAE::Element>;
    outVariable = 'mc: {
        let __mc_input = inVariable;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, kind, direction: dir, parallelism: prl, protection: vis, ty, binding: _, dims, connectorType: ct, source: src, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: e } => {
                    let mut bind_exp: Arc<DAE::Exp>;
                    bind_exp = moveBindings3(inEquation.clone())?;
                    Ok(Arc::new(DAE::Element::VAR { componentRef: cref.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: vis.clone(), ty: ty.clone(), binding: Some(bind_exp.clone()), dims: dims.clone(), connectorType: ct.clone(), source: src.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.moveBindings failed on ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariable)
}

fn moveBindings3(mut inEquation: Arc<DAE::Element>) -> Result<Arc<DAE::Exp>> {
    let mut outBinding: Arc<DAE::Exp>;
    outBinding = (::match_deref::match_deref! { match &(inEquation) {
        Deref @ DAE::Element::EQUATION { scalar: bind_exp, .. } => {
            bind_exp.clone()
        },
        Deref @ DAE::Element::DEFINE { exp: bind_exp, .. } => {
            bind_exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinding)
}

pub(crate) fn checkModificationOnOuter(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inCref: Arc<DAE::ComponentRef>, mut inMod: Arc<DAE::Mod>, mut inVariability: SCode::Variability, mut inInnerOuter: Absyn::InnerOuter, mut inImpl: bool, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inIH.clone(), inMod.clone(), inVariability, inInnerOuter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, SCode::Variability::CONST { .. }, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, SCode::Variability::PARAM { .. }, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: InnerOuter::TopInstance { sm, .. }, tail: _ }, Deref @ DAE::Mod::MOD { .. }, _, Absyn::InnerOuter::OUTER { .. }) => {
                    let mut cref: Arc<DAE::ComponentRef>;
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), sm.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (InnerOuter::modificationOnOuter(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), (inName.clone()).clone(), inCref.clone(), inMod.clone(), inInnerOuter.clone(), inImpl, inInfo.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn checkFunctionVar(mut inName: ArcStr, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inInfo: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inAttributes, inPrefixes)) {
        (SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, .. }) => {
            Error::addSourceMessage(Error::NON_FORMAL_PUBLIC_FUNCTION_VAR.clone(), list![(inName).clone()], inInfo)?;
            ()
        },
        (SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }) => (),
        (SCode::Attributes { .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }) => {
            Error::addSourceMessage(Error::PROTECTED_FORMAL_FUNCTION_VAR.clone(), list![(inName).clone()], inInfo)?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkFunctionVarType(mut inType: Arc<DAE::Type>, mut inState: ClassInf::State, mut inVarName: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Types::isValidFunctionVarType(inType.clone())) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ty_str: ArcStr;
            ty_str = (TypesDump::getTypeName(inType.clone())).clone();
            Error::addSourceMessage(Error::INVALID_FUNCTION_VAR_TYPE.clone(), list![(ty_str.clone()).clone(), (inVarName.clone()).clone()], inInfo.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn liftNonBasicTypes(mut tp: Arc<DAE::Type>, mut dimt: Arc<DAE::Dimension>) -> Arc<DAE::Type> {
    let mut outTp: Arc<DAE::Type>;
    outTp = 'mc: {
        let __mc_input = tp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                    let false = (TypesDump::getDimensions(ty.clone()).is_empty()) else { bail!("pattern mismatch") };
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Types::liftArray(tp.clone(), dimt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outTp
}

pub(crate) fn checkHigherVariability(mut compConst: DAE::Const, mut bindConst: DAE::Const, mut pre: DAE::Prefix, mut name: ArcStr, mut binding: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (compConst, bindConst, name, binding);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c, c1, _, _) => {
                    if !((c.clone() == c1.clone())) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Const::C_PARAM { .. }, DAE::Const::C_UNKNOWN { .. }, _, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c, c1, n, e) => {
                    let mut sc: ArcStr;
                    let mut sc1: ArcStr;
                    let mut se: ArcStr;
                    let mut sn: ArcStr;
                    sn = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStr2(pre.clone())?); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
                    sc = (DAEUtil::constStr(c.clone())?).clone();
                    sc1 = (DAEUtil::constStr(c1.clone())?).clone();
                    se = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(sn.clone()).clone(), (sc.clone()).clone(), (se.clone()).clone(), (sc1.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn makeArrayType(mut inDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inDimensionLst, inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ty) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: dim, tail: xs }, tty) => {
                    let mut ty_1: Arc<DAE::Type>;
                    ty_1 = makeArrayType(xs.clone(), tty.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: ty_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.makeArrayType failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub(crate) fn getUsertypeDimensions(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inBoolean: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<SCode::Element>, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut classToInstantiate: Arc<SCode::Element>;
    let mut outMods: Arc<DAE::Mod>;
    (outCache, outDimensionLst, classToInstantiate, outMods) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPrefix, inClass.clone(), inInstDims, inBoolean);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { name: id, .. }, _, _) => {
                    if !((id.clone() == literal!("Real") || id.clone() == literal!("Integer") || id.clone() == literal!("String") || id.clone() == literal!("Boolean"))) { bail!("guard") }
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. }, _, _) => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, pre, cl @ Deref @ SCode::Element::CLASS { name: id, info, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { .. }, arrayDim: ad, .. }, .. }, .. }, dims, r#impl) => {
                    let mut owncref: Arc<Absyn::ComponentRef>;
                    let mut ad_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut cache = (*cache).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    owncref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() });
                    ad_1 = getOptionArraydim(ad.clone());
                    (cache, dim1) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() }), ad_1.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), dims.clone())?;
                    Ok((cache.clone(), dim1.clone(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, partialPrefix: SCode::Partial::PARTIAL { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ SCode::Element::CLASS { name: id, info, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, .. }, _, _) => {
                    Error::addSourceMessage(Error::META_FUNCTION_TYPE_NO_PARTIAL_PREFIX.clone(), list![(id.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_UNIONTYPE { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ SCode::Element::CLASS { name: id, restriction: SCode::Restriction::R_TYPE { .. }, info, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: ad }, modifications: r#mod, .. }, .. }, dims, r#impl) => {
                    let mut cl: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut owncref: Arc<Absyn::ComponentRef>;
                    let mut ad_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut mod_1: Arc<DAE::Mod>;
                    let mut type_mods: Arc<DAE::Mod>;
                    let mut eq: Option<DAE::EqMod>;
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    (cache, cl, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?;
                    owncref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() });
                    ad_1 = getOptionArraydim(ad.clone());
                    env = addEnumerationLiteralsToEnv(env.clone(), cl.clone());
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    eq = Mod::modEquation(mod_1.clone())?;
                    (cache, dim1, cl, type_mods) = getUsertypeDimensions(cache.clone(), cenv.clone(), ih.clone(), pre.clone(), cl.clone(), dims.clone(), r#impl.clone())?;
                    (cache, dim2) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), cn.clone(), ad_1.clone(), eq.clone(), r#impl.clone(), true, false, pre.clone(), info.clone(), dims.clone())?;
                    type_mods = Mod::addEachIfNeeded(type_mods.clone(), dim2.clone())?;
                    type_mods = Mod::merge(mod_1.clone(), type_mods.clone(), (literal!("")).clone(), true)?;
                    res = listAppend(dim2.clone(), dim1.clone());
                    Ok((cache.clone(), res.clone(), cl.clone(), type_mods.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: Deref @ metamodelica::List::Nil, initialEquationLst: Deref @ metamodelica::List::Nil, normalAlgorithmLst: Deref @ metamodelica::List::Nil, initialAlgorithmLst: Deref @ metamodelica::List::Nil, .. }, .. }, _, r#impl) => {
                    let mut cl: Arc<SCode::Element>;
                    let mut mod_1: Arc<DAE::Mod>;
                    let mut type_mods: Arc<DAE::Mod>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut r#mod: Arc<SCode::Mod>;
                    let mut info: SourceInfo;
                    let mut path: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(splitElts(els.clone())?) {
                        (_, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: _, modifications: __pa1, ann: _, info: __pa2 }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    r#mod = __pa1.clone();
                    info = __pa2.clone();
                    (cache, mod_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::EXTENDS { path: path.clone() }, info.clone())?;
                    (cache, cl, _) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
                    (cache, res, cl, type_mods) = getUsertypeDimensions(cache.clone(), env.clone(), ih.clone(), pre.clone(), cl.clone(), metamodelica::nil(), r#impl.clone())?;
                    type_mods = Mod::merge(mod_1.clone(), type_mods.clone(), (literal!("")).clone(), true)?;
                    Ok((cache.clone(), res.clone(), cl.clone(), type_mods.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    let mut id: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    id = (SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.getUsertypeDimensions failed: ")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimensionLst, classToInstantiate, outMods))
}

fn addEnumerationLiteralsToEnv(mut inEnv: FCore::Graph, mut inClass: Arc<SCode::Element>) -> FCore::Graph {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: enums, .. }, .. } => {
                    let mut env: FCore::Graph;
                    env = List::fold(enums.clone(), (std::sync::Arc::new(addEnumerationLiteralToEnv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, FCore::Graph) -> Result<FCore::Graph> + 'static>), inEnv.clone())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inEnv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outEnv
}

fn addEnumerationLiteralToEnv(mut inEnum: Arc<SCode::Element>, mut inEnv: FCore::Graph) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = inEnum.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name: lit, .. } => {
                    let mut env: FCore::Graph;
                    env = FGraph::mkComponentNode(inEnv.clone(), Arc::new(DAE::Var { name: (lit.clone()).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), inEnum.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("InstUtil.addEnumerationLiteralToEnv: Unknown enumeration type!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

pub(crate) fn updateClassInfState(mut inCache: FCore::Cache, mut inNewEnv: FCore::Graph, mut inOldEnv: FCore::Graph, mut inCIState: ClassInf::State) -> ClassInf::State {
    let mut outCIState: ClassInf::State;
    outCIState = 'mc: {
        let __mc_input = inCIState.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut ci_state = __mc_input.clone() else { bail!("nomatch") };
            let true = (FGraph::isTopScope(inNewEnv.clone())) else { bail!("pattern mismatch") };
            Ok(ci_state.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut ci_state = __mc_input.clone() else { bail!("nomatch") };
            let true = (stringEq((FGraph::getGraphNameStr(inNewEnv.clone())).clone(), (FGraph::getGraphNameStr(inOldEnv.clone())).clone())) else { bail!("pattern mismatch") };
            Ok(ci_state.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ci_state: ClassInf::State;
            let mut rest: FCore::Graph;
            let mut id: ArcStr;
            let mut cls: Arc<SCode::Element>;
            let false = (FGraph::isTopScope(inNewEnv.clone())) else { bail!("pattern mismatch") };
            id = (FNode::refName(FGraph::lastScopeRef(inNewEnv.clone())?)?).clone();
            (rest, _) = FGraph::stripLastScopeRef(inNewEnv.clone())?;
            (_, cls, _) = Lookup::lookupClassIdent(inCache.clone(), rest.clone(), (id.clone()).clone(), None)?;
            ci_state = ClassInfUtil::start(SCodeUtil::getClassRestriction(cls.clone())?, FGraph::getGraphName(inNewEnv.clone())?)?;
            Ok(ci_state.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inCIState.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outCIState
}

pub(crate) fn evalEnumAndBoolDim(mut inDimension: Arc<DAE::Dimension>) -> Arc<DAE::Dimension> {
    let mut outDimension: Arc<DAE::Dimension>;
    outDimension = (::match_deref::match_deref! { match &(inDimension.clone()) {
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 }),
        _ => inDimension,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimension
}

/*TODO: mahge: Remove me*/
pub(crate) fn instDimExpNonSplit(mut inDimension: Arc<DAE::Dimension>, mut inBoolean: bool) -> Result<Arc<DAE::Subscript>> {
    let mut outSubscript: Arc<DAE::Subscript>;
    outSubscript = (::match_deref::match_deref! { match &(inDimension) {
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => {
            openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM()
        },
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: Arc::new(DAE::Exp::ICONST { integer: 2 }) })
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: e.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub(crate) fn instWholeDimFromMod(mut dimensionExp: Arc<DAE::Dimension>, mut modifier: Arc<DAE::Mod>, mut inVarName: ArcStr, mut inInfo: SourceInfo) -> Result<Arc<DAE::Dimension>> {
    let mut outDimension: Arc<DAE::Dimension>;
    outDimension = 'mc: {
        let __mc_input = (dimensionExp, modifier);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, .. }), .. }) => {
                    let mut d: Arc<DAE::Dimension>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::expDimensions(exp.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    d = __pa0.clone();
                    Ok(d.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, .. }), .. }) => {
                    let mut exp_str: ArcStr;
                    exp_str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Error::addSourceMessage(Error::FAILURE_TO_DEDUCE_DIMS_FROM_MOD.clone(), list![(inVarName.clone()).clone(), (exp_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.instWholeDimFromMod failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDimension)
}

pub(crate) fn propagateAttributes(mut inDae: DAE::DAElist, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inInfo: SourceInfo) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let DAE::DAE { elementLst: __pa0 } = (inDae) else { bail!("pattern mismatch") };
    elts = __pa0.clone();
    elts = List::map3(elts, (std::sync::Arc::new(propagateAllAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, SCode::Attributes, Arc<SCode::Prefixes>, SourceInfo) -> Result<Arc<DAE::Element>> + 'static>), inAttributes, inPrefixes, inInfo)?;
    outDae = DAE::DAElist { elementLst: elts };
    Ok(outDae)
}

fn propagateAllAttributes(mut inElement: Arc<DAE::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Element>> {
    let mut outElement: Arc<DAE::Element>;
    outElement = (::match_deref::match_deref! { match &((inElement.clone(), inAttributes.clone(), inPrefixes.clone())) {
        (_, SCode::Attributes { connectorType: SCode::ConnectorType::POTENTIAL { .. }, parallelism: SCode::Parallelism::NON_PARALLEL { .. }, variability: SCode::Variability::VAR { .. }, direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, finalPrefix: SCode::Final::NOT_FINAL { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, .. }) => {
            inElement
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, kind: vk, direction: vdir, parallelism: vprl, protection: vvis, ty, binding, dims, connectorType: ct2, source, variableAttributesOption: var_attrs, comment: cmt, innerOuter: io2, encrypted: e }, SCode::Attributes { connectorType: ct1, parallelism: sprl, variability: var, direction: dir, .. }, Deref @ SCode::Prefixes { visibility: vis, finalPrefix: fp, innerOuter: io1, .. }) => {
            let mut vk = (*vk).clone();
            let mut vdir = (*vdir).clone();
            let mut vprl = (*vprl).clone();
            let mut vvis = (*vvis).clone();
            let mut ct2 = (*ct2).clone();
            let mut var_attrs = (*var_attrs).clone();
            let mut io2 = (*io2).clone();
            vdir = propagateDirection(vdir.clone(), dir.clone(), cr.clone(), inInfo.clone())?;
            vk = propagateVariability(vk.clone(), var.clone());
            vprl = propagateParallelism(vprl.clone(), sprl.clone(), cr.clone(), inInfo.clone())?;
            vvis = propagateVisibility(vvis.clone(), vis.clone());
            var_attrs = propagateFinal(var_attrs.clone(), fp.clone())?;
            io2 = propagateInnerOuter(io2.clone(), io1.clone());
            ct2 = propagateConnectorType(ct2.clone(), ct1.clone(), cr.clone(), inInfo)?;
            Arc::new(DAE::Element::VAR { componentRef: cr.clone(), kind: vk.clone(), direction: vdir.clone(), parallelism: vprl.clone(), protection: vvis.clone(), ty: ty.clone(), binding: binding.clone(), dims: dims.clone(), connectorType: ct2.clone(), source: source.clone(), variableAttributesOption: var_attrs.clone(), comment: cmt.clone(), innerOuter: io2.clone(), encrypted: e.clone() })
        },
        (Deref @ DAE::Element::COMP { ident, dAElist: el, source, comment: cmt }, _, _) => {
            let mut el = (*el).clone();
            el = List::map3(el.clone(), (std::sync::Arc::new(propagateAllAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, SCode::Attributes, Arc<SCode::Prefixes>, SourceInfo) -> Result<Arc<DAE::Element>> + 'static>), inAttributes, inPrefixes, inInfo)?;
            Arc::new(DAE::Element::COMP { ident: (ident.clone()).clone(), dAElist: el.clone(), source: source.clone(), comment: cmt.clone() })
        },
        _ => {
            inElement
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn propagateDirection(mut inVarDirection: DAE::VarDirection, mut inDirection: Absyn::Direction, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<DAE::VarDirection> {
    let mut outVarDirection: DAE::VarDirection;
    outVarDirection = (match (inVarDirection.clone(), inDirection.clone()) {
        (_, Absyn::Direction::BIDIR { .. }) => {
            inVarDirection
        },
        (DAE::VarDirection::BIDIR { .. }, _) => {
            absynDirToDaeDir(inDirection)?
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            s1 = (Dump::directionSymbol(inDirection)?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inCref)?).clone();
            s3 = (DAEDump::dumpDirectionStr(inVarDirection)?).clone();
            Error::addSourceMessage(Error::COMPONENT_INPUT_OUTPUT_MISMATCH.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()], inInfo)?;
            bail!("fail")
        },
    });
    Ok(outVarDirection)
}

fn propagateParallelism(mut inVarParallelism: DAE::VarParallelism, mut inParallelism: SCode::Parallelism, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<DAE::VarParallelism> {
    let mut outVarParallelism: DAE::VarParallelism;
    outVarParallelism = 'mc: {
        let __mc_input = (inVarParallelism.clone(), inParallelism.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, SCode::Parallelism::NON_PARALLEL { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(inVarParallelism.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::VarParallelism::NON_PARALLEL { .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(DAEUtil::scodePrlToDaePrl(inParallelism.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut daeprl1, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut daeprl2: DAE::VarParallelism;
            daeprl2 = DAEUtil::scodePrlToDaePrl(inParallelism.clone())?;
            let true = (DAEUtil::daeParallelismEqual(daeprl1.clone(), daeprl2.clone())) else { bail!("pattern mismatch") };
            Ok(daeprl1.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut daeprl2: DAE::VarParallelism;
            daeprl2 = DAEUtil::scodePrlToDaePrl(inParallelism.clone())?;
            s1 = (DAEDump::dumpVarParallelismStr(daeprl2.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            s3 = (DAEDump::dumpVarParallelismStr(inVarParallelism.clone())?).clone();
            s4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Component declared as '")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("' when having the variable '")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("' declared as '")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("' : Subcomponent parallelism modified to.")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::PARMODELICA_WARNING.clone(), list![(s4.clone()).clone()], inInfo.clone())?;
            Ok(daeprl2.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarParallelism)
}

fn propagateVisibility(mut inVarVisibility: DAE::VarVisibility, mut inVisibility: SCode::Visibility) -> DAE::VarVisibility {
    let mut outVarVisibility: DAE::VarVisibility;
    outVarVisibility = (match inVisibility {
        SCode::Visibility::PROTECTED { .. } => openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
        _ => inVarVisibility,
    });
    outVarVisibility
}

fn propagateVariability(mut inVarKind: DAE::VarKind, mut inVariability: SCode::Variability) -> DAE::VarKind {
    let mut outVarKind: DAE::VarKind;
    outVarKind = (match (inVarKind.clone(), inVariability) {
        (_, SCode::Variability::VAR { .. }) => inVarKind,
        (DAE::VarKind::DISCRETE { .. }, _) => inVarKind,
        (_, SCode::Variability::DISCRETE { .. }) => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        (DAE::VarKind::CONST { .. }, _) => inVarKind,
        (_, SCode::Variability::CONST { .. }) => openmodelica_frontend_types::DAE::VarKind::CONST,
        (DAE::VarKind::PARAM { .. }, _) => inVarKind,
        (_, SCode::Variability::PARAM { .. }) => openmodelica_frontend_types::DAE::VarKind::PARAM,
        _ => inVarKind,
    });
    outVarKind
}

fn propagateFinal(mut inVarAttributes: Option<Arc<DAE::VariableAttributes>>, mut inFinal: SCode::Final) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outVarAttributes: Option<Arc<DAE::VariableAttributes>>;
    outVarAttributes = (match inFinal.clone() {
        SCode::Final::FINAL { .. } => DAEUtil::setFinalAttr(inVarAttributes, SCodeUtil::finalBool(inFinal)?)?,
        _ => inVarAttributes,
    });
    Ok(outVarAttributes)
}

fn propagateInnerOuter(mut inVarInnerOuter: Absyn::InnerOuter, mut inInnerOuter: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outVarInnerOuter: Absyn::InnerOuter;
    outVarInnerOuter = (match (inVarInnerOuter.clone(), inInnerOuter.clone()) {
        (_, Absyn::InnerOuter::NOT_INNER_OUTER { .. }) => inVarInnerOuter,
        (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, _) => inInnerOuter,
        _ => inVarInnerOuter,
    });
    outVarInnerOuter
}

fn propagateConnectorType(mut inVarConnectorType: Arc<DAE::ConnectorType>, mut inConnectorType: SCode::ConnectorType, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<Arc<DAE::ConnectorType>> {
    let mut outVarConnectorType: Arc<DAE::ConnectorType>;
    outVarConnectorType = (::match_deref::match_deref! { match &((inVarConnectorType.clone(), inConnectorType.clone())) {
        (_, SCode::ConnectorType::POTENTIAL { .. }) => {
            inVarConnectorType
        },
        (Deref @ DAE::ConnectorType::POTENTIAL { .. }, SCode::ConnectorType::FLOW { .. }) => {
            openmodelica_frontend_types::DAE::ConnectorType::interned_FLOW()
        },
        (Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }, SCode::ConnectorType::FLOW { .. }) => {
            openmodelica_frontend_types::DAE::ConnectorType::interned_FLOW()
        },
        (Deref @ DAE::ConnectorType::POTENTIAL { .. }, SCode::ConnectorType::STREAM { .. }) => {
            Arc::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        },
        (Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }, SCode::ConnectorType::STREAM { .. }) => {
            Arc::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            s1 = (SCodeDump::connectorTypeStr(inConnectorType)?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inCref)?).clone();
            s3 = (DAEDump::dumpConnectorType(inVarConnectorType)).clone();
            Error::addSourceMessage(Error::INVALID_TYPE_PREFIX.clone(), list![(s1.clone()).clone(), (literal!("variable")).clone(), (s2.clone()).clone(), (s3.clone()).clone()], inInfo)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarConnectorType)
}

fn absynDirToDaeDir(mut inDirection: Absyn::Direction) -> Result<DAE::VarDirection> {
    let mut outVarDirection: DAE::VarDirection;
    outVarDirection = (match inDirection {
        Absyn::Direction::INPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Absyn::Direction::OUTPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        Absyn::Direction::BIDIR { .. } => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        _ => bail!("match: no arm matched"),
    });
    Ok(outVarDirection)
}

fn attrIsParam(mut inAttributes: SCode::Attributes) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inAttributes {
        SCode::Attributes { variability: SCode::Variability::PARAM { .. }, .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn elabComponentArraydimFromEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    (outCache, outDimensionLst) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }) => {
                    let mut m: Arc<SCode::Mod>;
                    let mut m_1: Arc<SCode::Mod>;
                    let mut cmod: Arc<DAE::Mod>;
                    let mut cmod_1: Arc<DAE::Mod>;
                    let mut m_2: Arc<DAE::Mod>;
                    let mut mod_2: Arc<DAE::Mod>;
                    let mut eq: DAE::EqMod;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { modifications: __pa1, .. }, __pa2, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    m = __pa1.clone();
                    cmod = __pa2.clone();
                    cmod_1 = Mod::stripSubmod(cmod.clone());
                    m_1 = SCodeUtil::stripSubmod(m.clone());
                    (cache, m_2) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, m_1.clone(), false, Mod::ModScope::COMPONENT { name: (id.clone()).clone() }, info.clone())?;
                    mod_2 = Mod::merge(cmod_1.clone(), m_2.clone(), (literal!("")).clone(), true)?;
                    let __pa3 = ::match_deref::match_deref! { match &(Mod::modEquation(mod_2.clone())?) {
                        Some(__pa3) => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eq = __pa3.clone();
                    (cache, dims) = elabComponentArraydimFromEnv2(cache.clone(), eq.clone(), env.clone())?;
                    Ok((cache.clone(), dims.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }) => {
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: __pa1, .. }, .. }, _, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ad = __pa1.clone();
                    (cache, subs, _) = Static::elabSubscripts(cache.clone(), env.clone(), ad.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    dims = Expression::subscriptDimensions(subs.clone())?;
                    Ok((cache.clone(), dims.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cref) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.elabComponentArraydimFromEnv failed: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimensionLst))
}

fn elabComponentArraydimFromEnv2(mut inCache: FCore::Cache, mut inEqMod: DAE::EqMod, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    (outCache, outDimensionLst) = (match (inCache, inEqMod) {
        (mut cache, DAE::EqMod::TYPED { properties: DAE::Properties::PROP { type_: ref t, .. }, .. }) => {
            let mut lst: Arc<metamodelica::List<i32>>;
            let mut lst_1: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            lst = Types::getDimensionSizes(t.clone())?;
            lst_1 = List::map(lst.clone(), (std::sync::Arc::new(fnptr!(Expression::intDimension, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Dimension>> + 'static>))?;
            (cache.clone(), lst_1.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outCache, outDimensionLst))
}

pub(crate) fn elabArraydimOpt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut path: Arc<Absyn::Path>, mut inAbsynArrayDimOption: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut inTypesEqModOption: Option<DAE::EqMod>, mut inBoolean: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    (outCache, outDimensionLst) = (::match_deref::match_deref! { match &((inCache, inEnv, inComponentRef, inAbsynArrayDimOption, inTypesEqModOption, inBoolean, performVectorization, inPrefix, inInstDims)) {
        (cache, env, owncref, Some(ad), eq, r#impl, doVect, pre, inst_dims) => {
            let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut cache = (*cache).clone();
            (cache, res) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), path, ad.clone(), eq.clone(), r#impl.clone(), doVect.clone(), false, pre.clone(), info, inst_dims.clone())?;
            (cache.clone(), res.clone())
        },
        (cache, _, _, None, _, _, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outDimensionLst))
}

pub(crate) fn elabArraydim(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut path: Arc<Absyn::Path>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inTypesEqModOption: Option<DAE::EqMod>, mut inBoolean: bool, mut performVectorization: bool, mut isFunctionInput: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    (outCache, outDimensionLst) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef, inArrayDim, inTypesEqModOption, inBoolean, performVectorization, isFunctionInput, inPrefix, inInfo, inInstDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, _, _, doVect, true, pre, info, _) => {
                    let mut dim: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut cache = (*cache).clone();
                    (cache, dim) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), true, doVect.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, None, r#impl, doVect, _, pre, info, _) => {
                    let mut dim: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut cache = (*cache).clone();
                    (cache, dim) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop, modifierAsAbsynExp: _, .. }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut dim3: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut t: Arc<DAE::Type>;
                    let mut cache = (*cache).clone();
                    t = Types::getPropType(prop.clone())?;
                    (cache, dim1) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    dim2 = elabArraydimType(t.clone(), ad.clone(), e.clone(), path.clone(), pre.clone(), cref.clone(), info.clone(), inst_dims.clone())?;
                    dim3 = List::threadMap(dim1.clone(), dim2.clone(), (std::sync::Arc::new(compatibleArraydim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> + 'static>))?;
                    Ok((cache.clone(), dim3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, Some(DAE::EqMod::UNTYPED { exp: aexp }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut dim3: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), aexp.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    t = Types::getPropType(prop.clone())?;
                    (cache, dim1) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    dim2 = elabArraydimType(t.clone(), ad.clone(), e_1.clone(), path.clone(), pre.clone(), cref.clone(), info.clone(), inst_dims.clone())?;
                    dim3 = List::threadMap(dim1.clone(), dim2.clone(), (std::sync::Arc::new(compatibleArraydim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> + 'static>))?;
                    Ok((cache.clone(), dim3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: DAE::Properties::PROP { type_: t, constFlag: _ }, modifierAsAbsynExp: _, info: info2 }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut e_str: ArcStr;
                    let mut t_str: ArcStr;
                    let mut dim_str: ArcStr;
                    let false = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    (_, dim1) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    dim2 = elabArraydimType(t.clone(), ad.clone(), e.clone(), path.clone(), pre.clone(), cref.clone(), info.clone(), inst_dims.clone())?;
                    if '__try0: {
                        unwrap_break_err!(List::threadMap(dim1.clone(), dim2.clone(), (std::sync::Arc::new(compatibleArraydim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> + 'static>)), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    t_str = (TypesDump::unparseType(t.clone())?).clone();
                    dim_str = (ExpressionBasics::dimensionsString(dim1.clone())?).clone();
                    Error::addMultiSourceMessage(Error::ARRAY_DIMENSION_MISMATCH.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone(), (dim_str.clone()).clone()], metamodelica::cons(info2.clone(), metamodelica::cons(info.clone(), metamodelica::nil())))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cref, ad, eq, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.elabArraydim failed on: \n\tcref:")).clone())?;
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Dump::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printArraydimStr(ad.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*TypesDump::unparseOptionEqMod(eq.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimensionLst))
}

fn compatibleArraydim(mut inDimension1: Arc<DAE::Dimension>, mut inDimension2: Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> {
    let mut outDimension: Arc<DAE::Dimension>;
    outDimension = (::match_deref::match_deref! { match &((inDimension1.clone(), inDimension2.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(),
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => inDimension1,
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => inDimension2,
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => inDimension1,
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => inDimension2,
        (_, _) => {
            let true = (intEq(Expression::dimensionSize(inDimension1.clone())?, Expression::dimensionSize(inDimension2)?)) else { bail!("pattern mismatch") };
            inDimension1
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- InstUtil.compatibleArraydim failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDimension)
}

fn elabArraydimType(mut inType: Arc<DAE::Type>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inExp: Arc<DAE::Exp>, mut inPath: Arc<Absyn::Path>, mut inPrefix: DAE::Prefix, mut inCref: Arc<Absyn::ComponentRef>, mut inInfo: SourceInfo, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut flat_id: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut ad_str: ArcStr;
    let mut ty_str: ArcStr;
    let mut exp_str: ArcStr;
    let mut name_str: ArcStr;
    flat_id = if (Config::splitArrays()?) {metamodelica::nil()} else {List::flatten(inInstDims)?};
    match '__try0: {
        let true = (Types::numberOfDimensions(inType.clone()) >= (inArrayDim.clone().len() as i32) + (flat_id.clone().len() as i32)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        outDimensions = unwrap_break_err!(elabArraydimType2(inType.clone(), inArrayDim.clone(), flat_id.clone()), '__try0);
        Ok::<_, anyhow::Error>((outDimensions.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDimensions = __try0_o0;
        }
        Err(_) => {
            ad_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*Dump::printArraydimStr(inArrayDim.clone())?); ArcStr::from(__mm_s) }).clone();
            ty_str = (TypesDump::unparseTypeNoAttr(inType.clone())?).clone();
            exp_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            name_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(inPrefix.clone())?); __mm_s.push_str(&*Dump::printComponentRefStr(inCref.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessageAndFail(Error::MODIFIER_DECLARATION_TYPE_MISMATCH_ERROR.clone(), list![(name_str.clone()).clone(), (ad_str.clone()).clone(), (exp_str.clone()).clone(), (ty_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(outDimensions)
}

fn elabArraydimType2(mut inType: Arc<DAE::Type>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    outDimensions = 'mc: {
        let __mc_input = (inType.clone(), inArrayDim.clone(), inDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil }, ty: t }, _, Deref @ metamodelica::List::Cons { head: dim, tail: rest_dims }) => {
                    compatibleArraydim(d.clone(), dim.clone())?;
                    Ok(elabArraydimType2(t.clone(), inArrayDim.clone(), rest_dims.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil }, ty: t }, _, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::cons(d.clone(), elabArraydimType2(t.clone(), listRest(inArrayDim.clone())?, metamodelica::nil())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Undefined! The type detected: ")).clone())?;
                    Debug::traceln((TypesDump::printTypeStr(inType.clone())).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDimensions)
}

pub(crate) fn addFunctionsToDAE(mut inCache: FCore::Cache, mut funcs: Arc<metamodelica::List<DAE::Function>>, mut inPartialPrefix: SCode::Partial) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache;
    outCache = (match inCache {
        mut cache => {
            cache = FCore::addDaeFunction(cache.clone(), funcs)?;
            cache.clone()
        },
    });
    Ok(outCache)
}

pub(crate) fn addNameToDerivativeMapping(mut inElts: Arc<metamodelica::List<DAE::Function>>, mut path: Arc<Absyn::Path>) -> Arc<metamodelica::List<DAE::Function>> {
    let mut outElts: Arc<metamodelica::List<DAE::Function>>;
    outElts = ({
        let mut __acc: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
        for mut r#fn in (inElts).into_iter().cloned() {
            let __x = (match r#fn.clone() {
        DAE::Function::FUNCTION { .. } => {
            let __owned_variant_functions_0 = addNameToDerivativeMappingFunctionDefs(var_field!(r#fn.functions, DAE::Function::FUNCTION).clone(), path.clone());
            if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                *functions = __owned_variant_functions_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            r#fn.clone()
        },
        _ => r#fn.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElts
}

fn addNameToDerivativeMappingFunctionDefs(mut inFuncs: Arc<metamodelica::List<DAE::FunctionDefinition>>, mut path: Arc<Absyn::Path>) -> Arc<metamodelica::List<DAE::FunctionDefinition>> {
    let mut outFuncs: Arc<metamodelica::List<DAE::FunctionDefinition>>;
    outFuncs = ({
        let mut __acc: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
        for mut r#fn in (inFuncs).into_iter().cloned() {
            let __x = (match r#fn.clone() {
        DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. } => {
            let __owned_variant_lowerOrderDerivatives_0 = metamodelica::cons(path.clone(), var_field!(r#fn.lowerOrderDerivatives, DAE::FunctionDefinition::FUNCTION_DER_MAPPER).clone());
            if let DAE::FunctionDefinition::FUNCTION_DER_MAPPER { lowerOrderDerivatives, .. } = &mut r#fn {
                *lowerOrderDerivatives = __owned_variant_lowerOrderDerivatives_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_DER_MAPPER"); }
            r#fn.clone()
        },
        _ => r#fn.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outFuncs
}

pub(crate) fn getDeriveAnnotation(mut cd: Arc<SCode::ClassDef>, mut cmt: Arc<SCode::Comment>, mut baseFunc: Arc<Absyn::Path>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Arc<metamodelica::List<DAE::FunctionDefinition>> {
    let mut element: Arc<metamodelica::List<DAE::FunctionDefinition>>;
    element = 'mc: {
        let __mc_input = (cd, cmt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: elemDecl, externalDecl: Some(Deref @ SCode::ExternalDecl { annotation_: Some(ann), .. }), .. }, _) => {
                    Ok(getDeriveAnnotation2(ann.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: elemDecl, .. }, Deref @ SCode::Comment { annotation_: Some(ann), .. }) => {
                    Ok(getDeriveAnnotation2(ann.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    element
}

fn getDeriveAnnotation2(mut ann: Arc<SCode::Annotation>, mut elemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut baseFunc: Arc<Absyn::Path>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<Arc<metamodelica::List<DAE::FunctionDefinition>>> {
    let mut element: Arc<metamodelica::List<DAE::FunctionDefinition>>;
    element = (::match_deref::match_deref! { match &(ann) {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: smlst, .. } } => {
            getDeriveAnnotation3(smlst.clone(), elemDecl, baseFunc, inCache, inEnv, inIH, inPrefix, info)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

fn getDeriveAnnotation3(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut elemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut baseFunc: Arc<Absyn::Path>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<Arc<metamodelica::List<DAE::FunctionDefinition>>> {
    let mut element: Arc<metamodelica::List<DAE::FunctionDefinition>>;
    element = 'mc: {
        let __mc_input = inSubs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: Deref @ SCode::Mod::MOD { subModLst: subs2, binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs } => {
                    let mut deriveFunc: Arc<Absyn::Path>;
                    let mut defaultDerivative: Option<Arc<Absyn::Path>>;
                    let mut order: i32;
                    let mut conditionRefs: Arc<metamodelica::List<(i32, DAE::derivativeCond)>>;
                    let mut mapper: DAE::FunctionDefinition;
                    deriveFunc = AbsynUtil::crefToPath(acr.clone())?;
                    (_, deriveFunc) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), deriveFunc.clone())?;
                    order = getDerivativeOrder(subs2.clone());
                    ErrorExt::setCheckpoint((literal!("getDeriveAnnotation3")).clone());
                    conditionRefs = getDeriveCondition(subs2.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone());
                    ErrorExt::rollBack((literal!("getDeriveAnnotation3")).clone());
                    conditionRefs = List::sort(conditionRefs.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::derivativeOrder, (i32, DAE::derivativeCond), (i32, DAE::derivativeCond))) as std::sync::Arc<dyn ::std::ops::Fn((i32, DAE::derivativeCond), (i32, DAE::derivativeCond)) -> Result<bool> + 'static>))?;
                    defaultDerivative = getDerivativeSubModsOptDefault(subs.clone(), inCache.clone(), inEnv.clone(), inPrefix.clone());
                    mapper = DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: baseFunc.clone(), derivativeFunction: deriveFunc.clone(), derivativeOrder: order.clone(), conditionRefs: conditionRefs.clone(), defaultDerivative: defaultDerivative.clone(), lowerOrderDerivatives: metamodelica::nil() };
                    Ok(list![mapper.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: subs } => {
                    Ok(getDeriveAnnotation3(subs.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(element)
}

fn getDeriveCondition(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut elemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Arc<metamodelica::List<(i32, DAE::derivativeCond)>> {
    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = metamodelica::nil();
    outconds = 'mc: {
        let __mc_input = inSubs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "noDerivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs } => {
                    let mut name: ArcStr;
                    let mut varPos: i32;
                    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = outconds.clone();
                    name = (Dump::printComponentRefStr(acr.clone())?).clone();
                    outconds = getDeriveCondition(subs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone());
                    varPos = setFunctionInputIndex(elemDecl.clone(), (name.clone()).clone(), 1)?;
                    Ok((metamodelica::cons((varPos.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: Arc::new(DAE::Exp::ICONST { integer: 99 }) }), outconds.clone()), outconds.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outconds = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "zeroDerivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs } => {
                    let mut name: ArcStr;
                    let mut varPos: i32;
                    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = outconds.clone();
                    name = (Dump::printComponentRefStr(acr.clone())?).clone();
                    outconds = getDeriveCondition(subs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone());
                    varPos = setFunctionInputIndex(elemDecl.clone(), (name.clone()).clone(), 1)?;
                    Ok((metamodelica::cons((varPos.clone(), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE), outconds.clone()), outconds.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outconds = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "noDerivative", r#mod: m @ Deref @ SCode::Mod::MOD { .. } }, tail: subs } => {
                    let mut sub: Arc<DAE::SubMod>;
                    let mut name: ArcStr;
                    let mut cond: DAE::derivativeCond;
                    let mut varPos: i32;
                    let mut cache: FCore::Cache;
                    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = outconds.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), m.clone(), false, Mod::ModScope::COMPONENT { name: (literal!("noDerivative")).clone() }, info.clone())?) {
                        (__pa0, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    sub = __pa1.clone();
                    (name, cond) = extractNameAndExp(sub.clone());
                    outconds = getDeriveCondition(subs.clone(), elemDecl.clone(), cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone());
                    varPos = setFunctionInputIndex(elemDecl.clone(), (name.clone()).clone(), 1)?;
                    Ok((metamodelica::cons((varPos.clone(), cond.clone()), outconds.clone()), outconds.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outconds = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: subs } => {
                    Ok(getDeriveCondition(subs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outconds
}

fn setFunctionInputIndex(mut inElemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut r#str: ArcStr, mut currPos: i32) -> Result<i32> {
    let mut index: i32;
    index = 'mc: {
        let __mc_input = inElemDecl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" failure in setFunctionInputIndex, didn't find any index for: ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: str2, attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. }, tail: _ } => {
                    let true = (stringEq((str2.clone()).clone(), (r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(currPos)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. }, tail: elemDecl } => {
                    Ok(setFunctionInputIndex(elemDecl.clone(), (r#str.clone()).clone(), currPos + 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: elemDecl } => {
                    Ok(setFunctionInputIndex(elemDecl.clone(), (r#str.clone()).clone(), currPos)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(index)
}

fn extractNameAndExp(mut m: Arc<DAE::SubMod>) -> (ArcStr, DAE::derivativeCond) {
    let mut inputVar: ArcStr = arcstr::literal!("");
    let mut cond: DAE::derivativeCond;
    (inputVar, cond) = (::match_deref::match_deref! { match &(m) {
        Deref @ DAE::SubMod { ident: __esc_inputVar, r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, .. }), .. } } => {
            inputVar = (*__esc_inputVar).clone();
            (inputVar.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: e.clone() })
        },
        Deref @ DAE::SubMod { ident: __esc_inputVar, r#mod: Deref @ DAE::Mod::MOD { binding: None, .. } } => {
            inputVar = (*__esc_inputVar).clone();
            (inputVar.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: Arc::new(DAE::Exp::ICONST { integer: 1 }) })
        },
        Deref @ DAE::SubMod { ident: __esc_inputVar, r#mod: Deref @ DAE::Mod::MOD { binding: None, .. } } => {
            inputVar = (*__esc_inputVar).clone();
            (inputVar.clone(), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE)
        },
        _ => {
            (literal!(""), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (inputVar, cond)
}

fn getDerivativeSubModsOptDefault(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix) -> Option<Arc<Absyn::Path>> {
    let mut defaultDerivative: Option<Arc<Absyn::Path>>;
    defaultDerivative = 'mc: {
        let __mc_input = inSubs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: _ } => {
                    let mut p: Arc<Absyn::Path>;
                    p = AbsynUtil::crefToPath(acr.clone())?;
                    (_, p) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), p.clone())?;
                    Ok(Some(p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: subs } => {
                    Ok(getDerivativeSubModsOptDefault(subs.clone(), inCache.clone(), inEnv.clone(), inPrefix.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    defaultDerivative
}

fn getDerivativeOrder(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> i32 {
    let mut order: i32 = 0;
    order = (::match_deref::match_deref! { match &(inSubs) {
        Deref @ metamodelica::List::Nil => {
            1
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "order", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::INTEGER { value: __esc_order }), .. } }, tail: _ } => {
            order = (*__esc_order).clone();
            order.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: subs } => {
            getDerivativeOrder(subs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    order
}

pub(crate) fn setFullyQualifiedTypename(mut inType: Arc<DAE::Type>, mut path: Arc<Absyn::Path>) -> Arc<DAE::Type> {
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    resType = (::match_deref::match_deref! { match &(inType.clone()) {
        __esc_resType @ Deref @ DAE::Type::T_FUNCTION { .. } => {
            resType = (*__esc_resType).clone();
            assign_variant_field!(resType => DAE::Type::T_FUNCTION; path = path);
            resType.clone()
        },
        _ => inType,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    resType
}

pub(crate) fn classIsInlineFunc(mut elt: Arc<SCode::Element>) -> DAE::InlineType {
    let mut outInlineType: DAE::InlineType;
    outInlineType = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { .. } => InstBasics::commentIsInlineFunc(var_field!((*elt).cmt, SCode::Element::CLASS).clone()),
        _ => openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInlineType
}

pub(crate) fn stripFuncOutputsMod(mut elem: Arc<SCode::Element>) -> Arc<SCode::Element> {
    let mut stripped_elem: Arc<SCode::Element>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    stripped_elem = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, modifications: __esc_mod @ Deref @ SCode::Mod::MOD { binding: Some(_), .. }, .. } => {
            r#mod = (*__esc_mod).clone();
            assign_variant_field!(r#mod => SCode::Mod::MOD; binding = None);
            assign_variant_field!(elem => SCode::Element::COMPONENT; modifications = r#mod.clone());
            elem
        },
        _ => elem,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stripped_elem
}

pub(crate) fn checkExternalFunction(mut els: Arc<metamodelica::List<Arc<DAE::Element>>>, mut decl: DAE::ExternalDecl, mut name: ArcStr) -> Result<()> {
    if decl.language.clone() == literal!("builtin") {
        return Ok(());
    }
    List::map2_0(els.clone(), (std::sync::Arc::new(checkExternalFunctionOutputAssigned) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::ExternalDecl, ArcStr) -> Result<()> + 'static>), decl.clone(), (name.clone()).clone())?;
    checkFunctionInputUsed(els, Some(decl), (name).clone())?;
    Ok(())
}

pub(crate) fn checkFunctionInputUsed(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut decl: Option<DAE::ExternalDecl>, mut name: ArcStr) -> Result<()> {
    let mut invars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut algs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    (vars, _, _, _, algs, _, _, _, _, _) = DAEUtil::splitElements(elts)?;
    invars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isInputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    invars = List::select(invars, (std::sync::Arc::new(checkInputUsedAnnotation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    invars = checkExternalDeclInputUsed(invars, decl)?;
    invars = List::select1(invars, (std::sync::Arc::new(checkVarBindingsInputUsed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<bool> + 'static>), vars)?;
    let (_, (_, __pa0)) = DAEUtil::traverseDAEElementList(algs, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(checkExpInputUsed, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)> + 'static>), invars))?;
    invars = __pa0.clone();
    List::map1_0(invars, (std::sync::Arc::new(warnUnusedFunctionVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, ArcStr) -> Result<()> + 'static>), (name).clone())?;
    Ok(())
}

pub(crate) fn checkInputUsedAnnotation(mut inElement: Arc<DAE::Element>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::VAR { comment: cmt, .. } => {
            result = SCodeUtil::optCommentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_UnusedVariable")).clone())?;
            !(result)
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn warnUnusedFunctionVar(mut v: Arc<DAE::Element>, mut name: ArcStr) -> Result<()> {
    let mut cr: Arc<DAE::ComponentRef>;
    let mut source: Arc<DAE::ElementSource>;
    let mut r#str: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(v) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, source: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    source = __pa1.clone();
    r#str = (ComponentReferenceBasics::printComponentRefStr(cr)?).clone();
    Error::addSourceMessage(Error::FUNCTION_UNUSED_INPUT.clone(), list![(r#str).clone(), (name).clone()], ElementSource::getElementSourceFileInfo(source))?;
    Ok(())
}

fn checkExternalDeclInputUsed(mut inames: Arc<metamodelica::List<Arc<DAE::Element>>>, mut decl: Option<DAE::ExternalDecl>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut onames: Arc<metamodelica::List<Arc<DAE::Element>>>;
    onames = (::match_deref::match_deref! { match &((inames, decl)) {
        (names, None) => {
            names.clone()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (names, Some(DAE::ExternalDecl { returnArg: arg, args, .. })) => {
            let mut names = (*names).clone();
            names = List::select1(names.clone(), (std::sync::Arc::new(checkExternalDeclArgs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<metamodelica::List<DAE::ExtArg>>) -> Result<bool> + 'static>), metamodelica::cons(arg.clone(), args.clone()))?;
            names.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(onames)
}

fn checkExpInputUsed(mut inExp: Arc<DAE::Exp>, mut inEls: Arc<metamodelica::List<Arc<DAE::Element>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>) {
    let mut exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (exp, els) = 'mc: {
        let __mc_input = (inExp.clone(), inEls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, els) => {
                    let mut els = (*els).clone();
                    els = List::select1(els.clone(), (std::sync::Arc::new(checkExpInputUsed3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?;
                    Ok((exp.clone(), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CALL { path, .. }, els) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut els = (*els).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    cr = ComponentReference::pathToCref(path.clone())?;
                    els = List::select1(els.clone(), (std::sync::Arc::new(checkExpInputUsed3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?;
                    Ok((exp.clone(), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inEls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (exp, els)
}

fn checkExpInputUsed3(mut el: Arc<DAE::Element>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut noteq: bool;
    let mut cr1: Arc<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(el) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr1 = __pa0.clone();
    noteq = !(ComponentReferenceBasics::crefEqualNoStringCompare(cr1, cr2)?);
    Ok(noteq)
}

fn checkVarBindingsInputUsed(mut v: Arc<DAE::Element>, mut els: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<bool> {
    let mut notfound: bool;
    notfound = !(List::isMemberOnTrue(v, els, (std::sync::Arc::new(checkVarBindingInputUsed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<bool> + 'static>))?);
    Ok(notfound)
}

fn checkVarBindingInputUsed(mut v: Arc<DAE::Element>, mut el: Arc<DAE::Element>) -> Result<bool> {
    let mut found: bool;
    found = (::match_deref::match_deref! { match &((v, el)) {
        (Deref @ DAE::Element::VAR { .. }, Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }) => {
            false
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, Deref @ DAE::Element::VAR { binding: Some(exp), .. }) => {
            Expression::expHasCref(exp.clone(), cr.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(found)
}

fn checkExternalDeclArgs(mut v: Arc<DAE::Element>, mut args: Arc<metamodelica::List<DAE::ExtArg>>) -> Result<bool> {
    let mut notfound: bool;
    notfound = !(List::isMemberOnTrue(v, args, (std::sync::Arc::new(extArgCrefEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::ExtArg) -> Result<bool> + 'static>))?);
    Ok(notfound)
}

fn checkExternalFunctionOutputAssigned(mut v: Arc<DAE::Element>, mut decl: DAE::ExternalDecl, mut name: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((v.clone(), decl)) {
        (Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT { .. }, componentRef: cr, binding, source, .. }, DAE::ExternalDecl { returnArg: arg, args, .. }) => {
            let mut r#str: ArcStr;
            if !(List::isMemberOnTrue(v, metamodelica::cons(arg.clone(), args.clone()), (std::sync::Arc::new(extArgCrefEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::ExtArg) -> Result<bool> + 'static>))? || isSome(binding.clone())) {
                r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                Error::addSourceMessage(Error::EXTERNAL_NOT_SINGLE_RESULT.clone(), list![(r#str.clone()).clone(), (name).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                bail!("fail");
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn extArgCrefEq(mut v: Arc<DAE::Element>, mut arg: DAE::ExtArg) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((v, arg)) {
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARG { componentRef: cr2, .. }) => {
            let mut cr2 = (*cr2).clone();
            cr2 = ComponentReferenceBasics::crefFirstCref(cr2.clone())?;
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT { .. }, .. }, _) => {
            false
        },
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARGSIZE { componentRef: cr2, .. }) => {
            let mut cr2 = (*cr2).clone();
            cr2 = ComponentReferenceBasics::crefFirstCref(cr2.clone())?;
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARGEXP { exp, .. }) => {
            Expression::expHasCref(exp.clone(), cr1.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isExtExplicitCall(mut inExternalDecl: Arc<SCode::ExternalDecl>) -> bool {
    let mut isExplicit: bool;
    isExplicit = (::match_deref::match_deref! { match &(inExternalDecl) {
        Deref @ SCode::ExternalDecl { funcName: Some(_), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExplicit
}

fn isInoutVar(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = isOutputVar(inElement.clone()) || isInputVar(inElement);
    b
}

fn isOutputVar(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inElement) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isInputVar(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inElement) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn instExtGetFname(mut inExternalDecl: Arc<SCode::ExternalDecl>, mut inIdent: ArcStr) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &((inExternalDecl, inIdent)) {
        (Deref @ SCode::ExternalDecl { funcName: Some(id), .. }, _) => {
            id.clone()
        },
        (Deref @ SCode::ExternalDecl { funcName: None, .. }, fid) => {
            fid.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

pub(crate) fn instExtGetAnnotation(mut inExternalDecl: Arc<SCode::ExternalDecl>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut outAnnotation: Option<Arc<SCode::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(inExternalDecl) {
        Deref @ SCode::ExternalDecl { annotation_: ann, .. } => {
            ann.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

pub(crate) fn instExtGetLang(mut inExternalDecl: Arc<SCode::ExternalDecl>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inExternalDecl) {
        Deref @ SCode::ExternalDecl { lang: Some(lang), .. } => {
            lang.clone()
        },
        Deref @ SCode::ExternalDecl { lang: None, .. } => {
            literal!("C")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn elabExpListExt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<DAE::Properties>>)> {
    let mut outCache: FCore::Cache;
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut outTypesPropertiesLst: Arc<metamodelica::List<DAE::Properties>>;
    (outCache, outExpExpLst, outTypesPropertiesLst) = (::match_deref::match_deref! { match &((inCache, inEnv, inAbsynExpLst, inBoolean, inPrefix)) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _) => {
            (cache.clone(), metamodelica::nil(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: e, tail: rest }, r#impl, pre) => {
            let mut exp: Arc<DAE::Exp>;
            let mut p: DAE::Properties;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut props: Arc<metamodelica::List<DAE::Properties>>;
            let mut cache = (*cache).clone();
            (cache, exp, p) = elabExpExt(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache, exps, props) = elabExpListExt(cache.clone(), env.clone(), rest.clone(), r#impl.clone(), pre.clone(), info)?;
            (cache.clone(), metamodelica::cons(exp.clone(), exps.clone()), metamodelica::cons(p.clone(), props.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExpExpLst, outTypesPropertiesLst))
}

fn elabExpExt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "size", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: arraycr, tail: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }, r#impl, pre) => {
                    let mut dimp: Arc<DAE::Exp>;
                    let mut arraycrefe: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut arraycrprop: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, __pa2 @ DAE::Properties::PROP { type_: _, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dimp = __pa1.clone();
                    prop = __pa2.clone();
                    (cache, dimp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), dimp.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, arraycrefe, arraycrprop) = Static::elabExp(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, arraycrefe, arraycrprop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), arraycrefe.clone(), arraycrprop.clone(), r#impl.clone(), info.clone())?;
                    exp = Arc::new(DAE::Exp::SIZE { exp: arraycrefe.clone(), sz: Some(dimp.clone()) });
                    Ok((cache.clone(), exp.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, absynExp, r#impl, pre) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e, prop) = Static::elabExp(cache.clone(), env.clone(), absynExp.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, e, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), e.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("-Inst.elabExpExt failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

pub(crate) fn instExtGetFargs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExternalDecl: Arc<SCode::ExternalDecl>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<DAE::ExtArg>>)> {
    let mut outCache: FCore::Cache;
    let mut outDAEExtArgLst: Arc<metamodelica::List<DAE::ExtArg>>;
    (outCache, outDAEExtArgLst) = 'mc: {
        let __mc_input = (inCache, inEnv, inExternalDecl, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::ExternalDecl { lang, args: absexps, .. }, r#impl, pre) => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut props: Arc<metamodelica::List<DAE::Properties>>;
                    let mut extargs: Arc<metamodelica::List<DAE::ExtArg>>;
                    let mut cache = (*cache).clone();
                    (cache, exps, props) = elabExpListExt(cache.clone(), env.clone(), absexps.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, extargs) = instExtGetFargs2(cache.clone(), env.clone(), absexps.clone(), exps.clone(), props.clone(), lang.clone(), info.clone())?;
                    Ok((cache.clone(), extargs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- InstUtil.instExtGetFargs failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDAEExtArgLst))
}

fn instExtGetFargs2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut absynExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypesPropertiesLst: Arc<metamodelica::List<DAE::Properties>>, mut lang: Option<ArcStr>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<DAE::ExtArg>>)> {
    let mut outCache: FCore::Cache;
    let mut outDAEExtArgLst: Arc<metamodelica::List<DAE::ExtArg>>;
    (outCache, outDAEExtArgLst) = (::match_deref::match_deref! { match &((inCache, inEnv, absynExps, inExpExpLst, inTypesPropertiesLst)) {
        (cache, _, _, Deref @ metamodelica::List::Nil, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: ae, tail: aes }, Deref @ metamodelica::List::Cons { head: e, tail: exps }, Deref @ metamodelica::List::Cons { head: p, tail: props }) => {
            let mut extargs: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut extarg: DAE::ExtArg;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(instExtGetFargsSingle(cache.clone(), env.clone(), ae.clone(), e.clone(), p.clone(), lang.clone(), info.clone())?) {
                (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            extarg = __pa1.clone();
            (cache, extargs) = instExtGetFargs2(cache.clone(), env.clone(), aes.clone(), exps.clone(), props.clone(), lang, info)?;
            (cache.clone(), metamodelica::cons(extarg.clone(), extargs.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outDAEExtArgLst))
}

fn instExtGetFargsSingle(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut absynExp: Arc<Absyn::Exp>, mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut lang: Option<ArcStr>, mut info: SourceInfo) -> Result<(FCore::Cache, Option<DAE::ExtArg>)> {
    let mut outCache: FCore::Cache;
    let mut outExtArg: Option<DAE::ExtArg>;
    (outCache, outExtArg) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), absynExp, inExp.clone(), inProperties.clone(), lang);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Exp::CREF { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, .. }, DAE::Properties::PROP { constFlag: DAE::Const::C_VAR { .. }, .. }, _) => {
                    let mut fattr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type>;
                    let mut fcr: Arc<DAE::ComponentRef>;
                    let mut cache: FCore::Cache;
                    (cache, _, ty, _, _, _, _, _, _) = Lookup::lookupVarLocal(inCache.clone(), inEnv.clone(), cref.clone())?;
                    fcr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    (cache, fattr, _, _, _, _, _, _, _) = Lookup::lookupVarLocal(cache.clone(), inEnv.clone(), fcr.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARG { componentRef: cref.clone(), direction: DAEUtil::getAttrDirection(fattr.clone()), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Exp::CREF { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, .. }, DAE::Properties::PROP { constFlag: DAE::Const::C_VAR { .. }, .. }, _) => {
                    let mut attr: Arc<DAE::Attributes>;
                    let mut ty: Arc<DAE::Type>;
                    let mut cache: FCore::Cache;
                    (cache, attr, ty, _, _, _, _, _, _) = Lookup::lookupVarLocal(inCache.clone(), inEnv.clone(), cref.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARG { componentRef: cref.clone(), direction: DAEUtil::getAttrDirection(attr.clone()), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ DAE::Exp::CREF { componentRef: cref, .. }, DAE::Properties::PROP { .. }, _) => {
                    let mut crefstr: ArcStr;
                    let mut scope: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupVarLocal(cache.clone(), env.clone(), cref.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    crefstr = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    scope = (FGraph::printGraphPathStr(env.clone())).clone();
                    Error::addMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(crefstr.clone()).clone(), (scope.clone()).clone()])?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ DAE::Exp::SIZE { exp: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, sz: Some(dim) }, DAE::Properties::PROP { .. }, _) => {
                    let mut varty: Arc<DAE::Type>;
                    let mut cache = (*cache).clone();
                    (cache, _, varty, _, _, _, _, _, _) = Lookup::lookupVarLocal(cache.clone(), env.clone(), cref.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGSIZE { componentRef: cref.clone(), type_: varty.clone(), exp: dim.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, DAE::Properties::PROP { type_: ty, constFlag: DAE::Const::C_CONST { .. } }, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, exp, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), inExp.clone(), inProperties.clone(), false, info.clone())?;
                    let true = (Expression::isScalarConst(exp.clone())) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: exp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, DAE::Properties::PROP { type_: ty, .. }, Some(Deref @ "builtin")) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, DAE::Properties::PROP { type_: ty, .. }, _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::CREF { componentRef: _ }, _, DAE::Properties::PROP { type_: ty, .. }, _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, exp, DAE::Properties::PROP { .. }, _) => {
                    let mut r#str: ArcStr;
                    r#str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Error::addSourceMessage(Error::EXTERNAL_ARG_WRONG_EXP.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExtArg))
}

pub(crate) fn instExtGetRettype(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExternalDecl: Arc<SCode::ExternalDecl>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, DAE::ExtArg)> {
    let mut outCache: FCore::Cache;
    let mut outExtArg: DAE::ExtArg;
    (outCache, outExtArg) = 'mc: {
        let __mc_input = (inCache, inEnv, inExternalDecl, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ SCode::ExternalDecl { output_: None, .. }, _, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::ExtArg::NOEXTARG))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::ExternalDecl { lang, output_: Some(cref), .. }, r#impl, pre) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut extarg: DAE::ExtArg;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cref.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2, _))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp = __pa1.clone();
                    prop = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(instExtGetFargsSingle(cache.clone(), env.clone(), Arc::new(Absyn::Exp::CREF { componentRef: cref.clone() }), exp.clone(), prop.clone(), lang.clone(), info.clone())?) {
                        (__pa3, Some(__pa4)) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    extarg = __pa4.clone();
                    assertExtArgOutputIsCrefVariable(lang.clone(), extarg.clone(), Types::getPropType(prop.clone())?, Types::propAllConst(prop.clone())?, info.clone())?;
                    Ok((cache.clone(), extarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- InstUtil.instExtRettype failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExtArg))
}

fn assertExtArgOutputIsCrefVariable(mut lang: Option<ArcStr>, mut arg: DAE::ExtArg, mut ty: Arc<DAE::Type>, mut c: DAE::Const, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((lang, arg.clone(), ty.clone(), c)) {
        (Some(Deref @ "builtin"), _, _, _) => {
            ()
        },
        (_, _, Deref @ DAE::Type::T_ARRAY { .. }, _) => {
            let mut r#str: ArcStr;
            r#str = (TypesDump::unparseType(ty)?).clone();
            Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE.clone(), list![(r#str.clone()).clone()], info)?;
            bail!("fail")
        },
        (_, DAE::ExtArg::EXTARG { .. }, _, DAE::Const::C_VAR { .. }) => {
            ()
        },
        (_, _, _, DAE::Const::C_VAR { .. }) => {
            let mut r#str: ArcStr;
            r#str = (DAEDump::dumpExtArgStr(arg)?).clone();
            Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_NOT_CREF.clone(), list![(r#str.clone()).clone()], info)?;
            bail!("fail")
        },
        _ => {
            Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_NOT_VAR.clone(), metamodelica::nil(), info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn makeDaeProt(mut visibility: SCode::Visibility) -> Result<DAE::VarVisibility> {
    let mut res: DAE::VarVisibility;
    res = (match visibility {
        SCode::Visibility::PROTECTED { .. } => openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
        SCode::Visibility::PUBLIC { .. } => openmodelica_frontend_types::DAE::VarVisibility::PUBLIC,
    });
    Ok(res)
}

pub(crate) fn makeDaeVariability(mut inVariability: SCode::Variability) -> Result<DAE::VarKind> {
    let mut outVariability: DAE::VarKind;
    outVariability = (match inVariability {
        SCode::Variability::VAR { .. } => openmodelica_frontend_types::DAE::VarKind::VARIABLE,
        SCode::Variability::PARAM { .. } => openmodelica_frontend_types::DAE::VarKind::PARAM,
        SCode::Variability::CONST { .. } => openmodelica_frontend_types::DAE::VarKind::CONST,
        SCode::Variability::DISCRETE { .. } => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
    });
    Ok(outVariability)
}

pub(crate) fn makeDaeDirection(mut inDirection: Absyn::Direction) -> Result<DAE::VarDirection> {
    let mut outDirection: DAE::VarDirection;
    outDirection = (match inDirection {
        Absyn::Direction::INPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Absyn::Direction::OUTPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        Absyn::Direction::BIDIR { .. } => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        _ => bail!("match: no arm matched"),
    });
    Ok(outDirection)
}

pub(crate) fn mktype(mut inPath: Arc<Absyn::Path>, mut inState: ClassInf::State, mut inTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inTypesTypeOption: Option<Arc<DAE::Type>>, mut inEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, mut inClass: Arc<SCode::Element>, mut inheritedComment: Arc<SCode::Comment>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inPath.clone(), inState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_INTEGER { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_INTEGER { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_REAL { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_REAL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_STRING { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_STRING { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_BOOL { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_BOOL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_CLOCK { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_CLOCK { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_ENUM { .. }, _, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::FUNCTION { .. }, vl, _, _, cl) => {
                    let mut functype: Arc<DAE::Type>;
                    let mut funcattr: DAE::FunctionAttributes;
                    funcattr = getFunctionAttributes(cl.clone(), vl.clone(), inheritedComment.clone())?;
                    functype = Types::makeFunctionType(p.clone(), vl.clone(), funcattr.clone())?;
                    Ok(functype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::ENUMERATION { path: p }, _, Some(enumtype), _, _) => {
                    let mut enumtype = (*enumtype).clone();
                    enumtype = Types::makeEnumerationType(p.clone(), enumtype.clone())?;
                    Ok(enumtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE { .. }, _, Some(Deref @ DAE::Type::T_ARRAY { ty: arrayType, .. }), None, _) => {
                    let mut resType: Arc<DAE::Type>;
                    let mut classState: ClassInf::State;
                    classState = arrayTTypeToClassInfState(arrayType.clone())?;
                    resType = mktype(inPath.clone(), classState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone(), inheritedComment.clone())?;
                    Ok(resType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE { .. }, _, Some(Deref @ DAE::Type::T_ARRAY { ty: arrayType, .. }), Some(_), _) => {
                    let mut resType: Arc<DAE::Type>;
                    let mut classState: ClassInf::State;
                    classState = arrayTTypeToClassInfState(arrayType.clone())?;
                    resType = mktype(inPath.clone(), classState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone(), inheritedComment.clone())?;
                    resType = Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: inState.clone(), varLst: metamodelica::nil(), complexType: resType.clone(), equalityConstraint: inEqualityConstraint.clone() });
                    Ok(resType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_TUPLE { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_OPTION { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_LIST { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_POLYMORPHIC { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_ARRAY { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_UNIONTYPE { path: _, .. }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::META_UNIONTYPE { path: _, .. }, _, _, _, _) => {
                    let mut pstr: ArcStr;
                    let mut info: SourceInfo;
                    pstr = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    info = SCodeUtil::elementInfo(inClass.clone());
                    Error::addSourceMessage(Error::META_UNIONTYPE_ALIAS_MODS.clone(), list![(pstr.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, None, equalityConstraint, _) => {
                    if '__try0: {
                        let ClassInf::META_UNIONTYPE { path: _, .. } = (st.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: st.clone(), varLst: l.clone(), equalityConstraint: equalityConstraint.clone(), usedExternally: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, Some(bc), equalityConstraint, _) => {
                    if '__try0: {
                        let ClassInf::META_UNIONTYPE { path: _, .. } = (st.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: st.clone(), varLst: l.clone(), complexType: bc.clone(), equalityConstraint: equalityConstraint.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn arrayTTypeToClassInfState(mut arrayType: Arc<DAE::Type>) -> Result<ClassInf::State> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(arrayType) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            return Ok(ClassInf::State::TYPE_INTEGER { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            return Ok(ClassInf::State::TYPE_REAL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            return Ok(ClassInf::State::TYPE_STRING { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            return Ok(ClassInf::State::TYPE_BOOL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) })
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            return Ok(ClassInf::State::TYPE_CLOCK { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) })
        },
        Deref @ DAE::Type::T_ARRAY { ty: t, .. } => {
            let mut cs: ClassInf::State;
            { arrayType = t.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn mktypeWithArrays(mut inPath: Arc<Absyn::Path>, mut inState: ClassInf::State, mut inTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inTypesTypeOption: Option<Arc<DAE::Type>>, mut inClass: Arc<SCode::Element>, mut inheritedComment: Arc<SCode::Comment>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inPath, inState, inTypesVarLst, inTypesTypeOption, inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ci, _, Some(tp), _) => {
                    let true = (Types::isArray(tp.clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(ClassInfUtil::isConnector(ci.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_INTEGER { .. }, v, _, _) => {
                    getOptPath(p.clone());
                    Ok(Arc::new(DAE::Type::T_INTEGER { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_REAL { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_REAL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_STRING { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_STRING { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_BOOL { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_BOOL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_CLOCK { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_CLOCK { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_ENUM { .. }, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::FUNCTION { .. }, vl, _, cl) => {
                    let mut functype: Arc<DAE::Type>;
                    let mut funcattr: DAE::FunctionAttributes;
                    funcattr = getFunctionAttributes(cl.clone(), vl.clone(), inheritedComment.clone())?;
                    functype = Types::makeFunctionType(p.clone(), vl.clone(), funcattr.clone())?;
                    Ok(functype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::ENUMERATION { .. }, _, Some(enumtype), _) => {
                    let mut enumtype = (*enumtype).clone();
                    enumtype = Types::makeEnumerationType(p.clone(), enumtype.clone())?;
                    Ok(enumtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, None, _) => {
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: st.clone(), varLst: l.clone(), equalityConstraint: None, usedExternally: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, Some(bc), _) => {
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: st.clone(), varLst: l.clone(), complexType: bc.clone(), equalityConstraint: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("InstUtil.mktypeWithArrays failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn getOptPath(mut inPath: Arc<Absyn::Path>) -> Option<Arc<Absyn::Path>> {
    let mut outAbsynPathOption: Option<Arc<Absyn::Path>>;
    outAbsynPathOption = (::match_deref::match_deref! { match &(inPath) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => {
            None
        },
        p => {
            Some(p.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynPathOption
}

fn checkProt(mut inVisibility: SCode::Visibility, mut inMod: Arc<DAE::Mod>, mut inComponentRef: Arc<DAE::ComponentRef>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inVisibility, inMod.clone(), inComponentRef)) {
        (SCode::Visibility::PUBLIC { .. }, _, _) => {
            ()
        },
        (_, Deref @ DAE::Mod::NOMOD { .. }, _) => {
            ()
        },
        (_, Deref @ DAE::Mod::MOD { finalPrefix: _, eachPrefix: _, subModLst: Deref @ metamodelica::List::Nil, binding: None, .. }, _) => {
            ()
        },
        (SCode::Visibility::PROTECTED { .. }, _, cref) => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
            str2 = (Mod::prettyPrintMod(inMod, 0)?).clone();
            Error::addSourceMessage(Error::MODIFY_PROTECTED.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn getStateSelectFromExpOption(mut inExpExpOption: Option<Arc<DAE::Exp>>) -> Option<DAE::StateSelect> {
    let mut outDAEStateSelectOption: Option<DAE::StateSelect>;
    outDAEStateSelectOption = (::match_deref::match_deref! { match &(inExpExpOption) {
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "never" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::NEVER),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "avoid" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::AVOID),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "default" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::DEFAULT),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "prefer" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::PREFER),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "always" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::ALWAYS),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDAEStateSelectOption
}

pub(crate) fn isSubModNamed(mut inName: ArcStr, mut inSubMod: Arc<DAE::SubMod>) -> bool {
    let mut isNamed: bool;
    isNamed = (::match_deref::match_deref! { match &(inSubMod) {
        Deref @ DAE::SubMod { ident: submod_name, .. } => {
            stringEqual((inName).clone(), (submod_name.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

pub(crate) fn liftRecordBinding(mut inType: Arc<DAE::Type>, mut inExp: Arc<DAE::Exp>, mut inValue: Arc<Values::Value>) -> Result<(Arc<DAE::Exp>, Arc<Values::Value>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outValue: Arc<Values::Value>;
    (outExp, outValue) = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, ty } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut val: Arc<Values::Value>;
                    let mut ety: Arc<DAE::Type>;
                    let mut int_dim: i32;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>>;
                    int_dim = Expression::dimensionSize(dim.clone())?;
                    (exp, val) = liftRecordBinding(ty.clone(), inExp.clone(), inValue.clone())?;
                    ety = Types::simplifyType(inType.clone())?;
                    expl = List::fill(exp.clone(), int_dim.clone());
                    vals = List::fill(val.clone(), int_dim.clone());
                    exp = Arc::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: true, array: expl.clone() });
                    val = Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![int_dim.clone()] });
                    Ok((exp.clone(), val.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Types::isArray(inType.clone())) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), inValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outValue))
}

pub(crate) fn isTopCall(mut inCallingScope: InstTypes::CallingScope) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inCallingScope {
        InstTypes::CallingScope::TOP_CALL { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn extractCurrentName(mut sele: Arc<SCode::Element>) -> Result<(ArcStr, SourceInfo)> {
    let mut ostring: ArcStr;
    let mut oinfo: SourceInfo;
    (ostring, oinfo) = (::match_deref::match_deref! { match &(sele) {
        Deref @ SCode::Element::CLASS { name, info, .. } => {
            (name.clone(), info.clone())
        },
        Deref @ SCode::Element::COMPONENT { name, info, .. } => {
            (name.clone(), info.clone())
        },
        Deref @ SCode::Element::EXTENDS { baseClassPath: path, info, .. } => {
            let mut ret: ArcStr;
            ret = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            (ret.clone(), info.clone())
        },
        Deref @ SCode::Element::IMPORT { imp, info, .. } => {
            let mut name: ArcStr;
            name = (AbsynUtil::printImportString(imp.clone())?).clone();
            (name.clone(), info.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((ostring, oinfo))
}

pub(crate) fn reorderConnectEquationsExpandable(mut cache: FCore::Cache, mut env: FCore::Graph, mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<SCode::Equation>>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut delst: DoubleEnded::MutableList<Arc<SCode::Equation>>;
    let mut expandableEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut crefLeft: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut crefRight: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if if (inEquations.clone().is_empty()) {true} else {!(System::getHasExpandableConnectors())} {
        outEquations = inEquations;
        return Ok((cache.clone(), outEquations.clone()));
    }
    ErrorExt::setCheckpoint((literal!("expandableConnectorsOrder")).clone());
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    expandableEqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut eq in (inEquations.clone()).into_iter().cloned() {
            if !('mc: {
        let __mc_input = eq.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_CONNECT { crefLeft, crefRight, .. } => {
                    let mut ty1: Arc<DAE::Type> = ty1.clone();
                    let mut ty2: Arc<DAE::Type> = ty2.clone();
                    (_, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), ComponentReference::toExpCref(crefLeft.clone())?, true)?;
                    let true = (Types::isExpandableConnector(ty1.clone())) else { bail!("pattern mismatch") };
                    (_, ty2, _, _) = Lookup::lookupConnectorVar(env.clone(), ComponentReference::toExpCref(crefRight.clone())?, true)?;
                    let true = (Types::isExpandableConnector(ty2.clone())) else { bail!("pattern mismatch") };
                    Ok((true, ty1.clone(), ty2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { ty1 = __wb0; ty2 = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    DoubleEnded::push_back(delst.clone(), eq.clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }) { continue; }
            let __x = eq.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if expandableEqs.clone().is_empty() {
        ErrorExt::delCheckpoint((literal!("expandableConnectorsOrder")).clone());
        outEquations = inEquations;
        return Ok((cache.clone(), outEquations.clone()));
    }
    ErrorExt::rollBack((literal!("expandableConnectorsOrder")).clone());
    DoubleEnded::push_list_front(delst.clone(), expandableEqs.clone())?;
    DoubleEnded::push_list_back(delst.clone(), expandableEqs.clone())?;
    let () = (::match_deref::match_deref! { match &(expandableEqs.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            DoubleEnded::push_list_back(delst.clone(), expandableEqs)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEquations = DoubleEnded::toListAndClear(delst, metamodelica::nil());
    Ok((cache, outEquations))
}

pub(crate) fn sortInnerFirstTplLstElementMod(mut inTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    outTplLstElementMod = 'mc: {
        let __mc_input = inTplLstElementMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (System::getHasInnerOuterDefinitions()) else { bail!("pattern mismatch") };
                    Ok(inTplLstElementMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut innerElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut innerouterElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut otherElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut sorted: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut innerModelicaServices: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut innerModelica: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut innerOthers: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    (innerElts, innerouterElts, otherElts) = splitInnerAndOtherTplLstElementMod(inTplLstElementMod.clone());
                    (innerModelicaServices, innerModelica, innerOthers) = splitInners(innerElts.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
                    sorted = listAppend(innerModelicaServices.clone(), listAppend(innerModelica.clone(), listAppend(innerOthers.clone(), listAppend(innerouterElts.clone(), otherElts.clone()))));
                    Ok(sorted.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTplLstElementMod)
}

fn splitInners(mut inTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inAcc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inAcc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inAcc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) {
    let mut outModelicaServices: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    let mut outModelica: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    let mut outOthers: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    (outModelicaServices, outModelica, outOthers) = 'mc: {
        let __mc_input = inTplLstElementMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inAcc1.clone().reverse(), inAcc2.clone().reverse(), inAcc3.clone().reverse()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: em, tail: rest } => {
                    let mut acc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut acc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut acc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut e: Arc<SCode::Element>;
                    let mut p: Arc<Absyn::Path>;
                    e = Util::tuple21(em.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getComponentTypeSpec(e.clone())?) {
                        Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    let true = (stringEq((literal!("ModelicaServices")).clone(), (AbsynUtil::pathFirstIdent(p.clone())?).clone())) else { bail!("pattern mismatch") };
                    (acc1, acc2, acc3) = splitInners(rest.clone(), metamodelica::cons(em.clone(), inAcc1.clone()), inAcc2.clone(), inAcc3.clone());
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: em, tail: rest } => {
                    let mut acc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut acc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut acc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut e: Arc<SCode::Element>;
                    let mut p: Arc<Absyn::Path>;
                    e = Util::tuple21(em.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getComponentTypeSpec(e.clone())?) {
                        Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    let true = (stringEq((literal!("Modelica")).clone(), (AbsynUtil::pathFirstIdent(p.clone())?).clone())) else { bail!("pattern mismatch") };
                    (acc1, acc2, acc3) = splitInners(rest.clone(), inAcc1.clone(), metamodelica::cons(em.clone(), inAcc2.clone()), inAcc3.clone());
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: em @ _, tail: rest } => {
                    let mut acc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut acc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    let mut acc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
                    (acc1, acc2, acc3) = splitInners(rest.clone(), inAcc1.clone(), inAcc2.clone(), metamodelica::cons(em.clone(), inAcc3.clone()));
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outModelicaServices, outModelica, outOthers)
}

pub(crate) fn splitInnerAndOtherTplLstElementMod(mut inTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) {
    let mut outInnerTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outInnerOuterTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outOtherTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut comp: Arc<SCode::Element>;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    for mut e in &*inTplLstElementMod.reverse() {
        let mut e = e.clone();
        (comp, _) = e.clone();
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } if (AbsynUtil::isInner(io.clone())) => {
            if AbsynUtil::isOuter(io.clone()) {
                outInnerOuterTplLstElementMod = metamodelica::cons(e.clone(), outInnerOuterTplLstElementMod.clone());
            } else {
                outInnerTplLstElementMod = metamodelica::cons(e.clone(), outInnerTplLstElementMod.clone());
            }
            ()
        },
        _ => {
            outOtherTplLstElementMod = metamodelica::cons(e.clone(), outOtherTplLstElementMod.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (outInnerTplLstElementMod, outInnerOuterTplLstElementMod, outOtherTplLstElementMod)
}

pub(crate) fn splitEltsOrderInnerOuter(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut cdefImpElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    (cdefImpElts, classextendsElts, extElts, compElts) = (::match_deref::match_deref! { match &(elts.clone()) {
        _ => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut comps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(elts)?;
            comps = listAppend(innerComps.clone(), otherComps.clone());
            (cdefImpElts, classextendsElts, extElts, comps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cdefImpElts, classextendsElts, extElts, compElts))
}

pub(crate) fn splitElts(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut cdefImpElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    for mut elt in &*elts {
        let mut elt = elt.clone();
        let () = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            if (::match_deref::match_deref! { match &(var_field!((*elt).classDef, SCode::Element::CLASS).clone()) {
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
                classextendsElts = metamodelica::cons(elt.clone(), classextendsElts.clone());
            } else {
                cdefImpElts = metamodelica::cons(elt.clone(), cdefImpElts.clone());
            }
            ()
        },
        Deref @ SCode::Element::IMPORT { .. } => {
            cdefImpElts = metamodelica::cons(elt.clone(), cdefImpElts.clone());
            ()
        },
        Deref @ SCode::Element::DEFINEUNIT { .. } => {
            cdefImpElts = metamodelica::cons(elt.clone(), cdefImpElts.clone());
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            extElts = metamodelica::cons(elt.clone(), extElts.clone());
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            compElts = metamodelica::cons(elt.clone(), compElts.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    cdefImpElts = metamodelica::Dangerous::listReverseInPlace(cdefImpElts);
    classextendsElts = metamodelica::Dangerous::listReverseInPlace(classextendsElts);
    extElts = metamodelica::Dangerous::listReverseInPlace(extElts);
    compElts = metamodelica::Dangerous::listReverseInPlace(compElts);
    Ok((cdefImpElts, classextendsElts, extElts, compElts))
}

pub(crate) fn splitEltsNoComponents(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut impElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut defElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut filtered: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (impElts, defElts, classextendsElts, filtered) = (::match_deref::match_deref! { match &(elts) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts, defElts, metamodelica::cons(elt.clone(), classextendsElts), filtered)
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts, metamodelica::cons(elt.clone(), defElts), classextendsElts, metamodelica::cons(elt.clone(), filtered))
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (metamodelica::cons(elt.clone(), impElts), defElts, classextendsElts, filtered)
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts, metamodelica::cons(elt.clone(), defElts), classextendsElts, metamodelica::cons(elt.clone(), filtered))
        },
        Deref @ metamodelica::List::Cons { head: elt, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts, defElts, classextendsElts, metamodelica::cons(elt.clone(), filtered))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((impElts, defElts, classextendsElts, filtered))
}

pub(crate) fn splitEltsInnerAndOther(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut cdefImpElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut innerCompElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut otherCompElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    (cdefImpElts, classextendsElts, extElts, innerCompElts, otherCompElts) = (::match_deref::match_deref! { match &(elts) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts, metamodelica::cons(cdef.clone(), classextendsElts), extElts, innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (metamodelica::cons(cdef.clone(), cdefImpElts), classextendsElts, extElts, innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: imp @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (metamodelica::cons(imp.clone(), cdefImpElts), classextendsElts, extElts, innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: imp @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (metamodelica::cons(imp.clone(), cdefImpElts), classextendsElts, extElts, innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: ext @ Deref @ SCode::Element::EXTENDS { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts, classextendsElts, metamodelica::cons(ext.clone(), extElts), innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let true = (AbsynUtil::isInner(io.clone())) else { bail!("pattern mismatch") };
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts, classextendsElts, extElts, metamodelica::cons(comp.clone(), innerComps.clone()), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts, classextendsElts, extElts, innerComps.clone(), metamodelica::cons(comp.clone(), otherComps.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cdefImpElts, classextendsElts, extElts, innerCompElts, otherCompElts))
}

fn orderComponents(mut inComp: Arc<SCode::Element>, mut inCompElts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outCompElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outCompElts = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER_OUTER { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::CONST { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::PARAM { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
            compElts = listAppend(inCompElts, list![inComp]);
            compElts.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCompElts)
}

fn splitClassExtendsElts(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>) {
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    (classextendsElts, outElts) = (::match_deref::match_deref! { match &(elts) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (classextendsElts, res) = splitClassExtendsElts(xs.clone());
            (metamodelica::cons(cdef.clone(), classextendsElts), res.clone())
        },
        Deref @ metamodelica::List::Cons { head: cdef, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>>;
            (classextendsElts, res) = splitClassExtendsElts(xs.clone());
            (classextendsElts, metamodelica::cons(cdef.clone(), res.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (classextendsElts, outElts)
}

fn addClassdefsToEnv3(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inMod: Option<Arc<DAE::Mod>>, mut sele: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<SCode::Element>)> {
    let mut outCache: FCore::Cache;
    let mut oenv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>>;
    let mut osele: Arc<SCode::Element>;
    (outCache, oenv, outIH, osele) = (::match_deref::match_deref! { match &((inCache, inIH, inPrefix, inMod, sele.clone())) {
        (_, _, _, None, _) => {
            bail!("fail")
        },
        (cache, ih, pre, Some(Deref @ DAE::Mod::MOD { subModLst: lsm, .. }), Deref @ SCode::Element::CLASS { name: r#str, .. }) => {
            let mut mo2: Arc<DAE::Mod>;
            let mut sele2: Arc<SCode::Element>;
            let mut env2: FCore::Graph;
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            (mo2, _) = extractCorrectClassMod2(lsm.clone(), (r#str.clone()).clone(), metamodelica::nil());
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Inst::redeclareType(cache.clone(), env, ih.clone(), mo2.clone(), sele, pre.clone(), ClassInf::State::MODEL { path: Arc::new(Absyn::Path::IDENT { name: (r#str.clone()).clone() }) }, true, openmodelica_frontend_types::DAE::Mod::interned_NOMOD())?) {
                (__pa0, __pa1, __pa2, __pa3 @ Deref @ SCode::Element::CLASS { .. }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            env2 = __pa1.clone();
            ih = __pa2.clone();
            sele2 = __pa3.clone();
            (cache.clone(), env2.clone(), ih.clone(), sele2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, oenv, outIH, osele))
}

fn extractCorrectClassMod2(mut smod: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut name: ArcStr, mut premod: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> (Arc<DAE::Mod>, Arc<metamodelica::List<Arc<DAE::SubMod>>>) {
    let mut omod: Arc<DAE::Mod>;
    let mut restmods: Arc<metamodelica::List<Arc<DAE::SubMod>>>;
    (omod, restmods) = (::match_deref::match_deref! { match &(smod) {
        Deref @ metamodelica::List::Nil => {
            (openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), premod)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod }, tail: rest } if (stringEq((id.clone()).clone(), (name.clone()).clone())) => {
            let mut rest2: Arc<metamodelica::List<Arc<DAE::SubMod>>>;
            rest2 = listAppend(premod, rest.clone());
            (r#mod.clone(), rest2.clone())
        },
        Deref @ metamodelica::List::Cons { head: sub, tail: rest } => {
            let mut r#mod: Arc<DAE::Mod>;
            let mut rest2: Arc<metamodelica::List<Arc<DAE::SubMod>>>;
            (r#mod, rest2) = extractCorrectClassMod2(rest.clone(), (name.clone()).clone(), premod);
            (r#mod.clone(), metamodelica::cons(sub.clone(), rest2.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (omod, restmods)
}

pub(crate) fn traverseModAddFinal(mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = 'mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD { .. } => {
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { eachPrefix: each_, element: element1, .. } => {
                    let mut element2: Arc<SCode::Element>;
                    element2 = traverseModAddFinal3(element1.clone())?;
                    Ok(if (referenceEq(&*(element1.clone()),&*(element2.clone()))) {r#mod.clone()} else {Arc::new(SCode::Mod::REDECL { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: each_.clone(), element: element2.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs1, binding: eq, comment: cmt, info } => {
                    let mut subs2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    subs2 = List::mapCheckReferenceEq(subs1.clone(), (std::sync::Arc::new(traverseModAddFinal4) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<Arc<SCode::SubMod>> + 'static>))?;
                    Ok(if (openmodelica_frontend_types::SCode::Final::FINAL == f.clone() && metamodelica::ReferenceEq::reference_eq(&*(subs1.clone()), &*(subs2.clone()))) {r#mod.clone()} else {Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: each_.clone(), subModLst: subs2.clone(), binding: eq.clone(), comment: cmt.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("InstUtil.traverseModAddFinal"), metamodelica::sourceInfo!("FrontEnd/InstUtil.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(r#mod)
}

fn traverseModAddFinal3(mut inElement: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name, prefixes, attributes: attr, typeSpec: tySpec, modifications: oldmod, comment: cmt, condition: cond, info } => {
                    let mut r#mod: Arc<SCode::Mod>;
                    r#mod = traverseModAddFinal(oldmod.clone())?;
                    Ok(if (referenceEq(&*(oldmod.clone()),&*(r#mod.clone()))) {inElement.clone()} else {Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: tySpec.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::IMPORT { .. } => {
                    Ok(inElement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    Ok(inElement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { baseClassPath: p, visibility: vis, modifications: oldmod, ann, info } => {
                    let mut r#mod: Arc<SCode::Mod>;
                    r#mod = traverseModAddFinal(oldmod.clone())?;
                    Ok(if (referenceEq(&*(oldmod.clone()),&*(r#mod.clone()))) {inElement.clone()} else {Arc::new(SCode::Element::EXTENDS { baseClassPath: p.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!(" we failed with traverseModAddFinal3\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

fn traverseModAddFinal4(mut sub: Arc<SCode::SubMod>) -> Result<Arc<SCode::SubMod>> {
    let mut sub: Arc<SCode::SubMod> = sub;
    let mut r#mod: Arc<SCode::Mod>;
    r#mod = traverseModAddFinal(sub.r#mod.clone())?;
    if !(referenceEq(&*(sub.r#mod.clone()),&*(r#mod.clone()))) {
        assign_field!(sub.r#mod = r#mod);
    }
    Ok(sub)
}

pub(crate) fn traverseModAddDims(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inMod: Arc<SCode::Mod>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = 'mc: {
        let __mc_input = (inCache, inEnv, inPrefix, inMod.clone(), inInstDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r#mod, _) => {
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(inMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, pre, r#mod, inst_dims) => {
                    let mut mod2: Arc<SCode::Mod>;
                    let mut exps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
                    let mut aexps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
                    exps = List::map(inst_dims.clone(), (std::sync::Arc::new(fnptr!(Expression::dimensionsToExps, Arc<metamodelica::List<Arc<DAE::Dimension>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
                    aexps = List::mapList(exps.clone(), (std::sync::Arc::new(Expression::unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    mod2 = traverseModAddDims4(cache.clone(), env.clone(), pre.clone(), r#mod.clone(), aexps.clone(), true)?;
                    Ok(mod2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn traverseModAddDims4(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inMod: Arc<SCode::Mod>, mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inIsTop: bool) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &((inCache, inEnv, inPrefix, inMod.clone(), inExps)) {
        (_, _, _, Deref @ SCode::Mod::NOMOD { .. }, _) => {
            inMod
        },
        (_, _, _, Deref @ SCode::Mod::REDECL { .. }, _) => {
            inMod
        },
        (cache, env, pre, Deref @ SCode::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH { .. }, .. }, exps) => {
            let mut submods2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
            let mut binding: Option<Arc<Absyn::Exp>>;
            submods2 = traverseModAddDims5(cache.clone(), env.clone(), pre.clone(), var_field!((*inMod).subModLst, SCode::Mod::MOD).clone(), exps.clone())?;
            binding = insertSubsInBinding(var_field!((*inMod).binding, SCode::Mod::MOD).clone(), exps.clone())?;
            Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*inMod).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: submods2.clone(), binding: binding.clone(), comment: var_field!((*inMod).comment, SCode::Mod::MOD).clone(), info: var_field!((*inMod).info, SCode::Mod::MOD).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn traverseModAddDims5(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    outMods = (::match_deref::match_deref! { match &((inCache, inEnv, inPrefix, inMods)) {
        (_, _, _, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (cache, env, pre, Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod }, tail: smods }) => {
            let mut mod2: Arc<SCode::Mod>;
            let mut smods2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
            mod2 = traverseModAddDims4(cache.clone(), env.clone(), pre.clone(), r#mod.clone(), inExps.clone(), false)?;
            smods2 = traverseModAddDims5(cache.clone(), env.clone(), pre.clone(), smods.clone(), inExps)?;
            metamodelica::cons(Arc::new(SCode::SubMod { ident: (n.clone()).clone(), r#mod: mod2.clone() }), smods2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMods)
}

fn insertSubsInBinding(mut inOpt: Option<Arc<Absyn::Exp>>, mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outOpt: Option<Arc<Absyn::Exp>>;
    outOpt = (::match_deref::match_deref! { match &((inOpt, inExps)) {
        (None, _) => {
            None
        },
        (Some(e), exps) => {
            let mut e2: Arc<Absyn::Exp>;
            let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>;
            let mut vars: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
            vars = generateUnusedNamesLstCall(e.clone(), exps.clone())?;
            subs = List::mapList(vars.clone(), (std::sync::Arc::new(fnptr!(stringSub, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Subscript>> + 'static>))?;
            (e2, _) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefInsertSubscriptLstLst, Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>)> + 'static>), subs.clone())?;
            e2 = wrapIntoForLst(e2.clone(), vars.clone(), exps.clone())?;
            Some(e2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOpt)
}

fn generateUnusedNames(mut inExp: Arc<Absyn::Exp>, mut inList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outNames: Arc<metamodelica::List<ArcStr>>;
    (outNames, _) = generateUnusedNames2(inList, 1)?;
    Ok(outNames)
}

fn generateUnusedNames2(mut inList: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inInt: i32) -> Result<(Arc<metamodelica::List<ArcStr>>, i32)> {
    let mut outNames: Arc<metamodelica::List<ArcStr>>;
    let mut outInt: i32;
    (outNames, outInt) = (::match_deref::match_deref! { match &((inList, inInt)) {
        (Deref @ metamodelica::List::Nil, i) => {
            (metamodelica::nil(), i.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: exps }, i) => {
            let mut i1: i32;
            let mut i2: i32;
            let mut s: ArcStr;
            let mut names: Arc<metamodelica::List<ArcStr>>;
            s = (intString(i.clone())).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("i")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            i1 = i.clone() + 1;
            (names, i2) = generateUnusedNames2(exps.clone(), i1.clone())?;
            (metamodelica::cons((s.clone()).clone(), names.clone()), i2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNames, outInt))
}

fn generateUnusedNamesLst(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inInt: i32) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, i32)> {
    let mut outNames: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
    let mut outInt: i32;
    (outNames, outInt) = (::match_deref::match_deref! { match &((inList, inInt)) {
        (Deref @ metamodelica::List::Nil, i) => {
            (metamodelica::nil(), i.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e0, tail: exps }, i) => {
            let mut i1: i32;
            let mut i2: i32;
            let mut names: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
            let mut ns: Arc<metamodelica::List<ArcStr>>;
            (ns, i1) = generateUnusedNames2(e0.clone(), i.clone())?;
            (names, i2) = generateUnusedNamesLst(exps.clone(), i1.clone())?;
            (metamodelica::cons(ns.clone(), names.clone()), i2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNames, outInt))
}

fn generateUnusedNamesLstCall(mut inExp: Arc<Absyn::Exp>, mut inList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outNames: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
    (outNames, _) = generateUnusedNamesLst(inList, 1)?;
    Ok(outNames)
}

fn stringsSubs(mut inNames: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    outSubs = (::match_deref::match_deref! { match &(inNames) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: n, tail: names } => {
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            subs = stringsSubs(names.clone());
            metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: metamodelica::nil() }) }) }), subs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSubs
}

fn stringSub(mut inName: ArcStr) -> Arc<Absyn::Subscript> {
    let mut outSub: Arc<Absyn::Subscript>;
    outSub = (match inName {
        mut n => {
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: metamodelica::nil() }) }) })
        },
    });
    outSub
}

fn wrapIntoFor(mut inExp: Arc<Absyn::Exp>, mut inNames: Arc<metamodelica::List<ArcStr>>, mut inRanges: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp, inNames, inRanges)) {
        (e, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            e.clone()
        },
        (e, Deref @ metamodelica::List::Cons { head: n, tail: names }, Deref @ metamodelica::List::Cons { head: r, tail: ranges }) => {
            let mut e2: Arc<Absyn::Exp>;
            e2 = wrapIntoFor(e.clone(), names.clone(), ranges.clone())?;
            Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("array")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: e2.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, iterators: list![Arc::new(Absyn::ForIterator { name: (n.clone()).clone(), guardExp: None, range: Some(Arc::new(Absyn::Exp::RANGE { start: Arc::new(Absyn::Exp::INTEGER { value: 1 }), step: None, stop: r.clone() })) })] }), typeVars: metamodelica::nil() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn wrapIntoForLst(mut inExp: Arc<Absyn::Exp>, mut inNames: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut inRanges: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp, inNames, inRanges)) {
        (e, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            e.clone()
        },
        (e, Deref @ metamodelica::List::Cons { head: n, tail: names }, Deref @ metamodelica::List::Cons { head: r, tail: ranges }) => {
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            e2 = wrapIntoForLst(e.clone(), names.clone(), ranges.clone())?;
            e3 = wrapIntoFor(e2.clone(), n.clone(), r.clone())?;
            e3.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn componentHasCondition(mut component: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> bool {
    let mut hasCondition: bool;
    hasCondition = (::match_deref::match_deref! { match &(component) {
        (Deref @ SCode::Element::COMPONENT { condition: Some(_), .. }, _) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasCondition
}

pub(crate) fn instElementCondExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut component: Arc<SCode::Element>, mut prefix: DAE::Prefix, mut info: SourceInfo) -> (Option<bool>, FCore::Cache) {
    let mut outCondValue: Option<bool>;
    let mut outCache: FCore::Cache;
    (outCondValue, outCache) = 'mc: {
        let __mc_input = component;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { condition: Some(cond_exp), .. } => {
                    let mut cond_val: bool;
                    let mut cache: FCore::Cache;
                    (cond_val, cache) = instConditionalDeclaration(inCache.clone(), inEnv.clone(), cond_exp.clone(), prefix.clone(), info.clone())?;
                    Ok((Some(cond_val.clone()), cache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { condition: Some(_), .. } => {
                    Ok((None, inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Some(true), inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outCondValue, outCache)
}

fn instConditionalDeclaration(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCondition: Arc<Absyn::Exp>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(bool, FCore::Cache)> {
    let mut outIsConditional: bool;
    let mut outCache: FCore::Cache;
    let mut e: Arc<DAE::Exp>;
    let mut t: Arc<DAE::Type>;
    let mut c: DAE::Const;
    let mut b: bool = false;
    let mut val: Arc<Values::Value>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabExp(inCache, inEnv.clone(), inCondition.clone(), false, false, inPrefix, inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    e = __pa1.clone();
    t = __pa2.clone();
    c = __pa3.clone();
    if !(Types::isBoolean(t.clone())) {
        Error::addSourceMessageAndFail(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Dump::printExpStr(inCondition.clone())?).clone(), (TypesDump::unparseTypeNoAttr(t)?).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if !(Types::isParameterOrConstant(c)) {
        Error::addSourceMessageAndFail(Error::COMPONENT_CONDITION_VARIABILITY.clone(), list![(Dump::printExpStr(inCondition.clone())?).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, val) = Ceval::ceval(outCache, inEnv, e, false, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
    outIsConditional = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::BOOL { boolean: __esc_b } => {
            b = (*__esc_b).clone();
            b.clone()
        },
        Deref @ Values::Value::EMPTY { .. } => {
            if !(Config::getGraphicsExpMode()?) {
                Error::addSourceMessage(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone(), list![(Dump::printExpStr(inCondition)?).clone()], inInfo)?;
                bail!("fail");
            }
            true
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.instConditionalDeclaration got unexpected value ")); __mm_s.push_str(&*ValuesDump::valString(val)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/InstUtil.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outIsConditional, outCache))
}

pub(crate) fn propagateClassPrefix(mut attr: SCode::Attributes, mut pre: DAE::Prefix) -> SCode::Attributes {
    let mut outAttr: SCode::Attributes;
    outAttr = (match (attr.clone(), pre) {
        (_, DAE::Prefix::PREFIX { compPre: _, classPre: DAE::ClassPrefix { variability: SCode::Variability::VAR { .. } } }) => {
            attr
        },
        (SCode::Attributes { variability: SCode::Variability::CONST { .. }, .. }, _) => {
            attr
        },
        (SCode::Attributes { arrayDims: ref ad, connectorType: mut ct, parallelism: mut prl, variability: _, direction: mut dir, isField: mut isf }, DAE::Prefix::PREFIX { compPre: _, classPre: DAE::ClassPrefix { variability: mut vt } }) => {
            SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: vt.clone(), direction: dir.clone(), isField: isf.clone() }
        },
        _ => {
            attr
        },
    });
    outAttr
}

pub(crate) fn checkUseConstValue(mut useConstValue: bool, mut ie: Arc<DAE::Exp>, mut v: Option<Arc<Values::Value>>) -> Arc<DAE::Exp> {
    let mut outE: Arc<DAE::Exp>;
    outE = 'mc: {
        let __mc_input = (useConstValue, ie, v);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, e, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, Some(val)) => {
                    let mut e: Arc<DAE::Exp>;
                    e = ValuesUtil::valueExp(val.clone(), None)?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outE
}

pub(crate) fn propagateAbSCDirection(mut inVariability: SCode::Variability, mut inAttributes: SCode::Attributes, mut inClassAttributes: Option<SCode::Attributes>, mut inInfo: SourceInfo) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    outAttributes = (match inVariability {
        SCode::Variability::CONST { .. } => {
            inAttributes
        },
        SCode::Variability::PARAM { .. } => {
            inAttributes
        },
        _ => {
            let mut dir: Absyn::Direction;
            let SCode::ATTR { direction: __pa0, .. } = (inAttributes.clone()) else { bail!("pattern mismatch") };
            dir = __pa0.clone();
            dir = propagateAbSCDirection2(dir.clone(), inClassAttributes, inInfo)?;
            SCodeUtil::setAttributesDirection(inAttributes, dir.clone())
        },
    });
    Ok(outAttributes)
}

pub(crate) fn propagateAbSCDirection2(mut v1: Absyn::Direction, mut optDerAttr: Option<SCode::Attributes>, mut inInfo: SourceInfo) -> Result<Absyn::Direction> {
    let mut v3: Absyn::Direction;
    v3 = (match (v1.clone(), optDerAttr) {
        (_, None) => {
            v1.clone()
        },
        (Absyn::Direction::BIDIR { .. }, Some(SCode::Attributes { direction: mut v2, .. })) => {
            v2.clone()
        },
        (_, Some(SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, .. })) => {
            v1.clone()
        },
        (_, Some(SCode::Attributes { direction: mut v2, .. })) if (v1.clone() == v2.clone()) => {
            v1.clone()
        },
        _ => {
            metamodelica::print((literal!(" failure in propagateAbSCDirection2, Absyn.DIRECTION mismatch")).clone());
            Error::addSourceMessage(Error::COMPONENT_INPUT_OUTPUT_MISMATCH.clone(), list![(literal!("")).clone(), (literal!("")).clone()], inInfo)?;
            bail!("fail")
        },
    });
    Ok(v3)
}

pub(crate) fn makeCrefBaseType(mut inBaseType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = Types::simplifyType(makeCrefBaseType2(inBaseType, inDimensions)?)?;
    Ok(outType)
}

fn makeCrefBaseType2(mut inBaseType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inBaseType.clone(), inDimensions.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. }, _) => {
                    let false = (TypesDump::getDimensions(ty.clone()).is_empty()) else { bail!("pattern mismatch") };
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(inBaseType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    dims = List::last(inDimensions.clone())?;
                    ty = Expression::liftArrayLeftList(inBaseType.clone(), dims.clone());
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub(crate) fn getCrefFromCompDim(mut inEle: Arc<SCode::Element>) -> Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> {
    let mut cref: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    cref = 'mc: {
        let __mc_input = inEle;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: ads, .. }, .. } => {
                    Ok(AbsynUtil::getCrefsFromSubs(ads.clone(), true, true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    cref
}

pub(crate) fn getCrefFromCond(mut cond: Option<Arc<Absyn::Exp>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    crefs = (::match_deref::match_deref! { match &(cond) {
        None => {
            metamodelica::nil()
        },
        Some(e) => {
            AbsynUtil::getCrefFromExp(e.clone(), true, true)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

fn checkVariabilityOfUpdatedComponent(mut variability: SCode::Variability, mut cref: Arc<Absyn::ComponentRef>) -> Result<()> {
    let () = (match variability {
        SCode::Variability::VAR { .. } => (),
        SCode::Variability::DISCRETE { .. } => (),
        _ => bail!("fail"),
    });
    Ok(())
}

pub(crate) fn propagateBinding(mut inVarsDae: DAE::DAElist, mut inEquationsDae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outVarsDae: DAE::DAElist;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut vars1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut equations1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut v1: Arc<DAE::Element> = Arc::new(<DAE::Element as ::std::default::Default>::default());
    let mut i: i32 = 0;
    let mut is: Arc<metamodelica::List<i32>>;
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut path: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let DAE::DAE { elementLst: __pa0 } = (inVarsDae.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    let DAE::DAE { elementLst: __pa1 } = (inEquationsDae) else { bail!("pattern mismatch") };
    equations = __pa1.clone();
    if vars.clone().is_empty() || equations.clone().is_empty() {
        outVarsDae = inVarsDae;
        return Ok(outVarsDae.clone());
    }
    vars1 = metamodelica::nil();
    is = metamodelica::nil();
    for mut v in &*vars {
        let mut v = v.clone();
        v1 = 'mc: {
        let __mc_input = v.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v1 @ Deref @ DAE::Element::VAR { .. } => {
                    let mut v1 = (*v1).clone();
                    let mut e: Arc<DAE::Exp> = e.clone();
                    let mut i: i32 = i.clone();
                    let mut is: Arc<metamodelica::List<i32>> = is.clone();
                    (e, i) = findCorrespondingBinding(var_field!((*v1).componentRef, DAE::Element::VAR).clone(), equations.clone(), 1)?;
                    assign_variant_field!(v1 => DAE::Element::VAR; binding = Some(e.clone()));
                    is = metamodelica::cons(i, is.clone());
                    Ok((v1.clone(), e.clone(), i.clone(), is.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { e = __wb0; i = __wb1; is = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        vars1 = metamodelica::cons(v1.clone(), vars1.clone());
    }
    vars1 = vars1.reverse();
    equations1 = List::deletePositions(equations.clone(), is.clone(), false)?;
    equations1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut eq in (equations).into_iter().cloned() {
            let __x = DAEUtil::moveElementToInitialSection(eq.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    i = 1;
    for mut eq in &*equations1.clone() {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ DAE::Element::INITIALEQUATION { exp1: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { .. }, .. }, exp2: Deref @ DAE::Exp::CALL { path, .. }, .. } if (AbsynUtil::pathLastIdent(path.clone())? == literal!("constructor")) => {
            is = metamodelica::cons(i, is.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, .. } => {
            cr = (*__esc_cr).clone();
            vars1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut v in (vars1.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Element::VAR { binding: Some(_), .. } if (ComponentReferenceBasics::crefPrefixOf(cr.clone(), var_field!((*v).componentRef, DAE::Element::VAR).clone())?) => {
            is = metamodelica::cons(i, is.clone());
            v.clone()
        },
        Deref @ DAE::Element::VAR { binding: None, .. } if (ComponentReferenceBasics::crefPrefixOf(cr.clone(), var_field!((*v).componentRef, DAE::Element::VAR).clone())?) => {
            assign_variant_field!(v => DAE::Element::VAR; variableAttributesOption = DAEUtil::setFixedAttr(var_field!((*v).variableAttributesOption, DAE::Element::VAR).clone(), Some(Arc::new(DAE::Exp::BCONST { bool: false })))?);
            Error::addSourceMessage(Error::MOVING_PARAMETER_BINDING_TO_INITIAL_EQ_SECTION.clone(), list![(ComponentReferenceBasics::printComponentRefStr(var_field!((*v).componentRef, DAE::Element::VAR).clone())?).clone()], var_field!((*eq).source, DAE::Element::INITIAL_COMPLEX_EQUATION).info.clone())?;
            v.clone()
        },
        _ => v.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        i = i + 1;
    }
    equations1 = List::deletePositions(equations1, is, false)?;
    outVarsDae = DAE::DAElist { elementLst: listAppend(equations1, vars1) };
    Ok(outVarsDae)
}

fn findCorrespondingBinding(mut inCref: Arc<DAE::ComponentRef>, mut inEquations: Arc<metamodelica::List<Arc<DAE::Element>>>, mut i: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut i: i32 = i;
    outExp = (::match_deref::match_deref! { match &((inCref, inEquations)) {
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cref2, exp: e, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref2, ty: _ }, scalar: e, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cref2, cr2: cref3, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            Expression::crefExp(cref3.clone())?
        },
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cref2, ty: _ }, rhs: e, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: _, tail: equations }) => {
            (outExp, i) = findCorrespondingBinding(cref.clone(), equations.clone(), i + 1)?;
            outExp
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, i))
}

pub(crate) fn isPartial(mut partialPrefix: SCode::Partial, mut mods: Arc<DAE::Mod>) -> SCode::Partial {
    let mut outPartial: SCode::Partial;
    outPartial = (::match_deref::match_deref! { match &((partialPrefix, mods)) {
        (SCode::Partial::PARTIAL { .. }, Deref @ DAE::Mod::NOMOD { .. }) => openmodelica_frontend_types::SCode::Partial::PARTIAL,
        _ => openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPartial
}

pub(crate) fn isFunctionInput(mut classState: ClassInf::State, mut direction: Absyn::Direction) -> bool {
    let mut functionInput: bool;
    functionInput = (match (classState, direction) {
        (ClassInf::State::FUNCTION { .. }, Absyn::Direction::INPUT { .. }) => true,
        _ => false,
    });
    functionInput
}

pub(crate) fn extractComment(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<SCode::Comment>> {
    let mut cmt: Arc<SCode::Comment> = Arc::new(SCode::Comment { annotation_: None, comment: None });
    for mut elt in &*elts {
        let mut elt = elt.clone();
        let () = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::COMMENT { cmt: __esc_cmt } => {
            cmt = (*__esc_cmt).clone();
            return Ok(cmt.clone());
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(cmt)
}

fn mergeClassComments(mut comment1: Arc<SCode::Comment>, mut comment2: Arc<SCode::Comment>) -> Result<Arc<SCode::Comment>> {
    let mut outComment: Arc<SCode::Comment>;
    outComment = 'mc: {
        let __mc_input = (comment1, comment2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods1, info, .. } }), comment: str1 }, Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods2, .. } }), comment: str2 }) => {
                    let mut r#str: Option<ArcStr>;
                    let mut mods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    r#str = if (isSome(str1.clone())) {str1.clone()} else {str2.clone()};
                    mods = listAppend(mods1.clone(), mods2.clone());
                    Ok(Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: mods.clone(), binding: None, comment: None, info: info.clone() }) })), comment: r#str.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Comment { annotation_: ann1, comment: str1 }, Deref @ SCode::Comment { annotation_: ann2, comment: str2 }) => {
                    let mut ann: Option<Arc<SCode::Annotation>>;
                    let mut r#str: Option<ArcStr>;
                    r#str = if (isSome(str1.clone())) {str1.clone()} else {str2.clone()};
                    ann = if (isSome(ann1.clone())) {ann1.clone()} else {ann2.clone()};
                    Ok(Arc::new(SCode::Comment { annotation_: ann.clone(), comment: r#str.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComment)
}

pub(crate) fn makeNonExpSubscript(mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Subscript>> {
    let mut outSubscript: Arc<DAE::Subscript>;
    outSubscript = (::match_deref::match_deref! { match &(inSubscript) {
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: e.clone() })
        },
        subscript @ Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ } => {
            subscript.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscript)
}

fn getFunctionAttributes(mut cl: Arc<SCode::Element>, mut vl: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inheritedComment: Arc<SCode::Comment>) -> Result<DAE::FunctionAttributes> {
    let mut attr: DAE::FunctionAttributes;
    let mut restriction: SCode::Restriction;
    let mut fres: SCode::FunctionRestriction;
    let mut isOpenModelicaPure: bool = false;
    let mut isImpure: bool = false;
    let mut hasOutVars: bool = false;
    let mut unboxArgs: bool = false;
    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    let mut name: ArcStr = arcstr::literal!("");
    let mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut purity: Absyn::FunctionPurity = Absyn::FunctionPurity::IMPURE;
    let mut daePurity: DAE::Purity;
    restriction = SCodeUtil::getClassRestriction(cl.clone())?;
    let SCode::Restriction::R_FUNCTION { functionRestriction: __pa0 } = (restriction) else { bail!("pattern mismatch") };
    fres = __pa0.clone();
    daePurity = InstBasics::getFunctionRestrictionPurity(SCodeUtil::getFunctionRestrictionPurity(fres.clone()), inheritedComment.clone(), false)?;
    attr = 'mc: {
        let __mc_input = fres.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: mut purity } = __mc_input.clone() else { bail!("nomatch") };
            let mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>> = inVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isImpure: bool = isImpure.clone();
            let mut name: ArcStr = name.clone();
            let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = outVars.clone();
            let mut unboxArgs: bool = unboxArgs.clone();
            isImpure = AbsynUtil::isImpure(purity.clone(), false);
            inVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isInputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?;
            outVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?;
            name = (SCodeUtil::isBuiltinFunction(cl.clone(), List::map(inVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, List::map(outVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?)?).clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone());
            unboxArgs = SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_UnboxArguments")).clone())?;
            Ok((DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity, isFunctionPointer: false, isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((name.clone()).clone()), unboxArgs: unboxArgs }, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL }, inVars.clone(), inlineType.clone(), isImpure.clone(), name.clone(), outVars.clone(), unboxArgs.clone()))
        })() { inVars = __wb0; inlineType = __wb1; isImpure = __wb2; name = __wb3; outVars = __wb4; unboxArgs = __wb5; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>> = inVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isOpenModelicaPure: bool = isOpenModelicaPure.clone();
            let mut name: ArcStr = name.clone();
            let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = outVars.clone();
            let mut unboxArgs: bool = unboxArgs.clone();
            inVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isInputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?;
            outVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?;
            name = (SCodeUtil::isBuiltinFunction(cl.clone(), List::map(inVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, List::map(outVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?)?).clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone());
            isOpenModelicaPure = !(SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_Impure")).clone())?);
            unboxArgs = SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_UnboxArguments")).clone())?;
            Ok((DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity, isFunctionPointer: false, isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((name.clone()).clone()), unboxArgs: unboxArgs }, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION }, inVars.clone(), inlineType.clone(), isOpenModelicaPure.clone(), name.clone(), outVars.clone(), unboxArgs.clone()))
        })() { inVars = __wb0; inlineType = __wb1; isOpenModelicaPure = __wb2; name = __wb3; outVars = __wb4; unboxArgs = __wb5; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
            let mut isOpenModelicaPure: bool = isOpenModelicaPure.clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone());
            isBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_BuiltinPtr")).clone())?) {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR} else {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN};
            isOpenModelicaPure = !(SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_Impure")).clone())?);
            Ok((DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity, isFunctionPointer: false, isBuiltin: isBuiltin.clone(), functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION }, inlineType.clone(), isBuiltin.clone(), isOpenModelicaPure.clone()))
        })() { inlineType = __wb0; isBuiltin = __wb1; isOpenModelicaPure = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(DAE::FunctionAttributes { inline: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, generateEvents: false, purity: daePurity, isFunctionPointer: false, isBuiltin: openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_KERNEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut daePurity: DAE::Purity = daePurity.clone();
            let mut hasOutVars: bool = hasOutVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone());
            hasOutVars = List::any(vl.clone(), (std::sync::Arc::new(Types::isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?;
            isBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_BuiltinPtr")).clone())?) {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR} else {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN};
            if daePurity == DAE::Purity::UNDEFINED.clone() && SCodeUtil::isExternalFunctionRestriction(fres.clone()) && !(hasOutVars || Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())?) {
                daePurity = DAE::Purity::IMPURE.clone();
            }
            Ok((DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity, isFunctionPointer: false, isBuiltin: isBuiltin.clone(), functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL }, daePurity.clone(), hasOutVars.clone(), inlineType.clone(), isBuiltin.clone()))
        })() { daePurity = __wb0; hasOutVars = __wb1; inlineType = __wb2; isBuiltin = __wb3; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(attr)
}

pub(crate) fn checkFunctionElement(mut elt: Arc<DAE::Element>, mut isExternal: bool, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((elt.clone(), isExternal)) {
        (Deref @ DAE::Element::VAR { .. }, _) => {
            ()
        },
        (Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: Deref @ DAE::Exp::METARECORDCALL { .. }, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, _) => {
            ()
        },
        (Deref @ DAE::Element::ALGORITHM { .. }, false) => {
            ()
        },
        (Deref @ DAE::Element::COMMENT { .. }, _) => {
            ()
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = (DAEDump::dumpElementsStr(list![elt])?).clone();
            Error::addSourceMessage(Error::FUNCTION_ELEMENT_WRONG_KIND.clone(), list![(r#str.clone()).clone()], info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printElementAndModList(mut inLstElAndMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(inLstElAndMod) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: (e, m), tail: rest } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s: ArcStr;
            s1 = (SCodeDump::unparseElementStr(e.clone(), SCodeDump::defaultOptions.clone())?).clone();
            s2 = (Mod::printModStr(m.clone())?).clone();
            s3 = (printElementAndModList(rest.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Element:\n")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\nModifier: ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*s3.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

fn splitClassDefsAndComponents(mut inLstElAndMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<(Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outClassDefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    let mut outComponentDefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    (outClassDefs, outComponentDefs) = (::match_deref::match_deref! { match &(inLstElAndMod) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: (e @ Deref @ SCode::Element::COMPONENT { .. }, m), tail: rest } => {
            let mut clsdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
            let mut compdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
            (clsdefs, compdefs) = splitClassDefsAndComponents(rest.clone())?;
            (clsdefs.clone(), metamodelica::cons((e.clone(), m.clone()), compdefs.clone()))
        },
        Deref @ metamodelica::List::Cons { head: (e, m), tail: rest } => {
            let mut clsdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
            let mut compdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
            (clsdefs, compdefs) = splitClassDefsAndComponents(rest.clone())?;
            (metamodelica::cons((e.clone(), m.clone()), clsdefs.clone()), compdefs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDefs, outComponentDefs))
}

pub(crate) fn selectModifiers(mut fromMerging: Arc<DAE::Mod>, mut fromRedeclareType: Arc<DAE::Mod>, mut typePath: Arc<Absyn::Path>) -> (Arc<DAE::Mod>, Arc<DAE::Mod>) {
    let mut bindingMod: Arc<DAE::Mod>;
    let mut classMod: Arc<DAE::Mod>;
    (bindingMod, classMod) = 'mc: {
        let __mc_input = typePath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (redeclareBasicType(fromMerging.clone())) else { bail!("pattern mismatch") };
                    Ok((fromRedeclareType.clone(), fromRedeclareType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((fromMerging.clone(), fromRedeclareType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (bindingMod, classMod)
}

pub(crate) fn redeclareBasicType(mut r#mod: Arc<DAE::Mod>) -> bool {
    let mut isRedeclareOfBasicType: bool;
    isRedeclareOfBasicType = 'mc: {
        let __mc_input = r#mod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut name: ArcStr;
                    let true = (Config::synchronousFeaturesAllowed()?) else { bail!("pattern mismatch") };
                    name = (AbsynUtil::pathFirstIdent(path.clone())?).clone();
                    let true = (listMember((name.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone(), (literal!("Clock")).clone()])) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut name: ArcStr;
                    let false = (Config::synchronousFeaturesAllowed()?) else { bail!("pattern mismatch") };
                    name = (AbsynUtil::pathFirstIdent(path.clone())?).clone();
                    let true = (listMember((name.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone()])) else { bail!("pattern mismatch") };
                    Ok(true)
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
    isRedeclareOfBasicType
}

pub(crate) fn optimizeFunctionCheckForLocals(mut path: Arc<Absyn::Path>, mut inElts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut oalg: Option<Arc<DAE::Element>>, mut acc: Arc<metamodelica::List<Arc<DAE::Element>>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElts, oalg.clone())) {
        (Deref @ metamodelica::List::Nil, None) => {
            return Ok(acc.reverse())
        },
        (Deref @ metamodelica::List::Nil, Some(Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source })) => {
            let mut stmts = (*stmts).clone();
            stmts = optimizeLastStatementTail(path, stmts.clone(), invars.reverse(), outvars.reverse(), metamodelica::nil())?;
            return Ok(metamodelica::cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() }), acc).reverse())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil }, .. }, tail: elts }, _) => {
            { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, acc, invars, outvars); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: elt1 @ Deref @ DAE::Element::ALGORITHM { source, .. }, tail: elts }, Some(elt2)) => {
            let mut r#str: ArcStr;
            r#str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            if !(Config::acceptMetaModelicaGrammar()?) {
                Error::addSourceMessage(Error::FUNCTION_MULTIPLE_ALGORITHM.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            }
            { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), Some(elt1.clone()), metamodelica::cons(elt2.clone(), acc), invars, outvars); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: elt @ Deref @ DAE::Element::ALGORITHM { .. }, tail: elts }, None) => {
            { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), Some(elt.clone()), acc, invars, outvars); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: elt @ Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, direction: DAE::VarDirection::OUTPUT { .. }, .. }, tail: elts }, _) => {
            { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, metamodelica::cons(elt.clone(), acc), invars, metamodelica::cons((name.clone()).clone(), outvars)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: elt @ Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, direction: DAE::VarDirection::INPUT { .. }, .. }, tail: elts }, _) => {
            { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, metamodelica::cons(elt.clone(), acc), metamodelica::cons((name.clone()).clone(), invars), outvars); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: elt, tail: elts }, _) => {
            { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, metamodelica::cons(elt.clone(), acc), invars, outvars); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn optimizeLastStatementTail(mut path: Arc<Absyn::Path>, mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>, mut acc: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inStmts) {
        Deref @ metamodelica::List::Cons { head: stmt, tail: Deref @ metamodelica::List::Nil } => {
            let mut stmt = (*stmt).clone();
            stmt = optimizeStatementTail(path, stmt.clone(), invars, outvars);
            return Ok(metamodelica::cons(stmt.clone(), acc).reverse())
        },
        Deref @ metamodelica::List::Cons { head: stmt, tail: stmts } => {
            { (path, inStmts, invars, outvars, acc) = (path, stmts.clone(), invars, outvars, metamodelica::cons(stmt.clone(), acc)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn optimizeStatementTail(mut path: Arc<Absyn::Path>, mut inStmt: Arc<DAE::Statement>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>) -> Arc<DAE::Statement> {
    let mut ostmt: Arc<DAE::Statement>;
    ostmt = 'mc: {
        let __mc_input = (inStmt.clone(), outvars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, exp1: lhs, exp: rhs, source }, _) => {
                    let mut name: ArcStr;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    name = (Expression::simpleCrefName(lhs.clone())?).clone();
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), list![(name.clone()).clone()], invars.clone(), outvars.clone(), source.clone())?;
                    stmt = if (Expression::isTailCall(rhs.clone())) {Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })} else {Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() })};
                    Ok(stmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, expExpLst: lhsLst, exp: rhs, source }, _) => {
                    let mut lhsNames: Arc<metamodelica::List<ArcStr>>;
                    let mut stmt: Arc<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    lhsNames = List::map(lhsLst.clone(), (std::sync::Arc::new(Expression::simpleCrefName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?;
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), lhsNames.clone(), invars.clone(), outvars.clone(), source.clone())?;
                    stmt = if (Expression::isTailCall(rhs.clone())) {Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })} else {Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: lhsLst.clone(), exp: rhs.clone(), source: source.clone() })};
                    Ok(stmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: cond, statementLst: stmts, else_, source }, _) => {
                    let mut stmts = (*stmts).clone();
                    let mut else_ = (*else_).clone();
                    stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), metamodelica::nil())?;
                    else_ = optimizeElseTail(path.clone(), else_.clone(), invars.clone(), outvars.clone());
                    Ok(Arc::new(DAE::Statement::STMT_IF { exp: cond.clone(), statementLst: stmts.clone(), else_: else_.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { exp: rhs, source }, Deref @ metamodelica::List::Nil) => {
                    let mut stmt: Arc<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), metamodelica::nil(), invars.clone(), metamodelica::nil(), source.clone())?;
                    stmt = Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() });
                    Ok(stmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ostmt
}

fn optimizeElseTail(mut path: Arc<Absyn::Path>, mut inElse: Arc<DAE::Else>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>) -> Arc<DAE::Else> {
    let mut outElse: Arc<DAE::Else>;
    outElse = 'mc: {
        let __mc_input = inElse.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSEIF { exp: cond, statementLst: stmts, else_ } => {
                    let mut stmts = (*stmts).clone();
                    let mut else_ = (*else_).clone();
                    stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), metamodelica::nil())?;
                    else_ = optimizeElseTail(path.clone(), else_.clone(), invars.clone(), outvars.clone());
                    Ok(Arc::new(DAE::Else::ELSEIF { exp: cond.clone(), statementLst: stmts.clone(), else_: else_.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSE { statementLst: stmts } => {
                    let mut stmts = (*stmts).clone();
                    stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), metamodelica::nil())?;
                    Ok(Arc::new(DAE::Else::ELSE { statementLst: stmts.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElse.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outElse
}

fn optimizeStatementTail2(mut path: Arc<Absyn::Path>, mut rhs: Arc<DAE::Exp>, mut lhsVars: Arc<metamodelica::List<ArcStr>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Exp>> {
    let mut orhs: Arc<DAE::Exp>;
    let true = (lhsVars.clone() == outvars) else { bail!("pattern mismatch") };
    let __pa0 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path, rhs, invars, lhsVars, source)) {
        (__pa0, true) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orhs = __pa0.clone();
    Ok(orhs)
}

fn optimizeStatementTail3(mut path: Arc<Absyn::Path>, mut rhs: Arc<DAE::Exp>, mut vars: Arc<metamodelica::List<ArcStr>>, mut lhsVars: Arc<metamodelica::List<ArcStr>>, mut source: Arc<DAE::ElementSource>) -> (Arc<DAE::Exp>, bool) {
    let mut orhs: Arc<DAE::Exp>;
    let mut isTailRecursive: bool;
    (orhs, isTailRecursive) = 'mc: {
        let __mc_input = (path.clone(), rhs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path1, call @ Deref @ DAE::Exp::CALL { path: path2, attr: attr @ Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::NO_TAIL { .. }, .. }, .. }) => {
                    let mut r#str: ArcStr;
                    let mut call = (*call).clone();
                    let mut attr = (*attr).clone();
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tail recursion of: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" with input vars: ")); __mm_s.push_str(&*stringDelimitList(vars.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
                    if Flags::isSet(Flags::TAIL.clone())? {
                        Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    }
                    assign_field!(attr.tailCall = DAE::TailCall::TAIL { vars: vars.clone(), outVars: lhsVars.clone() });
                    assign_variant_field!(call => DAE::Exp::CALL; attr = attr.clone());
                    Ok((call.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut e2 = (*e2).clone();
                    let mut e3 = (*e3).clone();
                    (e2, b1) = optimizeStatementTail3(path.clone(), e2.clone(), vars.clone(), lhsVars.clone(), source.clone());
                    (e3, b2) = optimizeStatementTail3(path.clone(), e3.clone(), vars.clone(), lhsVars.clone(), source.clone());
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e2.clone(), expElse: e3.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchType @ DAE::MatchType::MATCH { switch: _ }, inputs, aliases, localDecls, cases, et }) => {
                    let mut cases = (*cases).clone();
                    cases = optimizeStatementTailMatchCases(path.clone(), cases.clone(), false, metamodelica::nil(), vars.clone(), lhsVars.clone(), source.clone())?;
                    Ok((Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: matchType.clone(), inputs: inputs.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((rhs.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (orhs, isTailRecursive)
}

fn optimizeStatementTailMatchCases(mut path: Arc<Absyn::Path>, mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut changed: bool, mut inAcc: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut vars: Arc<metamodelica::List<ArcStr>>, mut lhsVars: Arc<metamodelica::List<ArcStr>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut ocases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>;
    ocases = 'mc: {
        let __mc_input = (inCases, changed, inAcc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, true, acc) => {
                    Ok(acc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls, body, result: Some(exp), resultInfo, jump, info }, tail: cases }, _, acc) => {
                    let mut case_: Arc<DAE::MatchCase>;
                    let mut exp = (*exp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), exp.clone(), vars.clone(), lhsVars.clone(), source.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    case_ = Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: Some(exp.clone()), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() });
                    Ok(optimizeStatementTailMatchCases(path.clone(), cases.clone(), true, metamodelica::cons(case_.clone(), acc.clone()), vars.clone(), lhsVars.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls, body, result: Some(Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Nil }), resultInfo, jump, info }, tail: cases }, _, acc) => {
                    let mut case_: Arc<DAE::MatchCase>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut sourceStmt: Arc<DAE::ElementSource>;
                    let mut body = (*body).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::last(body.clone())?) {
                        Deref @ DAE::Statement::STMT_NORETCALL { exp: __pa0, source: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    sourceStmt = __pa1.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), exp.clone(), vars.clone(), lhsVars.clone(), source.clone())) {
                        (__pa2, true) => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa2.clone();
                    body = List::set(body.clone(), (body.clone().len() as i32), Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: sourceStmt.clone() }))?;
                    case_ = Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: Some(Arc::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() });
                    Ok(optimizeStatementTailMatchCases(path.clone(), cases.clone(), true, metamodelica::cons(case_.clone(), acc.clone()), vars.clone(), lhsVars.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: case_, tail: cases }, _, acc) => {
                    Ok(optimizeStatementTailMatchCases(path.clone(), cases.clone(), changed, metamodelica::cons(case_.clone(), acc.clone()), vars.clone(), lhsVars.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ocases)
}

pub(crate) fn pushStructuralParameters(mut cache: FCore::Cache) -> FCore::Cache {
    let mut ocache: FCore::Cache;
    ocache = (::match_deref::match_deref! { match &(cache.clone()) {
        FCore::Cache::CACHE { initialGraph: ie, functions: f, evaluatedParams: (ht, crs), modelName: p } => {
            FCore::Cache::CACHE { initialGraph: ie.clone(), functions: f.clone(), evaluatedParams: (ht.clone(), metamodelica::cons(metamodelica::nil(), crs.clone())), modelName: p.clone() }
        },
        _ => {
            cache
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocache
}

pub(crate) fn popStructuralParameters(mut cache: FCore::Cache, mut pre: DAE::Prefix) -> Result<FCore::Cache> {
    let mut ocache: FCore::Cache;
    ocache = (::match_deref::match_deref! { match &(cache.clone()) {
        FCore::Cache::CACHE { initialGraph: ie, functions: f, evaluatedParams: (ht, Deref @ metamodelica::List::Cons { head: crs, tail: crss }), modelName: p } => {
            let mut ht = (*ht).clone();
            ht = prefixAndAddCrefsToHt(cache, ht.clone(), pre, crs.clone())?;
            FCore::Cache::CACHE { initialGraph: ie.clone(), functions: f.clone(), evaluatedParams: (ht.clone(), crss.clone()), modelName: p.clone() }
        },
        FCore::Cache::CACHE { initialGraph: _, functions: _, evaluatedParams: (_, Deref @ metamodelica::List::Nil), modelName: _ } => {
            cache
        },
        FCore::Cache::NO_CACHE { .. } => {
            cache
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ocache)
}

fn prefixAndAddCrefsToHt(mut cache: FCore::Cache, mut set: Arc<AvlSetCR::Tree>, mut pre: DAE::Prefix, mut icrs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<AvlSetCR::Tree>> {
    let mut set: Arc<AvlSetCR::Tree> = set;
    for mut cr in &*icrs {
        let mut cr = cr.clone();
        (_, cr) = PrefixUtil::prefixCref(cache.clone(), FGraph::empty(), InnerOuter::emptyInstHierarchy().clone(), pre.clone(), cr.clone())?;
        set = AvlSetCR::add(set.clone(), cr.clone())?;
    }
    Ok(set)
}

fn numStructuralParameterScopes(mut cache: FCore::Cache) -> Result<i32> {
    let mut i: i32;
    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
    let FCore::CACHE { evaluatedParams: (_, __pa0), .. } = (cache) else { bail!("pattern mismatch") };
    lst = __pa0.clone();
    i = (lst.len() as i32);
    Ok(i)
}

pub(crate) fn checkFunctionDefUse(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut info: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = info.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            checkFunctionDefUse2(elts.clone(), None, metamodelica::nil(), metamodelica::nil(), info.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("InstUtil.checkFunctionDefUse failed")).clone()], info.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn checkFunctionDefUse2(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut alg: Option<Arc<metamodelica::List<Arc<DAE::Statement>>>>, mut inUnbound: Arc<metamodelica::List<ArcStr>>, mut inOutputs: Arc<metamodelica::List<ArcStr>>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<ArcStr>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((elts, alg.clone(), inUnbound.clone(), inOutputs.clone())) {
        (Deref @ metamodelica::List::Nil, None, _, _) => {
            return Ok(inUnbound)
        },
        (Deref @ metamodelica::List::Nil, Some(stmts), unbound, outputs) => {
            let mut unbound = (*unbound).clone();
            (_, _, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), false, (false, false, unbound.clone()))?;
            return Ok(List::fold1(outputs.clone(), (std::sync::Arc::new(checkOutputDefUse) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, SourceInfo, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), inInfo, unbound.clone())?)
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }, tail: rest }, _, unbound, _) => {
            let mut unbound = (*unbound).clone();
            { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), inOutputs, inInfo); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { direction: dir, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst: vars, .. }, dims, binding: None, .. }, tail: rest }, _, unbound, _) => {
            let mut outputs: Arc<metamodelica::List<ArcStr>>;
            let mut names: Arc<metamodelica::List<ArcStr>>;
            let mut outNames: Arc<metamodelica::List<ArcStr>>;
            let mut vars = (*vars).clone();
            let mut unbound = (*unbound).clone();
            vars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(Types::varIsVariable, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>))?;
            names = List::map1r(List::map(vars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone())?;
            outNames = if (DAEUtil::varDirectionEqual(dir.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT)) {names.clone()} else {metamodelica::nil()};
            names = if (Expression::dimensionsKnownAndNonZero(dims.clone())?) {names.clone()} else {metamodelica::nil()};
            unbound = listAppend(names.clone(), unbound.clone());
            outputs = listAppend(outNames.clone(), inOutputs);
            { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), outputs.clone(), inInfo); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { direction: dir, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, dims, binding: None, .. }, tail: rest }, _, unbound, _) => {
            let mut outputs: Arc<metamodelica::List<ArcStr>>;
            let mut unbound = (*unbound).clone();
            unbound = List::consOnTrue(Expression::dimensionsKnownAndNonZero(dims.clone())?, (name.clone()).clone(), unbound.clone());
            outputs = List::consOnTrue(DAEUtil::varDirectionEqual(dir.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT), (name.clone()).clone(), inOutputs);
            { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), outputs.clone(), inInfo); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest }, None, unbound, _) => {
            let mut unbound = (*unbound).clone();
            { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), Some(stmts.clone()), unbound.clone(), inOutputs, inInfo); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, unbound, _) => {
            let mut unbound = (*unbound).clone();
            { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), inOutputs, inInfo); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn checkOutputDefUse(mut name: ArcStr, mut info: SourceInfo, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnbound: Arc<metamodelica::List<ArcStr>>;
    let mut b: bool;
    b = listMember((name.clone()).clone(), inUnbound.clone());
    Error::assertionOrAddSourceMessage(!(b), Error::WARNING_DEF_USE.clone(), list![(name.clone()).clone()], info)?;
    outUnbound = List::filter1OnTrue(inUnbound, (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (name).clone())?;
    Ok(outUnbound)
}

fn checkFunctionDefUseStmt(mut inStmt: Arc<DAE::Statement>, mut inLoop: bool, mut inUnbound: (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: (bool, bool, Arc<metamodelica::List<ArcStr>>);
    outUnbound = (::match_deref::match_deref! { match &((inStmt.clone(), inUnbound.clone())) {
        (_, (true, _, _)) => {
            inUnbound
        },
        (_, (false, true, _)) => {
            let mut info: SourceInfo;
            info = ElementSource::getElementSourceFileInfo(ElementSource::getStatementSource(inStmt)?);
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("InstUtil.checkFunctionDefUseStmt failed")).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ DAE::Statement::STMT_ASSIGN { exp1: lhs, exp: rhs, source, .. }, (_, _, unbound)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound = traverseCrefSubs(lhs.clone(), info.clone(), unbound.clone())?;
            unbound = crefFiltering(lhs.clone(), unbound.clone())?;
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: lhss, exp: rhs, source, .. }, (_, _, unbound)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound = List::fold1(lhss.clone(), (std::sync::Arc::new(traverseCrefSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, SourceInfo, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), info.clone(), unbound.clone())?;
            unbound = List::fold(lhss.clone(), (std::sync::Arc::new(crefFiltering) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), unbound.clone())?;
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, exp: rhs, source, .. }, (_, _, unbound)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound = traverseCrefSubs(lhs.clone(), info.clone(), unbound.clone())?;
            unbound = crefFiltering(lhs.clone(), unbound.clone())?;
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_IF { exp, statementLst: stmts, else_, source }, (_, _, unbound)) => {
            let mut b1: bool;
            let mut b2: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            (b1, b2, unbound) = checkFunctionDefUseElse(Arc::new(DAE::Else::ELSEIF { exp: exp.clone(), statementLst: stmts.clone(), else_: else_.clone() }), unbound.clone(), inLoop, info.clone())?;
            (b1.clone(), b2.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_FOR { iter, range: exp, statementLst: stmts, source, .. }, (_, _, unbound)) => {
            let mut b: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (iter.clone()).clone())?;
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()))?;
            (b.clone(), b.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_PARFOR { iter, range: exp, statementLst: stmts, source, .. }, (_, _, unbound)) => {
            let mut b: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (iter.clone()).clone())?;
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()))?;
            (b.clone(), b.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_WHILE { exp, statementLst: stmts, source }, (_, _, unbound)) => {
            let mut b: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()))?;
            (b.clone(), b.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: Deref @ DAE::Exp::BCONST { bool: false }, .. }, _) => {
            (true, true, metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: exp1, msg: exp2, source, .. }, (_, _, unbound)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp1.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            let (_, (__pa1, _)) = Expression::traverseExpTopDown(exp2.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa1.clone();
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_TERMINATE { msg: exp, source }, (_, _, unbound)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (true, true, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, expLst: Deref @ metamodelica::List::Nil, .. }, .. }, _) => {
            (true, true, metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp, source }, (_, _, unbound)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_BREAK { .. }, (_, _, unbound)) => {
            (true, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_RETURN { .. }, (_, _, unbound)) => {
            (true, true, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_CONTINUE { .. }, (_, _, unbound)) => {
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_ARRAY_INIT { .. }, _) => {
            inUnbound
        },
        (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, (_, _, unbound)) => {
            let mut b: bool;
            let mut unbound = (*unbound).clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), inLoop, (false, false, unbound.clone()))?;
            (b.clone(), b.clone(), unbound.clone())
        },
        _ => {
            let mut r#str: ArcStr;
            let mut info: SourceInfo;
            r#str = (DAEDump::ppStatementStr(inStmt.clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.checkFunctionDefUseStmt failed: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
            info = ElementSource::getElementSourceFileInfo(ElementSource::getStatementSource(inStmt)?);
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnbound)
}

fn checkFunctionDefUseElse(mut inElse: Arc<DAE::Else>, mut inUnbound: Arc<metamodelica::List<ArcStr>>, mut inLoop: bool, mut info: SourceInfo) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: (bool, bool, Arc<metamodelica::List<ArcStr>>);
    outUnbound = (::match_deref::match_deref! { match &((inElse, inUnbound.clone(), inLoop)) {
        (Deref @ DAE::Else::NOELSE { .. }, _, _) => {
            (false, false, inUnbound)
        },
        (Deref @ DAE::Else::ELSEIF { exp, statementLst: stmts, else_ }, unbound, iloop) => {
            let mut unboundBranch: Arc<metamodelica::List<ArcStr>>;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut b4: bool;
            let mut unbound = (*unbound).clone();
            let mut iloop = (*iloop).clone();
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (b1, b2, unboundBranch) = checkFunctionDefUseElse(else_.clone(), unbound.clone(), inLoop, info)?;
            (b3, b4, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), inLoop, (false, false, unbound.clone()))?;
            iloop = true;
            unbound = if (iloop.clone()) {List::intersectionOnTrue(unboundBranch.clone(), unbound.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?} else {unbound.clone()};
            unbound = if (!(iloop.clone() || b1.clone())) {List::union(unboundBranch.clone(), unbound.clone())} else {unbound.clone()};
            b1 = b1.clone() && b3.clone();
            b2 = b2.clone() && b4.clone();
            (b1.clone(), b2.clone(), unbound.clone())
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, unbound, _) => {
            let mut b1: bool;
            let mut b2: bool;
            let mut unbound = (*unbound).clone();
            (b1, b2, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), inLoop, (false, false, unbound.clone()))?;
            (b1.clone(), b2.clone(), unbound.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outUnbound)
}

fn crefFiltering(mut inExp: Arc<DAE::Exp>, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, inUnbound.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _) => {
            return Ok(inUnbound)
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, .. }, .. }, .. }, unbound) => {
            let mut unbound = (*unbound).clone();
            return Ok(List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id1.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*id2.clone()); ArcStr::from(__mm_s) }).clone())?)
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id1, .. }, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, unbound) => {
            let mut id1 = (*id1).clone();
            let mut unbound = (*unbound).clone();
            id1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id1.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
            return Ok(List::filter2OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::notStrncmp, ArcStr, ArcStr, i32)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, i32) -> Result<bool> + 'static>), (id1.clone()).clone(), ((id1.clone()).clone().len() as i32))?)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, unbound) => {
            let mut unbound = (*unbound).clone();
            return Ok(List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone())?)
        },
        (Deref @ DAE::Exp::ASUB { exp, .. }, unbound) => {
            { (inExp, inUnbound) = (exp.clone(), unbound.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::PATTERN { pattern }, unbound) => {
            let mut unbound = (*unbound).clone();
            (_, unbound) = Patternm::traversePattern(pattern.clone(), (std::sync::Arc::new(patternFiltering) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)> + 'static>), unbound.clone())?;
            return Ok(unbound.clone())
        },
        _ => {
            return Ok(inUnbound)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn patternFiltering(mut inPat: Arc<DAE::Pattern>, mut inLst: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outPat: Arc<DAE::Pattern> = inPat.clone();
    let mut unbound: Arc<metamodelica::List<ArcStr>> = inLst.clone();
    unbound = (::match_deref::match_deref! { match &(inPat.clone()) {
        Deref @ DAE::Pattern::PAT_AS { .. } => List::filter1OnTrue(unbound, (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*inPat).id, DAE::Pattern::PAT_AS).clone()).clone())?,
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. } => List::filter1OnTrue(unbound, (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*inPat).id, DAE::Pattern::PAT_AS_FUNC_PTR).clone()).clone())?,
        _ => unbound,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPat, unbound))
}

fn traverseCrefSubs(mut exp: Arc<DAE::Exp>, mut info: SourceInfo, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnbound: Arc<metamodelica::List<ArcStr>>;
    outUnbound = (::match_deref::match_deref! { match &((exp, inUnbound.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, unbound) => {
            let mut unbound = (*unbound).clone();
            let (_, (__pa0, _)) = Expression::traverseExpTopDownCrefHelper(cr.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info))?;
            unbound = __pa0.clone();
            unbound.clone()
        },
        _ => {
            inUnbound
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnbound)
}

fn findUnboundVariableUse(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (Arc<metamodelica::List<ArcStr>>, SourceInfo);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (exp @ Deref @ DAE::Exp::SIZE { .. }, arg) => {
            (exp.clone(), false, arg.clone())
        },
        (exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "isPresent" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, arg) => {
            (exp.clone(), false, arg.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, (_, info)) => {
            Error::addSourceMessage(Error::WARNING_DEF_USE.clone(), list![(literal!("_")).clone()], info.clone())?;
            (inExp, true, inTpl)
        },
        (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (unbound, info)) => {
            let mut r#str: ArcStr;
            let mut b: bool;
            let mut unbound = (*unbound).clone();
            b = listMember((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), unbound.clone());
            r#str = (ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone();
            Error::assertionOrAddSourceMessage(!(b.clone()), Error::WARNING_DEF_USE.clone(), list![(r#str.clone()).clone()], info.clone())?;
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (r#str.clone()).clone())?;
            (exp.clone(), true, (unbound.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, (unbound, info)) => {
            let mut b: bool;
            let mut unbound = (*unbound).clone();
            b = listMember((name.clone()).clone(), unbound.clone());
            Error::assertionOrAddSourceMessage(!(b.clone()), Error::WARNING_DEF_USE.clone(), list![(name.clone()).clone()], info.clone())?;
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone())?;
            (exp.clone(), true, (unbound.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::MATCHEXPRESSION { inputs, localDecls, cases, .. }, (unbound, info)) => {
            let mut unboundLocal: Arc<metamodelica::List<ArcStr>>;
            let mut unbounds: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
            let mut unbound = (*unbound).clone();
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(Arc::new(DAE::Exp::LIST { valList: inputs.clone() }), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unboundLocal = checkFunctionDefUse2(localDecls.clone(), None, unbound.clone(), metamodelica::nil(), info.clone())?;
            unbounds = List::map1(cases.clone(), (std::sync::Arc::new(findUnboundVariableUseInCase) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), unboundLocal.clone())?;
            unbound = List::fold1r(unbounds.clone(), (std::sync::Arc::new(List::intersectionOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), unbound.clone())?;
            (exp.clone(), false, (unbound.clone(), info.clone()))
        },
        (exp, arg) => {
            (exp.clone(), true, arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn findUnboundVariableUseInCase(mut case_: Arc<DAE::MatchCase>, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut unbound: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    unbound = (::match_deref::match_deref! { match &((case_, inUnbound)) {
        (Deref @ DAE::MatchCase { patterns, patternGuard, body, result, info, resultInfo, .. }, __esc_unbound) => {
            unbound = (*__esc_unbound).clone();
            (_, unbound) = Patternm::traversePatternList(patterns.clone(), (std::sync::Arc::new(patternFiltering) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)> + 'static>), unbound.clone())?;
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(Arc::new(DAE::Exp::META_OPTION { exp: patternGuard.clone() }), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, _, unbound) = List::fold1(body.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()))?;
            let (_, (__pa1, _)) = Expression::traverseExpTopDown(Arc::new(DAE::Exp::META_OPTION { exp: result.clone() }), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), resultInfo.clone()))?;
            unbound = __pa1.clone();
            unbound.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(unbound)
}

pub(crate) fn checkParallelismWRTEnv(mut inEnv: FCore::Graph, mut inName: ArcStr, mut inAttr: SCode::Attributes, mut inInfo: SourceInfo) -> bool {
    let mut isValid: bool;
    isValid = 'mc: {
        let __mc_input = inAttr;
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::Attributes { parallelism: mut prl, direction: mut dir, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut errorString: ArcStr;
            let mut scopeName: ArcStr;
            let mut isparglobal: bool;
            let mut hasnodir: bool;
            let mut r: metamodelica::Array<FCore::Node>;
            r = FGraph::lastScopeRef(inEnv.clone())?;
            let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            scopeName = (FNode::refName(r.clone())?).clone();
            let true = (FGraph::checkScopeType(list![r.clone()], Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE))) else { bail!("pattern mismatch") };
            isparglobal = SCodeUtil::parallelismEqual(prl.clone(), openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL);
            hasnodir = !(AbsynUtil::isInputOrOutput(dir.clone())?);
            let true = (isparglobal.clone() && hasnodir.clone()) else { bail!("pattern mismatch") };
            errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- local parglobal component '")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("' is declared in parallel/parkernel function '")); __mm_s.push_str(&*scopeName.clone()); __mm_s.push_str(&*literal!("'. \n")); __mm_s.push_str(&*literal!("- parglobal variables can be declared only in normal functions. \n")); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
            Ok(false)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    isValid
}

pub(crate) fn instDimsHasZeroDims(mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> bool {
    let mut outHasZeroDims: bool;
    outHasZeroDims = 'mc: {
        let __mc_input = inInstDims;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: dims, tail: _ } => {
                    let true = (List::any(dims.clone(), (std::sync::Arc::new(Expression::dimensionIsZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } => {
                    Ok(instDimsHasZeroDims(rest_dims.clone()))
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
    outHasZeroDims
}

pub(crate) fn noModForUpdatedComponents(mut variability: SCode::Variability, mut updatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut cref: Arc<Absyn::ComponentRef>, mut mods: Arc<DAE::Mod>, mut cmod: Arc<DAE::Mod>, mut m: Arc<SCode::Mod>) -> (Arc<DAE::Mod>, Arc<DAE::Mod>, Arc<SCode::Mod>) {
    let mut outMods: Arc<DAE::Mod>;
    let mut outCmod: Arc<DAE::Mod>;
    let mut outM: Arc<SCode::Mod>;
    (outMods, outCmod, outM) = 'mc: {
        let __mc_input = m.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((BaseHashTable::hasKey(cref.clone(), updatedComps.clone())?)) { bail!("guard") }
                    checkVariabilityOfUpdatedComponent(variability.clone(), cref.clone())?;
                    Ok((openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::SCode::Mod::interned_NOMOD()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((mods.clone(), cmod.clone(), m.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outMods, outCmod, outM)
}

pub(crate) fn propagateModFinal(mut inMod: Arc<DAE::Mod>, mut inFinal: SCode::Final) -> SCode::Final {
    let mut outFinal: SCode::Final;
    outFinal = (::match_deref::match_deref! { match &((inMod, inFinal.clone())) {
        (_, SCode::Final::FINAL { .. }) => {
            inFinal
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: fp, .. }, _) => {
            fp.clone()
        },
        _ => {
            inFinal
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFinal
}

//------------------------------
//------  PDE extension:  ------
//------------------------------
pub type DomainFieldOpt = Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)>;

pub type DomainFieldsLst = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>>;

pub(crate) fn addGhostCells(mut inCompelts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outCompelts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    let mut fieldNamesP: Arc<metamodelica::List<ArcStr>>;
    let mut ghostCompelts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    fieldNamesP = List::fold(inEqs, (std::sync::Arc::new(fieldsInPderEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), metamodelica::nil())?;
    ghostCompelts = List::fold1(inCompelts.clone(), (std::sync::Arc::new(fnptr!(addGhostCells2, (Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> + 'static>), fieldNamesP, metamodelica::nil())?;
    outCompelts = listAppend(inCompelts, ghostCompelts);
    Ok(outCompelts)
}

pub(crate) fn fieldsInPderEq(mut eq: Arc<SCode::Equation>, mut inFieldNames: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outFieldNames: Arc<metamodelica::List<ArcStr>>;
    outFieldNames = (::match_deref::match_deref! { match &(eq) {
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, .. } => {
            let mut fieldNames1: Arc<metamodelica::List<ArcStr>>;
            (_, fieldNames1) = AbsynUtil::traverseExpTopDown(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(fieldInPderExp, Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)> + 'static>), inFieldNames.clone())?;
            (_, fieldNames1) = AbsynUtil::traverseExpTopDown(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(fieldInPderExp, Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)> + 'static>), fieldNames1.clone())?;
            listAppend(inFieldNames, fieldNames1.clone())
        },
        _ => {
            inFieldNames
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFieldNames)
}

pub(crate) fn fieldInPderExp(mut inExp: Arc<Absyn::Exp>, mut inFieldNames: Arc<metamodelica::List<ArcStr>>) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outFieldNames: Arc<metamodelica::List<ArcStr>>;
    outFieldNames = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: newFieldName, .. } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            List::unionElt((newFieldName.clone()).clone(), inFieldNames)
        },
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: newFieldName, .. } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. } => {
            List::unionElt((newFieldName.clone()).clone(), inFieldNames)
        },
        _ => {
            inFieldNames
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp = inExp;
    (outExp, outFieldNames)
}

pub(crate) fn addGhostCells2(mut inCompelt: (Arc<SCode::Element>, Arc<DAE::Mod>), mut fieldNamesP: Arc<metamodelica::List<ArcStr>>, mut inGhosts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outGhosts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>;
    outGhosts = 'mc: {
        let __mc_input = inCompelt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { name, prefixes, attributes: SCode::Attributes { arrayDims, connectorType, parallelism, variability, direction, isField: Absyn::IsField::FIELD { .. } }, typeSpec, modifications: r#mod @ Deref @ SCode::Mod::MOD { .. }, comment, condition, info }, daeMod) => {
                    if !((listMember((name.clone()).clone(), fieldNamesP.clone()))) { bail!("guard") }
                    let mut ghostL: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    let mut ghostR: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    let mut r#mod = (*r#mod).clone();
                    assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = List::filterOnFalse(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), (std::sync::Arc::new(fnptr!(isSubModDomainOrStart, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>))?);
                    ghostL = (Arc::new(SCode::Element::COMPONENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: arrayDims.clone(), connectorType: connectorType.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: typeSpec.clone(), modifications: r#mod.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), daeMod.clone());
                    ghostR = (Arc::new(SCode::Element::COMPONENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: arrayDims.clone(), connectorType: connectorType.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: typeSpec.clone(), modifications: r#mod.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), daeMod.clone());
                    Ok(metamodelica::cons(ghostL.clone(), metamodelica::cons(ghostR.clone(), inGhosts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGhosts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outGhosts
}

pub(crate) fn isSubModDomainOrStart(mut subMod: Arc<SCode::SubMod>) -> bool {
    let mut isNotDomain: bool;
    isNotDomain = (::match_deref::match_deref! { match &(subMod) {
        Deref @ SCode::SubMod { ident: idn, .. } if (idn.clone() == literal!("domain") || idn.clone() == literal!("start")) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNotDomain
}

pub(crate) fn elabField(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut name: ArcStr, mut attr: SCode::Attributes, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inMod: Arc<DAE::Mod>, mut inInfo: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<DAE::Mod>, DomainFieldOpt)> {
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outFieldDomOpt: DomainFieldOpt;
    (outDims, outMod, outFieldDomOpt) = ({
        let mut N: i32 = -1;
        (::match_deref::match_deref! { match &((attr, inMod.clone())) {
        (SCode::Attributes { isField: Absyn::IsField::NONFIELD { .. }, .. }, _) => {
            (inDims, inMod, None)
        },
        (SCode::Attributes { isField: Absyn::IsField::FIELD { .. }, .. }, Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix, subModLst, binding, info }) => {
            let mut dim_f: Arc<DAE::Dimension>;
            let mut dcr: Arc<DAE::ComponentRef>;
            let mut domainSubMod: Arc<DAE::SubMod>;
            let mut subModLst = (*subModLst).clone();
            (domainSubMod, subModLst) = List::findAndRemove(subModLst.clone(), (std::sync::Arc::new(fnptr!(findDomainSubMod, Arc<DAE::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<bool> + 'static>))?;
            dcr = getQualDcr(domainSubMod.clone(), inInfo.clone())?;
            (N, dcr) = getNDcr(dcr.clone())?;
            if N.clone() == -1 {
                Error::addSourceMessageAndFail(Error::PDEModelica_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Domain of the field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' not found.")); ArcStr::from(__mm_s) }).clone()], inInfo)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            subModLst = subModLst.clone().reverse();
            subModLst = List::map(subModLst.clone(), (std::sync::Arc::new(addEach) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<Arc<DAE::SubMod>> + 'static>))?;
            outMod = Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: eachPrefix.clone(), subModLst: subModLst.clone(), binding: binding.clone(), info: info.clone() });
            dim_f = Arc::new(DAE::Dimension::DIM_INTEGER { integer: N.clone() });
            (metamodelica::cons(dim_f.clone(), inDims), outMod, Some((Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name).clone(), subscripts: metamodelica::nil() }), dcr.clone())))
        },
        (_, Deref @ DAE::Mod::NOMOD { .. }) => {
            Error::addSourceMessageAndFail(Error::PDEModelica_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("' has no domain modifier.")); ArcStr::from(__mm_s) }).clone()], inInfo)?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            (inDims, inMod, None)
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok((outDims, outMod, outFieldDomOpt))
}

fn findDomainSubMod(mut subMod: Arc<DAE::SubMod>) -> bool {
    let mut isDomain: bool;
    isDomain = (::match_deref::match_deref! { match &(subMod) {
        Deref @ DAE::SubMod { ident: Deref @ "domain", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDomain
}

fn getQualDcr(mut domainSubMod: Arc<DAE::SubMod>, mut inInfo: SourceInfo) -> Result<Arc<DAE::ComponentRef>> {
    let mut dcr: Arc<DAE::ComponentRef>;
    dcr = (::match_deref::match_deref! { match &(domainSubMod) {
        Deref @ DAE::SubMod { ident: Deref @ "domain", r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DomainLineSegment1D" } } }, .. } }, .. }), .. } } => {
            cr.clone()
        },
        _ => {
            Error::addSourceMessageAndFail(Error::PDEModelica_ERROR.clone(), list![(literal!("The domain type is wrong.\n")).clone()], inInfo)?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dcr)
}

fn getNDcr(mut dcr: Arc<DAE::ComponentRef>) -> Result<(i32, Arc<DAE::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(dcr.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr1, .. } => {
            { dcr = cr1.clone(); continue '__tco; }
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, .. } => {
            let mut N: i32;
            let __pa0 = ::match_deref::match_deref! { match &(List::findSome(varLst.clone(), (std::sync::Arc::new(fnptr!(findN, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Option<i32>> + 'static>))?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            N = __pa0.clone();
            return Ok((N.clone(), dcr))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn findN(mut inVar: Arc<DAE::Var>) -> Option<i32> {
    let mut optN: Option<i32>;
    optN = (::match_deref::match_deref! { match &(inVar) {
        Deref @ DAE::Var { name: Deref @ "N", binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::INTEGER { integer: N }), .. }, .. } => {
            Some(N.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    optN
}

fn addEach(mut inSubMod: Arc<DAE::SubMod>) -> Result<Arc<DAE::SubMod>> {
    let mut outSubMod: Arc<DAE::SubMod>;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut finalPrefix: SCode::Final = SCode::Final::FINAL;
    let mut subModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut binding: Option<DAE::EqMod> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outSubMod = (::match_deref::match_deref! { match &(inSubMod) {
        Deref @ DAE::SubMod { ident: __esc_ident, r#mod: Deref @ DAE::Mod::MOD { finalPrefix: __esc_finalPrefix, eachPrefix: _, subModLst: __esc_subModLst, binding: __esc_binding, info: __esc_info } } => {
            ident = (*__esc_ident).clone();
            finalPrefix = (*__esc_finalPrefix).clone();
            subModLst = (*__esc_subModLst).clone();
            binding = (*__esc_binding).clone();
            info = (*__esc_info).clone();
            Arc::new(DAE::SubMod { ident: (ident.clone()).clone(), r#mod: Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, subModLst: subModLst.clone(), binding: binding.clone(), info: info.clone() }) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMod)
}

//----end elabField and sub funs
pub(crate) fn optAppendField(mut inDomFieldsLst: DomainFieldsLst, mut fieldDomOpt: DomainFieldOpt) -> Result<DomainFieldsLst> {
    let mut outDomFieldsLst: DomainFieldsLst = metamodelica::nil();
    outDomFieldsLst = (::match_deref::match_deref! { match &(fieldDomOpt) {
        None => {
            inDomFieldsLst
        },
        Some((fieldCr, domainCr)) => {
            let mut found: bool;
            (outDomFieldsLst, found) = List::map2Fold(inDomFieldsLst.clone(), (std::sync::Arc::new(fnptr!(optAppendFieldMapFun, (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), Arc<DAE::ComponentRef>, Arc<Absyn::ComponentRef>, bool)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), Arc<DAE::ComponentRef>, Arc<Absyn::ComponentRef>, bool) -> Result<((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), bool)> + 'static>), domainCr.clone(), fieldCr.clone(), false, metamodelica::nil())?;
            if !(found.clone()) {
                outDomFieldsLst = metamodelica::cons((domainCr.clone(), list![fieldCr.clone()]), inDomFieldsLst);
            }
            outDomFieldsLst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDomFieldsLst)
}

fn optAppendFieldMapFun(mut inDomainFields: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), mut domainCrToAdd: Arc<DAE::ComponentRef>, mut fieldCrToAdd: Arc<Absyn::ComponentRef>, mut inFound: bool) -> ((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), bool) {
    let mut outDomainFields: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>);
    let mut outFound: bool;
    (outDomainFields, outFound) = 'mc: {
        let __mc_input = (inDomainFields.clone(), inFound);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((domainCr, fieldCrLst), false) => {
                    let true = (ComponentReferenceBasics::crefEqual(domainCr.clone(), domainCrToAdd.clone())?) else { bail!("pattern mismatch") };
                    Ok(((domainCr.clone(), metamodelica::cons(fieldCrToAdd.clone(), fieldCrLst.clone())), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDomainFields.clone(), inFound))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outDomainFields, outFound)
}

//----end optAppendField and sub funs
pub(crate) fn discretizePDE(mut inEQ: Arc<SCode::Equation>, mut inDomFieldLst: DomainFieldsLst, mut inDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut newDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    newDiscretizedEQs = (::match_deref::match_deref! { match &(inEQ.clone()) {
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: domainCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, comment, info } => {
            let mut N: i32;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            (N, fieldLst) = getDomNFields(inDomFieldLst, domainCr.clone(), info.clone())?;
            creatFieldEqs(lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N.clone(), fieldLst.clone(), comment.clone(), info.clone())?
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: domainCr @ Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "interior", .. } }, comment, info } => {
            let mut domainCr1: Arc<Absyn::ComponentRef>;
            let mut N: i32;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            domainCr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst, domainCr1.clone(), info.clone())?;
            creatFieldEqs(lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N.clone(), fieldLst.clone(), comment.clone(), info.clone())?
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "left", .. } }, comment, info } => {
            let mut domainCr1: Arc<Absyn::ComponentRef>;
            let mut N: i32;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut lhs_exp = (*lhs_exp).clone();
            let mut rhs_exp = (*rhs_exp).clone();
            domainCr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst, domainCr1.clone(), info.clone())?;
            (lhs_exp, _) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 1)?;
            (rhs_exp, _) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 1)?;
            list![newEQFun(1, lhs_exp.clone(), rhs_exp.clone(), domainCr1.clone(), N.clone(), true, fieldLst.clone(), comment.clone(), info.clone())?]
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "right", .. } }, comment, info } => {
            let mut domainCr1: Arc<Absyn::ComponentRef>;
            let mut N: i32;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut lhs_exp = (*lhs_exp).clone();
            let mut rhs_exp = (*rhs_exp).clone();
            domainCr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst, domainCr1.clone(), info.clone())?;
            (lhs_exp, _) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), N.clone())?;
            (rhs_exp, _) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), N.clone())?;
            list![newEQFun(N.clone(), lhs_exp.clone(), rhs_exp.clone(), domainCr1.clone(), N.clone(), true, fieldLst.clone(), comment.clone(), info.clone())?]
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            metamodelica::print((literal!("Unhandled type of EQ_PDE in discretizePDE\n")).clone());
            bail!("fail");
            metamodelica::nil()
        },
        _ => {
            list![inEQ]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDiscretizedEQs = listAppend(inDiscretizedEQs, newDiscretizedEQs);
    Ok(outDiscretizedEQs)
}

fn extrapFieldTraverseFun(mut inExp: Arc<Absyn::Exp>, mut inN: i32) -> (Arc<Absyn::Exp>, i32) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outN: i32 = inN;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "extrapolateField", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            let mut i: i32;
            if inN == 1 {
                i = 1;
            } else {
                i = -1;
            }
            Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: inN }) }), subscripts.clone()) }) }) }), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: inN + i.clone() }) }), subscripts.clone()) }) }) })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outN)
}

fn getDomNFields(mut inDomFieldLst: DomainFieldsLst, mut inDomainCr: Arc<Absyn::ComponentRef>, mut info: SourceInfo) -> Result<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)> {
    let mut outN: i32 = 0;
    let mut outFieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::findSome(inDomFieldLst.clone(), (std::sync::Arc::new({ let __pe_b1 = inDomainCr.clone(); move |__pe_a0| Ok(domNFieldsFindFun(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)) -> Result<Option<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>> + 'static>))) {
        Ok(Some((__pa0, __pa1))) => (__pa0.clone(), __pa1.clone()),
        _ => {
        Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![(literal!("There are no fields defined within the domain of this equation.")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        },
    } };
    outN = __pa0.clone();
    outFieldLst = __pa1.clone();
    Ok((outN, outFieldLst))
}

fn domNFieldsFindFun(mut inDomFields: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), mut inDomainCr: Arc<Absyn::ComponentRef>) -> Option<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)> {
    let mut outOptNFields: Option<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>;
    outOptNFields = 'mc: {
        let __mc_input = inDomFields;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (domainCr, fieldCrLst) => {
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>>;
                    let mut N: i32;
                    let true = (absynDAECrefEqualName(inDomainCr.clone(), domainCr.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(domainCr.clone()) {
                        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(List::findSome(varLst.clone(), (std::sync::Arc::new(fnptr!(findN, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Option<i32>> + 'static>))?) {
                        Some(__pa2) => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    N = __pa2.clone();
                    Ok(Some((N.clone(), fieldCrLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outOptNFields
}

fn absynDAECrefEqualName(mut domainCr1: Arc<Absyn::ComponentRef>, mut domainCr2: Arc<DAE::ComponentRef>) -> bool {
    let mut equal: bool;
    let mut name1: ArcStr = arcstr::literal!("");
    let mut name2: ArcStr = arcstr::literal!("");
    equal = (::match_deref::match_deref! { match &((domainCr1, domainCr2)) {
        (Deref @ Absyn::ComponentRef::CREF_IDENT { name: name1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: name2, .. }) if (stringEqual((name1.clone()).clone(), (name2.clone()).clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

fn extrapolateFieldEq(mut isRight: bool, mut fieldCr: Arc<Absyn::ComponentRef>, mut domainCr: Arc<Absyn::ComponentRef>, mut N: i32, mut comment: Arc<SCode::Comment>, mut info: SourceInfo, mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> Result<Arc<SCode::Equation>> {
    let mut outEQ: Arc<SCode::Equation>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let mut i3: i32 = 3;
    if List::isMemberOnTrue(fieldCr.clone(), fieldLst, (std::sync::Arc::new(AbsynUtil::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))? {
        (name, subscripts) = (::match_deref::match_deref! { match &(fieldCr) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, subscripts: __esc_subscripts } => {
            name = (*__esc_name).clone();
            subscripts = (*__esc_subscripts).clone();
            (name.clone(), subscripts.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
        if isRight {
            i1 = N;
            i2 = N - 1;
            i3 = N - 2;
        }
        outEQ = Arc::new(SCode::Equation::EQ_EQUALS { expLeft: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i1 }) }), subscripts.clone()) }) }), expRight: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i2 }) }), subscripts.clone()) }) }) }), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i3 }) }), subscripts) }) }) }), comment: comment, info: info });
    } else {
        bail!("fail");
    }
    Ok(outEQ)
}

fn creatFieldEqs(mut lhs_exp: Arc<Absyn::Exp>, mut rhs_exp: Arc<Absyn::Exp>, mut domainCr: Arc<Absyn::ComponentRef>, mut N: i32, mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut bl: bool;
    let mut br: bool;
    (_, bl) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(hasPderTraverseFun, Arc<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), false)?;
    (_, br) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(hasPderTraverseFun, Arc<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), false)?;
    outDiscretizedEQs = (match (bl, br) {
        (false, false) => ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut i in (1..=N).into_iter() {
            let __x = newEQFun(i.clone(), lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N, false, fieldLst.clone(), comment.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut i in (1..=N).into_iter() {
            let __x = newEQFun(i.clone(), lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N, false, fieldLst.clone(), comment.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
    });
    Ok(outDiscretizedEQs)
}

fn hasPderTraverseFun(mut inExp: Arc<Absyn::Exp>, mut inHasPder: bool) -> (Arc<Absyn::Exp>, bool) {
    let mut outExp: Arc<Absyn::Exp> = inExp.clone();
    let mut outHasPder: bool;
    outHasPder = (::match_deref::match_deref! { match &((inExp, inHasPder)) {
        (_, true) => true,
        (Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, .. }, _) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHasPder)
}

fn newEQFun(mut i: i32, mut inLhs_exp: Arc<Absyn::Exp>, mut inRhs_exp: Arc<Absyn::Exp>, mut domainCr: Arc<Absyn::ComponentRef>, mut N: i32, mut isBC: bool, mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<Arc<SCode::Equation>> {
    let mut outEQ: Arc<SCode::Equation>;
    let mut outLhs_exp: Arc<Absyn::Exp>;
    let mut outRhs_exp: Arc<Absyn::Exp>;
    (outLhs_exp, _) = AbsynUtil::traverseExpTopDown(inLhs_exp, (std::sync::Arc::new(discretizeTraverseFun) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool)) -> Result<(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool))> + 'static>), (i, fieldLst.clone(), domainCr.clone(), info.clone(), false, N, isBC))?;
    (outRhs_exp, _) = AbsynUtil::traverseExpTopDown(inRhs_exp, (std::sync::Arc::new(discretizeTraverseFun) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool)) -> Result<(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool))> + 'static>), (i, fieldLst, domainCr, info.clone(), false, N, isBC))?;
    outEQ = Arc::new(SCode::Equation::EQ_EQUALS { expLeft: outLhs_exp, expRight: outRhs_exp, comment: comment, info: info });
    Ok(outEQ)
}

fn discretizeTraverseFun(mut inExp: Arc<Absyn::Exp>, mut inTup: (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool)) -> Result<(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool))> {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outTup: (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool);
    let mut i: i32;
    let mut N: i32;
    let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    let mut info: SourceInfo;
    let mut skip: bool;
    let mut failVar: bool;
    let mut isBC: bool;
    let mut domainCr: Arc<Absyn::ComponentRef>;
    let mut domName: ArcStr;
    failVar = false;
    (i, fieldLst, domainCr, info, skip, N, isBC) = inTup.clone();
    let __pa0 = ::match_deref::match_deref! { match &(domainCr.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    domName = __pa0.clone();
    if skip {
        outExp = inExp;
        outTup = inTup;
        return Ok((outExp.clone(), outTup.clone()));
    }
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { name: domName, subscripts: Deref @ metamodelica::List::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", subscripts: Deref @ metamodelica::List::Nil } } } => {
                    Ok(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (domName.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("x")).clone(), subscripts: list![Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i }) })] }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } } => {
                    let mut exp: Arc<Absyn::Exp>;
                    let true = (List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(AbsynUtil::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    exp = if (isBC && i == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), subscripts: subscripts.clone() }) })} else if (isBC && i == N) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i }) }), subscripts.clone()) }) })};
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::List::Nil } }, argNames: _ }, typeVars: Deref @ metamodelica::List::Nil } => {
                    let mut leftVar: Arc<Absyn::Exp>;
                    let mut rightVar: Arc<Absyn::Exp>;
                    let mut failVar: bool = failVar.clone();
                    if !(List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(AbsynUtil::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))?) {
                        failVar = true;
                        Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' has different domain than the equation or is not a field.")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    leftVar = if (i == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i - 1 }) }), subscripts.clone()) }) })};
                    rightVar = if (i == N) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i + 1 }) }), subscripts.clone()) }) })};
                    Ok((Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: rightVar.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: leftVar.clone() }), op: openmodelica_ast::Absyn::Operator::DIV, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (domName.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("dx")).clone(), subscripts: metamodelica::nil() }) }) }) }) }), failVar.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { failVar = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::List::Nil } } }, argNames: _ }, typeVars: Deref @ metamodelica::List::Nil } => {
                    let mut leftVar: Arc<Absyn::Exp>;
                    let mut actualVar: Arc<Absyn::Exp>;
                    let mut rightVar: Arc<Absyn::Exp>;
                    let mut failVar: bool = failVar.clone();
                    if !(List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(AbsynUtil::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))?) {
                        failVar = true;
                        Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' has different domain than the equation or is not a field.")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    leftVar = if (i == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i - 1 }) }), subscripts.clone()) }) })};
                    actualVar = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i }) }), subscripts.clone()) }) });
                    rightVar = if (i == N) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i + 1 }) }), subscripts.clone()) }) })};
                    Ok((Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: leftVar.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: actualVar.clone() }) }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: rightVar.clone() }), op: openmodelica_ast::Absyn::Operator::DIV, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (domName.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("dx")).clone(), subscripts: metamodelica::nil() }) }) }), op: openmodelica_ast::Absyn::Operator::POW, exp2: Arc::new(Absyn::Exp::INTEGER { value: 2 }) }) }), failVar.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { failVar = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: _ }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, argNames: _ }, .. } => {
                    Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![(literal!("You are differentiating with respect to variable that is not a coordinate.")).clone()], info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, argNames: _ }, .. } => {
                    Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![(literal!("Unsupported partial derivative.")).clone()], info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if failVar {
        bail!("fail");
    }
    outTup = (i, fieldLst, domainCr, info, skip, N, isBC);
    Ok((outExp, outTup))
}

fn findDomF<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inTup: (ArcStr, T), mut name: ArcStr) -> bool {
    let mut found: bool;
    found = (match inTup {
        (mut nameLoc, _) if (stringEqual((nameLoc.clone()).clone(), (name.clone()).clone())) => {
            true
        },
        _ => {
            false
        },
    });
    found
}

/*
public function findDomains
  input SCode.Element el;
  input list<tuple<String,Integer>> domainLstIn;
  output list<tuple<String,Integer>> domainLstOut;
algorithm
//TODO: rewrite to use instantiated domain elements
  domainLstOut := match el
    local
      String name;
      list<SCode.SubMod> subModLst;
      Integer N;
    case SCode.COMPONENT(typeSpec=Absyn.TPATH(path=Absyn.IDENT(name="DomainLineSegment1D")), name = name,
      modifications = SCode.MOD(subModLst = subModLst))
      then
        (name,findDomains1(subModLst))::domainLstIn;
      else
      domainLstIn;
  end match;
end findDomains;


protected function findDomains1
  input list<SCode.SubMod> subModLst;
  output Integer N;
algorithm
  try
    N := match List.find(subModLst,findDomains2)
    local
      Integer n;
    case SCode.NAMEMOD("N",SCode.MOD(binding = SOME(Absyn.INTEGER(n))))
    then
      n;
    end match;
  else
    print("\nError: Variable N not found in the domain.\n");
    fail();
  end try;
end findDomains1;

protected function findDomains2
  input SCode.SubMod subMod;
  output Boolean found;
algorithm
  found := match subMod
    case SCode.NAMEMOD(ident = "N")
      then
        true;
      else
        false;
  end match;
end findDomains2;
*/
