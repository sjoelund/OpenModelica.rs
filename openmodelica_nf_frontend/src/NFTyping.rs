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

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFModifier::ModTable;
use crate::NFModifier::Modifier;
use crate::NFOperator as Operator;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPackage as Package;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub mod TypingError {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum TypingError {
        NO_ERROR,
        OUT_OF_BOUNDS {
            upperBound: i32,
        },
    }
    impl metamodelica::gc::MMTrace for TypingError {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                TypingError::NO_ERROR => Ok(()),
                TypingError::OUT_OF_BOUNDS { upperBound } => {
                    metamodelica::gc::MMTrace::mm_accept(upperBound, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl TypingError {
        pub fn interned_NO_ERROR() -> Arc<TypingError> {
            static INTERNED: std::sync::LazyLock<Arc<TypingError>> = std::sync::LazyLock::new(|| Arc::new(TypingError::NO_ERROR));
            (*INTERNED).clone()
        }
    }
    pub fn interned_NO_ERROR() -> Arc<TypingError> { TypingError::interned_NO_ERROR() }
    impl Default for TypingError {
        fn default() -> Self { Self::NO_ERROR }
    }
    pub(crate) use self::TypingError::{NO_ERROR,OUT_OF_BOUNDS};
    pub(crate) fn isError(mut error: Arc<TypingError>) -> bool {
        let mut isError: bool;
        isError = (::match_deref::match_deref! { match &(error) {
        Deref @ NO_ERROR { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isError
    }

}

// Used by typeDimension for catching cyclic dimension involving :
thread_local! { static __WHOLEDIM_CREF_TLS: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::CREF { ty: crate::NFType::interned_UNKNOWN(), cref: Arc::new(ComponentRef::NFComponentRef::CREF { node: Arc::new(InstNode::InstNode::NAME_NODE { name: (literal!(":")).clone() }), subscripts: metamodelica::nil(), ty: crate::NFType::interned_UNKNOWN(), origin: ComponentRef::Origin::CREF.clone(), restCref: crate::NFComponentRef::interned_EMPTY() }) }); }
pub(crate) fn WHOLEDIM_CREF() -> Arc<Expression::NFExpression> { __WHOLEDIM_CREF_TLS.with(|__t| __t.clone()) }

pub fn typeClass(mut cls: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut next_context: i32;
    next_context = InstContext::set(context, InstContext::CLASS.clone());
    typeClassType(cls.clone(), Binding::EMPTY_BINDING().clone(), next_context, cls.clone())?;
    typeComponents(cls.clone(), next_context, false)?;
    execStat((literal!("NFTyping.typeComponents")).clone())?;
    typeBindings(cls.clone(), next_context)?;
    execStat((literal!("NFTyping.typeBindings")).clone())?;
    typeClassSections(cls, next_context)?;
    execStat((literal!("NFTyping.typeClassSections")).clone())?;
    Ok(())
}

pub fn typeComponents(mut cls: Arc<InstNode::InstNode>, mut context: i32, mut preserveDerived: bool) -> Result<()> {
    let mut c: Arc<Class::NFClass> = InstNode::getClass(cls.clone())?;
    let mut c2: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut con: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut de: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::TYPE, .. } => (),
        Deref @ Class::INSTANCED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            if InstContext::inInstanceAPI(context) {
                let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range0 {
                    typeComponentTry(c.clone(), context)?;
                }
            } else {
                let __range1 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    typeComponent(c.clone(), context, true)?;
                }
            }
            let () = (::match_deref::match_deref! { match &(var_field!((*c).ty, Class::NFClass::INSTANCED_CLASS).clone()) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: __esc_con, .. }, .. } => {
            con = (*__esc_con).clone();
            typeStructor(con.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } if (preserveDerived || Type::isArray(var_field!((*c).ty, Class::NFClass::TYPED_DERIVED).clone())) => {
            typeComponents(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context, false)?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeComponents(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context, false)?;
            if !(InstContext::inInstanceAPI(context)) {
                c2 = InstNode::getClass(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone())?;
                c2 = Class::setRestriction(var_field!((*c).restriction, Class::NFClass::TYPED_DERIVED).clone(), c2)?;
                InstNode::updateClass(c2, cls)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __esc_con, destructor: __esc_de }, .. }, .. } => {
            con = (*__esc_con).clone();
            de = (*__esc_de).clone();
            typeStructor(con.clone())?;
            typeStructor(de.clone())?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponents")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(cls)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeStructor(mut node: Arc<InstNode::InstNode>) -> Result<()> {
    let mut cache: Arc<CachedData::CachedData>;
    let mut fnl: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut context: i32 = 0;
    cache = InstNode::getFuncCache(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { funcs: __esc_fnl, typed: false, .. } => {
            fnl = (*__esc_fnl).clone();
            context = InstContext::set(InstContext::FUNCTION.clone(), InstContext::RELAXED.clone());
            fnl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (fnl.clone()).into_iter().cloned() {
            let __x = Function::typeFunction(r#fn.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            fnl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (fnl.clone()).into_iter().cloned() {
            let __x = OperatorOverloading::patchOperatorRecordConstructorBinding(r#fn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            InstNode::setFuncCache(node, Arc::new(CachedData::CachedData::FUNCTION { funcs: fnl.clone(), typed: true, specialBuiltin: var_field!((*cache).specialBuiltin, CachedData::CachedData::FUNCTION).clone() }))?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeClassType(mut clsNode: Arc<InstNode::InstNode>, mut componentBinding: Arc<Binding::NFBinding>, mut context: i32, mut instanceNode: Arc<InstNode::InstNode>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cls: Arc<Class::NFClass>;
    let mut ty_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut is_expandable: bool = false;
    cls = InstNode::getClass(clsNode.clone())?;
    ty = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::CONNECTOR { isExpandable: __esc_is_expandable }, .. } => {
            is_expandable = (*__esc_is_expandable).clone();
            ty = Arc::new(Type::NFType::COMPLEX { cls: clsNode.clone(), complexTy: makeConnectorType(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone(), is_expandable.clone())? });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { ty: Deref @ Type::COMPLEX { cls: __esc_ty_node, complexTy: Deref @ ComplexType::RECORD { constructor: __esc_node, .. } }, .. } => {
            ty_node = (*__esc_ty_node).clone();
            node = (*__esc_node).clone();
            ty = Arc::new(Type::NFType::COMPLEX { cls: ty_node.clone(), complexTy: makeRecordType(node.clone())? });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTENDS_TYPE { baseClass: __esc_node }, .. }, .. } => {
            node = (*__esc_node).clone();
            ty = typeClassType(node.clone(), componentBinding, context, instanceNode.clone())?;
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::FUNCTION, .. } if (InstNode::isComponent(instanceNode.clone())?) => {
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(clsNode.clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            ty = Arc::new(Type::NFType::FUNCTION { r#fn: r#fn, fnType: Type::FunctionType::FUNCTIONAL_PARAMETER.clone() });
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; ty = ty.clone());
            InstNode::updateClass(cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_CLASS { .. } => var_field!((*cls).ty, Class::NFClass::INSTANCED_CLASS).clone(),
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            typeDimensions(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone(), clsNode.clone(), componentBinding.clone(), context, InstNode::info(clsNode.clone()))?;
            ty = typeClassType(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), componentBinding, context, instanceNode.clone())?;
            ty = Type::liftArrayLeftList(ty, Arc::new(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            ty_cls = Arc::new(Class::NFClass::TYPED_DERIVED { ty: ty.clone(), baseClass: var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::EXPANDED_DERIVED).clone() });
            InstNode::updateClass(ty_cls, clsNode)?;
            ty
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => var_field!((*cls).ty, Class::NFClass::INSTANCED_BUILTIN).clone(),
        Deref @ Class::TYPED_DERIVED { .. } => var_field!((*cls).ty, Class::NFClass::TYPED_DERIVED).clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeClassType")); __mm_s.push_str(&*literal!(" got noninstantiated class ")); __mm_s.push_str(&*InstNode::name(clsNode)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub(crate) fn makeConnectorType(mut ctree: Arc<ClassTree::ClassTree>, mut isExpandable: bool) -> Result<Arc<ComplexType::NFComplexType>> {
    let mut connectorTy: Arc<ComplexType::NFComplexType>;
    let mut pots: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut flows: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut streams: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut cty: i32;
    if isExpandable {
        for mut c in &*ClassTree::enumerateComponents(ctree)? {
            let mut c = c.clone();
            cty = Component::connectorType(InstNode::component(InstNode::resolveInner(c.clone()))?);
            if intBitAnd(cty, ConnectorType::EXPANDABLE.clone()) > 0 {
                exps = metamodelica::cons(c.clone(), exps.clone());
            } else {
                pots = metamodelica::cons(c.clone(), pots.clone());
            }
        }
        connectorTy = Arc::new(ComplexType::NFComplexType::EXPANDABLE_CONNECTOR { potentiallyPresents: pots, expandableConnectors: exps });
    } else {
        for mut c in &*ClassTree::enumerateComponents(ctree)? {
            let mut c = c.clone();
            cty = Component::connectorType(InstNode::component(InstNode::resolveInner(c.clone()))?);
            if intBitAnd(cty, ConnectorType::FLOW.clone()) > 0 {
                flows = metamodelica::cons(c.clone(), flows.clone());
            } else if intBitAnd(cty, ConnectorType::STREAM.clone()) > 0 {
                streams = metamodelica::cons(c.clone(), streams.clone());
            } else if intBitAnd(cty, ConnectorType::POTENTIAL.clone()) > 0 {
                pots = metamodelica::cons(c.clone(), pots.clone());
            } else {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Invalid connector type on component ")); __mm_s.push_str(&*InstNode::name(c.clone())?); ArcStr::from(__mm_s) }).clone(), InstNode::info(c.clone()))?;
                bail!("fail");
            }
        }
        connectorTy = Arc::new(ComplexType::NFComplexType::CONNECTOR { potentials: pots, flows: flows, streams: streams.clone() });
        if !(streams.is_empty()) {
            System::setHasStreamConnectors(true);
        }
    }
    Ok(connectorTy)
}

pub(crate) fn checkConnectorTypeBalance(mut component: Arc<InstNode::InstNode>) -> Result<()> {
    let mut pots: i32;
    let mut flows: i32;
    let mut streams: i32;
    let mut known_size: bool;
    let mut comp: Arc<Component::NFComponent>;
    let mut parent: Arc<InstNode::InstNode>;
    comp = InstNode::component(component.clone())?;
    if !(Prefixes::ConnectorType::isConnector(Component::connectorType(comp.clone()))) {
        return Ok(());
    }
    parent = InstNode::instanceParent(component.clone())?;
    if InstNode::isComponent(parent.clone())? && Component::isConnector(InstNode::component(parent)?) {
        return Ok(());
    }
    (pots, flows, streams, known_size) = Component::countConnectorVars(comp, true)?;
    if !(known_size) {
        return Ok(());
    }
    if pots != flows && !(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("unbalancedModel")).clone())?) {
        Error::addStrictMessage(Error::UNBALANCED_CONNECTOR.clone(), list![(InstNode::name(component.clone())?).clone(), ArcStr::from(::std::format!("{}", pots)), ArcStr::from(::std::format!("{}", flows))], InstNode::info(component.clone()))?;
    }
    if streams > 0 && flows != 1 {
        Error::addSourceMessage(Error::MISMATCHED_FLOW_IN_STREAM_CONNECTOR.clone(), list![(InstNode::name(component.clone())?).clone(), ArcStr::from(::std::format!("{}", flows))], InstNode::info(component))?;
        bail!("fail");
    }
    Ok(())
}

pub(crate) fn makeRecordType(mut constructor: Arc<InstNode::InstNode>) -> Result<Arc<ComplexType::NFComplexType>> {
    let mut recordTy: Arc<ComplexType::NFComplexType>;
    let mut cache: Arc<CachedData::CachedData>;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut fields: metamodelica::Array<Arc<Record::Field::Field>> = Default::default();
    let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>> = <Arc<UnorderedMap::UnorderedMap<ArcStr, i32>> as ::std::default::Default>::default();
    cache = InstNode::getFuncCache(constructor.clone())?;
    recordTy = 'mc: {
        let __mc_input = cache.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ CachedData::FUNCTION { .. } => {
                    let mut fields: metamodelica::Array<Arc<Record::Field::Field>> = fields.clone();
                    let mut r#fn: Arc<Function::Function> = r#fn.clone();
                    let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>> = indexMap.clone();
                    r#fn = List::find(var_field!((*cache).funcs, CachedData::CachedData::FUNCTION).clone(), (std::sync::Arc::new(fnptr!(Function::isDefaultRecordConstructor, Arc<Function::Function>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>) -> Result<bool> + 'static>))?;
                    (fields, indexMap) = Record::collectRecordFields(r#fn.node.clone())?;
                    Ok((Arc::new(ComplexType::NFComplexType::RECORD { constructor: constructor.clone(), fields: fields.clone(), indexMap: indexMap.clone() }), fields.clone(), r#fn.clone(), indexMap.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { fields = __wb0; r#fn = __wb1; indexMap = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.makeRecordType")); __mm_s.push_str(&*literal!(" got record type without constructor")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(recordTy)
}

pub(crate) fn typeComponent(mut component: Arc<InstNode::InstNode>, mut context: i32, mut typeChildren: bool) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut node: Arc<InstNode::InstNode>;
    let mut c: Arc<Component::NFComponent>;
    let mut is_deleted: bool = false;
    let mut dims: metamodelica::Array<Arc<Dimension::NFDimension>> = Default::default();
    if InstNode::isEmpty(component.clone()) || InstNode::isOnlyOuter(component.clone())? {
        return Ok(ty.clone());
    }
    node = InstNode::resolveOuter(component.clone());
    c = InstNode::component(node.clone())?;
    ty = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: __esc_dims, .. }, .. } => {
            dims = (*__esc_dims).clone();
            typeDimensions(dims.clone(), node.clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context, var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
            if InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone()) {
                ty = crate::NFType::interned_UNKNOWN();
            } else {
                ty = typeClassType(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context, component.clone())?;
            }
            ty = Type::liftArrayLeftList(ty, Arc::new(dims.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()));
            if Binding::isBound(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone()) {
                assign_variant_field!(c => Component::NFComponent::COMPONENT; condition = typeComponentCondition(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone(), context, true)?);
                is_deleted = Expression::isFalse(Binding::getExp(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone())?);
            } else {
                is_deleted = false;
            }
            if typeChildren {
                assign_variant_field!(c => Component::NFComponent::COMPONENT;
                    ty = ty.clone(),
                    state = ComponentState::Typed.clone()
                );
                InstNode::updateComponent(c.clone(), node.clone())?;
                if !(is_deleted) && !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                    checkComponentStreamAttribute(var_field!((*c).attributes, Component::NFComponent::COMPONENT).connectorType.clone(), ty.clone(), component)?;
                    typeComponents(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context, false)?;
                    checkConnectorTypeBalance(node)?;
                }
            }
            ty.clone()
        },
        Deref @ Component::COMPONENT { .. } => var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(),
        Deref @ Component::ITERATOR { .. } => var_field!((*c).ty, Component::NFComponent::ITERATOR).clone(),
        Deref @ Component::ENUM_LITERAL { literal: Deref @ Expression::ENUM_LITERAL { ty: __esc_ty, .. }, .. } => {
            ty = (*__esc_ty).clone();
            ty.clone()
        },
        Deref @ Component::INVALID_COMPONENT { .. } => Component::getType(c)?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponent")); __mm_s.push_str(&*literal!(" got noninstantiated component ")); __mm_s.push_str(&*InstNode::name(component)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub(crate) fn typeComponentTry(mut componentNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut comp: Arc<Component::NFComponent>;
    ErrorExt::setCheckpoint(literal!("NFTyping.typeComponentTry"));
    if '__try0: {
        unwrap_break_err!(typeComponent(componentNode.clone(), context, true), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        comp = InstNode::component(componentNode.clone())?;
        comp = Arc::new(Component::NFComponent::INVALID_COMPONENT { component: comp.clone(), errors: (ErrorExt::printCheckpointMessagesStr(false)).clone() });
        InstNode::updateComponent(comp.clone(), componentNode.clone())?;
    }
    ErrorExt::delCheckpoint(literal!("NFTyping.typeComponentTry"));
    Ok(())
}

pub(crate) fn checkComponentStreamAttribute(mut cty: i32, mut ty: Arc<Type::NFType>, mut component: Arc<InstNode::InstNode>) -> Result<()> {
    let mut ety: Arc<Type::NFType>;
    if Prefixes::ConnectorType::isFlowOrStream(cty) {
        ety = Type::arrayElementType(ty);
        if !(Type::isReal(ety.clone())? || Type::isComplex(ety)) {
            Error::addSourceMessageAndFail(Error::NON_REAL_FLOW_OR_STREAM.clone(), list![(Prefixes::ConnectorType::toString(cty)).clone(), (InstNode::name(component.clone())?).clone()], InstNode::info(component))?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(())
}

pub(crate) fn typeIterator(mut iterator: Arc<InstNode::InstNode>, mut range: Arc<Expression::NFExpression>, mut context: i32, mut structural: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outRange: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut c: Arc<Component::NFComponent> = InstNode::component(iterator.clone())?;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (outRange, ty, var) = (::match_deref::match_deref! { match &(c) {
        Deref @ Component::ITERATOR { info: __esc_info, .. } => {
            info = (*__esc_info).clone();
            (exp, ty, var, purity) = typeExp(range, InstContext::set(context, InstContext::ITERATION_RANGE.clone()), info.clone(), false)?;
            if structural && var > Variability::PARAMETER.clone() && (!(var == Variability::NON_STRUCTURAL_PARAMETER.clone()) || Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
                Error::addSourceMessageAndFail(Error::NON_PARAMETER_ITERATOR_RANGE.clone(), list![(Expression::toString(exp.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if !(Type::isVector(ty.clone())?) {
                Error::addSourceMessageAndFail(Error::FOR_EXPRESSION_TYPE_ERROR.clone(), list![(Expression::toString(exp.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            c = Arc::new(Component::NFComponent::ITERATOR { ty: Type::arrayElementType(ty.clone()), variability: var, info: info.clone() });
            InstNode::updateComponent(c, iterator)?;
            (exp, ty, var)
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeIterator")); __mm_s.push_str(&*literal!(" got non-iterator ")); __mm_s.push_str(&*InstNode::name(iterator)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRange, ty, var, purity))
}

pub(crate) fn typeDimensions(mut dimensions: metamodelica::Array<Arc<Dimension::NFDimension>>, mut component: Arc<InstNode::InstNode>, mut binding: Arc<Binding::NFBinding>, mut context: i32, mut info: SourceInfo) -> Result<metamodelica::Array<Arc<Dimension::NFDimension>>> {
    let mut dimensions: metamodelica::Array<Arc<Dimension::NFDimension>> = dimensions;
    for mut i in 1..=metamodelica::arrayLength(dimensions.clone()) {
        typeDimension(dimensions.clone(), i.clone(), component.clone(), binding.clone(), context, info.clone())?;
    }
    Ok(dimensions)
}

pub(crate) fn typeDimension(mut dimensions: metamodelica::Array<Arc<Dimension::NFDimension>>, mut index: i32, mut component: Arc<InstNode::InstNode>, mut binding: Arc<Binding::NFBinding>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dimension: Arc<Dimension::NFDimension> = ({let __elt = dimensions.borrow()[(index-1) as usize].clone(); __elt});
    dimension = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::UNTYPED { isProcessing: true, .. } => {
            let mut dim: Arc<Dimension::NFDimension>;
            if InstContext::inFunction(context) {
                dim = crate::NFDimension::interned_UNKNOWN();
                metamodelica::arrayUpdate(dimensions.clone(), index, dim.clone())?;
            } else {
                dim = dimension;
            }
            dim
        },
        Deref @ Dimension::UNTYPED { .. } => {
            let mut exp: Arc<Expression::NFExpression>;
            let mut var: Variability;
            let mut dim: Arc<Dimension::NFDimension>;
            let mut ty: Arc<Type::NFType>;
            let mut target: Arc<Ceval::EvalTarget::EvalTarget>;
            metamodelica::arrayUpdate(dimensions.clone(), index, Arc::new(Dimension::NFDimension::UNTYPED { dimension: var_field!((*dimension).dimension, Dimension::NFDimension::UNTYPED).clone(), isProcessing: true }))?;
            (exp, ty, var, _) = typeExp(var_field!((*dimension).dimension, Dimension::NFDimension::UNTYPED).clone(), InstContext::set(context, InstContext::DIMENSION.clone()), info.clone(), false)?;
            TypeCheck::checkDimensionType(exp.clone(), ty, info.clone())?;
            if !(InstContext::inFunction(context)) {
                if var <= Variability::PARAMETER.clone() {
                    if InstContext::inRelaxed(context) {
                        exp = Ceval::tryEvalExp(exp, Ceval::noTarget().clone());
                    } else {
                        target = Ceval::EvalTarget::new(info, context, Some(Arc::new(Ceval::EvalTargetData { component: component.clone(), index: index, exp: exp.clone() })));
                        exp = Ceval::tryEvalExpResizable(exp, target)?;
                    }
                } else if !(var == Variability::NON_STRUCTURAL_PARAMETER.clone()) {
                    Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(Expression::toString(exp.clone())?).clone()], info)?;
                    bail!("fail");
                }
            } else {
                if var <= Variability::STRUCTURAL_PARAMETER.clone() && !(Expression::contains(exp.clone(), (std::sync::Arc::new(fnptr!(Expression::isFunctionInputCref, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
                    exp = Ceval::tryEvalExp(exp, Ceval::noTarget().clone());
                }
            }
            exp = subscriptDimExp(exp, component.clone())?;
            dim = Dimension::fromExp(exp, var)?;
            metamodelica::arrayUpdate(dimensions.clone(), index, dim.clone())?;
            dim
        },
        Deref @ Dimension::UNKNOWN if (InstContext::inFunction(context) && (Binding::isUnbound(binding.clone()) && InstNode::isOutput(component.clone()) || !(InstNode::isOutput(component.clone())))) => {
            dimension
        },
        Deref @ Dimension::UNKNOWN if (InstContext::inFunction(context) && Binding::hasExp(binding.clone()) && Expression::contains(Binding::getExp(binding.clone())?, (std::sync::Arc::new(fnptr!(Expression::isCref, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) => {
            dimension
        },
        Deref @ Dimension::UNKNOWN => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut dim: Arc<Dimension::NFDimension>;
            let mut b: Arc<Binding::NFBinding>;
            let mut ty_err: Arc<TypingError::TypingError>;
            let mut parent_dims: i32;
            let mut target: Arc<Ceval::EvalTarget::EvalTarget> = Arc::new(<Ceval::EvalTarget::EvalTarget as ::std::default::Default>::default());
            b = binding.clone();
            parent_dims = 0;
            metamodelica::arrayUpdate(dimensions.clone(), index, Arc::new(Dimension::NFDimension::UNTYPED { dimension: WHOLEDIM_CREF().clone(), isProcessing: true }))?;
            if Binding::isUnbound(binding.clone()) {
                (b, parent_dims) = getRecordElementBinding(component.clone(), context)?;
                if Binding::isUnbound(b.clone()) {
                    parent_dims = 0;
                    b = Class::lookupAttributeBinding((literal!("start")).clone(), InstNode::getClass(component.clone())?);
                    b = Binding::mapExp(b, (std::sync::Arc::new({ let __pe_b1 = component.clone(); move |__pe_a0| Expression::filterSplitIndices(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                }
            }
            (dim, ty_err) = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Binding::UNBOUND if (!(InstContext::inRelaxed(context))) => {
            Error::addSourceMessage(Error::FAILURE_TO_DEDUCE_DIMS_NO_MOD.clone(), list![ArcStr::from(::std::format!("{}", index)), (InstNode::name(component.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ Binding::UNTYPED_BINDING { .. } => deduceDimensionFromExp(var_field!((*b).bindingExp, Binding::NFBinding::UNTYPED_BINDING).clone(), None, index, parent_dims, component.clone(), context, info.clone())?,
        Deref @ Binding::TYPED_BINDING { .. } => deduceDimensionFromExp(var_field!((*b).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), Some(var_field!((*b).bindingType, Binding::NFBinding::TYPED_BINDING).clone()), index, parent_dims, component.clone(), context, info.clone())?,
        _ => (dimension, crate::NFTyping::TypingError::interned_NO_ERROR()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            let () = (::match_deref::match_deref! { match &(ty_err) {
        Deref @ TypingError::OUT_OF_BOUNDS { .. } if (!(InstContext::inRelaxed(context))) => {
            Error::addSourceMessage(Error::DIMENSION_DEDUCTION_FROM_BINDING_FAILURE.clone(), list![ArcStr::from(::std::format!("{}", index)), (InstNode::name(component.clone())?).clone(), (Binding::toString(b, (literal!("")).clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { exp: __esc_exp, .. } => {
            exp = (*__esc_exp).clone();
            Structural::markExp(exp.clone())?;
            if InstContext::inRelaxed(context) {
                exp = Ceval::tryEvalExp(exp.clone(), Ceval::noTarget().clone());
            } else {
                target = Ceval::EvalTarget::new(info, context, Some(Arc::new(Ceval::EvalTargetData { component: component.clone(), index: index, exp: exp.clone() })));
                exp = Ceval::evalExp(exp.clone(), target)?;
            }
            exp = subscriptDimExp(exp.clone(), component.clone())?;
            Dimension::fromExp(exp.clone(), var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?
        },
        Deref @ Dimension::UNKNOWN if (!(InstContext::inRelaxed(context))) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeDimension")); __mm_s.push_str(&*literal!(" returned unknown dimension in a non-function context")); ArcStr::from(__mm_s) }).clone(), info)?;
            bail!("fail")
        },
        _ => dim,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            metamodelica::arrayUpdate(dimensions.clone(), index, dim.clone())?;
            dim
        },
        _ => {
            dimension
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimension)
}

pub(crate) fn deduceDimensionFromExp(mut exp: Arc<Expression::NFExpression>, mut ty: Option<Arc<Type::NFType>>, mut index: i32, mut parentDims: i32, mut component: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension>;
    let mut error: Arc<TypingError::TypingError>;
    let mut oe: Option<Arc<Expression::NFExpression>>;
    let mut e: Arc<Expression::NFExpression>;
    let mut dim_index: i32;
    dim_index = index + parentDims;
    if isSome(ty.clone()) {
        (dim, error) = nthDimensionBoundsChecked(Util::getOption(ty)?, dim_index, 0)?;
        oe = None;
    } else {
        (dim, oe, error) = typeExpDim(exp.clone(), dim_index, InstContext::set(context, InstContext::DIMENSION.clone()), info.clone())?;
    }
    if Dimension::isUnknown(dim.clone()) && !(TypingError::isError(error.clone())) {
        e = if (isSome(oe.clone())) {Util::getOption(oe)?} else {exp};
        if InstContext::inRelaxed(context) {
            e = Ceval::tryEvalExp(e, Ceval::noTarget().clone());
        } else {
            e = Ceval::evalExp(e.clone(), Ceval::EvalTarget::new(info, context, Some(Arc::new(Ceval::EvalTargetData { component: component, index: index, exp: e }))))?;
        }
        (dim, error) = nthDimensionBoundsChecked(Expression::typeOf(e), dim_index, 0)?;
    }
    Ok((dim, error))
}

pub(crate) fn subscriptDimExp(mut dimExp: Arc<Expression::NFExpression>, mut component: Arc<InstNode::InstNode>) -> Result<Arc<Expression::NFExpression>> {
    let mut dimExp: Arc<Expression::NFExpression> = dimExp;
    let mut exp_dims: i32;
    let mut parent_dims: i32;
    let mut parent: Arc<InstNode::InstNode>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    exp_dims = Expression::dimensionCount(dimExp.clone())?;
    if exp_dims == 0 {
        return Ok(dimExp.clone());
    }
    subs = metamodelica::nil();
    parent = InstNode::instanceParent(component)?;
    while exp_dims > 0 && !(InstNode::isEmpty(parent.clone())) {
        parent_dims = InstNode::dimensionCount(parent.clone());
        for mut i in ({let __s=parent_dims; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            subs = metamodelica::cons(Subscript::makeSplitIndex(parent.clone(), i.clone())?, subs.clone());
            exp_dims = exp_dims - 1;
            if exp_dims == 0 {
                break;
            }
        }
        parent = InstNode::instanceParent(parent.clone())?;
    }
    dimExp = Expression::applySubscripts(subs, dimExp, false)?;
    Ok(dimExp)
}

pub(crate) fn simplifyDimExp(mut dimExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut dimExp: Arc<Expression::NFExpression> = dimExp;
    let mut exp: Arc<Expression::NFExpression>;
    dimExp = (::match_deref::match_deref! { match &(dimExp.clone()) {
        Deref @ Expression::ARRAY { .. } if (Expression::arrayAllEqual(dimExp.clone())) => Expression::arrayFirstScalar(dimExp.clone())?,
        Deref @ Expression::SUBSCRIPTED_EXP { split: true, .. } if (Expression::isArray(var_field!((*dimExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) && Expression::arrayAllEqual(var_field!((*dimExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())) => Expression::arrayFirstScalar(var_field!((*dimExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())?,
        _ => dimExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimExp)
}

pub(crate) fn makeDimension(mut dimExp: Arc<Expression::NFExpression>, mut unevaledExp: Arc<Expression::NFExpression>, mut variability: Variability) -> Result<Arc<Dimension::NFDimension>> {
    let mut outDimension: Arc<Dimension::NFDimension>;
    let mut exp: Arc<Expression::NFExpression> = dimExp.clone();
    if Expression::isArray(exp.clone()) {
        if Expression::arrayAllEqual(exp.clone()) {
            exp = Expression::arrayFirstScalar(exp)?;
        }
    }
    outDimension = Dimension::fromExp(exp, variability)?;
    Ok(outDimension)
}

pub(crate) fn getRecordElementBinding(mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<Binding::NFBinding>, i32)> {
    let mut binding: Arc<Binding::NFBinding>;
    let mut parentDims: i32 = 0;
    let mut parent: Arc<InstNode::InstNode>;
    let mut comp: Arc<Component::NFComponent>;
    let mut parent_binding: Arc<Binding::NFBinding>;
    parent = InstNode::instanceParent(component.clone())?;
    if InstNode::isComponent(parent.clone())? {
        comp = InstNode::component(parent.clone())?;
        parent_binding = Component::getBinding(comp.clone());
        if Binding::isUnbound(parent_binding.clone()) {
            (binding, parentDims) = getRecordElementBinding(parent, context)?;
        } else {
            binding = typeBinding(parent_binding.clone(), InstContext::set(context, InstContext::DIMENSION.clone()))?;
            if !(referenceEq(&*(parent_binding),&*(binding.clone()))) {
                InstNode::componentApply(parent, (std::sync::Arc::new(Component::setBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Binding::NFBinding>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), binding.clone())?;
            }
        }
        parentDims = parentDims + Component::dimensionCount(comp);
        if Binding::isBound(binding.clone()) {
            binding = Binding::recordFieldBinding(component, binding)?;
        }
    } else {
        binding = Binding::EMPTY_BINDING().clone();
    }
    Ok((binding, parentDims))
}

pub fn typeBindings(mut cls: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut c: Arc<Class::NFClass>;
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    c = InstNode::getClass(cls.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Class::INSTANCED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentBinding(c.clone(), context, true)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentBinding(c.clone(), context, true)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeBindings(var_field!((*c).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context)?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeBindings")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(cls)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeComponentBinding(mut component: Arc<InstNode::InstNode>, mut context: i32, mut typeChildren: bool) -> Result<()> {
    let mut node: Arc<InstNode::InstNode>;
    let mut c: Arc<Component::NFComponent>;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut name: ArcStr = arcstr::literal!("");
    let mut comp_var: Variability = Variability::CONSTANT;
    let mut attrs: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType>;
    if InstNode::isEmpty(component.clone()) || InstNode::isOnlyOuter(component.clone())? {
        return Ok(());
    }
    node = InstNode::resolveOuter(component.clone());
    c = InstNode::component(node.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { .. } if (Component::isDeleted(c.clone())? || Component::isInvalid(c.clone())) => (),
        Deref @ Component::COMPONENT { binding: Deref @ Binding::UNTYPED_BINDING { .. }, attributes: __esc_attrs, .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() == ComponentState::Typed.clone()) => {
            attrs = (*__esc_attrs).clone();
            name = (InstNode::name(component)?).clone();
            binding = var_field!((*c).binding, Component::NFComponent::COMPONENT).clone();
            ErrorExt::setCheckpoint(literal!("NFTyping.typeComponentBinding"));
            match '__try0: {
                binding = unwrap_break_err!(typeBinding(binding.clone(), InstContext::set(context, InstContext::BINDING.clone())), '__try0);
                if !(InstContext::inAnnotation(context) && stringEq((name.clone()).clone(), (literal!("graphics")).clone()) || InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                    binding = unwrap_break_err!(TypeCheck::matchBinding(binding.clone(), var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), (name.clone()).clone(), node.clone(), context), '__try0);
                }
                comp_var = unwrap_break_err!(checkComponentBindingVariability((name.clone()).clone(), c.clone(), binding.clone(), context), '__try0);
                if comp_var != attrs.variability.clone() {
                    assign_field!(attrs.variability = comp_var);
                    assign_variant_field!(c => Component::NFComponent::COMPONENT; attributes = attrs.clone());
                }
                Ok::<_, anyhow::Error>((binding.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    binding = __try0_o0;
                }
                Err(_) => {
                    if Binding::isBound(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone()) || InstContext::inInstanceAPI(context) {
                        binding = Arc::new(Binding::NFBinding::INVALID_BINDING { binding: binding.clone(), errors: ErrorExt::getCheckpointMessages() });
                    } else {
                        ErrorExt::delCheckpoint(literal!("NFTyping.typeComponentBinding"));
                        bail!("fail");
                    }
                }
            }
            ErrorExt::delCheckpoint(literal!("NFTyping.typeComponentBinding"));
            assign_variant_field!(c => Component::NFComponent::COMPONENT;
                binding = binding,
                state = ComponentState::TypeChecked.clone()
            );
            InstNode::updateComponent(c.clone(), node)?;
            if typeChildren && !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                typeBindings(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context)?;
            }
            ()
        },
        Deref @ Component::COMPONENT { .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() >= ComponentState::Typed.clone()) => {
            if var_field!((*c).state, Component::NFComponent::COMPONENT).clone() == ComponentState::Typed.clone() {
                if Binding::isTyped(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone()) {
                    assign_variant_field!(c => Component::NFComponent::COMPONENT; binding = TypeCheck::matchBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), (InstNode::name(component.clone())?).clone(), node.clone(), context)?);
                    checkComponentBindingVariability((InstNode::name(component)?).clone(), c.clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context)?;
                }
                assign_variant_field!(c => Component::NFComponent::COMPONENT; state = ComponentState::TypeChecked.clone());
                InstNode::updateComponent(c.clone(), node)?;
            }
            if typeChildren && !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                typeBindings(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), context)?;
            }
            ()
        },
        Deref @ Component::COMPONENT { binding: Deref @ Binding::UNTYPED_BINDING { .. }, attributes: __esc_attrs, .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() < ComponentState::Typed.clone()) => {
            attrs = (*__esc_attrs).clone();
            name = (InstNode::name(component)?).clone();
            binding = typeBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), InstContext::set(context, InstContext::BINDING.clone()))?;
            comp_var = checkComponentBindingVariability((name).clone(), c.clone(), binding.clone(), context)?;
            if comp_var != attrs.variability.clone() {
                assign_field!(attrs.variability = comp_var);
                assign_variant_field!(c => Component::NFComponent::COMPONENT; attributes = attrs.clone());
            }
            assign_variant_field!(c => Component::NFComponent::COMPONENT; binding = binding);
            InstNode::updateComponent(c.clone(), node)?;
            ()
        },
        Deref @ Component::COMPONENT { .. } => (),
        Deref @ Component::ENUM_LITERAL { .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: Deref @ Modifier::NOMOD, .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { .. } => {
            assign_variant_field!(c => Component::NFComponent::TYPE_ATTRIBUTE; modifier = typeTypeAttribute(var_field!((*c).modifier, Component::NFComponent::TYPE_ATTRIBUTE).clone(), var_field!((*c).ty, Component::NFComponent::TYPE_ATTRIBUTE).clone(), component, context)?);
            InstNode::updateComponent(c.clone(), node)?;
            ()
        },
        Deref @ Component::INVALID_COMPONENT { .. } => (),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponentBinding")); __mm_s.push_str(&*literal!(" got invalid node ")); __mm_s.push_str(&*InstNode::name(node)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkComponentBindingVariability(mut name: ArcStr, mut component: Arc<Component::NFComponent>, mut binding: Arc<Binding::NFBinding>, mut context: i32) -> Result<Variability> {
    let mut var: Variability;
    let mut comp_var: Variability;
    let mut comp_eff_var: Variability;
    let mut bind_var: Variability;
    let mut bind_eff_var: Variability;
    comp_var = Component::variability(component.clone())?;
    comp_eff_var = Prefixes::effectiveVariability(comp_var);
    bind_var = Binding::variability(binding.clone())?;
    bind_eff_var = Prefixes::effectiveVariability(bind_var);
    if bind_eff_var > comp_eff_var && !(InstContext::inFunction(context)) {
        Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(name).clone(), (Prefixes::variabilityString(comp_eff_var)?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Binding::toString(Component::getBinding(component), (literal!("")).clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (Prefixes::variabilityString(bind_eff_var)?).clone()], Binding::getInfo(binding.clone()))?;
        if !(InstContext::inRelaxed(context)) {
            bail!("fail");
        }
    }
    if comp_var == Variability::PARAMETER.clone() && (bind_var == Variability::STRUCTURAL_PARAMETER.clone() && Binding::isCrefExp(binding) || bind_var == Variability::NON_STRUCTURAL_PARAMETER.clone()) {
        var = bind_var;
    } else {
        var = comp_var;
    }
    Ok(var)
}

pub fn typeBinding(mut binding: Arc<Binding::NFBinding>, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    binding = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::UNTYPED_BINDING { bindingExp: exp, .. } => {
            let mut ty: Arc<Type::NFType>;
            let mut var: Variability;
            let mut purity: Purity;
            let mut info: SourceInfo;
            let mut exp = (*exp).clone();
            info = Binding::getInfo(binding.clone());
            (exp, ty, var, purity) = typeExp(exp.clone(), context, info, false)?;
            Arc::new(Binding::NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty, variability: var, purity: purity, eachType: var_field!((*binding).eachType, Binding::NFBinding::UNTYPED_BINDING).clone(), evalState: Mutable::create(Binding::EvalState::NOT_EVALUATED.clone()), isFlattened: false, source: var_field!((*binding).source, Binding::NFBinding::UNTYPED_BINDING).clone(), confidence: var_field!((*binding).confidence, Binding::NFBinding::UNTYPED_BINDING).clone(), info: var_field!((*binding).info, Binding::NFBinding::UNTYPED_BINDING).clone() })
        },
        Deref @ Binding::TYPED_BINDING { .. } => {
            binding
        },
        Deref @ Binding::UNBOUND => {
            binding
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeBinding")); __mm_s.push_str(&*literal!(" got uninstantiated binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub(crate) fn typeComponentCondition(mut condition: Arc<Binding::NFBinding>, mut context: i32, mut evaluate: bool) -> Result<Arc<Binding::NFBinding>> {
    let mut condition: Arc<Binding::NFBinding> = condition;
    condition = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Binding::UNTYPED_BINDING { bindingExp: exp, .. } => {
            let mut ty: Arc<Type::NFType>;
            let mut var: Variability;
            let mut purity: Purity;
            let mut info: SourceInfo;
            let mut mk: MatchKind;
            let mut eval_state: Binding::EvalState;
            let mut next_context: i32;
            let mut exp = (*exp).clone();
            next_context = InstContext::set(context, InstContext::CONDITION.clone());
            info = Binding::getInfo(condition.clone());
            (exp, ty, var, purity) = typeExp(exp.clone(), next_context, info.clone(), false)?;
            (exp, _, mk) = TypeCheck::matchTypes(ty.clone(), crate::NFType::interned_BOOLEAN(), exp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Expression::toString(exp.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            if var > Variability::PARAMETER.clone() {
                Error::addSourceMessage(Error::COMPONENT_CONDITION_VARIABILITY.clone(), list![(Expression::toString(exp.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            eval_state = Binding::EvalState::NOT_EVALUATED.clone();
            if evaluate {
                ErrorExt::setCheckpoint(literal!("NFTyping.typeComponentCondition"));
                if '__try0: {
                    exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), Ceval::EvalTarget::new(info.clone(), next_context, None)), '__try0);
                    exp = unwrap_break_err!(simplifyDimExp(exp.clone()), '__try0);
                    eval_state = Binding::EvalState::EVALUATED.clone();
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
                ErrorExt::rollBack(literal!("NFTyping.typeComponentCondition"));
            }
            Arc::new(Binding::NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty, variability: var, purity: purity, eachType: Binding::EachType::NOT_EACH.clone(), evalState: Mutable::create(eval_state), isFlattened: false, source: var_field!((*condition).source, Binding::NFBinding::UNTYPED_BINDING).clone(), confidence: var_field!((*condition).confidence, Binding::NFBinding::UNTYPED_BINDING).clone(), info: info })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(condition)
}

pub(crate) fn typeTypeAttribute(mut attribute: Arc<Modifier::Modifier>, mut attrType: Arc<Type::NFType>, mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Modifier::Modifier>> {
    let mut attribute: Arc<Modifier::Modifier> = attribute;
    let mut name: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    attribute = (::match_deref::match_deref! { match &(attribute.clone()) {
        Deref @ Modifier::MODIFIER { .. } if (!(ModTable::isEmpty(var_field!((*attribute).subModifiers, Modifier::Modifier::MODIFIER).clone()))) => {
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*attribute).name, Modifier::Modifier::MODIFIER).clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Util::tuple21(listHead(ModTable::toList(var_field!((*attribute).subModifiers, Modifier::Modifier::MODIFIER).clone(), metamodelica::nil()))?)); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(name.clone()).clone(), (Type::toString(attrType)?).clone()], var_field!((*attribute).info, Modifier::Modifier::MODIFIER).clone())?;
            bail!("fail")
        },
        Deref @ Modifier::MODIFIER { .. } if (Binding::isUnbound(var_field!((*attribute).binding, Modifier::Modifier::MODIFIER).clone())) => crate::NFModifier::Modifier::interned_NOMOD(),
        Deref @ Modifier::MODIFIER { binding: Deref @ Binding::TYPED_BINDING { .. }, .. } => attribute.clone(),
        Deref @ Modifier::MODIFIER { name: __esc_name, binding: __esc_binding, .. } => {
            name = (*__esc_name).clone();
            binding = (*__esc_binding).clone();
            if Binding::isBound(binding.clone()) {
                binding = typeBinding(binding.clone(), context)?;
                parent = InstNode::parent(component);
                binding = TypeCheck::matchBinding(binding.clone(), attrType, (name.clone()).clone(), parent, context)?;
                if Binding::variability(binding.clone())? >= Variability::DISCRETE.clone() && !(InstContext::inFunction(context)) {
                    Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(name.clone()).clone(), (Prefixes::variabilityString(Variability::PARAMETER.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Binding::toString(binding.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (Prefixes::variabilityString(Binding::variability(binding.clone())?)?).clone()], Binding::getInfo(binding.clone()))?;
                    bail!("fail");
                }
                assign_variant_field!(attribute => Modifier::Modifier::MODIFIER; binding = binding.clone());
            }
            attribute.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(attribute)
}

pub fn typeExp(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    (exp, ty, variability, purity) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            (exp, crate::NFType::interned_INTEGER(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::REAL { .. } => {
            (exp, crate::NFType::interned_REAL(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::STRING { .. } => {
            (exp, crate::NFType::interned_STRING(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (exp, crate::NFType::interned_BOOLEAN(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            (exp.clone(), var_field!((*exp).ty, Expression::NFExpression::ENUM_LITERAL).clone(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::CREF { .. } => {
            typeCrefExp(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), context, info)?
        },
        Deref @ Expression::TYPENAME { .. } => {
            if !(InstContext::inValidTypenameScope(context)) {
                Error::addSourceMessage(Error::INVALID_TYPENAME_USE.clone(), list![(Type::typenameString(Type::arrayElementType(var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone()))?).clone()], info)?;
                bail!("fail");
            }
            (exp.clone(), var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::ARRAY { .. } => {
            typeArray(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone(), var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone(), context, info)?
        },
        Deref @ Expression::MATRIX { .. } => {
            typeMatrix(var_field!((*exp).elements, Expression::NFExpression::MATRIX).clone(), context, info)?
        },
        Deref @ Expression::RANGE { .. } => {
            typeRange(exp, context, info)?
        },
        Deref @ Expression::TUPLE { .. } => {
            typeTuple(var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone(), context, info)?
        },
        Deref @ Expression::SIZE { .. } => {
            typeSize(exp, context, info, true)?
        },
        Deref @ Expression::END => {
            Error::addSourceMessage(Error::END_ILLEGAL_USE_ERROR.clone(), metamodelica::nil(), info)?;
            bail!("fail")
        },
        Deref @ Expression::BINARY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut var1: Variability;
            let mut var2: Variability;
            let mut pur1: Purity;
            let mut pur2: Purity;
            let mut ty1: Arc<Type::NFType>;
            let mut ty2: Arc<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), next_context, info.clone(), false)?;
            (e2, ty2, var2, pur2) = typeExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), next_context, info.clone(), false)?;
            (exp, ty) = TypeCheck::checkBinaryOperation(e1, ty1, var1, var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), e2, ty2, var2, context, info, retype)?;
            (exp, ty, Prefixes::variabilityMax(var1, var2), Prefixes::purityMin(pur1, pur2))
        },
        Deref @ Expression::UNARY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut var1: Variability;
            let mut pur1: Purity;
            let mut ty1: Arc<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), next_context, info.clone(), false)?;
            (exp, ty) = TypeCheck::checkUnaryOperation(e1, ty1, var1, var_field!((*exp).operator, Expression::NFExpression::UNARY).clone(), context, info)?;
            (exp, ty, var1, pur1)
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut var1: Variability;
            let mut var2: Variability;
            let mut pur1: Purity;
            let mut pur2: Purity;
            let mut ty1: Arc<Type::NFType>;
            let mut ty2: Arc<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), next_context, info.clone(), false)?;
            (e2, ty2, var2, pur2) = typeExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), next_context, info.clone(), false)?;
            (exp, ty) = TypeCheck::checkLogicalBinaryOperation(e1, ty1, var1, var_field!((*exp).operator, Expression::NFExpression::LBINARY).clone(), e2, ty2, var2, context, info)?;
            (exp, ty, Prefixes::variabilityMax(var1, var2), Prefixes::purityMin(pur1, pur2))
        },
        Deref @ Expression::LUNARY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut var1: Variability;
            let mut pur1: Purity;
            let mut ty1: Arc<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), next_context, info.clone(), false)?;
            (exp, ty) = TypeCheck::checkLogicalUnaryOperation(e1, ty1, var1, var_field!((*exp).operator, Expression::NFExpression::LUNARY).clone(), context, info)?;
            (exp, ty, var1, pur1)
        },
        Deref @ Expression::RELATION { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut var1: Variability;
            let mut var2: Variability;
            let mut pur1: Purity;
            let mut pur2: Purity;
            let mut ty1: Arc<Type::NFType>;
            let mut ty2: Arc<Type::NFType>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty1, var1, pur1) = typeExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), next_context, info.clone(), false)?;
            (e2, ty2, var2, pur2) = typeExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), next_context, info.clone(), false)?;
            (exp, ty) = TypeCheck::checkRelationOperation(e1, ty1, var1, var_field!((*exp).operator, Expression::NFExpression::RELATION).clone(), e2, ty2, var2, var_field!((*exp).index, Expression::NFExpression::RELATION).clone(), context, info)?;
            variability = Prefixes::variabilityMax(var1, var2);
            purity = Prefixes::purityMin(pur1, pur2);
            if !(InstContext::inNoEvent(context)) && variability == Variability::CONTINUOUS.clone() {
                variability = Variability::DISCRETE.clone();
            }
            (exp, ty, variability, purity)
        },
        Deref @ Expression::IF { .. } => {
            typeIfExpression(exp, context, info)?
        },
        Deref @ Expression::RECORD { .. } => {
            typeRecordExp(exp, context, info)?
        },
        Deref @ Expression::CALL { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut var1: Variability;
            let mut pur1: Purity;
            (e1, ty, var1, pur1) = Call::typeCall(exp, context, info, retype)?;
            if Type::isTuple(ty.clone()) && !(InstContext::isSingleExpression(context)) {
                ty = Type::firstTupleType(ty)?;
                e1 = Expression::tupleElement(e1, ty.clone(), 1)?;
            }
            (e1, ty, var1, pur1)
        },
        Deref @ Expression::CAST { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            (e1, ty, variability, purity) = typeExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), next_context, info, retype)?;
            assign_variant_field!(exp => Expression::NFExpression::CAST;
                exp = e1,
                ty = Type::copyDims(ty, var_field!((*exp).ty, Expression::NFExpression::CAST).clone())
            );
            (exp.clone(), var_field!((*exp).ty, Expression::NFExpression::CAST).clone(), variability, purity)
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            typeSubscriptedExp(exp, context, info)?
        },
        Deref @ Expression::MUTABLE { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            e1 = Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone());
            (e1, ty, variability, purity) = typeExp(e1, context, info, retype)?;
            assign_variant_field!(exp => Expression::NFExpression::MUTABLE; exp = Mutable::create(e1));
            (exp, ty, variability, purity)
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            Function::typePartialApplication(exp, context, info)?
        },
        Deref @ Expression::FILENAME { .. } => {
            (exp, crate::NFType::interned_STRING(), Variability::CONSTANT.clone(), Purity::PURE.clone())
        },
        Deref @ Expression::MULTARY { .. } => {
            typeExp(SimplifyExp::splitMultary(exp)?, context, info, retype)?
        },
        _ => {
            (exp.clone(), Expression::typeOf(exp.clone()), Expression::variability(exp.clone())?, Expression::purity(exp)?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if InstContext::inDiscreteScope(context) && variability == Variability::CONTINUOUS.clone() {
        variability = Variability::DISCRETE.clone();
    }
    Ok((exp, ty, variability, purity))
}

pub(crate) fn typeExpl(mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Arc<Type::NFType>>>, Arc<metamodelica::List<Variability>>)> {
    let mut explTyped: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut tyl: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut varl: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression>;
    let mut var: Variability;
    let mut ty: Arc<Type::NFType>;
    for mut e in &*expl.reverse() {
        let mut e = e.clone();
        (exp, ty, var, _) = typeExp(e.clone(), context, info.clone(), false)?;
        explTyped = metamodelica::cons(exp.clone(), explTyped.clone());
        tyl = metamodelica::cons(ty.clone(), tyl.clone());
        varl = metamodelica::cons(var, varl.clone());
    }
    Ok((explTyped, tyl, varl))
}

pub(crate) fn typeRecordExp(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType>;
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut path: Arc<Absyn::Path>;
    let mut elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut ty_elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut next_context: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::RECORD { path: __pa0, ty: __pa1, elements: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    ty = __pa1.clone();
    elems = __pa2.clone();
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    for mut e in &*elems {
        let mut e = e.clone();
        (e, _, var, pur) = typeExp(e.clone(), context, info.clone(), false)?;
        variability = Prefixes::variabilityMax(var, variability);
        purity = Prefixes::purityMin(pur, purity);
        ty_elems = metamodelica::cons(e.clone(), ty_elems.clone());
    }
    exp = Expression::makeRecord(path, ty.clone(), metamodelica::Dangerous::listReverseInPlace(ty_elems));
    Ok((exp, ty, variability, purity))
}

pub(crate) fn typeSubscriptedExp(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut e: Arc<Expression::NFExpression>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut expanded_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut fill_dims: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut split: bool;
    let mut subs_var: Variability;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, ty: __pa2, split: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    subs = __pa1.clone();
    ty = __pa2.clone();
    split = __pa3.clone();
    if split {
        (expanded_subs, fill_dims) = expandProxySubscripts(subs, context)?;
        (exp, ty, variability, purity) = typeSubscriptedExp2(e, expanded_subs.clone(), context, info)?;
        if !(fill_dims.clone().is_empty()) {
            fill_dims = metamodelica::Dangerous::listReverseInPlace(fill_dims);
            ty = Type::liftArrayLeftList(ty, ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (fill_dims.clone()).into_iter().cloned() {
            let __x = Dimension::fromExp(d.clone(), Variability::CONSTANT.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FILL_FUNC().clone(), metamodelica::cons(exp, fill_dims), variability, purity, ty.clone()) });
        }
        if !(expanded_subs.clone().is_empty()) {
            ty = Type::subscript(ty, expanded_subs.clone(), false)?;
            if Type::isUnknown(ty.clone()) {
                exp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: exp, subscripts: expanded_subs.clone(), ty: ty.clone(), split: true });
            } else {
                exp = Expression::applySubscripts(expanded_subs.clone(), exp, false)?;
            }
            if purity == Purity::PURE.clone() {
                purity = Subscript::purityList(expanded_subs.clone())?;
            }
            if variability != Variability::CONTINUOUS.clone() {
                variability = Prefixes::variabilityMax(variability, Subscript::variabilityList(expanded_subs)?);
            }
        }
    } else {
        (e, ty, variability, purity) = typeExp(e, context, info.clone(), false)?;
        (subs, subs_var) = typeSubscripts(subs, ty.clone(), exp, context, info, true)?;
        ty = Type::subscript(ty, subs.clone(), true)?;
        exp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: e, subscripts: subs, ty: ty.clone(), split: false });
    }
    Ok((exp, ty, variability, purity))
}

pub(crate) fn expandProxySubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut context: i32) -> Result<(Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut fillDimensions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut dim_count: i32 = 0;
    let mut cr_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    for mut s in &*subscripts {
        let mut s = s.clone();
        outSubscripts = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SPLIT_PROXY { .. } => {
            dim_count = InstNode::dimensionCount(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone());
            for mut i in 1..=dim_count {
                outSubscripts = metamodelica::cons(Subscript::makeSplitIndex(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone(), i.clone())?, outSubscripts.clone());
            }
            if !(InstNode::refEqual(var_field!((*s).origin, Subscript::NFSubscript::SPLIT_PROXY).clone(), var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone())) {
                dim_count = dim_count - InstNode::dimensionCount(var_field!((*s).origin, Subscript::NFSubscript::SPLIT_PROXY).clone());
                if dim_count > 0 {
                    ty = InstNode::getType(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone())?;
                    cr_exp = Expression::fromCref(ComponentRef::fromNode(var_field!((*s).parent, Subscript::NFSubscript::SPLIT_PROXY).clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()), false)?;
                    dims = Type::arrayDims(ty.clone());
                    for mut i in 1..=dim_count {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        dim = __pa0.clone();
                        dims = __pa1.clone();
                        if Dimension::isKnown(dim.clone(), true) {
                            fillDimensions = metamodelica::cons(Dimension::sizeExp(dim.clone())?, fillDimensions.clone());
                        } else {
                            fillDimensions = metamodelica::cons(Arc::new(Expression::NFExpression::SIZE { exp: cr_exp.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) }), fillDimensions.clone());
                        }
                    }
                }
            }
            outSubscripts.clone()
        },
        _ => metamodelica::cons(s.clone(), outSubscripts.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outSubscripts = List::trim(outSubscripts, (std::sync::Arc::new(fnptr!(Subscript::isWhole, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>))?;
    outSubscripts = metamodelica::Dangerous::listReverseInPlace(outSubscripts);
    Ok((outSubscripts, fillDimensions))
}

pub(crate) fn typeSubscriptedExp2(mut exp: Arc<Expression::NFExpression>, mut splitSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    (outExp, ty, variability, purity) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } if (!(splitSubs.clone().is_empty()) && !(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty())) => {
            expl = metamodelica::nil();
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            let __range0 = var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                (e, ty, variability, purity) = typeSubscriptedExp2(e.clone(), listRest(splitSubs.clone())?, context, info.clone())?;
                expl = metamodelica::cons(e.clone(), expl.clone());
            }
            expl = metamodelica::Dangerous::listReverseInPlace(expl);
            ty = Type::liftArrayLeft(ty, Dimension::fromInteger((expl.clone().len() as i32), Prefixes::Variability::CONSTANT.clone()));
            outExp = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(expl.into_iter().cloned().collect()), var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone());
            (outExp, ty, variability, purity)
        },
        _ => typeExp(exp.clone(), context, info, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, ty, variability, purity))
}

pub(crate) fn typeExpDim(mut exp: Arc<Expression::NFExpression>, mut dimIndex: i32, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Dimension::NFDimension>, Option<Arc<Expression::NFExpression>>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension>;
    let mut typedExp: Option<Arc<Expression::NFExpression>> = None;
    let mut error: Arc<TypingError::TypingError>;
    let mut ty: Arc<Type::NFType>;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut next_context: i32;
    ty = Expression::typeOf(exp.clone());
    if Type::isKnown(ty.clone()) {
        (dim, error) = nthDimensionBoundsChecked(ty, dimIndex, 0)?;
        typedExp = Some(exp.clone());
        if !(Dimension::isUnknown(dim.clone())) {
            return Ok((dim.clone(), typedExp.clone(), error.clone()));
        }
    }
    next_context = InstContext::clearExpFlags(context);
    (dim, error) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::UNKNOWN, .. } => typeArrayDim(exp, dimIndex)?,
        Deref @ Expression::CREF { .. } => typeCrefDim(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), dimIndex, next_context, info)?,
        _ => {
            (e, ty, _, _) = typeExp(exp, next_context, info.clone(), false)?;
            if Type::isTuple(ty.clone()) {
                ty = Type::firstTupleType(ty)?;
                e = Expression::tupleElement(e, ty.clone(), 1)?;
            }
            if Type::isConditionalArray(ty.clone()) {
                e = Expression::map(e, (std::sync::Arc::new({ let __pe_b1 = Ceval::EvalTarget::new(info.clone(), next_context, None); move |__pe_a0| evaluateArrayIf(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                (e, ty, _, _) = typeExp(e, next_context, info, false)?;
            }
            typedExp = Some(e);
            nthDimensionBoundsChecked(ty, dimIndex, 0)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, typedExp, error))
}

pub(crate) fn evaluateArrayIf(mut exp: Arc<Expression::NFExpression>, mut target: Arc<Ceval::EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { .. } if (Type::isConditionalArray(var_field!((*exp).ty, Expression::NFExpression::IF).clone())) => {
            let mut cond: Arc<Expression::NFExpression>;
            cond = Ceval::evalExp(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), target.clone())?;
            if Expression::isTrue(cond.clone()) {
                outExp = var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone();
            } else if Expression::isFalse(cond) {
                outExp = var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone();
            } else {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.evaluateArrayIf")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), Ceval::EvalTarget::getInfo(target))?;
                bail!("fail");
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

pub(crate) fn typeArrayDim(mut arrayExp: Arc<Expression::NFExpression>, mut dimIndex: i32) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension>;
    let mut error: Arc<TypingError::TypingError>;
    if dimIndex < 1 {
        dim = crate::NFDimension::interned_UNKNOWN();
        error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: Expression::dimensionCount(arrayExp)? });
    } else {
        (dim, error) = typeArrayDim2(arrayExp, dimIndex, 0)?;
    }
    Ok((dim, error))
}

pub(crate) fn typeArrayDim2(mut arrayExp: Arc<Expression::NFExpression>, mut dimIndex: i32, mut dimCount: i32) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    (dim, error) = (::match_deref::match_deref! { match &((arrayExp.clone(), dimIndex)) {
        (Deref @ Expression::ARRAY { .. }, 1) => (Dimension::fromExpArray(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone()), crate::NFTyping::TypingError::interned_NO_ERROR()),
        (Deref @ Expression::ARRAY { .. }, _) => typeArrayDim2(metamodelica::arrayGet(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), 1)?, dimIndex - 1, dimCount + 1)?,
        _ => {
            dim = crate::NFDimension::interned_UNKNOWN();
            error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dimCount });
            (dim, error)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, error))
}

pub(crate) fn typeCrefDim(mut cref: Arc<ComponentRef::NFComponentRef>, mut dimIndex: i32, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut error: Arc<TypingError::TypingError> = crate::NFTyping::TypingError::interned_NO_ERROR();
    let mut crl: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut index: i32;
    let mut dim_count: i32 = 0;
    let mut dim_total: i32 = 0;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut ty: Arc<Type::NFType>;
    let mut dims: metamodelica::Array<Arc<Dimension::NFDimension>> = Default::default();
    if ComponentRef::hasSubscripts(cref.clone())? {
        (_, ty, _, _) = typeCref(cref, context, info)?;
        (dim, error) = nthDimensionBoundsChecked(ty, dimIndex, 0)?;
        return Ok((dim.clone(), error.clone()));
    }
    crl = ComponentRef::toListReverse(cref, false, metamodelica::nil());
    index = dimIndex;
    for mut cr in &*crl {
        let mut cr = cr.clone();
        let () = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::COMPONENT_NODE { .. }, subscripts: _, .. } => {
            node = InstNode::resolveOuter(var_field!((*cr).node, ComponentRef::NFComponentRef::CREF).clone());
            c = InstNode::component(node.clone())?;
            if Class::hasDimensions(InstNode::getClass(Component::classInstance(c.clone()))?)? {
                typeComponent(node.clone(), context, true)?;
                c = InstNode::component(node.clone())?;
            }
            dim_count = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: __esc_dims, .. }, .. } => {
            dims = (*__esc_dims).clone();
            dim_count = metamodelica::arrayLength(dims.clone());
            if index <= dim_count && index > 0 {
                dim = typeDimension(dims.clone(), index, node.clone(), var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context, var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
                checkCyclicDimension(dim.clone(), node.clone(), index, var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
                return Ok((dim.clone(), error.clone()));
            }
            dim_count
        },
        Deref @ Component::COMPONENT { .. } => {
            dim_count = Type::dimensionCount(var_field!((*c).ty, Component::NFComponent::COMPONENT).clone());
            if index <= dim_count && index > 0 {
                dim = Type::nthDimension(var_field!((*c).ty, Component::NFComponent::COMPONENT).clone(), index)?;
                return Ok((dim.clone(), error.clone()));
            }
            dim_count
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            index = index - dim_count;
            dim_total = dim_total + dim_count;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    dim = crate::NFDimension::interned_UNKNOWN();
    error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dim_total });
    Ok((dim, error))
}

pub(crate) fn checkCyclicDimension(mut dim: Arc<Dimension::NFDimension>, mut component: Arc<InstNode::InstNode>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::UNTYPED { isProcessing: true, .. } => {
            Error::addSourceMessage(Error::CYCLIC_DIMENSIONS.clone(), list![ArcStr::from(::std::format!("{}", index)), (InstNode::name(component)?).clone(), (Expression::toString(var_field!((*dim).dimension, Dimension::NFDimension::UNTYPED).clone())?).clone()], info)?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn nthDimensionBoundsChecked(mut ty: Arc<Type::NFType>, mut dimIndex: i32, mut offset: i32) -> Result<(Arc<Dimension::NFDimension>, Arc<TypingError::TypingError>)> {
    let mut dim: Arc<Dimension::NFDimension>;
    let mut error: Arc<TypingError::TypingError>;
    let mut dim_size: i32 = Type::dimensionCount(ty.clone());
    let mut index: i32 = dimIndex + offset;
    if index < 1 || index > dim_size {
        dim = crate::NFDimension::interned_UNKNOWN();
        error = Arc::new(TypingError::TypingError::OUT_OF_BOUNDS { upperBound: dim_size - offset });
    } else {
        dim = Type::nthDimension(ty, index)?;
        error = crate::NFTyping::TypingError::interned_NO_ERROR();
    }
    Ok((dim, error))
}

pub(crate) fn typeCrefExp(mut cref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut cr: Arc<ComponentRef::NFComponentRef>;
    let mut node_var: Variability;
    let mut subs_var: Variability;
    (cr, ty, node_var, subs_var) = typeCref(cref.clone(), context, info)?;
    exp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: cr });
    variability = Prefixes::variabilityMax(node_var, subs_var);
    purity = ComponentRef::purity(cref)?;
    Ok((exp, ty, variability, purity))
}

pub(crate) fn typeCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<Type::NFType>, Variability, Variability)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut ty: Arc<Type::NFType>;
    let mut nodeVariability: Variability;
    let mut subsVariability: Variability;
    if InstContext::inFunction(context) && ComponentRef::isTime(cref.clone())? {
        Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("time")).clone()], info.clone())?;
        bail!("fail");
    }
    (cref, subsVariability) = typeCref2(cref, context, info, true)?;
    if ComponentRef::hasImplicitTrailingIndex(cref.clone()) {
        cref = ComponentRef::fillSubscripts(cref);
    }
    ty = ComponentRef::getSubscriptedType(cref.clone(), false)?;
    nodeVariability = ComponentRef::nodeVariability(cref.clone())?;
    Ok((cref, ty, nodeVariability, subsVariability))
}

pub(crate) fn typeCref2(mut cref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo, mut firstPart: bool) -> Result<(Arc<ComponentRef::NFComponentRef>, Variability)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut subsVariability: Variability = Variability::CONSTANT;
    (cref, subsVariability) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::SCOPE, .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF;
                ty = InstNode::getType(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?,
                restCref = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context, info, false)?.0
            );
            (cref.clone(), Variability::CONSTANT.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::COMPONENT_NODE { .. }, .. } => {
            let mut rest_cr: Arc<ComponentRef::NFComponentRef>;
            let mut node_ty: Arc<Type::NFType>;
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
            let mut subs_var: Variability;
            let mut rest_var: Variability;
            node_ty = typeComponent(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), InstContext::nodeContext(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), context), firstPart || !(InstContext::inDimension(context)))?;
            (subs, subs_var) = typeSubscripts(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), node_ty.clone(), Arc::new(Expression::NFExpression::CREF { ty: node_ty.clone(), cref: cref.clone() }), context, info.clone(), true)?;
            (rest_cr, rest_var) = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context, info, false)?;
            subsVariability = Prefixes::variabilityMax(subs_var, rest_var);
            (Arc::new(ComponentRef::NFComponentRef::CREF { node: var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), subscripts: subs, ty: node_ty, origin: var_field!((*cref).origin, ComponentRef::NFComponentRef::CREF).clone(), restCref: rest_cr }), subsVariability)
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::CLASS_NODE { .. }, .. } if (firstPart && InstNode::isFunction(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?) => {
            let mut r#fn: Arc<Function::Function>;
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF;
                ty = Arc::new(Type::NFType::FUNCTION { r#fn: r#fn, fnType: Type::FunctionType::FUNCTION_REFERENCE.clone() }),
                restCref = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context, info, false)?.0
            );
            (cref.clone(), Variability::CONSTANT.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::CLASS_NODE { .. }, .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; ty = InstNode::getType(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?);
            (cref.clone(), Variability::CONSTANT.clone())
        },
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::NAME_NODE { .. }, .. } => {
            let mut rest_cr: Arc<ComponentRef::NFComponentRef>;
            let mut subs_var: Variability;
            let mut rest_var: Variability;
            (_, subs_var) = typeSubscripts(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone(), Arc::new(Expression::NFExpression::CREF { ty: var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone(), cref: cref.clone() }), context, info.clone(), false)?;
            (rest_cr, rest_var) = typeCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), context, info, false)?;
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cr);
            subsVariability = Prefixes::variabilityMax(subs_var, rest_var);
            (cref.clone(), rest_var)
        },
        _ => {
            (cref.clone(), Variability::CONSTANT.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, subsVariability))
}

pub(crate) fn typeSubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut crefType: Arc<Type::NFType>, mut subscriptedExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut checkSubscripts: bool) -> Result<(Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, Variability)> {
    let mut typedSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut dim: Arc<Dimension::NFDimension>;
    let mut next_context: i32;
    let mut i: i32;
    let mut sub: Arc<Subscript::NFSubscript>;
    let mut var: Variability;
    if subscripts.clone().is_empty() {
        typedSubs = subscripts;
        return Ok((typedSubs.clone(), variability.clone()));
    }
    dims = Type::arrayDims(crefType);
    typedSubs = metamodelica::nil();
    next_context = InstContext::set(context, InstContext::SUBSCRIPT.clone());
    i = 1;
    if (subscripts.clone().len() as i32) > (dims.clone().len() as i32) && checkSubscripts {
        Error::addSourceMessage(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone(), list![(Expression::toString(subscriptedExp.clone())?).clone(), ArcStr::from(::std::format!("{}", (subscripts.clone().len() as i32))), ArcStr::from(::std::format!("{}", (dims.clone().len() as i32)))], info.clone())?;
        bail!("fail");
    }
    for mut s in &*subscripts {
        let mut s = s.clone();
        if checkSubscripts {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim = __pa0.clone();
            dims = __pa1.clone();
        } else {
            dim = crate::NFDimension::interned_UNKNOWN();
        }
        (sub, var) = typeSubscript(s.clone(), dim.clone(), subscriptedExp.clone(), i, next_context, info.clone(), checkSubscripts)?;
        typedSubs = metamodelica::cons(sub.clone(), typedSubs.clone());
        variability = Prefixes::variabilityMax(variability, var);
        i = i + 1;
        if var == Variability::PARAMETER.clone() {
            Structural::markSubscript(sub.clone())?;
        }
    }
    typedSubs = metamodelica::Dangerous::listReverseInPlace(typedSubs);
    Ok((typedSubs, variability))
}

pub(crate) fn typeSubscript(mut subscript: Arc<Subscript::NFSubscript>, mut dimension: Arc<Dimension::NFDimension>, mut subscriptedExp: Arc<Expression::NFExpression>, mut index: i32, mut context: i32, mut info: SourceInfo, mut checkSubscript: bool) -> Result<(Arc<Subscript::NFSubscript>, Variability)> {
    let mut outSubscript: Arc<Subscript::NFSubscript> = subscript.clone();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: crate::NFType::interned_UNKNOWN() });
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matched_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    (ty, variability) = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::UNTYPED { .. } => {
            e = evaluateEnd(var_field!((*subscript).exp, Subscript::NFSubscript::UNTYPED).clone(), dimension.clone(), subscriptedExp, index, context, info.clone())?;
            (e, ty, variability, _) = typeExp(e, context, info.clone(), false)?;
            if Type::isArray(ty.clone()) && InstContext::inEquation(context) {
                Structural::markExp(e.clone())?;
                e = Ceval::tryEvalExp(e, Ceval::noTarget().clone());
                ty = Expression::typeOf(e.clone());
            }
            if checkSubscript {
                (e, matched_ty) = checkSubscriptType(e, Type::arrayElementType(ty.clone()), dimension, info)?;
            } else {
                matched_ty = ty.clone();
            }
            outSubscript = if (Type::isArray(ty)) {Arc::new(Subscript::NFSubscript::SLICE { slice: e.clone() })} else {Arc::new(Subscript::NFSubscript::INDEX { index: e.clone() })};
            (matched_ty, variability)
        },
        Deref @ Subscript::INDEX { index: __esc_e } => {
            e = (*__esc_e).clone();
            if checkSubscript {
                (e, ty) = checkSubscriptType(e.clone(), Expression::typeOf(e.clone()), dimension, info)?;
            } else {
                ty = Expression::typeOf(e.clone());
            }
            outSubscript = Arc::new(Subscript::NFSubscript::INDEX { index: e.clone() });
            (ty, Expression::variability(e.clone())?)
        },
        Deref @ Subscript::SLICE { slice: __esc_e } => {
            e = (*__esc_e).clone();
            if checkSubscript {
                (e, ty) = checkSubscriptType(e.clone(), Type::unliftArray(Expression::typeOf(e.clone()))?, dimension, info)?;
            } else {
                ty = Type::unliftArray(Expression::typeOf(e.clone()))?;
            }
            outSubscript = Arc::new(Subscript::NFSubscript::SLICE { slice: e.clone() });
            (ty, Expression::variability(e.clone())?)
        },
        Deref @ Subscript::WHOLE => (crate::NFType::interned_UNKNOWN(), Dimension::variability(dimension)?),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeSubscript")); __mm_s.push_str(&*literal!(" got unknown subscript")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubscript, variability))
}

pub(crate) fn checkSubscriptType(mut subscriptExp: Arc<Expression::NFExpression>, mut subscriptType: Arc<Type::NFType>, mut dimension: Arc<Dimension::NFDimension>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut subscriptExp: Arc<Expression::NFExpression> = subscriptExp;
    let mut outType: Arc<Type::NFType>;
    let mut expected_ty: Arc<Type::NFType>;
    let mut mk: MatchKind;
    expected_ty = Dimension::subscriptType(dimension);
    (subscriptExp, outType, mk) = TypeCheck::matchTypes(subscriptType.clone(), expected_ty.clone(), subscriptExp, TypeCheck::ALLOW_UNKNOWN.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(Error::SUBSCRIPT_TYPE_MISMATCH.clone(), list![(Expression::toString(subscriptExp.clone())?).clone(), (Type::toString(subscriptType)?).clone(), (Type::toString(expected_ty)?).clone()], info)?;
        bail!("fail");
    }
    Ok((subscriptExp, outType))
}

pub(crate) fn typeArray(mut elements: metamodelica::Array<Arc<Expression::NFExpression>>, mut isLiteral: bool, mut ty: Arc<Type::NFType>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut arrayExp: Arc<Expression::NFExpression>;
    let mut arrayType: Arc<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: Arc<Expression::NFExpression>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut ty1: Arc<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty2: Arc<Type::NFType>;
    let mut ty3: Arc<Type::NFType>;
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut mk: MatchKind;
    let mut array_len: i32;
    let mut idx: i32;
    let mut next_context: i32;
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    array_len = metamodelica::arrayLength(elements.clone());
    if array_len > 0 {
        (exp, ty1, variability, purity) = typeExp(metamodelica::arrayGet(elements.clone(), 1)?, next_context, info.clone(), false)?;
        expl = metamodelica::cons(exp.clone(), expl);
        tys = metamodelica::cons(ty1.clone(), tys);
        for mut i in 2..=array_len {
            (exp, ty2, var, pur) = typeExp(metamodelica::arrayGet(elements.clone(), i.clone())?, next_context, info.clone(), false)?;
            variability = Prefixes::variabilityMax(var, variability);
            purity = Prefixes::purityMin(pur, purity);
            (_, ty3, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), exp.clone(), TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk) {
                (_, ty3, mk) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), exp.clone(), TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone())?;
                if TypeCheck::isCompatibleMatch(mk) {
                    ty1 = ty3.clone();
                }
            } else {
                ty1 = ty3.clone();
            }
            expl = metamodelica::cons(exp.clone(), expl.clone());
            tys = metamodelica::cons(ty2.clone(), tys.clone());
        }
    } else {
        ty1 = Type::arrayElementType(ty);
    }
    idx = array_len;
    for mut e in &*expl {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty2 = __pa0.clone();
        tys = __pa1.clone();
        (exp, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), e.clone(), TypeCheck::IGNORE_DIMENSIONS_IN_RECORDS.clone())?;
        expl2 = metamodelica::cons(exp.clone(), expl2.clone());
        if !(InstContext::inAnnotation(context)) {
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessage(Error::NF_ARRAY_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", idx)), (Expression::toString(exp.clone())?).clone(), (Type::toString(ty2.clone())?).clone(), (Type::toString(ty1.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
        }
        idx = idx - 1;
    }
    arrayType = Type::liftArrayLeft(ty1, Dimension::fromExpList(expl2.clone()));
    arrayExp = Expression::makeArray(arrayType.clone(), metamodelica::arrayFromVec(expl2.into_iter().cloned().collect()), isLiteral);
    Ok((arrayExp, arrayType, variability, purity))
}

pub(crate) fn typeMatrix(mut elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut arrayExp: Arc<Expression::NFExpression>;
    let mut arrayType: Arc<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: Arc<Expression::NFExpression>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut ty: Arc<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut resTys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut n: i32 = 2;
    let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    if (elements.clone().len() as i32) > 1 {
        for mut el in &*elements {
            let mut el = el.clone();
            (exp, ty, var, pur) = typeMatrixComma(el.clone(), next_context, info.clone())?;
            variability = Prefixes::variabilityMax(var, variability);
            purity = Prefixes::purityMin(pur, purity);
            expl = metamodelica::cons(exp.clone(), expl.clone());
            tys = metamodelica::cons(ty.clone(), tys.clone());
            n = std::cmp::max(n, Type::dimensionCount(ty.clone()));
        }
        for mut e in &*expl {
            let mut e = e.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            tys = __pa1.clone();
            (e, ty) = Expression::promote(e.clone(), ty.clone(), n)?;
            resTys = metamodelica::cons(ty.clone(), resTys.clone());
            res = metamodelica::cons(e.clone(), res.clone());
        }
        (arrayExp, arrayType) = BuiltinCall::makeCatExp(1, res, resTys, variability, purity, info)?;
    } else {
        (arrayExp, arrayType, variability, purity) = typeMatrixComma(listHead(elements)?, next_context, info)?;
        if Type::dimensionCount(arrayType.clone()) < 2 {
            (arrayExp, arrayType) = Expression::promote(arrayExp, arrayType, n)?;
        }
    }
    Ok((arrayExp, arrayType, variability, purity))
}

pub(crate) fn typeMatrixComma(mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut arrayExp: Arc<Expression::NFExpression>;
    let mut arrayType: Arc<Type::NFType>;
    let mut variability: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut exp: Arc<Expression::NFExpression>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut var: Variability;
    let mut pur: Purity;
    let mut ty: Arc<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut ty3: Arc<Type::NFType>;
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut tys2: Arc<metamodelica::List<Arc<Type::NFType>>>;
    let mut n: i32 = 2;
    let mut pos: i32;
    let mut mk: MatchKind;
    Error::assertion(!(elements.clone().is_empty()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeMatrixComma")); __mm_s.push_str(&*literal!(" expected non-empty arguments")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
    if (elements.clone().len() as i32) > 1 {
        for mut e in &*elements {
            let mut e = e.clone();
            (exp, ty1, var, pur) = typeExp(e.clone(), context, info.clone(), false)?;
            expl = metamodelica::cons(exp.clone(), expl.clone());
            if Type::isEqual(ty.clone(), crate::NFType::interned_UNKNOWN())? {
                ty = ty1.clone();
            } else {
                (_, _, ty2, mk) = TypeCheck::matchExpressions(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Type::arrayElementType(ty1.clone()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Type::arrayElementType(ty.clone()), TypeCheck::DEFAULT_OPTIONS.clone())?;
                if TypeCheck::isCompatibleMatch(mk) {
                    ty = ty2.clone();
                }
            }
            tys = metamodelica::cons(ty1.clone(), tys.clone());
            variability = Prefixes::variabilityMax(variability, var);
            purity = Prefixes::purityMin(purity, pur);
            n = std::cmp::max(n, Type::dimensionCount(ty.clone()));
        }
        tys2 = metamodelica::nil();
        res = metamodelica::nil();
        pos = n + 1;
        for mut e in &*expl {
            let mut e = e.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty1 = __pa0.clone();
            tys = __pa1.clone();
            pos = pos - 1;
            if Type::dimensionCount(ty1.clone()) != n {
                (e, ty1) = Expression::promote(e.clone(), ty1.clone(), n)?;
            }
            ty2 = Type::setArrayElementType(ty1.clone(), ty.clone());
            (e, ty3, mk) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), e.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", pos)), (literal!("matrix constructor ")).clone(), (literal!("arg")).clone(), (Expression::toString(e.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (Type::toString(ty2.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            res = metamodelica::cons(e.clone(), res.clone());
            tys2 = metamodelica::cons(ty3.clone(), tys2.clone());
        }
        (arrayExp, arrayType) = BuiltinCall::makeCatExp(2, res, tys2, variability, purity, info)?;
    } else {
        (arrayExp, arrayType, variability, _) = typeExp(listHead(elements)?, context, info, false)?;
    }
    Ok((arrayExp, arrayType, variability, purity))
}

pub(crate) fn typeRange(mut rangeExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut rangeExp: Arc<Expression::NFExpression> = rangeExp;
    let mut rangeType: Arc<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut start_exp: Arc<Expression::NFExpression>;
    let mut step_exp: Arc<Expression::NFExpression>;
    let mut stop_exp: Arc<Expression::NFExpression>;
    let mut start_ty: Arc<Type::NFType>;
    let mut step_ty: Arc<Type::NFType>;
    let mut stop_ty: Arc<Type::NFType>;
    let mut ostep_exp: Option<Arc<Expression::NFExpression>>;
    let mut ostep_ty: Option<Arc<Type::NFType>>;
    let mut start_var: Variability;
    let mut step_var: Variability;
    let mut stop_var: Variability;
    let mut start_pur: Purity;
    let mut step_pur: Purity;
    let mut stop_pur: Purity;
    let mut ty_match: MatchKind;
    let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rangeExp) {
        Deref @ Expression::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    start_exp = __pa0.clone();
    ostep_exp = __pa1.clone();
    stop_exp = __pa2.clone();
    (start_exp, start_ty, start_var, start_pur) = typeExp(start_exp, next_context, info.clone(), false)?;
    (stop_exp, stop_ty, stop_var, stop_pur) = typeExp(stop_exp, next_context, info.clone(), false)?;
    variability = Prefixes::variabilityMax(start_var, stop_var);
    purity = Prefixes::purityMin(start_pur, stop_pur);
    (start_exp, stop_exp, rangeType, ty_match) = TypeCheck::matchExpressions(start_exp, start_ty.clone(), stop_exp, stop_ty.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(ty_match) {
        printRangeTypeError(start_exp.clone(), start_ty.clone(), stop_exp.clone(), stop_ty.clone(), info.clone())?;
    }
    if isSome(ostep_exp.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(ostep_exp) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        step_exp = __pa3.clone();
        (step_exp, step_ty, step_var, step_pur) = typeExp(step_exp, next_context, info.clone(), false)?;
        variability = Prefixes::variabilityMax(step_var, variability);
        purity = Prefixes::purityMin(step_pur, purity);
        (start_exp, step_exp, rangeType, ty_match) = TypeCheck::matchExpressions(start_exp, start_ty.clone(), step_exp, step_ty.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
        if TypeCheck::isIncompatibleMatch(ty_match) {
            printRangeTypeError(start_exp.clone(), start_ty, step_exp.clone(), step_ty.clone(), info.clone())?;
        }
        (stop_exp, _, _) = TypeCheck::matchTypes_cast(stop_ty, rangeType.clone(), stop_exp, TypeCheck::DEFAULT_OPTIONS.clone())?;
        ostep_exp = Some(step_exp);
        ostep_ty = Some(step_ty);
    } else {
        ostep_exp = None;
        ostep_ty = None;
    }
    rangeType = TypeCheck::getRangeType(start_exp.clone(), ostep_exp.clone(), stop_exp.clone(), rangeType, info)?;
    rangeExp = Arc::new(Expression::NFExpression::RANGE { ty: rangeType.clone(), start: start_exp, step: ostep_exp, stop: stop_exp });
    if variability <= Variability::PARAMETER.clone() && purity == Purity::PURE.clone() && !(InstContext::inFunction(context)) {
        Structural::markExp(rangeExp.clone())?;
    }
    Ok((rangeExp, rangeType, variability, purity))
}

pub(crate) fn typeTuple(mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut tupleExp: Arc<Expression::NFExpression>;
    let mut tupleType: Arc<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity = Purity::PURE.clone();
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut tyl: Arc<metamodelica::List<Arc<Type::NFType>>>;
    let mut valr: Arc<metamodelica::List<Variability>>;
    let mut next_context: i32;
    if !(InstContext::onLHS(context)) || InstContext::inSubexpression(context) {
        Error::addSourceMessage(Error::RHS_TUPLE_EXPRESSION.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::TUPLE { ty: crate::NFType::interned_UNKNOWN(), elements: elements.clone() }))?).clone()], info.clone())?;
        bail!("fail");
    }
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    (expl, tyl, valr) = typeExpl(elements, next_context, info.clone())?;
    tupleType = Arc::new(Type::NFType::TUPLE { types: tyl, names: None });
    tupleExp = Arc::new(Expression::NFExpression::TUPLE { ty: tupleType.clone(), elements: expl.clone() });
    if !(List::all(expl, (std::sync::Arc::new(fnptr!(Expression::isCref, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
        Error::addSourceMessage(Error::TUPLE_ASSIGN_CREFS_ONLY.clone(), list![(Expression::toString(tupleExp.clone())?).clone()], info)?;
        bail!("fail");
    }
    variability = if (valr.clone().is_empty()) {Variability::CONSTANT.clone()} else {listHead(valr)?};
    Ok((tupleExp, tupleType, variability, purity))
}

pub(crate) fn printRangeTypeError(mut exp1: Arc<Expression::NFExpression>, mut ty1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut ty2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<()> {
    Error::addSourceMessage(Error::RANGE_TYPE_MISMATCH.clone(), list![(Expression::toString(exp1)?).clone(), (Type::toString(ty1)?).clone(), (Expression::toString(exp2)?).clone(), (Type::toString(ty2)?).clone()], info)?;
    bail!("fail");
    Ok(())
}

pub(crate) fn typeSize(mut sizeExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut evaluate: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut sizeExp: Arc<Expression::NFExpression> = sizeExp;
    let mut sizeType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut index_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty_match: MatchKind = MatchKind::EXACT;
    let mut iindex: i32 = 0;
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty_err: Arc<TypingError::TypingError> = Arc::new(TypingError::NO_ERROR);
    let mut oexp: Option<Arc<Expression::NFExpression>> = None;
    let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    (sizeExp, sizeType, variability, purity) = (::match_deref::match_deref! { match &(sizeExp.clone()) {
        Deref @ Expression::SIZE { exp: __esc_exp, dimIndex: Some(__esc_index) } => {
            exp = (*__esc_exp).clone();
            index = (*__esc_index).clone();
            (index, index_ty, variability, purity) = typeExp(index.clone(), next_context, info.clone(), false)?;
            (index, _, ty_match) = TypeCheck::matchTypes(index_ty.clone(), crate::NFType::interned_INTEGER(), index.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isIncompatibleMatch(ty_match) {
                Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (literal!("size ")).clone(), (literal!("dim")).clone(), (Expression::toString(index.clone())?).clone(), (Type::toString(index_ty)?).clone(), (literal!("Integer")).clone()], info.clone())?;
                bail!("fail");
            }
            if variability <= Variability::STRUCTURAL_PARAMETER.clone() && purity == Purity::PURE.clone() {
                index = Ceval::evalExp(index.clone(), Ceval::noTarget().clone())?;
                let __pa0 = ::match_deref::match_deref! { match &(index.clone()) {
                    Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                iindex = __pa0.clone();
                (dim, oexp, ty_err) = typeExpDim(exp.clone(), iindex, next_context, info.clone())?;
                checkSizeTypingError(ty_err, exp.clone(), iindex, info.clone())?;
                if Dimension::isKnown(dim.clone(), false) && evaluate {
                    exp = Dimension::sizeExp(dim.clone())?;
                } else {
                    if isSome(oexp.clone()) {
                        let __pa1 = ::match_deref::match_deref! { match &(oexp) {
                            Some(__pa1) => __pa1.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        exp = __pa1.clone();
                    } else {
                        (exp, _, _, _) = typeExp(exp.clone(), next_context, info, false)?;
                    }
                    exp = Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
                if !(InstContext::inFunction(context)) || Dimension::isKnown(dim, false) {
                    variability = Variability::CONSTANT.clone();
                } else {
                    variability = Variability::DISCRETE.clone();
                    purity = Purity::IMPURE.clone();
                }
            } else {
                (exp, exp_ty, _, purity) = typeExp(var_field!((*sizeExp).exp, Expression::NFExpression::SIZE).clone(), next_context, info.clone(), false)?;
                if !(Type::isArray(exp_ty.clone())) {
                    Error::addSourceMessage(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone(), list![(literal!("size")).clone()], info)?;
                    bail!("fail");
                }
                if Type::isEmptyArray(exp_ty.clone())? && !(InstContext::inFunction(context)) {
                    expl = Array::mapList(Type::arrayDims(exp_ty), (std::sync::Arc::new(Dimension::sizeExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                    exp = Expression::makeExpArray(expl.clone(), crate::NFType::interned_INTEGER(), false);
                    exp = Expression::makeSubscriptedExp(list![Subscript::makeIndex(index.clone())?], exp.clone(), false)?;
                } else {
                    exp = Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
            }
            (exp.clone(), crate::NFType::interned_INTEGER(), variability, purity)
        },
        Deref @ Expression::SIZE { .. } => {
            (exp, exp_ty, _, _) = typeExp(var_field!((*sizeExp).exp, Expression::NFExpression::SIZE).clone(), next_context, info, false)?;
            sizeType = Type::sizeType(exp_ty);
            (Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: None }), sizeType, Variability::PARAMETER.clone(), Purity::PURE.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((sizeExp, sizeType, variability, purity))
}

pub(crate) fn checkSizeTypingError(mut typingError: Arc<TypingError::TypingError>, mut exp: Arc<Expression::NFExpression>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(typingError.clone()) {
        Deref @ TypingError::NO_ERROR { .. } => (),
        Deref @ TypingError::OUT_OF_BOUNDS { upperBound: 0 } => {
            Error::addSourceMessage(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone(), list![(literal!("size")).clone()], info)?;
            bail!("fail")
        },
        Deref @ TypingError::OUT_OF_BOUNDS { .. } => {
            Error::addSourceMessage(Error::INVALID_SIZE_INDEX.clone(), list![ArcStr::from(::std::format!("{}", index)), (Expression::toString(exp)?).clone(), ArcStr::from(::std::format!("{}", var_field!((*typingError).upperBound, TypingError::TypingError::OUT_OF_BOUNDS).clone()))], info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn evaluateEnd(mut exp: Arc<Expression::NFExpression>, mut dim: Arc<Dimension::NFDimension>, mut subscriptedExp: Arc<Expression::NFExpression>, mut index: i32, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::END => Dimension::endExp(dim, subscriptedExp, index)?,
        Deref @ Expression::CREF { .. } => exp,
        _ => Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = dim; let __pe_b2 = subscriptedExp; let __pe_b3 = index; let __pe_b4 = context; let __pe_b5 = info; move |__pe_a0| evaluateEnd(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn typeIfExpression(mut ifExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut ifExp: Arc<Expression::NFExpression> = ifExp;
    let mut ty: Arc<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut cond: Arc<Expression::NFExpression>;
    let mut tb: Arc<Expression::NFExpression>;
    let mut fb: Arc<Expression::NFExpression>;
    let mut tb2: Arc<Expression::NFExpression>;
    let mut fb2: Arc<Expression::NFExpression>;
    let mut next_context: i32;
    let mut cond_ty: Arc<Type::NFType>;
    let mut tb_ty: Arc<Type::NFType>;
    let mut fb_ty: Arc<Type::NFType>;
    let mut cond_var: Variability;
    let mut tb_var: Variability;
    let mut fb_var: Variability;
    let mut cond_pur: Purity;
    let mut tb_pur: Purity;
    let mut fb_pur: Purity;
    let mut ty_match: MatchKind;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ifExp) {
        Deref @ Expression::IF { condition: __pa0, trueBranch: __pa1, falseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cond = __pa0.clone();
    tb = __pa1.clone();
    fb = __pa2.clone();
    next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    (cond, cond_ty, cond_var, cond_pur) = typeExp(cond, next_context, info.clone(), false)?;
    (cond, _, ty_match) = TypeCheck::matchTypes(cond_ty.clone(), crate::NFType::interned_BOOLEAN(), cond, TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(ty_match) {
        Error::addSourceMessage(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Expression::toString(cond.clone())?).clone(), (Type::toString(cond_ty)?).clone()], info.clone())?;
        bail!("fail");
    }
    (tb, tb_ty, tb_var, tb_pur) = typeExp(tb, next_context, info.clone(), false)?;
    (fb, fb_ty, fb_var, fb_pur) = typeExp(fb, next_context, info.clone(), false)?;
    (tb2, fb2, ty, ty_match) = TypeCheck::matchIfBranches(tb.clone(), tb_ty.clone(), fb.clone(), fb_ty.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(ty_match) {
        Error::addSourceMessage(Error::TYPE_MISMATCH_IF_EXP.clone(), list![(literal!("")).clone(), (Expression::toString(tb)?).clone(), (Type::toString(tb_ty)?).clone(), (Expression::toString(fb)?).clone(), (Type::toString(fb_ty)?).clone()], info)?;
        bail!("fail");
    }
    if Expression::contains(tb2.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("der")).clone(); move |__pe_a0| Expression::isCallNamed(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))? != Expression::contains(fb2.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("der")).clone(); move |__pe_a0| Expression::isCallNamed(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))? && Flags::getConfigString(Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone())? == literal!("all") {
        Structural::markExp(cond.clone())?;
    }
    ifExp = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond, trueBranch: tb2, falseBranch: fb2 });
    var = Prefixes::variabilityMax(cond_var, Prefixes::variabilityMax(tb_var, fb_var));
    purity = Prefixes::purityMin(cond_pur, Prefixes::purityMin(tb_pur, fb_pur));
    Ok((ifExp, ty, var, purity))
}

pub(crate) fn typeClassSections(mut classNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cls: Arc<Class::NFClass>;
    let mut typed_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut components: metamodelica::Array<Arc<InstNode::InstNode>> = Default::default();
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut info: SourceInfo;
    let mut initial_context: i32 = 0;
    cls = InstNode::getClass(classNode.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { .. } if (Type::isBasic(Type::arrayElementType(var_field!((*cls).ty, Class::NFClass::INSTANCED_CLASS).clone()))) => (),
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_components, .. }, sections: __esc_sections, .. } => {
            components = (*__esc_components).clone();
            sections = (*__esc_sections).clone();
            sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { .. } => {
            initial_context = InstContext::set(context, InstContext::INITIAL.clone());
            Sections::map(sections.clone(), (std::sync::Arc::new({ let __pe_b1 = InstContext::set(context, InstContext::EQUATION.clone()); move |__pe_a0| typeEquation(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>), (std::sync::Arc::new({ let __pe_b1 = InstContext::set(context, InstContext::ALGORITHM.clone()); move |__pe_a0| typeAlgorithm(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>), (std::sync::Arc::new({ let __pe_b1 = InstContext::set(initial_context, InstContext::EQUATION.clone()); move |__pe_a0| typeEquation(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>), (std::sync::Arc::new({ let __pe_b1 = InstContext::set(initial_context, InstContext::ALGORITHM.clone()); move |__pe_a0| typeAlgorithm(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>))?
        },
        Deref @ Sections::EXTERNAL { .. } => {
            Error::addSourceMessage(Error::TRANS_VIOLATION.clone(), list![(InstNode::name(classNode.clone())?).clone(), (Restriction::toString(var_field!((*cls).restriction, Class::NFClass::INSTANCED_CLASS).clone())).clone(), (literal!("external declaration")).clone()], InstNode::info(classNode.clone()))?;
            bail!("fail")
        },
        _ => sections.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            typed_cls = Class::setSections(sections.clone(), cls.clone())?;
            let __range0 = components.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                typeComponentSections(InstNode::resolveOuter(c.clone()), context)?;
            }
            InstNode::updateClass(typed_cls, classNode)?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeClassSections(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context)?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeClassSections")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(classNode)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeFunctionSections(mut classNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cls: Arc<Class::NFClass>;
    let mut typed_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut alg: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
    cls = InstNode::getClass(classNode.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { sections: __esc_sections, .. } => {
            sections = (*__esc_sections).clone();
            sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { equations: Deref @ metamodelica::List::Nil, initialEquations: Deref @ metamodelica::List::Nil, algorithms: Deref @ metamodelica::List::Cons { head: __esc_alg, tail: Deref @ metamodelica::List::Nil }, initialAlgorithms: Deref @ metamodelica::List::Nil } => {
            alg = (*__esc_alg).clone();
            assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = list![typeAlgorithm(alg.clone(), InstContext::set(context, InstContext::ALGORITHM.clone()))?]);
            sections.clone()
        },
        Deref @ Sections::SECTIONS { .. } => {
            Error::addSourceMessage(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone(), list![(InstNode::name(classNode.clone())?).clone()], InstNode::info(classNode.clone()))?;
            bail!("fail")
        },
        Deref @ Sections::EXTERNAL { explicit: true, .. } => {
            info = InstNode::info(classNode.clone());
            assign_variant_field!(sections => Sections::NFSections::EXTERNAL;
                args = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*sections).args, Sections::NFSections::EXTERNAL).clone()).into_iter().cloned() {
            let __x = typeExternalArg(arg.clone(), info.clone(), classNode.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                outputRef = typeCref(var_field!((*sections).outputRef, Sections::NFSections::EXTERNAL).clone(), context, info.clone())?.0
            );
            checkExternalCallResult(var_field!((*sections).outputRef, Sections::NFSections::EXTERNAL).clone(), info)?;
            sections.clone()
        },
        Deref @ Sections::EXTERNAL { .. } => makeDefaultExternalCall(sections.clone(), classNode.clone())?,
        _ => sections.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            typed_cls = Class::setSections(sections.clone(), cls)?;
            InstNode::updateClass(typed_cls, classNode)?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { .. } => {
            typeFunctionSections(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone(), context)?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeFunctionSections")); __mm_s.push_str(&*literal!(" got uninstantiated class ")); __mm_s.push_str(&*InstNode::name(classNode)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeExternalArg(mut arg: Arc<Expression::NFExpression>, mut info: SourceInfo, mut node: Arc<InstNode::InstNode>) -> Result<Arc<Expression::NFExpression>> {
    let mut outArg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut index: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outArg = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::SIZE { dimIndex: Some(_), .. } => {
            (outArg, _, _, _) = typeSize(arg.clone(), InstContext::FUNCTION.clone(), info.clone(), false)?;
            let __pa0 = ::match_deref::match_deref! { match &(outArg.clone()) {
                Deref @ Expression::SIZE { dimIndex: Some(__pa0), .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            index = __pa0.clone();
            if !(Expression::isInteger(index)) {
                Error::addSourceMessage(Error::EXTERNAL_ARG_NONCONSTANT_SIZE_INDEX.clone(), list![(Expression::toString(arg)?).clone()], info)?;
                bail!("fail");
            }
            outArg
        },
        _ => {
            (outArg, ty, var, _) = typeExp(arg.clone(), InstContext::FUNCTION.clone(), info.clone(), false)?;
            Call::updateExternalRecordArgsInType(ty.clone())?;
            (::match_deref::match_deref! { match &(arg) {
        Deref @ Expression::CREF { .. } => outArg,
        _ => {
            if Type::isScalarBuiltin(ty)? && var == Variability::CONSTANT.clone() {
                outArg = Ceval::evalExp(outArg, Ceval::EvalTarget::new(info, InstContext::FUNCTION.clone(), None))?;
            } else {
                Error::addSourceMessage(Error::EXTERNAL_ARG_WRONG_EXP.clone(), list![(Expression::toString(outArg.clone())?).clone()], info)?;
                bail!("fail");
            }
            outArg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub(crate) fn makeDefaultExternalCall(mut extDecl: Arc<Sections::NFSections>, mut fnNode: Arc<InstNode::InstNode>) -> Result<Arc<Sections::NFSections>> {
    let mut extDecl: Arc<Sections::NFSections> = extDecl;
    extDecl = (::match_deref::match_deref! { match &(extDecl.clone()) {
        Deref @ Sections::EXTERNAL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            let mut r#fn: Arc<Function::Function>;
            let mut single_output: bool;
            let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut comp: Arc<Component::NFComponent>;
            let mut ty: Arc<Type::NFType>;
            let mut node: Arc<InstNode::InstNode>;
            let mut exp: Arc<Expression::NFExpression>;
            if var_field!((*extDecl).language, Sections::NFSections::EXTERNAL).clone() == literal!("builtin") {
                return Ok(extDecl.clone());
            }
            let __pa0 = ::match_deref::match_deref! { match &(InstNode::getFuncCache(fnNode.clone())?) {
                Deref @ CachedData::FUNCTION { funcs: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            single_output = (r#fn.outputs.clone().len() as i32) == 1;
            if single_output && Type::isArray(Function::returnType(r#fn.clone())) {
                single_output = false;
                Error::addSourceMessage(Error::EXT_FN_SINGLE_RETURN_ARRAY.clone(), list![(var_field!((*extDecl).language, Sections::NFSections::EXTERNAL).clone()).clone()], InstNode::info(fnNode))?;
            }
            if single_output {
                let __pa2 = ::match_deref::match_deref! { match &(r#fn.outputs.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                node = __pa2.clone();
                ty = InstNode::getType(node.clone())?;
                assign_variant_field!(extDecl => Sections::NFSections::EXTERNAL; outputRef = ComponentRef::fromNode(node, ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()));
            }
            comps = ClassTree::getComponents(Class::classTree(InstNode::getClass(r#fn.node.clone())?)?)?;
            if metamodelica::arrayLength(comps.clone()) > 0 {
                args = metamodelica::nil();
                let __range4 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range4 {
                    comp = InstNode::component(c.clone())?;
                    if !(single_output) || Component::direction(comp.clone()) != Direction::OUTPUT.clone() {
                        ty = Component::getType(comp.clone())?;
                        exp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: ComponentRef::fromNode(c.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()) });
                        args = metamodelica::cons(exp.clone(), args.clone());
                        for mut i in 1..=Type::dimensionCount(ty.clone()) {
                            args = metamodelica::cons(Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) }), args.clone());
                        }
                    }
                }
                assign_variant_field!(extDecl => Sections::NFSections::EXTERNAL; args = args.reverse());
            }
            extDecl
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(extDecl)
}

pub(crate) fn checkExternalCallResult(mut result: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<()> {
    let mut ty: Arc<Type::NFType>;
    if !(ComponentRef::isCref(result.clone())) {
        return Ok(());
    }
    ty = ComponentRef::nodeType(result.clone())?;
    if Type::isArray(ty.clone()) {
        Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE.clone(), list![(Type::toString(ty)?).clone()], info.clone())?;
        bail!("fail");
    }
    if ComponentRef::variability(result)? < Variability::DISCRETE.clone() {
        Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_NOT_VAR.clone(), metamodelica::nil(), info)?;
        bail!("fail");
    }
    Ok(())
}

pub(crate) fn typeComponentSections(mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut comp: Arc<Component::NFComponent>;
    if InstNode::isEmpty(component.clone()) {
        return Ok(());
    }
    comp = InstNode::component(component.clone())?;
    if Component::isDeleted(comp.clone())? || InstNode::isOnlyOuter(component.clone())? {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { .. } if (var_field!((*comp).state, Component::NFComponent::COMPONENT).clone() >= ComponentState::TypeChecked.clone()) => {
            typeClassSections(var_field!((*comp).classInst, Component::NFComponent::COMPONENT).clone(), context)?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTyping.typeComponentSections")); __mm_s.push_str(&*literal!(" got uninstantiated component ")); __mm_s.push_str(&*InstNode::name(component)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFTyping.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn typeEquation(mut eq: Arc<Equation::NFEquation>, mut context: i32) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            typeEqualityEquation(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), context, var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone())?
        },
        Deref @ Equation::CONNECT { .. } => {
            typeConnect(var_field!((*eq).lhs, Equation::NFEquation::CONNECT).clone(), var_field!((*eq).rhs, Equation::NFEquation::CONNECT).clone(), context, var_field!((*eq).scope, Equation::NFEquation::CONNECT).clone(), var_field!((*eq).source, Equation::NFEquation::CONNECT).clone())?
        },
        Deref @ Equation::FOR { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
            let mut next_context: i32;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::FOR).clone());
            if isSome(var_field!((*eq).range, Equation::NFEquation::FOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*eq).range, Equation::NFEquation::FOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
            } else {
                e1 = deduceIterationRangeEq(eq.clone(), var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), info)?;
            }
            (e1, _, _, _) = typeIterator(var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), e1, context, true)?;
            next_context = InstContext::set(context, InstContext::FOR.clone());
            body = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).body, Equation::NFEquation::FOR).clone()).into_iter().cloned() {
            let __x = typeEquation(e.clone(), next_context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(Equation::NFEquation::FOR { iterator: var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), range: Some(e1), body: body, scope: var_field!((*eq).scope, Equation::NFEquation::FOR).clone(), source: var_field!((*eq).source, Equation::NFEquation::FOR).clone() })
        },
        Deref @ Equation::IF { .. } => {
            typeIfEquation(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), context, var_field!((*eq).scope, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone())?
        },
        Deref @ Equation::WHEN { .. } => {
            typeWhenEquation(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), context, var_field!((*eq).scope, Equation::NFEquation::WHEN).clone(), var_field!((*eq).source, Equation::NFEquation::WHEN).clone())?
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut e3: Arc<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::ASSERT).clone());
            (e1, e2, e3) = typeAssert(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), context, info)?;
            Arc::new(Equation::NFEquation::ASSERT { condition: e1, message: e2, level: e3, scope: var_field!((*eq).scope, Equation::NFEquation::ASSERT).clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() })
        },
        Deref @ Equation::TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone());
            (e1, _) = typeOperatorArg(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), crate::NFType::interned_STRING(), context, (literal!("terminate")).clone(), (literal!("message")).clone(), 1, info)?;
            Arc::new(Equation::NFEquation::TERMINATE { message: e1, scope: var_field!((*eq).scope, Equation::NFEquation::TERMINATE).clone(), source: var_field!((*eq).source, Equation::NFEquation::TERMINATE).clone() })
        },
        Deref @ Equation::REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            (e1, e2) = typeReinit(var_field!((*eq).cref, Equation::NFEquation::REINIT).clone(), var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), context, var_field!((*eq).source, Equation::NFEquation::REINIT).clone())?;
            Arc::new(Equation::NFEquation::REINIT { cref: e1, reinitExp: e2, scope: var_field!((*eq).scope, Equation::NFEquation::REINIT).clone(), source: var_field!((*eq).source, Equation::NFEquation::REINIT).clone() })
        },
        Deref @ Equation::NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            (e1, _, _, _) = typeExp(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), context, ElementSource::getInfo(var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone()), false)?;
            Arc::new(Equation::NFEquation::NORETCALL { exp: e1, scope: var_field!((*eq).scope, Equation::NFEquation::NORETCALL).clone(), source: var_field!((*eq).source, Equation::NFEquation::NORETCALL).clone() })
        },
        _ => {
            eq
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub(crate) fn typeConnect(mut lhsConn: Arc<Expression::NFExpression>, mut rhsConn: Arc<Expression::NFExpression>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut connEq: Arc<Equation::NFEquation>;
    let mut lhs: Arc<Expression::NFExpression>;
    let mut rhs: Arc<Expression::NFExpression>;
    let mut lhs_ty: Arc<Type::NFType>;
    let mut rhs_ty: Arc<Type::NFType>;
    let mut mk: MatchKind;
    let mut next_context: i32;
    let mut info: SourceInfo;
    let mut lhs_deleted: bool;
    let mut rhs_deleted: bool;
    info = ElementSource::getInfo(source.clone());
    if InstContext::inNonexpandable(context) {
        Error::addSourceMessage(Error::CONNECT_IN_IF.clone(), list![(Expression::toString(lhsConn.clone())?).clone(), (Expression::toString(rhsConn.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    next_context = InstContext::set(context, InstContext::CONNECT.clone());
    (lhs, lhs_ty, lhs_deleted) = typeConnector(lhsConn.clone(), next_context, info.clone())?;
    (rhs, rhs_ty, rhs_deleted) = typeConnector(rhsConn.clone(), next_context, info.clone())?;
    if !(lhs_deleted || rhs_deleted) && !(Type::isExpandableConnector(Type::arrayElementType(lhs_ty.clone())) || Type::isExpandableConnector(Type::arrayElementType(rhs_ty.clone()))) {
        (lhs, rhs, _, mk) = TypeCheck::matchExpressions(lhs, lhs_ty, rhs, rhs_ty, TypeCheck::ALLOW_UNKNOWN.clone())?;
        if TypeCheck::isIncompatibleMatch(mk) {
            Error::addSourceMessage(Error::CONNECT_TYPE_MISMATCH.clone(), list![(Expression::toString(lhsConn)?).clone(), (Expression::toString(rhsConn)?).clone()], info)?;
            bail!("fail");
        }
    }
    connEq = Arc::new(Equation::NFEquation::CONNECT { lhs: lhs, rhs: rhs, scope: scope, source: source });
    Ok(connEq)
}

pub(crate) fn typeConnector(mut connExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, bool)> {
    let mut connExp: Arc<Expression::NFExpression> = connExp;
    let mut ty: Arc<Type::NFType>;
    let mut deleted: bool;
    (connExp, ty, _, _) = typeExp(connExp, context, info.clone(), false)?;
    deleted = checkConnector(connExp.clone(), info)?;
    Ok((connExp, ty, deleted))
}

pub(crate) fn checkConnector(mut connExp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<bool> {
    let mut deleted: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(connExp.clone()) {
        Deref @ Expression::CREF { cref: __esc_cr @ Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, .. }, .. } => {
            cr = (*__esc_cr).clone();
            if !(InstNode::isConnector(var_field!((*cr).node, ComponentRef::NFComponentRef::CREF).clone())?) {
                Error::addSourceMessageAndFail(Error::INVALID_CONNECTOR_TYPE.clone(), list![(ComponentRef::toString(cr.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if !(checkConnectorForm(cr.clone(), true)?) {
                Error::addSourceMessageAndFail(Error::INVALID_CONNECTOR_FORM.clone(), list![(ComponentRef::toString(cr.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if ComponentRef::subscriptsVariability(cr.clone(), Prefixes::Variability::CONSTANT.clone())? > Variability::PARAMETER.clone() {
                subs = ComponentRef::subscriptsAllFlat(cr.clone())?;
                for mut sub in &*subs {
                    let mut sub = sub.clone();
                    if Subscript::variability(sub.clone())? > Variability::PARAMETER.clone() {
                        Error::addSourceMessage(Error::CONNECTOR_NON_PARAMETER_SUBSCRIPT.clone(), list![(Expression::toString(connExp.clone())?).clone(), (Subscript::toString(sub.clone())?).clone()], info.clone())?;
                        bail!("fail");
                    }
                }
            }
            deleted = ComponentRef::isDeleted(cr.clone())?;
            ()
        },
        _ => {
            Error::addSourceMessage(Error::INVALID_CONNECTOR_TYPE.clone(), list![(Expression::toString(connExp)?).clone()], info)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(deleted)
}

pub(crate) fn checkConnectorForm(mut cref: Arc<ComponentRef::NFComponentRef>, mut isConnector: bool) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, .. } => if (isConnector) {{ (cref, isConnector) = (var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), InstNode::isConnector(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?); continue '__tco; }} else {return Ok(false)},
        _ => return Ok(true),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn checkLhsInWhen(mut exp: Arc<Expression::NFExpression>) -> bool {
    let mut isValid: bool;
    isValid = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => true,
        Deref @ Expression::TUPLE { .. } => {
            for mut e in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                checkLhsInWhen(e.clone());
            }
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isValid
}

pub(crate) fn typeAssert(mut condition: Arc<Expression::NFExpression>, mut message: Arc<Expression::NFExpression>, mut level: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut condition: Arc<Expression::NFExpression> = condition;
    let mut message: Arc<Expression::NFExpression> = message;
    let mut level: Arc<Expression::NFExpression> = level;
    let mut next_context: i32;
    let mut level_var: Variability;
    next_context = InstContext::set(context, InstContext::ASSERT.clone());
    (condition, _) = typeOperatorArg(condition, crate::NFType::interned_BOOLEAN(), InstContext::set(next_context, InstContext::CONDITION.clone()), (literal!("assert")).clone(), (literal!("condition")).clone(), 1, info.clone())?;
    (message, _) = typeOperatorArg(message, crate::NFType::interned_STRING(), next_context, (literal!("assert")).clone(), (literal!("message")).clone(), 2, info.clone())?;
    (level, level_var) = typeOperatorArg(level, Builtin::ASSERTIONLEVEL_TYPE().clone(), next_context, (literal!("assert")).clone(), (literal!("level")).clone(), 3, info.clone())?;
    if level_var > Variability::PARAMETER.clone() {
        Error::addSourceMessage(Error::FUNCTION_SLOT_VARIABILITY.clone(), list![(literal!("level")).clone(), (Expression::toString(level.clone())?).clone(), (literal!("assert")).clone(), (Prefixes::variabilityString(level_var)?).clone(), (literal!("parameter")).clone()], info)?;
        bail!("fail");
    }
    Structural::markExp(level.clone())?;
    Ok((condition, message, level))
}

pub(crate) fn typeAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut context: i32) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = typeStatement(s.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(alg)
}

pub(crate) fn typeStatements(mut alg: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut alg: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = alg;
    alg = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut stmt in (alg).into_iter().cloned() {
            let __x = typeStatement(stmt.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(alg)
}

pub(crate) fn typeStatement(mut st: Arc<Statement::NFStatement>, mut context: i32) -> Result<Arc<Statement::NFStatement>> {
    let mut st: Arc<Statement::NFStatement> = st;
    st = (::match_deref::match_deref! { match &(st.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut ty1: Arc<Type::NFType>;
            let mut ty2: Arc<Type::NFType>;
            let mut mk: MatchKind;
            let mut info: SourceInfo;
            let mut var: Variability;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::ASSIGNMENT).clone());
            (e1, ty1, var, _) = typeExp(var_field!((*st).lhs, Statement::NFStatement::ASSIGNMENT).clone(), InstContext::set(context, InstContext::LHS.clone()), info.clone(), false)?;
            (e2, ty2, _, _) = typeExp(var_field!((*st).rhs, Statement::NFStatement::ASSIGNMENT).clone(), InstContext::set(context, InstContext::RHS.clone()), info.clone(), false)?;
            (e2, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), e2, TypeCheck::ALLOW_UNKNOWN.clone())?;
            if TypeCheck::isIncompatibleMatch(mk) {
                Error::addSourceMessage(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone(), list![(Expression::toString(e1.clone())?).clone(), (Expression::toString(e2.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (Type::toString(ty2)?).clone()], info.clone())?;
                bail!("fail");
            }
            checkAssignment(e1.clone(), e2.clone(), var, context, info)?;
            if Expression::isExternalCall(e2.clone())? {
                Call::updateExternalRecordArgs(Expression::tupleElements(e1.clone()))?;
            }
            Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: e1, rhs: e2, ty: ty1, source: var_field!((*st).source, Statement::NFStatement::ASSIGNMENT).clone() })
        },
        Deref @ Statement::FOR { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>;
            let mut next_context: i32;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::FOR).clone());
            if isSome(var_field!((*st).range, Statement::NFStatement::FOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*st).range, Statement::NFStatement::FOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
            } else {
                e1 = deduceIterationRangeStmt(st.clone(), var_field!((*st).iterator, Statement::NFStatement::FOR).clone(), info)?;
            }
            (e1, _, _, _) = typeIterator(var_field!((*st).iterator, Statement::NFStatement::FOR).clone(), e1, context, false)?;
            next_context = InstContext::set(context, InstContext::FOR.clone());
            body = typeStatements(var_field!((*st).body, Statement::NFStatement::FOR).clone(), next_context)?;
            Arc::new(Statement::NFStatement::FOR { iterator: var_field!((*st).iterator, Statement::NFStatement::FOR).clone(), range: Some(e1), body: body, forType: var_field!((*st).forType, Statement::NFStatement::FOR).clone(), source: var_field!((*st).source, Statement::NFStatement::FOR).clone() })
        },
        Deref @ Statement::IF { .. } => {
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut tybrs: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>;
            let mut next_context: i32;
            let mut cond_context: i32;
            next_context = InstContext::set(context, InstContext::IF.clone());
            cond_context = InstContext::set(next_context, InstContext::CONDITION.clone());
            tybrs = ({
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut br in (var_field!((*st).branches, Statement::NFStatement::IF).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(br.clone()) {
        (__esc_cond, __esc_body) => {
            cond = (*__esc_cond).clone();
            body = (*__esc_body).clone();
            (e1, _, _) = typeCondition(cond.clone(), cond_context, var_field!((*st).source, Statement::NFStatement::IF).clone(), Error::IF_CONDITION_TYPE_ERROR.clone(), false, false)?;
            sts1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (body.clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), next_context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1.clone(), sts1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(Statement::NFStatement::IF { branches: tybrs, source: var_field!((*st).source, Statement::NFStatement::IF).clone() })
        },
        Deref @ Statement::WHEN { .. } => {
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut tybrs: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>;
            let mut next_context: i32;
            next_context = InstContext::set(context, InstContext::WHEN.clone());
            tybrs = ({
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut br in (var_field!((*st).branches, Statement::NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(br.clone()) {
        (__esc_cond, __esc_body) => {
            cond = (*__esc_cond).clone();
            body = (*__esc_body).clone();
            (e1, _, _) = typeWhenCondition(cond.clone(), context, var_field!((*st).source, Statement::NFStatement::WHEN).clone(), false)?;
            sts1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (body.clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), next_context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1.clone(), sts1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(Statement::NFStatement::WHEN { branches: tybrs, source: var_field!((*st).source, Statement::NFStatement::WHEN).clone() })
        },
        Deref @ Statement::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut e3: Arc<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::ASSERT).clone());
            (e1, e2, e3) = typeAssert(var_field!((*st).condition, Statement::NFStatement::ASSERT).clone(), var_field!((*st).message, Statement::NFStatement::ASSERT).clone(), var_field!((*st).level, Statement::NFStatement::ASSERT).clone(), context, info)?;
            Arc::new(Statement::NFStatement::ASSERT { condition: e1, message: e2, level: e3, source: var_field!((*st).source, Statement::NFStatement::ASSERT).clone() })
        },
        Deref @ Statement::TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut info: SourceInfo;
            info = ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::TERMINATE).clone());
            if InstContext::inFunction(context) {
                Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("terminate")).clone()], info.clone())?;
                bail!("fail");
            }
            (e1, _) = typeOperatorArg(var_field!((*st).message, Statement::NFStatement::TERMINATE).clone(), crate::NFType::interned_STRING(), context, (literal!("terminate")).clone(), (literal!("message")).clone(), 1, info)?;
            Arc::new(Statement::NFStatement::TERMINATE { message: e1, source: var_field!((*st).source, Statement::NFStatement::TERMINATE).clone() })
        },
        Deref @ Statement::REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            if InstContext::inFunction(context) {
                Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("reinit")).clone()], ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::REINIT).clone()))?;
                bail!("fail");
            }
            (e1, e2) = typeReinit(var_field!((*st).cref, Statement::NFStatement::REINIT).clone(), var_field!((*st).reinitExp, Statement::NFStatement::REINIT).clone(), context, var_field!((*st).source, Statement::NFStatement::REINIT).clone())?;
            Arc::new(Statement::NFStatement::REINIT { cref: e1, reinitExp: e2, source: var_field!((*st).source, Statement::NFStatement::REINIT).clone() })
        },
        Deref @ Statement::NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            (e1, _, _, _) = typeExp(var_field!((*st).exp, Statement::NFStatement::NORETCALL).clone(), context, ElementSource::getInfo(var_field!((*st).source, Statement::NFStatement::NORETCALL).clone()), false)?;
            Arc::new(Statement::NFStatement::NORETCALL { exp: e1, source: var_field!((*st).source, Statement::NFStatement::NORETCALL).clone() })
        },
        Deref @ Statement::WHILE { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>>;
            (e1, _, _) = typeCondition(var_field!((*st).condition, Statement::NFStatement::WHILE).clone(), context, var_field!((*st).source, Statement::NFStatement::WHILE).clone(), Error::WHILE_CONDITION_TYPE_ERROR.clone(), false, false)?;
            sts1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (var_field!((*st).body, Statement::NFStatement::WHILE).clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(Statement::NFStatement::WHILE { condition: e1, body: sts1, source: var_field!((*st).source, Statement::NFStatement::WHILE).clone() })
        },
        Deref @ Statement::FAILURE { .. } => {
            let mut sts1: Arc<metamodelica::List<Arc<Statement::NFStatement>>>;
            sts1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut bst in (var_field!((*st).body, Statement::NFStatement::FAILURE).clone()).into_iter().cloned() {
            let __x = typeStatement(bst.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(Statement::NFStatement::FAILURE { body: sts1, source: var_field!((*st).source, Statement::NFStatement::FAILURE).clone() })
        },
        _ => {
            st
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(st)
}

pub(crate) fn checkAssignment(mut lhsExp: Arc<Expression::NFExpression>, mut rhsExp: Arc<Expression::NFExpression>, mut lhsVar: Variability, mut context: i32, mut info: SourceInfo) -> Result<()> {
    if InstContext::inInstanceAPI(context) {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(lhsExp.clone()) {
        Deref @ Expression::TUPLE { .. } => {
            let mut i: i32;
            i = 1;
            for mut e in &*var_field!((*lhsExp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                checkAssignment(e.clone(), Expression::tupleElement(rhsExp.clone(), var_field!((*lhsExp).ty, Expression::NFExpression::TUPLE).clone(), i)?, Expression::variability(e.clone())?, context, info.clone())?;
                i = i + 1;
            }
            ()
        },
        Deref @ Expression::CREF { .. } if (InstContext::inFunction(context)) => {
            if ComponentRef::isCref(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone()) && InstNode::isInput(ComponentRef::node(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone())?) {
                Error::addSourceMessage(Error::ASSIGN_READONLY_ERROR.clone(), list![(literal!("input")).clone(), (ComponentRef::toString(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone())?).clone()], info)?;
                bail!("fail");
            }
            ()
        },
        _ => {
            if lhsVar < Variability::DISCRETE.clone() {
                if lhsVar == Variability::CONSTANT.clone() {
                    Error::addSourceMessage(Error::ASSIGN_CONSTANT_ERROR.clone(), list![(Expression::toString(lhsExp)?).clone(), (Expression::toString(rhsExp)?).clone()], info)?;
                    bail!("fail");
                } else if !(InstContext::inInitial(context)) {
                    Error::addSourceMessage(Error::ASSIGN_PARAM_ERROR.clone(), list![(Expression::toString(lhsExp)?).clone(), (Expression::toString(rhsExp)?).clone()], info)?;
                    bail!("fail");
                }
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn typeEqualityEquation(mut lhsExp: Arc<Expression::NFExpression>, mut rhsExp: Arc<Expression::NFExpression>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation>;
    let mut info: SourceInfo = ElementSource::getInfo(source.clone());
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut ty: Arc<Type::NFType>;
    let mut mk: MatchKind;
    if InstContext::inWhen(context) && !(InstContext::inClocked(context)) {
        if checkLhsInWhen(lhsExp.clone()) {
            Structural::markSubscriptsInExp(lhsExp.clone())?;
        } else {
            Error::addSourceMessage(Error::WHEN_EQ_LHS.clone(), list![(Expression::toString(lhsExp.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
    }
    (e1, ty1, _, _) = typeExp(lhsExp.clone(), InstContext::set(context, InstContext::LHS.clone()), info.clone(), false)?;
    (e2, ty2, _, _) = typeExp(rhsExp.clone(), InstContext::set(context, InstContext::RHS.clone()), info.clone(), false)?;
    (e2, e1, ty, mk) = TypeCheck::matchExpressions(e2, ty2.clone(), e1, ty1.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(Error::EQUATION_TYPE_MISMATCH_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(lhsExp)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(rhsExp)?); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Type::toString(ty1)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Type::toString(ty2)?); ArcStr::from(__mm_s) }).clone()], info)?;
        bail!("fail");
    }
    eq = Equation::makeEquality(e1.clone(), e2.clone(), ty, source, scope, Equation::ScalarizeMode::NO_PREFERENCE.clone());
    if Expression::isExternalCall(e2)? {
        Call::updateExternalRecordArgs(Expression::tupleElements(e1))?;
    }
    Ok(eq)
}

pub(crate) fn typeCondition(mut condition: Arc<Expression::NFExpression>, mut context: i32, mut source: Arc<DAE::ElementSource>, mut errorMsg: ErrorTypes::Message, mut allowVector: bool, mut allowClock: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability)> {
    let mut condition: Arc<Expression::NFExpression> = condition;
    let mut ty: Arc<Type::NFType>;
    let mut variability: Variability;
    let mut info: SourceInfo;
    let mut ety: Arc<Type::NFType>;
    info = ElementSource::getInfo(source);
    (condition, ty, variability, _) = typeExp(condition, context, info.clone(), false)?;
    if allowVector && Type::isArray(ty.clone()) {
        ety = Type::unliftArray(ty.clone())?;
    } else {
        ety = ty.clone();
    }
    if !(Type::isBoolean(ety.clone()) || allowClock && Type::isClock(ety)?) {
        Error::addSourceMessage(errorMsg, list![(Expression::toString(condition.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info)?;
        bail!("fail");
    }
    Ok((condition, ty, variability))
}

pub(crate) fn typeIfEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut ifEq: Arc<Equation::NFEquation>;
    let mut cond: Arc<Expression::NFExpression>;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut accum_var: Variability = Variability::CONSTANT.clone();
    let mut var: Variability;
    let mut bl: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut bl2: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut next_context: i32 = InstContext::set(context, InstContext::IF.clone());
    let mut cond_context: i32 = InstContext::set(next_context, InstContext::CONDITION.clone());
    for mut b in &*branches {
        let mut b = b.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: _, body: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa0.clone();
        eql = __pa1.clone();
        (cond, _, var) = typeCondition(cond.clone(), cond_context, source.clone(), Error::IF_CONDITION_TYPE_ERROR.clone(), false, false)?;
        if var > Variability::PARAMETER.clone() || Structural::isExpressionNotFixed(cond.clone(), false, 100)? {
            next_context = InstContext::set(next_context, InstContext::NONEXPANDABLE.clone());
        } else if var == Variability::PARAMETER.clone() && (accum_var <= Variability::PARAMETER.clone() || Equation::containsList(eql.clone(), (std::sync::Arc::new(Equation::isConnection) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?) {
            var = Variability::STRUCTURAL_PARAMETER.clone();
        }
        accum_var = Prefixes::variabilityMax(accum_var, var);
        bl = metamodelica::cons(Arc::new(Equation::Branch::Branch::BRANCH { condition: cond.clone(), conditionVar: var, body: eql.clone() }), bl.clone());
    }
    for mut b in &*bl {
        let mut b = b.clone();
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa2, conditionVar: __pa3, body: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa2.clone();
        var = __pa3.clone();
        eql = __pa4.clone();
        ErrorExt::setCheckpoint(literal!("NFTyping.typeIfEquation"));
        match '__try5: {
            eql = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (eql.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(typeEquation(e.clone(), next_context), '__try5);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            bl2 = metamodelica::cons(Equation::makeBranch(cond.clone(), eql.clone(), var), bl2.clone());
            Ok::<_, anyhow::Error>((bl2.clone(),))
        } {
            Ok((__try5_o0,)) => {
                bl2 = __try5_o0;
            }
            Err(_) => {
                bl2 = metamodelica::cons(Arc::new(Equation::Branch::Branch::INVALID_BRANCH { branch: Equation::makeBranch(cond.clone(), eql.clone(), var), errors: ErrorExt::getCheckpointMessages() }), bl2.clone());
            }
        }
        ErrorExt::delCheckpoint(literal!("NFTyping.typeIfEquation"));
    }
    ifEq = Arc::new(Equation::NFEquation::IF { branches: bl2, scope: scope, source: source });
    Ok(ifEq)
}

pub(crate) fn typeWhenEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut context: i32, mut scope: Arc<InstNode::InstNode>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut whenEq: Arc<Equation::NFEquation>;
    let mut next_context: i32 = InstContext::set(context, InstContext::WHEN.clone());
    let mut accum_branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression>;
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut ty: Arc<Type::NFType>;
    let mut var: Variability;
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: _, body: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cond = __pa0.clone();
        body = __pa1.clone();
        (cond, ty, var) = typeWhenCondition(cond.clone(), context, source.clone(), true)?;
        if Type::isClock(ty.clone())? {
            if (branches.clone().len() as i32) != 1 {
                if referenceEq(&*(branch.clone()),&*(listHead(branches.clone())?)) {
                    Error::addSourceMessage(Error::ELSE_WHEN_CLOCK.clone(), metamodelica::nil(), ElementSource::getInfo(source.clone()))?;
                } else {
                    Error::addSourceMessage(Error::CLOCKED_WHEN_BRANCH.clone(), metamodelica::nil(), ElementSource::getInfo(source.clone()))?;
                }
                bail!("fail");
            } else {
                next_context = InstContext::set(context, InstContext::CLOCKED.clone());
            }
        }
        body = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (body.clone()).into_iter().cloned() {
            let __x = typeEquation(eq.clone(), next_context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        accum_branches = metamodelica::cons(Equation::makeBranch(cond.clone(), body.clone(), var), accum_branches.clone());
    }
    whenEq = Arc::new(Equation::NFEquation::WHEN { branches: metamodelica::Dangerous::listReverseInPlace(accum_branches), scope: scope, source: source });
    Ok(whenEq)
}

pub(crate) fn typeWhenCondition(mut condition: Arc<Expression::NFExpression>, mut context: i32, mut source: Arc<DAE::ElementSource>, mut allowClock: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability)> {
    let mut outCondition: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut variability: Variability;
    (outCondition, ty, variability) = typeCondition(condition.clone(), context, source.clone(), Error::WHEN_CONDITION_TYPE_ERROR.clone(), true, allowClock)?;
    if variability > Variability::IMPLICITLY_DISCRETE.clone() && !(Type::isClock(ty.clone())?) {
        Error::addSourceMessage(Error::NON_DISCRETE_WHEN_CONDITION.clone(), list![(Expression::toString(condition.clone())?).clone()], ElementSource::getInfo(source.clone()))?;
        bail!("fail");
    }
    if !(checkWhenInitial(outCondition.clone())?) {
        Error::addSourceMessage(Error::INITIAL_CALL_WARNING.clone(), list![(Expression::toString(condition)?).clone()], ElementSource::getInfo(source))?;
    }
    Ok((outCondition, ty, variability))
}

pub(crate) fn checkWhenInitial(mut condition: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut invalid: bool = false;
    invalid = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            let __range0 = var_field!((*condition).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                if checkWhenInitial(e.clone())? {
                    invalid = true;
                    return Ok(invalid.clone());
                }
            }
            false
        },
        _ => !(Expression::containsShallow(condition, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b1 = (literal!("initial")).clone(); move |__pe_a0| Expression::isCallNamed(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>); move |__pe_a0| Expression::contains(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(invalid)
}

pub(crate) fn typeOperatorArg(mut arg: Arc<Expression::NFExpression>, mut expectedType: Arc<Type::NFType>, mut context: i32, mut operatorName: ArcStr, mut argName: ArcStr, mut argIndex: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Variability)> {
    let mut arg: Arc<Expression::NFExpression> = arg;
    let mut var: Variability;
    let mut ty: Arc<Type::NFType>;
    let mut mk: MatchKind;
    (arg, ty, var, _) = typeExp(arg, context, info.clone(), false)?;
    (arg, _, mk) = TypeCheck::matchTypes(ty.clone(), expectedType.clone(), arg, TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(intString(argIndex)).clone(), (operatorName).clone(), (argName).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty)?).clone(), (Type::toString(expectedType)?).clone()], info)?;
        bail!("fail");
    }
    Ok((arg, var))
}

pub(crate) fn typeReinit(mut crefExp: Arc<Expression::NFExpression>, mut exp: Arc<Expression::NFExpression>, mut context: i32, mut source: Arc<DAE::ElementSource>) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut crefExp: Arc<Expression::NFExpression> = crefExp;
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut mk: MatchKind;
    let mut ty1: Arc<Type::NFType>;
    let mut ty2: Arc<Type::NFType>;
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    let mut info: SourceInfo;
    info = ElementSource::getInfo(source);
    (crefExp, ty1, _, _) = typeExp(crefExp, context, info.clone(), false)?;
    (exp, ty2, _, _) = typeExp(exp, context, info.clone(), false)?;
    cref = (::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ Expression::CREF { .. } => {
            if ComponentRef::isIterator(var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone()) {
                Error::addSourceMessage(Error::ASSIGN_ITERATOR_ERROR.clone(), list![(ComponentRef::toString(var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone()
        },
        _ => {
            Error::addSourceMessage(Error::REINIT_MUST_BE_VAR_OR_ARRAY.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if ComponentRef::nodeVariability(cref.clone())? < Variability::IMPLICITLY_DISCRETE.clone() {
        Error::addSourceMessage(Error::REINIT_MUST_BE_VAR.clone(), list![(Expression::toString(crefExp.clone())?).clone(), (Prefixes::variabilityString(ComponentRef::nodeVariability(cref)?)?).clone()], info.clone())?;
        bail!("fail");
    }
    (_, _, mk) = TypeCheck::matchTypes(Type::arrayElementType(ty1.clone()), crate::NFType::interned_REAL(), crefExp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(Error::REINIT_MUST_BE_REAL.clone(), list![(Expression::toString(crefExp.clone())?).clone(), (Type::toString(Type::arrayElementType(ty1.clone()))?).clone()], info.clone())?;
        bail!("fail");
    }
    (exp, _, mk) = TypeCheck::matchTypes(ty2.clone(), ty1.clone(), exp, TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (literal!("reinit")).clone(), (literal!("")).clone(), (Expression::toString(exp.clone())?).clone(), (Type::toString(ty2)?).clone(), (Type::toString(ty1)?).clone()], info)?;
        bail!("fail");
    }
    Ok((crefExp, exp))
}

pub(crate) fn deduceIterationRangeEq(mut eq: Arc<Equation::NFEquation>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression>;
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>;
    crefs = Equation::foldExp(eq, (std::sync::Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> + 'static>), metamodelica::nil())?;
    iterationRange = deduceIterationRange(crefs, iterator, info)?;
    Ok(iterationRange)
}

pub(crate) fn deduceIterationRangeStmt(mut stmt: Arc<Statement::NFStatement>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression>;
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>;
    crefs = Statement::foldExp(stmt, (std::sync::Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> + 'static>), metamodelica::nil())?;
    iterationRange = deduceIterationRange(crefs, iterator, info)?;
    Ok(iterationRange)
}

pub(crate) fn deduceIterationRangeExp(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression>;
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>;
    crefs = Expression::fold(exp, (std::sync::Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0, __pe_a2| collectIteratorCrefs2(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> + 'static>), metamodelica::nil())?;
    iterationRange = deduceIterationRange(crefs, iterator, info)?;
    Ok(iterationRange)
}

pub(crate) fn deduceIterationRange(mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>, mut iterator: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut iterationRange: Arc<Expression::NFExpression>;
    let mut range_cr: (Arc<ComponentRef::NFComponentRef>, i32);
    let mut cr: Arc<ComponentRef::NFComponentRef>;
    let mut dim_index: i32;
    let mut dim: Arc<Dimension::NFDimension>;
    let mut start_exp: Arc<Expression::NFExpression>;
    let mut stop_exp: Arc<Expression::NFExpression>;
    if crefs.clone().is_empty() {
        Error::addSourceMessage(Error::IMPLICIT_ITERATOR_NOT_FOUND_IN_LOOP_BODY.clone(), list![(InstNode::name(iterator)?).clone()], info.clone())?;
        bail!("fail");
    }
    range_cr = List::reduce(crefs, (std::sync::Arc::new({ let __pe_b2 = info; move |__pe_a0, __pe_a1| deduceIterationRange2(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, i32), (Arc<ComponentRef::NFComponentRef>, i32)) -> Result<(Arc<ComponentRef::NFComponentRef>, i32)> + 'static>))?;
    (cr, dim_index) = range_cr;
    dim = Type::nthDimension(InstNode::getType(ComponentRef::node(cr.clone())?)?, dim_index)?;
    start_exp = Dimension::lowerBoundExp(dim.clone())?;
    stop_exp = Dimension::endExp(dim, Arc::new(Expression::NFExpression::CREF { ty: crate::NFType::interned_UNKNOWN(), cref: cr }), dim_index)?;
    iterationRange = Arc::new(Expression::NFExpression::RANGE { ty: crate::NFType::interned_UNKNOWN(), start: start_exp, step: None, stop: stop_exp });
    Ok(iterationRange)
}

pub(crate) fn collectIteratorCrefs(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> {
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = crefs;
    crefs = Expression::fold(exp, (std::sync::Arc::new({ let __pe_b1 = iterator; move |__pe_a0, __pe_a2| collectIteratorCrefs2(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> + 'static>), crefs)?;
    Ok(crefs)
}

pub(crate) fn collectIteratorCrefs2(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>> {
    let mut crefs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>> = crefs;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut index: i32 = 0;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::CREF { cref: __esc_cref, .. } => {
            cref = (*__esc_cref).clone();
            while ComponentRef::isCref(cref.clone()) {
                (cref, subs) = ComponentRef::stripSubscripts(cref.clone());
                index = 1;
                for mut sub in &*subs.clone() {
                    let mut sub = sub.clone();
                    if Subscript::equalsIterator(sub.clone(), iterator.clone())? {
                        crefs = metamodelica::cons((cref.clone(), index), crefs.clone());
                    }
                    index = index + 1;
                }
                cref = ComponentRef::rest(cref.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

pub(crate) fn deduceIterationRange2(mut range1: (Arc<ComponentRef::NFComponentRef>, i32), mut range2: (Arc<ComponentRef::NFComponentRef>, i32), mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, i32)> {
    let mut range: (Arc<ComponentRef::NFComponentRef>, i32) = range2.clone();
    let mut cref1: Arc<ComponentRef::NFComponentRef>;
    let mut cref2: Arc<ComponentRef::NFComponentRef>;
    let mut index1: i32;
    let mut index2: i32;
    let mut node1: Arc<InstNode::InstNode>;
    let mut node2: Arc<InstNode::InstNode>;
    let mut dim1: Arc<Dimension::NFDimension>;
    let mut dim2: Arc<Dimension::NFDimension>;
    (cref1, index1) = range1;
    (cref2, index2) = range2;
    node1 = ComponentRef::node(cref1.clone())?;
    node2 = ComponentRef::node(cref2.clone())?;
    if index1 == index2 && InstNode::refEqual(node1.clone(), node2.clone()) {
        return Ok(range.clone());
    }
    dim1 = Type::nthDimension(InstNode::getType(node1.clone())?, index1)?;
    dim2 = Type::nthDimension(InstNode::getType(node2.clone())?, index2)?;
    if !(Dimension::isEqualKnownSize(dim1, node1, index1, dim2, node2, index2)?) {
        Error::addSourceMessage(Error::INCOMPATIBLE_IMPLICIT_RANGES.clone(), list![ArcStr::from(::std::format!("{}", index1)), (ComponentRef::toString(cref1)?).clone(), ArcStr::from(::std::format!("{}", index2)), (ComponentRef::toString(cref2)?).clone()], info)?;
        bail!("fail");
    }
    Ok(range)
}

