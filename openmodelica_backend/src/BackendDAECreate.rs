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

use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::Vectorization;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ConnectUtil;
use openmodelica_frontend::HashTableCrToExpSourceTpl;
use openmodelica_frontend::HashTableExpToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::VarTransform;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StackOverflow;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub fn lower(mut lst: DAE::DAElist, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExtraInfo: BackendDAE::ExtraInfo) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut globalKnownVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut extvarlst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut vars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut localKnownVars: BackendDAE::Variables;
    let mut vars_1: BackendDAE::Variables;
    let mut extVars: BackendDAE::Variables;
    let mut aliasVars: BackendDAE::Variables;
    let mut extAliasVars: BackendDAE::Variables;
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut constrs: Arc<metamodelica::List<Arc<DAE::Constraint>>>;
    let mut clsAttrs: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>;
    let mut eqnarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut reqnarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut ieqnarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut extObjCls: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>;
    let mut symjacs: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>;
    let mut einfo: BackendDAE::EventInfo;
    let mut elems: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut aliaseqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut functionTree: Arc<AvlTreePathFunction::Tree>;
    let mut timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>;
    let mut numCheckpoints: i32;
    let mut syst: Arc<BackendDAE::EqSystem>;
    numCheckpoints = ErrorExt::getNumCheckpoints();
    StackOverflow::clearStacktraceMessages();
    System::tmpTickResetIndex(0, Global::backendDAE_fileSequence.clone());
    System::tmpTickResetIndex(1, Global::backendDAE_cseIndex.clone());
    System::tmpTickResetIndex(0, Global::strongComponent_index.clone());
    functionTree = FCore::getFunctionTree(inCache.clone());
    functionTree = lowerFunctions(functionTree)?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(processBuiltinExpressions(lst, functionTree)?) {
        (DAE::DAElist { elementLst: __pa0 }, __pa1, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    elems = __pa0.clone();
    functionTree = __pa1.clone();
    timeEvents = __pa2.clone();
    (varlst, globalKnownVarLst, extvarlst, eqns, reqns, ieqns, constrs, clsAttrs, extObjCls, aliaseqns, _) = lower2(elems.reverse(), functionTree.clone(), HashTableExpToExp::emptyHashTable(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
    globalKnownVars = BackendVariable::listVar(globalKnownVarLst.clone())?;
    localKnownVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    extVars = BackendVariable::listVar(extvarlst.clone())?;
    aliasVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    if Flags::isSet(Flags::VECTORIZE.clone())? {
        (varlst, eqns) = Vectorization::collectForLoops(varlst, eqns)?;
    }
    vars = BackendVariable::listVar(varlst.clone())?;
    (vars, globalKnownVars, extVars, aliasVars, eqns, reqns, ieqns) = handleAliasEquations(aliaseqns, vars, globalKnownVars, extVars, aliasVars, eqns, reqns, ieqns)?;
    (ieqns, eqns, reqns, extAliasVars, globalKnownVars, extVars) = getExternalObjectAlias(ieqns, eqns, reqns, globalKnownVars, extVars)?;
    aliasVars = BackendVariable::addVariables(extAliasVars, aliasVars)?;
    (globalKnownVarLst, eqns, reqns, ieqns) = patchRecordBindings(varlst, extvarlst, globalKnownVarLst, eqns, reqns, ieqns)?;
    vars_1 = detectImplicitDiscrete(vars, globalKnownVars.clone(), eqns.clone())?;
    eqnarr = BackendEquation::listEquation(eqns)?;
    reqnarr = BackendEquation::listEquation(reqns)?;
    ieqnarr = BackendEquation::listEquation(ieqns)?;
    einfo = BackendDAE::EventInfo { timeEvents: timeEvents, zeroCrossings: ZeroCrossings::new()?, relations: DoubleEnded::fromList(metamodelica::nil())?, samples: ZeroCrossings::new()?, numberMathEvents: 0 };
    symjacs = list![(None, (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1)), (None, (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1)), (None, (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1)), (None, (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), -1))];
    syst = BackendDAEUtil::createEqSystem(vars_1, eqnarr, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, reqnarr);
    outBackendDAE = Arc::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(syst.clone(), metamodelica::nil()), shared: Arc::new(BackendDAE::Shared { globalKnownVars: globalKnownVars, localKnownVars: localKnownVars, externalObjects: extVars, aliasVars: aliasVars, initialEqs: ieqnarr, removedEqs: BackendEquation::emptyEqns(), constraints: constrs, classAttrs: clsAttrs, cache: inCache, graph: inEnv, functionTree: functionTree.clone(), eventInfo: einfo, extObjClasses: extObjCls, backendDAEType: openmodelica_backend_types::BackendDAE::BackendDAEType::SIMULATION, symjacs: symjacs, info: inExtraInfo, partitionsInfo: BackendDAEUtil::emptyPartitionsInfo(), daeModeData: BackendDAE::emptyDAEModeData().clone(), dataReconciliationData: None, timeInterval: None }) });
    BackendDAEUtil::checkBackendDAEWithErrorMsg(outBackendDAE.clone())?;
    BackendDAEUtil::checkAdjacencyMatrixSolvability(syst.clone(), functionTree, BackendDAEUtil::isInitializationDAE(outBackendDAE.shared.clone()))?;
    if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())? {
        Error::addSourceMessage(Error::BACKENDDAEINFO_LOWER.clone(), list![ArcStr::from(::std::format!("{}", BackendEquation::equationArraySize(syst.orderedEqs.clone())?)), ArcStr::from(::std::format!("{}", BackendVariable::varsSize(syst.orderedVars.clone())))], Absyn::dummyInfo.clone())?;
    }
    execStat((literal!("Generate backend data structure")).clone())?;
    return Ok(outBackendDAE.clone());
    bail!("fail");
    Ok(outBackendDAE)
}

pub type Functiontuple = (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>);

pub type ArrayBindingList = Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>;

