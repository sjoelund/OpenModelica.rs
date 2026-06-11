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

use crate::Global;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DebugFlag {
    /// Unique index.
    pub index: i32,
    /// The name of the flag used by -d
    pub name: ArcStr,
    /// Default enabled or not
    pub default: bool,
    /// A description of the flag.
    pub description: ArcStr,
}

impl metamodelica::gc::MMTrace for DebugFlag {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.default, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.description, __mmv)?;
        Ok(())
    }
}
pub type DEBUG_FLAG = DebugFlag;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ConfigFlag {
    /// Unique index.
    pub index: i32,
    /// The whole name of the flag.
    pub name: ArcStr,
    /// A short name one-character name for the flag.
    pub shortname: Option<ArcStr>,
    /// Whether the flag is visible to the user or not.
    pub visibility: FlagVisibility,
    /// The default value of the flag.
    pub defaultValue: FlagData,
    /// The valid options for the flag.
    pub validOptions: Option<ValidOptions>,
    /// A description of the flag.
    pub description: ArcStr,
}

impl metamodelica::gc::MMTrace for ConfigFlag {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.shortname, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visibility, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.defaultValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.validOptions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.description, __mmv)?;
        Ok(())
    }
}
impl Default for ConfigFlag {
    fn default() -> Self {
        Self {
            index: Default::default(),
            name: Default::default(),
            shortname: Default::default(),
            visibility: Default::default(),
            defaultValue: Default::default(),
            validOptions: Default::default(),
            description: Default::default(),
        }
    }
}

pub type CONFIG_FLAG = ConfigFlag;


/// This uniontype is used to store the values of configuration flags.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum FlagData {
    /// Only used to initialize the flag array.
    EMPTY_FLAG,
    BOOL_FLAG {
        /// Value of a boolean flag.
        data: bool,
    },
    INT_FLAG {
        /// Value of an integer flag.
        data: i32,
    },
    INT_LIST_FLAG {
        /// Value of an integer flag that can have multiple values.
        data: Arc<metamodelica::List<i32>>,
    },
    REAL_FLAG {
        /// Value of a real flag.
        data: metamodelica::Real,
    },
    STRING_FLAG {
        /// Value of a string flag.
        data: ArcStr,
    },
    STRING_LIST_FLAG {
        /// Values of a string flag that can have multiple values.
        data: Arc<metamodelica::List<ArcStr>>,
    },
    ENUM_FLAG {
        /// Value of an enumeration flag.
        data: i32,
        /// The valid values of the enum.
        validValues: Arc<metamodelica::List<(ArcStr, i32)>>,
    },
}
impl metamodelica::gc::MMTrace for FlagData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FlagData::EMPTY_FLAG => Ok(()),
            FlagData::BOOL_FLAG { data } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                Ok(())
            }
            FlagData::INT_FLAG { data } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                Ok(())
            }
            FlagData::INT_LIST_FLAG { data } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                Ok(())
            }
            FlagData::REAL_FLAG { data } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                Ok(())
            }
            FlagData::STRING_FLAG { data } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                Ok(())
            }
            FlagData::STRING_LIST_FLAG { data } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                Ok(())
            }
            FlagData::ENUM_FLAG { data, validValues } => {
                metamodelica::gc::MMTrace::mm_accept(data, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(validValues, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for FlagData {
    fn default() -> Self { Self::EMPTY_FLAG }
}
pub use self::FlagData::{EMPTY_FLAG,BOOL_FLAG,INT_FLAG,INT_LIST_FLAG,REAL_FLAG,STRING_FLAG,STRING_LIST_FLAG,ENUM_FLAG};

/// This uniontype is used to specify the visibility of a configuration flag.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum FlagVisibility {
    /// An internal flag that is hidden to the user.
    INTERNAL,
    /// An external flag that is visible to the user.
    EXTERNAL,
}
impl metamodelica::gc::MMTrace for FlagVisibility {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FlagVisibility::INTERNAL => Ok(()),
            FlagVisibility::EXTERNAL => Ok(()),
        }
    }
}
impl Default for FlagVisibility {
    fn default() -> Self { Self::INTERNAL }
}
pub(crate) use self::FlagVisibility::{INTERNAL,EXTERNAL};

/// The structure which stores the flags.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Flag {
    FLAGS {
        debugFlags: metamodelica::Array<bool>,
        configFlags: metamodelica::Array<FlagData>,
    },
    NO_FLAGS,
}
impl metamodelica::gc::MMTrace for Flag {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Flag::FLAGS { debugFlags, configFlags } => {
                metamodelica::gc::MMTrace::mm_accept(debugFlags, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(configFlags, __mmv)?;
                Ok(())
            }
            Flag::NO_FLAGS => Ok(()),
        }
    }
}
impl Default for Flag {
    fn default() -> Self { Self::NO_FLAGS }
}
pub use self::Flag::{FLAGS,NO_FLAGS};

