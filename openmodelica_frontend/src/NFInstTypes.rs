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

use openmodelica_ast::Absyn;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;

//public import NFConnect2;
pub type Prefix = Arc<NFInstPrefix::Prefix>;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Element {
    ELEMENT {
        component: Arc<Component>,
        cls: Arc<Class>,
    },
    CONDITIONAL_ELEMENT {
        component: Arc<Component>,
    },
    /// This record is used by NFInst.instElementList to store elements from
    ///     extends, but is removed by instFlatten. Most functions which handle
    ///     elements should therefore be able to ignore this record.
    EXTENDED_ELEMENTS {
        baseClass: Arc<Absyn::Path>,
        cls: Arc<Class>,
        ty: Arc<DAE::Type>,
    },
}
impl metamodelica::gc::MMTrace for Element {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Element::ELEMENT { component, cls } => {
                metamodelica::gc::MMTrace::mm_accept(component, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                Ok(())
            }
            Element::CONDITIONAL_ELEMENT { component } => {
                metamodelica::gc::MMTrace::mm_accept(component, __mmv)?;
                Ok(())
            }
            Element::EXTENDED_ELEMENTS { baseClass, cls, ty } => {
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Element::{ELEMENT,CONDITIONAL_ELEMENT,EXTENDED_ELEMENTS};

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Class {
    COMPLEX_CLASS {
        name: Arc<Absyn::Path>,
        components: Arc<metamodelica::List<Arc<Element>>>,
        equations: Arc<metamodelica::List<Arc<Equation>>>,
        initialEquations: Arc<metamodelica::List<Arc<Equation>>>,
        algorithms: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement>>>>>,
        initialAlgorithms: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement>>>>>,
    },
    BASIC_TYPE {
        name: Arc<Absyn::Path>,
    },
}
impl metamodelica::gc::MMTrace for Class {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Class::COMPLEX_CLASS { name, components, equations, initialEquations, algorithms, initialAlgorithms } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialEquations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialAlgorithms, __mmv)?;
                Ok(())
            }
            Class::BASIC_TYPE { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Class::{COMPLEX_CLASS,BASIC_TYPE};

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Function {
    /// A function has inputs, output and locals without binding.
    ///     These are resolved to statements in the algorithm section
    FUNCTION {
        path: Arc<Absyn::Path>,
        inputs: Arc<metamodelica::List<Arc<Element>>>,
        outputs: Arc<metamodelica::List<Arc<Element>>>,
        locals: Arc<metamodelica::List<Arc<Element>>>,
        /// TODO: Add default bindings
        algorithms: Arc<metamodelica::List<Arc<Statement>>>,
    },
    /// A record constructor has inputs and locals (with bindings)?
    RECORD_CONSTRUCTOR {
        path: Arc<Absyn::Path>,
        recType: Arc<DAE::Type>,
        /// componets of the original record which CAN be modified
        inputs: Arc<metamodelica::List<Arc<Element>>>,
        /// componets of the original record which CAN NOT be modified (protected, final, constant WITH binding)
        locals: Arc<metamodelica::List<Arc<Element>>>,
        /// TODO: Add default bindings
        algorithms: Arc<metamodelica::List<Arc<Statement>>>,
    },
}
impl metamodelica::gc::MMTrace for Function {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Function::FUNCTION { path, inputs, outputs, locals, algorithms } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                Ok(())
            }
            Function::RECORD_CONSTRUCTOR { path, recType, inputs, locals, algorithms } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(recType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Function::{FUNCTION,RECORD_CONSTRUCTOR};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Dimension {
    UNTYPED_DIMENSION {
        dimension: Arc<DAE::Dimension>,
        isProcessing: bool,
    },
    TYPED_DIMENSION {
        dimension: Arc<DAE::Dimension>,
    },
}
impl metamodelica::gc::MMTrace for Dimension {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Dimension::UNTYPED_DIMENSION { dimension, isProcessing } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                Ok(())
            }
            Dimension::TYPED_DIMENSION { dimension } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Dimension::{UNTYPED_DIMENSION,TYPED_DIMENSION};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Binding {
    UNBOUND,
    RAW_BINDING {
        bindingExp: Arc<Absyn::Exp>,
        env: Env,
        /// See NFSCodeMod.propagateMod.
        propagatedDims: i32,
        info: SourceInfo,
    },
    UNTYPED_BINDING {
        bindingExp: Arc<DAE::Exp>,
        isProcessing: bool,
        /// See NFSCodeMod.propagateMod.
        propagatedDims: i32,
        info: SourceInfo,
    },
    TYPED_BINDING {
        bindingExp: Arc<DAE::Exp>,
        bindingType: Arc<DAE::Type>,
        /// See NFSCodeMod.propagateMod.
        propagatedDims: i32,
        info: SourceInfo,
    },
}
impl metamodelica::gc::MMTrace for Binding {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Binding::UNBOUND => Ok(()),
            Binding::RAW_BINDING { bindingExp, env, propagatedDims, info } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(propagatedDims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Binding::UNTYPED_BINDING { bindingExp, isProcessing, propagatedDims, info } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(propagatedDims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Binding::TYPED_BINDING { bindingExp, bindingType, propagatedDims, info } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(bindingType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(propagatedDims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Binding::{UNBOUND,RAW_BINDING,UNTYPED_BINDING,TYPED_BINDING};

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Component {
    UNTYPED_COMPONENT {
        name: Arc<Absyn::Path>,
        baseType: Arc<DAE::Type>,
        dimensions: metamodelica::Array<Dimension>,
        prefixes: Prefixes,
        paramType: ParamType,
        binding: Binding,
        info: SourceInfo,
    },
    TYPED_COMPONENT {
        name: Arc<Absyn::Path>,
        ty: Arc<DAE::Type>,
        parent: Option<Arc<Component>>,
        prefixes: DaePrefixes,
        binding: Binding,
        info: SourceInfo,
    },
    CONDITIONAL_COMPONENT {
        name: Arc<Absyn::Path>,
        condition: Arc<DAE::Exp>,
        element: Arc<SCode::Element>,
        modifier: Arc<Modifier>,
        prefixes: Prefixes,
        env: Env,
        prefix: Prefix,
        info: SourceInfo,
    },
    DELETED_COMPONENT {
        name: Arc<Absyn::Path>,
    },
    OUTER_COMPONENT {
        name: Arc<Absyn::Path>,
        innerName: Option<Arc<Absyn::Path>>,
    },
    COMPONENT_ALIAS {
        componentName: Arc<Absyn::Path>,
    },
}
impl metamodelica::gc::MMTrace for Component {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Component::UNTYPED_COMPONENT { name, baseType, dimensions, prefixes, paramType, binding, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(paramType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Component::TYPED_COMPONENT { name, ty, parent, prefixes, binding, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Component::CONDITIONAL_COMPONENT { name, condition, element, modifier, prefixes, env, prefix, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Component::DELETED_COMPONENT { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            Component::OUTER_COMPONENT { name, innerName } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerName, __mmv)?;
                Ok(())
            }
            Component::COMPONENT_ALIAS { componentName } => {
                metamodelica::gc::MMTrace::mm_accept(componentName, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Component::{UNTYPED_COMPONENT,TYPED_COMPONENT,CONDITIONAL_COMPONENT,DELETED_COMPONENT,OUTER_COMPONENT,COMPONENT_ALIAS};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Condition {
    SINGLE_CONDITION {
        condition: bool,
    },
    ARRAY_CONDITION {
        conditions: Arc<metamodelica::List<Arc<Condition>>>,
    },
}
impl metamodelica::gc::MMTrace for Condition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Condition::SINGLE_CONDITION { condition } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                Ok(())
            }
            Condition::ARRAY_CONDITION { conditions } => {
                metamodelica::gc::MMTrace::mm_accept(conditions, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Condition::{SINGLE_CONDITION,ARRAY_CONDITION};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ParamType {
    /// Not a parameter.
    NON_PARAM,
    /// A non-structural parameter.
    NON_STRUCT_PARAM,
    /// A structural parameter.
    STRUCT_PARAM,
}
impl metamodelica::gc::MMTrace for ParamType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ParamType::NON_PARAM => Ok(()),
            ParamType::NON_STRUCT_PARAM => Ok(()),
            ParamType::STRUCT_PARAM => Ok(()),
        }
    }
}
pub use self::ParamType::{NON_PARAM,NON_STRUCT_PARAM,STRUCT_PARAM};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Modifier {
    MODIFIER {
        name: ArcStr,
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        binding: Binding,
        subModifiers: Arc<metamodelica::List<Arc<Modifier>>>,
        info: SourceInfo,
    },
    REDECLARE {
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        element: Arc<SCode::Element>,
        env: Env,
        r#mod: Arc<Modifier>,
        constrainingClass: Option<Arc<ConstrainingClass>>,
    },
    NOMOD,
}
impl metamodelica::gc::MMTrace for Modifier {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Modifier::MODIFIER { name, finalPrefix, eachPrefix, binding, subModifiers, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subModifiers, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Modifier::REDECLARE { finalPrefix, eachPrefix, element, env, r#mod, constrainingClass } => {
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(constrainingClass, __mmv)?;
                Ok(())
            }
            Modifier::NOMOD => Ok(()),
        }
    }
}
impl Modifier {
    pub fn interned_NOMOD() -> Arc<Modifier> {
        thread_local! {
            static INTERNED: Arc<Modifier> = Arc::new(Modifier::NOMOD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOMOD() -> Arc<Modifier> { Modifier::interned_NOMOD() }
pub use self::Modifier::{MODIFIER,REDECLARE,NOMOD};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ConstrainingClass {
    pub classPath: Arc<Absyn::Path>,
    pub r#mod: Arc<Modifier>,
}

impl metamodelica::gc::MMTrace for ConstrainingClass {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.classPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#mod, __mmv)?;
        Ok(())
    }
}
pub type CONSTRAINING_CLASS = ConstrainingClass;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Prefixes {
    NO_PREFIXES,
    PREFIXES {
        visibility: SCode::Visibility,
        variability: SCode::Variability,
        finalPrefix: SCode::Final,
        innerOuter: Absyn::InnerOuter,
        direction: (Absyn::Direction, SourceInfo),
        connectorType: (SCode::ConnectorType, SourceInfo),
        varArgs: VarArgs,
    },
}
impl metamodelica::gc::MMTrace for Prefixes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Prefixes::NO_PREFIXES => Ok(()),
            Prefixes::PREFIXES { visibility, variability, finalPrefix, innerOuter, direction, connectorType, varArgs } => {
                metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerOuter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(direction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(connectorType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varArgs, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Prefixes::{NO_PREFIXES,PREFIXES};

pub(crate) static DEFAULT_PROTECTED_PREFIXES: std::sync::LazyLock<Prefixes> = std::sync::LazyLock::new(|| { Prefixes::PREFIXES { visibility: openmodelica_frontend_types::SCode::Visibility::PROTECTED, variability: openmodelica_frontend_types::SCode::Variability::VAR, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, direction: (openmodelica_ast::Absyn::Direction::BIDIR, Absyn::dummyInfo.clone()), connectorType: (openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, Absyn::dummyInfo.clone()), varArgs: crate::NFInstTypes::VarArgs::NO_VARARG } });

pub(crate) static DEFAULT_INPUT_PREFIXES: std::sync::LazyLock<Prefixes> = std::sync::LazyLock::new(|| { Prefixes::PREFIXES { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, variability: openmodelica_frontend_types::SCode::Variability::VAR, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, direction: (openmodelica_ast::Absyn::Direction::INPUT, Absyn::dummyInfo.clone()), connectorType: (openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, Absyn::dummyInfo.clone()), varArgs: crate::NFInstTypes::VarArgs::NO_VARARG } });

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarArgs {
    NO_VARARG,
    IS_VARARG,
}
impl metamodelica::gc::MMTrace for VarArgs {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarArgs::NO_VARARG => Ok(()),
            VarArgs::IS_VARARG => Ok(()),
        }
    }
}
pub use self::VarArgs::{NO_VARARG,IS_VARARG};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum DaePrefixes {
    NO_DAE_PREFIXES,
    DAE_PREFIXES {
        visibility: DAE::VarVisibility,
        variability: DAE::VarKind,
        finalPrefix: SCode::Final,
        innerOuter: Absyn::InnerOuter,
        direction: DAE::VarDirection,
        connectorType: Arc<DAE::ConnectorType>,
    },
}
impl metamodelica::gc::MMTrace for DaePrefixes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            DaePrefixes::NO_DAE_PREFIXES => Ok(()),
            DaePrefixes::DAE_PREFIXES { visibility, variability, finalPrefix, innerOuter, direction, connectorType } => {
                metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerOuter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(direction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(connectorType, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::DaePrefixes::{NO_DAE_PREFIXES,DAE_PREFIXES};

thread_local! { static __DEFAULT_CONST_DAE_PREFIXES_TLS: DaePrefixes = DaePrefixes::DAE_PREFIXES { visibility: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC, variability: openmodelica_frontend_types::DAE::VarKind::CONST, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR() }; }
pub(crate) fn DEFAULT_CONST_DAE_PREFIXES() -> DaePrefixes { __DEFAULT_CONST_DAE_PREFIXES_TLS.with(|__t| __t.clone()) }

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Equation {
    EQUALITY_EQUATION {
        /// The left hand side expression.
        lhs: Arc<DAE::Exp>,
        /// The right hand side expression.
        rhs: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    CONNECT_EQUATION {
        /// The left hand side component.
        lhs: Arc<DAE::ComponentRef>,
        /// The type of the lhs component.
        lhsType: Arc<DAE::Type>,
        /// The right hand side component.
        rhs: Arc<DAE::ComponentRef>,
        /// The type of the rhs component.
        rhsType: Arc<DAE::Type>,
        prefix: Prefix,
        info: SourceInfo,
    },
    FOR_EQUATION {
        /// The name of the iterator variable.
        name: ArcStr,
        /// The index of the iterator variable.
        index: i32,
        /// The type of the index/iterator variable.
        indexType: Arc<DAE::Type>,
        /// The range expression to loop over.
        range: Option<Arc<DAE::Exp>>,
        /// The body of the for loop.
        body: Arc<metamodelica::List<Arc<Equation>>>,
        info: SourceInfo,
    },
    IF_EQUATION {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Equation>>>)>>,
        info: SourceInfo,
    },
    WHEN_EQUATION {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Equation>>>)>>,
        info: SourceInfo,
    },
    ASSERT_EQUATION {
        /// The assert condition.
        condition: Arc<DAE::Exp>,
        /// The message to display if the assert fails.
        message: Arc<DAE::Exp>,
        /// Error or warning
        level: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    TERMINATE_EQUATION {
        /// The message to display if the terminate triggers.
        message: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    REINIT_EQUATION {
        /// The variable to reinitialize.
        cref: Arc<DAE::ComponentRef>,
        /// The new value of the variable.
        reinitExp: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    NORETCALL_EQUATION {
        exp: Arc<DAE::Exp>,
        info: SourceInfo,
    },
}
impl metamodelica::gc::MMTrace for Equation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Equation::EQUALITY_EQUATION { lhs, rhs, info } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::CONNECT_EQUATION { lhs, lhsType, rhs, rhsType, prefix, info } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhsType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhsType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::FOR_EQUATION { name, index, indexType, range, body, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(indexType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::IF_EQUATION { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::WHEN_EQUATION { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::ASSERT_EQUATION { condition, message, level, info } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::TERMINATE_EQUATION { message, info } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::REINIT_EQUATION { cref, reinitExp, info } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::NORETCALL_EQUATION { exp, info } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Equation::{EQUALITY_EQUATION,CONNECT_EQUATION,FOR_EQUATION,IF_EQUATION,WHEN_EQUATION,ASSERT_EQUATION,TERMINATE_EQUATION,REINIT_EQUATION,NORETCALL_EQUATION};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Statement {
    ASSIGN_STMT {
        /// The asignee
        lhs: Arc<DAE::Exp>,
        /// The expression
        rhs: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    /// Used to mark in which order local array variables in functions should be initialized
    FUNCTION_ARRAY_INIT {
        name: ArcStr,
        ty: Arc<DAE::Type>,
        info: SourceInfo,
    },
    FOR_STMT {
        /// The name of the iterator variable.
        name: ArcStr,
        /// The index of the scope of the iterator variable.
        index: i32,
        /// The type of the index/iterator variable.
        indexType: Arc<DAE::Type>,
        /// The range expression to loop over.
        range: Option<Arc<DAE::Exp>>,
        /// The body of the for loop.
        body: Arc<metamodelica::List<Arc<Statement>>>,
        info: SourceInfo,
    },
    IF_STMT {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Statement>>>)>>,
        info: SourceInfo,
    },
    WHEN_STMT {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Statement>>>)>>,
        info: SourceInfo,
    },
    ASSERT_STMT {
        /// The assert condition.
        condition: Arc<DAE::Exp>,
        /// The message to display if the assert fails.
        message: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    TERMINATE_STMT {
        /// The message to display if the terminate triggers.
        message: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    REINIT_STMT {
        /// The variable to reinitialize.
        cref: Arc<DAE::ComponentRef>,
        /// The new value of the variable.
        reinitExp: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    NORETCALL_STMT {
        exp: Arc<DAE::Exp>,
        info: SourceInfo,
    },
    WHILE_STMT {
        exp: Arc<DAE::Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        info: SourceInfo,
    },
    RETURN_STMT {
        info: SourceInfo,
    },
    BREAK_STMT {
        info: SourceInfo,
    },
    FAILURE_STMT {
        body: Arc<metamodelica::List<Arc<Statement>>>,
        info: SourceInfo,
    },
}
impl metamodelica::gc::MMTrace for Statement {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Statement::ASSIGN_STMT { lhs, rhs, info } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::FUNCTION_ARRAY_INIT { name, ty, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::FOR_STMT { name, index, indexType, range, body, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(indexType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::IF_STMT { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::WHEN_STMT { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::ASSERT_STMT { condition, message, info } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::TERMINATE_STMT { message, info } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::REINIT_STMT { cref, reinitExp, info } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::NORETCALL_STMT { exp, info } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::WHILE_STMT { exp, statementLst, info } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::RETURN_STMT { info } => {
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::BREAK_STMT { info } => {
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::FAILURE_STMT { body, info } => {
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Statement::{ASSIGN_STMT,FUNCTION_ARRAY_INIT,FOR_STMT,IF_STMT,WHEN_STMT,ASSERT_STMT,TERMINATE_STMT,REINIT_STMT,NORETCALL_STMT,WHILE_STMT,RETURN_STMT,BREAK_STMT,FAILURE_STMT};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FunctionSlot {
    pub name: ArcStr,
    pub arg: Option<Arc<DAE::Exp>>,
    pub defaultValue: Option<Arc<DAE::Exp>>,
}

impl metamodelica::gc::MMTrace for FunctionSlot {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arg, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.defaultValue, __mmv)?;
        Ok(())
    }
}
pub type SLOT = FunctionSlot;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum EntryOrigin {
    /// An entry declared in the local scope.
    LOCAL_ORIGIN,
    /// An entry declared in the builtin scope.
    BUILTIN_ORIGIN,
    /// An entry that has been inherited through an extends clause.
    INHERITED_ORIGIN {
        /// The path of the baseclass the entry was inherited from.
        baseClass: Arc<Absyn::Path>,
        /// The info of the extends clause.
        info: SourceInfo,
        /// The origins of the element in the baseclass.
        origin: Arc<metamodelica::List<Arc<EntryOrigin>>>,
        /// The environment the entry was inherited from.
        originEnv: Env,
        /// Index used to identify the extends clause for optimization.
        index: i32,
    },
    /// An entry that has replaced another entry through redeclare.
    REDECLARED_ORIGIN {
        /// The replaced entry.
        replacedEntry: Arc<Entry>,
        /// The environment the replacement came from.
        originEnv: Env,
    },
    /// An entry that has been imported with an import statement.
    IMPORTED_ORIGIN {
        imp: Absyn::Import,
        info: SourceInfo,
        /// The environment the entry was imported from.
        originEnv: Env,
    },
}
impl metamodelica::gc::MMTrace for EntryOrigin {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EntryOrigin::LOCAL_ORIGIN => Ok(()),
            EntryOrigin::BUILTIN_ORIGIN => Ok(()),
            EntryOrigin::INHERITED_ORIGIN { baseClass, info, origin, originEnv, index } => {
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(origin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originEnv, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            EntryOrigin::REDECLARED_ORIGIN { replacedEntry, originEnv } => {
                metamodelica::gc::MMTrace::mm_accept(replacedEntry, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originEnv, __mmv)?;
                Ok(())
            }
            EntryOrigin::IMPORTED_ORIGIN { imp, info, originEnv } => {
                metamodelica::gc::MMTrace::mm_accept(imp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originEnv, __mmv)?;
                Ok(())
            }
        }
    }
}
impl EntryOrigin {
    pub fn interned_LOCAL_ORIGIN() -> Arc<EntryOrigin> {
        thread_local! {
            static INTERNED: Arc<EntryOrigin> = Arc::new(EntryOrigin::LOCAL_ORIGIN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_BUILTIN_ORIGIN() -> Arc<EntryOrigin> {
        thread_local! {
            static INTERNED: Arc<EntryOrigin> = Arc::new(EntryOrigin::BUILTIN_ORIGIN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_LOCAL_ORIGIN() -> Arc<EntryOrigin> { EntryOrigin::interned_LOCAL_ORIGIN() }
pub fn interned_BUILTIN_ORIGIN() -> Arc<EntryOrigin> { EntryOrigin::interned_BUILTIN_ORIGIN() }
pub use self::EntryOrigin::{LOCAL_ORIGIN,BUILTIN_ORIGIN,INHERITED_ORIGIN,REDECLARED_ORIGIN,IMPORTED_ORIGIN};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Entry {
    pub name: ArcStr,
    pub element: Arc<SCode::Element>,
    pub r#mod: Arc<Modifier>,
    pub origins: Arc<metamodelica::List<Arc<EntryOrigin>>>,
}

impl metamodelica::gc::MMTrace for Entry {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.element, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#mod, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.origins, __mmv)?;
        Ok(())
    }
}
pub type ENTRY = Entry;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ScopeType {
    BUILTIN_SCOPE,
    TOP_SCOPE,
    NORMAL_SCOPE {
        isEncapsulated: bool,
    },
    /// This scope contains one or more iterators; they are made unique by the following index (plus their name)
    IMPLICIT_SCOPE {
        iterIndex: i32,
    },
}
impl metamodelica::gc::MMTrace for ScopeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ScopeType::BUILTIN_SCOPE => Ok(()),
            ScopeType::TOP_SCOPE => Ok(()),
            ScopeType::NORMAL_SCOPE { isEncapsulated } => {
                metamodelica::gc::MMTrace::mm_accept(isEncapsulated, __mmv)?;
                Ok(())
            }
            ScopeType::IMPLICIT_SCOPE { iterIndex } => {
                metamodelica::gc::MMTrace::mm_accept(iterIndex, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::ScopeType::{BUILTIN_SCOPE,TOP_SCOPE,NORMAL_SCOPE,IMPLICIT_SCOPE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Frame {
    pub name: Option<ArcStr>,
    pub prefix: Option<Arc<NFInstPrefix::Prefix>>,
    pub scopeType: ScopeType,
    pub entries: Arc<AvlTree>,
}

impl metamodelica::gc::MMTrace for Frame {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.prefix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scopeType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.entries, __mmv)?;
        Ok(())
    }
}
pub type FRAME = Frame;


pub type Env = Arc<metamodelica::List<Arc<Frame>>>;

pub type AvlKey = ArcStr;

pub type AvlValue = Arc<Entry>;

/// The binary tree data structure
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct AvlTree {
    /// Value
    pub value: Option<Arc<AvlTreeValue>>,
    /// height of tree, used for balancing
    pub height: i32,
    /// left subtree
    pub left: Option<Arc<AvlTree>>,
    /// right subtree
    pub right: Option<Arc<AvlTree>>,
}

impl metamodelica::gc::MMTrace for AvlTree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.height, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.left, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.right, __mmv)?;
        Ok(())
    }
}
pub type AVLTREENODE = AvlTree;


/// Each node in the binary tree can have a value associated with it.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct AvlTreeValue {
    /// Key
    pub key: AvlKey,
    /// Value
    pub value: AvlValue,
}

impl metamodelica::gc::MMTrace for AvlTreeValue {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.key, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
pub type AVLTREEVALUE = AvlTreeValue;