fn printArrayBindingList(mut arrayBindingList: ArrayBindingList) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut subscriptLst: Arc<metamodelica::List<i32>>;
    let mut bindingExp: Arc<DAE::Exp>;
    for mut tpl in &*arrayBindingList {
        let mut tpl = tpl.clone();
        (subscriptLst, bindingExp) = tpl.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[")); ArcStr::from(__mm_s) }).clone();
        for mut subscript in &*subscriptLst.clone() {
            let mut subscript = subscript.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*intString(subscript.clone())); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(bindingExp.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn patchRecordBindings(mut varlst: Arc<metamodelica::List<BackendDAE::Var>>, mut extvarlst: Arc<metamodelica::List<BackendDAE::Var>>, mut globalKnownVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut globalKnownVarLst: Arc<metamodelica::List<BackendDAE::Var>> = globalKnownVarLst;
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqns;
    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = reqns;
    let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = ieqns;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>;
    let mut arrayMap: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>>>;
    let mut debug: bool = false;
    map = UnorderedMap::new((std::sync::Arc::new(ComponentReferenceBasics::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 1);
    collectRecordTypesVarLst(map.clone(), globalKnownVarLst.clone())?;
    eqns = List::map(eqns, (std::sync::Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
    reqns = List::map(reqns, (std::sync::Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
    ieqns = List::map(ieqns, (std::sync::Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
    arrayMap = UnorderedMap::new((std::sync::Arc::new(ComponentReferenceBasics::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 1);
    List::apply(varlst, (std::sync::Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = arrayMap.clone(); move |__pe_a0| collectRecordElementBindings(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<()> + 'static>))?;
    List::apply(globalKnownVarLst.clone(), (std::sync::Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = arrayMap.clone(); move |__pe_a0| collectRecordElementBindings(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<()> + 'static>))?;
    List::apply(extvarlst, (std::sync::Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = arrayMap.clone(); move |__pe_a0| collectRecordElementBindings(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<()> + 'static>))?;
    map = collapseArrayBindings(arrayMap.clone(), map)?;
    if debug {
        metamodelica::print((literal!("patchRecordBindings arrayMap:\n")).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*UnorderedMap::toString(arrayMap, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(printArrayBindingList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print((literal!("\npatchRecordBindings map\n")).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(fnptr!(TypesDump::printTypeStr, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    globalKnownVarLst = updateRecordTypesVarLst(map.clone(), globalKnownVarLst)?;
    eqns = List::map(eqns, (std::sync::Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
    reqns = List::map(reqns, (std::sync::Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
    ieqns = List::map(ieqns, (std::sync::Arc::new({ let __pe_b1 = map; move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> + 'static>))?;
    Ok((globalKnownVarLst, eqns, reqns, ieqns))
}

fn collectRecordElementBindings(mut var: BackendDAE::Var, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut arrayMap: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>>>) -> Result<()> {
    let mut rec_cref: Arc<DAE::ComponentRef>;
    let mut is_rec: bool;
    (rec_cref, is_rec) = ComponentReference::crefGetFirstRec(var.varName.clone())?;
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(binding) if (is_rec && UnorderedMap::contains(rec_cref.clone(), map.clone())? && Expression::isConst(binding.clone())?) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut arrayCref: Arc<DAE::ComponentRef>;
            let mut arrayBindingExpList: ArrayBindingList;
            let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut intSubLst: Arc<metamodelica::List<i32>>;
            if ComponentReference::isArrayElement(var.varName.clone()) {
                arrayCref = ComponentReference::crefStripSubsExceptModelSubs(var.varName.clone());
                arrayBindingExpList = UnorderedMap::getOrDefault(arrayCref.clone(), arrayMap.clone(), metamodelica::nil())?;
                subscriptLst = ComponentReferenceBasics::crefSubs(var.varName.clone())?;
                intSubLst = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut subscript in (subscriptLst).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            i.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collectRecordElementBindings")); __mm_s.push_str(&*literal!(" failed because index not integer.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                UnorderedMap::add(arrayCref, metamodelica::cons((intSubLst, binding.clone()), arrayBindingExpList), arrayMap)?;
            } else {
                ty = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(rec_cref.clone(), map.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?) {
        __esc_ty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            ty = (*__esc_ty).clone();
            assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = updateConstantRecordElementBinding(v.clone(), binding.clone(), (ComponentReferenceBasics::crefLastIdent(var.varName.clone())?).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ty.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collectRecordElementBindings")); __mm_s.push_str(&*literal!(" failed because the type is not T_COMPLEX.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                UnorderedMap::add(rec_cref.clone(), ty, map.clone())?;
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

fn updateConstantRecordElementBinding(mut var: Arc<DAE::Var>, mut binding: Arc<DAE::Exp>, mut name: ArcStr) -> Result<Arc<DAE::Var>> {
    let mut var: Arc<DAE::Var> = var;
    let mut r#const: DAE::Const;
    if DAEUtil::isConstVar(var.clone()) && var.name.clone() == name {
        r#const = if (Expression::isConst(binding.clone())?) {openmodelica_frontend_types::DAE::Const::C_CONST} else {openmodelica_frontend_types::DAE::Const::C_VAR};
        assign_field!(var.binding = Arc::new(DAE::Binding::EQBOUND { exp: binding, evaluatedExp: None, constant_: r#const, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    }
    Ok(var)
}

fn collectRecordTypesVarLst(mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut varLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<()> {
    for mut var in &*varLst {
        let mut var = var.clone();
        collectRecordTypesVar(map.clone(), var.clone())?;
    }
    Ok(())
}

fn collectRecordTypesVar(mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut var: BackendDAE::Var) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(exp) => {
            Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(collectRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>), map)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn collectRecordTypesEqn(mut eqn: Arc<BackendDAE::Equation>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    (eqn, _) = BackendEquation::traverseExpsOfEquation(eqn, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(collectRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>); move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), map)?;
    Ok(eqn)
}

fn collectRecordTypesExp(mut exp: Arc<DAE::Exp>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>> = map;
    cont = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } if (Types::isRecord(var_field!((*exp).ty, DAE::Exp::CREF).clone()) && Types::recordHasConstVar(var_field!((*exp).ty, DAE::Exp::CREF).clone())?) => {
            UnorderedMap::add(cref.clone(), var_field!((*exp).ty, DAE::Exp::CREF).clone(), map.clone())?;
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cont, map))
}

fn updateRecordTypesEqn(mut eqn: Arc<BackendDAE::Equation>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqn: Arc<BackendDAE::Equation> = eqn;
    (eqn, _) = BackendEquation::traverseExpsOfEquation(eqn, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(updateRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>); move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), map)?;
    Ok(eqn)
}

fn updateRecordTypesExp(mut exp: Arc<DAE::Exp>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>> = map;
    (exp, cont) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } if (UnorderedMap::contains(cref.clone(), map.clone())?) => {
            assign_variant_field!(exp => DAE::Exp::CREF; ty = UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?);
            (exp, false)
        },
        _ => {
            (exp, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, cont, map))
}

fn compareArrayBindingExp(mut inElement1: (Arc<metamodelica::List<i32>>, Arc<DAE::Exp>), mut inElement2: (Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)) -> Result<bool> {
    let mut inRes: bool = false;
    let mut indiceLstElem1: Arc<metamodelica::List<i32>>;
    let mut indiceLstElem2: Arc<metamodelica::List<i32>>;
    let mut rest_e2: Arc<metamodelica::List<i32>>;
    let mut e2: i32;
    (indiceLstElem1, _) = inElement1;
    (indiceLstElem2, _) = inElement2;
    if (indiceLstElem1.clone().len() as i32) != (indiceLstElem2.clone().len() as i32) {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.compareArrayBindingExp")); __mm_s.push_str(&*literal!(" failed because lists have different lengths.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?;
        bail!("fail");
    }
    rest_e2 = indiceLstElem2;
    for mut e1 in &*indiceLstElem1 {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        if e1.clone() < e2 {
            inRes = true;
            return Ok(inRes.clone());
        } else if e1.clone() > e2 {
            inRes = false;
            return Ok(inRes.clone());
        }
    }
    inRes = true;
    return Ok(inRes.clone());
    Ok(inRes)
}

fn collapseArrayBindings(mut arrayMap: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)>>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>> {
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>> = map;
    let mut cref: Arc<DAE::ComponentRef>;
    let mut rec_cref: Arc<DAE::ComponentRef>;
    let mut arrayBindingExpList: ArrayBindingList;
    let mut subscriptLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut binding: Arc<DAE::Exp>;
    let mut scalarBinding: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut firstDim: i32 = 0;
    for mut pair in &*UnorderedMap::toList(arrayMap) {
        let mut pair = pair.clone();
        (cref, arrayBindingExpList) = pair.clone();
        arrayBindingExpList = List::sort(arrayBindingExpList.clone(), (std::sync::Arc::new(compareArrayBindingExp) as std::sync::Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, Arc<DAE::Exp>), (Arc<metamodelica::List<i32>>, Arc<DAE::Exp>)) -> Result<bool> + 'static>))?;
        expLst = metamodelica::nil();
        for mut scalBind in &*arrayBindingExpList.clone() {
            let mut scalBind = scalBind.clone();
            (subscriptLst, scalarBinding) = scalBind.clone();
            expLst = metamodelica::cons(scalarBinding.clone(), expLst.clone());
        }
        binding = (match (subscriptLst.clone().len() as i32) {
        1 => {
            Arc::new(DAE::Exp::ARRAY { ty: ComponentReference::crefTypeFull(cref.clone())?, scalar: true, array: expLst.clone() })
        },
        2 => {
            let mut matLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            dims = TypesDump::getDimensions(ComponentReference::crefLastType(cref.clone())?);
            firstDim = (::match_deref::match_deref! { match &(listHead(dims.clone())?) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: __esc_firstDim } => {
            firstDim = (*__esc_firstDim).clone();
            firstDim.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
            if let Ok(__iflet0) = List::splitEqualParts(expLst.clone(), firstDim) {
                matLst = __iflet0;
            } else {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!(" failed to reshape matrix.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?;
                bail!("fail");
            }
            Arc::new(DAE::Exp::MATRIX { ty: ComponentReference::crefTypeFull(cref.clone())?, integer: firstDim, matrix: matLst.clone() })
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!("failed. Array of dimension greater 2 not yet supported. Open a ticket about it.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?;
            bail!("fail")
        },
    });
        let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefGetFirstRec(cref.clone())?) {
            (__pa0, true) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rec_cref = __pa0.clone();
        ty = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(rec_cref.clone(), map.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?) {
        __esc_ty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            ty = (*__esc_ty).clone();
            assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = updateConstantRecordElementBinding(v.clone(), binding.clone(), (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ty.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!(" failed because the type is not T_COMPLEX.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(rec_cref.clone(), ty.clone(), map.clone())?;
    }
    Ok(map)
}

fn updateRecordTypesVarLst(mut map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>, mut varLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = varLst;
    varLst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (varLst).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            (exp, _) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(updateRecordTypesExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Type>>>)> + 'static>), map.clone())?;
            var.bindExp = Some(exp.clone());
            var.clone()
        },
        _ => {
            var.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(varLst)
}

fn getExternalObjectAlias(mut inInitEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut globalVarsIn: BackendDAE::Variables, mut extVars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables)> {
    let mut oInitEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut oEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut oRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut extAliasVars: BackendDAE::Variables;
    let mut globalVarsOut: BackendDAE::Variables;
    let mut extVarsOut: BackendDAE::Variables;
    let mut extCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut aliasEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut aliasVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    extCrefs = BackendVariable::getAllCrefFromVariables(extVars.clone())?;
    (oEqs, aliasEqs) = List::fold1(inEqs, (std::sync::Arc::new(fnptr!(getExternalObjectAlias2, Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), extCrefs.clone(), (metamodelica::nil(), metamodelica::nil()))?;
    (oInitEqs, aliasEqs) = List::fold1(inInitEqs, (std::sync::Arc::new(fnptr!(getExternalObjectAlias2, Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), extCrefs.clone(), (metamodelica::nil(), aliasEqs))?;
    (oRemEqs, aliasEqs) = List::fold1(inRemEqs, (std::sync::Arc::new(fnptr!(getExternalObjectAlias2, Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), extCrefs, (metamodelica::nil(), aliasEqs))?;
    if !(aliasEqs.clone().is_empty()) {
        Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Alias equations of external objects are not Modelica compliant as in:\n    ")); __mm_s.push_str(&*stringDelimitList(List::map(aliasEqs.clone(), (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?, (literal!("\n    ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    repl = BackendVarTransform::emptyReplacements();
    (aliasVarLst, repl) = List::fold1(aliasEqs, (std::sync::Arc::new(getExternalObjectAlias3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> + 'static>), extVars.clone(), (metamodelica::nil(), repl))?;
    extAliasVars = BackendVariable::listVar1(aliasVarLst)?;
    extVarsOut = BackendVariable::deleteVars(extAliasVars.clone(), extVars);
    extVarsOut = removeExtAliasBinding(extVarsOut, repl.clone())?;
    (oEqs, _) = BackendVarTransform::replaceEquations(oEqs, repl.clone(), None)?;
    (oInitEqs, _) = BackendVarTransform::replaceEquations(oInitEqs, repl.clone(), None)?;
    (oRemEqs, _) = BackendVarTransform::replaceEquations(oRemEqs, repl.clone(), None)?;
    (globalVarsOut, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalVarsIn, (std::sync::Arc::new(BackendVarTransform::replaceVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl)?;
    oEqs = oEqs.reverse();
    oInitEqs = oInitEqs.reverse();
    oRemEqs = oRemEqs.reverse();
    Ok((oInitEqs, oEqs, oRemEqs, extAliasVars, globalVarsOut, extVarsOut))
}

fn removeExtAliasBinding(mut extVarsIn: BackendDAE::Variables, mut repl: BackendVarTransform::VariableReplacements) -> Result<BackendDAE::Variables> {
    let mut extVarsOut: BackendDAE::Variables;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut extVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    varLst = BackendVariable::varList(extVarsIn)?;
    extVarLst = metamodelica::nil();
    for mut var in &*varLst {
        let mut var = var.clone();
        var = (::match_deref::match_deref! { match &(var.clone()) {
        BackendDAE::Var { bindExp: Some(Deref @ DAE::Exp::CREF { componentRef: __esc_cref, .. }), .. } => {
            cref = (*__esc_cref).clone();
            if BackendVarTransform::hasReplacement(repl.clone(), cref.clone())? {
                var.bindExp = None;
            }
            var.clone()
        },
        _ => var.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        extVarLst = metamodelica::cons(var.clone(), extVarLst.clone());
    }
    extVarsOut = BackendVariable::listVar(extVarLst)?;
    Ok(extVarsOut)
}

fn getExternalObjectAlias3(mut eqIn: Arc<BackendDAE::Equation>, mut extVars: BackendDAE::Variables, mut tplIn: (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut tplOut: (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements);
    let mut eq: Arc<BackendDAE::Equation>;
    let mut crefs_lhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut crefs_rhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut extAliasVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (extAliasVars, repl) = tplIn.clone();
    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eqIn.clone()], repl.clone(), None)?) {
        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eq = __pa0.clone();
    match '__try2: {
        (crefs_lhs, crefs_rhs) = unwrap_break_err!(BackendEquation::equationCrefsSolved(eq.clone()), '__try2);
        (extAliasVars, repl) = (::match_deref::match_deref! { match &((crefs_lhs.clone(), crefs_rhs.clone())) {
        (Deref @ metamodelica::List::Cons { head: lhs, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: rhs, tail: Deref @ metamodelica::List::Nil }) => {
            crefs_lhs = unwrap_break_err!(ComponentReference::expandCref(lhs.clone(), true), '__try2);
            crefs_rhs = unwrap_break_err!(ComponentReference::expandCref(rhs.clone(), true), '__try2);
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        (Deref @ metamodelica::List::Cons { head: lhs, tail: Deref @ metamodelica::List::Nil }, _) => {
            crefs_lhs = unwrap_break_err!(ComponentReference::expandCref(lhs.clone(), true), '__try2);
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        (_, Deref @ metamodelica::List::Cons { head: rhs, tail: Deref @ metamodelica::List::Nil }) => {
            crefs_rhs = unwrap_break_err!(ComponentReference::expandCref(rhs.clone(), true), '__try2);
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        _ => {
            (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars.clone(), extAliasVars.clone(), repl.clone()), '__try2);
            (extAliasVars.clone(), repl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        tplOut = (extAliasVars.clone(), repl.clone());
        Ok::<_, anyhow::Error>((tplOut.clone(),))
    } {
        Ok((__try2_o0,)) => {
            tplOut = __try2_o0;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.getExternalObjectAlias3")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*BackendDump::equationString(eqIn.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?;
            tplOut = tplIn.clone();
        }
    }
    Ok(tplOut)
}

fn addExternalObjectReplacementRules(mut crefs_lhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut crefs_rhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut extVars: BackendDAE::Variables, mut extAliasVars: Arc<metamodelica::List<BackendDAE::Var>>, mut repl: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut extAliasVars: Arc<metamodelica::List<BackendDAE::Var>> = extAliasVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut lhs: Arc<DAE::ComponentRef>;
    let mut rhs: Arc<DAE::ComponentRef>;
    let mut v1: BackendDAE::Var;
    let mut v2: BackendDAE::Var;
    let mut simVar: BackendDAE::Var;
    let mut aliasVar: BackendDAE::Var;
    if (crefs_lhs.clone().len() as i32) == (crefs_rhs.clone().len() as i32) {
        for mut tpl in &*List::zip(crefs_lhs, crefs_rhs) {
            let mut tpl = tpl.clone();
            (lhs, rhs) = tpl.clone();
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(lhs.clone(), extVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(rhs.clone(), extVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, _) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            (simVar, aliasVar) = chooseExternalAlias(v1.clone(), v2.clone())?;
            extAliasVars = metamodelica::cons(aliasVar.clone(), extAliasVars.clone());
            repl = BackendVarTransform::addReplacement(repl.clone(), BackendVariable::varCref(aliasVar.clone())?, Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?, None)?;
        }
    } else {
        bail!("fail");
    }
    Ok((extAliasVars, repl))
}

fn chooseExternalAlias(mut var1: BackendDAE::Var, mut var2: BackendDAE::Var) -> Result<(BackendDAE::Var, BackendDAE::Var)> {
    let mut simVar: BackendDAE::Var;
    let mut aliasVar: BackendDAE::Var;
    if BackendVariable::varHasBindExp(var1.clone()) && !(BackendVariable::varHasBindExp(var2.clone())) {
        simVar = var1;
        aliasVar = BackendVariable::setBindExp(var2, Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
    } else if BackendVariable::varHasBindExp(var2.clone()) && !(BackendVariable::varHasBindExp(var1.clone())) {
        simVar = var2;
        aliasVar = BackendVariable::setBindExp(var1, Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
    } else if BackendVariable::varHasBindExp(var2.clone()) && BackendVariable::varHasBindExp(var1.clone()) {
        if Expression::isCall(BackendVariable::varBindExp(var1.clone())?) {
            simVar = var1;
            aliasVar = BackendVariable::setBindExp(var2, Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
        } else {
            simVar = var2;
            aliasVar = BackendVariable::setBindExp(var1, Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
        }
    } else {
        simVar = var1;
        aliasVar = BackendVariable::setBindExp(var2, Some(Expression::crefExp(BackendVariable::varCref(simVar.clone())?)?));
    }
    Ok((simVar, aliasVar))
}

fn getExternalObjectAlias2(mut eqIn: Arc<BackendDAE::Equation>, mut extCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut eqTplIn: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) {
    let mut eqTplOut: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    eqTplOut = 'mc: {
        let __mc_input = (eqIn.clone(), eqTplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, right: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, .. }, (noAliasEqs, aliasEqs)) => {
                    let true = (List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr1.clone())? && List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr2.clone())?) else { bail!("pattern mismatch") };
                    Ok((noAliasEqs.clone(), metamodelica::cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } }, scalar: Deref @ DAE::Exp::CREF { componentRef: cr2, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } }, .. }, (noAliasEqs, aliasEqs)) => {
                    let true = (List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr1.clone())? && List::exist1(extCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr2.clone())?) else { bail!("pattern mismatch") };
                    Ok((noAliasEqs.clone(), metamodelica::cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, right: Deref @ DAE::Exp::ARRAY { .. }, .. }, (noAliasEqs, aliasEqs)) => {
                    Ok((noAliasEqs.clone(), metamodelica::cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, right: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, .. }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::ARRAY { .. }, right: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, .. }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut noAliasEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut aliasEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (noAliasEqs, aliasEqs) = eqTplIn.clone();
                    Ok((metamodelica::cons(eqIn.clone(), noAliasEqs.clone()), aliasEqs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    eqTplOut
}

fn lower2(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inExVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inConstraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut inClassAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, mut inExtObjClasses: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>, mut inAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>, Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>, Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)))> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = inVars.clone();
    let mut outGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>> = inGlobalKnownVars.clone();
    let mut outExVars: Arc<metamodelica::List<BackendDAE::Var>> = inExVars.clone();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inREqns.clone();
    let mut outIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inIEqns.clone();
    let mut outConstraints: Arc<metamodelica::List<Arc<DAE::Constraint>>> = inConstraints.clone();
    let mut outClassAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>> = inClassAttributes.clone();
    let mut outExtObjClasses: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>> = inExtObjClasses.clone();
    let mut outAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>> = inAliasEqns.clone();
    let mut outInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)) = inInlineHT.clone();
    let mut path: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut dae_elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut class_attrs: Arc<DAE::ClassAttributes> = Arc::new(<DAE::ClassAttributes as ::std::default::Default>::default());
    let mut constraints: Arc<DAE::Constraint> = Arc::new(<DAE::Constraint as ::std::default::Default>::default());
    let mut el: Arc<DAE::Element> = Arc::new(<DAE::Element as ::std::default::Default>::default());
    let mut eq_attrs: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
    let mut whenClkCnt: i32 = 1;
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut el in &*inElements {
        let mut el = el.clone();
        let () = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ DAE::Element::EXTOBJECTCLASS { path: __esc_path, source: __esc_src } => {
            path = (*__esc_path).clone();
            src = (*__esc_src).clone();
            outExtObjClasses = metamodelica::cons(BackendDAE::ExternalObjectClass { path: path.clone(), source: src.clone() }, outExtObjClasses.clone());
            ()
        },
        Deref @ DAE::Element::VAR { .. } => {
            (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, outInlineHT) = lowerVar(el.clone(), inFunctions.clone(), outVars.clone(), outGlobalKnownVars.clone(), outExVars.clone(), outEqns.clone(), outREqns.clone(), outInlineHT.clone())?;
            ()
        },
        Deref @ DAE::Element::EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), true)?;
            ()
        },
        Deref @ DAE::Element::EQUEQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::DEFINE { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::INITIALDEFINE { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), true)?;
            ()
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), true)?;
            ()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::FOR_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), true)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), true)?;
            ()
        },
        Deref @ DAE::Element::WHEN_EQUATION { condition: __esc_e, equations: __esc_dae_elts, .. } => {
            e = (*__esc_e).clone();
            dae_elts = (*__esc_dae_elts).clone();
            if Config::synchronousFeaturesAllowed()? && Types::isClockOrSubTypeClock(Expression::r#typeof(e.clone())?) {
                (outEqns, outVars, eq_attrs) = createWhenClock(whenClkCnt, e.clone(), outEqns.clone(), outVars.clone());
                whenClkCnt = whenClkCnt + 1;
                (outVars, outGlobalKnownVars, outExVars, eqns, reqns, outIEqns, outConstraints, outClassAttributes, outExtObjClasses, outAliasEqns, outInlineHT) = lower2(dae_elts.clone(), inFunctions.clone(), outInlineHT.clone(), outVars.clone(), outGlobalKnownVars.clone(), outExVars.clone(), metamodelica::nil(), metamodelica::nil(), outIEqns.clone(), outConstraints.clone(), outClassAttributes.clone(), outExtObjClasses.clone(), outAliasEqns.clone())?;
                outEqns = listAppend(List::map1(eqns.clone(), (std::sync::Arc::new(BackendEquation::setEquationAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), eq_attrs)?, outEqns.clone());
                outREqns = listAppend(List::map1(reqns.clone(), (std::sync::Arc::new(BackendEquation::setEquationAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), eq_attrs)?, outREqns.clone());
            } else {
                (eqns, reqns, outVars) = lowerWhenEqn(el.clone(), inFunctions.clone(), metamodelica::nil(), metamodelica::nil(), outVars.clone())?;
                outEqns = listAppend(outEqns.clone(), eqns.clone());
                outREqns = listAppend(outREqns.clone(), reqns.clone());
            }
            ()
        },
        Deref @ DAE::Element::IF_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), false)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            (outEqns, outREqns, outIEqns) = lowerEqn(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), true)?;
            ()
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND, false)?;
            ()
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND, true)?;
            ()
        },
        Deref @ DAE::Element::COMP { dAElist: __esc_dae_elts, .. } => {
            dae_elts = (*__esc_dae_elts).clone();
            (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, outIEqns, outConstraints, outClassAttributes, outExtObjClasses, outAliasEqns, outInlineHT) = lower2(dae_elts.clone().reverse(), inFunctions.clone(), outInlineHT.clone(), outVars.clone(), outGlobalKnownVars.clone(), outExVars.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), outConstraints.clone(), outClassAttributes.clone(), outExtObjClasses.clone(), outAliasEqns.clone())?;
            ()
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, false)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, true)?;
            ()
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, false)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, true)?;
            ()
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, false)?;
            ()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            (outEqns, outREqns, outIEqns) = lowerAlgorithm(el.clone(), inFunctions.clone(), outEqns.clone(), outREqns.clone(), outIEqns.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, true)?;
            ()
        },
        Deref @ DAE::Element::CONSTRAINT { constraints: __esc_constraints, .. } => {
            constraints = (*__esc_constraints).clone();
            outConstraints = metamodelica::cons(constraints.clone(), outConstraints.clone());
            ()
        },
        Deref @ DAE::Element::CLASS_ATTRIBUTES { classAttrs: __esc_class_attrs } => {
            class_attrs = (*__esc_class_attrs).clone();
            outClassAttributes = metamodelica::cons(class_attrs.clone(), outClassAttributes.clone());
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lower2")); __mm_s.push_str(&*literal!(" failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![el.clone()])?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, outIEqns, outConstraints, outClassAttributes, outExtObjClasses, outAliasEqns, outInlineHT))
}

// =============================================================================
// section for processing builtin expressions
//
// Insert a unique index (starting with 1) before the first arguments of some
// builtin calls. Equal calls will get the same index.
//   - delay(expr, delayTime, delayMax)
//       => delay(index, expr, delayTime, delayMax)
//   - sample(start, interval)
//       => sample(index, start, interval)
// =============================================================================
fn processBuiltinExpressions(mut inDAE: DAE::DAElist, mut functionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(DAE::DAElist, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<BackendDAE::TimeEvent>>)> {
    let mut outDAE: DAE::DAElist;
    let mut outTree: Arc<AvlTreePathFunction::Tree>;
    let mut outTimeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    ht = HashTableExpToIndex::emptyHashTable();
    let (__pa0, __pa1, (_, (_, _, _, _, __pa2))) = DAEUtil::traverseDAE(inDAE, functionTree, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(transformBuiltinExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>))> + 'static>), (ht, 0, 0, 0, metamodelica::nil())))?;
    outDAE = __pa0.clone();
    outTree = __pa1.clone();
    outTimeEvents = __pa2.clone();
    Ok((outDAE, outTree, outTimeEvents))
}

fn transformBuiltinExpression(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32, i32, i32, Arc<metamodelica::List<BackendDAE::TimeEvent>>);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: es, attr }, (ht, _, _, _, _)) if (BaseHashTable::hasKey(inExp.clone(), ht.clone())?) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delay")).clone() }), expLst: metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), ht.clone())? }), es.clone()), attr: attr.clone() }), inTuple)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: es, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((inExp.clone(), iDelay.clone() + 1), ht.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delay")).clone() }), expLst: metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: iDelay.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone() + 1, iSample.clone(), iSpatial.clone(), timeEvents.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: es, attr }, (ht, _, _, _, _)) if (BaseHashTable::hasKey(inExp.clone(), ht.clone())?) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistribution")).clone() }), expLst: metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), ht.clone())? }), es.clone()), attr: attr.clone() }), inTuple)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: es, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((inExp.clone(), iSpatial.clone() + 1), ht.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistribution")).clone() }), expLst: metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: iSpatial.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone(), iSample.clone(), iSpatial.clone() + 1, timeEvents.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: es @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: interval, tail: Deref @ metamodelica::List::Nil } }, attr }, (ht, _, _, _, _)) if (!(Types::isClockOrSubTypeClock(Expression::r#typeof(interval.clone())?)) && BaseHashTable::hasKey(inExp.clone(), ht.clone())?) => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), ht.clone())? }), es.clone()), attr: attr.clone() }), inTuple)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: es @ Deref @ metamodelica::List::Cons { head: start, tail: Deref @ metamodelica::List::Cons { head: interval, tail: Deref @ metamodelica::List::Nil } }, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) if (!(Types::isClockOrSubTypeClock(Expression::r#typeof(interval.clone())?))) => {
            let mut ht = (*ht).clone();
            let mut iSample = (*iSample).clone();
            let mut timeEvents = (*timeEvents).clone();
            iSample = iSample.clone() + 1;
            timeEvents = List::appendElt(BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: iSample.clone(), startExp: start.clone(), intervalExp: interval.clone(), iter: None }, timeEvents.clone());
            ht = BaseHashTable::add((inExp.clone(), iSample.clone()), ht.clone())?;
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: iSample.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone(), iSample.clone(), iSpatial.clone(), timeEvents.clone()))
        },
        _ => {
            (inExp.clone(), inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

/*
 *  lower all variables
 */
pub(crate) fn lowerVars(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inExVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = inVars.clone();
    let mut outGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>> = inGlobalKnownVars.clone();
    let mut outExVars: Arc<metamodelica::List<BackendDAE::Var>> = inExVars.clone();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inREqns.clone();
    let mut cr: Arc<DAE::ComponentRef>;
    let mut arr_ty: Arc<DAE::Type>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut new_vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut inline_ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)) = HashTableExpToExp::emptyHashTable();
    for mut el in &*inElements {
        let mut el = el.clone();
        match '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(el.clone()) {
                Deref @ DAE::Element::VAR { componentRef: __pa1, ty: Deref @ DAE::Type::T_ARRAY { ty: __pa2, .. }, .. } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            cr = __pa1.clone();
            arr_ty = __pa2.clone();
            crefs = unwrap_break_err!(ComponentReference::expandCref(cr.clone(), false), '__try0);
            el = unwrap_break_err!(DAEUtil::replaceTypeInVar(arr_ty.clone(), el.clone()), '__try0);
            new_vars = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut c in (crefs.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(DAEUtil::replaceCrefInVar(c.clone(), el.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns) = unwrap_break_err!(lowerVars(new_vars.clone(), functionTree.clone(), outVars.clone(), outGlobalKnownVars.clone(), outExVars.clone(), outEqns.clone(), outREqns.clone()), '__try0);
            Ok::<_, anyhow::Error>((outEqns.clone(), outExVars.clone(), outGlobalKnownVars.clone(), outREqns.clone(), outVars.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
                outEqns = __try0_o0;
                outExVars = __try0_o1;
                outGlobalKnownVars = __try0_o2;
                outREqns = __try0_o3;
                outVars = __try0_o4;
            }
            Err(_) => {
                (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, _) = lowerVar(el.clone(), functionTree.clone(), outVars.clone(), outGlobalKnownVars.clone(), outExVars.clone(), outEqns.clone(), outREqns.clone(), inline_ht.clone())?;
            }
        }
    }
    Ok((outVars, outGlobalKnownVars, outExVars, outEqns, outREqns))
}

fn lowerVar(mut inElement: Arc<DAE::Element>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inExVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)))> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = inVars.clone();
    let mut outGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>> = inGlobalKnownVars.clone();
    let mut outExVars: Arc<metamodelica::List<BackendDAE::Var>> = inExVars.clone();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inREqns.clone();
    let mut outInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)) = inInlineHT.clone();
    let () = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. } => {
                    let mut outExVars: Arc<metamodelica::List<BackendDAE::Var>> = outExVars.clone();
                    outExVars = metamodelica::cons(lowerExtObjVar(inElement.clone(), inFunctions.clone())?, outExVars.clone());
                    Ok(((), outExVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outExVars = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cr, binding: Some(e2), source: src, .. } => {
                    if !((isStateOrAlgvar(inElement.clone()))) { bail!("guard") }
                    let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut tp: Arc<DAE::Type>;
                    let mut attr: BackendDAE::EquationAttributes;
                    let mut recordSize: Option<i32>;
                    let mut e2 = (*e2).clone();
                    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = outEqns.clone();
                    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = outVars.clone();
                    outVars = metamodelica::cons(lowerDynamicVar(inElement.clone(), inFunctions.clone())?, outVars.clone());
                    attr = BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone();
                    (tp, dims) = ComponentReference::crefTypeFull2(cr.clone(), metamodelica::nil())?;
                    tp = DAEUtil::expTypeElementType(tp.clone());
                    if DAEUtil::expTypeComplex(tp.clone()) {
                        recordSize = Some(Expression::sizeOf(tp.clone()));
                    } else {
                        recordSize = None;
                    }
                    e2 = (::match_deref::match_deref! { match &((Flags::isSet(Flags::NF_SCALARIZE.clone())?, dims.clone().is_empty(), e2.clone())) {
        (false, false, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::List::Cons { head: __esc_e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    e1 = (*__esc_e1).clone();
                    e1.clone()
        },
        _ => e2.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    e1 = Expression::crefExp(cr.clone())?;
                    if dims.clone().is_empty() {
                        outEqns = metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: src.clone(), attr: attr.clone() }), outEqns.clone());
                    } else {
                        outEqns = metamodelica::cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: Expression::dimensionsSizes(dims.clone())?, left: e1.clone(), right: e2.clone(), source: src.clone(), attr: attr.clone(), recordSize: recordSize.clone() }), outEqns.clone());
                    }
                    Ok(((), outEqns.clone(), outVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outEqns = __wb0; outVars = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { binding: None, .. } => {
                    if !((isStateOrAlgvar(inElement.clone()))) { bail!("guard") }
                    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = outVars.clone();
                    outVars = metamodelica::cons(lowerDynamicVar(inElement.clone(), inFunctions.clone())?, outVars.clone());
                    Ok(((), outVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVars = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { .. } => {
                    let mut var: BackendDAE::Var;
                    let mut outGlobalKnownVars: Arc<metamodelica::List<BackendDAE::Var>> = outGlobalKnownVars.clone();
                    let mut outInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)) = outInlineHT.clone();
                    let mut outREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = outREqns.clone();
                    (var, outInlineHT, outREqns) = lowerKnownVar(inElement.clone(), inFunctions.clone(), outInlineHT.clone(), outREqns.clone())?;
                    outGlobalKnownVars = metamodelica::cons(var.clone(), outGlobalKnownVars.clone());
                    Ok(((), outGlobalKnownVars.clone(), outInlineHT.clone(), outREqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outGlobalKnownVars = __wb0; outInlineHT = __wb1; outREqns = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerVar failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![inElement.clone()])?); ArcStr::from(__mm_s) }).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, outInlineHT))
}

fn isStateOrAlgvar(mut e: Arc<DAE::Element>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::VARIABLE { .. }, .. } => true,
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::DISCRETE { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn lowerDynamicVar(mut inElement: Arc<DAE::Element>, mut functionTree: Arc<AvlTreePathFunction::Tree>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::VAR { componentRef: name, kind, direction: dir, parallelism: prl, protection, ty: t, dims, connectorType: ct, source, variableAttributesOption: dae_var_attr, comment, innerOuter: io, encrypted, .. } => {
            let mut kind_1: BackendDAE::VarKind;
            let mut tp: Arc<DAE::Type>;
            let mut ts: Option<BackendDAE::TearingSelect>;
            let mut hideResult: Option<Arc<DAE::Exp>>;
            let mut b: bool;
            let mut dae_var_attr = (*dae_var_attr).clone();
            kind_1 = lowerVarkind(kind.clone(), t.clone(), name.clone(), dir.clone(), ct.clone(), dae_var_attr.clone(), protection.clone())?;
            tp = lowerType(t.clone())?;
            b = DAEUtil::boolVarVisibility(protection.clone())?;
            dae_var_attr = DAEUtil::setProtectedAttr(dae_var_attr.clone(), b)?;
            dae_var_attr = setMinMaxFromEnumeration(t.clone(), dae_var_attr.clone());
            if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) && !(dims.clone().is_empty()) {
                dae_var_attr = replaceFillWithExpInAttributes(dae_var_attr.clone());
            }
            ts = BackendDAEUtil::setTearingSelectAttribute(comment.clone())?;
            hideResult = BackendDAEUtil::setHideResultAttribute(comment.clone(), name.clone());
            BackendDAE::Var { varName: name.clone(), varKind: kind_1, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp, bindExp: None, tplExp: None, arryDim: dims.clone(), source: source.clone(), values: dae_var_attr.clone(), tearingSelectOption: ts, hideResult: hideResult, comment: comment.clone(), connectorType: ct.clone(), innerOuter: DAEUtil::toDAEInnerOuter(io.clone())?, unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn lowerKnownVar(mut inElement: Arc<DAE::Element>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut iInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), mut assrtEqIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outVar: BackendDAE::Var;
    let mut oInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
    let mut assrtEqOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (outVar, oInlineHT, assrtEqOut) = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: name, kind, direction: dir, parallelism: prl, protection, ty: t, binding: bind, dims, connectorType: ct, source, variableAttributesOption: dae_var_attr, comment, innerOuter: io, encrypted } => {
                    let mut kind_1: BackendDAE::VarKind;
                    let mut tp: Arc<DAE::Type>;
                    let mut ts: Option<BackendDAE::TearingSelect>;
                    let mut hideResult: Option<Arc<DAE::Exp>>;
                    let mut b: bool;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut bind = (*bind).clone();
                    let mut dae_var_attr = (*dae_var_attr).clone();
                    kind_1 = lowerKnownVarkind(kind.clone(), name.clone(), dir.clone(), ct.clone(), protection.clone())?;
                    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) && !(dims.clone().is_empty()) {
                        bind = replaceFillWithExp(bind.clone());
                        dae_var_attr = replaceFillWithExpInAttributes(dae_var_attr.clone());
                    }
                    tp = lowerType(t.clone())?;
                    b = DAEUtil::boolVarVisibility(protection.clone())?;
                    dae_var_attr = DAEUtil::setProtectedAttr(dae_var_attr.clone(), b.clone())?;
                    dae_var_attr = setMinMaxFromEnumeration(t.clone(), dae_var_attr.clone());
                    eqLst = buildAssertAlgorithms(metamodelica::nil(), source.clone(), assrtEqIn.clone());
                    ts = None;
                    hideResult = BackendDAEUtil::setHideResultAttribute(comment.clone(), name.clone());
                    Ok((BackendDAE::Var { varName: name.clone(), varKind: kind_1.clone(), varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: None, arryDim: dims.clone(), source: source.clone(), values: dae_var_attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: DAEUtil::toDAEInnerOuter(io.clone())?, unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() }, iInlineHT.clone(), eqLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerKnownVar failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![inElement.clone()])?); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, oInlineHT, assrtEqOut))
}

pub(crate) fn lowerKnownVarSingle(mut element: Arc<DAE::Element>) -> Result<Option<BackendDAE::Var>> {
    let mut var_opt: Option<BackendDAE::Var>;
    var_opt = (::match_deref::match_deref! { match &(element.clone()) {
        elem @ Deref @ DAE::Element::VAR { .. } if (DAEUtil::isParamOrConstVarKind(var_field!((**elem).kind, DAE::Element::VAR).clone())) => {
            let mut visibility: bool;
            let mut var: BackendDAE::Var;
            visibility = DAEUtil::boolVarVisibility(var_field!((**elem).protection, DAE::Element::VAR).clone())?;
            var = BackendDAE::Var { varName: var_field!((**elem).componentRef, DAE::Element::VAR).clone(), varKind: lowerKnownVarkind(var_field!((**elem).kind, DAE::Element::VAR).clone(), var_field!((**elem).componentRef, DAE::Element::VAR).clone(), var_field!((**elem).direction, DAE::Element::VAR).clone(), var_field!((**elem).connectorType, DAE::Element::VAR).clone(), var_field!((**elem).protection, DAE::Element::VAR).clone())?, varDirection: var_field!((**elem).direction, DAE::Element::VAR).clone(), varParallelism: var_field!((**elem).parallelism, DAE::Element::VAR).clone(), varType: lowerType(var_field!((**elem).ty, DAE::Element::VAR).clone())?, bindExp: var_field!((**elem).binding, DAE::Element::VAR).clone(), tplExp: None, arryDim: var_field!((*element).dims, DAE::Element::VAR).clone(), source: var_field!((*element).source, DAE::Element::VAR).clone(), values: setMinMaxFromEnumeration(var_field!((**elem).ty, DAE::Element::VAR).clone(), DAEUtil::setProtectedAttr(var_field!((**elem).variableAttributesOption, DAE::Element::VAR).clone(), visibility)?), tearingSelectOption: None, hideResult: BackendDAEUtil::setHideResultAttribute(var_field!((*element).comment, DAE::Element::VAR).clone(), var_field!((**elem).componentRef, DAE::Element::VAR).clone()), comment: var_field!((*element).comment, DAE::Element::VAR).clone(), connectorType: var_field!((*element).connectorType, DAE::Element::VAR).clone(), innerOuter: DAEUtil::toDAEInnerOuter(var_field!((*element).innerOuter, DAE::Element::VAR).clone())?, unreplaceable: false, initNonlinear: false, encrypted: var_field!((**elem).encrypted, DAE::Element::VAR).clone() };
            Some(var)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var_opt)
}

fn replaceFillWithExpInAttributes(mut attr: Option<Arc<DAE::VariableAttributes>>) -> Option<Arc<DAE::VariableAttributes>> {
    let mut attr: Option<Arc<DAE::VariableAttributes>> = attr;
    attr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut u = (*u).clone();
            let mut du = (*du).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            let mut n = (*n).clone();
            q = replaceFillWithExp(q.clone());
            u = replaceFillWithExp(u.clone());
            du = replaceFillWithExp(du.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            n = replaceFillWithExp(n.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut u = (*u).clone();
            let mut du = (*du).clone();
            q = replaceFillWithExp(q.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            u = replaceFillWithExp(u.clone());
            du = replaceFillWithExp(du.clone());
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => {
            attr
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

fn replaceFillWithExp(mut bind: Option<Arc<DAE::Exp>>) -> Option<Arc<DAE::Exp>> {
    let mut bind: Option<Arc<DAE::Exp>> = bind;
    let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    bind = (::match_deref::match_deref! { match &(bind.clone()) {
        Some(Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::List::Cons { head: __esc_e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
            e1 = (*__esc_e1).clone();
            Some(e1.clone())
        },
        _ => bind,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bind
}

fn buildAssertAlgorithms(mut assrtIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut source: Arc<DAE::ElementSource>, mut eqIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut eqOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqIn.clone();
    for mut assrt in &*assrtIn {
        let mut assrt = assrt.clone();
        eqOut = metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![assrt.clone()] }), source: source.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), eqOut.clone());
    }
    eqOut
}

fn inlineExpOpt(mut iOptExp: Option<Arc<DAE::Exp>>, mut fnstpl: Functiontuple, mut iSource: Arc<DAE::ElementSource>, mut iInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> (Option<Arc<DAE::Exp>>, Arc<DAE::ElementSource>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Statement>>>) {
    let mut oOptExp: Option<Arc<DAE::Exp>>;
    let mut oSource: Arc<DAE::ElementSource>;
    let mut oInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
    let mut assrtLstOut: Arc<metamodelica::List<Arc<DAE::Statement>>>;
    (oOptExp, oSource, oInlineHT, assrtLstOut) = (::match_deref::match_deref! { match &(iOptExp.clone()) {
        None => {
            (iOptExp, iSource, iInlineHT, metamodelica::nil())
        },
        Some(e) => {
            let mut source: Arc<DAE::ElementSource>;
            let mut inlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
            let mut assrtLst: Arc<metamodelica::List<Arc<DAE::Statement>>>;
            let mut e = (*e).clone();
            (e, source, inlineHT, assrtLst) = inlineExpOpt1(e.clone(), fnstpl, iSource, iInlineHT);
            (Some(e.clone()), source, inlineHT, assrtLst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oOptExp, oSource, oInlineHT, assrtLstOut)
}

fn inlineExpOpt1(mut iExp: Arc<DAE::Exp>, mut fnstpl: Functiontuple, mut iSource: Arc<DAE::ElementSource>, mut iInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> (Arc<DAE::Exp>, Arc<DAE::ElementSource>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Statement>>>) {
    let mut oExp: Arc<DAE::Exp>;
    let mut oSource: Arc<DAE::ElementSource>;
    let mut oInlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
    let mut assrtLstOut: Arc<metamodelica::List<Arc<DAE::Statement>>>;
    (oExp, oSource, oInlineHT, assrtLstOut) = 'mc: {
        let __mc_input = iExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    e1 = BaseHashTable::get(iExp.clone(), iInlineHT.clone())?;
                    source = ElementSource::addSymbolicTransformation(iSource.clone(), Arc::new(DAE::SymbolicOperation::OP_INLINE { before: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: iExp.clone() }), after: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1.clone() }) }))?;
                    Ok((e1.clone(), source.clone(), iInlineHT.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut inlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
                    let mut inlined: bool;
                    (e1, source, inlined, _) = Inline::inlineExp(iExp.clone(), fnstpl.clone(), iSource.clone());
                    inlineHT = if (inlined.clone()) {BaseHashTable::add((iExp.clone(), e1.clone()), iInlineHT.clone())?} else {iInlineHT.clone()};
                    Ok((e1.clone(), source.clone(), inlineHT.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e, sub: elst } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut e = (*e).clone();
                    e1 = BaseHashTable::get(e.clone(), iInlineHT.clone())?;
                    source = ElementSource::addSymbolicTransformation(iSource.clone(), Arc::new(DAE::SymbolicOperation::OP_INLINE { before: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), after: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1.clone() }) }))?;
                    (e, source, _, _) = Inline::inlineExp(Arc::new(DAE::Exp::ASUB { exp: e1.clone(), sub: elst.clone() }), fnstpl.clone(), source.clone());
                    Ok((e.clone(), source.clone(), iInlineHT.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e, sub: elst } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut inlineHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
                    let mut inlined: bool;
                    let mut assrtLst: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut assrtLst1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut assrtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut e = (*e).clone();
                    (e1, _, inlined, assrtLst1) = Inline::inlineExp(e.clone(), fnstpl.clone(), iSource.clone());
                    inlineHT = if (inlined.clone()) {BaseHashTable::add((e.clone(), e1.clone()), iInlineHT.clone())?} else {iInlineHT.clone()};
                    (e, source, _, assrtLst2) = Inline::inlineExp(Arc::new(DAE::Exp::ASUB { exp: e1.clone(), sub: elst.clone() }), fnstpl.clone(), iSource.clone());
                    assrtLst = listAppend(assrtLst1.clone(), assrtLst2.clone());
                    Ok((e.clone(), source.clone(), inlineHT.clone(), assrtLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    (e, source, _, _) = Inline::inlineExp(iExp.clone(), fnstpl.clone(), iSource.clone());
                    Ok((e.clone(), source.clone(), iInlineHT.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (oExp, oSource, oInlineHT, assrtLstOut)
}

fn setMinMaxFromEnumeration(mut inType: Arc<DAE::Type>, mut inVarAttr: Option<Arc<DAE::VariableAttributes>>) -> Option<Arc<DAE::VariableAttributes>> {
    let mut outVarAttr: Option<Arc<DAE::VariableAttributes>>;
    outVarAttr = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_ENUMERATION { path, names, .. } => {
            let mut min: Option<Arc<DAE::Exp>>;
            let mut max: Option<Arc<DAE::Exp>>;
            (min, max) = DAEUtil::getMinMaxValues(inVarAttr.clone());
            setMinMaxFromEnumeration1(min, max, inVarAttr, path.clone(), names.clone())
        },
        _ => {
            inVarAttr
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outVarAttr
}

fn setMinMaxFromEnumeration1(mut inMin: Option<Arc<DAE::Exp>>, mut inMax: Option<Arc<DAE::Exp>>, mut inVarAttr: Option<Arc<DAE::VariableAttributes>>, mut inPath: Arc<Absyn::Path>, mut inNames: Arc<metamodelica::List<ArcStr>>) -> Option<Arc<DAE::VariableAttributes>> {
    let mut outVarAttr: Option<Arc<DAE::VariableAttributes>>;
    outVarAttr = 'mc: {
        let __mc_input = (inMin.clone(), inMax.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, None) => {
                    let mut i: i32;
                    let mut namee1: Arc<Absyn::Path>;
                    let mut nameen: Arc<Absyn::Path>;
                    let mut s1: ArcStr;
                    let mut sn: ArcStr;
                    i = (inNames.clone().len() as i32);
                    s1 = (listHead(inNames.clone())?).clone();
                    namee1 = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (s1.clone()).clone() }))?;
                    sn = ((inNames.clone()).get(i.clone())?).clone();
                    nameen = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (sn.clone()).clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: namee1.clone(), index: 1 })), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: nameen.clone(), index: i.clone() })))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, Some(_)) => {
                    let mut namee1: Arc<Absyn::Path>;
                    let mut s1: ArcStr;
                    s1 = (listHead(inNames.clone())?).clone();
                    namee1 = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (s1.clone()).clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: namee1.clone(), index: 1 })), inMax.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(_), None) => {
                    let mut i: i32;
                    let mut nameen: Arc<Absyn::Path>;
                    let mut sn: ArcStr;
                    i = (inNames.clone().len() as i32);
                    sn = ((inNames.clone()).get(i.clone())?).clone();
                    nameen = AbsynUtil::joinPaths(inPath.clone(), Arc::new(Absyn::Path::IDENT { name: (sn.clone()).clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), inMin.clone(), Some(Arc::new(DAE::Exp::ENUM_LITERAL { name: nameen.clone(), index: i.clone() })))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVarAttr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outVarAttr
}

// protected function fixParameterStartBinding
//   input Option<DAE.Exp> bind;
//   input DAE.Type ty;
//   input Option<DAE.VariableAttributes> attr;
//   input BackendDAE.VarKind kind;
//   output Option<DAE.Exp> outBind;
// algorithm
//   outBind := matchcontinue (bind, ty, attr, kind)
//     local
//       DAE.Exp exp;
//     case (NONE(), DAE.T_REAL(source=_), _, BackendDAE.PARAM())
//       equation
//         exp = DAEUtil.getStartAttr(attr);
//       then SOME(exp);
//     else bind;
//   end matchcontinue;
// end fixParameterStartBinding;
fn lowerVarkind(mut inVarKind: DAE::VarKind, mut inType: Arc<DAE::Type>, mut inComponentRef: Arc<DAE::ComponentRef>, mut inVarDirection: DAE::VarDirection, mut inConnectorType: Arc<DAE::ConnectorType>, mut daeAttr: Option<Arc<DAE::VariableAttributes>>, mut protection: DAE::VarVisibility) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind;
    outVarKind = (::match_deref::match_deref! { match &((inVarKind, daeAttr)) {
        (DAE::VarKind::VARIABLE { .. }, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. })) if (!(Types::isDiscreteType(inType.clone()))) => BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false },
        (DAE::VarKind::VARIABLE { .. }, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::PREFER { .. }), .. })) if (!(Types::isDiscreteType(inType.clone()))) => BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false },
        _ => {
            let false = (ConnectUtil::topLevelInput(inComponentRef, inVarDirection, inConnectorType, protection)?) else { bail!("pattern mismatch") };
            (::match_deref::match_deref! { match &((inVarKind, inType.clone())) {
        (DAE::VarKind::VARIABLE { .. }, Deref @ DAE::Type::T_BOOL { .. }) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE { .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE { .. }, _) => openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        (DAE::VarKind::DISCRETE { .. }, _) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        _ => bail!("match: no arm matched"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarKind)
}

fn lowerKnownVarkind(mut varKind: DAE::VarKind, mut componentRef: Arc<DAE::ComponentRef>, mut varDirection: DAE::VarDirection, mut connectorType: Arc<DAE::ConnectorType>, mut visibility: DAE::VarVisibility) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind;
    outVarKind = 'mc: {
        let __mc_input = varKind;
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::PARAM { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(openmodelica_backend_types::BackendDAE::VarKind::PARAM)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::CONST { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(openmodelica_backend_types::BackendDAE::VarKind::CONST)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::VARIABLE { .. } = __mc_input.clone() else { bail!("nomatch") };
            let true = (ConnectUtil::topLevelInput(componentRef.clone(), varDirection, connectorType.clone(), visibility)?) else { bail!("pattern mismatch") };
            Ok(openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("function lowerKnownVarkind failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarKind)
}

fn lowerType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => DAE::T_REAL_DEFAULT().clone(),
        Deref @ DAE::Type::T_INTEGER { .. } => DAE::T_INTEGER_DEFAULT().clone(),
        Deref @ DAE::Type::T_BOOL { .. } => DAE::T_BOOL_DEFAULT().clone(),
        Deref @ DAE::Type::T_STRING { .. } => DAE::T_STRING_DEFAULT().clone(),
        Deref @ DAE::Type::T_CLOCK { .. } => DAE::T_CLOCK_DEFAULT().clone(),
        Deref @ DAE::Type::T_ENUMERATION { .. } => inType,
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } => inType,
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => inType,
        Deref @ DAE::Type::T_ARRAY { .. } => inType,
        Deref @ DAE::Type::T_FUNCTION { .. } => inType,
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lowerType: ")); __mm_s.push_str(&*TypesDump::printTypeStr(inType)); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn lowerExtObjVar(mut inElement: Arc<DAE::Element>, mut functionTree: Arc<AvlTreePathFunction::Tree>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::VAR { componentRef: name, direction: dir, parallelism: prl, ty: t, binding: bind, dims, connectorType: ct, source, variableAttributesOption: dae_var_attr, comment, innerOuter: io, encrypted, .. } => {
            let mut kind_1: BackendDAE::VarKind;
            let mut tp: Arc<DAE::Type>;
            let mut ts: Option<BackendDAE::TearingSelect>;
            let mut hideResult: Option<Arc<DAE::Exp>>;
            kind_1 = lowerExtObjVarkind(t.clone())?;
            tp = lowerType(t.clone())?;
            ts = None;
            hideResult = None;
            BackendDAE::Var { varName: name.clone(), varKind: kind_1, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp, bindExp: bind.clone(), tplExp: None, arryDim: dims.clone(), source: source.clone(), values: dae_var_attr.clone(), tearingSelectOption: ts, hideResult: hideResult, comment: comment.clone(), connectorType: ct.clone(), innerOuter: DAEUtil::toDAEInnerOuter(io.clone())?, unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn lowerExtObjVarkind(mut inType: Arc<DAE::Type>) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind;
    let mut path: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    outVarKind = BackendDAE::VarKind::EXTOBJ { fullClassName: path };
    Ok(outVarKind)
}

/*
 *  lower all equation types
 */
fn lowerEqn(mut inElement: Arc<DAE::Element>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inIEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inInitialization: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outREquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outIEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (outEquations, outREquations, outIEquations) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: explst }, scalar: Deref @ DAE::Exp::TUPLE { PR: explst1 }, source } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            eqns = lowerTupleAssignment(explst.clone(), explst1.clone(), source.clone(), functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: Deref @ DAE::Exp::TUPLE { PR: explst }, exp2: Deref @ DAE::Exp::TUPLE { PR: explst1 }, source } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            eqns = lowerTupleAssignment(explst.clone(), explst1.clone(), source.clone(), functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::EQUATION { exp: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, scalar: e2 @ Deref @ DAE::Exp::CALL { .. }, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::EQUATION { exp: e2 @ Deref @ DAE::Exp::CALL { .. }, scalar: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, exp2: e2 @ Deref @ DAE::Exp::CALL { .. }, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e2 @ Deref @ DAE::Exp::CALL { .. }, exp2: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            (metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEquations), inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            (inEquations, inREquations, metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), inIEquations))
        },
        Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
                e1 = Expression::crefExp(cr1.clone())?;
                e2 = Expression::crefExp(cr2.clone())?;
            } else {
                e1 = Expression::crefToExp(cr1.clone())?;
                e2 = Expression::crefToExp(cr2.clone())?;
            }
            eqns = lowerExtendedRecordEqn(e1.clone(), e2.clone(), source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e2, source } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cr1.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            (metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEquations), inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALDEFINE { componentRef: cr1, exp: e2, source } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cr1.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            (inEquations, inREquations, metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inIEquations))
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Inline::simplifyAndForceInlineEquationExp(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), (Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Inline::simplifyAndForceInlineEquationExp(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), (Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1 @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, array: e2 @ Deref @ DAE::Exp::CALL { path, .. }, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut b1: bool;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            b1 = stringEq((AbsynUtil::pathLastIdent(path.clone())?).clone(), (literal!("equalityConstraint")).clone());
            eqns = if (b1) {inREquations.clone()} else {inEquations.clone()};
            eqns = lowerArrayEqn(dims.clone(), e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), eqns)?;
            (eqns, _) = if (b1) {(inEquations, eqns)} else {(eqns, inREquations.clone())};
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerArrayEqn(dims.clone(), e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1_1 = __pa0.clone();
            e2_1 = __pa1.clone();
            source = __pa2.clone();
            eqns = lowerArrayEqn(dims.clone(), e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::FOR_EQUATION { iter: s, range: e1, equations: eqnslst, .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerEqns(eqnslst.clone(), functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            eqns = listAppend(List::map2(eqns, (std::sync::Arc::new(lowerForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ArcStr, Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> + 'static>), (s.clone()).clone(), e1.clone())?, inEquations);
            reqns = listAppend(List::map2(reqns, (std::sync::Arc::new(lowerForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ArcStr, Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> + 'static>), (s.clone()).clone(), e1.clone())?, inREquations);
            ieqns = listAppend(List::map2(ieqns, (std::sync::Arc::new(lowerForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ArcStr, Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> + 'static>), (s.clone()).clone(), e1.clone())?, inIEquations);
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { iter: s, range: e1, equations: eqnslst, .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerEqns(eqnslst.clone(), functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            eqns = listAppend(List::map2(eqns, (std::sync::Arc::new(lowerForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ArcStr, Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> + 'static>), (s.clone()).clone(), e1.clone())?, inEquations);
            reqns = listAppend(List::map2(reqns, (std::sync::Arc::new(lowerForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ArcStr, Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> + 'static>), (s.clone()).clone(), e1.clone())?, inREquations);
            ieqns = listAppend(List::map2(ieqns, (std::sync::Arc::new(lowerForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ArcStr, Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> + 'static>), (s.clone()).clone(), e1.clone())?, inIEquations);
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, tail: Deref @ metamodelica::List::Nil }, equations2: Deref @ metamodelica::List::Cons { head: eqnslst, tail: Deref @ metamodelica::List::Nil }, equations3: Deref @ metamodelica::List::Nil, .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerEqns(eqnslst.clone(), functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            ieqns = List::flatten(list![eqns, reqns, ieqns, inIEquations])?;
            (inEquations, inREquations, ieqns)
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: explst, equations2: eqnslstlst, equations3: eqnslst, source } => {
            let mut daeElts: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqnslstlst = (*eqnslstlst).clone();
            let mut eqnslst = (*eqnslst).clone();
            (eqnslstlst, eqnslst, daeElts) = lowerIfEquationAsserts(explst.clone(), eqnslstlst.clone(), eqnslst.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            (eqns, reqns, ieqns) = lowerEqns(daeElts, functionTree.clone(), inEquations, inREquations, inIEquations, inInitialization)?;
            eqns = lowerIfEquation(explst.clone(), eqnslstlst.clone(), eqnslst.clone(), metamodelica::nil(), metamodelica::nil(), source.clone(), functionTree, eqns)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: explst, equations2: eqnslstlst, equations3: eqnslst, source } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            eqns = lowerIfEquation(explst.clone(), eqnslstlst.clone(), eqnslst.clone(), metamodelica::nil(), metamodelica::nil(), source.clone(), functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::EXPAND, false)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::EXPAND, true)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, inInitialization)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, inInitialization)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::TERMINATE { message: msg, source } => {
            (inEquations, metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inREquations), inIEquations)
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { message: msg, source } => {
            (inEquations, inREquations, metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inIEquations))
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, false)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, true)?;
            (eqns, reqns, ieqns)
        },
        _ => {
            let mut s: ArcStr;
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerEqn failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![inElement.clone()])?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(s.clone()).clone()], ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(inElement)?))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEquations, outREquations, outIEquations))
}

fn lowerForEquation(mut eq: Arc<BackendDAE::Equation>, mut iter: ArcStr, mut range: Arc<DAE::Exp>) -> Result<Arc<BackendDAE::Equation>> {
    let mut forEq: Arc<BackendDAE::Equation>;
    let mut iterExp: Arc<DAE::Exp>;
    let mut start: Arc<DAE::Exp>;
    let mut stop: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(range) {
        Deref @ DAE::Exp::RANGE { ty: __pa0, start: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    start = __pa1.clone();
    stop = __pa2.clone();
    ty = Types::unliftArray(ty)?;
    iterExp = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iter).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty });
    forEq = Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: iterExp, start: start, stop: stop, body: eq.clone(), source: BackendEquation::equationSource(eq.clone())?, attr: BackendEquation::getEquationAttributes(eq)? });
    Ok(forEq)
}

fn lowerIfEquation(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut inSource: Arc<DAE::ElementSource>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEquations = 'mc: {
        let __mc_input = (conditions, theneqns, conditions1.clone(), theneqns1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut breqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut bieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (beqns, breqns, bieqns) = lowerEqns(elseenqs.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone(), inEquations.clone()])?;
                    Ok(beqns.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut beqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
                    let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut breqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut bieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    explst = conditions1.clone().reverse();
                    beqnslst = lowerEqnsLst(theneqns1.clone(), functionTree.clone(), metamodelica::nil(), false)?;
                    (beqns, breqns, bieqns) = lowerEqns(elseenqs.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone()])?;
                    Ok(metamodelica::cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: beqnslst.clone(), eqnsfalse: beqns.clone(), source: inSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _, _) => {
                    let mut source: Arc<DAE::ElementSource>;
                    let mut e = (*e).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), inSource.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    source = __pa1.clone();
                    Ok(lowerIfEquation1(e.clone(), explst.clone(), eqns.clone(), eqnslst.clone(), elseenqs.clone(), conditions1.clone(), theneqns1.clone(), source.clone(), functionTree.clone(), inEquations.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEquations)
}

fn lowerIfEquation1(mut cond: Arc<DAE::Exp>, mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqn: Arc<metamodelica::List<Arc<DAE::Element>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut source: Arc<DAE::ElementSource>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEqns = 'mc: {
        let __mc_input = (cond.clone(), conditions1.clone(), theneqns1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut breqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut bieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (beqns, breqns, bieqns) = lowerEqns(theneqn.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone(), inEqns.clone()])?;
                    Ok(beqns.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut beqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
                    let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut breqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut bieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    explst = conditions1.clone().reverse();
                    beqnslst = lowerEqnsLst(theneqns1.clone(), functionTree.clone(), metamodelica::nil(), false)?;
                    (beqns, breqns, bieqns) = lowerEqns(theneqn.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone()])?;
                    Ok(metamodelica::cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: beqnslst.clone(), eqnsfalse: beqns.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, _) => {
                    Ok(lowerIfEquation(conditions.clone(), theneqns.clone(), elseenqs.clone(), conditions1.clone(), theneqns1.clone(), source.clone(), functionTree.clone(), inEqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(lowerIfEquation(conditions.clone(), theneqns.clone(), elseenqs.clone(), metamodelica::cons(cond.clone(), conditions1.clone()), metamodelica::cons(theneqn.clone(), theneqns1.clone()), source.clone(), functionTree.clone(), inEqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEqns)
}

fn lowerEqns(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inIEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inInitialization: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElements) {
        Deref @ metamodelica::List::Nil => {
            return Ok((inEquations, inREquations, inIEquations))
        },
        Deref @ metamodelica::List::Cons { head: element, tail: elements } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerEqn(element.clone(), functionTree.clone(), inEquations, inREquations, inIEquations, inInitialization)?;
            { (inElements, functionTree, inEquations, inREquations, inIEquations, inInitialization) = (elements.clone(), functionTree, eqns, reqns, ieqns, inInitialization); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerEqnsLst(mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inInitialization: bool) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElements) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inEquations)
        },
        Deref @ metamodelica::List::Cons { head: element, tail: elements } => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, reqns, ieqns) = lowerEqns(element.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            eqns = List::flatten(list![eqns, reqns, ieqns])?;
            { (inElements, functionTree, inEquations, inInitialization) = (elements.clone(), functionTree, metamodelica::cons(eqns, inEquations), inInitialization); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerIfEquationAsserts(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut inEqns: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, theneqns)) {
        (_, Deref @ metamodelica::List::Nil) => {
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            (beqns, eqns) = lowerIfEquationAsserts1(elseenqs, None, conditions1, metamodelica::nil(), inEqns)?;
            return Ok((theneqns1.reverse(), beqns, eqns.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }) => {
            let mut eqns1: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqnslst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>;
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), Some(e.clone()), conditions1.clone(), metamodelica::nil(), inEqns)?;
            { (conditions, theneqns, elseenqs, conditions1, theneqns1, inEqns) = (explst.clone(), eqnslst.clone(), elseenqs, metamodelica::cons(e.clone(), conditions1), metamodelica::cons(beqns, theneqns1), eqns.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerIfEquationAsserts1(mut brancheqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut condition: Option<Arc<DAE::Exp>>, mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut brancheqns1: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inEqns: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((brancheqns, condition.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((brancheqns1.reverse(), inEqns))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { condition: cond, message: msg, level, source }, tail: eqns }, None) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), cond.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(DAE::Element::ASSERT { condition: e.clone(), message: msg.clone(), level: level.clone(), source: source.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { condition: cond, message: msg, level, source }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: cond.clone(), expElse: Arc::new(DAE::Exp::BCONST { bool: true }) });
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(DAE::Element::ASSERT { condition: e.clone(), message: msg.clone(), level: level.clone(), source: source.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { message: msg, source }, tail: eqns }, None) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), Arc::new(DAE::Exp::BCONST { bool: true }))?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { message: msg, source }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { exp, source }, tail: eqns }, None) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { exp, source }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut eqns = (*eqns).clone();
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, metamodelica::cons(eqn.clone(), brancheqns1), inEqns); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn makeIfExp(mut cond: Arc<DAE::Exp>, mut else_: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = Arc::new(DAE::Exp::IFEXP { expCond: cond, expThen: Arc::new(DAE::Exp::BCONST { bool: true }), expElse: else_ });
    oExp
}

fn lowerExtendedRecordEqns(mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source: Arc<DAE::ElementSource>, mut inEqAttributes: BackendDAE::EquationAttributes, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((explst1, explst2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(inEqns)
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: elst1 }, Deref @ metamodelica::List::Cons { head: e2, tail: elst2 }) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            eqns = lowerExtendedRecordEqn(e1.clone(), e2.clone(), source.clone(), inEqAttributes, functionTree.clone(), inEqns)?;
            { (explst1, explst2, source, inEqAttributes, functionTree, inEqns) = (elst1.clone(), elst2.clone(), source, inEqAttributes, functionTree, eqns); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerExtendedRecordEqn(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inEqAttributes: BackendDAE::EquationAttributes, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEqns = 'mc: {
        let __mc_input = inEqns.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    explst1 = Expression::splitRecord(inExp1.clone(), Expression::r#typeof(inExp1.clone())?)?;
                    explst2 = Expression::splitRecord(inExp2.clone(), Expression::r#typeof(inExp2.clone())?)?;
                    Ok(lowerExtendedRecordEqns(explst1.clone(), explst2.clone(), source.clone(), inEqAttributes, functionTree.clone(), inEqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: Arc<DAE::Type>;
                    let mut size: i32;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeComplex(tp.clone())) else { bail!("pattern mismatch") };
                    size = Expression::sizeOf(tp.clone());
                    Ok(metamodelica::cons(Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: inEqAttributes }), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeArray(tp.clone())) else { bail!("pattern mismatch") };
                    dims = Expression::arrayDimension(tp.clone());
                    Ok(lowerArrayEqn(dims.clone(), inExp1.clone(), inExp2.clone(), source.clone(), inEqAttributes, inEqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: Arc<DAE::Type>;
                    let mut size: i32;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (Types::isTuple(tp.clone())) else { bail!("pattern mismatch") };
                    size = Expression::sizeOf(tp.clone());
                    Ok(metamodelica::cons(Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: inEqAttributes }), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: Arc<DAE::Type>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    b1 = DAEUtil::expTypeComplex(tp.clone());
                    b2 = DAEUtil::expTypeArray(tp.clone());
                    b3 = Types::isTuple(tp.clone());
                    let false = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    Ok(metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: inExp1.clone(), scalar: inExp2.clone(), source: source.clone(), attr: inEqAttributes }), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAECreate.lowerExtendedRecordEqn failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEqns)
}

fn lowerArrayEqn(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inEqAttributes: BackendDAE::EquationAttributes, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut dimensions: Arc<metamodelica::List<i32>>;
    let mut ea1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut ea2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut tp: Arc<DAE::Type>;
    let mut recordSize: i32;
    tp = Expression::r#typeof(e1.clone())?;
    tp = DAEUtil::expTypeElementType(tp);
    if DAEUtil::expTypeComplex(tp.clone()) {
        recordSize = Expression::sizeOf(tp);
        dimensions = Expression::dimensionsSizes(dims)?;
        outEqsLst = metamodelica::cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimensions, left: e1, right: e2, source: source, attr: inEqAttributes, recordSize: Some(recordSize) }), iAcc);
    } else if (Expression::isArray(e1.clone()) || Expression::isMatrix(e1.clone())) && (Expression::isArray(e2.clone()) || Expression::isMatrix(e2.clone())) {
        ea1 = Expression::flattenArrayExpToList(e1);
        ea2 = Expression::flattenArrayExpToList(e2);
        outEqsLst = generateEquations(ea1, ea2, source, inEqAttributes, iAcc)?;
    } else {
        dimensions = Expression::dimensionsSizes(dims)?;
        outEqsLst = metamodelica::cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimensions, left: e1, right: e2, source: source, attr: inEqAttributes, recordSize: None }), iAcc);
    }
    Ok(outEqsLst)
}

fn generateEquations(mut iE1lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iE2lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source: Arc<DAE::ElementSource>, mut inEqAttributes: BackendDAE::EquationAttributes, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iE1lst, iE2lst)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(iAcc)
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: e1lst }, Deref @ metamodelica::List::Cons { head: e2, tail: e2lst }) => {
            { (iE1lst, iE2lst, source, inEqAttributes, iAcc) = (e1lst.clone(), e2lst.clone(), source.clone(), inEqAttributes, metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source, attr: inEqAttributes }), iAcc)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn createWhenClock(mut whenClkCnt: i32, mut e: Arc<DAE::Exp>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, BackendDAE::EquationAttributes) {
    let mut outEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outEqAttrs: BackendDAE::EquationAttributes;
    let mut cr: Arc<DAE::ComponentRef>;
    cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(whenClkCnt)); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
    outVars = metamodelica::cons(BackendDAE::Var { varName: cr.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_CLOCK_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::DEFAULT), hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false }, inVars);
    outEqs = metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr, ty: DAE::T_CLOCK_DEFAULT().clone() }), scalar: e, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEqs);
    outEqAttrs = BackendEquation::defaultClockedEqAttr(whenClkCnt);
    (outEqs, outVars, outEqAttrs)
}

fn lowerWhenEqn(mut inElement: Arc<DAE::Element>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outREquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = inVars.clone();
    (outEquationLst, outREquationLst) = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqnl, elsewhen_: None, source } => {
                    let mut res: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut rEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut cond = (*cond).clone();
                    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = outVars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: cond.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cond = __pa0.clone();
                    (res, rEqns, outVars) = lowerWhenEqn2(eqnl.clone().reverse(), cond.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), outVars.clone())?;
                    res = mergeWhenEqns(inEquationLst.clone(), res.clone(), metamodelica::nil())?;
                    rEqns = mergeWhenEqns(inREquationLst.clone(), rEqns.clone(), metamodelica::nil())?;
                    Ok(((res.clone(), rEqns.clone()), outVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVars = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqnl, elsewhen_: Some(elsePart), source } => {
                    let mut res: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut rEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut trueEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut trueREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut cond = (*cond).clone();
                    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = outVars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: cond.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cond = __pa0.clone();
                    (trueEqnLst, trueREqns, outVars) = lowerWhenEqn2(eqnl.clone().reverse(), cond.clone(), functionTree.clone(), metamodelica::nil(), metamodelica::nil(), outVars.clone())?;
                    res = mergeWhenEqns(inEquationLst.clone(), trueEqnLst.clone(), metamodelica::nil())?;
                    rEqns = mergeWhenEqns(inREquationLst.clone(), trueREqns.clone(), metamodelica::nil())?;
                    (res, rEqns, outVars) = lowerWhenEqn(elsePart.clone(), functionTree.clone(), res.clone(), rEqns.clone(), outVars.clone())?;
                    Ok(((res.clone(), rEqns.clone()), outVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVars = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut source: Arc<DAE::ElementSource>;
                    source = ElementSource::getElementSource(inElement.clone())?;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerWhenEqn: equation not handled:\n")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![inElement.clone()])?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquationLst, outREquationLst, outVars))
}

fn lowerWhenEqn2(mut inDAEElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inCond: Arc<DAE::Exp>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut iEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iREquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVar_lst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outREquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = inVar_lst.clone();
    (outEquationLst, outREquationLst) = 'mc: {
        let __mc_input = inDAEElementLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((iEquationLst.clone(), iREquationLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut e: Arc<DAE::Exp>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    e = Expression::crefExp(cr2.clone())?;
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 1, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), metamodelica::cons(eq.clone(), iEquationLst.clone()), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    (e, _) = ExpressionSolve::solve(Expression::crefExp(cr.clone())?, e.clone(), Expression::crefExp(cr.clone())?, None)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    source = __pa1.clone();
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 1, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), metamodelica::cons(eq.clone(), iEquationLst.clone()), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: lhs @ Deref @ DAE::Exp::TUPLE { .. }, scalar: e @ Deref @ DAE::Exp::CALL { path: _, .. }, source }, tail: xs } => {
                    let mut size: i32;
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut ty: Arc<DAE::Type>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    ty = Expression::r#typeof(lhs.clone())?;
                    size = Expression::sizeOf(ty.clone());
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: e.clone(), source: source.clone() }], elsewhenPart: None }), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    eqnl = metamodelica::cons(eq.clone(), iEquationLst.clone());
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), eqnl.clone(), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: expl }, scalar: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    eqnl = lowerWhenTupleEqn(expl.clone(), inCond.clone(), e.clone(), source.clone(), 1, iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), eqnl.clone(), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: el @ Deref @ DAE::Element::EQUATION { exp: cre @ Deref @ DAE::Exp::CREF { .. }, scalar: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    if let Ok(__iflet0) = ExpressionSolve::solve(cre.clone(), e.clone(), cre.clone(), None) {
                        e = __iflet0.0;
                    } else {
                        Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to solve ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![el.clone()])?); ArcStr::from(__mm_s) }).clone())?;
                        bail!("fail");
                    }
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa1 }, __pa2) => (__pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa1.clone();
                    source = __pa2.clone();
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: cre.clone(), right: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 1, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), metamodelica::cons(eq.clone(), iEquationLst.clone()), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: cre @ Deref @ DAE::Exp::CREF { .. }, rhs: e, source }, tail: xs } => {
                    let mut size: i32;
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    source = __pa1.clone();
                    size = Expression::sizeOf(Expression::r#typeof(cre.clone())?);
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: cre.clone(), right: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), metamodelica::cons(eq.clone(), iEquationLst.clone()), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: cre @ Deref @ DAE::Exp::TUPLE { PR: expl }, rhs: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    source = __pa1.clone();
                    eqnl = lowerWhenTupleEqn(expl.clone(), inCond.clone(), e.clone(), source.clone(), 1, iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), eqnl.clone(), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqnslst, equations3: eqns, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                    let mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    ht = HashTableCrToExpSourceTpl::emptyHashTable();
                    ht = lowerWhenIfEqnsElse(eqns.clone(), functionTree.clone(), ht.clone())?;
                    ht = lowerWhenIfEqns(expl.clone().reverse(), eqnslst.clone().reverse(), functionTree.clone(), ht.clone())?;
                    crexplst = BaseHashTable::hashTableList(ht.clone())?;
                    eqnl = lowerWhenIfEqns2(crexplst.clone(), inCond.clone(), source.clone(), iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), eqnl.clone(), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { dimension: ds, exp: cre @ Deref @ DAE::Exp::CREF { .. }, array: e, source }, tail: xs } => {
                    let mut size: i32;
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    source = __pa1.clone();
                    size = List::fold(Expression::dimensionsSizes(ds.clone())?, (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: cre.clone(), right: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), metamodelica::cons(eq.clone(), iEquationLst.clone()), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: cre @ Deref @ DAE::Exp::TUPLE { PR: expl }, array: e, source, .. }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    source = __pa1.clone();
                    eqnl = lowerWhenTupleEqn(expl.clone(), inCond.clone(), e.clone(), source.clone(), 1, iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), eqnl.clone(), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { condition: cond, message: e, level, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::ASSERT { condition: cond.clone(), message: e.clone(), level: level.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), iEquationLst.clone(), metamodelica::cons(eq.clone(), iREquationLst.clone()), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::REINIT { componentRef: cr, exp: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut var_opt: Option<Arc<metamodelica::List<BackendDAE::Var>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::REINIT { stateVar: cr.clone(), value: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    vars = BackendVariable::listVar(outVar_lst.clone())?;
                    var_opt = BackendVariable::getVarTryHard(cr.clone(), vars.clone());
                    if isSome(var_opt.clone()) {
                        for mut var in &*Util::getOption(var_opt.clone())? {
                            let mut var = var.clone();
                            var = BackendVariable::setVarStateSelect(var.clone(), openmodelica_frontend_types::DAE::StateSelect::ALWAYS)?;
                            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                        }
                    }
                    outVar_lst = BackendVariable::varList(vars.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), iEquationLst.clone(), metamodelica::cons(eq.clone(), iREquationLst.clone()), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { message: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::TERMINATE { message: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), iEquationLst.clone(), metamodelica::cons(eq.clone(), iREquationLst.clone()), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { exp: e, source }, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut whenEq: Arc<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::NORETCALL { exp: e.clone(), source: source.clone() };
                    whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), iEquationLst.clone(), metamodelica::cons(eq.clone(), iREquationLst.clone()), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: el, tail: _ } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAECreate.lowerWhenEqn2 failed on:")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![el.clone()])?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut eqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqnl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut outVar_lst: Arc<metamodelica::List<BackendDAE::Var>> = outVar_lst.clone();
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(xs.clone(), inCond.clone(), functionTree.clone(), iEquationLst.clone(), iREquationLst.clone(), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outVar_lst = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquationLst, outREquationLst, outVar_lst))
}

fn lowerWhenTupleEqn(mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCond: Arc<DAE::Exp>, mut e: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut i: i32, mut iEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(explst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iEquationLst)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: rest } => {
            let mut size: i32;
            let mut whenEq: Arc<BackendDAE::WhenEquation>;
            let mut whenOp: BackendDAE::WhenOperator;
            size = Expression::sizeOf(ty.clone());
            whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: Arc::new(DAE::Exp::TSUB { exp: e.clone(), ix: i, ty: ty.clone() }), source: source.clone() };
            whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp], elsewhenPart: None });
            { (explst, inCond, e, source, i, iEquationLst) = (rest.clone(), inCond, e, source.clone(), i + 1, metamodelica::cons(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: whenEq, source: source, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), iEquationLst)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerWhenIfEqns2(mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>, mut inCond: Arc<DAE::Exp>, mut iSource: Arc<DAE::ElementSource>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(crexplst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inEqns)
        },
        Deref @ metamodelica::List::Cons { head: (cr, (e, source)), tail: rest } => {
            let mut size: i32;
            let mut whenEq: Arc<BackendDAE::WhenEquation>;
            let mut whenOp: BackendDAE::WhenOperator;
            let mut source = (*source).clone();
            source = ElementSource::mergeSources(iSource.clone(), source.clone())?;
            size = Expression::sizeOf(Expression::r#typeof(e.clone())?);
            whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
            whenEq = Arc::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp], elsewhenPart: None });
            { (crexplst, inCond, iSource, inEqns) = (rest.clone(), inCond, iSource, metamodelica::cons(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: whenEq, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEqns)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerWhenIfEqns(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, theneqns)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(iHt)
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: rest }) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            ht = lowerWhenIfEqns1(c.clone(), eqns.clone(), functionTree.clone(), iHt)?;
            { (conditions, theneqns, functionTree, iHt) = (explst.clone(), rest.clone(), functionTree, ht); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerWhenIfEqns1(mut condition: Arc<DAE::Exp>, mut brancheqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(brancheqns) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iHt)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2, source }, tail: rest } => {
            let mut e: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut source1: Arc<DAE::ElementSource>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            e = Expression::crefExp(cr2.clone())?;
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            (exp, source1) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            source = ElementSource::mergeSources(source.clone(), source1)?;
            ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
            { (condition, brancheqns, functionTree, iHt) = (condition, rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut source1: Arc<DAE::ElementSource>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            (exp, source1) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            source = ElementSource::mergeSources(source.clone(), source1)?;
            ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
            { (condition, brancheqns, functionTree, iHt) = (condition, rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, source }, tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut source1: Arc<DAE::ElementSource>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            (exp, source1) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            source = ElementSource::mergeSources(source.clone(), source1)?;
            ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
            { (condition, brancheqns, functionTree, iHt) = (condition, rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, rhs: e, source }, tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut source1: Arc<DAE::ElementSource>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            (exp, source1) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            source = ElementSource::mergeSources(source.clone(), source1)?;
            ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
            { (condition, brancheqns, functionTree, iHt) = (condition, rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, array: e, source, .. }, tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut source1: Arc<DAE::ElementSource>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            (exp, source1) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            source = ElementSource::mergeSources(source.clone(), source1)?;
            ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
            { (condition, brancheqns, functionTree, iHt) = (condition, rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqnslst, equations3: eqns, source }, tail: rest } => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>;
            ht = HashTableCrToExpSourceTpl::emptyHashTable();
            ht = lowerWhenIfEqnsElse(eqns.clone(), functionTree.clone(), ht)?;
            ht = lowerWhenIfEqns(expl.clone().reverse(), eqnslst.clone().reverse(), functionTree.clone(), ht)?;
            crexplst = BaseHashTable::hashTableList(ht)?;
            ht = lowerWhenIfEqnsMergeNestedIf(crexplst, condition.clone(), source.clone(), iHt)?;
            { (condition, brancheqns, functionTree, iHt) = (condition, rest.clone(), functionTree, ht); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerWhenIfEqnsMergeNestedIf(mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>, mut inCond: Arc<DAE::Exp>, mut iSource: Arc<DAE::ElementSource>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(crexplst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iHt)
        },
        Deref @ metamodelica::List::Cons { head: (cr, (e, source)), tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let mut source = (*source).clone();
            (exp, _) = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: inCond.clone(), expThen: e.clone(), expElse: exp });
            source = ElementSource::mergeSources(iSource.clone(), source.clone())?;
            ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
            { (crexplst, inCond, iSource, iHt) = (rest.clone(), inCond, iSource, ht); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lowerWhenIfEqnsElse(mut elseenqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::ElementSource>)) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(elseenqs) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iHt.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            e = Expression::crefExp(cr2.clone())?;
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
            { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
            { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
            { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, rhs: e, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
            { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, array: e, source, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
            { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqnslst, equations3: eqns, .. }, tail: rest } => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<DAE::Exp>, Arc<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
            ht = lowerWhenIfEqnsElse(eqns.clone(), functionTree.clone(), iHt.clone())?;
            ht = lowerWhenIfEqns(expl.clone().reverse(), eqnslst.clone().reverse(), functionTree.clone(), ht)?;
            { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn mergeWhenEqns(mut trueEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut elseEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEquationLst = 'mc: {
        let __mc_input = (trueEqnList.clone(), elseEqnList.clone(), inEquationLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok(inEquationLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(trueEqnList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil) => {
                    Ok(elseEqnList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(listAppend(inEquationLst.clone(), elseEqnList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(listAppend(inEquationLst.clone(), trueEqnList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: inEqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: whenEq @ Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: whenElsePart }, source, attr }, tail: trueEqns }, _, _) => {
                    let mut res: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut elseEqnsRest: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut result: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut whenEqRes: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
                    let mut added: bool;
                    result = inEquationLst.clone();
                    elseEqnsRest = metamodelica::nil();
                    added = false;
                    for mut eqn in &*elseEqnList.clone() {
                        let mut eqn = eqn.clone();
                        let () = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: eq @ Deref @ BackendDAE::WhenEquation { whenStmtLst: whenStmtLst2, .. }, .. } => {
                    for mut elem in &*whenStmtLst.clone() {
                        let mut elem = elem.clone();
                        let () = (match elem.clone() {
        BackendDAE::WhenOperator::ASSIGN { left: ref eleft, .. } => {
                    for mut stmt in &*whenStmtLst2.clone() {
                        let mut stmt = stmt.clone();
                        let () = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                    let BackendDAE::WhenOperator::ASSIGN { left: ref eleft2, .. } = __mc_input.clone() else { bail!("nomatch") };
                    let mut added: bool = added.clone();
                    let mut res: Arc<BackendDAE::Equation>;
                    let mut result: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = result.clone();
                    let mut whenEqRes: Arc<BackendDAE::WhenEquation>;
                    let true = (ExpressionBasics::expEqual(eleft.clone(), eleft2.clone())?) else { bail!("pattern mismatch") };
                    whenEqRes = BackendEquation::setWhenElsePart(whenEq.clone(), eq.clone())?;
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                    result = metamodelica::cons(res.clone(), result.clone());
                    added = true;
                    Ok(((), added.clone(), result.clone()))
        })() { added = __wb0; result = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut elseEqnsRest: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = elseEqnsRest.clone();
                    elseEqnsRest = metamodelica::cons(eqn.clone(), elseEqnsRest.clone());
                    Ok(((), elseEqnsRest.clone()))
        })() { elseEqnsRest = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    }
                    ()
        },
        BackendDAE::WhenOperator::REINIT { stateVar: ref crleft, .. } => {
                    for mut stmt in &*whenStmtLst2.clone() {
                        let mut stmt = stmt.clone();
                        let () = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                    let BackendDAE::WhenOperator::REINIT { stateVar: ref crleft2, .. } = __mc_input.clone() else { bail!("nomatch") };
                    let mut added: bool = added.clone();
                    let mut res: Arc<BackendDAE::Equation>;
                    let mut result: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = result.clone();
                    let mut whenEqRes: Arc<BackendDAE::WhenEquation>;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(crleft.clone(), crleft2.clone())?) else { bail!("pattern mismatch") };
                    whenEqRes = BackendEquation::setWhenElsePart(whenEq.clone(), eq.clone())?;
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                    result = metamodelica::cons(res.clone(), result.clone());
                    added = true;
                    Ok(((), added.clone(), result.clone()))
        })() { added = __wb0; result = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut elseEqnsRest: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = elseEqnsRest.clone();
                    elseEqnsRest = metamodelica::cons(eqn.clone(), elseEqnsRest.clone());
                    Ok(((), elseEqnsRest.clone()))
        })() { elseEqnsRest = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    }
                    ()
        },
        _ => {
                    whenEqRes = BackendEquation::setWhenElsePart(whenEq.clone(), eq.clone())?;
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                    result = metamodelica::cons(res.clone(), result.clone());
                    added = true;
                    ()
        },
    });
                    }
                    ()
        },
        _ => {
                    res = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: whenElsePart.clone() }), source: source.clone(), attr: attr.clone() });
                    result = metamodelica::cons(res.clone(), result.clone());
                    ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                    if !(added.clone()) {
                        result = metamodelica::cons(inEqn.clone(), result.clone());
                    }
                    result = mergeWhenEqns(trueEqns.clone(), elseEqnsRest.clone(), result.clone())?;
                    Ok(result.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("BackendDAECreate.mergeWhenEqns: Error in mergeWhenEqns.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEquationLst)
}

fn lowerTupleAssignment(mut target_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inEq_source: Arc<DAE::ElementSource>, mut funcs: Arc<AvlTreePathFunction::Tree>, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((target_expl, source_expl)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(iEqns)
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, tail: rest_targets }, Deref @ metamodelica::List::Cons { head: _, tail: rest_sources }) => {
            { (target_expl, source_expl, inEq_source, funcs, iEqns) = (rest_targets.clone(), rest_sources.clone(), inEq_source, funcs, iEqns); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: target, tail: rest_targets }, Deref @ metamodelica::List::Cons { head: source, tail: rest_sources }) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eq_source: Arc<DAE::ElementSource>;
            let mut target = (*target).clone();
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: target.clone(), rhs: source.clone() }), inEq_source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            target = __pa0.clone();
            source = __pa1.clone();
            eq_source = __pa2.clone();
            eqns = lowerExtendedRecordEqn(target.clone(), source.clone(), inEq_source, BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(), funcs.clone(), iEqns)?;
            { (target_expl, source_expl, inEq_source, funcs, iEqns) = (rest_targets.clone(), rest_sources.clone(), eq_source, funcs, eqns); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

/*
 *   lower algorithms
 */
fn lowerAlgorithm(mut inElement: Arc<DAE::Element>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inREquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inIEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inCrefExpansion: DAE::Expand, mut inInitialization: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outREquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outIEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (outEquations, outREquations, outIEquations) = ({
        let mut eqAttributes: BackendDAE::EquationAttributes = if (inInitialization) {BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone()} else {BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone()};
        'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil }, .. } => {
                    Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil }, .. } => {
                    Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source } => {
                    let mut size: i32;
                    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    crefLst = CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), inCrefExpansion)?;
                    size = (crefLst.clone().len() as i32);
                    if inInitialization {
                        ieqns = metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone());
                        eqns = inEquations.clone();
                        reqns = inREquations.clone();
                    } else {
                        if size.clone() > 0 {
                            eqns = metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inEquations.clone());
                            reqns = inREquations.clone();
                        } else {
                            eqns = inEquations.clone();
                            reqns = metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone());
                        }
                        ieqns = inIEquations.clone();
                    }
                    Ok((eqns.clone(), reqns.clone(), ieqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { algorithm_: alg, source } => {
                    let mut size: i32;
                    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefLst = CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), inCrefExpansion)?;
                    size = (crefLst.clone().len() as i32);
                    Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: Deref @ DAE::Exp::BCONST { bool: true }, .. } => {
                    Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { condition: Deref @ DAE::Exp::BCONST { bool: true }, .. } => {
                    Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: cond, message: msg, level, source } => {
                    let mut alg: Arc<DAE::Algorithm>;
                    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    BackendDAEUtil::checkAssertCondition(cond.clone(), msg.clone(), level.clone(), ElementSource::getElementSourceFileInfo(source.clone()))?;
                    alg = Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })] });
                    if inInitialization {
                        reqns = inREquations.clone();
                        ieqns = metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone());
                    } else {
                        reqns = metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone());
                        ieqns = inIEquations.clone();
                    }
                    Ok((inEquations.clone(), reqns.clone(), ieqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { condition: cond, message: msg, level, source } => {
                    let mut alg: Arc<DAE::Algorithm>;
                    BackendDAEUtil::checkAssertCondition(cond.clone(), msg.clone(), level.clone(), ElementSource::getElementSourceFileInfo(source.clone()))?;
                    alg = Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })] });
                    Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), inIEquations.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { message: msg, source } => {
                    Ok((inEquations.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone()), inIEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { message: msg, source } => {
                    Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { exp: e, source } => {
                    let mut alg: Arc<DAE::Algorithm>;
                    alg = Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] });
                    Ok((inEquations.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone()), inIEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_NORETCALL { exp: e, source } => {
                    let mut alg: Arc<DAE::Algorithm>;
                    alg = Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] });
                    Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let 0 = (Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerAlgorithm failed for:\n")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![inElement.clone()])?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(inElement.clone())?))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok((outEquations, outREquations, outIEquations))
}

/*
 *  alias Equations
 */
fn handleAliasEquations(mut iAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut oREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut oIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns) = (::match_deref::match_deref! { match &(iAliasEqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iVars, inGlobalKnownVars, iExtVars, iAVars, iEqns, iREqns, iIEqns)
        },
        _ => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ieqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (vars, globalKnownVars, extvars, avars, eqns, reqns, ieqns) = handleAliasEquations1(iAliasEqns, iVars, inGlobalKnownVars, iExtVars, iAVars, iEqns, iREqns, iIEqns)?;
            (vars, globalKnownVars, extvars, avars, eqns, reqns, ieqns)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns))
}

fn handleAliasEquations1(mut iAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut oREqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut oIEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    repl = BackendVarTransform::emptyReplacements();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, repl, oEqns) = handleAliasEquations2(iAliasEqns, iVars, inGlobalKnownVars, iExtVars, iAVars, repl, iEqns)?;
    (oAVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(oAVars, (std::sync::Arc::new(fnptr!(replaceAliasVarTraverser, BackendDAE::Var, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
    oVars = BackendVariable::rehashVariables(oVars)?;
    (oEqns, _) = BackendVarTransform::replaceEquations(oEqns, repl.clone(), None)?;
    (oREqns, _) = BackendVarTransform::replaceEquations(iREqns, repl.clone(), None)?;
    (oIEqns, _) = BackendVarTransform::replaceEquations(iIEqns, repl, None)?;
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns))
}

fn replaceAliasVarTraverser(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> (BackendDAE::Var, BackendVarTransform::VariableReplacements) {
    let mut outVar: BackendDAE::Var;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (outVar, repl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(e), .. }, repl) => {
                    let mut v1: BackendDAE::Var;
                    let mut e1: Arc<DAE::Exp>;
                    let mut b: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    b = Expression::isConst(e1.clone())?;
                    v1 = if (!(b.clone())) {BackendVariable::setBindExp(v.clone(), Some(e1.clone()))} else {v.clone()};
                    Ok((v1.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, repl)
}

fn handleAliasEquations2(mut iAliasEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iAliasEqns) {
        Deref @ metamodelica::List::Nil => {
            return Ok((iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns))
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source }, tail: aliaseqns } => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ecr1: Arc<DAE::Exp>;
            let mut ecr2: Arc<DAE::Exp>;
            ecr1 = Expression::crefExp(cr1.clone())?;
            (ecr1, _) = BackendVarTransform::replaceExp(ecr1, iRepl.clone(), None);
            ecr2 = Expression::crefExp(cr2.clone())?;
            (ecr2, _) = BackendVarTransform::replaceExp(ecr2, iRepl.clone(), None);
            (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAlias(ecr1, ecr2, source.clone(), iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns)?;
            { (iAliasEqns, iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns) = (aliaseqns.clone(), vars, globalKnownVars, extvars, avars, repl, eqns); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn selectAlias(mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut oEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns) = 'mc: {
        let __mc_input = (exp1.clone(), exp2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: explst1, .. }, Deref @ DAE::Exp::ARRAY { array: explst2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, ty: Deref @ DAE::Type::T_ARRAY { dims: dims1, .. } }, Deref @ DAE::Exp::ARRAY { array: explst2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefs1 = ComponentReference::expandArrayCref(cr1.clone(), dims1.clone())?;
                    explst1 = List::map(crefs1.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: explst1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: Deref @ DAE::Type::T_ARRAY { dims: dims2, .. } }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefs2 = ComponentReference::expandArrayCref(cr2.clone(), dims2.clone())?;
                    explst2 = List::map(crefs2.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, ty: Deref @ DAE::Type::T_ARRAY { dims: dims1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: Deref @ DAE::Type::T_ARRAY { dims: dims2, .. } }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefs1 = ComponentReference::expandArrayCref(cr1.clone(), dims1.clone())?;
                    explst1 = List::map(crefs1.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    crefs2 = ComponentReference::expandArrayCref(cr2.clone(), dims2.clone())?;
                    explst2 = List::map(crefs2.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: explstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: explstlst2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(List::flatten(explstlst1.clone())?, List::flatten(explstlst2.clone())?, source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut arrayTyp1: i32;
                    let mut arrayTyp2: i32;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut v1: BackendDAE::Var;
                    let mut v2: BackendDAE::Var;
                    (v1, i1, arrayTyp1) = getVar(cr1.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone())?;
                    (v2, i2, arrayTyp2) = getVar(cr2.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone())?;
                    (vars, globalKnownVars, extvars, avars, repl) = selectAliasVar(v1.clone(), i1.clone(), arrayTyp1.clone(), exp1.clone(), v2.clone(), i2.clone(), arrayTyp2.clone(), exp2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), iEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    explst1 = Expression::splitRecord(exp1.clone(), Expression::r#typeof(exp1.clone())?)?;
                    explst2 = Expression::splitRecord(exp2.clone(), Expression::r#typeof(exp2.clone())?)?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(explst1.clone(), explst2.clone(), source.clone(), iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok((iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: exp1.clone(), scalar: exp2.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), iEqns.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns))
}

fn getVar(mut cr: Arc<DAE::ComponentRef>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, i32, i32)> {
    let mut oVar: BackendDAE::Var;
    let mut index: i32;
    let mut varrArray: i32;
    (oVar, index, varrArray) = 'mc: {
        let __mc_input = iExtVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut v: BackendDAE::Var;
            let mut i: i32;
            (v, i) = BackendVariable::getVarSingle(cr.clone(), iVars.clone())?;
            Ok((v.clone(), i.clone(), 1))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut v: BackendDAE::Var;
            let mut i: i32;
            (v, i) = BackendVariable::getVarSingle(cr.clone(), inGlobalKnownVars.clone())?;
            Ok((v.clone(), i.clone(), 2))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut v: BackendDAE::Var;
            let mut i: i32;
            (v, i) = BackendVariable::getVarSingle(cr.clone(), iExtVars.clone())?;
            Ok((v.clone(), i.clone(), 3))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oVar, index, varrArray))
}

fn selectAliasLst(mut iexplst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iexplst2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source: Arc<DAE::ElementSource>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements, mut iEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iexplst1, iexplst2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok((iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns))
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: explst1 }, Deref @ metamodelica::List::Cons { head: e2, tail: explst2 }) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, _) = BackendVarTransform::replaceExp(e1.clone(), iRepl.clone(), None);
            (e2, _) = BackendVarTransform::replaceExp(e2.clone(), iRepl.clone(), None);
            (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAlias(e1.clone(), e2.clone(), source.clone(), iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns)?;
            { (iexplst1, iexplst2, source, iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns) = (explst1.clone(), explst2.clone(), source, vars, globalKnownVars, extvars, avars, repl, eqns); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn selectAliasVar(mut v1: BackendDAE::Var, mut index1: i32, mut arrayIndx1: i32, mut e1: Arc<DAE::Exp>, mut v2: BackendDAE::Var, mut index2: i32, mut arrayIndx2: i32, mut e2: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut iVars: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut iExtVars: BackendDAE::Variables, mut iAVars: BackendDAE::Variables, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendDAE::Variables, BackendVarTransform::VariableReplacements)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl) = (match (v1.clone(), arrayIndx1, v2.clone(), arrayIndx2) {
        (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, 1, BackendDAE::Var { varName: ref cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            let false = (BackendVariable::isStateVar(v2.clone())) else { bail!("pattern mismatch") };
            replaceableAlias(v2.clone())?;
            var = BackendVariable::mergeAliasVars(v1, v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v2, metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e1.clone()));
            (vars, _) = BackendVariable::removeVar(index2, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            vars = BackendVariable::addVar(var, vars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr2.clone(), (literal!(" = ")).clone(), e1, (literal!(" found (4).\n")).clone())?;
            }
            (vars, inGlobalKnownVars, iExtVars, avars, repl)
        },
        (BackendDAE::Var { varName: ref cr1, .. }, 1, BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            let false = (BackendVariable::isStateVar(v1.clone())) else { bail!("pattern mismatch") };
            replaceableAlias(v1.clone())?;
            var = BackendVariable::mergeAliasVars(v2, v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v1, metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e2.clone()));
            (vars, _) = BackendVariable::removeVar(index1, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            vars = BackendVariable::addVar(var, vars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), e2, (literal!(" found (4).\n")).clone())?;
            }
            (vars, inGlobalKnownVars, iExtVars, avars, repl)
        },
        (BackendDAE::Var { varName: ref cr1, .. }, 1, BackendDAE::Var { varName: ref cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            let mut acr: Arc<DAE::ComponentRef>;
            let mut w1: i32;
            let mut w2: i32;
            let mut aindx: i32;
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut e: Arc<DAE::Exp>;
            b1 = BackendVariable::isStateVar(v1.clone());
            b2 = BackendVariable::isStateVar(v2.clone());
            let true = (boolEq(b1, b2)) else { bail!("pattern mismatch") };
            replaceableAlias(v1.clone())?;
            replaceableAlias(v2.clone())?;
            w1 = BackendVariable::calcAliasKey(v1.clone())?;
            w2 = BackendVariable::calcAliasKey(v2.clone())?;
            b = intGt(w2, w1);
            (acr, avar, aindx, _, _, var, e) = if (b) {(cr2.clone(), v2, index2, e2, cr1.clone(), v1, e1)} else {(cr1.clone(), v1, index1, e1, cr2.clone(), v2, e2)};
            var = BackendVariable::mergeAliasVars(var, avar.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(avar, metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: acr.clone(), exp: e.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e.clone()));
            avar = if (b1) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(aindx, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            vars = BackendVariable::addVar(var, vars)?;
            repl = BackendVarTransform::addReplacement(iRepl, acr.clone(), e.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), acr, (literal!(" = ")).clone(), e, (literal!(" found (4).\n")).clone())?;
            }
            (vars, inGlobalKnownVars, iExtVars, avars, repl)
        },
        (BackendDAE::Var { varName: ref cr1, .. }, 1, BackendDAE::Var { .. }, 2) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v1.clone())?;
            var = BackendVariable::mergeAliasVars(v2, v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v1.clone(), metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e2.clone()));
            avar = if (BackendVariable::isStateVar(v1)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index1, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            globalKnownVars = BackendVariable::addVar(var, inGlobalKnownVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), e2, (literal!(" found (4).\n")).clone())?;
            }
            (vars, globalKnownVars, iExtVars, avars, repl)
        },
        (BackendDAE::Var { .. }, 2, BackendDAE::Var { varName: ref cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v2.clone())?;
            var = BackendVariable::mergeAliasVars(v1, v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v2.clone(), metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e1.clone()));
            avar = if (BackendVariable::isStateVar(v2)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index2, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            globalKnownVars = BackendVariable::addVar(var, inGlobalKnownVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr2.clone(), (literal!(" = ")).clone(), e1, (literal!(" found (4).\n")).clone())?;
            }
            (vars, globalKnownVars, iExtVars, avars, repl)
        },
        (BackendDAE::Var { varName: ref cr1, .. }, 1, BackendDAE::Var { .. }, 3) => {
            let mut vars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v1.clone())?;
            var = BackendVariable::mergeAliasVars(v2, v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v1.clone(), metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e2.clone()));
            avar = if (BackendVariable::isStateVar(v1)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index1, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            extvars = BackendVariable::addVar(var, iExtVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr1.clone(), (literal!(" = ")).clone(), e2, (literal!(" found (4).\n")).clone())?;
            }
            (vars, inGlobalKnownVars, extvars, avars, repl)
        },
        (BackendDAE::Var { .. }, 3, BackendDAE::Var { varName: ref cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut var: BackendDAE::Var;
            let mut avar: BackendDAE::Var;
            replaceableAlias(v2.clone())?;
            var = BackendVariable::mergeAliasVars(v1, v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v2.clone(), metamodelica::cons(Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e1.clone()));
            avar = if (BackendVariable::isStateVar(v2)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index2, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            extvars = BackendVariable::addVar(var, iExtVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr((literal!("Alias Equation ")).clone(), cr2.clone(), (literal!(" = ")).clone(), e1, (literal!(" found (4).\n")).clone())?;
            }
            (vars, inGlobalKnownVars, extvars, avars, repl)
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl))
}

fn replaceableAlias(mut var: BackendDAE::Var) -> Result<()> {
    let () = (match var.clone() {
        _ => {
            let false = (BackendVariable::isVarOnTopLevelAndOutput(var.clone())) else { bail!("pattern mismatch") };
            let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
            let false = (BackendVariable::varHasUncertainValueRefine(var)) else { bail!("pattern mismatch") };
            ()
        },
    });
    Ok(())
}

/*
 *     other helping functions
 */
fn detectImplicitDiscrete(mut inVariables: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold1(inEquationLst, (std::sync::Arc::new(fnptr!(detectImplicitDiscreteFold, Arc<BackendDAE::Equation>, BackendDAE::Variables, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), inGlobalKnownVars, inVariables)?;
    Ok(outVariables)
}

fn detectImplicitDiscreteFold(mut inEquation: Arc<BackendDAE::Equation>, mut inGlobalKnownVars: BackendDAE::Variables, mut inVariables: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = inEquation;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cr.clone(), inVariables.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    Ok(BackendVariable::addVars(vars.clone(), inVariables.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    crefs = Expression::getAllCrefs(e.clone())?;
                    crefs = List::flatten(List::map1(crefs.clone(), (std::sync::Arc::new(ComponentReference::expandCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), true)?)?;
                    (vars, _) = BackendVariable::getVarLst(crefs.clone(), inVariables.clone());
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    Ok(BackendVariable::addVars(vars.clone(), inVariables.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst }, .. } => {
                    Ok(detectImplicitDiscreteAlgsStatemens(inVariables.clone(), inGlobalKnownVars.clone(), statementLst.clone(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVariables.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outVariables
}

fn getVarsFromExp(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = (inExpLst, inVariables);
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
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: expLst }, variables) => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cref.clone(), variables.clone())?;
                    varLst = getVarsFromExp(expLst.clone(), variables.clone())?;
                    Ok(listAppend(vars.clone(), varLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: expLst }, variables) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    varLst = getVarsFromExp(expLst.clone(), variables.clone())?;
                    Ok(varLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

fn detectImplicitDiscreteAlgsStatemens(mut inVariables: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut insideWhen: bool) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (inVariables, inGlobalKnownVars, inStatementLst, insideWhen);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, _, Deref @ metamodelica::List::Nil, _) => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    vars = List::map(vars.clone(), (std::sync::Arc::new({ let __pe_b1 = openmodelica_backend_types::BackendDAE::VarKind::DISCRETE; move |__pe_a0| BackendVariable::setVarKind(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<BackendDAE::Var> + 'static>))?;
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, sub: subs }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::subscriptCref(cr.clone(), subs.clone())?;
                    (vars, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    vars = getVarsFromExp(expExpLst.clone(), v.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cr.clone(), v.clone())?;
                    vars = List::map1(vars.clone(), (std::sync::Arc::new(BackendVariable::setVarKind) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::VarKind) -> Result<BackendDAE::Var> + 'static>), openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    v_1 = BackendVariable::addVars(vars.clone(), v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { statementLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_: tp, iter: iteratorName, range: e, statementLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut iteratorExp: Arc<DAE::Exp>;
                    let mut iteratorexps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    cr = ComponentReferenceBasics::makeCrefIdent((iteratorName.clone()).clone(), tp.clone(), metamodelica::nil());
                    iteratorExp = Expression::crefExp(cr.clone())?;
                    iteratorexps = BackendDAEUtil::extendRange(e.clone(), globalKnownVars.clone())?;
                    v_1 = detectImplicitDiscreteAlgsStatemensFor(iteratorExp.clone(), iteratorexps.clone(), v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), true)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { statementLst, elseWhen: None, .. }, tail: xs }, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), xs.clone(), false)?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { statementLst, elseWhen: Some(statement), .. }, tail: xs }, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut v_3: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), list![statement.clone()], true)?;
                    v_3 = detectImplicitDiscreteAlgsStatemens(v_2.clone(), globalKnownVars.clone(), xs.clone(), false)?;
                    Ok(v_3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::List::Cons { head: _, tail: xs }, b) => {
                    let mut v_1: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), xs.clone(), b.clone())?;
                    Ok(v_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariables)
}

fn detectImplicitDiscreteAlgsStatemensFor(mut inIteratorExp: Arc<DAE::Exp>, mut inExplst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVariables: BackendDAE::Variables, mut inGlobalKnownVars: BackendDAE::Variables, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut insideWhen: bool) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (inIteratorExp, inExplst, inVariables, inGlobalKnownVars, inStatementLst.clone(), insideWhen);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, v, globalKnownVars, _, _) => {
                    let mut v_1: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), inStatementLst.clone(), true)?;
                    Ok(v_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, v, globalKnownVars, statementLst, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut statementLst1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()))?;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst1.clone(), true)?;
                    Ok(v_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::List::Cons { head: e, tail: rest }, v, globalKnownVars, statementLst, b) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut statementLst1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()))?;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst1.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemensFor(ie.clone(), rest.clone(), v_1.clone(), globalKnownVars.clone(), statementLst.clone(), b.clone())?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::List::Cons { head: e, tail: rest }, v, globalKnownVars, statementLst, b) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut statementLst1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()))?;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), statementLst1.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemensFor(ie.clone(), rest.clone(), v_1.clone(), globalKnownVars.clone(), statementLst.clone(), b.clone())?;
                    Ok(v_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    metamodelica::print((literal!("BackendDAECreate.detectImplicitDiscreteAlgsStatemensFor failed \n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariables)
}

fn lowerFunctions(mut funcTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    let mut funcTree: Arc<AvlTreePathFunction::Tree> = funcTree;
    funcTree = AvlTreePathFunction::map(funcTree, (std::sync::Arc::new(deriveFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>) -> Result<Option<DAE::Function>> + 'static>))?;
    Ok(funcTree)
}

fn deriveFunction(mut key: Arc<Absyn::Path>, mut value: Option<DAE::Function>) -> Result<Option<DAE::Function>> {
    let mut value: Option<DAE::Function> = value;
    value = (::match_deref::match_deref! { match &(value.clone()) {
        Some(r#fn @ DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE { .. }, tail: _ }, .. }) => {
            Error::addSourceMessage(Error::UNSUPPORTED_LANGUAGE_FEATURE.clone(), list![(literal!("partial derivative of function")).clone(), (literal!("use --newBackend flag.")).clone()], var_field!(r#fn.source, DAE::Function::FUNCTION).info.clone())?;
            bail!("fail")
        },
        _ => {
            value
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

fn renameFunctionParameter(mut fTreeIn: Arc<AvlTreePathFunction::Tree>) -> Arc<AvlTreePathFunction::Tree> {
    let mut fTreeOut: Arc<AvlTreePathFunction::Tree>;
    fTreeOut = 'mc: {
        let __mc_input = fTreeIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut funcLst: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<DAE::Function>)>>;
                    let mut funcs: Arc<AvlTreePathFunction::Tree>;
                    let true = (stringEq((Flags::getConfigString(Flags::SIMCODE_TARGET.clone())?).clone(), (literal!("Cpp")).clone())) else { bail!("pattern mismatch") };
                    funcLst = AvlTreePathFunction::toList(fTreeIn.clone(), metamodelica::nil());
                    funcLst = List::map(funcLst.clone(), (std::sync::Arc::new(renameFunctionParameter1) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Path>, Option<DAE::Function>)) -> Result<(Arc<Absyn::Path>, Option<DAE::Function>)> + 'static>))?;
                    funcs = AvlTreePathFunction::addList(AvlTreePathFunction::new(), funcLst.clone(), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
                    Ok(funcs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(fTreeIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    fTreeOut
}

fn renameFunctionParameter1(mut funcIn: (Arc<Absyn::Path>, Option<DAE::Function>)) -> Result<(Arc<Absyn::Path>, Option<DAE::Function>)> {
    let mut funcOut: (Arc<Absyn::Path>, Option<DAE::Function>);
    let mut key: Arc<Absyn::Path>;
    let mut value: Option<DAE::Function>;
    let mut pathName: ArcStr = arcstr::literal!("");
    let mut r#fn: DAE::Function = <DAE::Function as ::std::default::Default>::default();
    (key, value) = funcIn.clone();
    funcOut = (match value {
        Some(mut __esc_fn @ DAE::Function::FUNCTION { .. }) => {
            r#fn = __esc_fn.clone();
            pathName = (AbsynUtil::pathString(var_field!(r#fn.path, DAE::Function::FUNCTION).clone(), (literal!(".")).clone(), true, false)?).clone();
            pathName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::stringReplaceChar((pathName).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!("_")); ArcStr::from(__mm_s) }).clone();
            let __owned_variant_functions_0 = ({
        let mut __acc: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
        for mut fn_def in (var_field!(r#fn.functions, DAE::Function::FUNCTION).clone()).into_iter().cloned() {
            let __x = renameFunctionParameter2(fn_def.clone(), (pathName.clone()).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                *functions = __owned_variant_functions_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            (key, Some(r#fn.clone()))
        },
        _ => funcIn,
    });
    Ok(funcOut)
}

fn renameFunctionParameter2(mut funcIn: DAE::FunctionDefinition, mut pathName: ArcStr) -> DAE::FunctionDefinition {
    let mut funcOut: DAE::FunctionDefinition;
    funcOut = 'mc: {
        let __mc_input = funcIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::FunctionDefinition::FUNCTION_DEF { body: mut body } = __mc_input.clone() else { bail!("nomatch") };
            let mut params: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut crefs_new: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut params_new: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut repl: VarTransform::VariableReplacements;
            let mut body = body.clone();
            params = List::filterOnTrue(body.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isParameter, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
            let false = (params.clone().is_empty()) else { bail!("pattern mismatch") };
            crefs = List::map(params.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            crefs_new = List::map1r(crefs.clone(), (std::sync::Arc::new(ComponentReference::prependStringCref) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), (pathName.clone()).clone())?;
            params_new = List::map(crefs_new.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            repl = VarTransform::emptyReplacements();
            repl = VarTransform::addReplacementLst(repl.clone(), crefs.clone(), params_new.clone())?;
            (body, _) = DAEUtil::traverseDAEElementList(body.clone(), (std::sync::Arc::new(replaceParameters) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, VarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, VarTransform::VariableReplacements)> + 'static>), repl.clone())?;
            Ok(DAE::FunctionDefinition::FUNCTION_DEF { body: body.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(funcIn.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    funcOut
}

fn replaceParameters(mut inExp: Arc<DAE::Exp>, mut replIn: VarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = VarTransform::replaceExp(inExp, replIn, None)?;
    Ok((outExp, replOut))
}