/// Specifies valid options for a flag.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum ValidOptions {
    STRING_OPTION {
        /// Options for a string flag.
        options: Arc<metamodelica::List<ArcStr>>,
    },
    STRING_DESC_OPTION {
        /// Options for a string flag, with a description for each option.
        options: Arc<metamodelica::List<(ArcStr, ArcStr)>>,
    },
}
impl metamodelica::gc::MMTrace for ValidOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ValidOptions::STRING_OPTION { options } => {
                metamodelica::gc::MMTrace::mm_accept(options, __mmv)?;
                Ok(())
            }
            ValidOptions::STRING_DESC_OPTION { options } => {
                metamodelica::gc::MMTrace::mm_accept(options, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for ValidOptions {
    fn default() -> Self {
        Self::STRING_OPTION {
            options: Default::default(),
        }
    }
}
pub(crate) use self::ValidOptions::{STRING_OPTION,STRING_DESC_OPTION};

// Change this to a proper enum when we have support for them.
pub const MODELICA: i32 = 1;

pub const METAMODELICA: i32 = 2;

pub const PARMODELICA: i32 = 3;

pub const OPTIMICA: i32 = 4;

pub const PDEMODELICA: i32 = 5;

// FMI-ModelDescription-ENUM-FLAGS
pub const FMI_NONE: i32 = 1;

pub const FMI_INTERNAL: i32 = 2;

pub const FMI_PROTECTED: i32 = 3;

pub const FMI_BLACKBOX: i32 = 4;

pub(crate) const collapseArrayExpressionsText: &'static str = "Simplifies {x[1],x[2],x[3]} → x for arrays of whole variable references (simplifies code generation).";

// DEBUG FLAGS
pub static FAILTRACE: DebugFlag = DebugFlag { index: 1, name: literal!("failtrace"), default: false, description: literal!("Sets whether to print a failtrace or not.") };

pub static CEVAL: DebugFlag = DebugFlag { index: 2, name: literal!("ceval"), default: false, description: literal!("Prints extra information from Ceval.") };

pub static CHECK_BACKEND_DAE: DebugFlag = DebugFlag { index: 3, name: literal!("checkBackendDae"), default: false, description: literal!("Do some simple analyses on the datastructure from the frontend to check if it is consistent.") };

pub static PTHREADS: DebugFlag = DebugFlag { index: 4, name: literal!("pthreads"), default: false, description: literal!("Experimental: Unused parallelization.") };

pub static EVENTS: DebugFlag = DebugFlag { index: 5, name: literal!("events"), default: true, description: literal!("Turns on/off events handling.") };

pub static DUMP_INLINE_SOLVER: DebugFlag = DebugFlag { index: 6, name: literal!("dumpInlineSolver"), default: false, description: literal!("Dumps the inline solver equation system.") };

pub static EVAL_FUNC: DebugFlag = DebugFlag { index: 7, name: literal!("evalfunc"), default: true, description: literal!("Turns on/off symbolic function evaluation.") };

pub static GEN: DebugFlag = DebugFlag { index: 8, name: literal!("gen"), default: false, description: literal!("Turns on/off dynamic loading of functions that are compiled during translation. Only enable this if external functions are needed to calculate structural parameters or constants.") };

pub static DYN_LOAD: DebugFlag = DebugFlag { index: 9, name: literal!("dynload"), default: false, description: literal!("Display debug information about dynamic loading of compiled functions.") };

pub static GENERATE_CODE_CHEAT: DebugFlag = DebugFlag { index: 10, name: literal!("generateCodeCheat"), default: false, description: literal!("Used to generate code for the bootstrapped compiler.") };

pub static CGRAPH_GRAPHVIZ_FILE: DebugFlag = DebugFlag { index: 11, name: literal!("cgraphGraphVizFile"), default: false, description: literal!("Generates a graphviz file of the connection graph.") };

pub static CGRAPH_GRAPHVIZ_SHOW: DebugFlag = DebugFlag { index: 12, name: literal!("cgraphGraphVizShow"), default: false, description: literal!("Displays the connection graph with the GraphViz lefty tool.") };

pub static GC_PROF: DebugFlag = DebugFlag { index: 13, name: literal!("gcProfiling"), default: false, description: literal!("Prints garbage collection stats to standard output.") };

pub static CHECK_DAE_CREF_TYPE: DebugFlag = DebugFlag { index: 14, name: literal!("checkDAECrefType"), default: false, description: literal!("Enables extra type checking for cref expressions.") };

pub static CHECK_ASUB: DebugFlag = DebugFlag { index: 15, name: literal!("checkASUB"), default: false, description: literal!("Prints out a warning if an ASUB is created from a CREF expression.") };

pub static INSTANCE: DebugFlag = DebugFlag { index: 16, name: literal!("instance"), default: false, description: literal!("Prints extra failtrace from InstanceHierarchy.") };

pub static CACHE: DebugFlag = DebugFlag { index: 17, name: literal!("Cache"), default: true, description: literal!("Turns off the instantiation cache.") };

pub static RML: DebugFlag = DebugFlag { index: 18, name: literal!("rml"), default: false, description: literal!("Converts Modelica-style arrays to lists.") };

pub static TAIL: DebugFlag = DebugFlag { index: 19, name: literal!("tail"), default: false, description: literal!("Prints out a notification if tail recursion optimization has been applied.") };

pub static LOOKUP: DebugFlag = DebugFlag { index: 20, name: literal!("lookup"), default: false, description: literal!("Print extra failtrace from lookup.") };

pub static PATTERNM_SKIP_FILTER_UNUSED_AS_BINDINGS: DebugFlag = DebugFlag { index: 21, name: literal!("patternmSkipFilterUnusedBindings"), default: false, description: literal!("") };

pub static PATTERNM_ALL_INFO: DebugFlag = DebugFlag { index: 22, name: literal!("patternmAllInfo"), default: false, description: literal!("Adds notifications of all pattern-matching optimizations that are performed.") };

pub static PATTERNM_DCE: DebugFlag = DebugFlag { index: 23, name: literal!("patternmDeadCodeElimination"), default: true, description: literal!("Performs dead code elimination in match-expressions.") };

pub static PATTERNM_MOVE_LAST_EXP: DebugFlag = DebugFlag { index: 24, name: literal!("patternmMoveLastExp"), default: true, description: literal!("Optimization that moves the last assignment(s) into the result of a match-expression. For example: equation c = fn(b); then c; => then fn(b);") };

pub static EXPERIMENTAL_REDUCTIONS: DebugFlag = DebugFlag { index: 25, name: literal!("experimentalReductions"), default: false, description: literal!("Turns on custom reduction functions (OpenModelica extension).") };

pub static EVAL_PARAM: DebugFlag = DebugFlag { index: 26, name: literal!("evaluateAllParameters"), default: false, description: literal!("Evaluates all parameters if set, except the ones that have annotation(Evaluate = false).") };

pub static TYPES: DebugFlag = DebugFlag { index: 27, name: literal!("types"), default: false, description: literal!("Prints extra failtrace from Types.") };

pub static SHOW_STATEMENT: DebugFlag = DebugFlag { index: 28, name: literal!("showStatement"), default: false, description: literal!("Shows the statement that is currently being evaluated when evaluating a script.") };

pub static DUMP: DebugFlag = DebugFlag { index: 29, name: literal!("dump"), default: false, description: literal!("Dumps the absyn representation of a program.") };

pub static DUMP_GRAPHVIZ: DebugFlag = DebugFlag { index: 30, name: literal!("graphviz"), default: false, description: literal!("Dumps the absyn representation of a program in graphviz format.") };

pub static EXEC_STAT: DebugFlag = DebugFlag { index: 31, name: literal!("execstat"), default: false, description: literal!("Prints out execution statistics for the compiler.") };

pub(crate) static TRANSFORMS_BEFORE_DUMP: DebugFlag = DebugFlag { index: 32, name: literal!("transformsbeforedump"), default: false, description: literal!("Applies transformations required for code generation before dumping flat code.") };

pub(crate) static DAE_DUMP_GRAPHV: DebugFlag = DebugFlag { index: 33, name: literal!("daedumpgraphv"), default: false, description: literal!("Dumps the DAE in graphviz format.") };

pub(crate) static INTERACTIVE_TCP: DebugFlag = DebugFlag { index: 34, name: literal!("interactive"), default: false, description: literal!("Starts omc as a server listening on the socket interface.") };

pub(crate) static INTERACTIVE_CORBA: DebugFlag = DebugFlag { index: 35, name: literal!("interactiveCorba"), default: false, description: literal!("Starts omc as a server listening on the Corba interface.") };

pub static INTERACTIVE_DUMP: DebugFlag = DebugFlag { index: 36, name: literal!("interactivedump"), default: false, description: literal!("Prints out debug information for the interactive server.") };

pub static RELIDX: DebugFlag = DebugFlag { index: 37, name: literal!("relidx"), default: false, description: literal!("Prints out debug information about relations, that are used as zero crossings.") };

pub static DUMP_REPL: DebugFlag = DebugFlag { index: 38, name: literal!("dumprepl"), default: false, description: literal!("Dump the found replacements for simple equation removal.") };

pub(crate) static DUMP_FP_REPL: DebugFlag = DebugFlag { index: 39, name: literal!("dumpFPrepl"), default: false, description: literal!("Dump the found replacements for final parameters.") };

pub static DUMP_PARAM_REPL: DebugFlag = DebugFlag { index: 40, name: literal!("dumpParamrepl"), default: false, description: literal!("Dump the found replacements for remove parameters.") };

pub static DUMP_PP_REPL: DebugFlag = DebugFlag { index: 41, name: literal!("dumpPPrepl"), default: false, description: literal!("Dump the found replacements for protected parameters.") };

pub(crate) static DUMP_EA_REPL: DebugFlag = DebugFlag { index: 42, name: literal!("dumpEArepl"), default: false, description: literal!("Dump the found replacements for evaluate annotations (evaluate=true) parameters.") };

pub static DEBUG_ALIAS: DebugFlag = DebugFlag { index: 43, name: literal!("debugAlias"), default: false, description: literal!("Dumps some information about the process of removeSimpleEquations.") };

pub static TEARING_DUMP: DebugFlag = DebugFlag { index: 44, name: literal!("tearingdump"), default: false, description: literal!("Dumps tearing information.") };

pub static JAC_DUMP: DebugFlag = DebugFlag { index: 45, name: literal!("symjacdump"), default: false, description: literal!("Dumps information about symbolic Jacobians.") };

pub static JAC_DUMP2: DebugFlag = DebugFlag { index: 46, name: literal!("symjacdumpverbose"), default: false, description: literal!("Dumps information in verbose mode about symbolic Jacobians.") };

pub static DUMP_BINDINGS: DebugFlag = DebugFlag { index: 47, name: literal!("dumpBindings"), default: false, description: literal!("Dumps information about the equations created from bindings.") };

pub static DUMP_SORTING: DebugFlag = DebugFlag { index: 48, name: literal!("dumpSorting"), default: false, description: literal!("Dumps information about the process of sorting.") };

pub static DUMP_SPARSE: DebugFlag = DebugFlag { index: 49, name: literal!("dumpSparsePattern"), default: false, description: literal!("Dumps sparse pattern with coloring used for simulation.") };

pub static DUMP_SPARSE_VERBOSE: DebugFlag = DebugFlag { index: 50, name: literal!("dumpSparsePatternVerbose"), default: false, description: literal!("Dumps in verbose mode sparse pattern with coloring used for simulation.") };

pub static BLT_DUMP: DebugFlag = DebugFlag { index: 51, name: literal!("bltdump"), default: false, description: literal!("Dumps information from index reduction.") };

pub static DUMMY_SELECT: DebugFlag = DebugFlag { index: 52, name: literal!("dummyselect"), default: false, description: literal!("Dumps information from dummy state selection heuristic.") };

pub static DUMP_DAE_LOW: DebugFlag = DebugFlag { index: 53, name: literal!("dumpdaelow"), default: false, description: literal!("Dumps the equation system at the beginning of the back end.") };

pub static DUMP_INDX_DAE: DebugFlag = DebugFlag { index: 54, name: literal!("dumpindxdae"), default: false, description: literal!("Dumps the equation system after index reduction and optimization.") };

pub static OPT_DAE_DUMP: DebugFlag = DebugFlag { index: 55, name: literal!("optdaedump"), default: false, description: literal!("Dumps information from the optimization modules.") };

pub static EXEC_HASH: DebugFlag = DebugFlag { index: 56, name: literal!("execHash"), default: false, description: literal!("Measures the time it takes to hash all simcode variables before code generation.") };

pub static PARAM_DLOW_DUMP: DebugFlag = DebugFlag { index: 57, name: literal!("paramdlowdump"), default: false, description: literal!("Enables dumping of the parameters in the order they are calculated.") };

pub static DUMP_ENCAPSULATECONDITIONS: DebugFlag = DebugFlag { index: 58, name: literal!("dumpEncapsulateConditions"), default: false, description: literal!("Dumps the results of the preOptModule encapsulateWhenConditions.") };

pub static SHORT_OUTPUT: DebugFlag = DebugFlag { index: 59, name: literal!("shortOutput"), default: false, description: literal!("Enables short output of the simulate() command. Useful for tools like OMNotebook.") };

pub static COUNT_OPERATIONS: DebugFlag = DebugFlag { index: 60, name: literal!("countOperations"), default: false, description: literal!("Count operations.") };

pub static CGRAPH: DebugFlag = DebugFlag { index: 61, name: literal!("cgraph"), default: false, description: literal!("Prints out connection graph information.") };

pub static UPDMOD: DebugFlag = DebugFlag { index: 62, name: literal!("updmod"), default: false, description: literal!("Prints information about modification updates.") };

pub static STATIC: DebugFlag = DebugFlag { index: 63, name: literal!("static"), default: false, description: literal!("Enables extra debug output from the static elaboration.") };

pub static TPL_PERF_TIMES: DebugFlag = DebugFlag { index: 64, name: literal!("tplPerfTimes"), default: false, description: literal!("Enables output of template performance data for rendering text to file.") };

pub static CHECK_SIMPLIFY: DebugFlag = DebugFlag { index: 65, name: literal!("checkSimplify"), default: false, description: literal!("Enables checks for expression simplification and prints a notification whenever an undesirable transformation has been performed.") };

pub static SCODE_INST: DebugFlag = DebugFlag { index: 66, name: literal!("newInst"), default: true, description: literal!("Enables new instantiation phase.") };

pub static WRITE_TO_BUFFER: DebugFlag = DebugFlag { index: 67, name: literal!("writeToBuffer"), default: false, description: literal!("Enables writing simulation results to buffer.") };

pub static DUMP_BACKENDDAE_INFO: DebugFlag = DebugFlag { index: 68, name: literal!("backenddaeinfo"), default: false, description: literal!("Enables dumping of back-end information about system (Number of equations before back-end,...).") };

pub static GEN_DEBUG_SYMBOLS: DebugFlag = DebugFlag { index: 69, name: literal!("gendebugsymbols"), default: false, description: literal!("Generate code with debugging symbols.") };

pub static DUMP_STATESELECTION_INFO: DebugFlag = DebugFlag { index: 70, name: literal!("stateselection"), default: false, description: literal!("Enables dumping of selected states. Extends -d=backenddaeinfo.") };

pub static DUMP_EQNINORDER: DebugFlag = DebugFlag { index: 71, name: literal!("dumpeqninorder"), default: false, description: literal!("Enables dumping of the equations in the order they are calculated.") };

pub static SEMILINEAR: DebugFlag = DebugFlag { index: 72, name: literal!("semiLinear"), default: false, description: literal!("Enables dumping of the optimization information when optimizing calls to semiLinear.") };

pub(crate) static UNCERTAINTIES: DebugFlag = DebugFlag { index: 73, name: literal!("uncertainties"), default: false, description: literal!("Enables dumping of status when calling modelEquationsUC.") };

pub static SHOW_START_ORIGIN: DebugFlag = DebugFlag { index: 74, name: literal!("showStartOrigin"), default: false, description: literal!("Enables dumping of the DAE startOrigin attribute of the variables.") };

pub static DUMP_SIMCODE: DebugFlag = DebugFlag { index: 75, name: literal!("dumpSimCode"), default: false, description: literal!("Dumps the simCode model used for code generation.") };

pub static DUMP_INITIAL_SYSTEM: DebugFlag = DebugFlag { index: 76, name: literal!("dumpinitialsystem"), default: false, description: literal!("Dumps the initial equation system.") };

pub static GRAPH_INST: DebugFlag = DebugFlag { index: 77, name: literal!("graphInst"), default: false, description: literal!("Do graph based instantiation.") };

pub static GRAPH_INST_RUN_DEP: DebugFlag = DebugFlag { index: 78, name: literal!("graphInstRunDep"), default: false, description: literal!("Run scode dependency analysis. Use with -d=graphInst") };

pub static GRAPH_INST_GEN_GRAPH: DebugFlag = DebugFlag { index: 79, name: literal!("graphInstGenGraph"), default: false, description: literal!("Dumps a graph of the program. Use with -d=graphInst") };

pub static DUMP_CONST_REPL: DebugFlag = DebugFlag { index: 80, name: literal!("dumpConstrepl"), default: false, description: literal!("Dump the found replacements for constants.") };

pub static SHOW_EQUATION_SOURCE: DebugFlag = DebugFlag { index: 81, name: literal!("showEquationSource"), default: false, description: literal!("Display the element source information in the dumped DAE for easier debugging.") };

pub static LS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag { index: 82, name: literal!("LSanalyticJacobian"), default: false, description: literal!("Enables analytical jacobian for linear strong components. Defaults to false") };

pub static NLS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag { index: 83, name: literal!("NLSanalyticJacobian"), default: true, description: literal!("Enables analytical jacobian for non-linear strong components without user-defined function calls, for that see forceNLSanalyticJacobian") };

pub(crate) static INLINE_SOLVER: DebugFlag = DebugFlag { index: 84, name: literal!("inlineSolver"), default: false, description: literal!("Generates code for inline solver.") };

pub static HPCOM: DebugFlag = DebugFlag { index: 85, name: literal!("hpcom"), default: false, description: literal!("Enables parallel calculation based on task-graphs.") };

pub static INITIALIZATION: DebugFlag = DebugFlag { index: 86, name: literal!("initialization"), default: false, description: literal!("Shows additional information from the initialization process.") };

pub static INLINE_FUNCTIONS: DebugFlag = DebugFlag { index: 87, name: literal!("inlineFunctions"), default: true, description: literal!("Controls if function inlining should be performed.") };

pub static DUMP_SCC_GRAPHML: DebugFlag = DebugFlag { index: 88, name: literal!("dumpSCCGraphML"), default: false, description: literal!("Dumps graphml files with the strongly connected components.") };

pub static TEARING_DUMPVERBOSE: DebugFlag = DebugFlag { index: 89, name: literal!("tearingdumpV"), default: false, description: literal!("Dumps verbose tearing information.") };

pub static DISABLE_SINGLE_FLOW_EQ: DebugFlag = DebugFlag { index: 90, name: literal!("disableSingleFlowEq"), default: false, description: literal!("Disables the generation of single flow equations.") };

pub static DUMP_DISCRETEVARS_INFO: DebugFlag = DebugFlag { index: 91, name: literal!("discreteinfo"), default: false, description: literal!("Enables dumping of discrete variables. Extends -d=backenddaeinfo.") };

pub static ADDITIONAL_GRAPHVIZ_DUMP: DebugFlag = DebugFlag { index: 92, name: literal!("graphvizDump"), default: false, description: literal!("Activates additional graphviz dumps (as .dot files). It can be used in addition to one of the following flags: {dumpdaelow|dumpinitialsystems|dumpindxdae}.") };

pub static INFO_XML_OPERATIONS: DebugFlag = DebugFlag { index: 93, name: literal!("infoXmlOperations"), default: false, description: literal!("Enables output of the operations in the _info.xml file when translating models.") };

pub static HPCOM_DUMP: DebugFlag = DebugFlag { index: 94, name: literal!("hpcomDump"), default: false, description: literal!("Dumps additional information on the parallel execution with hpcom.") };

pub static RESOLVE_LOOPS_DUMP: DebugFlag = DebugFlag { index: 95, name: literal!("resolveLoopsDump"), default: false, description: literal!("Debug Output for ResolveLoops Module.") };

pub static DISABLE_WINDOWS_PATH_CHECK_WARNING: DebugFlag = DebugFlag { index: 96, name: literal!("disableWindowsPathCheckWarning"), default: false, description: literal!("Disables warnings on Windows if OPENMODELICAHOME/MinGW is missing.") };

pub static DISABLE_RECORD_CONSTRUCTOR_OUTPUT: DebugFlag = DebugFlag { index: 97, name: literal!("disableRecordConstructorOutput"), default: false, description: literal!("Disables output of record constructors in the flat code.") };

pub(crate) static IMPL_ODE: DebugFlag = DebugFlag { index: 98, name: literal!("implOde"), default: false, description: literal!("activates implicit codegen") };

pub static EVAL_FUNC_DUMP: DebugFlag = DebugFlag { index: 99, name: literal!("evalFuncDump"), default: false, description: literal!("dumps debug information about the function evaluation") };

pub static PRINT_STRUCTURAL: DebugFlag = DebugFlag { index: 100, name: literal!("printStructuralParameters"), default: false, description: literal!("Prints the structural parameters identified by the front-end") };

pub static ITERATION_VARS: DebugFlag = DebugFlag { index: 101, name: literal!("iterationVars"), default: false, description: literal!("Shows a list of all iteration variables.") };

pub static ALLOW_RECORD_TOO_MANY_FIELDS: DebugFlag = DebugFlag { index: 102, name: literal!("acceptTooManyFields"), default: false, description: literal!("Accepts passing records with more fields than expected to a function. This is not allowed, but is used in Fluid.Dissipation. See https://trac.modelica.org/Modelica/ticket/1245 for details.") };

pub static HPCOM_MEMORY_OPT: DebugFlag = DebugFlag { index: 103, name: literal!("hpcomMemoryOpt"), default: false, description: literal!("Optimize the memory structure regarding the selected scheduler") };

pub static DUMP_SYNCHRONOUS: DebugFlag = DebugFlag { index: 104, name: literal!("dumpSynchronous"), default: false, description: literal!("Dumps information of the clock partitioning.") };

pub static STRIP_PREFIX: DebugFlag = DebugFlag { index: 105, name: literal!("stripPrefix"), default: true, description: literal!("Strips the environment prefix from path/crefs. Defaults to true.") };

pub static DO_SCODE_DEP: DebugFlag = DebugFlag { index: 106, name: literal!("scodeDep"), default: true, description: literal!("Does scode dependency analysis prior to instantiation. Defaults to true.") };

pub static SHOW_INST_CACHE_INFO: DebugFlag = DebugFlag { index: 107, name: literal!("showInstCacheInfo"), default: false, description: literal!("Prints information about instantiation cache hits and additions. Defaults to false.") };

pub static DUMP_UNIT: DebugFlag = DebugFlag { index: 108, name: literal!("dumpUnits"), default: false, description: literal!("Dumps all the calculated units.") };

pub(crate) static DUMP_EQ_UNIT: DebugFlag = DebugFlag { index: 109, name: literal!("dumpEqInUC"), default: false, description: literal!("Dumps all equations handled by the unit checker.") };

pub static DUMP_EQ_UNIT_STRUCT: DebugFlag = DebugFlag { index: 110, name: literal!("dumpEqUCStruct"), default: false, description: literal!("Dumps all the equations handled by the unit checker as tree-structure.") };

pub static SHOW_DAE_GENERATION: DebugFlag = DebugFlag { index: 111, name: literal!("showDaeGeneration"), default: false, description: literal!("Show the dae variable declarations as they happen.") };

pub static RESHUFFLE_POST: DebugFlag = DebugFlag { index: 112, name: literal!("reshufflePost"), default: false, description: literal!("Reshuffles the systems of equations.") };

pub(crate) static SHOW_EXPANDABLE_INFO: DebugFlag = DebugFlag { index: 113, name: literal!("showExpandableInfo"), default: false, description: literal!("Show information about expandable connector handling.") };

pub(crate) static DUMP_HOMOTOPY: DebugFlag = DebugFlag { index: 114, name: literal!("dumpHomotopy"), default: false, description: literal!("Dumps the results of the postOptModule optimizeHomotopyCalls.") };

pub static OMC_RELOCATABLE_FUNCTIONS: DebugFlag = DebugFlag { index: 115, name: literal!("relocatableFunctions"), default: false, description: literal!("Generates relocatable code: all functions become function pointers and can be replaced at run-time.") };

pub static GRAPHML: DebugFlag = DebugFlag { index: 116, name: literal!("graphml"), default: false, description: literal!("Dumps .graphml files for the bipartite graph after Index Reduction and a task graph for the SCCs. Can be displayed with yEd. ") };

pub static USEMPI: DebugFlag = DebugFlag { index: 117, name: literal!("useMPI"), default: false, description: literal!("Add MPI init and finalize to main method (CPPruntime). ") };

pub static DUMP_CSE: DebugFlag = DebugFlag { index: 118, name: literal!("dumpCSE"), default: false, description: literal!("Additional output for CSE module.") };

pub static DUMP_CSE_VERBOSE: DebugFlag = DebugFlag { index: 119, name: literal!("dumpCSE_verbose"), default: false, description: literal!("Additional output for CSE module.") };

pub static NO_START_CALC: DebugFlag = DebugFlag { index: 120, name: literal!("disableStartCalc"), default: false, description: literal!("Deactivates the pre-calculation of start values during compile-time.") };

pub static CONSTJAC: DebugFlag = DebugFlag { index: 121, name: literal!("constjac"), default: false, description: literal!("solves linear systems with constant Jacobian and variable b-Vector symbolically") };

pub static VISUAL_XML: DebugFlag = DebugFlag { index: 122, name: literal!("visxml"), default: false, description: literal!("Outputs a xml-file that contains information for visualization.") };

pub static VECTORIZE: DebugFlag = DebugFlag { index: 123, name: literal!("vectorize"), default: false, description: literal!("Activates vectorization in the backend.") };

pub static CHECK_EXT_LIBS: DebugFlag = DebugFlag { index: 124, name: literal!("buildExternalLibs"), default: true, description: literal!("Use the autotools project in the Resources folder of the library to build missing external libraries.") };

pub static RUNTIME_STATIC_LINKING: DebugFlag = DebugFlag { index: 125, name: literal!("runtimeStaticLinking"), default: false, description: literal!("Use the static simulation runtime libraries (C++ simulation runtime).") };

pub static SORT_EQNS_AND_VARS: DebugFlag = DebugFlag { index: 126, name: literal!("dumpSortEqnsAndVars"), default: false, description: literal!("Dumps debug output for the modules sortEqnsVars.") };

pub static DUMP_SIMPLIFY_LOOPS: DebugFlag = DebugFlag { index: 127, name: literal!("dumpSimplifyLoops"), default: false, description: literal!("Dump between steps of simplifyLoops") };

pub static DUMP_RTEARING: DebugFlag = DebugFlag { index: 128, name: literal!("dumpRecursiveTearing"), default: false, description: literal!("Dump between steps of recursiveTearing") };

pub static DIS_SYMJAC_FMI20: DebugFlag = DebugFlag { index: 129, name: literal!("disableDirectionalDerivatives"), default: true, description: literal!("For FMI 2.0 only dependecy analysis will be perform.") };

pub static EVAL_OUTPUT_ONLY: DebugFlag = DebugFlag { index: 130, name: literal!("evalOutputOnly"), default: false, description: literal!("Generates equations to calculate top level outputs only.") };

pub static HARDCODED_START_VALUES: DebugFlag = DebugFlag { index: 131, name: literal!("hardcodedStartValues"), default: false, description: literal!("Embed the start values of variables and parameters into the c++ code and do not read it from xml file.") };

pub static DUMP_FUNCTIONS: DebugFlag = DebugFlag { index: 132, name: literal!("dumpFunctions"), default: false, description: literal!("Add functions to backend dumps.") };

pub static DEBUG_DIFFERENTIATION: DebugFlag = DebugFlag { index: 133, name: literal!("debugDifferentiation"), default: false, description: literal!("Dumps debug output for the differentiation process.") };

pub static DEBUG_DIFFERENTIATION_VERBOSE: DebugFlag = DebugFlag { index: 134, name: literal!("debugDifferentiationVerbose"), default: false, description: literal!("Dumps verbose debug output for the differentiation process.") };

pub static FMU_EXPERIMENTAL: DebugFlag = DebugFlag { index: 135, name: literal!("fmuExperimental"), default: false, description: literal!("Adds features to the FMI export that are considered experimental as of now: fmi2GetSpecificDerivatives, canGetSetFMUState, canSerializeFMUstate") };

pub static DUMP_DGESV: DebugFlag = DebugFlag { index: 136, name: literal!("dumpdgesv"), default: false, description: literal!("Enables dumping of the information whether DGESV is used to solve linear systems.") };

pub static MULTIRATE_PARTITION: DebugFlag = DebugFlag { index: 137, name: literal!("multirate"), default: false, description: literal!("The solver can switch partitions in the system.") };

pub static DUMP_EXCLUDED_EXP: DebugFlag = DebugFlag { index: 138, name: literal!("dumpExcludedSymJacExps"), default: false, description: literal!("This flags dumps all expression that are excluded from differentiation of a symbolic Jacobian.") };

pub static DEBUG_ALGLOOP_JACOBIAN: DebugFlag = DebugFlag { index: 139, name: literal!("debugAlgebraicLoopsJacobian"), default: false, description: literal!("Dumps debug output while creating symbolic jacobians for non-linear systems.") };

pub static DISABLE_JACSCC: DebugFlag = DebugFlag { index: 140, name: literal!("disableJacsforSCC"), default: false, description: literal!("Disables calculation of jacobians to detect if a SCC is linear or non-linear. By disabling all SCC will handled like non-linear.") };

pub static FORCE_NLS_ANALYTIC_JACOBIAN: DebugFlag = DebugFlag { index: 141, name: literal!("forceNLSanalyticJacobian"), default: false, description: literal!("Forces calculation analytical jacobian also for non-linear strong components with user-defined functions.") };

pub static DUMP_LOOPS: DebugFlag = DebugFlag { index: 142, name: literal!("dumpLoops"), default: false, description: literal!("Dumps loop equation.") };

pub static DUMP_LOOPS_VERBOSE: DebugFlag = DebugFlag { index: 143, name: literal!("dumpLoopsVerbose"), default: false, description: literal!("Dumps loop equation and enhanced adjacency matrix.") };

pub static SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR: DebugFlag = DebugFlag { index: 144, name: literal!("skipInputOutputSyntacticSugar"), default: false, description: literal!("Used when bootstrapping to preserve the input output parsing of the code output by the list command.") };

pub static OMC_RECORD_ALLOC_WORDS: DebugFlag = DebugFlag { index: 145, name: literal!("metaModelicaRecordAllocWords"), default: false, description: literal!("Instrument the source code to record memory allocations (requires run-time and generated files compiled with -DOMC_RECORD_ALLOC_WORDS).") };

pub static TOTAL_TEARING_DUMP: DebugFlag = DebugFlag { index: 146, name: literal!("totaltearingdump"), default: false, description: literal!("Dumps total tearing information.") };

pub static TOTAL_TEARING_DUMPVERBOSE: DebugFlag = DebugFlag { index: 147, name: literal!("totaltearingdumpV"), default: false, description: literal!("Dumps verbose total tearing information.") };

pub static PARALLEL_CODEGEN: DebugFlag = DebugFlag { index: 148, name: literal!("parallelCodegen"), default: true, description: literal!("Enables code generation in parallel (disable this if compiling a model causes you to run out of RAM).") };

pub static SERIALIZED_SIZE: DebugFlag = DebugFlag { index: 149, name: literal!("reportSerializedSize"), default: false, description: literal!("Reports serialized sizes of various data structures used in the compiler.") };

pub static BACKEND_KEEP_ENV_GRAPH: DebugFlag = DebugFlag { index: 150, name: literal!("backendKeepEnv"), default: true, description: literal!("When enabled, the environment is kept when entering the backend, which enables CevalFunction (function interpretation) to work. This module not essential for the backend to function in most cases, but can improve simulation performance by evaluating functions. The drawback to keeping the environment graph in memory is that it is huge (~80% of the total memory in use when returning the frontend DAE).") };

pub static DUMPBACKENDINLINE: DebugFlag = DebugFlag { index: 151, name: literal!("dumpBackendInline"), default: false, description: literal!("Dumps debug output while inline function.") };

pub static DUMPBACKENDINLINE_VERBOSE: DebugFlag = DebugFlag { index: 152, name: literal!("dumpBackendInlineVerbose"), default: false, description: literal!("Dumps debug output while inline function.") };

pub static BLT_MATRIX_DUMP: DebugFlag = DebugFlag { index: 153, name: literal!("bltmatrixdump"), default: false, description: literal!("Dumps the blt matrix in html file. IE seems to be very good in displaying large matrices.") };

pub static LIST_REVERSE_WRONG_ORDER: DebugFlag = DebugFlag { index: 154, name: literal!("listAppendWrongOrder"), default: true, description: literal!("Print notifications about bad usage of listAppend.") };

pub static PARTITION_INITIALIZATION: DebugFlag = DebugFlag { index: 155, name: literal!("partitionInitialization"), default: true, description: literal!("This flag controls if partitioning is applied to the initialization system.") };

pub static EVAL_PARAM_DUMP: DebugFlag = DebugFlag { index: 156, name: literal!("evalParameterDump"), default: false, description: literal!("Dumps information for evaluating parameters.") };

pub(crate) static NF_UNITCHECK: DebugFlag = DebugFlag { index: 157, name: literal!("frontEndUnitCheck"), default: false, description: literal!("Checks the consistency of units in equation.") };

pub static DISABLE_COLORING: DebugFlag = DebugFlag { index: 158, name: literal!("disableColoring"), default: false, description: literal!("Disables coloring algorithm while spasity detection.") };

pub static MERGE_ALGORITHM_SECTIONS: DebugFlag = DebugFlag { index: 159, name: literal!("mergeAlgSections"), default: false, description: literal!("Disables coloring algorithm while sparsity detection.") };

pub static WARN_NO_NOMINAL: DebugFlag = DebugFlag { index: 160, name: literal!("warnNoNominal"), default: false, description: literal!("Prints the iteration variables in the initialization and simulation DAE, which do not have a nominal value.") };

pub static REDUCE_DAE: DebugFlag = DebugFlag { index: 161, name: literal!("backendReduceDAE"), default: false, description: literal!("Prints all Reduce DAE debug information.") };

pub static IGNORE_CYCLES: DebugFlag = DebugFlag { index: 162, name: literal!("ignoreCycles"), default: false, description: literal!("Ignores cycles between constant/parameter components.") };

pub static ALIAS_CONFLICTS: DebugFlag = DebugFlag { index: 163, name: literal!("aliasConflicts"), default: false, description: literal!("Dumps alias sets with different start or nominal values.") };

pub static SUSAN_MATCHCONTINUE_DEBUG: DebugFlag = DebugFlag { index: 164, name: literal!("susanDebug"), default: false, description: literal!("Makes Susan generate code using try/else to better debug which function broke the expected match semantics.") };

pub(crate) static OLD_FE_UNITCHECK: DebugFlag = DebugFlag { index: 165, name: literal!("oldFrontEndUnitCheck"), default: false, description: literal!("Checks the consistency of units in equation (for the old front-end).") };

pub(crate) static EXEC_STAT_EXTRA_GC: DebugFlag = DebugFlag { index: 166, name: literal!("execstatGCcollect"), default: false, description: literal!("When running execstat, also perform an extra full garbage collection.") };

pub static DEBUG_DAEMODE: DebugFlag = DebugFlag { index: 167, name: literal!("debugDAEmode"), default: false, description: literal!("Dump debug output for the DAEmode.") };

pub static NF_SCALARIZE: DebugFlag = DebugFlag { index: 168, name: literal!("nfScalarize"), default: true, description: literal!("Run scalarization in NF, default true.") };

pub static NF_EVAL_CONST_ARG_FUNCS: DebugFlag = DebugFlag { index: 169, name: literal!("nfEvalConstArgFuncs"), default: true, description: literal!("Evaluate all functions with constant arguments in the new frontend.") };

pub static NF_EXPAND_OPERATIONS: DebugFlag = DebugFlag { index: 170, name: literal!("nfExpandOperations"), default: true, description: literal!("Expand all unary/binary operations to scalar expressions in the new frontend.") };

pub static NF_API: DebugFlag = DebugFlag { index: 171, name: literal!("nfAPI"), default: true, description: literal!("Enables experimental new instantiation use in the OMC API.") };

pub static NF_API_DYNAMIC_SELECT: DebugFlag = DebugFlag { index: 172, name: literal!("nfAPIDynamicSelect"), default: false, description: literal!("Show DynamicSelect(static, dynamic) in annotations. Default to false and will select the first (static) expression") };

pub static NF_API_NOISE: DebugFlag = DebugFlag { index: 173, name: literal!("nfAPINoise"), default: false, description: literal!("Enables error display for the experimental new instantiation use in the OMC API.") };

pub static FMI20_DEPENDENCIES: DebugFlag = DebugFlag { index: 174, name: literal!("disableFMIDependency"), default: false, description: literal!("Disables the dependency analysis and generation for FMI 2.0.") };

pub static WARNING_MINMAX_ATTRIBUTES: DebugFlag = DebugFlag { index: 175, name: literal!("warnMinMax"), default: true, description: literal!("Makes a warning assert from min/max variable attributes instead of error.") };

pub static NF_EXPAND_FUNC_ARGS: DebugFlag = DebugFlag { index: 176, name: literal!("nfExpandFuncArgs"), default: false, description: literal!("Expand all function arguments in the new frontend.") };

pub static DUMP_JL: DebugFlag = DebugFlag { index: 177, name: literal!("dumpJL"), default: false, description: literal!("Dumps the absyn representation of a program as a Julia representation") };

pub static DUMP_ASSC: DebugFlag = DebugFlag { index: 178, name: literal!("dumpASSC"), default: false, description: literal!("Dumps the conversion process of analytical to structural singularities.") };

pub static SPLIT_CONSTANT_PARTS_SYMJAC: DebugFlag = DebugFlag { index: 179, name: literal!("symJacConstantSplit"), default: false, description: literal!("Generates all symbolic Jacobians with splitted constant parts.") };

pub static DUMP_FORCE_FMI_ATTRIBUTES: DebugFlag = DebugFlag { index: 180, name: literal!("force-fmi-attributes"), default: false, description: literal!("Force to export all fmi attributes to the modelDescription.xml, including those which have default values") };

pub static DUMP_DATARECONCILIATION: DebugFlag = DebugFlag { index: 181, name: literal!("dataReconciliation"), default: false, description: literal!("Dumps all the dataReconciliation extraction algorithm procedure") };

pub static ARRAY_CONNECT: DebugFlag = DebugFlag { index: 182, name: literal!("arrayConnect"), default: false, description: literal!("Use experimental array connection handler.") };

pub static COMBINE_SUBSCRIPTS: DebugFlag = DebugFlag { index: 183, name: literal!("combineSubscripts"), default: false, description: literal!("Move all subscripts to the end of component references.") };

pub static ZMQ_LISTEN_TO_ALL: DebugFlag = DebugFlag { index: 184, name: literal!("zmqDangerousAcceptConnectionsFromAnywhere"), default: false, description: literal!("When opening a zmq connection, listen on all interfaces instead of only connections from 127.0.0.1.") };

pub static DUMP_CONVERSION_RULES: DebugFlag = DebugFlag { index: 185, name: literal!("dumpConversionRules"), default: false, description: literal!("Dumps the rules when converting a package using a conversion script.") };

pub static PRINT_RECORD_TYPES: DebugFlag = DebugFlag { index: 186, name: literal!("printRecordTypes"), default: false, description: literal!("Prints out record types as part of the flat code.") };

pub static DUMP_SIMPLIFY: DebugFlag = DebugFlag { index: 187, name: literal!("dumpSimplify"), default: false, description: literal!("Dumps expressions before and after simplification.") };

pub static DUMP_BACKEND_CLOCKS: DebugFlag = DebugFlag { index: 188, name: literal!("dumpBackendClocks"), default: false, description: literal!("Dumps times for each backend module (only new backend).") };

pub static DUMP_SET_BASED_GRAPHS: DebugFlag = DebugFlag { index: 189, name: literal!("dumpSetBasedGraphs"), default: false, description: literal!("Dumps information about set based graphs for efficient array handling (only new frontend and new backend).") };

pub static MERGE_COMPONENTS: DebugFlag = DebugFlag { index: 190, name: literal!("mergeComponents"), default: false, description: literal!("Enables automatic merging of components into arrays.") };

pub static DUMP_SLICE: DebugFlag = DebugFlag { index: 191, name: literal!("dumpSlice"), default: false, description: literal!("Dumps information about the slicing process (pseudo-array causalization).") };

pub static VECTORIZE_BINDINGS: DebugFlag = DebugFlag { index: 192, name: literal!("vectorizeBindings"), default: false, description: literal!("Turns on vectorization of bindings when scalarization is turned off.") };

pub static DUMP_EVENTS: DebugFlag = DebugFlag { index: 193, name: literal!("dumpEvents"), default: false, description: literal!("Dumps information about the detected event functions.") };

pub static DUMP_RESIZABLE: DebugFlag = DebugFlag { index: 194, name: literal!("dumpResizable"), default: false, description: literal!("Dumps information about resizable paremeter handling.") };

pub static DUMP_SOLVE: DebugFlag = DebugFlag { index: 195, name: literal!("dumpSolve"), default: false, description: literal!("Dumps information about equation solving.") };

pub static FORCE_SCALARIZE: DebugFlag = DebugFlag { index: 196, name: literal!("forceScalarize"), default: false, description: literal!("Forces scalarization to be done when it would normally be automatically disabled.") };

pub static DEBUG_ADJOINT: DebugFlag = DebugFlag { index: 197, name: literal!("debugAdjoint"), default: false, description: literal!("Dumps debug output for the adjoint differentiation process in the new backend.") };

pub static FLOW_ALIAS_ELIMINATION: DebugFlag = DebugFlag { index: 198, name: literal!("flowAliasElimination"), default: false, description: literal!("Enables simple alias elimination of flow variables in stream connectors.") };

pub static DUMP_CHECK_MODEL: DebugFlag = DebugFlag { index: 199, name: literal!("dumpCheckModel"), default: false, description: literal!("Dumps the variables and equations found by checkModel.") };

// CONFIGURATION FLAGS
pub(crate) static DEBUG: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 1, name: (literal!("debug")).clone(), shortname: Some((literal!("d")).clone()), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Sets debug flags. Use --help=debug to see available flags.")).clone() } });

pub(crate) static HELP: ConfigFlag = ConfigFlag { index: 2, name: literal!("help"), shortname: Some(literal!("h")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Displays the help text. Use --help=topics for more information.") };

pub(crate) static RUNNING_TESTSUITE: ConfigFlag = ConfigFlag { index: 3, name: literal!("running-testsuite"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Used when running the testsuite.") };

pub(crate) static SHOW_VERSION: ConfigFlag = ConfigFlag { index: 4, name: literal!("version"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Print the version and exit.") };

pub static TARGET: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 5, name: (literal!("target")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("gcc")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("gcc")).clone(), (literal!("msvc")).clone(), (literal!("msvc10")).clone(), (literal!("msvc12")).clone(), (literal!("msvc13")).clone(), (literal!("msvc15")).clone(), (literal!("msvc19")).clone(), (literal!("vxworks69")).clone(), (literal!("debugrt")).clone()] }), description: (literal!("Sets the target compiler to use.")).clone() } });

pub static GRAMMAR: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 6, name: (literal!("grammar")).clone(), shortname: Some((literal!("g")).clone()), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: MODELICA.clone(), validValues: list![(literal!("Modelica"), MODELICA.clone()), (literal!("MetaModelica"), METAMODELICA.clone()), (literal!("ParModelica"), PARMODELICA.clone()), (literal!("Optimica"), OPTIMICA.clone()), (literal!("PDEModelica"), PDEMODELICA.clone())] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("Modelica")).clone(), (literal!("MetaModelica")).clone(), (literal!("ParModelica")).clone(), (literal!("Optimica")).clone(), (literal!("PDEModelica")).clone()] }), description: (literal!("Sets the grammar and semantics to accept.")).clone() } });

