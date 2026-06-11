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

use crate::BaseModelica;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode::Element;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFClass {
    NOT_INSTANTIATED,
    PARTIAL_CLASS {
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
    },
    PARTIAL_BUILTIN {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    EXPANDED_CLASS {
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    EXPANDED_DERIVED {
        baseClass: Arc<InstNode::InstNode>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        dims: metamodelica::Array<Arc<Dimension::NFDimension>>,
        prefixes: Arc<Prefixes::Prefixes>,
        attributes: Arc<Attributes::NFAttributes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    INSTANCED_CLASS {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        sections: Arc<Sections::NFSections>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    INSTANCED_BUILTIN {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    TYPED_DERIVED {
        ty: Arc<Type::NFType>,
        baseClass: Arc<InstNode::InstNode>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    DAE_TYPE {
        ty: Arc<DAE::Type>,
    },
}
impl metamodelica::gc::MMTrace for NFClass {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFClass::NOT_INSTANTIATED => Ok(()),
            NFClass::PARTIAL_CLASS { elements, modifier, ccMod, prefixes } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ccMod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                Ok(())
            }
            NFClass::PARTIAL_BUILTIN { ty, elements, modifier, prefixes, restriction } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::EXPANDED_CLASS { elements, modifier, ccMod, prefixes, restriction } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ccMod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::EXPANDED_DERIVED { baseClass, modifier, ccMod, dims, prefixes, attributes, restriction } => {
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ccMod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::INSTANCED_CLASS { ty, elements, sections, prefixes, restriction } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sections, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::INSTANCED_BUILTIN { ty, elements, restriction } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::TYPED_DERIVED { ty, baseClass, restriction } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::DAE_TYPE { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
        }
    }
}
impl NFClass {
    pub fn interned_NOT_INSTANTIATED() -> Arc<NFClass> {
        thread_local! {
            static INTERNED: Arc<NFClass> = Arc::new(NFClass::NOT_INSTANTIATED);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOT_INSTANTIATED() -> Arc<NFClass> { NFClass::interned_NOT_INSTANTIATED() }
impl Default for NFClass {
    fn default() -> Self { Self::NOT_INSTANTIATED }
}
pub use self::NFClass::{NOT_INSTANTIATED,PARTIAL_CLASS,PARTIAL_BUILTIN,EXPANDED_CLASS,EXPANDED_DERIVED,INSTANCED_CLASS,INSTANCED_BUILTIN,TYPED_DERIVED,DAE_TYPE};
pub(crate) static DEFAULT_PREFIXES: std::sync::LazyLock<Arc<Prefixes::Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(Prefixes::Prefixes { encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE() }) });

pub mod Prefixes {
    use super::*;
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Prefixes {
        pub encapsulatedPrefix: SCode::Encapsulated,
        pub partialPrefix: SCode::Partial,
        pub finalPrefix: SCode::Final,
        pub innerOuter: Absyn::InnerOuter,
        pub replaceablePrefix: Arc<SCode::Replaceable>,
    }

    impl metamodelica::gc::MMTrace for Prefixes {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.encapsulatedPrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partialPrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.finalPrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.replaceablePrefix, __mmv)?;
            Ok(())
        }
    }
    impl Default for Prefixes {
        fn default() -> Self {
            Self {
                encapsulatedPrefix: Default::default(),
                partialPrefix: Default::default(),
                finalPrefix: Default::default(),
                innerOuter: Default::default(),
                replaceablePrefix: Default::default(),
            }
        }
    }

    pub type PREFIXES = Prefixes;

    pub(crate) fn isEqual(mut prefs1: Arc<Prefixes>, mut prefs2: Arc<Prefixes>) -> bool {
        let mut isEqual: bool = prefs1.clone() == prefs2.clone();
        isEqual
    }

    pub(crate) fn isPartial(mut prefs: Arc<Prefixes>) -> Result<bool> {
        let mut isPartial: bool = SCodeUtil::partialBool(prefs.partialPrefix.clone())?;
        Ok(isPartial)
    }

    pub(crate) fn isEncapsulated(mut prefs: Arc<Prefixes>) -> Result<bool> {
        let mut isEncapsulated: bool = SCodeUtil::encapsulatedBool(prefs.encapsulatedPrefix.clone())?;
        Ok(isEncapsulated)
    }

}

pub(crate) fn fromSCode(mut elements: Arc<metamodelica::List<Arc<Element>>>, mut isClassExtends: bool, mut scope: Arc<InstNode::InstNode>, mut prefixes: Arc<Prefixes::Prefixes>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass>;
    let mut tree: Arc<ClassTree::ClassTree>;
    tree = ClassTree::fromSCode(elements, isClassExtends, scope)?;
    cls = Arc::new(NFClass::PARTIAL_CLASS { elements: tree, modifier: crate::NFModifier::Modifier::interned_NOMOD(), ccMod: crate::NFModifier::Modifier::interned_NOMOD(), prefixes: prefixes });
    Ok(cls)
}

pub(crate) fn initImports(mut cls: Arc<NFClass>, mut parent: Arc<InstNode::InstNode>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = ClassTree::initImports(var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone(), parent)?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn fromEnumeration(mut literals: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut enumType: Arc<Type::NFType>, mut prefixes: Arc<Prefixes::Prefixes>, mut enumClass: Arc<InstNode::InstNode>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass>;
    let mut tree: Arc<ClassTree::ClassTree>;
    tree = ClassTree::fromEnumeration(literals, enumType.clone(), enumClass)?;
    cls = Arc::new(NFClass::PARTIAL_BUILTIN { ty: enumType, elements: tree, modifier: crate::NFModifier::Modifier::interned_NOMOD(), prefixes: prefixes, restriction: crate::NFRestriction::interned_ENUMERATION() });
    Ok(cls)
}

pub(crate) fn makeRecordConstructor(mut fields: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut out: Arc<InstNode::InstNode>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass>;
    let mut tree: Arc<ClassTree::ClassTree>;
    tree = ClassTree::fromRecordConstructor(fields, out)?;
    cls = Arc::new(NFClass::INSTANCED_CLASS { ty: crate::NFType::interned_UNKNOWN(), elements: tree, sections: crate::NFSections::interned_EMPTY(), prefixes: DEFAULT_PREFIXES.clone(), restriction: crate::NFRestriction::interned_RECORD_CONSTRUCTOR() });
    Ok(cls)
}

pub(crate) fn initExpandedClass(mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    cls = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => Arc::new(NFClass::EXPANDED_CLASS { elements: var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone(), modifier: var_field!((*cls).modifier, NFClass::PARTIAL_CLASS).clone(), ccMod: var_field!((*cls).ccMod, NFClass::PARTIAL_CLASS).clone(), prefixes: var_field!((*cls).prefixes, NFClass::PARTIAL_CLASS).clone(), restriction: crate::NFRestriction::interned_UNKNOWN() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn getSections(mut cls: Arc<NFClass>) -> Result<Arc<Sections::NFSections>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => return Ok(var_field!((*cls).sections, NFClass::INSTANCED_CLASS).clone()),
        Deref @ TYPED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?; continue '__tco; },
        _ => return Ok(crate::NFSections::interned_EMPTY()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn setSections(mut sections: Arc<Sections::NFSections>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    cls = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => Arc::new(NFClass::INSTANCED_CLASS { ty: var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone(), elements: var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone(), sections: sections, prefixes: var_field!((*cls).prefixes, NFClass::INSTANCED_CLASS).clone(), restriction: var_field!((*cls).restriction, NFClass::INSTANCED_CLASS).clone() }),
        Deref @ TYPED_DERIVED { .. } => {
            InstNode::classApply(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone(), (std::sync::Arc::new(setSections) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Sections::NFSections>, Arc<NFClass>) -> Result<Arc<NFClass>> + 'static>), sections)?;
            cls
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub(crate) fn lookupElement(mut name: ArcStr, mut cls: Arc<NFClass>) -> Result<(Arc<InstNode::InstNode>, bool)> {
    let mut node: Arc<InstNode::InstNode>;
    let mut isImport: bool;
    (node, isImport) = ClassTree::lookupElement((name).clone(), classTree(cls)?)?;
    Ok((node, isImport))
}

pub(crate) fn tryLookupElement(mut name: ArcStr, mut cls: Arc<NFClass>) -> (Option<Arc<InstNode::InstNode>>, bool) {
    let mut node: Option<Arc<InstNode::InstNode>>;
    let mut isImport: bool;
    let mut n: Arc<InstNode::InstNode>;
    match '__try0: {
        (n, isImport) = unwrap_break_err!(ClassTree::lookupElement((name.clone()).clone(), unwrap_break_err!(classTree(cls.clone()), '__try0)), '__try0);
        node = Some(n.clone());
        Ok::<_, anyhow::Error>((isImport.clone(), node.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            isImport = __try0_o0;
            node = __try0_o1;
        }
        Err(_) => {
            node = None;
            isImport = false;
        }
    }
    (node, isImport)
}

pub(crate) fn lookupComponentIndex(mut name: ArcStr, mut cls: Arc<NFClass>) -> Result<i32> {
    let mut index: i32;
    index = ClassTree::lookupComponentIndex((name).clone(), classTree(cls)?)?;
    Ok(index)
}

pub(crate) fn nthComponent(mut index: i32, mut cls: Arc<NFClass>) -> Result<Arc<InstNode::InstNode>> {
    let mut component: Arc<InstNode::InstNode>;
    component = ClassTree::nthComponent(index, classTree(cls)?)?;
    Ok(component)
}

pub fn getComponents(mut cls: Arc<NFClass>) -> Result<metamodelica::Array<Arc<InstNode::InstNode>>> {
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>> = ClassTree::getComponents(classTree(cls.clone())?)?;
    Ok(comps)
}

pub(crate) fn lookupAttributeBinding(mut name: ArcStr, mut cls: Arc<NFClass>) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding>;
    let mut attr_node: Arc<InstNode::InstNode>;
    match '__try0: {
        (attr_node, _) = unwrap_break_err!(ClassTree::lookupElement((name.clone()).clone(), unwrap_break_err!(classTree(cls.clone()), '__try0)), '__try0);
        binding = Component::getBinding(unwrap_break_err!(InstNode::component(attr_node.clone()), '__try0));
        Ok::<_, anyhow::Error>((binding.clone(),))
    } {
        Ok((__try0_o0,)) => {
            binding = __try0_o0;
        }
        Err(_) => {
            binding = Binding::EMPTY_BINDING().clone();
        }
    }
    binding
}

pub(crate) fn lookupAttributeValue(mut name: ArcStr, mut cls: Arc<NFClass>) -> Option<Arc<Expression::NFExpression>> {
    let mut value: Option<Arc<Expression::NFExpression>> = Binding::typedExp(lookupAttributeBinding((name.clone()).clone(), cls.clone()));
    value
}

pub fn isOnlyBuiltin(mut cls: Arc<NFClass>) -> bool {
    let mut builtin: bool;
    builtin = (::match_deref::match_deref! { match &(cls) {
        Deref @ PARTIAL_BUILTIN { .. } => true,
        Deref @ INSTANCED_BUILTIN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    builtin
}

pub(crate) fn isBuiltin(mut cls: Arc<NFClass>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => return Ok(true),
        Deref @ INSTANCED_BUILTIN { .. } => return Ok(true),
        Deref @ EXPANDED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?; continue '__tco; },
        Deref @ TYPED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?; continue '__tco; },
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn classTree(mut cls: Arc<NFClass>) -> Result<Arc<ClassTree::ClassTree>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => return Ok(var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone()),
        Deref @ PARTIAL_BUILTIN { .. } => return Ok(var_field!((*cls).elements, NFClass::PARTIAL_BUILTIN).clone()),
        Deref @ EXPANDED_CLASS { .. } => return Ok(var_field!((*cls).elements, NFClass::EXPANDED_CLASS).clone()),
        Deref @ EXPANDED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?; continue '__tco; },
        Deref @ INSTANCED_CLASS { .. } => return Ok(var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone()),
        Deref @ INSTANCED_BUILTIN { .. } => return Ok(var_field!((*cls).elements, NFClass::INSTANCED_BUILTIN).clone()),
        Deref @ TYPED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?; continue '__tco; },
        _ => return Ok(crate::NFClassTree::ClassTree::interned_EMPTY_TREE()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn setClassTree(mut tree: Arc<ClassTree::ClassTree>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = tree);
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; elements = tree);
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; elements = tree);
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; elements = tree);
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; elements = tree);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub(crate) fn classTreeApply(mut cls: Arc<NFClass>, mut func: Arc<dyn ::std::ops::Fn(Arc<ClassTree::ClassTree>) -> Result<Arc<ClassTree::ClassTree>> + 'static>) -> Result<Arc<NFClass>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<ClassTree::ClassTree>) -> Result<Arc<ClassTree::ClassTree>> + 'static>;

    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = func(var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone())?);
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; elements = func(var_field!((*cls).elements, NFClass::EXPANDED_CLASS).clone())?);
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; elements = func(var_field!((*cls).elements, NFClass::PARTIAL_BUILTIN).clone())?);
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; elements = func(var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone())?);
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; elements = func(var_field!((*cls).elements, NFClass::INSTANCED_BUILTIN).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn getModifier(mut cls: Arc<NFClass>) -> Arc<Modifier::Modifier> {
    let mut modifier: Arc<Modifier::Modifier>;
    modifier = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => var_field!((*cls).modifier, NFClass::PARTIAL_CLASS).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).modifier, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).modifier, NFClass::EXPANDED_DERIVED).clone(),
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).modifier, NFClass::PARTIAL_BUILTIN).clone(),
        _ => crate::NFModifier::Modifier::interned_NOMOD(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    modifier
}

pub(crate) fn getCCModifier(mut cls: Arc<NFClass>) -> Arc<Modifier::Modifier> {
    let mut modifier: Arc<Modifier::Modifier>;
    modifier = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => var_field!((*cls).ccMod, NFClass::PARTIAL_CLASS).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).ccMod, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).ccMod, NFClass::EXPANDED_DERIVED).clone(),
        _ => crate::NFModifier::Modifier::interned_NOMOD(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    modifier
}

pub(crate) fn setModifier(mut modifier: Arc<Modifier::Modifier>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; modifier = modifier);
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; modifier = modifier);
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; modifier = modifier);
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; modifier = modifier);
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClass.setModifier")); __mm_s.push_str(&*literal!(" got non-modifiable class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFClass.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn mergeModifier(mut modifier: Arc<Modifier::Modifier>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; modifier = Modifier::merge(modifier, var_field!((*cls).modifier, NFClass::PARTIAL_CLASS).clone(), (literal!("")).clone())?);
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; modifier = Modifier::merge(modifier, var_field!((*cls).modifier, NFClass::EXPANDED_CLASS).clone(), (literal!("")).clone())?);
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; modifier = Modifier::merge(modifier, var_field!((*cls).modifier, NFClass::EXPANDED_DERIVED).clone(), (literal!("")).clone())?);
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; modifier = Modifier::merge(modifier, var_field!((*cls).modifier, NFClass::PARTIAL_BUILTIN).clone(), (literal!("")).clone())?);
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClass.mergeModifier")); __mm_s.push_str(&*literal!(" got non-modifiable class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFClass.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn isIdentical(mut cls1: Arc<NFClass>, mut cls2: Arc<NFClass>) -> Result<bool> {
    let mut identical: bool = false;
    if referenceEq(&*(cls1.clone()),&*(cls2.clone())) {
        identical = true;
    } else {
        identical = (::match_deref::match_deref! { match &((cls1.clone(), cls2.clone())) {
        (Deref @ EXPANDED_CLASS { .. }, Deref @ EXPANDED_CLASS { .. }) => Prefixes::isEqual(var_field!((*cls1).prefixes, NFClass::EXPANDED_CLASS).clone(), var_field!((*cls2).prefixes, NFClass::EXPANDED_CLASS).clone()) && ClassTree::isIdentical(var_field!((*cls1).elements, NFClass::EXPANDED_CLASS).clone(), var_field!((*cls2).elements, NFClass::EXPANDED_CLASS).clone()),
        (Deref @ INSTANCED_BUILTIN { .. }, Deref @ INSTANCED_BUILTIN { .. }) => {
            if !(Type::isEqual(var_field!((*cls1).ty, NFClass::INSTANCED_BUILTIN).clone(), var_field!((*cls2).ty, NFClass::INSTANCED_BUILTIN).clone())?) {
                return Ok(identical.clone());
            }
            true
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(identical)
}

pub(crate) fn hasDimensions(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut hasDims: bool;
    hasDims = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => metamodelica::arrayLength(var_field!((*cls).dims, NFClass::EXPANDED_DERIVED).clone()) > 0 || hasDimensions(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?)?,
        Deref @ TYPED_DERIVED { .. } => Type::isArray(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasDims)
}

pub(crate) fn getDimensions(mut cls: Arc<NFClass>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    dims = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => Type::arrayDims(var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone()),
        Deref @ INSTANCED_BUILTIN { .. } => Type::arrayDims(var_field!((*cls).ty, NFClass::INSTANCED_BUILTIN).clone()),
        Deref @ TYPED_DERIVED { .. } => Type::arrayDims(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dims
}

pub(crate) fn dimensionCount(mut cls: Arc<NFClass>) -> i32 {
    let mut count: i32;
    count = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => metamodelica::arrayLength(var_field!((*cls).dims, NFClass::EXPANDED_DERIVED).clone()),
        Deref @ INSTANCED_CLASS { .. } => Type::dimensionCount(var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone()),
        Deref @ INSTANCED_BUILTIN { .. } => Type::dimensionCount(var_field!((*cls).ty, NFClass::INSTANCED_BUILTIN).clone()),
        Deref @ TYPED_DERIVED { .. } => Type::dimensionCount(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    count
}

pub(crate) fn getAttributes(mut cls: Arc<NFClass>) -> Arc<Attributes::NFAttributes> {
    let mut attr: Arc<Attributes::NFAttributes>;
    attr = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).attributes, NFClass::EXPANDED_DERIVED).clone(),
        _ => Attributes::DEFAULT_ATTR().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

pub(crate) fn getTypeAttributes(mut cls: Arc<NFClass>) -> Arc<metamodelica::List<Arc<Modifier::Modifier>>> {
    let mut attributes: Arc<metamodelica::List<Arc<Modifier::Modifier>>> = metamodelica::nil();
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut r#mod: Arc<Modifier::Modifier>;
    if '__try0: {
        comps = unwrap_break_err!(ClassTree::getComponents(unwrap_break_err!(classTree(cls.clone()), '__try0)), '__try0);
        let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range1 {
            r#mod = Component::getModifier(unwrap_break_err!(InstNode::component(c.clone()), '__try0));
            if !(Modifier::isEmpty(r#mod.clone())) {
                attributes = metamodelica::cons(r#mod.clone(), attributes.clone());
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    attributes
}

pub(crate) fn getType(mut cls: Arc<NFClass>, mut clsNode: Arc<InstNode::InstNode>) -> Result<Arc<Type::NFType>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => return Ok(var_field!((*cls).ty, NFClass::PARTIAL_BUILTIN).clone()),
        Deref @ EXPANDED_DERIVED { .. } => { (cls, clsNode) = (InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?, var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone()); continue '__tco; },
        Deref @ INSTANCED_CLASS { .. } => return Ok(var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone()),
        Deref @ INSTANCED_BUILTIN { .. } => return Ok(var_field!((*cls).ty, NFClass::INSTANCED_BUILTIN).clone()),
        Deref @ TYPED_DERIVED { .. } => return Ok(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => return Ok(crate::NFType::interned_UNKNOWN()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn setType(mut ty: Arc<Type::NFType>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; ty = ty);
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            InstNode::classApply(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone(), (std::sync::Arc::new(setType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>, Arc<NFClass>) -> Result<Arc<NFClass>> + 'static>), ty)?;
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; ty = ty);
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; ty = ty);
            ()
        },
        Deref @ TYPED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::TYPED_DERIVED; ty = ty);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn restriction(mut cls: Arc<NFClass>) -> Arc<Restriction::NFRestriction> {
    let mut res: Arc<Restriction::NFRestriction>;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).restriction, NFClass::PARTIAL_BUILTIN).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).restriction, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).restriction, NFClass::EXPANDED_DERIVED).clone(),
        Deref @ INSTANCED_CLASS { .. } => var_field!((*cls).restriction, NFClass::INSTANCED_CLASS).clone(),
        Deref @ INSTANCED_BUILTIN { .. } => var_field!((*cls).restriction, NFClass::INSTANCED_BUILTIN).clone(),
        Deref @ TYPED_DERIVED { .. } => var_field!((*cls).restriction, NFClass::TYPED_DERIVED).clone(),
        _ => crate::NFRestriction::interned_UNKNOWN(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn setRestriction(mut res: Arc<Restriction::NFRestriction>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; restriction = res);
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; restriction = res);
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; restriction = res);
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; restriction = res);
            ()
        },
        Deref @ TYPED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::TYPED_DERIVED; restriction = res);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub(crate) fn isConnectorClass(mut cls: Arc<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isConnector(restriction(cls.clone()));
    isConnector
}

pub(crate) fn isNonexpandableConnectorClass(mut cls: Arc<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isNonexpandableConnector(restriction(cls.clone()));
    isConnector
}

pub(crate) fn isExpandableConnectorClass(mut cls: Arc<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isExpandableConnector(restriction(cls.clone()));
    isConnector
}

pub(crate) fn isExternalObject(mut cls: Arc<NFClass>) -> bool {
    let mut isExternalObject: bool = Restriction::isExternalObject(restriction(cls.clone()));
    isExternalObject
}

pub(crate) fn isFunction(mut cls: Arc<NFClass>) -> bool {
    let mut isFunction: bool = Restriction::isFunction(restriction(cls.clone()));
    isFunction
}

pub fn isEnumeration(mut cls: Arc<NFClass>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { .. }, .. } => return Ok(true),
        Deref @ INSTANCED_BUILTIN { ty: Deref @ Type::ENUMERATION { .. }, .. } => return Ok(true),
        Deref @ EXPANDED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?; continue '__tco; },
        Deref @ TYPED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?; continue '__tco; },
        _ => return Ok(false),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isExternalFunction(mut cls: Arc<NFClass>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => {
            { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?; continue '__tco; }
        },
        Deref @ INSTANCED_CLASS { sections: Deref @ Sections::EXTERNAL { language: lang, .. }, .. } => {
            return Ok(lang.clone() != literal!("builtin"))
        },
        Deref @ TYPED_DERIVED { .. } => {
            { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?; continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isOverdetermined(mut cls: Arc<NFClass>) -> bool {
    let mut isOverdetermined: bool;
    match '__try0: {
        unwrap_break_err!(lookupElement((literal!("equalityConstraint")).clone(), cls.clone()), '__try0);
        System::setHasOverconstrainedConnectors(true);
        isOverdetermined = true;
        Ok::<_, anyhow::Error>((isOverdetermined.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isOverdetermined = __try0_o0;
        }
        Err(_) => {
            isOverdetermined = false;
        }
    }
    isOverdetermined
}

pub(crate) fn getPrefixes(mut cls: Arc<NFClass>) -> Result<Arc<Prefixes::Prefixes>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => return Ok(var_field!((*cls).prefixes, NFClass::PARTIAL_CLASS).clone()),
        Deref @ PARTIAL_BUILTIN { .. } => return Ok(var_field!((*cls).prefixes, NFClass::PARTIAL_BUILTIN).clone()),
        Deref @ EXPANDED_CLASS { .. } => return Ok(var_field!((*cls).prefixes, NFClass::EXPANDED_CLASS).clone()),
        Deref @ EXPANDED_DERIVED { .. } => return Ok(var_field!((*cls).prefixes, NFClass::EXPANDED_DERIVED).clone()),
        Deref @ INSTANCED_CLASS { .. } => return Ok(var_field!((*cls).prefixes, NFClass::INSTANCED_CLASS).clone()),
        Deref @ TYPED_DERIVED { .. } => { cls = InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?; continue '__tco; },
        _ => return Ok(DEFAULT_PREFIXES.clone()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn setPrefixes(mut prefs: Arc<Prefixes::Prefixes>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; prefixes = prefs);
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; prefixes = prefs);
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; prefixes = prefs);
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; prefixes = prefs);
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; prefixes = prefs);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub(crate) fn isEncapsulated(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut isEncapsulated: bool = Prefixes::isEncapsulated(getPrefixes(cls.clone())?)?;
    Ok(isEncapsulated)
}

pub(crate) fn isPartial(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut isPartial: bool = Prefixes::isPartial(getPrefixes(cls.clone())?)?;
    Ok(isPartial)
}

pub(crate) fn lastBaseClass(mut node: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cls: Arc<NFClass> = InstNode::getClass(node.clone())?;
    node = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => lastBaseClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?,
        Deref @ TYPED_DERIVED { .. } => lastBaseClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?,
        _ => node,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub(crate) fn getDerivedComments(mut cls: Arc<NFClass>, mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Comment>>>> {
    let mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>> = cmts;
    cmts = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => InstNode::getComments(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone(), cmts)?,
        Deref @ TYPED_DERIVED { .. } => InstNode::getComments(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone(), cmts)?,
        _ => {
            let __range0 = ClassTree::getExtends(classTree(cls)?).borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range0 {
                cmts = InstNode::getComments(ext.clone(), cmts.clone())?;
            }
            cmts
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cmts)
}

pub fn constrainingClassPath(mut clsNode: Arc<InstNode::InstNode>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut cls_node: Arc<InstNode::InstNode> = lastBaseClass(clsNode.clone())?;
    let mut prefs: Arc<Prefixes::Prefixes> = getPrefixes(InstNode::getClass(cls_node.clone())?)?;
    path = (::match_deref::match_deref! { match &(prefs) {
        Deref @ Prefixes::PREFIXES { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: __esc_path, .. }) }, .. } => {
            path = (*__esc_path).clone();
            path.clone()
        },
        _ => InstNode::enclosingScopePath(cls_node, false, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(path)
}

pub fn hasOperator(mut name: ArcStr, mut cls: Arc<NFClass>) -> bool {
    let mut hasOperator: bool;
    let mut op_node: Arc<InstNode::InstNode>;
    if Restriction::isOperatorRecord(restriction(cls.clone())) {
        match '__try0: {
            (op_node, _) = unwrap_break_err!(lookupElement((name.clone()).clone(), cls.clone()), '__try0);
            hasOperator = SCodeUtil::isOperator(unwrap_break_err!(InstNode::definition(op_node.clone()), '__try0));
            Ok::<_, anyhow::Error>((hasOperator.clone(),))
        } {
            Ok((__try0_o0,)) => {
                hasOperator = __try0_o0;
            }
            Err(_) => {
                hasOperator = false;
            }
        }
    } else {
        hasOperator = false;
    }
    hasOperator
}

pub(crate) fn makeRecordExp(mut clsNode: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut typed: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut cls: Arc<NFClass>;
    let mut ty: Arc<Type::NFType>;
    let mut ty_node: Arc<InstNode::InstNode>;
    let mut fields: Arc<metamodelica::List<Arc<Record::Field::Field>>>;
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    cls = InstNode::getClass(clsNode.clone())?;
    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(getType(cls.clone(), clsNode)?) {
        __pa1 @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: __pa0, .. }, .. } => (__pa1.clone(), __pa0.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty_node = __pa0.clone();
    ty = __pa1.clone();
    comps = ClassTree::getComponents(classTree(cls)?)?;
    if typed {
        args = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut c in (comps.clone()).borrow().iter() {
            let __x = Binding::getExp(Component::getImplicitBinding(InstNode::component(c.clone())?, scope.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        exp = Expression::makeRecord(InstNode::fullPath(ty_node, false)?, ty, args);
    } else {
        args = metamodelica::nil();
        let __range3 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range3 {
            fields = Record::collectRecordField(c.clone(), metamodelica::nil())?;
            if !(fields.clone().is_empty()) && Record::Field::isInput(listHead(fields.clone())?) {
                args = metamodelica::cons(Binding::getExp(Component::getImplicitBinding(InstNode::component(c.clone())?, scope.clone()))?, args.clone());
            }
        }
        args = metamodelica::Dangerous::listReverseInPlace(args);
        exp = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: ComponentRef::fromNode(ty_node, ty, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), arguments: args, named_args: metamodelica::nil(), call_scope: scope }) });
    }
    Ok(exp)
}

pub(crate) fn toFlatStream(mut cls: Arc<NFClass>, mut clsNode: Arc<InstNode::InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr;
    name = (Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(clsNode, InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?).clone();
    s = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => {
            s = IOStream::append(s, (indent.clone()).clone())?;
            s = IOStream::append(s, (Restriction::toString(var_field!((*cls).restriction, NFClass::INSTANCED_CLASS).clone())).clone())?;
            s = IOStream::append(s, (literal!(" ")).clone())?;
            s = IOStream::append(s, (name.clone()).clone())?;
            s = IOStream::append(s, (literal!("\n")).clone())?;
            let __range0 = ClassTree::getComponents(var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone())?.borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                s = IOStream::append(s.clone(), (InstNode::toFlatString(comp.clone(), format, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            }
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end ")).clone())?;
            s = IOStream::append(s, (name).clone())?;
            s
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("INSTANCED_BUILTIN(")).clone())?;
            s = IOStream::append(s, (name).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ TYPED_DERIVED { .. } => {
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (Restriction::toString(var_field!((*cls).restriction, NFClass::TYPED_DERIVED).clone())).clone())?;
            s = IOStream::append(s, (literal!(" ")).clone())?;
            s = IOStream::append(s, (name).clone())?;
            s = IOStream::append(s, (literal!(" = ")).clone())?;
            s = IOStream::append(s, (Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?).clone())?;
            s
        },
        _ => IOStream::append(s, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKNOWN_CLASS(")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn toFlatString(mut cls: Arc<NFClass>, mut clsNode: Arc<InstNode::InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(literal!("NFClass.toFlatString"), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toFlatStream(cls, clsNode, format, (indent).clone(), s)?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s)?;
    Ok(r#str)
}


