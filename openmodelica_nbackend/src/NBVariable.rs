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

use crate::NBAdjacency::Mapping;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::Iterator;
use crate::NBSlice as Slice;
use crate::NBackendDAE as BackendDAE;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_nf_frontend::NFAttributes as Attributes;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::OptimizerExpression;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::TearingSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltin;
use openmodelica_nf_frontend::NFCeval as Ceval;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

//OF Imports
//NF Imports
// Backend Imports
//Util Imports
// mainly used for mapping purposes
pub type VariablePointer = Pointer::Pointer<Arc<Variable::NFVariable>>;

pub type VarSlice = Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;

// ==========================================================================
//               Single Variable constants and functions
// ==========================================================================
thread_local! { static __DUMMY_VARIABLE_TLS: Arc<Variable::NFVariable> = Arc::new(Variable::NFVariable { name: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(), ty: openmodelica_nf_frontend::NFType::interned_ANY(), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() }); }
pub fn DUMMY_VARIABLE() -> Arc<Variable::NFVariable> { __DUMMY_VARIABLE_TLS.with(|__t| __t.clone()) }

thread_local! { static __SUBST_VARIABLE_TLS: Arc<Variable::NFVariable> = Arc::new(Variable::NFVariable { name: NFBuiltin::SUBST_CREF().clone(), ty: openmodelica_nf_frontend::NFType::interned_ANY(), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() }); }
pub(crate) fn SUBST_VARIABLE() -> Arc<Variable::NFVariable> { __SUBST_VARIABLE_TLS.with(|__t| __t.clone()) }

thread_local! { static __TIME_VARIABLE_TLS: Arc<Variable::NFVariable> = Arc::new(Variable::NFVariable { name: NFBuiltin::TIME_CREF().clone(), ty: openmodelica_nf_frontend::NFType::interned_REAL(), binding: Binding::EMPTY_BINDING().clone(), visibility: Prefixes::Visibility::PUBLIC.clone(), attributes: Attributes::DEFAULT_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: SCode::noComment.clone(), info: SCodeUtil::dummyInfo.clone(), backendinfo: Arc::new(BackendInfo::BackendInfo { varKind: openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_TIME(), attributes: BackendExtension::EMPTY_VAR_ATTR_REAL().clone(), annotations: BackendExtension::EMPTY_ANNOTATIONS.clone(), var_pre: None, var_seed: None, var_pder_res: None, var_pder_tmp: None, var_start: None, parent: None }) }); }
pub(crate) fn TIME_VARIABLE() -> Arc<Variable::NFVariable> { __TIME_VARIABLE_TLS.with(|__t| __t.clone()) }

pub(crate) const DERIVATIVE_STR: &'static str = "$DER";

pub(crate) const DUMMY_DERIVATIVE_STR: &'static str = "$dDER";

pub(crate) const PARTIAL_DERIVATIVE_STR: &'static str = "$pDER";

pub(crate) const FUNCTION_DERIVATIVE_STR: &'static str = "$fDER";

pub(crate) const FUNCTION_STR: &'static str = "$FUN";

pub(crate) const PREVIOUS_STR: &'static str = "$PRE";

pub(crate) const AUXILIARY_STR: &'static str = "$AUX";

pub(crate) const STATE_ALIAS_STR: &'static str = "$STA";

pub(crate) const START_STR: &'static str = "$START";

pub(crate) const RESIDUAL_STR: &'static str = "$RES";

pub(crate) const TEMPORARY_STR: &'static str = "$TMP";

pub(crate) const SEED_STR: &'static str = "$SEED";

pub(crate) const TIME_EVENT_STR: &'static str = "$TEV";

pub(crate) const STATE_EVENT_STR: &'static str = "$SEV";

pub(crate) const CLOCK_STR: &'static str = "$CLK";