pub(crate) static ANNOTATION_VERSION: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 7, name: (literal!("annotationVersion")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("3.x")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("1.x")).clone(), (literal!("2.x")).clone(), (literal!("3.x")).clone()] }), description: (literal!("Sets the annotation version that should be used.")).clone() } });

pub static LANGUAGE_STANDARD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 8, name: (literal!("std")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 1000, validValues: list![(literal!("1.x"), 10), (literal!("2.x"), 20), (literal!("3.0"), 30), (literal!("3.1"), 31), (literal!("3.2"), 32), (literal!("3.3"), 33), (literal!("3.4"), 34), (literal!("3.5"), 35), (literal!("3.6"), 36), (literal!("latest"), 1000), (literal!("experimental"), 9999)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("1.x")).clone(), (literal!("2.x")).clone(), (literal!("3.1")).clone(), (literal!("3.2")).clone(), (literal!("3.3")).clone(), (literal!("3.4")).clone(), (literal!("3.5")).clone(), (literal!("3.6")).clone(), (literal!("latest")).clone(), (literal!("experimental")).clone()] }), description: (literal!("Sets the language standard that should be used.")).clone() } });

pub(crate) static SHOW_ERROR_MESSAGES: ConfigFlag = ConfigFlag { index: 9, name: literal!("showErrorMessages"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Show error messages immediately when they happen.") };

pub(crate) static SHOW_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 10, name: literal!("showAnnotations"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Show annotations in the flattened code.") };

pub static NO_SIMPLIFY: ConfigFlag = ConfigFlag { index: 11, name: literal!("noSimplify"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Do not simplify expressions if set.") };

pub(crate) const removeSimpleEquationDesc: &'static str = "Performs alias elimination and removes constant variables from the DAE, replacing all occurrences of the old variable reference with the new value (constants) or variable reference (alias elimination).";

pub(crate) static PRE_OPT_MODULES: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 12, name: (literal!("preOptModules")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("normalInlineFunction")).clone(), (literal!("evaluateParameters")).clone(), (literal!("simplifyIfEquations")).clone(), (literal!("expandDerOperator")).clone(), (literal!("clockPartitioning")).clone(), (literal!("findStateOrder")).clone(), (literal!("replaceEdgeChange")).clone(), (literal!("inlineArrayEqn")).clone(), (literal!("removeEqualRHS")).clone(), (literal!("removeSimpleEquations")).clone(), (literal!("comSubExp")).clone(), (literal!("resolveLoops")).clone(), (literal!("evalFunc")).clone(), (literal!("encapsulateWhenConditions")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("introduceOutputAliases"), literal!("Introduces aliases for top-level outputs.")), (literal!("clockPartitioning"), literal!("Does the clock partitioning.")), (literal!("collapseArrayExpressions"), arcstr::literal!(collapseArrayExpressionsText)), (literal!("comSubExp"), literal!("Introduces alias assignments for variables which are assigned to simple terms i.e. a = b/c; d = b/c; --> a=d")), (literal!("dumpDAE"), literal!("dumps the DAE representation of the current transformation state")), (literal!("dumpDAEXML"), literal!("dumps the DAE as xml representation of the current transformation state")), (literal!("encapsulateWhenConditions"), literal!("This module replaces each when condition with a boolean variable.")), (literal!("evalFunc"), literal!("evaluates functions partially")), (literal!("evaluateParameters"), literal!("Evaluates parameters with annotation(Evaluate=true). Use '--evaluateFinalParameters=true' or '--evaluateProtectedParameters=true' to specify additional parameters to be evaluated. Use '--replaceEvaluatedParameters=true' if the evaluated parameters should be replaced in the DAE. To evaluate all parameters in the Frontend use -d=evaluateAllParameters.")), (literal!("expandDerOperator"), literal!("Expands der(expr) using Derive.differentiteExpTime.")), (literal!("findStateOrder"), literal!("Sets derivative information to states.")), (literal!("inlineArrayEqn"), literal!("This module expands all array equations to scalar equations.")), (literal!("normalInlineFunction"), literal!("Perform function inlining for function with annotation Inline=true.")), (literal!("inputDerivativesForDynOpt"), literal!("Allowed derivatives of inputs in dyn. optimization.")), (literal!("introduceDerAlias"), literal!("Adds for every der-call an alias equation e.g. dx = der(x).")), (literal!("removeEqualRHS"), literal!("Detects equal expressions of the form a=<exp> and b=<exp> and substitutes them to get speed up.")), (literal!("removeProtectedParameters"), literal!("Replace all parameters with protected=true in the system.")), (literal!("removeSimpleEquations"), arcstr::literal!(removeSimpleEquationDesc)), (literal!("removeUnusedParameter"), literal!("Strips all parameter not present in the equations from the system.")), (literal!("removeUnusedVariables"), literal!("Strips all variables not present in the equations from the system.")), (literal!("removeVerySimpleEquations"), literal!("[Experimental] Like removeSimpleEquations, but less thorough. Note that this always uses the experimental new alias elimination, --removeSimpleEquations=new, which makes it unstable. In particular, MultiBody systems fail to translate correctly. It can be used for simple (but large) systems of equations.")), (literal!("replaceEdgeChange"), literal!("Replace edge(b) = b and not pre(b) and change(b) = v <> pre(v).")), (literal!("residualForm"), literal!("Transforms simple equations x=y to zero-sum equations 0=y-x.")), (literal!("resolveLoops"), literal!("resolves linear equations in loops")), (literal!("simplifyAllExpressions"), literal!("Does simplifications on all expressions.")), (literal!("simplifyIfEquations"), literal!("Tries to simplify if equations by use of information from evaluated parameters.")), (literal!("sortEqnsVars"), literal!("Heuristic sorting for equations and variables.")), (literal!("unitChecking"), literal!("This module is no longer available and its use is deprecated. Use --unitChecking instead.")), (literal!("wrapFunctionCalls"), literal!("This module introduces variables for each function call and substitutes all these calls with the newly introduced variables."))] }), description: (literal!("Sets the pre optimization modules to use in the back end. See --help=optmodules for more info.")).clone() } });

