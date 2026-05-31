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
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;

/// these are the simulation/buildModel* options
/// simulation/buildModel* options
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimulationOptions {
    /// start time, default 0.0
    pub startTime: Arc<DAE::Exp>,
    /// stop time, default 1.0
    pub stopTime: Arc<DAE::Exp>,
    /// number of intervals, default 500
    pub numberOfIntervals: Arc<DAE::Exp>,
    /// stepSize, default (stopTime-startTime)/numberOfIntervals
    pub stepSize: Arc<DAE::Exp>,
    /// tolerance, default 1e-6
    pub tolerance: Arc<DAE::Exp>,
    /// method, default 'dassl'
    pub method: Arc<DAE::Exp>,
    /// file name prefix, default ''
    pub fileNamePrefix: Arc<DAE::Exp>,
    /// options, default ''
    pub options: Arc<DAE::Exp>,
    /// output format, default 'plt'
    pub outputFormat: Arc<DAE::Exp>,
    /// variable filter, regex does whole string matching, i.e. it becomes ^.*$ in the runtime
    pub variableFilter: Arc<DAE::Exp>,
    /// Compiler flags, in addition to MODELICAUSERCFLAGS
    pub cflags: Arc<DAE::Exp>,
    /// Flags sent to the simulation executable (doesn't do anything for buildModel)
    pub simflags: Arc<DAE::Exp>,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            startTime: Default::default(),
            stopTime: Default::default(),
            numberOfIntervals: Default::default(),
            stepSize: Default::default(),
            tolerance: Default::default(),
            method: Default::default(),
            fileNamePrefix: Default::default(),
            options: Default::default(),
            outputFormat: Default::default(),
            variableFilter: Default::default(),
            cflags: Default::default(),
            simflags: Default::default(),
        }
    }
}

pub type SIMULATION_OPTIONS = SimulationOptions;


/// - InteractiveTypes.Variable
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variable {
    /// The variable identifier
    pub varIdent: ArcStr,
    /// The value
    pub value: Arc<Values::Value>,
    /// The type of the expression
    pub type_: Arc<DAE::Type>,
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            varIdent: Default::default(),
            value: Default::default(),
            type_: Default::default(),
        }
    }
}

pub type IVAR = Variable;


/// - a component in a class
///  this is used in extracting all the components in all the classes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Component {
    COMPONENTITEM {
        /// the class where the component is
        the1: Arc<Absyn::Path>,
        /// the type of the component
        the2: Arc<Absyn::Path>,
        /// the name of the component
        the3: Arc<Absyn::ComponentRef>,
    },
    EXTENDSITEM {
        /// the class which is extended
        the1: Arc<Absyn::Path>,
        /// the class which is the extension
        the2: Arc<Absyn::Path>,
    },
}
impl Default for Component {
    fn default() -> Self {
        Self::EXTENDSITEM {
            the1: Default::default(),
            the2: Default::default(),
        }
    }
}
pub use self::Component::{COMPONENTITEM,EXTENDSITEM};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Components {
    pub componentLst: Arc<metamodelica::List<Component>>,
    /// the number of components in list. used to optimize the get_dependency_on_class
    pub the: i32,
}

impl Default for Components {
    fn default() -> Self {
        Self {
            componentLst: Default::default(),
            the: Default::default(),
        }
    }
}

pub type COMPONENTS = Components;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentReplacement {
    /// which class contain the old cref
    pub which1: Arc<Absyn::Path>,
    /// the old cref
    pub the2: Arc<Absyn::ComponentRef>,
    /// the new cref
    pub the3: Arc<Absyn::ComponentRef>,
}

impl Default for ComponentReplacement {
    fn default() -> Self {
        Self {
            which1: Default::default(),
            the2: Default::default(),
            the3: Default::default(),
        }
    }
}

pub type COMPONENTREPLACEMENT = ComponentReplacement;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentReplacementRules {
    pub componentReplacementLst: Arc<metamodelica::List<ComponentReplacement>>,
    /// the number of rules
    pub the: i32,
}

impl Default for ComponentReplacementRules {
    fn default() -> Self {
        Self {
            componentReplacementLst: Default::default(),
            the: Default::default(),
        }
    }
}

pub type COMPONENTREPLACEMENTRULES = ComponentReplacementRules;