pub(crate) fn toString(mut var: Arc<Variable::NFVariable>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut attr: ArcStr;
    attr = (BackendExtension::VariableAttributes::toString(var.backendinfo.attributes.clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*BackendExtension::VariableKind::toString(var.backendinfo.varKind.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(Variable::size(var.clone(), true)?)); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*Variable::toString(var, (literal!("")).clone(), false)?); __mm_s.push_str(&*if (attr.clone() == literal!("")) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*attr); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub(crate) fn pointerToString(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = toString(Pointer::access(var_ptr.clone()), (literal!("")).clone())?;
    Ok(r#str)
}

pub(crate) fn nameString(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = ComponentRef::toString(getVarName(var_ptr.clone()))?;
    Ok(r#str)
}

pub fn hash(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> {
    let mut i: i32 = Variable::hash(Pointer::access(var_ptr.clone()))?;
    Ok(i)
}

pub fn equalName(mut var_ptr1: Pointer::Pointer<Arc<Variable::NFVariable>>, mut var_ptr2: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool = Variable::equalName(Pointer::access(var_ptr1.clone()), Pointer::access(var_ptr2.clone()))?;
    Ok(b)
}

pub(crate) fn size(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut resize: bool) -> Result<i32> {
    let mut s: i32 = Variable::size(Pointer::access(var_ptr.clone()), resize)?;
    Ok(s)
}

pub(crate) fn applyToType(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Result<()> {
    pub type typeFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;

    let mut new: Arc<Variable::NFVariable>;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    new = Variable::applyToType(var.clone(), func.clone())?;
    if !(referenceEq(&*(var),&*(new.clone()))) {
        Pointer::update(var_ptr, new);
    }
    Ok(())
}

pub(crate) fn fromCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut attr: Arc<Attributes::NFAttributes>, mut binding: Arc<Binding::NFBinding>) -> Result<Arc<Variable::NFVariable>> {
    let mut variable: Arc<Variable::NFVariable>;
    let mut node: Arc<InstNode::InstNode>;
    let mut class_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut child_nodes: metamodelica::Array<Arc<InstNode::InstNode>> = Default::default();
    let mut ty: Arc<Type::NFType>;
    let mut vis: Prefixes::Visibility;
    let mut info: SourceInfo;
    let mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    node = ComponentRef::node(cref.clone())?;
    ty = ComponentRef::getSubscriptedType(cref.clone(), true)?;
    vis = InstNode::visibility(node.clone());
    info = InstNode::info(node);
    if !(Type::isExternalObject(ty.clone())) {
        children = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::COMPLEX { cls: __esc_class_node, .. } => {
            class_node = (*__esc_class_node).clone();
            child_nodes = Class::getComponents(InstNode::getClass(class_node.clone())?)?;
            children = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut c in (child_nodes.clone()).borrow().iter() {
            let __x = fromCref(ComponentRef::prefixCref(c.clone(), InstNode::getType(c.clone())?, metamodelica::nil(), cref.clone()), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            children
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    variable = Arc::new(Variable::NFVariable { name: cref, ty: ty, binding: binding, visibility: vis, attributes: attr, typeAttributes: metamodelica::nil(), children: children, comment: SCode::noComment.clone(), info: info, backendinfo: BackendExtension::DUMMY_BACKEND_INFO().clone() });
    Ok(variable)
}

pub(crate) fn makeVarPtrCyclic(mut var: Arc<Variable::NFVariable>, mut name: Arc<ComponentRef::NFComponentRef>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut name: Arc<ComponentRef::NFComponentRef> = name;
    var_ptr = Pointer::create(var.clone());
    name = BackendDAE::lowerComponentReferenceInstNode(name, var_ptr.clone())?;
    assign_field!(var.name = name.clone());
    Pointer::update(var_ptr.clone(), var);
    Ok((var_ptr, name))
}

pub(crate) fn connectPartners(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut par_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut par: Arc<Variable::NFVariable> = Pointer::access(par_ptr.clone());
    assign_field!(var.backendinfo = func(var.backendinfo.clone(), Some(par_ptr.clone()))?);
    assign_field!(par.backendinfo = func(par.backendinfo.clone(), Some(var_ptr.clone()))?);
    Pointer::update(var_ptr, var);
    Pointer::update(par_ptr, par);
    Ok(())
}

pub(crate) fn removePartner(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.backendinfo = func(var.backendinfo.clone(), None)?);
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn getVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(getVarPointer(cref, info)?);
    Ok(var)
}

// The following functions provide layers of protection. Whenever accessing names or pointers use these!
pub(crate) fn getVarPointer(mut cref: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    var = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { varPointer, .. }, .. } => {
            varPointer.clone()
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::NAME_NODE { .. }, .. } => {
            Pointer::create(DUMMY_VARIABLE().clone())
        },
        Deref @ ComponentRef::WILD => {
            Pointer::create(DUMMY_VARIABLE().clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getVarPointer")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); __mm_s.push_str(&*literal!(", because of wrong InstNode (not VAR_NODE). Show lowering errors with -d=failtrace.")); ArcStr::from(__mm_s) }).clone(), info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn getVarName(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<ComponentRef::NFComponentRef> {
    let mut name: Arc<ComponentRef::NFComponentRef>;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    name = var.name.clone();
    name
}

pub(crate) fn setVarName(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: Arc<ComponentRef::NFComponentRef>) -> Pointer::Pointer<Arc<Variable::NFVariable>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.name = name);
    Pointer::update(var_ptr.clone(), var);
    var_ptr
}

pub(crate) fn subIdxName(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut index: Pointer::Pointer<i32>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    assign_field!(var.name = ComponentRef::rename(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(var.name.clone(), false)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(Pointer::access(index))); ArcStr::from(__mm_s) }).clone(), var.name.clone())?);
    var_ptr = Pointer::create(var);
    Ok(var_ptr)
}

pub(crate) fn getVarKind(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<VariableKind::VariableKind> {
    let mut kind: Arc<VariableKind::VariableKind>;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    kind = BackendExtension::BackendInfo::getVarKind(var.backendinfo.clone());
    kind
}

pub(crate) fn toExpression(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Expression::fromCref(getVarName(var_ptr.clone()), false)?;
    Ok(exp)
}

pub type checkVar = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>;

pub(crate) fn isArray(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = Type::isArray(var.ty.clone());
    b
}

pub(crate) fn getDimensions(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(var_ptr);
    dims = Type::arrayDims(var.ty.clone());
    dims
}

pub(crate) fn isEmpty(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = ComponentRef::isEmpty(var.name.clone());
    b
}

pub(crate) fn isForcedState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { natural, .. } => {
            !(natural.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isStateDerivative(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE_DER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isAlgebraic(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::ALGEBRAIC => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isStart(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::START { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isExtObj(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::EXTOBJ { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isTime(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::TIME => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isContinuous(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut staticAsContinuous: bool) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DISCRETE_STATE => false,
        Deref @ BackendExtension::VariableKind::DISCRETE => false,
        Deref @ BackendExtension::VariableKind::PREVIOUS => false,
        Deref @ BackendExtension::VariableKind::CONSTANT => false,
        Deref @ BackendExtension::VariableKind::ITERATOR => false,
        Deref @ BackendExtension::VariableKind::EXTOBJ { .. } => false,
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => staticAsContinuous && Type::isContinuous(var.ty.clone())?,
        Deref @ BackendExtension::VariableKind::RECORD { .. } => List::all(getRecordChildren(var_ptr), (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| isContinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isDiscontinuous(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut staticAsContinuous: bool) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isContinuous(var_ptr, staticAsContinuous)?);
    Ok(b)
}

pub(crate) fn isContinuousRecordAware(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut staticAsContinuous: bool) -> Result<bool> {
    '__tco: loop {
        let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
        match getParent(var_ptr.clone()) {
        Some(mut parent) => {
            { (var_ptr, staticAsContinuous) = (parent.clone(), staticAsContinuous); continue '__tco; }
        },
        _ => {
            return Ok(isContinuous(var_ptr, staticAsContinuous)?)
        },
    }
    }
}

pub(crate) fn isDiscreteState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DISCRETE_STATE => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDiscrete(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DISCRETE => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isPrevious(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::PREVIOUS => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isRecord(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isKnownRecord(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RECORD { max_var: variability, .. } if (variability.clone() < Prefixes::Variability::DISCRETE.clone()) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isUnknownRecord(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RECORD { min_var: variability, .. } if (variability.clone() > Prefixes::Variability::NON_STRUCTURAL_PARAMETER.clone()) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClock(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CLOCK => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClocked(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CLOCKED => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClockOrClocked(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CLOCK => true,
        Deref @ BackendExtension::VariableKind::CLOCKED => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isIterator(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::ITERATOR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isPDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::JAC_VAR => true,
        Deref @ BackendExtension::VariableKind::JAC_TMP_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn hasTearingSelect(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut compareTS: TearingSelect, mut func: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>) -> Result<bool> {
    pub type compare = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

    let mut b: bool = func(((getTearingSelect(varPointer.clone())?) as i32), ((compareTS) as i32))?;
    Ok(b)
}

pub type getVarPartner = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>;

pub(crate) fn getVarPre(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("pre variable")).clone();
    partner = var.backendinfo.var_pre.clone();
    (partner, partnerName)
}

pub(crate) fn getVarSeed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("seed variable")).clone();
    partner = var.backendinfo.var_seed.clone();
    (partner, partnerName)
}

pub(crate) fn getVarPDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut isTmp: bool) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    if isTmp {
        partnerName = (literal!("partial derivative (temp)")).clone();
        partner = var.backendinfo.var_pder_tmp.clone();
    } else {
        partnerName = (literal!("partial derivative (result)")).clone();
        partner = var.backendinfo.var_pder_res.clone();
    }
    (partner, partnerName)
}

pub(crate) fn getVarDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>> = None;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("derivative")).clone();
    partner = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { derivative: __esc_partner, .. } => {
            partner = (*__esc_partner).clone();
            partner.clone()
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (partner, partnerName)
}

pub(crate) fn getVarState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("state")).clone();
    partner = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE_DER { state: p, .. } => {
            Some(p.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (partner, partnerName)
}

pub(crate) fn getVarDummyDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("dummy derivative")).clone();
    partner = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DUMMY_STATE { dummy_der: p } => {
            Some(p.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (partner, partnerName)
}

pub(crate) fn getVarStart(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) {
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    partnerName = (literal!("start")).clone();
    partner = var.backendinfo.var_start.clone();
    (partner, partnerName)
}

pub(crate) fn getPartnerCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>, mut scalarized: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut partner_cref: Arc<ComponentRef::NFComponentRef>;
    let mut partner: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut partnerName: ArcStr;
    (partner, partnerName) = func(getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?)?;
    if isSome(partner.clone()) {
        partner_cref = getVarName(Util::getOption(partner)?);
        if !(scalarized) {
            partner_cref = ComponentRef::copySubscripts(cref, partner_cref)?;
        }
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getPartnerCref")); __mm_s.push_str(&*literal!(" failed because ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); __mm_s.push_str(&*literal!(" has no corresponding ")); __mm_s.push_str(&*partnerName); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(partner_cref)
}

pub(crate) fn hasStartAttr(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = isSome(BackendExtension::VariableAttributes::getStartAttribute(var.backendinfo.attributes.clone()));
    b
}

pub(crate) fn hasPre(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isPrevious(var_ptr.clone())) && isSome((getVarPre(var_ptr)).0);
    b
}

pub(crate) fn isJacobianResultVar(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (match (getVarPDer(var_ptr.clone(), false)).0 {
        Some(mut der_var) => {
            isJacobianResultVarPDer(der_var.clone())
        },
        _ => {
            let mut der_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            (match (getVarPDer(var_ptr, true)).0 {
        Some(mut __esc_der_var) => {
            der_var = __esc_der_var.clone();
            isJacobianResultVarPDer(der_var.clone())
        },
        _ => false,
    })
        },
    });
    b
}

pub(crate) fn isJacobianResultVarPDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::JAC_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDummyState(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DUMMY_STATE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDummyDer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::DUMMY_DER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isParamOrConst(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => true,
        Deref @ BackendExtension::VariableKind::CONSTANT => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isConst(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::CONSTANT => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isKnown(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => true,
        Deref @ BackendExtension::VariableKind::CONSTANT => true,
        Deref @ BackendExtension::VariableKind::STATE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isOptimizable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { .. }, annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizable: true, .. }, .. } => true,
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizable: true, .. }, .. } if (isInput(var_ptr.clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isStateOrOptimizable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = isState(var_ptr.clone()) || isOptimizable(var_ptr);
    b
}

pub(crate) fn isInitialTime(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::INITIAL_TIME.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isFinalTime(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::FINAL_TIME.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isLagrange(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::LAGRANGE.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isMayer(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::MAYER.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isPathConstraint(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::PATH_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isFinalConstraint(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::FINAL_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isInitialConstraint(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::INITIAL_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isLfgFunction(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    if isStateDerivative(var_ptr) {
        b = true;
        return b.clone();
    }
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::LAGRANGE.clone() || optExp.clone() == OptimizerExpression::PATH_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isMrfFunction(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut optExp: OptimizerExpression = OptimizerExpression::MAYER;
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { optimizerExpression: Some(__esc_optExp), .. }, .. } => {
            optExp = (*__esc_optExp).clone();
            optExp.clone() == OptimizerExpression::MAYER.clone() || optExp.clone() == OptimizerExpression::FINAL_CONSTRAINT.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isLfgVariable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isFinalTime(var_ptr.clone()) || isInitialTime(var_ptr));
    b
}

pub(crate) fn isMrfVariable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isInitialTime(var_ptr));
    b
}

pub(crate) fn isR0Variable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(isFinalTime(var_ptr));
    b
}

pub(crate) fn isResizable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = List::any(Type::arrayDims(var.ty.clone()), (std::sync::Arc::new(fnptr!(Dimension::isResizable, Arc<Dimension::NFDimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>))?;
    Ok(b)
}

pub(crate) fn isResizableParameter(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { .. }, annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { resizable: true, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn updateResizableParameter(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut optimal_values: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut val: Option<Arc<Expression::NFExpression>> = UnorderedMap::get(var.name.clone(), optimal_values.clone())?;
    let () = (::match_deref::match_deref! { match &((val, var.backendinfo.clone())) {
        (Some(Deref @ Expression::INTEGER { value: i }), Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::PARAMETER { .. }, annotations: Deref @ BackendExtension::Annotations::ANNOTATIONS { resizable: true, .. }, .. }) => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::PARAMETER; resize_value = Some(i.clone()));
            setVarKind(var_ptr, varKind.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn getResizableValue(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> {
    let mut val: i32 = 0;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let _ = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::PARAMETER { resize_value: Some(__esc_val) }, .. } => {
            val = (*__esc_val).clone();
            val.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getResizableValue")); __mm_s.push_str(&*literal!(" failed because following variable is not a resizable parameter: ")); __mm_s.push_str(&*toString(var, (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(val)
}

pub(crate) fn isResidual(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::RESIDUAL_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isSeed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::SEED_VAR => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isInput(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = var.attributes.direction.clone() == Prefixes::Direction::INPUT.clone();
    b
}

pub(crate) fn isOutput(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = var.attributes.direction.clone() == Prefixes::Direction::OUTPUT.clone();
    b
}

pub(crate) fn isFixed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.attributes.clone()) {
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_REAL { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_INT { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_STRING { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())?
        },
        Deref @ BackendExtension::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(fixed), .. } => {
            Expression::isAllTrue(fixed.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isFixable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::STATE { .. } => !(isFixed(var_ptr)?),
        Deref @ BackendExtension::VariableKind::DISCRETE_STATE => !(isFixed(var_ptr.clone())?) || hasPre(var_ptr),
        Deref @ BackendExtension::VariableKind::PARAMETER { .. } => !(isFixed(var_ptr)?),
        Deref @ BackendExtension::VariableKind::PREVIOUS => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isStateSelect(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut stateSelect: StateSelect) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = BackendExtension::VariableAttributes::getStateSelect(var.backendinfo.attributes.clone()) == stateSelect;
    b
}

pub(crate) fn setVariableAttributes(mut var: Arc<Variable::NFVariable>, mut variableAttributes: Arc<VariableAttributes::VariableAttributes>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = variableAttributes);
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn setMin(mut var: Arc<Variable::NFVariable>, mut min_val: Option<Arc<Expression::NFExpression>>, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setMin(variableAttributes.clone(), min_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub(crate) fn setMax(mut var: Arc<Variable::NFVariable>, mut max_val: Option<Arc<Expression::NFExpression>>, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setMax(variableAttributes.clone(), max_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub(crate) fn setStartAttribute(mut var: Arc<Variable::NFVariable>, mut start_val: Arc<Expression::NFExpression>, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setStartAttribute(variableAttributes.clone(), start_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub(crate) fn setStateSelect(mut var: Arc<Variable::NFVariable>, mut stateSelect_val: StateSelect, mut overwrite: bool) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setStateSelect(variableAttributes.clone(), stateSelect_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub(crate) fn setTearingSelect(mut var: Arc<Variable::NFVariable>, mut tearingSelect_val: TearingSelect, mut overwrite: bool) -> Arc<Variable::NFVariable> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: backendinfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            let mut backendinfo = (*backendinfo).clone();
            assign_field!(backendinfo.attributes = BackendExtension::VariableAttributes::setTearingSelect(variableAttributes.clone(), tearingSelect_val, overwrite));
            assign_field!(var.backendinfo = backendinfo.clone());
            var
        },
        _ => {
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    var
}

pub(crate) fn getTearingSelect(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<TearingSelect> {
    let mut tearingSelect_val: TearingSelect;
    tearingSelect_val = (::match_deref::match_deref! { match &(Pointer::access(varPointer.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: variableAttributes, .. }, .. } => {
            BackendExtension::VariableAttributes::getTearingSelect(variableAttributes.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getTearingSelect")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*pointerToString(varPointer)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tearingSelect_val)
}

pub(crate) fn setVarKind(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut varKind: Arc<VariableKind::VariableKind>) -> () {
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind));
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn setParent(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Pointer::Pointer<Arc<Variable::NFVariable>> {
    let mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>> = varPointer;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setParent(var.backendinfo.clone(), parent));
    Pointer::update(varPointer.clone(), var);
    varPointer
}

pub(crate) fn getParent(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Option<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut parent: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(varPointer.clone());
    parent = var.backendinfo.parent.clone();
    parent
}

pub(crate) fn isDummyVariable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.backendinfo.varKind.clone()) {
        Deref @ BackendExtension::VariableKind::FRONTEND_DUMMY => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isArtificial(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith((ComponentRef::firstName(getVarName(var_ptr), false)?).clone(), (literal!("$")).clone());
    Ok(b)
}

pub(crate) fn isFunctionAlias(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith((ComponentRef::firstName(getVarName(var_ptr), false)?).clone(), (arcstr::literal!(FUNCTION_STR)).clone());
    Ok(b)
}

pub(crate) fn isClockAlias(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = StringUtil::startsWith((ComponentRef::firstName(getVarName(var_ptr), false)?).clone(), (arcstr::literal!(CLOCK_STR)).clone());
    Ok(b)
}

pub(crate) fn createTimeVar() -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var: Arc<Variable::NFVariable> = TIME_VARIABLE().clone();
    (var_ptr, _) = makeVarPtrCyclic(var.clone(), var.name.clone())?;
    Ok(var_ptr)
}

pub(crate) fn setStateDerivativeVar(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut derivative: Pointer::Pointer<Arc<Variable::NFVariable>>) -> () {
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::STATE { index: 1, derivative: Some(derivative), natural: true })));
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn makeAlgStateVar(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> () {
    let mut var: Arc<Variable::NFVariable>;
    if isAlgebraic(varPointer.clone()) {
        var = Pointer::access(varPointer.clone());
        assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_ALG_STATE()));
        Pointer::update(varPointer, var);
    }
    ()
}

pub(crate) fn makeDerVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut scalarized: bool) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut der_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut state_cref: Arc<ComponentRef::NFComponentRef> = if (scalarized) {cref.clone()} else {ComponentRef::stripSubscriptsAll(cref.clone())};
    let () = ({
        let mut dummy_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = Pointer::create(DUMMY_VARIABLE().clone());
        (::match_deref::match_deref! { match &(ComponentRef::node(state_cref.clone())?) {
        Deref @ InstNode::VAR_NODE { .. } => {
            let mut derNode: Arc<InstNode::InstNode>;
            let mut state: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut var: Arc<Variable::NFVariable>;
            state = getVarPointer(state_cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            derNode = Arc::new(InstNode::InstNode::VAR_NODE { name: (arcstr::literal!(DERIVATIVE_STR)).clone(), varPointer: dummy_ptr });
            der_cref = ComponentRef::append(state_cref.clone(), ComponentRef::fromNode(derNode, ComponentRef::scalarType(state_cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            var = fromCref(ComponentRef::stripSubscriptsAll(der_cref.clone()), Variable::attributes(Pointer::access(state.clone())), Binding::EMPTY_BINDING().clone())?;
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::STATE_DER { state: state, alias: None })));
            (var_ptr, der_cref) = makeVarPtrCyclic(var, der_cref)?;
            if !(scalarized) {
                der_cref = ComponentRef::copySubscripts(cref, der_cref)?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((der_cref, var_ptr))
}

pub(crate) fn hasDerVar(mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(Pointer::access(state_var)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: Deref @ BackendExtension::VariableKind::STATE { derivative: Some(_), .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn addRecordChild(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut child: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::cons(child, var_field!((*varKind).children, VariableKind::VariableKind::RECORD).clone()));
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.addRecordChild")); __mm_s.push_str(&*literal!(" failed adding ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(child))?); __mm_s.push_str(&*literal!(" as a child to ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(var_ptr.clone()))?); __mm_s.push_str(&*literal!(" because it is not a record.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn setRecordChildren(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = children);
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind.clone()));
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setRecordChildren")); __mm_s.push_str(&*literal!(" failed adding new children to ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(var_ptr.clone()))?); __mm_s.push_str(&*literal!(" because it is not a record.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn getRecordChildren(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    children = (::match_deref::match_deref! { match &(Pointer::access(var)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. }, .. }, .. } => {
            var_field!((**varKind).children, VariableKind::VariableKind::RECORD).clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    children
}

pub(crate) fn getRecordChildrenCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut arg_children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    subscripts = ComponentRef::subscriptsAllFlat(cref.clone())?;
    arg_children = getRecordChildren(getVarPointer(cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?);
    children = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut child in (arg_children).into_iter().cloned() {
            let __x = ComponentRef::mergeSubscripts(subscripts.clone(), getVarName(child.clone()), true, true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(children)
}

pub(crate) fn getRecordChildrenCrefOrSelf(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = getRecordChildrenCref(cref.clone())?;
    children = if (children.clone().is_empty()) {list![cref]} else {children};
    Ok(children)
}

pub(crate) fn makeDummyState(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut derivative: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = (::match_deref::match_deref! { match &(BackendExtension::BackendInfo::getVarKind(var.backendinfo.clone())) {
        Deref @ BackendExtension::VariableKind::STATE { derivative: Some(__esc_derivative), .. } => {
            derivative = (*__esc_derivative).clone();
            let mut der_var: Arc<Variable::NFVariable>;
            der_var = Pointer::access(derivative.clone());
            assign_field!(der_var.backendinfo = BackendExtension::BackendInfo::setVarKind(der_var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::DUMMY_DER { dummy_state: varPointer.clone() })));
            assign_field!(der_var.backendinfo = BackendExtension::BackendInfo::setStateSelect(der_var.backendinfo.clone(), BackendExtension::StateSelect::AVOID.clone(), false));
            Pointer::update(derivative.clone(), der_var);
            BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::DUMMY_STATE { dummy_der: derivative.clone() }))
        },
        Deref @ BackendExtension::VariableKind::DUMMY_STATE { dummy_der: __esc_derivative } => {
            derivative = (*__esc_derivative).clone();
            var.backendinfo.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeDummyState")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(getVarName(varPointer.clone()))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
    Pointer::update(varPointer, var);
    Ok(derivative)
}

pub(crate) fn makeDiscreteStateVar(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>) -> () {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(varPointer.clone());
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE_STATE()));
    Pointer::update(varPointer, var);
    ()
}

pub(crate) fn makePreVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut pre_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut pre_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut pre: Arc<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(PREVIOUS_STR));
            pre_cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            pre = fromCref(pre_cref.clone(), Variable::attributes(Pointer::access(var_ptr.clone())), Binding::EMPTY_BINDING().clone())?;
            assign_field!(pre.backendinfo = BackendExtension::BackendInfo::setVarKind(pre.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_PREVIOUS()));
            (pre_ptr, pre_cref) = makeVarPtrCyclic(pre, pre_cref)?;
            connectPartners(var_ptr, pre_ptr.clone(), (std::sync::Arc::new(fnptr!(BackendExtension::BackendInfo::setVarPre, Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>))?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makePreVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((pre_cref, pre_ptr))
}

pub(crate) fn makeSeedVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut name: ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut ovar: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
            let mut var: Arc<Variable::NFVariable>;
            let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            (ovar, _) = getVarSeed(old_var_ptr.clone());
            if isSome(ovar.clone()) {
                var_ptr = Util::getOption(ovar)?;
                cref = getVarName(var_ptr.clone());
            } else {
                assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(SEED_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) });
                cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
                var = fromCref(cref.clone(), Attributes::IMPL_DISCRETE_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
                varKind = (::match_deref::match_deref! { match &(getVarKind(old_var_ptr.clone())) {
        __esc_varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            varKind = (*__esc_varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::nil());
            varKind.clone()
        },
        _ => openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_SEED_VAR(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind));
                (var_ptr, cref) = makeVarPtrCyclic(var, cref)?;
                connectPartners(old_var_ptr, var_ptr.clone(), (std::sync::Arc::new(fnptr!(BackendExtension::BackendInfo::setVarSeed, Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>))?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeSeedVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, var_ptr))
}

pub(crate) fn makePDerVar(mut cref: Arc<ComponentRef::NFComponentRef>, mut name: ArcStr, mut isTmp: bool) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut res_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut ovar: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>;
            let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
            let mut var: Arc<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            res_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            (ovar, _) = getVarPDer(res_ptr.clone(), isTmp);
            if isSome(ovar.clone()) {
                var_ptr = Util::getOption(ovar)?;
                cref = getVarName(var_ptr.clone());
            } else {
                assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(PARTIAL_DERIVATIVE_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) });
                cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
                var = fromCref(cref.clone(), Variable::attributes(Pointer::access(res_ptr.clone())), Binding::EMPTY_BINDING().clone())?;
                varKind = (::match_deref::match_deref! { match &(getVarKind(res_ptr.clone())) {
        __esc_varKind @ Deref @ BackendExtension::VariableKind::RECORD { .. } => {
            varKind = (*__esc_varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = metamodelica::nil());
            varKind.clone()
        },
        _ => if (isTmp) {openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_JAC_TMP_VAR()} else {openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_JAC_VAR()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), varKind));
                (var_ptr, cref) = makeVarPtrCyclic(var, cref)?;
                connectPartners(res_ptr, var_ptr.clone(), (std::sync::Arc::new({ let __pe_b2 = isTmp; move |__pe_a0, __pe_a1| Ok(BackendExtension::BackendInfo::setVarPDer(__pe_a0, __pe_a1, __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>))?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makePDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, var_ptr))
}

pub(crate) fn makeFDerVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::EMPTY, .. } => (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::COMPONENT_NODE { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => InstNode::InstNode::COMPONENT_NODE; name = BackendUtil::makeFDerString((ComponentRef::toString(cref.clone())?).clone(), None)?);
            ComponentRef::fromNode(qual.clone(), ComponentRef::nodeType(cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())
        },
        qual @ Deref @ InstNode::CLASS_NODE { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => InstNode::InstNode::CLASS_NODE; name = BackendUtil::makeFDerString((ComponentRef::toString(cref.clone())?).clone(), None)?);
            ComponentRef::fromNode(qual.clone(), ComponentRef::nodeType(cref)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeFDerVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ ComponentRef::CREF { .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = makeFDerVar(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone())?);
            cref
        },
        _ => cref,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub(crate) fn makeStartVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> {
    let mut start_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    (start_cref, var_ptr) = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut old_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            (start_cref, var_ptr) = (match (getVarStart(old_var_ptr.clone())).0 {
        Some(mut __esc_var_ptr) => {
            var_ptr = __esc_var_ptr.clone();
            (getVarName(var_ptr.clone()), var_ptr.clone())
        },
        _ => {
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(START_STR));
            start_cref = ComponentRef::append(ComponentRef::stripSubscriptsAll(cref.clone()), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            var = fromCref(start_cref.clone(), Variable::attributes(getVar(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?), Binding::EMPTY_BINDING().clone())?;
            if isRecord(old_var_ptr.clone()) {
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: Prefixes::Variability::PARAMETER.clone(), max_var: Prefixes::Variability::CONTINUOUS.clone() })));
            } else {
                assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), Arc::new(VariableKind::VariableKind::START { original: old_var_ptr.clone() })));
            }
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarStart(var.backendinfo.clone(), Some(old_var_ptr.clone())));
            (var_ptr, start_cref) = makeVarPtrCyclic(var, start_cref)?;
            old_var = Pointer::access(old_var_ptr.clone());
            assign_field!(old_var.backendinfo = BackendExtension::BackendInfo::setVarStart(old_var.backendinfo.clone(), Some(var_ptr.clone())));
            Pointer::update(old_var_ptr, old_var);
            (start_cref, var_ptr.clone())
        },
    });
            start_cref = ComponentRef::copySubscripts(cref, start_cref)?;
            (start_cref, var_ptr)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeStartVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start_cref, var_ptr))
}

pub(crate) fn makeResidualVar(mut name: ArcStr, mut uniqueIndex: i32, mut ty: Arc<Type::NFType>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut node: Arc<InstNode::InstNode>;
    let mut var: Arc<Variable::NFVariable>;
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(RESIDUAL_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex)); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node, subscripts: metamodelica::nil(), ty: ty, origin: ComponentRef::Origin::CREF.clone(), restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY() });
    var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_RESIDUAL_VAR()));
    (var_ptr, cref) = makeVarPtrCyclic(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn makeEventVar(mut name: ArcStr, mut uniqueIndex: i32, mut var_ty: Arc<Type::NFType>, mut iterator: Arc<Iterator::Iterator>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut node: Arc<InstNode::InstNode>;
    let mut var_cref: Arc<ComponentRef::NFComponentRef>;
    let mut var: Arc<Variable::NFVariable>;
    let mut iter_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut ty: Arc<Type::NFType>;
    iter_subs = BEquation::Iterator::normalizedSubscripts(iterator.clone(), UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?;
    if iter_subs.clone().is_empty() {
        ty = var_ty;
    } else {
        ty = Type::liftArrayLeftList(var_ty, BEquation::Iterator::dimensions(iterator)?);
    }
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex)); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: iter_subs, ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY() });
    var_cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node, subscripts: metamodelica::nil(), ty: ty, origin: ComponentRef::Origin::CREF.clone(), restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY() });
    var = fromCref(var_cref, Attributes::IMPL_DISCRETE_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()));
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setHideResult(var.backendinfo.clone(), true));
    (var_ptr, cref) = makeVarPtrCyclic(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn makeAuxVar(mut name: ArcStr, mut uniqueIndex: i32, mut ty: Arc<Type::NFType>, mut makeParam: bool) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    fn updateBackendInfo(mut var: Arc<Variable::NFVariable>, mut makeParam: bool) -> Result<Arc<Variable::NFVariable>> {
        let mut var: Arc<Variable::NFVariable> = var;
        assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), BackendExtension::VariableKind::fromType(Variable::typeOf(var.clone()), makeParam)?));
        assign_field!(var.backendinfo = BackendExtension::BackendInfo::setHideResult(var.backendinfo.clone(), true));
        Ok(var)
    }

    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut node: Arc<InstNode::InstNode>;
    let mut var: Arc<Variable::NFVariable>;
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex.clone())); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY() });
    var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    var = updateBackendInfo(var, makeParam)?;
    assign_field!(var.children = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut child in (var.children.clone()).into_iter().cloned() {
            let __x = updateBackendInfo(child.clone(), makeParam)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    (var_ptr, cref) = makeVarPtrCyclic(var, cref.clone())?;
    Ok((var_ptr, cref))
}

pub(crate) fn makeAuxStateVar(mut uniqueIndex: i32, mut binding: Option<Arc<Expression::NFExpression>>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut der_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut der_cref: Arc<ComponentRef::NFComponentRef>;
    let mut node: Arc<InstNode::InstNode>;
    let mut var: Arc<Variable::NFVariable>;
    let mut bnd: Arc<Expression::NFExpression>;
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(AUXILIARY_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex)); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node, subscripts: metamodelica::nil(), ty: openmodelica_nf_frontend::NFType::interned_REAL(), origin: ComponentRef::Origin::CREF.clone(), restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY() });
    if isSome(binding.clone()) {
        bnd = Util::getOption(binding)?;
        var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::makeFlat(bnd.clone(), Expression::variability(bnd)?, Binding::Source::BINDING.clone(), Binding::NO_CONFIDENCE.clone()))?;
    } else {
        var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    }
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setStateSelect(var.backendinfo.clone(), BackendExtension::StateSelect::AVOID.clone(), false));
    (var_ptr, cref) = makeVarPtrCyclic(var, cref)?;
    (der_cref, der_var) = makeDerVar(cref.clone(), false)?;
    setStateDerivativeVar(var_ptr.clone(), der_var.clone());
    Ok((var_ptr, cref, der_var, der_cref))
}

pub(crate) fn makeTmpVar(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut tmp_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &(ComponentRef::node(cref.clone())?) {
        qual @ Deref @ InstNode::VAR_NODE { .. } => {
            let mut old_var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut var: Arc<Variable::NFVariable>;
            let mut qual = (*qual).clone();
            old_var_ptr = getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
            assign_variant_field!(qual => InstNode::InstNode::VAR_NODE; name = arcstr::literal!(TEMPORARY_STR));
            tmp_cref = ComponentRef::append(cref.clone(), ComponentRef::fromNode(qual.clone(), ComponentRef::scalarType(cref.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()))?;
            var = fromCref(tmp_cref.clone(), Variable::attributes(getVar(cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?), Binding::EMPTY_BINDING().clone())?;
            assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), getVarKind(old_var_ptr)));
            (var_ptr, tmp_cref) = makeVarPtrCyclic(var, tmp_cref)?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.makeTmpVar")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tmp_cref)
}

pub(crate) fn makeClockVar(mut uniqueIndex: i32, mut ty: Arc<Type::NFType>) -> Result<(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<ComponentRef::NFComponentRef>)> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut node: Arc<InstNode::InstNode>;
    let mut var: Arc<Variable::NFVariable>;
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(CLOCK_STR)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(uniqueIndex)); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(DUMMY_VARIABLE().clone()) });
    cref = Arc::new(ComponentRef::NFComponentRef::CREF { node: node, subscripts: metamodelica::nil(), ty: ty, origin: ComponentRef::Origin::CREF.clone(), restCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY() });
    var = fromCref(cref.clone(), Attributes::DEFAULT_ATTR().clone(), Binding::EMPTY_BINDING().clone())?;
    assign_field!(var.backendinfo = BackendExtension::BackendInfo::setVarKind(var.backendinfo.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCK()));
    (var_ptr, cref) = makeVarPtrCyclic(var, cref)?;
    Ok((var_ptr, cref))
}

pub(crate) fn getBindingVariability(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Prefixes::Variability> {
    let mut variability: Prefixes::Variability;
    variability = (::match_deref::match_deref! { match &(Pointer::access(var_ptr)) {
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::TYPED_BINDING { variability: tmp, .. }, .. } => {
            tmp.clone()
        },
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::FLAT_BINDING { variability: tmp, .. }, .. } => {
            tmp.clone()
        },
        Deref @ Variable::VARIABLE { binding: Deref @ Binding::UNBOUND, .. } => {
            Prefixes::Variability::CONTINUOUS.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.getBindingVariability")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(variability)
}

pub(crate) fn hasEvaluableBinding(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    fn isEvaluable(mut exp: Arc<Expression::NFExpression>) -> Result<bool> {
        let mut b: bool;
        let mut new_exp: Arc<Expression::NFExpression>;
        b = Expression::isLiteralXML(exp.clone())?;
        if !(b) {
            (_, new_exp) = BEquation::Iterator::extract(exp, UnorderedSet::new((std::sync::Arc::new(hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13), UnorderedMap::new((std::sync::Arc::new(Dimension::hashList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<i32> + 'static>), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(Dimension::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, Arc<Dimension::NFDimension>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>), 1))?;
            new_exp = SimplifyExp::simplifyDump(new_exp, true, literal!("NBVariable.hasEvaluableBinding.isEvaluable"), (literal!("")).clone())?;
            b = Expression::isLiteralXML(Ceval::tryEvalExp(new_exp, Ceval::noTarget().clone()))?;
        }
        Ok(b)
    }

    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut binding: Arc<Expression::NFExpression>;
    if isBound(var_ptr.clone()) {
        binding = Binding::getExp(var.binding.clone())?;
        b = isEvaluable(binding.clone())?;
    } else {
        b = false;
    }
    Ok(b)
}

pub(crate) fn mapExp(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut opt_start: Option<Arc<Expression::NFExpression>>;
    let mut binding: Arc<Expression::NFExpression>;
    let mut new_binding: Arc<Expression::NFExpression>;
    let mut start: Arc<Expression::NFExpression>;
    let mut new_start: Arc<Expression::NFExpression>;
    let mut changed: bool = false;
    if isBound(var_ptr.clone()) {
        binding = Binding::getExp(var.binding.clone())?;
        new_binding = mapFunc(binding.clone(), funcExp.clone())?;
        if !(referenceEq(&*(binding),&*(new_binding.clone()))) {
            assign_field!(var.binding = Binding::setExp(new_binding, var.binding.clone())?);
            changed = true;
        }
    }
    opt_start = getStartAttribute(var_ptr.clone());
    if isSome(opt_start.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(opt_start) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        start = __pa0.clone();
        new_start = mapFunc(start.clone(), funcExp.clone())?;
        if !(referenceEq(&*(start),&*(new_start.clone()))) {
            var = setStartAttribute(var, new_start, true)?;
            changed = true;
        }
    }
    if changed {
        Pointer::update(var_ptr, var);
    }
    Ok(())
}

pub(crate) fn setFixed(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut b: bool, mut overwrite: bool) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { .. }, .. } => {
            let mut binfo = (*binfo).clone();
            assign_field!(binfo.attributes = BackendExtension::VariableAttributes::setFixed(binfo.attributes.clone(), var.ty.clone(), b, overwrite)?);
            assign_field!(var.backendinfo = binfo.clone());
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setFixed")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var);
    Ok(var_ptr)
}

pub(crate) fn setBindingAsStart(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut overwrite: bool) -> Result<()> {
    let mut var: Arc<Variable::NFVariable>;
    var = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendExtension::BackendInfo::BACKEND_INFO { .. }, .. } => {
            let mut start: Arc<Expression::NFExpression>;
            let mut binfo = (*binfo).clone();
            start = Binding::getExp(var.binding.clone())?;
            assign_field!(binfo.attributes = BackendExtension::VariableAttributes::setStartAttribute(binfo.attributes.clone(), start, overwrite));
            assign_field!(var.backendinfo = binfo.clone());
            var
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.setBindingAsStart")); __mm_s.push_str(&*literal!(" failed because of wrong binding.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn setBindingAsStartAndFix(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut b: bool, mut overwrite: bool) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    setBindingAsStart(var_ptr.clone(), overwrite)?;
    var_ptr = setFixed(var_ptr, b, false)?;
    Ok(var_ptr)
}

pub(crate) fn getStartAttribute(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Option<Arc<Expression::NFExpression>> {
    let mut start: Option<Arc<Expression::NFExpression>> = BackendExtension::VariableAttributes::getStartAttribute(Variable::getVariableAttributes(Pointer::access(var_ptr.clone())));
    start
}

pub(crate) fn hasNonTrivialAliasBinding(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut binding: Arc<Expression::NFExpression> = Binding::getExp(var.binding.clone())?;
    b = !(Expression::isTrivialCref(binding.clone())) && checkExpMap(binding, (std::sync::Arc::new(fnptr!(isTimeDependent, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?;
    Ok(b)
}

pub(crate) fn hasConstOrParamAliasBinding(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = !(checkExpMap(Binding::getExp(var.binding.clone())?, (std::sync::Arc::new(fnptr!(isTimeDependent, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Classes/NBVariable.mo"))?);
    Ok(b)
}

pub(crate) fn isTimeDependent(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = BackendExtension::VariableKind::isTimeDependent(var.backendinfo.varKind.clone());
    b
}

pub(crate) fn isBound(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = (::match_deref::match_deref! { match &(var.binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => true,
        Deref @ Binding::UNTYPED_BINDING { .. } => true,
        Deref @ Binding::FLAT_BINDING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// ==========================================================================
//                        Other type wrappers
//
// ==========================================================================
pub(crate) fn checkExp(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::CREF { cref, .. } => {
            func(getVarPointer(cref.clone(), info)?)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn checkExpMap(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo) -> Result<bool> {
    pub(crate) fn checkExpTraverse(mut exp: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo, mut b: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut b: bool = b;
        if !(b) {
            b = checkExp(exp.clone(), func.clone(), info)?;
        }
        Ok((exp, b))
    }

    let mut b: bool;
    (_, b) = Expression::mapFold(exp, (std::sync::Arc::new({ let __pe_b1 = func.clone(); let __pe_b2 = info; move |__pe_a0, __pe_a3| checkExpTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?;
    Ok(b)
}

pub(crate) fn checkCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut info: SourceInfo) -> Result<bool> {
    let mut b: bool = func(getVarPointer(cref.clone(), info.clone())?)?;
    Ok(b)
}

// ==========================================================================
//                        Variable Array Stuff
//    All variable arrays are pointer arrays to avoid duplicates
// ==========================================================================
pub mod VariablePointers {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct VariablePointers {
        /// Map for cref->index
        pub map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>,
        /// Array of variable pointers
        pub varArr: Arc<ExpandableArray::ExpandableArray<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
        /// true if the variables are scalarized
        pub scalarized: bool,
    }

    impl metamodelica::gc::MMTrace for VariablePointers {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.varArr, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.scalarized, __mmv)?;
            Ok(())
        }
    }
    impl Default for VariablePointers {
        fn default() -> Self {
            Self {
                map: Default::default(),
                varArr: Default::default(),
                scalarized: Default::default(),
            }
        }
    }

    pub type VARIABLE_POINTERS = VariablePointers;

    pub(crate) fn toString(mut variables: Arc<VariablePointers>, mut r#str: ArcStr, mut mapping_opt: Option<metamodelica::Array<(i32, i32)>>, mut printEmpty: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut numberOfElements: i32 = size(variables.clone());
        let mut length: i32;
        let mut scal_start: i32;
        let mut index: ArcStr;
        let mut useMapping: bool = isSome(mapping_opt.clone());
        let mut mapping: metamodelica::Array<(i32, i32)> = Default::default();
        if useMapping {
            length = 15;
            mapping = Util::getOption(mapping_opt)?;
        } else {
            length = 10;
        }
        if printEmpty || numberOfElements > 0 {
            r#str = (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" Variables (")); __mm_s.push_str(&*intString(numberOfElements)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(scalarSize(variables.clone(), true)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone();
            for mut i in 1..=numberOfElements {
                if useMapping {
                    (scal_start, _) = ({let __elt = mapping.borrow()[(i.clone()-1) as usize].clone(); __elt});
                    index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*intString(scal_start)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                } else {
                    index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                }
                index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*index.clone()); __mm_s.push_str(&*StringUtil::repeat((literal!(" ")).clone(), length - ((index.clone()).clone().len() as i32))?); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*super::toString(Pointer::access(ExpandableArray::get(i.clone(), variables.varArr.clone())?), (index.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = (literal!("")).clone();
        }
        Ok(r#str)
    }

    pub(crate) fn map(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> + 'static>) -> Result<Arc<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> + 'static>;

        let mut variables: Arc<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut var: Arc<Variable::NFVariable>;
        let mut new_var: Arc<Variable::NFVariable>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i.clone(), variables.varArr.clone())?;
                var = Pointer::access(var_ptr.clone());
                new_var = func(var.clone())?;
                if !(referenceEq(&*(var.clone()),&*(new_var.clone()))) {
                    Pointer::update(var_ptr.clone(), new_var.clone());
                }
            }
        }
        Ok(variables)
    }

    pub(crate) fn mapPtr(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>) -> Result<Arc<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>;

        let mut variables: Arc<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i.clone(), variables.varArr.clone())?;
                func(var_ptr.clone())?;
            }
        }
        Ok(variables)
    }

    pub(crate) fn mapRemovePtr(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>) -> Result<Arc<VariablePointers>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>;

        let mut variables: Arc<VariablePointers> = variables;
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(variables.varArr.clone()) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                var_ptr = ExpandableArray::get(i.clone(), variables.varArr.clone())?;
                if func(var_ptr.clone())? {
                    variables = remove(var_ptr.clone(), variables.clone())?;
                }
            }
        }
        variables = compress(variables)?;
        Ok(variables)
    }

    pub(crate) fn empty(mut size: i32, mut scalarized: bool) -> Arc<VariablePointers> {
        let mut variables: Arc<VariablePointers>;
        let mut arr_size: i32;
        let mut bucketSize: i32;
        let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        arr_size = std::cmp::max(size, BaseHashTable::lowBucketSize.clone());
        bucketSize = Util::nextPrime(arr_size);
        if scalarized {
            map = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), bucketSize);
        } else {
            map = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hashStrip) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqualStrip) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), bucketSize);
        }
        variables = Arc::new(VariablePointers { map: map, varArr: ExpandableArray::new(arr_size, Pointer::create(DUMMY_VARIABLE().clone())), scalarized: scalarized });
        variables
    }

    pub(crate) fn clone(mut variables: Arc<VariablePointers>, mut shallow: bool) -> Result<Arc<VariablePointers>> {
        let mut new: Arc<VariablePointers>;
        if shallow {
            new = fromList(toList(variables)?, false)?;
        } else {
            new = fromList(({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (toList(variables)?).into_iter().cloned() {
            let __x = Pointer::create(Pointer::access(eqn.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
        }
        Ok(new)
    }

    pub(crate) fn size(mut variables: Arc<VariablePointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getNumberOfElements(variables.varArr.clone());
        sz
    }

    pub(crate) fn scalarSize(mut variables: Arc<VariablePointers>, mut resize: bool) -> Result<i32> {
        let mut sz: i32 = 0;
        for mut var_ptr in &*toList(variables)? {
            let mut var_ptr = var_ptr.clone();
            sz = sz + super::size(var_ptr.clone(), resize)?;
        }
        Ok(sz)
    }

    pub(crate) fn toList(mut variables: Arc<VariablePointers>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
        let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
        var_lst = ExpandableArray::toList(variables.varArr.clone())?;
        Ok(var_lst)
    }

    pub(crate) fn fromList(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut scalarized: bool) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers>;
        variables = empty((var_lst.clone().len() as i32), scalarized);
        variables = addList(var_lst, variables)?;
        Ok(variables)
    }

    pub(crate) fn addList(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        variables = List::fold(var_lst, (std::sync::Arc::new(move |__pe_a0, __pe_a1| add(__pe_a0, __pe_a1)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<VariablePointers>) -> Result<Arc<VariablePointers>> + 'static>), variables)?;
        Ok(variables)
    }

    pub(crate) fn removeList(mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        variables = List::fold(var_lst, (std::sync::Arc::new(move |__pe_a0, __pe_a1| remove(__pe_a0, __pe_a1)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<VariablePointers>) -> Result<Arc<VariablePointers>> + 'static>), variables)?;
        variables = compress(variables)?;
        Ok(variables)
    }

    pub(crate) fn removeCheck(mut variables: Arc<VariablePointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
        vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (toList(variables)?).into_iter().cloned() {
            if !(!(func(var.clone())?)) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        variables = fromList(vars, false)?;
        Ok(variables)
    }

    pub(crate) fn add(mut varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut var: Arc<Variable::NFVariable>;
        let mut index: i32 = 0;
        var = Pointer::access(varPointer.clone());
        let () = (match UnorderedMap::get(var.name.clone(), variables.map.clone())? {
        Some(mut index) if (index > 0) => {
            ExpandableArray::update(index, varPointer, variables.varArr.clone())?;
            ()
        },
        _ => {
            (_, index) = ExpandableArray::add(varPointer, variables.varArr.clone())?;
            UnorderedMap::add(var.name.clone(), index, variables.map.clone())?;
            ()
        },
    });
        Ok(variables)
    }

    pub(crate) fn remove(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut var: Arc<Variable::NFVariable>;
        let mut index: i32 = 0;
        var = Pointer::access(var_ptr);
        let () = (match UnorderedMap::get(var.name.clone(), variables.map.clone())? {
        Some(mut index) if (index > 0) => {
            ExpandableArray::delete(index, variables.varArr.clone())?;
            UnorderedMap::add(var.name.clone(), -1, variables.map.clone())?;
            ()
        },
        _ => (),
    });
        Ok(variables)
    }

    pub(crate) fn setVarAt(mut variables: Arc<VariablePointers>, mut idx: i32, mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> {
        let mut var: Arc<Variable::NFVariable>;
        ExpandableArray::set(idx, var_ptr.clone(), variables.varArr.clone())?;
        var = Pointer::access(var_ptr);
        UnorderedMap::add(var.name.clone(), idx, variables.map.clone())?;
        Ok(())
    }

    pub(crate) fn getVarAt(mut variables: Arc<VariablePointers>, mut idx: i32) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
        let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        var = ExpandableArray::get(idx, variables.varArr.clone())?;
        Ok(var)
    }

    pub(crate) fn getVarSafe(mut variables: Arc<VariablePointers>, mut cref: Arc<ComponentRef::NFComponentRef>, mut info: Option<SourceInfo>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut index: i32 = 0;
        var_ptr = (match UnorderedMap::get(cref.clone(), variables.map.clone())? {
        Some(mut index) if (index > 0) => ExpandableArray::get(index, variables.varArr.clone())?,
        _ => {
            if isSome(info.clone()) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VariablePointers.getVarSafe")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }).clone(), Util::getOption(info)?)?;
            }
            bail!("fail")
        },
    });
        Ok(var_ptr)
    }

    pub(crate) fn getVarIndex(mut variables: Arc<VariablePointers>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<i32> {
        let mut index: i32 = UnorderedMap::getOrDefault(cref.clone(), variables.map.clone(), -1)?;
        Ok(index)
    }

    pub(crate) fn contains(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers>) -> Result<bool> {
        let mut b: bool = containsCref(getVarName(var.clone()), variables.clone())?;
        Ok(b)
    }

    pub(crate) fn containsCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut variables: Arc<VariablePointers>) -> Result<bool> {
        let mut b: bool = getVarIndex(variables.clone(), cref.clone())? > 0;
        Ok(b)
    }

    pub(crate) fn getVarNames(mut variables: Arc<VariablePointers>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> = Pointer::create(metamodelica::nil());
        mapPtr(variables, (std::sync::Arc::new({ let __pe_b1 = acc.clone(); move |__pe_a0| Ok(getVarNameTraverse(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>))?;
        names = Pointer::access(acc).reverse();
        Ok(names)
    }

    pub(crate) fn getScalarVarNames(mut variables: Arc<VariablePointers>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut var: Arc<Variable::NFVariable>;
        for mut var_ptr in &*toList(variables)? {
            let mut var_ptr = var_ptr.clone();
            var = Pointer::access(var_ptr.clone());
            if Type::isArray(var.ty.clone()) {
                for mut cr in &*ComponentRef::scalarizeAll(ComponentRef::stripSubscriptsAll(var.name.clone()), resize)? {
                    let mut cr = cr.clone();
                    if Type::isComplex(ComponentRef::nodeType(cr.clone())?) {
                        names = listAppend(ComponentRef::getRecordChildren(cr.clone())?, names.clone());
                    } else {
                        names = metamodelica::cons(cr.clone(), names.clone());
                    }
                }
            } else {
                names = metamodelica::cons(var.name.clone(), names.clone());
            }
        }
        Ok(names)
    }

    pub(crate) fn getMarkedVars(mut variables: Arc<VariablePointers>, mut marks: metamodelica::Array<bool>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
        let mut marked_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
        let mut indices: Arc<metamodelica::List<i32>> = BackendUtil::findTrueIndices(marks.clone());
        if metamodelica::arrayLength(marks.clone()) == size(variables.clone()) {
            marked_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut index in (indices).into_iter().cloned() {
            let __x = getVarAt(variables.clone(), index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VariablePointers.getMarkedVars")); __mm_s.push_str(&*literal!(" failed because the number var marks (")); __mm_s.push_str(&*intString(metamodelica::arrayLength(marks.clone()))); __mm_s.push_str(&*literal!(") is not equal to the number of variables (")); __mm_s.push_str(&*intString(size(variables))); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(marked_vars)
    }

    pub(crate) fn compress(mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut i in ({let __s=ExpandableArray::getLastUsedIndex(variables.varArr.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            if ExpandableArray::occupied(i.clone(), variables.varArr.clone()) {
                vars = metamodelica::cons(ExpandableArray::get(i.clone(), variables.varArr.clone())?, vars.clone());
            }
        }
        variables = fromList(vars, false)?;
        Ok(variables)
    }

    pub(crate) fn sort(mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut size: i32;
        let mut hash_lst: Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>>;
        let mut hash_lst_ptr: Pointer::Pointer<Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>>> = Pointer::create(metamodelica::nil());
        let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
        size = ExpandableArray::getNumberOfElements(variables.varArr.clone());
        mapPtr(variables.clone(), (std::sync::Arc::new({ let __pe_b1 = ((metamodelica::OrderedFloat((size) as f64) * (metamodelica::OrderedFloat((size) as f64)).ln()).0.floor() as i32); let __pe_b2 = hash_lst_ptr.clone(); move |__pe_a0| createSortHashTpl(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>))?;
        hash_lst = List::sort(Pointer::access(hash_lst_ptr), std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)))?;
        variables = empty(size, variables.scalarized.clone());
        for mut tpl in &*hash_lst {
            let mut tpl = tpl.clone();
            (_, var_ptr) = tpl.clone();
            variables = add(var_ptr.clone(), variables.clone())?;
        }
        Ok(variables)
    }

    pub(crate) fn scalarize(mut variables: Arc<VariablePointers>) -> Result<Arc<VariablePointers>> {
        let mut variables: Arc<VariablePointers> = variables;
        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
        let mut flattened: bool;
        (vars, flattened) = scalarizeList(toList(variables.clone())?)?;
        if flattened {
            variables = fromList(vars, true)?;
        }
        Ok(variables)
    }

    pub(crate) fn scalarizeList(mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, bool)> {
        let mut new_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut flattened: bool = false;
        let mut scalar_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
        let mut element_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
        let mut var: Arc<Variable::NFVariable>;
        for mut var_ptr in &*vars {
            let mut var_ptr = var_ptr.clone();
            var = Pointer::access(var_ptr.clone());
            if Type::isArray(var.ty.clone()) {
                flattened = true;
                scalar_vars = Scalarize::scalarizeBackendVariable(var.clone(), metamodelica::nil())?;
            } else {
                scalar_vars = list![Pointer::access(var_ptr.clone())];
            }
            for mut var in &*scalar_vars.clone() {
                let mut var = var.clone();
                if Type::isComplex(var.ty.clone()) {
                    flattened = true;
                    element_vars = Scalarize::scalarizeComplexVariable(var.clone(), metamodelica::nil())?;
                    for mut elem_var in &*element_vars.clone().reverse() {
                        let mut elem_var = elem_var.clone();
                        new_vars = metamodelica::cons(Pointer::create(elem_var.clone()), new_vars.clone());
                    }
                } else {
                    new_vars = metamodelica::cons(Pointer::create(var.clone()), new_vars.clone());
                }
            }
        }
        new_vars = new_vars.reverse();
        Ok((new_vars, flattened))
    }

    pub(crate) fn varSlice(mut vars: Arc<VariablePointers>, mut scal: i32, mut arr: i32, mut mapping: Arc<Mapping::Mapping>, mut resize: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut cref: Arc<ComponentRef::NFComponentRef>;
        let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut start: i32;
        let mut ty: Arc<Type::NFType>;
        let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
        let mut sizes: Arc<metamodelica::List<i32>>;
        let mut vals: Arc<metamodelica::List<i32>>;
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
        (start, _) = ({let __elt = mapping.var_AtS.borrow()[(arr-1) as usize].clone(); __elt});
        var = getVarAt(vars, arr)?;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Pointer::access(var)) {
            Deref @ Variable::VARIABLE { name: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cref = __pa0.clone();
        ty = __pa1.clone();
        dims = Type::arrayDims(ty);
        sizes = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), resize)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        vals = Slice::indexToLocation(scal - start, sizes).reverse();
        subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let __thr_src0 = dims;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = vals;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(dim), Some(val)) => {
                    let __x = Subscript::nth(dim.clone(), val.clone() + 1)?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
        cref = ComponentRef::mergeSubscripts(subs, cref, true, true, false)?;
        Ok(cref)
    }

    fn createSortHashTpl(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut r#mod: i32, mut hash_lst_ptr: Pointer::Pointer<Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>>>) -> Result<()> {
        let mut var: Arc<Variable::NFVariable>;
        let mut hash: i32;
        var = Pointer::access(var_ptr.clone());
        hash = stringHashDjb2Mod((BackendExtension::BackendInfo::toString(var.backendinfo.clone())?).clone(), r#mod);
        Pointer::update(hash_lst_ptr.clone(), metamodelica::cons((hash, var_ptr), Pointer::access(hash_lst_ptr)));
        Ok(())
    }

}

// ==========================================================================
//                        Variable Data
//    All variable arrays are pointer arrays to avoid duplicates
// ==========================================================================
pub mod VarData {
    use super::*;
    /// All variable arrays are pointer subsets of an array of variables indicated
    ///    by preceding comment. Used to traverse all variables of a special kind.
    #[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum VarData {
        /// Only to be used for simulation systems.
        VAR_DATA_SIM {
            /// use when trying to create unique variables
            uniqueIndex: Pointer::Pointer<i32>,
            /// All variables
            variables: Arc<VariablePointers::VariablePointers>,
            /// All state derivatives, algebraic variables,
            ///                                          discrete variables
            unknowns: Arc<VariablePointers::VariablePointers>,
            /// Parameters, constants, states
            knowns: Arc<VariablePointers::VariablePointers>,
            /// All initial unknowns (unknowns + states + previous + parameters(non const binding))
            initials: Arc<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal with 1 or -1 coefficient
            aliasVars: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal with gain * alias + offset function
            nonTrivialAlias: Arc<VariablePointers::VariablePointers>,
            /// State derivatives (der(x) -> $DER.x)
            derivatives: Arc<VariablePointers::VariablePointers>,
            /// Algebraic variables
            algebraics: Arc<VariablePointers::VariablePointers>,
            /// Discrete variables
            discretes: Arc<VariablePointers::VariablePointers>,
            /// Discrete state variables
            discrete_states: Arc<VariablePointers::VariablePointers>,
            /// Clocked state variables
            clocked_states: Arc<VariablePointers::VariablePointers>,
            /// Previous variables (pre(d) -> $PRE.d)
            previous: Arc<VariablePointers::VariablePointers>,
            /// clock variables
            clocks: Arc<VariablePointers::VariablePointers>,
            /// States
            states: Arc<VariablePointers::VariablePointers>,
            /// Top level inputs
            top_level_inputs: Arc<VariablePointers::VariablePointers>,
            /// Resizable Parameters
            resizables: Arc<VariablePointers::VariablePointers>,
            /// Parameters
            parameters: Arc<VariablePointers::VariablePointers>,
            /// Constants
            constants: Arc<VariablePointers::VariablePointers>,
            /// Records
            records: Arc<VariablePointers::VariablePointers>,
            /// External Objects
            external_objects: Arc<VariablePointers::VariablePointers>,
            /// artificial variables to have pointers on crefs
            artificials: Arc<VariablePointers::VariablePointers>,
            state_order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>,
        },
        /// Only to be used for Jacobians.
        VAR_DATA_JAC {
            /// All jacobian variables
            variables: Arc<VariablePointers::VariablePointers>,
            /// All result and temporary vars
            unknowns: Arc<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal
            aliasVars: Arc<VariablePointers::VariablePointers>,
            /// Differentiation variables z where J = dF/dz
            diffVars: Arc<VariablePointers::VariablePointers>,
            /// All occurring unknowns for linearity analysis
            dependencies: Arc<VariablePointers::VariablePointers>,
            /// Result variable depending on current seed
            ///                                          ($RES.[jacname].[eq_idx])
            resultVars: Arc<VariablePointers::VariablePointers>,
            /// Temporary variables (inner partial derivatives)
            ///                                          dy/dz with y!=z for all y and z
            ///                                          ($TMP.[jacname].y)
            tmpVars: Arc<VariablePointers::VariablePointers>,
            /// Seed variables representing a generic derivative
            ///                                          dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED.[jacname].x)
            seedVars: Arc<VariablePointers::VariablePointers>,
        },
        /// Only to be used for Hessians.
        VAR_DATA_HES {
            /// All hessian variables
            variables: Arc<VariablePointers::VariablePointers>,
            /// All state derivatives, algebraic variables,
            ///                                          discrete variables
            unknowns: Arc<VariablePointers::VariablePointers>,
            /// Parameters, constants
            knowns: Arc<VariablePointers::VariablePointers>,
            /// Variables created by the backend known to be solved
            ///                                          by given binding. E.g. $cse
            auxiliaries: Arc<VariablePointers::VariablePointers>,
            /// Variables removed due to alias removal
            aliasVars: Arc<VariablePointers::VariablePointers>,
            /// Differentiation variables z where J = dF/dz
            diffVars: Arc<VariablePointers::VariablePointers>,
            /// All occurring unknowns for linearity analysis
            dependencies: Arc<VariablePointers::VariablePointers>,
            /// Result variable depending on current seed
            ///                                          ($RES.[jacname].[eq_idx])
            resultVars: Arc<VariablePointers::VariablePointers>,
            /// Temporary variables (inner partial derivatives)
            ///                                          dy/dz with y!=z for all y and z
            ///                                          ($TMP.[jacname].y)
            tmpVars: Arc<VariablePointers::VariablePointers>,
            /// Seed variables representing a generic derivative
            ///                                          dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED.[jacname].x)
            seedVars: Arc<VariablePointers::VariablePointers>,
            /// Second seed variables representing a generic
            ///                                          derivative dx/dz which is 1 for x==z and 0 otherwise.
            ///                                          ($SEED2.[jacname].x)
            seedVars2: Arc<VariablePointers::VariablePointers>,
            /// Lambda variables for optimization
            lambdaVars: Option<Arc<VariablePointers::VariablePointers>>,
        },
        VAR_DATA_EMPTY,
    }
    impl metamodelica::gc::MMTrace for VarData {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                VarData::VAR_DATA_SIM { uniqueIndex, variables, unknowns, knowns, initials, auxiliaries, aliasVars, nonTrivialAlias, derivatives, algebraics, discretes, discrete_states, clocked_states, previous, clocks, states, top_level_inputs, resizables, parameters, constants, records, external_objects, artificials, state_order } => {
                    metamodelica::gc::MMTrace::mm_accept(uniqueIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unknowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(knowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(initials, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(nonTrivialAlias, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(derivatives, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(algebraics, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discretes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discrete_states, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(clocked_states, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(previous, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(clocks, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(states, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(top_level_inputs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(resizables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(parameters, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(constants, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(records, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(external_objects, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(artificials, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(state_order, __mmv)?;
                    Ok(())
                }
                VarData::VAR_DATA_JAC { variables, unknowns, auxiliaries, aliasVars, diffVars, dependencies, resultVars, tmpVars, seedVars } => {
                    metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unknowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(diffVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dependencies, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(resultVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(tmpVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(seedVars, __mmv)?;
                    Ok(())
                }
                VarData::VAR_DATA_HES { variables, unknowns, knowns, auxiliaries, aliasVars, diffVars, dependencies, resultVars, tmpVars, seedVars, seedVars2, lambdaVars } => {
                    metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unknowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(knowns, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(diffVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dependencies, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(resultVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(tmpVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(seedVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(seedVars2, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lambdaVars, __mmv)?;
                    Ok(())
                }
                VarData::VAR_DATA_EMPTY => Ok(()),
            }
        }
    }
    impl VarData {
        pub fn interned_VAR_DATA_EMPTY() -> Arc<VarData> {
            thread_local! {
                static INTERNED: Arc<VarData> = Arc::new(VarData::VAR_DATA_EMPTY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_VAR_DATA_EMPTY() -> Arc<VarData> { VarData::interned_VAR_DATA_EMPTY() }
    impl Default for VarData {
        fn default() -> Self { Self::VAR_DATA_EMPTY }
    }
    pub use self::VarData::{VAR_DATA_SIM,VAR_DATA_JAC,VAR_DATA_HES,VAR_DATA_EMPTY};
    pub(crate) fn size(mut varData: Arc<VarData>) -> Result<i32> {
        let mut s: i32;
        s = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => VariablePointers::size(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone()),
        Deref @ VAR_DATA_JAC { .. } => VariablePointers::size(var_field!((*varData).unknowns, VarData::VAR_DATA_JAC).clone()),
        Deref @ VAR_DATA_HES { .. } => VariablePointers::size(var_field!((*varData).unknowns, VarData::VAR_DATA_HES).clone()),
        _ => bail!("match: no arm matched"),
    } });
        Ok(s)
    }

    pub(crate) fn scalarSize(mut varData: Arc<VarData>, mut resize: bool) -> Result<i32> {
        let mut s: i32;
        s = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), resize)?,
        Deref @ VAR_DATA_JAC { .. } => VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_JAC).clone(), resize)?,
        Deref @ VAR_DATA_HES { .. } => VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_HES).clone(), resize)?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(s)
    }

    pub(crate) fn toString(mut varData: Arc<VarData>, mut level: i32) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (if (level == 0) {(::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => VariablePointers::toString(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), (literal!("Simulation")).clone(), None, true)?,
        Deref @ VAR_DATA_JAC { .. } => VariablePointers::toString(var_field!((*varData).variables, VarData::VAR_DATA_JAC).clone(), (literal!("Jacobian")).clone(), None, true)?,
        Deref @ VAR_DATA_HES { .. } => VariablePointers::toString(var_field!((*varData).variables, VarData::VAR_DATA_HES).clone(), (literal!("Hessian")).clone(), None, true)?,
        Deref @ VAR_DATA_EMPTY { .. } => literal!("Empty variable Data!\n"),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })} else if (level == 1) {toStringVerbose(varData, false)?} else {toStringVerbose(varData, true)?}).clone();
        Ok(r#str)
    }

    pub(crate) fn toStringVerbose(mut varData: Arc<VarData>, mut full: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (({
        let mut tmp: ArcStr = literal!("");
        (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => {
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable Data Simulation (scalar unknowns: ")); __mm_s.push_str(&*intString(VariablePointers::scalarSize(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), true)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((tmp).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if !(full) {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), (literal!("Unknown")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), (literal!("Local Known")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), (literal!("Global Known")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            } else {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), (literal!("State")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone(), (literal!("Derivative")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone(), (literal!("Algebraic")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone(), (literal!("Discrete")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).discrete_states, VarData::VAR_DATA_SIM).clone(), (literal!("Discrete State")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).clocked_states, VarData::VAR_DATA_SIM).clone(), (literal!("Clocked State")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).previous, VarData::VAR_DATA_SIM).clone(), (literal!("Previous")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone(), (literal!("Clock")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).top_level_inputs, VarData::VAR_DATA_SIM).clone(), (literal!("Top Level Input")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).resizables, VarData::VAR_DATA_SIM).clone(), (literal!("Resizable Parameter")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone(), (literal!("Parameter")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).constants, VarData::VAR_DATA_SIM).clone(), (literal!("Constant")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), (literal!("Record")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).external_objects, VarData::VAR_DATA_SIM).clone(), (literal!("External Object")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone(), (literal!("Artificial")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            }
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).auxiliaries, VarData::VAR_DATA_SIM).clone(), (literal!("Auxiliary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).aliasVars, VarData::VAR_DATA_SIM).clone(), (literal!("Alias")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            tmp
        },
        Deref @ VAR_DATA_JAC { .. } => {
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).unknowns, VarData::VAR_DATA_JAC).clone(), (literal!("Partial Derivative")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).seedVars, VarData::VAR_DATA_JAC).clone(), (literal!("Seed")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            if full {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).diffVars, VarData::VAR_DATA_JAC).clone(), (literal!("Differentiation")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).resultVars, VarData::VAR_DATA_JAC).clone(), (literal!("Residual")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).tmpVars, VarData::VAR_DATA_JAC).clone(), (literal!("Inner")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).dependencies, VarData::VAR_DATA_JAC).clone(), (literal!("Dependencies")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).auxiliaries, VarData::VAR_DATA_JAC).clone(), (literal!("Auxiliary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).aliasVars, VarData::VAR_DATA_JAC).clone(), (literal!("Alias")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            }
            tmp
        },
        Deref @ VAR_DATA_HES { .. } => {
            let mut lambdaVars: Arc<VariablePointers::VariablePointers>;
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Variable Data Hessian")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).unknowns, VarData::VAR_DATA_HES).clone(), (literal!("Unknown")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).knowns, VarData::VAR_DATA_HES).clone(), (literal!("Known")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).auxiliaries, VarData::VAR_DATA_HES).clone(), (literal!("Auxiliary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).aliasVars, VarData::VAR_DATA_HES).clone(), (literal!("Alias")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
            if full {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).diffVars, VarData::VAR_DATA_HES).clone(), (literal!("Differentiation")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).dependencies, VarData::VAR_DATA_HES).clone(), (literal!("Dependencies")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).resultVars, VarData::VAR_DATA_HES).clone(), (literal!("Result")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).tmpVars, VarData::VAR_DATA_HES).clone(), (literal!("Temporary")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).seedVars, VarData::VAR_DATA_HES).clone(), (literal!("First Seed")).clone(), None, false)?); __mm_s.push_str(&*VariablePointers::toString(var_field!((*varData).seedVars2, VarData::VAR_DATA_HES).clone(), (literal!("Second Seed")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
                if isSome(var_field!((*varData).lambdaVars, VarData::VAR_DATA_HES).clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(var_field!((*varData).lambdaVars, VarData::VAR_DATA_HES).clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lambdaVars = __pa0.clone();
                    tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp); __mm_s.push_str(&*VariablePointers::toString(lambdaVars, (literal!("Lagrangian Lambda")).clone(), None, false)?); ArcStr::from(__mm_s) }).clone();
                }
            }
            tmp
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    })).clone();
        Ok(r#str)
    }

    pub(crate) fn getVariables(mut varData: Arc<VarData>) -> Result<Arc<VariablePointers::VariablePointers>> {
        let mut variables: Arc<VariablePointers::VariablePointers>;
        variables = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(),
        Deref @ VAR_DATA_JAC { .. } => var_field!((*varData).variables, VarData::VAR_DATA_JAC).clone(),
        Deref @ VAR_DATA_HES { .. } => var_field!((*varData).variables, VarData::VAR_DATA_HES).clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(variables)
    }

    pub(crate) fn setVariables(mut varData: Arc<VarData>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<VarData>> {
        let mut varData: Arc<VarData> = varData;
        varData = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; variables = variables);
            varData
        },
        Deref @ VAR_DATA_JAC { .. } => {
            assign_variant_field!(varData => VarData::VAR_DATA_JAC; variables = variables);
            varData
        },
        Deref @ VAR_DATA_HES { .. } => {
            assign_variant_field!(varData => VarData::VAR_DATA_HES; variables = variables);
            varData
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(varData)
    }

    pub(crate) fn getUniqueIndex(mut varData: Arc<VarData>) -> Result<Pointer::Pointer<i32>> {
        let mut uniqueIndex: Pointer::Pointer<i32>;
        uniqueIndex = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => var_field!((*varData).uniqueIndex, VarData::VAR_DATA_SIM).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.getUniqueIndex")); __mm_s.push_str(&*literal!(" failed because of incorrect record type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(uniqueIndex)
    }

    pub(crate) fn getStateOrder(mut varData: Arc<VarData>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>> {
        let mut state_order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
        state_order = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ VAR_DATA_SIM { .. } => var_field!((*varData).state_order, VarData::VAR_DATA_SIM).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.getStateOrder")); __mm_s.push_str(&*literal!(" failed because of incorrect record type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(state_order)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub enum VarType {
        STATE = 1,
        STATE_DER = 2,
        ALGEBRAIC = 3,
        DISCRETE = 4,
        DISC_STATE = 5,
        PREVIOUS = 6,
        START = 7,
        PARAMETER = 8,
        ITERATOR = 9,
        RECORD = 10,
        CLOCK = 11,
    }
    impl PartialOrd for VarType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for VarType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
    }
    impl metamodelica::gc::MMTrace for VarType {
        fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
    }

    pub(crate) fn addTypedList(mut varData: Arc<VarData>, mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut varType: VarType) -> Result<Arc<VarData>> {
        let mut varData: Arc<VarData> = varData;
        varData = (::match_deref::match_deref! { match &((varData.clone(), varType)) {
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                knowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?,
                states = VariablePointers::addList(var_lst.clone(), var_field!((*varData).states, VarData::VAR_DATA_SIM).clone())?,
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?,
                unknowns = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                algebraics = VariablePointers::removeList(var_lst, var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE_DER) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                unknowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                derivatives = VariablePointers::addList(var_lst.clone(), var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone())?,
                initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ALGEBRAIC) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                unknowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                algebraics = VariablePointers::addList(var_lst.clone(), var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone())?,
                initials = VariablePointers::addList(var_lst.clone(), var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?,
                states = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).states, VarData::VAR_DATA_SIM).clone())?,
                derivatives = VariablePointers::removeList(var_lst.clone(), var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone())?,
                knowns = VariablePointers::removeList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::DISCRETE) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                unknowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone())?,
                discretes = VariablePointers::addList(var_lst.clone(), var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone())?,
                initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::START { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                initials = VariablePointers::addList(var_lst, var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::PARAMETER { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                parameters = VariablePointers::addList(var_lst.clone(), var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone())?,
                knowns = VariablePointers::addList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ITERATOR { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                knowns = VariablePointers::addList(var_lst.clone(), var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?,
                artificials = VariablePointers::addList(var_lst, var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::CLOCK) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; clocks = VariablePointers::addList(var_lst, var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone())?);
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::RECORD { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::addList(var_lst.clone(), var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone())?,
                records = VariablePointers::addList(var_lst.clone(), var_field!((*varData).records, VarData::VAR_DATA_SIM).clone())?,
                knowns = VariablePointers::addList(var_lst, var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone())?
            );
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; records = VariablePointers::mapPtr(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), (std::sync::Arc::new({ let __pe_b1 = var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(); move |__pe_a0| BackendDAE::lowerRecordChildren(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>))?);
            varData
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.addTypedList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(varData)
    }

    pub(crate) fn removeTypedCheck(mut varData: Arc<VarData>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>, mut varType: VarType) -> Result<Arc<VarData>> {
        let mut varData: Arc<VarData> = varData;
        varData = (::match_deref::match_deref! { match &((varData.clone(), varType)) {
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                states = VariablePointers::removeCheck(var_field!((*varData).states, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::STATE_DER) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                derivatives = VariablePointers::removeCheck(var_field!((*varData).derivatives, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ALGEBRAIC) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                algebraics = VariablePointers::removeCheck(var_field!((*varData).algebraics, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::DISCRETE) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                unknowns = VariablePointers::removeCheck(var_field!((*varData).unknowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                discretes = VariablePointers::removeCheck(var_field!((*varData).discretes, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::START { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                initials = VariablePointers::removeCheck(var_field!((*varData).initials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::PARAMETER { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                parameters = VariablePointers::removeCheck(var_field!((*varData).parameters, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::ITERATOR { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                artificials = VariablePointers::removeCheck(var_field!((*varData).artificials, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::CLOCK) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM; clocks = VariablePointers::removeCheck(var_field!((*varData).clocks, VarData::VAR_DATA_SIM).clone(), func.clone())?);
            varData
        },
        (Deref @ VAR_DATA_SIM { .. }, VarType::RECORD { .. }) => {
            assign_variant_field!(varData => VarData::VAR_DATA_SIM;
                variables = VariablePointers::removeCheck(var_field!((*varData).variables, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                records = VariablePointers::removeCheck(var_field!((*varData).records, VarData::VAR_DATA_SIM).clone(), func.clone())?,
                knowns = VariablePointers::removeCheck(var_field!((*varData).knowns, VarData::VAR_DATA_SIM).clone(), func.clone())?
            );
            varData
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBVariable.VarData.removeTypedCheck")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(varData)
    }

}

// ==========================================================================
//                      Protected utility functions
// ==========================================================================
fn getVarNameTraverse(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>) -> () {
    Pointer::update(acc.clone(), metamodelica::cons(getVarName(var), Pointer::access(acc)));
    ()
}