pub(crate) static CHEAPMATCHING_ALGORITHM: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 13, name: (literal!("cheapmatchingAlgorithm")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 3 }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("0"), literal!("No cheap matching.")), (literal!("1"), literal!("Cheap matching, traverses all equations and match the first free variable.")), (literal!("3"), literal!("Random Karp-Sipser: R. M. Karp and M. Sipser. Maximum matching in sparse random graphs."))] }), description: (literal!("Sets the cheap matching algorithm to use. A cheap matching algorithm gives a jump start matching by heuristics.")).clone() } });

pub static MATCHING_ALGORITHM: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 14, name: (literal!("matchingAlgorithm")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("PFPlusExt")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("BFSB"), literal!("Breadth First Search based algorithm.")), (literal!("DFSB"), literal!("Depth First Search based algorithm.")), (literal!("MC21A"), literal!("Depth First Search based algorithm with look ahead feature.")), (literal!("PF"), literal!("Depth First Search based algorithm with look ahead feature.")), (literal!("PFPlus"), literal!("Depth First Search based algorithm with look ahead feature and fair row traversal.")), (literal!("HK"), literal!("Combined BFS and DFS algorithm.")), (literal!("HKDW"), literal!("Combined BFS and DFS algorithm.")), (literal!("ABMP"), literal!("Combined BFS and DFS algorithm.")), (literal!("PR"), literal!("Matching algorithm using push relabel mechanism.")), (literal!("DFSBExt"), literal!("Depth First Search based Algorithm external c implementation.")), (literal!("BFSBExt"), literal!("Breadth First Search based Algorithm external c implementation.")), (literal!("MC21AExt"), literal!("Depth First Search based Algorithm with look ahead feature external c implementation.")), (literal!("PFExt"), literal!("Depth First Search based Algorithm with look ahead feature external c implementation.")), (literal!("PFPlusExt"), literal!("Depth First Search based Algorithm with look ahead feature and fair row traversal external c implementation.")), (literal!("HKExt"), literal!("Combined BFS and DFS algorithm external c implementation.")), (literal!("HKDWExt"), literal!("Combined BFS and DFS algorithm external c implementation.")), (literal!("ABMPExt"), literal!("Combined BFS and DFS algorithm external c implementation.")), (literal!("PRExt"), literal!("Matching algorithm using push relabel mechanism external c implementation.")), (literal!("BB"), literal!("BBs try.")), (literal!("SBGraph"), literal!("Set-Based Graph matching algorithm for efficient array handling.")), (literal!("pseudo"), literal!("Pseudo array matching that uses scalar matching and reconstructs arrays afterwards as much as possible."))] }), description: (literal!("Sets the matching algorithm to use. See --help=optmodules for more info.")).clone() } });

pub static INDEX_REDUCTION_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 15, name: (literal!("indexReductionMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("dynamicStateSelection")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Skip index reduction")), (literal!("uode"), literal!("Use the underlying ODE without the constraints.")), (literal!("dynamicStateSelection"), literal!("Simple index reduction method, select (dynamic) dummy states based on analysis of the system.")), (literal!("dummyDerivatives"), literal!("Simple index reduction method, select (static) dummy states based on heuristic."))] }), description: (literal!("Sets the index reduction method to use. See --help=optmodules for more info.")).clone() } });

pub(crate) static POST_OPT_MODULES: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 16, name: (literal!("postOptModules")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("lateInlineFunction")).clone(), (literal!("wrapFunctionCalls")).clone(), (literal!("inlineArrayEqn")).clone(), (literal!("constantLinearSystem")).clone(), (literal!("simplifysemiLinear")).clone(), (literal!("removeSimpleEquations")).clone(), (literal!("simplifyComplexFunction")).clone(), (literal!("solveSimpleEquations")).clone(), (literal!("tearingSystem")).clone(), (literal!("inputDerivativesUsed")).clone(), (literal!("calculateStrongComponentJacobians")).clone(), (literal!("calculateStateSetsJacobians")).clone(), (literal!("symbolicJacobian")).clone(), (literal!("removeConstants")).clone(), (literal!("simplifyTimeIndepFuncCalls")).clone(), (literal!("simplifyAllExpressions")).clone(), (literal!("findZeroCrossings")).clone(), (literal!("collapseArrayExpressions")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("addScaledVars_states"), literal!("added var_norm = var/nominal, where var is state")), (literal!("addScaledVars_inputs"), literal!("added var_norm = var/nominal, where var is input")), (literal!("addTimeAsState"), literal!("Experimental feature: this replaces each occurrence of variable time with a new introduced state $time with equation der($time) = 1.0")), (literal!("calculateStateSetsJacobians"), literal!("Generates analytical jacobian for dynamic state selection sets.")), (literal!("calculateStrongComponentJacobians"), literal!("Generates analytical jacobian for torn linear and non-linear strong components. By default linear components and non-linear components with user-defined function calls are skipped. See also debug flags: LSanalyticJacobian, NLSanalyticJacobian and forceNLSanalyticJacobian")), (literal!("collapseArrayExpressions"), arcstr::literal!(collapseArrayExpressionsText)), (literal!("constantLinearSystem"), literal!("Evaluates constant linear systems (a*x+b*y=c; d*x+e*y=f; a,b,c,d,e,f are constants) at compile-time.")), (literal!("countOperations"), literal!("Count the mathematical operations of the system.")), (literal!("cseBinary"), literal!("Common Sub-expression Elimination")), (literal!("dumpComponentsGraphStr"), literal!("Dumps the assignment graph used to determine strong components to format suitable for Mathematica")), (literal!("dumpDAE"), literal!("dumps the DAE representation of the current transformation state")), (literal!("dumpDAEXML"), literal!("dumps the DAE as xml representation of the current transformation state")), (literal!("evaluateParameters"), literal!("Evaluates parameters with annotation(Evaluate=true). Use '--evaluateFinalParameters=true' or '--evaluateProtectedParameters=true' to specify additional parameters to be evaluated. Use '--replaceEvaluatedParameters=true' if the evaluated parameters should be replaced in the DAE. To evaluate all parameters in the Frontend use -d=evaluateAllParameters.")), (literal!("extendDynamicOptimization"), literal!("Move loops to constraints.")), (literal!("generateSymbolicLinearization"), literal!("Generates symbolic linearization matrices A,B,C,D for linear model:\n\t:math:`\\dot{x} = Ax + Bu`\n\t:math:`y = Cx + Du`")), (literal!("generateSymbolicSensitivities"), literal!("Generates symbolic Sensivities matrix, where der(x) is differentiated w.r.t. param.")), (literal!("inlineArrayEqn"), literal!("This module expands all array equations to scalar equations.")), (literal!("inputDerivativesUsed"), literal!("Checks if derivatives of inputs are need to calculate the model.")), (literal!("lateInlineFunction"), literal!("Perform function inlining for function with annotation LateInline=true.")), (literal!("partlintornsystem"), literal!("partitions linear torn systems.")), (literal!("recursiveTearing"), literal!("inline and repeat tearing")), (literal!("reduceDynamicOptimization"), literal!("Removes equations which are not needed for the calculations of cost and constraints. This module requires --postOptModules+=reduceDynamicOptimization.")), (literal!("relaxSystem"), literal!("relaxation from gausian elemination")), (literal!("removeConstants"), literal!("Remove all constants in the system.")), (literal!("removeEqualRHS"), literal!("Detects equal function calls of the form a=f(b) and c=f(b) and substitutes them to get speed up.")), (literal!("removeSimpleEquations"), arcstr::literal!(removeSimpleEquationDesc)), (literal!("removeUnusedParameter"), literal!("Strips all parameter not present in the equations from the system to get speed up for compilation of target code.")), (literal!("removeUnusedVariables"), literal!("Strips all variables not present in the equations from the system to get speed up for compilation of target code.")), (literal!("reshufflePost"), literal!("Reshuffles algebraic loops.")), (literal!("simplifyAllExpressions"), literal!("Does simplifications on all expressions.")), (literal!("simplifyComplexFunction"), literal!("Some simplifications on complex functions (complex refers to the internal data structure)")), (literal!("simplifyConstraints"), literal!("Rewrites nonlinear constraints into box constraints if possible. This module requires +gDynOpt.")), (literal!("simplifyLoops"), literal!("Simplifies algebraic loops. This modules requires +simplifyLoops.")), (literal!("simplifyTimeIndepFuncCalls"), literal!("Simplifies time independent built in function calls like pre(param) -> param, der(param) -> 0.0, change(param) -> false, edge(param) -> false.")), (literal!("simplifysemiLinear"), literal!("Simplifies calls to semiLinear.")), (literal!("solveLinearSystem"), literal!("solve linear system with newton step")), (literal!("solveSimpleEquations"), literal!("Solves simple equations")), (literal!("symSolver"), literal!("Rewrites the ode system for implicit Euler method. This module requires +symSolver.")), (literal!("symbolicJacobian"), literal!("Detects the sparse pattern of the ODE system and calculates also the symbolic Jacobian if flag '--generateDynamicJacobian=symbolic'.")), (literal!("tearingSystem"), literal!("For method selection use flag tearingMethod.")), (literal!("wrapFunctionCalls"), literal!("This module introduces variables for each function call and substitutes all these calls with the newly introduced variables."))] }), description: (literal!("Sets the post optimization modules to use in the back end. See --help=optmodules for more info.")).clone() } });

pub static SIMCODE_TARGET: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 17, name: (literal!("simCodeTarget")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("C")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("None")).clone(), (literal!("C")).clone(), (literal!("Cpp")).clone(), (literal!("omsicpp")).clone(), (literal!("ExperimentalEmbeddedC")).clone(), (literal!("JavaScript")).clone(), (literal!("omsic")).clone(), (literal!("XML")).clone(), (literal!("MidC")).clone()] }), description: (literal!("Sets the target language for the code generation.")).clone() } });

pub(crate) static ORDER_CONNECTIONS: ConfigFlag = ConfigFlag { index: 18, name: literal!("orderConnections"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Orders connect equations alphabetically if set.") };

pub(crate) static TYPE_INFO: ConfigFlag = ConfigFlag { index: 19, name: literal!("typeinfo"), shortname: Some(literal!("t")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Prints out extra type information if set.") };

pub(crate) static KEEP_ARRAYS: ConfigFlag = ConfigFlag { index: 20, name: literal!("keepArrays"), shortname: Some(literal!("a")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Sets whether to split arrays or not.") };

pub static MODELICA_OUTPUT: ConfigFlag = ConfigFlag { index: 21, name: literal!("modelicaOutput"), shortname: Some(literal!("m")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enables valid modelica output for flat modelica.") };

pub(crate) static SILENT: ConfigFlag = ConfigFlag { index: 22, name: literal!("silent"), shortname: Some(literal!("q")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Turns on silent mode.") };

pub(crate) static CORBA_SESSION: ConfigFlag = ConfigFlag { index: 23, name: literal!("corbaSessionName"), shortname: Some(literal!("c")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Sets the name of the corba session if -d=interactiveCorba or --interactive=corba is used.") };

pub static NUM_PROC: ConfigFlag = ConfigFlag { index: 24, name: literal!("numProcs"), shortname: Some(literal!("n")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Sets the number of processors to use (0=default=auto).") };

pub(crate) static INST_CLASS: ConfigFlag = ConfigFlag { index: 25, name: literal!("instClass"), shortname: Some(literal!("i")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Instantiate the class given by the fully qualified path.") };

pub(crate) static VECTORIZATION_LIMIT: ConfigFlag = ConfigFlag { index: 26, name: literal!("vectorizationLimit"), shortname: Some(literal!("v")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Sets the vectorization limit, arrays and matrices larger than this will not be vectorized.") };

pub(crate) static SIMULATION_CG: ConfigFlag = ConfigFlag { index: 27, name: literal!("simulationCg"), shortname: Some(literal!("s")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Turns on simulation code generation.") };

pub(crate) static EVAL_PARAMS_IN_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 28, name: literal!("evalAnnotationParams"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Sets whether to evaluate parameters in annotations or not.") };

pub static CHECK_MODEL: ConfigFlag = ConfigFlag { index: 29, name: literal!("checkModel"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Set when checkModel is used to turn on specific features for checking.") };

pub static CEVAL_EQUATION: ConfigFlag = ConfigFlag { index: 30, name: literal!("cevalEquation"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("") };

pub static UNIT_CHECKING: ConfigFlag = ConfigFlag { index: 31, name: literal!("unitChecking"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enable unit checking.") };

pub static GENERATE_LABELED_SIMCODE: ConfigFlag = ConfigFlag { index: 32, name: literal!("generateLabeledSimCode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Turns on labeled SimCode generation for reduction algorithms.") };

pub static REDUCE_TERMS: ConfigFlag = ConfigFlag { index: 33, name: literal!("reduceTerms"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Turns on reducing terms for reduction algorithms.") };

pub static REDUCTION_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 34, name: (literal!("reductionMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("deletion")).clone() }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("deletion")).clone(), (literal!("substitution")).clone(), (literal!("linearization")).clone()] }), description: (literal!("Sets the reduction method to be used.")).clone() } });

pub(crate) static DEMO_MODE: ConfigFlag = ConfigFlag { index: 35, name: literal!("demoMode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Disable Warning/Error Massages.") };

pub(crate) static LOCALE_FLAG: ConfigFlag = ConfigFlag { index: 36, name: literal!("locale"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Override the locale from the environment.") };

pub(crate) static DEFAULT_OPENCL_DEVICE: ConfigFlag = ConfigFlag { index: 37, name: literal!("defaultOCLDevice"), shortname: Some(literal!("o")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Sets the default OpenCL device to be used for parallel execution.") };

pub static MAXTRAVERSALS: ConfigFlag = ConfigFlag { index: 38, name: literal!("maxTraversals"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 2 }, validOptions: None, description: literal!("Maximal traversals to find simple equations in the acausal system.") };

pub static DUMP_TARGET: ConfigFlag = ConfigFlag { index: 39, name: literal!("dumpTarget"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Redirect the dump to file. If the file ends with .html HTML code is generated.") };

pub static DELAY_BREAK_LOOP: ConfigFlag = ConfigFlag { index: 40, name: literal!("delayBreakLoop"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Enables (very) experimental code to break algebraic loops using the delay() operator. Probably messes with initialization.") };

pub static TEARING_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 41, name: (literal!("tearingMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("cellier")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("noTearing"), literal!("Deprecated, use minimalTearing.")), (literal!("minimalTearing"), literal!("Minimal tearing method to only tear discrete variables.")), (literal!("omcTearing"), literal!("Tearing method developed by TU Dresden: Frenkel, Schubert.")), (literal!("cellier"), literal!("Tearing based on Celliers method, revised by FH Bielefeld: Täuber, Patrick")), (literal!("guruTearing"), literal!("Tearing based solely on TearingSelect annotation. Forces prefer/always variables to be iteration variables."))] }), description: (literal!("Sets the tearing method to use. Select no tearing or choose tearing method.")).clone() } });

pub(crate) static TEARING_HEURISTIC: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 42, name: (literal!("tearingHeuristic")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("MC3")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("MC1"), literal!("Original cellier with consideration of impossible assignments and discrete Vars.")), (literal!("MC2"), literal!("Modified cellier, drop first step.")), (literal!("MC11"), literal!("Modified MC1, new last step 'count impossible assignments'.")), (literal!("MC21"), literal!("Modified MC2, new last step 'count impossible assignments'.")), (literal!("MC12"), literal!("Modified MC1, step 'count impossible assignments' before last step.")), (literal!("MC22"), literal!("Modified MC2, step 'count impossible assignments' before last step.")), (literal!("MC13"), literal!("Modified MC1, build sum of impossible assignment and causalizable equations, choose var with biggest sum.")), (literal!("MC23"), literal!("Modified MC2, build sum of impossible assignment and causalizable equations, choose var with biggest sum.")), (literal!("MC231"), literal!("Modified MC23, Two rounds, choose better potentials-set.")), (literal!("MC3"), literal!("Modified cellier, build sum of impossible assignment and causalizable equations for all vars, choose var with biggest sum.")), (literal!("MC4"), literal!("Modified cellier, use all heuristics, choose var that occurs most in potential sets"))] }), description: (literal!("Sets the tearing heuristic to use for Cellier-tearing.")).clone() } });

pub(crate) static SCALARIZE_MINMAX: ConfigFlag = ConfigFlag { index: 43, name: literal!("scalarizeMinMax"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Scalarizes the builtin min/max reduction operators if true.") };

pub static STRICT: ConfigFlag = ConfigFlag { index: 44, name: literal!("strict"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enables stricter enforcement of Modelica language rules.") };

pub(crate) static SCALARIZE_BINDINGS: ConfigFlag = ConfigFlag { index: 45, name: literal!("scalarizeBindings"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Always scalarizes bindings if set.") };

pub(crate) static CORBA_OBJECT_REFERENCE_FILE_PATH: ConfigFlag = ConfigFlag { index: 46, name: literal!("corbaObjectReferenceFilePath"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Sets the path for corba object reference file if -d=interactiveCorba is used.") };

pub static HPCOM_SCHEDULER: ConfigFlag = ConfigFlag { index: 47, name: literal!("hpcomScheduler"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("level") }, validOptions: None, description: literal!("Sets the scheduler for task graph scheduling (list | listr | level | levelfix | ext | metis | mcp | taskdep | tds | bls | rand | none). Default: level.") };

pub static HPCOM_CODE: ConfigFlag = ConfigFlag { index: 48, name: literal!("hpcomCode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("openmp") }, validOptions: None, description: literal!("Sets the code-type produced by hpcom (openmp | pthreads | pthreads_spin | tbb | mpi). Default: openmp.") };

pub static REWRITE_RULES_FILE: ConfigFlag = ConfigFlag { index: 49, name: literal!("rewriteRulesFile"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Activates user given rewrite rules for Absyn expressions. The rules are read from the given file and are of the form rewrite(fromExp, toExp);") };

pub static REPLACE_HOMOTOPY: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 50, name: (literal!("replaceHomotopy")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Default, do not replace homotopy.")), (literal!("actual"), literal!("Replace homotopy(actual, simplified) with actual.")), (literal!("simplified"), literal!("Replace homotopy(actual, simplified) with simplified."))] }), description: (literal!("Replaces homotopy(actual, simplified) with the actual expression or the simplified expression. Good for debugging models which use homotopy. The default is to not replace homotopy.")).clone() } });

pub static GENERATE_DYNAMIC_JACOBIAN: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 51, name: (literal!("generateDynamicJacobian")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("numeric")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Does not generate Jacobian. For use with explicit solvers.")), (literal!("numeric"), literal!("Generates sparsity pattern for numeric Jacobian.")), (literal!("symbolic"), literal!("Generates symbolic Jacobian. Used by dassl or ida solver with simulation flag '-jacobian'.")), (literal!("symbolicadjoint"), literal!("Generates adjoint Jacobian symbolically."))] }), description: (literal!("Select how Jacobian matrix is generated, where der(x) is differentiated w.r.t. x.")).clone() } });

pub static GENERATE_SYMBOLIC_LINEARIZATION: ConfigFlag = ConfigFlag { index: 52, name: literal!("generateSymbolicLinearization"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Generates symbolic linearization matrices A,B,C,D for linear model:\n\t\t:math:`\\dot{x} = Ax + Bu`\n\t\t:math:`y = Cx + Du`") };

pub(crate) static INT_ENUM_CONVERSION: ConfigFlag = ConfigFlag { index: 53, name: literal!("intEnumConversion"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Allow Integer to enumeration conversion.") };

pub static PROFILING_LEVEL: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 54, name: (literal!("profiling")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Generate code without profiling")), (literal!("blocks"), literal!("Generate code for profiling function calls as well as linear and non-linear systems of equations")), (literal!("blocks+html"), literal!("Like blocks, but also run xsltproc and gnuplot to generate an html report")), (literal!("all"), literal!("Generate code for profiling of all functions and equations")), (literal!("all_perf"), literal!("Generate code for profiling of all functions and equations with additional performance data using the papi-interface (cpp-runtime)")), (literal!("all_stat"), literal!("Generate code for profiling of all functions and equations with additional statistics (cpp-runtime)"))] }), description: (literal!("Sets the profiling level to use. Profiled equations and functions record execution time and count for each time step taken by the integrator.")).clone() } });

pub static RESHUFFLE: ConfigFlag = ConfigFlag { index: 55, name: literal!("reshuffle"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 1 }, validOptions: None, description: literal!("sets tolerance of reshuffling algorithm: 1: conservative, 2: more tolerant, 3 resolve all") };

pub static GENERATE_DYN_OPTIMIZATION_PROBLEM: ConfigFlag = ConfigFlag { index: 56, name: literal!("gDynOpt"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Generate dynamic optimization problem based on annotation approach.") };

pub static MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM: ConfigFlag = ConfigFlag { index: 57, name: literal!("maxSizeSolveLinearSystem"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Max size for solveLinearSystem.") };

pub static CPP_FLAGS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 58, name: (literal!("cppFlags")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("")).clone()] }, validOptions: None, description: (literal!("Sets extra flags for compilation with the C++ compiler (e.g. +cppFlags=-O3,-Wall)")).clone() } });

pub static REMOVE_SIMPLE_EQUATIONS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 59, name: (literal!("removeSimpleEquations")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("default")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Disables module")), (literal!("default"), literal!("Performs alias elimination and removes constant variables. Default case uses in preOpt phase the fastAcausal and in postOpt phase the causal implementation.")), (literal!("causal"), literal!("Performs alias elimination and removes constant variables. Causal implementation.")), (literal!("fastAcausal"), literal!("Performs alias elimination and removes constant variables. fastImplementation fastAcausal.")), (literal!("allAcausal"), literal!("Performs alias elimination and removes constant variables. Implementation allAcausal.")), (literal!("new"), literal!("New implementation (experimental)"))] }), description: (literal!("Specifies method that removes simple equations.")).clone() } });

pub(crate) static DYNAMIC_TEARING: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 60, name: (literal!("dynamicTearing")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("false")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("false"), literal!("No dynamic tearing.")), (literal!("true"), literal!("Dynamic tearing for linear and nonlinear systems.")), (literal!("linear"), literal!("Dynamic tearing only for linear systems.")), (literal!("nonlinear"), literal!("Dynamic tearing only for nonlinear systems."))] }), description: (literal!("Activates dynamic tearing (TearingSet can be changed automatically during runtime, strict set vs. casual set.)")).clone() } });

pub static SYM_SOLVER: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 61, name: (literal!("symSolver")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 0, validValues: list![(literal!("none"), 0), (literal!("impEuler"), 1), (literal!("expEuler"), 2)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("none")).clone(), (literal!("impEuler")).clone(), (literal!("expEuler")).clone()] }), description: (literal!("Activates symbolic implicit solver (original system is not changed).")).clone() } });

pub static LOOP2CON: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 62, name: (literal!("loop2con")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Disables module")), (literal!("lin"), literal!("linear loops --> constraints")), (literal!("noLin"), literal!("no linear loops --> constraints")), (literal!("all"), literal!("loops --> constraints"))] }), description: (literal!("Specifies method that transform loops in constraints. hint: using initial guess from file!")).clone() } });

pub static FORCE_TEARING: ConfigFlag = ConfigFlag { index: 63, name: literal!("forceTearing"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Use tearing set even if it is not smaller than the original component.") };

pub static SIMPLIFY_LOOPS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 64, name: (literal!("simplifyLoops")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("0"), literal!("do nothing")), (literal!("1"), literal!("special modification of residual expressions")), (literal!("2"), literal!("special modification of residual expressions with helper variables"))] }), description: (literal!("Simplify algebraic loops.")).clone() } });

pub static RTEARING: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 65, name: (literal!("recursiveTearing")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("0"), literal!("do nothing")), (literal!("1"), literal!("linear tearing set of size 1")), (literal!("2"), literal!("linear tearing"))] }), description: (literal!("Inline and repeat tearing.")).clone() } });

pub static FLOW_THRESHOLD: ConfigFlag = ConfigFlag { index: 66, name: literal!("flowThreshold"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::REAL_FLAG { data: metamodelica::OrderedFloat(1e-7_f64) }, validOptions: None, description: literal!("Sets the minium threshold for stream flow rates") };

pub static MATRIX_FORMAT: ConfigFlag = ConfigFlag { index: 67, name: literal!("matrixFormat"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("dense") }, validOptions: None, description: literal!("Sets the matrix format type in cpp runtime which should be used (dense | sparse ). Default: dense.") };

pub static PARTLINTORN: ConfigFlag = ConfigFlag { index: 68, name: literal!("partlintorn"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Sets the limit for partitionin of linear torn systems.") };

pub(crate) static INIT_OPT_MODULES: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 69, name: (literal!("initOptModules")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("simplifyComplexFunction")).clone(), (literal!("tearingSystem")).clone(), (literal!("solveSimpleEquations")).clone(), (literal!("calculateStrongComponentJacobians")).clone(), (literal!("simplifyAllExpressions")).clone(), (literal!("collapseArrayExpressions")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("calculateStrongComponentJacobians"), literal!("Generates analytical jacobian for torn linear and non-linear strong components. By default linear components and non-linear components with user-defined function calls are skipped. See also debug flags: LSanalyticJacobian, NLSanalyticJacobian and forceNLSanalyticJacobian")), (literal!("collapseArrayExpressions"), arcstr::literal!(collapseArrayExpressionsText)), (literal!("inlineArrayEqn"), literal!("This module expands all array equations to scalar equations.")), (literal!("constantLinearSystem"), literal!("Evaluates constant linear systems (a*x+b*y=c; d*x+e*y=f; a,b,c,d,e,f are constants) at compile-time.")), (literal!("extendDynamicOptimization"), literal!("Move loops to constraints.")), (literal!("generateHomotopyComponents"), literal!("Finds the parts of the DAE that have to be handled by the homotopy solver and creates a strong component out of it.")), (literal!("inlineHomotopy"), literal!("Experimental: Inlines the homotopy expression to allow symbolic simplifications.")), (literal!("inputDerivativesUsed"), literal!("Checks if derivatives of inputs are need to calculate the model.")), (literal!("recursiveTearing"), literal!("inline and repeat tearing")), (literal!("reduceDynamicOptimization"), literal!("Removes equations which are not needed for the calculations of cost and constraints. This module requires --postOptModules+=reduceDynamicOptimization.")), (literal!("replaceHomotopyWithSimplified"), literal!("Replaces the homotopy expression homotopy(actual, simplified) with the simplified part.")), (literal!("simplifyAllExpressions"), literal!("Does simplifications on all expressions.")), (literal!("simplifyComplexFunction"), literal!("Some simplifications on complex functions (complex refers to the internal data structure)")), (literal!("simplifyConstraints"), literal!("Rewrites nonlinear constraints into box constraints if possible. This module requires +gDynOpt.")), (literal!("simplifyLoops"), literal!("Simplifies algebraic loops. This modules requires +simplifyLoops.")), (literal!("solveSimpleEquations"), literal!("Solves simple equations")), (literal!("tearingSystem"), literal!("For method selection use flag tearingMethod.")), (literal!("wrapFunctionCalls"), literal!("This module introduces variables for each function call and substitutes all these calls with the newly introduced variables."))] }), description: (literal!("Sets the initialization optimization modules to use in the back end. See --help=optmodules for more info.")).clone() } });

pub static MAX_MIXED_DETERMINED_INDEX: ConfigFlag = ConfigFlag { index: 70, name: literal!("maxMixedDeterminedIndex"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 10 }, validOptions: None, description: literal!("Sets the maximum mixed-determined index that is handled by the initialization.") };

pub static USE_LOCAL_DIRECTION: ConfigFlag = ConfigFlag { index: 71, name: literal!("useLocalDirection"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Keeps the input/output prefix for all variables in the flat model, not only top-level ones.") };

pub static DEFAULT_OPT_MODULES_ORDERING: ConfigFlag = ConfigFlag { index: 72, name: literal!("defaultOptModulesOrdering"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("If this is activated, then the specified pre-/post-/init-optimization modules will be rearranged to the recommended ordering.") };

pub static PRE_OPT_MODULES_ADD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 73, name: (literal!("preOptModules+")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Enables additional pre-optimization modules, e.g. --preOptModules+=module1,module2 would additionally enable module1 and module2. See --help=optmodules for more info.")).clone() } });

pub static PRE_OPT_MODULES_SUB: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 74, name: (literal!("preOptModules-")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Disables a list of pre-optimization modules, e.g. --preOptModules-=module1,module2 would disable module1 and module2. See --help=optmodules for more info.")).clone() } });

pub static POST_OPT_MODULES_ADD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 75, name: (literal!("postOptModules+")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Enables additional post-optimization modules, e.g. --postOptModules+=module1,module2 would additionally enable module1 and module2. See --help=optmodules for more info.")).clone() } });

pub static POST_OPT_MODULES_SUB: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 76, name: (literal!("postOptModules-")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Disables a list of post-optimization modules, e.g. --postOptModules-=module1,module2 would disable module1 and module2. See --help=optmodules for more info.")).clone() } });

pub static INIT_OPT_MODULES_ADD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 77, name: (literal!("initOptModules+")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Enables additional init-optimization modules, e.g. --initOptModules+=module1,module2 would additionally enable module1 and module2. See --help=optmodules for more info.")).clone() } });

pub static INIT_OPT_MODULES_SUB: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 78, name: (literal!("initOptModules-")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Disables a list of init-optimization modules, e.g. --initOptModules-=module1,module2 would disable module1 and module2. See --help=optmodules for more info.")).clone() } });

pub static PERMISSIVE: ConfigFlag = ConfigFlag { index: 79, name: literal!("permissive"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Disables some error checks to allow erroneous models to compile.") };

pub static HETS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 80, name: (literal!("hets")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("do nothing")), (literal!("derCalls"), literal!("sort terms based on der-calls"))] }), description: (literal!("Heuristic equation terms sort")).clone() } });

pub static DEFAULT_CLOCK_PERIOD: ConfigFlag = ConfigFlag { index: 81, name: literal!("defaultClockPeriod"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::REAL_FLAG { data: metamodelica::OrderedFloat(1.0_f64) }, validOptions: None, description: literal!("Sets the default clock period (in seconds) for state machines (default: 1.0).") };

pub static INST_CACHE_SIZE: ConfigFlag = ConfigFlag { index: 82, name: literal!("instCacheSize"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 25343 }, validOptions: None, description: literal!("Sets the size of the internal hash table used for instantiation caching.") };

pub static MAX_SIZE_LINEAR_TEARING: ConfigFlag = ConfigFlag { index: 83, name: literal!("maxSizeLinearTearing"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 200 }, validOptions: None, description: literal!("Sets the maximum system size for tearing of linear systems (default 200).") };

pub static MAX_SIZE_NONLINEAR_TEARING: ConfigFlag = ConfigFlag { index: 84, name: literal!("maxSizeNonlinearTearing"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 10000 }, validOptions: None, description: literal!("Sets the maximum system size for tearing of nonlinear systems (default 10000).") };

pub static NO_TEARING_FOR_COMPONENT: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 85, name: (literal!("noTearingForComponent")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Deactivates tearing for the specified components.\nUse '-d=tearingdump' to find out the relevant indexes.")).clone() } });

pub static CT_STATE_MACHINES: ConfigFlag = ConfigFlag { index: 86, name: literal!("ctStateMachines"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Experimental: Enable continuous-time state machine prototype") };

pub static DAE_MODE: ConfigFlag = ConfigFlag { index: 87, name: literal!("daeMode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Generates code to simulate models in DAE mode. The whole system is passed directly to the DAE solver SUNDIALS/IDA and no algebraic solver is involved in the simulation process.") };

pub static INLINE_METHOD: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 88, name: (literal!("inlineMethod")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 1, validValues: list![(literal!("replace"), 1), (literal!("append"), 2)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("replace")).clone(), (literal!("append")).clone()] }), description: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sets the inline method to use.\n")); __mm_s.push_str(&*literal!("replace : This method inlines by replacing in place all expressions. Might lead to very long expression.\n")); __mm_s.push_str(&*literal!("append  : This method inlines by adding additional variables to the whole system. Might lead to much bigger system.")); ArcStr::from(__mm_s) }).clone() } });

pub static SET_TEARING_VARS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 89, name: (literal!("setTearingVars")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Sets the tearing variables by its strong component indexes. Use '-d=tearingdump' to find out the relevant indexes.\nUse following format: '--setTearingVars=(sci,n,t1,...,tn)*', with sci = strong component index, n = number of tearing variables, t1,...tn = tearing variables.\nE.g.: '--setTearingVars=4,2,3,5' would select variables 3 and 5 in strong component 4.")).clone() } });

pub static SET_RESIDUAL_EQNS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 90, name: (literal!("setResidualEqns")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Sets the residual equations by its strong component indexes. Use '-d=tearingdump' to find out the relevant indexes for the collective equations.\nUse following format: '--setResidualEqns=(sci,n,r1,...,rn)*', with sci = strong component index, n = number of residual equations, r1,...rn = residual equations.\nE.g.: '--setResidualEqns=4,2,3,5' would select equations 3 and 5 in strong component 4.\nOnly works in combination with 'setTearingVars'.")).clone() } });

pub(crate) static IGNORE_COMMAND_LINE_OPTIONS_ANNOTATION: ConfigFlag = ConfigFlag { index: 91, name: literal!("ignoreCommandLineOptionsAnnotation"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Ignores the command line options specified as annotation in the class.") };

pub static CALCULATE_SENSITIVITIES: ConfigFlag = ConfigFlag { index: 92, name: literal!("calculateSensitivities"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Generates sensitivities variables and matrices.") };

pub static ALARM: ConfigFlag = ConfigFlag { index: 93, name: literal!("alarm"), shortname: Some(literal!("r")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Sets the number seconds until omc timeouts and exits. Used by the testing framework to terminate infinite running processes.") };

pub static TOTAL_TEARING: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 94, name: (literal!("totalTearing")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Activates total tearing (determination of all possible tearing sets) for the specified components.\nUse '-d=tearingdump' to find out the relevant indexes.")).clone() } });

pub static IGNORE_SIMULATION_FLAGS_ANNOTATION: ConfigFlag = ConfigFlag { index: 95, name: literal!("ignoreSimulationFlagsAnnotation"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Ignores the simulation flags specified as annotation in the class.") };

pub static DYNAMIC_TEARING_FOR_INITIALIZATION: ConfigFlag = ConfigFlag { index: 96, name: literal!("dynamicTearingForInitialization"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enable Dynamic Tearing also for the initialization system.") };

pub static PREFER_TVARS_WITH_START_VALUE: ConfigFlag = ConfigFlag { index: 97, name: literal!("preferTVarsWithStartValue"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Prefer tearing variables with start value for initialization.") };

pub static EQUATIONS_PER_FILE: ConfigFlag = ConfigFlag { index: 98, name: literal!("equationsPerFile"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 500 }, validOptions: None, description: literal!("Generate code for at most this many equations per C-file (partially implemented in the compiler).") };

pub static EVALUATE_FINAL_PARAMS: ConfigFlag = ConfigFlag { index: 99, name: literal!("evaluateFinalParameters"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Evaluates all the final parameters in addition to parameters with annotation(Evaluate=true).") };

pub static EVALUATE_PROTECTED_PARAMS: ConfigFlag = ConfigFlag { index: 100, name: literal!("evaluateProtectedParameters"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Evaluates all the protected parameters in addition to parameters with annotation(Evaluate=true).") };

pub static REPLACE_EVALUATED_PARAMS: ConfigFlag = ConfigFlag { index: 101, name: literal!("replaceEvaluatedParameters"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Replaces all the evaluated parameters in the DAE.") };

pub static CONDENSE_ARRAYS: ConfigFlag = ConfigFlag { index: 102, name: literal!("condenseArrays"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Sets whether array expressions containing function calls are condensed or not.") };

pub static WFC_ADVANCED: ConfigFlag = ConfigFlag { index: 103, name: literal!("wfcAdvanced"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("wrapFunctionCalls ignores more then default cases, e.g. exp, sin, cos, log, (experimental flag)") };

pub(crate) static GRAPHICS_EXP_MODE: ConfigFlag = ConfigFlag { index: 104, name: literal!("graphicsExpMode"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Sets whether we are in graphics exp mode (evaluating icons).") };

pub static TEARING_STRICTNESS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 105, name: (literal!("tearingStrictness")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("strict")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("casual"), literal!("Loose tearing rules using ExpressionSolve to determine the solvability instead of considering the partial derivative. Allows to solve for everything that is analytically possible. This could lead to singularities during simulation.")), (literal!("strict"), literal!("Robust tearing rules by consideration of the partial derivative. Allows to divide by parameters that are not equal to or close to zero.")), (literal!("veryStrict"), literal!("Very strict tearing rules that do not allow to divide by any parameter. Use this if you aim at overriding parameters after compilation with values equal to or close to zero."))] }), description: (literal!("Sets the strictness of the tearing method regarding the solvability restrictions.")).clone() } });

pub static INTERACTIVE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 106, name: (literal!("interactive")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("do nothing")), (literal!("corba"), literal!("Starts omc as a server listening on the Corba interface.")), (literal!("tcp"), literal!("Starts omc as a server listening on the socket interface.")), (literal!("zmq"), literal!("Starts omc as a ZeroMQ server listening on the socket interface."))] }), description: (literal!("Sets the interactive mode for omc.")).clone() } });

pub static ZEROMQ_FILE_SUFFIX: ConfigFlag = ConfigFlag { index: 107, name: literal!("zeroMQFileSuffix"), shortname: Some(literal!("z")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Sets the file suffix for zeroMQ port file if --interactive=zmq is used.") };

pub(crate) static HOMOTOPY_APPROACH: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 108, name: (literal!("homotopyApproach")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("equidistantGlobal")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("equidistantLocal"), literal!("Local homotopy approach with equidistant lambda steps. The homotopy parameter only effects the local strongly connected component.")), (literal!("adaptiveLocal"), literal!("Local homotopy approach with adaptive lambda steps. The homotopy parameter only effects the local strongly connected component.")), (literal!("equidistantGlobal"), literal!("Default, global homotopy approach with equidistant lambda steps. The homotopy parameter effects the entire initialization system.")), (literal!("adaptiveGlobal"), literal!("Global homotopy approach with adaptive lambda steps. The homotopy parameter effects the entire initialization system."))] }), description: (literal!("Sets the homotopy approach.")).clone() } });

pub static IGNORE_REPLACEABLE: ConfigFlag = ConfigFlag { index: 109, name: literal!("ignoreReplaceable"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Sets whether to ignore replaceability or not when redeclaring.") };

pub static LABELED_REDUCTION: ConfigFlag = ConfigFlag { index: 110, name: literal!("labeledReduction"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Turns on labeling and reduce terms to do whole process of reduction.") };

pub static DISABLE_EXTRA_LABELING: ConfigFlag = ConfigFlag { index: 111, name: literal!("disableExtraLabeling"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Disable adding extra label into the whole expression with more than one term and +,- operations.") };

pub static LOAD_MSL_MODEL: ConfigFlag = ConfigFlag { index: 112, name: literal!("loadMSLModel"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Used to know loadFile doesn't need to be called in cpp-runtime (for labeled model reduction).") };

pub static LOAD_PACKAGE_FILE: ConfigFlag = ConfigFlag { index: 113, name: literal!("loadPackageFile"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Used when the outside name is different with the inside name of the packge, in cpp-runtime (for labeled model reduction).") };

pub static BUILDING_FMU: ConfigFlag = ConfigFlag { index: 114, name: literal!(""), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Is true when building an FMU (so the compiler can look for URIs to package as FMI resources).") };

pub static BUILDING_MODEL: ConfigFlag = ConfigFlag { index: 115, name: literal!(""), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Is true when building a model (as opposed to running a Modelica script).") };

pub(crate) static POST_OPT_MODULES_DAE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 116, name: (literal!("postOptModulesDAE")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("lateInlineFunction")).clone(), (literal!("wrapFunctionCalls")).clone(), (literal!("simplifysemiLinear")).clone(), (literal!("simplifyComplexFunction")).clone(), (literal!("removeConstants")).clone(), (literal!("simplifyTimeIndepFuncCalls")).clone(), (literal!("simplifyAllExpressions")).clone(), (literal!("findZeroCrossings")).clone(), (literal!("createDAEmodeBDAE")).clone(), (literal!("symbolicJacobianDAE")).clone(), (literal!("setEvaluationStage")).clone()] }, validOptions: None, description: (literal!("Sets the optimization modules for the DAEmode in the back end. See --help=optmodules for more info.")).clone() } });

pub static EVAL_LOOP_LIMIT: ConfigFlag = ConfigFlag { index: 117, name: literal!("evalLoopLimit"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 100000 }, validOptions: None, description: literal!("The loop iteration limit used when evaluating constant function calls.") };

pub static EVAL_RECURSION_LIMIT: ConfigFlag = ConfigFlag { index: 118, name: literal!("evalRecursionLimit"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 256 }, validOptions: None, description: literal!("The recursion limit used when evaluating constant function calls.") };

pub static SINGLE_INSTANCE_AGLSOLVER: ConfigFlag = ConfigFlag { index: 119, name: literal!("singleInstanceAglSolver"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Sets to instantiate only  one algebraic loop solver all algebraic loops") };

pub(crate) static SHOW_STRUCTURAL_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 120, name: literal!("showStructuralAnnotations"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Show annotations affecting the solution process in the flattened code.") };

pub static INITIAL_STATE_SELECTION: ConfigFlag = ConfigFlag { index: 121, name: literal!("initialStateSelection"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Activates the state selection inside initialization to avoid singularities.") };

pub static LINEARIZATION_DUMP_LANGUAGE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 122, name: (literal!("linearizationDumpLanguage")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("Don't generate code for linearization.")), (literal!("modelica"), literal!("Generate linearized Modelica model.")), (literal!("matlab"), literal!("Generate matlab function that returns linearization matrices A,B,C,D.")), (literal!("julia"), literal!("Generate julia function that returns linearization matrices A,B,C,D.")), (literal!("python"), literal!("Generate python function that returns linearization matrices A,B,C,D."))] }), description: (literal!("Sets the target language for the produced code of linearization.")).clone() } });

pub static NO_ASSC: ConfigFlag = ConfigFlag { index: 123, name: literal!("noASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Disables analytical to structural singularity conversion.") };

pub static FULL_ASSC: ConfigFlag = ConfigFlag { index: 124, name: literal!("fullASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enables full equation replacement for BLT transformation from the ASSC algorithm.") };

pub static REAL_ASSC: ConfigFlag = ConfigFlag { index: 125, name: literal!("realASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enables the ASSC algorithm to evaluate real valued coefficients (usually only integers).") };

pub static INIT_ASSC: ConfigFlag = ConfigFlag { index: 126, name: literal!("initASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enables the ASSC algorithm for initialization.") };

pub static MAX_SIZE_ASSC: ConfigFlag = ConfigFlag { index: 127, name: literal!("maxSizeASSC"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 200 }, validOptions: None, description: literal!("Sets the maximum system size for the analytical to structural conversion algorithm (default 200).") };

pub static USE_ZEROMQ_IN_SIM: ConfigFlag = ConfigFlag { index: 128, name: literal!("useZeroMQInSim"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Configures to use zeroMQ in simulation runtime to exchange information via ZeroMQ with other applications") };

pub static ZEROMQ_PUB_PORT: ConfigFlag = ConfigFlag { index: 129, name: literal!("zeroMQPubPort"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::INT_FLAG { data: 3203 }, validOptions: None, description: literal!("Configures port number for simulation runtime to send information via ZeroMQ") };

pub static ZEROMQ_SUB_PORT: ConfigFlag = ConfigFlag { index: 130, name: literal!("zeroMQSubPort"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::INT_FLAG { data: 3204 }, validOptions: None, description: literal!("Configures port number for simulation runtime to receive information via ZeroMQ") };

pub static ZEROMQ_JOB_ID: ConfigFlag = ConfigFlag { index: 131, name: literal!("zeroMQJOBID"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("empty") }, validOptions: None, description: literal!("Configures the ID with which the omc api call is labelled for zeroMQ communication.") };

pub static ZEROMQ_SERVER_ID: ConfigFlag = ConfigFlag { index: 132, name: literal!("zeroMQServerID"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("empty") }, validOptions: None, description: literal!("Configures the ID with which server application is labelled for zeroMQ communication.") };

pub static ZEROMQ_CLIENT_ID: ConfigFlag = ConfigFlag { index: 133, name: literal!("zeroMQClientID"), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("empty") }, validOptions: None, description: literal!("Configures the ID with which the client application is labelled for zeroMQ communication.") };

pub static FMI_VERSION: ConfigFlag = ConfigFlag { index: 134, name: literal!(""), shortname: None, visibility: crate::Flags::FlagVisibility::INTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("returns the FMI Version either 1.0 or 2.0.") };

pub static BASE_MODELICA: ConfigFlag = ConfigFlag { index: 135, name: literal!("baseModelica"), shortname: Some(literal!("f")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Outputs experimental Base Modelica.") };

pub static FMI_FILTER: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 136, name: (literal!("fmiFilter")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: FMI_PROTECTED.clone(), validValues: list![(literal!("none"), FMI_NONE.clone()), (literal!("internal"), FMI_INTERNAL.clone()), (literal!("protected"), FMI_PROTECTED.clone()), (literal!("blackBox"), FMI_BLACKBOX.clone())] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("All variables are exposed, even variables introduced by the symbolic transformations. This is mainly for debugging purposes.")), (literal!("internal"), literal!("All model variables are exposed, including protected ones. Variables introduced by the symbolic transformations are filtered out, with minor exceptions, e.g. for state sets.")), (literal!("protected"), literal!("All public model variables are exposed. Internal and protected variables are filtered out, with small exceptions, e.g. for state sets.")), (literal!("blackBox"), literal!("Only the interface is exposed. All other variables are hidden or exposed with concealed names."))] }), description: (literal!("Specifies which model variables are exposed by the modelDescription.xml")).clone() } });

pub static FMI_SOURCES: ConfigFlag = ConfigFlag { index: 137, name: literal!("fmiSources"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Defines if FMUs will be exported with sources or not. --fmiFilter=blackBox might override this, because black box FMUs do never contain their source code.") };

pub static FMI_FLAGS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 138, name: (literal!("fmiFlags")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Add simulation flags to FMU. Will create <fmiPrefix>_flags.json in resources folder with given flags. Use --fmiFlags or --fmiFlags=none to disable [default]. Use --fmiFlags=default for the default simulation flags. To pass flags use e.g. --fmiFlags=s:cvode,nls:homotopy or --fmiFlags=path/to/yourFlags.json.")).clone() } });

pub static FMU_CMAKE_BUILD: ConfigFlag = ConfigFlag { index: 139, name: literal!("fmuCMakeBuild"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Configured and build FMU with CMake if true.") };

pub static NEW_BACKEND: ConfigFlag = ConfigFlag { index: 140, name: literal!("newBackend"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Activates experimental new backend for better array handling. This also activates the new frontend. [WIP]") };

pub static PARMODAUTO: ConfigFlag = ConfigFlag { index: 141, name: literal!("parmodauto"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Experimental: Enable parallelization of independent systems of equations in the translated model. Only works on Linux systems.") };

pub static INTERACTIVE_PORT: ConfigFlag = ConfigFlag { index: 142, name: literal!("interactivePort"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: literal!("Sets the port used by the interactive server.") };

pub static ALLOW_NON_STANDARD_MODELICA: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 143, name: (literal!("allowNonStandardModelica")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("nonStdMultipleExternalDeclarations"), literal!("Allow several external declarations in functions.\nSee: https://specification.modelica.org/maint/3.5/functions.html#function-as-a-specialized-class")), (literal!("nonStdEnumerationAsIntegers"), literal!("Allow enumeration as integer without casting via Integer(Enum).\nSee: https://specification.modelica.org/maint/3.5/class-predefined-types-and-declarations.html#type-conversion-of-enumeration-values-to-string-or-integer")), (literal!("nonStdIntegersAsEnumeration"), literal!("Allow integer as enumeration without casting via Enum(Integer).\nSee: https://specification.modelica.org/maint/3.5/class-predefined-types-and-declarations.html#type-conversion-of-integer-to-enumeration-values")), (literal!("nonStdDifferentCaseFileVsClassName"), literal!("Allow directory or file with different case in the name than the contained class name.\nSee: https://specification.modelica.org/maint/3.5/packages.html#mapping-package-class-structures-to-a-hierarchical-file-system")), (literal!("nonStdTopLevelOuter"), literal!("Allow top level outer.\nSee: https://specification.modelica.org/maint/3.6/scoping-name-lookup-and-flattening.html#S4.p1")), (literal!("protectedAccess"), literal!("Allow access of protected elements")), (literal!("reinitInAlgorithms"), literal!("Allow reinit in algorithm sections")), (literal!("unbalancedModel"), literal!("Allow models to be locally unbalanced and to have unbalanced connectors")), (literal!("implicitParameterStartAttribute"), literal!("Allow fixed parameters with no binding or start attribute")), (literal!("initialSimplified"), literal!("Allow use of experimental operator `initialSimplified()`")), (literal!("illegalConditionalContext"), literal!("Allow use of components with false conditions in illegal contexts"))] }), description: (literal!("Flags to allow non-standard Modelica.")).clone() } });

pub static EXPORT_CLOCKS_IN_MODELDESCRIPTION: ConfigFlag = ConfigFlag { index: 144, name: literal!("exportClocksInModelDescription"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("exports clocks in modeldescription.xml for fmus, The default is false.") };

pub static LINK_TYPE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 145, name: (literal!("linkType")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::ENUM_FLAG { data: 1, validValues: list![(literal!("dynamic"), 1), (literal!("static"), 2)] }, validOptions: Some(ValidOptions::STRING_OPTION { options: list![(literal!("dynamic")).clone(), (literal!("static")).clone()] }), description: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sets the link type for the simulation executable.\n")); __mm_s.push_str(&*literal!("dynamic: libraries are dynamically linked; the executable is built very fast but is not portable because of DLL dependencies.\n")); __mm_s.push_str(&*literal!("static: libraries are statically linked; the executable is built more slowly but it is portable and dependency-free.\n")); ArcStr::from(__mm_s) }).clone() } });

pub static TEARING_ALWAYS_DERIVATIVES: ConfigFlag = ConfigFlag { index: 146, name: literal!("tearingAlwaysDer"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Always choose state derivatives as iteration variables in strong components.") };

pub static DUMP_FLAT_MODEL: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 147, name: (literal!("dumpFlatModel")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: list![(literal!("all")).clone()] }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("flatten"), literal!("After flattening but before connection handling.")), (literal!("connections"), literal!("After connection handling.")), (literal!("eval"), literal!("After evaluating constants.")), (literal!("simplify"), literal!("After model simplification.")), (literal!("scalarize"), literal!("After scalarizing arrays.")), (literal!("translateResidualsDAE"), literal!("Show the result of the translateResidualsDAE API."))] }), description: (literal!("Dumps the flat model at the given stages of the frontend.")).clone() } });

pub(crate) static SIMULATION: ConfigFlag = ConfigFlag { index: 148, name: literal!("simulation"), shortname: Some(literal!("u")), visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Simulates the last model in the given Modelica file.") };

pub static OBFUSCATE: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 149, name: (literal!("obfuscate")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("none")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("none"), literal!("No obfuscation.")), (literal!("encrypted"), literal!("Obfuscates protected variables in encrypted models")), (literal!("protected"), literal!("Obfuscates protected variables in all models.")), (literal!("full"), literal!("Obfuscates everything."))] }), description: (literal!("Obfuscates identifiers in the simulation model")).clone() } });

pub static FMU_RUNTIME_DEPENDS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 150, name: (literal!("fmuRuntimeDepends")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("default")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("default"), literal!("Depending on CMake version. If CMake version >= 3.21 use  \"modelica\", otherwise use \"none\"")), (literal!("none"), literal!("No runtime library dependencies are copied into the FMU.")), (literal!("modelica"), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All modelica runtime library dependencies are copied into the FMU.")); __mm_s.push_str(&*literal!("System librarys located in '/lib*', '/usr/lib*' and '/usr/local/lib*' are excluded.")); __mm_s.push_str(&*literal!("Needs --fmuCMakeBuild=true and CMake version >= 3.21.")); ArcStr::from(__mm_s) }), (literal!("all"), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All runtime library dependencies are copied into the FMU.")); __mm_s.push_str(&*literal!("System librarys are copied as well.")); __mm_s.push_str(&*literal!("Needs --fmuCMakeBuild=true and CMake version >= 3.21.")); ArcStr::from(__mm_s) })] }), description: (literal!("Defines if runtime library dependencies are included in the FMU. Only used when compiler flag fmuCMakeBuild=true.")).clone() } });

pub static FRONTEND_INLINE: ConfigFlag = ConfigFlag { index: 151, name: literal!("frontendInline"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Enables inlining of functions in the frontend.") };

pub static EXPOSE_LOCAL_IOS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 152, name: (literal!("exposeLocalIOs")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 0 }, validOptions: None, description: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Introduces top-level inputs/outputs for unconnected input/output connectors at requested levels, provided they are public, ")); __mm_s.push_str(&*literal!("0 meaning top-level (standard Modelica), 1 inputs/outputs of top-level components, >1 going deeper. ")); __mm_s.push_str(&*literal!("This flag is particularly useful for FMI export.")); ArcStr::from(__mm_s) }).clone() } });

pub static BASE_MODELICA_FORMAT: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 153, name: (literal!("baseModelicaFormat")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("scalarized"), literal!("Include subscripts in the quoted identifiers ('a[1].x[3]').")), (literal!("partiallyScalarized"), literal!("Include subscripts in the quoted identifiers, except for the last name ('a[1].x'[3]).")), (literal!("nonScalarized"), literal!("Don't include subscripts in the quoted identifiers ('a'[1].'x'[3]).")), (literal!("withRecords"), literal!("Keep records and don't expand them.")), (literal!("withoutRecords"), literal!("Expand records into separate components.")), (literal!("showConfidence"), literal!("Add comments that show confidence numbers for binding equations."))] }), description: (literal!("Formatting options for Base Modelica")).clone() } });

pub static BASE_MODELICA_OPTIONS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 154, name: (literal!("baseModelicaOptions")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("moveBindings"), literal!("Moves movable binding equations to normal equations.")), (literal!("scalarize"), literal!("Fully scalarize the Base Modelica model.")), (literal!("inlineFunctions"), literal!("Inline all functions."))] }), description: (literal!("Enables optional Base Modelica options.")).clone() } });

pub static DEBUG_FOLLOW_EQUATIONS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 155, name: (literal!("debugFollowEquations")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_LIST_FLAG { data: metamodelica::nil() }, validOptions: None, description: (literal!("Takes a list of equation names and prints the corresponding equations after each stage of the backend process.")).clone() } });

pub static MAX_SIZE_LINEARIZATION: ConfigFlag = ConfigFlag { index: 156, name: literal!("maxSizeLinearization"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::INT_FLAG { data: 1000 }, validOptions: None, description: literal!("Sets the maximum system size for which linearization code is generated.") };

pub static RESIZABLE_ARRAYS: ConfigFlag = ConfigFlag { index: 157, name: literal!("resizableArrays"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Assumes all arrays are resizable. Only works with the new backend --newBackend.") };

pub static EVALUATE_STRUCTURAL_PARAMETERS: std::sync::LazyLock<ConfigFlag> = std::sync::LazyLock::new(|| { ConfigFlag { index: 158, name: (literal!("evaluateStructuralParameters")).clone(), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: (literal!("all")).clone() }, validOptions: Some(ValidOptions::STRING_DESC_OPTION { options: list![(literal!("all"), literal!("Evaluates all structural parameters")), (literal!("strictlyNecessary"), literal!("Evaluates only structural parameters strictly required by the frontend"))] }), description: (literal!("Sets which structural parameters are evaluated by the frontend.")).clone() } });

pub static LOAD_MISSING_LIBRARIES: ConfigFlag = ConfigFlag { index: 159, name: literal!("loadMissingLibraries"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Automatically try to load a matching library if a name can't be found during name lookup.") };

pub static CAUSALIZE_DAE_MODE: ConfigFlag = ConfigFlag { index: 160, name: literal!("causalizeDaeMode"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("The system is partially causalized and simple assignments are generated for equations that can be solved explicitly. Only works with --daeMode.") };

/* please remove me once this is supported */
pub static SIM_CODE_SCALARIZE: ConfigFlag = ConfigFlag { index: 161, name: literal!("simCodeScalarize"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: true }, validOptions: None, description: literal!("Scalarizes variables during simcode phase.") };

pub static EXECUTE_COMMAND: ConfigFlag = ConfigFlag { index: 162, name: literal!("cmd"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Executes the string argument as a script before any other operation.") };

pub static MOO_DYNAMIC_OPTIMIZATION: ConfigFlag = ConfigFlag { index: 163, name: literal!("moo"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::BOOL_FLAG { data: false }, validOptions: None, description: literal!("Generate code for dynamic optimization library MOO.") };

pub static FMI_EXTRA_ANNOTATIONS: ConfigFlag = ConfigFlag { index: 164, name: literal!("fmiExtraAnnotations"), shortname: None, visibility: crate::Flags::FlagVisibility::EXTERNAL, defaultValue: FlagData::STRING_FLAG { data: literal!("") }, validOptions: None, description: literal!("Export annotations matching the given regex to extra/org.openmodelica/modelAnnotations.json.") };

pub fn getFlags(mut initialize: bool) -> Flag {
    let mut flags: Flag;
    flags = crate::Globals::flagsIndex.with(|__root| __root.borrow().clone());
    flags
}

pub fn isSet(mut inFlag: DebugFlag) -> Result<bool> {
    let mut outValue: bool;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut flags: Flag;
    let mut index: i32;
    let DebugFlag { index: __pa0, .. } = (inFlag) else { bail!("pattern mismatch") };
    index = __pa0.clone();
    flags = getFlags(true);
    let Flag::FLAGS { debugFlags: __pa1, .. } = (flags) else { bail!("pattern mismatch") };
    debug_flags = __pa1.clone();
    outValue = metamodelica::arrayGet(debug_flags.clone(), index)?;
    Ok(outValue)
}

pub fn isConfigFlagSet(mut inFlag: ConfigFlag, mut hasMember: ArcStr) -> Result<bool> {
    let mut isMember: bool;
    isMember = listMember((hasMember).clone(), getConfigStringList(inFlag)?);
    Ok(isMember)
}

pub fn getConfigName(mut inFlag: ConfigFlag) -> Result<ArcStr> {
    let mut name: ArcStr;
    let ConfigFlag { name: __pa0, .. } = (inFlag) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    Ok(name)
}

pub fn getConfigValue(mut inFlag: ConfigFlag) -> Result<FlagData> {
    let mut outValue: FlagData;
    let mut config_flags: metamodelica::Array<FlagData>;
    let mut index: i32;
    let mut flags: Flag;
    let mut name: ArcStr;
    let ConfigFlag { name: __pa0, index: __pa1, .. } = (inFlag) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    index = __pa1.clone();
    flags = getFlags(true);
    let Flag::FLAGS { configFlags: __pa2, .. } = (flags) else { bail!("pattern mismatch") };
    config_flags = __pa2.clone();
    outValue = metamodelica::arrayGet(config_flags.clone(), index)?;
    Ok(outValue)
}

pub fn getConfigBool(mut inFlag: ConfigFlag) -> Result<bool> {
    let mut outValue: bool;
    let FlagData::BOOL_FLAG { data: __pa0 } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigInt(mut inFlag: ConfigFlag) -> Result<i32> {
    let mut outValue: i32;
    let FlagData::INT_FLAG { data: __pa0 } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigIntList(mut inFlag: ConfigFlag) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outValue: Arc<metamodelica::List<i32>>;
    let FlagData::INT_LIST_FLAG { data: __pa0 } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigReal(mut inFlag: ConfigFlag) -> Result<metamodelica::Real> {
    let mut outValue: metamodelica::Real;
    let FlagData::REAL_FLAG { data: __pa0 } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigString(mut inFlag: ConfigFlag) -> Result<ArcStr> {
    let mut outValue: ArcStr;
    let FlagData::STRING_FLAG { data: __pa0 } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigStringList(mut inFlag: ConfigFlag) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outValue: Arc<metamodelica::List<ArcStr>>;
    let FlagData::STRING_LIST_FLAG { data: __pa0 } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn getConfigEnum(mut inFlag: ConfigFlag) -> Result<i32> {
    let mut outValue: i32;
    let FlagData::ENUM_FLAG { data: __pa0, .. } = (getConfigValue(inFlag)?) else { bail!("pattern mismatch") };
    outValue = __pa0.clone();
    Ok(outValue)
}

