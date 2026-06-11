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

use crate::ConnectUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an edge is a tuple with two component references
pub type Edge = (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>);

/// A list of edges
pub type Edges = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>;

/// a tuple with two crefs and dae elements for equatityConstraint function call
pub type DaeEdge = (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>);

/// A list of edges, each edge associated with two lists of DAE elements
/// (these elements represent equations to be added if the edge
/// is broken)
pub type DaeEdges = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>)>>;

/// root defined with Connection.root
pub type DefiniteRoot = Arc<DAE::ComponentRef>;

/// roots defined with Connection.root
pub type DefiniteRoots = Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;

/// roots defined with Connection.uniqueRoot
pub type UniqueRoots = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>;

/// potential root defined with Connections.potentialRoot
pub type PotentialRoot = (Arc<DAE::ComponentRef>, metamodelica::Real);

/// potential roots defined with Connections.potentialRoot
pub type PotentialRoots = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, metamodelica::Real)>>;

/// Input structure for connection breaking algorithm. It is collected during instantiation phase.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ConnectionGraph {
    pub updateGraph: bool,
    /// Roots defined with Connection.root
    pub definiteRoots: DefiniteRoots,
    /// Roots defined with Connection.potentialRoot
    pub potentialRoots: PotentialRoots,
    /// Roots defined with Connection.uniqueRoot
    pub uniqueRoots: UniqueRoots,
    /// Edges defined with Connection.branch
    pub branches: Edges,
    /// Edges defined with connect statement
    pub connections: DaeEdges,
}

impl metamodelica::gc::MMTrace for ConnectionGraph {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.updateGraph, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.definiteRoots, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.potentialRoots, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.uniqueRoots, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.branches, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.connections, __mmv)?;
        Ok(())
    }
}
impl Default for ConnectionGraph {
    fn default() -> Self {
        Self {
            updateGraph: Default::default(),
            definiteRoots: Default::default(),
            potentialRoots: Default::default(),
            uniqueRoots: Default::default(),
            branches: Default::default(),
            connections: Default::default(),
        }
    }
}

pub type GRAPH = ConnectionGraph;


thread_local! { static __EMPTY_TLS: ConnectionGraph = ConnectionGraph { updateGraph: true, definiteRoots: metamodelica::nil(), potentialRoots: metamodelica::nil(), uniqueRoots: metamodelica::nil(), branches: metamodelica::nil(), connections: metamodelica::nil() }; }
pub fn EMPTY() -> ConnectionGraph { __EMPTY_TLS.with(|__t| __t.clone()) }

thread_local! { static __NOUPDATE_EMPTY_TLS: ConnectionGraph = ConnectionGraph { updateGraph: false, definiteRoots: metamodelica::nil(), potentialRoots: metamodelica::nil(), uniqueRoots: metamodelica::nil(), branches: metamodelica::nil(), connections: metamodelica::nil() }; }
pub(crate) fn NOUPDATE_EMPTY() -> ConnectionGraph { __NOUPDATE_EMPTY_TLS.with(|__t| __t.clone()) }

pub(crate) fn handleOverconstrainedConnections(mut inGraph: ConnectionGraph, mut modelNameQualified: ArcStr, mut inDAE: DAE::DAElist) -> Result<(DAE::DAElist, DaeEdges, DaeEdges)> {
    let mut outDAE: DAE::DAElist;
    let mut outConnected: DaeEdges;
    let mut outBroken: DaeEdges;
    (outDAE, outConnected, outBroken) = 'mc: {
        let __mc_input = (inGraph, inDAE.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ConnectionGraph { updateGraph: _, definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil }, _) => {
                    Ok((inDAE.clone(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (graph, DAE::DAElist { elementLst: elts }) => {
                    let mut roots: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut broken: DaeEdges;
                    let mut connected: DaeEdges;
                    let mut elts = (*elts).clone();
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Summary: \n\t")); __mm_s.push_str(&*literal!("Nr Roots:           ")); __mm_s.push_str(&*intString((getDefiniteRoots(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Potential Roots: ")); __mm_s.push_str(&*intString((getPotentialRoots(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Unique Roots:    ")); __mm_s.push_str(&*intString((getUniqueRoots(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Branches:        ")); __mm_s.push_str(&*intString((getBranches(graph.clone())?.len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Connections:     ")); __mm_s.push_str(&*intString((getConnections(graph.clone())?.len() as i32))); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (roots, connected, broken) = findResultGraph(graph.clone(), (modelNameQualified.clone()).clone())?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(roots.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Broken connections: ")); __mm_s.push_str(&*stringDelimitList(List::map1(broken.clone(), (std::sync::Arc::new(printConnectionStr) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>), ArcStr) -> Result<ArcStr> + 'static>), (literal!("broken")).clone())?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowed connections: ")); __mm_s.push_str(&*stringDelimitList(List::map1(connected.clone(), (std::sync::Arc::new(printConnectionStr) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>), ArcStr) -> Result<ArcStr> + 'static>), (literal!("allowed")).clone())?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    elts = evalConnectionsOperators(roots.clone(), graph.clone(), elts.clone())?;
                    Ok((DAE::DAElist { elementLst: elts.clone() }, connected.clone(), broken.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::CGRAPH.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.handleOverconstrainedConnections failed for model: ")); __mm_s.push_str(&*modelNameQualified.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDAE, outConnected, outBroken))
}

pub(crate) fn addDefiniteRoot(mut inGraph: ConnectionGraph, mut inRoot: Arc<DAE::ComponentRef>) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((inGraph, inRoot)) {
        (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, root) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addDefiniteRoot(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(root.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            }
            ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: metamodelica::cons(root.clone(), definiteRoots.clone()), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn addPotentialRoot(mut inGraph: ConnectionGraph, mut inRoot: Arc<DAE::ComponentRef>, mut inPriority: metamodelica::Real) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((inGraph, inRoot, inPriority)) {
        (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, root, priority) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addPotentialRoot(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            }
            ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: metamodelica::cons((root.clone(), priority.clone()), potentialRoots.clone()), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn addUniqueRoots(mut inGraph: ConnectionGraph, mut inRoots: Arc<DAE::Exp>, mut inMessage: Arc<DAE::Exp>) -> Result<ConnectionGraph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inGraph.clone(), inRoots)) {
        (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, Deref @ DAE::Exp::CREF { componentRef: root, ty: _ }) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addUniqueRoots(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inMessage.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            }
            return Ok(ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: metamodelica::cons((root.clone(), inMessage), uniqueRoots.clone()), branches: branches.clone(), connections: connections.clone() })
        },
        (ConnectionGraph { .. }, Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: Deref @ metamodelica::List::Nil }) => {
            return Ok(inGraph)
        },
        (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, Deref @ DAE::Exp::ARRAY { ty, scalar, array: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: root, ty: _ }, tail: rest } }) => {
            let mut graph: ConnectionGraph;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addUniqueRoots(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inMessage.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            }
            graph = ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: metamodelica::cons((root.clone(), inMessage.clone()), uniqueRoots.clone()), branches: branches.clone(), connections: connections.clone() };
            { (inGraph, inRoots, inMessage) = (graph.clone(), Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: scalar.clone(), array: rest.clone() }), inMessage); continue '__tco; }
        },
        (_, _) => {
            return Ok(inGraph)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn addBranch(mut inGraph: ConnectionGraph, mut inRef1: Arc<DAE::ComponentRef>, mut inRef2: Arc<DAE::ComponentRef>) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((inGraph, inRef1, inRef2)) {
        (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, ref1, ref2) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addBranch(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ref2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            }
            ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: metamodelica::cons((ref1.clone(), ref2.clone()), branches.clone()), connections: connections.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn addConnection(mut inGraph: ConnectionGraph, mut inRef1: Arc<DAE::ComponentRef>, mut inRef2: Arc<DAE::ComponentRef>, mut inDae: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((inGraph, inRef1, inRef2, inDae)) {
        (ConnectionGraph { updateGraph, definiteRoots, potentialRoots, uniqueRoots, branches, connections }, ref1, ref2, dae) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.addConnection(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ref2.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: metamodelica::cons((ref1.clone(), ref2.clone(), dae.clone()), connections.clone()) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

// ************************************* //
// ********* protected section ********* //
// ************************************* //
fn canonical(mut inPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCanonical: Arc<DAE::ComponentRef>;
    outCanonical = 'mc: {
        let __mc_input = (inPartition, inRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, r#ref) => {
                    let mut parent: Arc<DAE::ComponentRef>;
                    let mut parentCanonical: Arc<DAE::ComponentRef>;
                    parent = BaseHashTable::get(r#ref.clone(), partition.clone())?;
                    parentCanonical = canonical(partition.clone(), parent.clone())?;
                    Ok(parentCanonical.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#ref) => {
                    Ok(r#ref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCanonical)
}

fn areInSameComponent(mut inPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inRef1: Arc<DAE::ComponentRef>, mut inRef2: Arc<DAE::ComponentRef>) -> bool {
    let mut outResult: bool;
    outResult = 'mc: {
        let __mc_input = (inPartition, inRef1, inRef2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let mut canon1: Arc<DAE::ComponentRef>;
                    let mut canon2: Arc<DAE::ComponentRef>;
                    canon1 = canonical(partition.clone(), ref1.clone())?;
                    canon2 = canonical(partition.clone(), ref2.clone())?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(canon1.clone(), canon2.clone())?) else { bail!("pattern mismatch") };
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
    outResult
}

fn connectBranchComponents(mut inPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inRef1: Arc<DAE::ComponentRef>, mut inRef2: Arc<DAE::ComponentRef>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    let mut outPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    outPartition = 'mc: {
        let __mc_input = (inPartition, inRef1, inRef2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let mut canon1: Arc<DAE::ComponentRef>;
                    let mut canon2: Arc<DAE::ComponentRef>;
                    let mut partition = (*partition).clone();
                    canon1 = canonical(partition.clone(), ref1.clone())?;
                    canon2 = canonical(partition.clone(), ref2.clone())?;
                    let (__pa0, true) = (connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone())?) else { bail!("pattern mismatch") };
                    partition = __pa0.clone();
                    Ok(partition.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, _, _) => {
                    Ok(partition.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPartition)
}

fn connectComponents(mut inPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inDaeEdge: DaeEdge) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), DaeEdges, DaeEdges)> {
    let mut outPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut outConnectedConnections: DaeEdges;
    let mut outBrokenConnections: DaeEdges;
    (outPartition, outConnectedConnections, outBrokenConnections) = 'mc: {
        let __mc_input = (inPartition, inDaeEdge.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (ref1, _, _)) => {
                    if '__try0: {
                        unwrap_break_err!(canonical(partition.clone(), ref1.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((partition.clone(), list![inDaeEdge.clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (_, ref2, _)) => {
                    if '__try0: {
                        unwrap_break_err!(canonical(partition.clone(), ref2.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((partition.clone(), list![inDaeEdge.clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (ref1, ref2, _)) => {
                    let mut canon1: Arc<DAE::ComponentRef>;
                    let mut canon2: Arc<DAE::ComponentRef>;
                    let mut partition = (*partition).clone();
                    canon1 = canonical(partition.clone(), ref1.clone())?;
                    canon2 = canonical(partition.clone(), ref2.clone())?;
                    let (__pa0, true) = (connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone())?) else { bail!("pattern mismatch") };
                    partition = __pa0.clone();
                    Ok((partition.clone(), list![inDaeEdge.clone()], metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, (ref1, ref2, _)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.connectComponents: should remove equations generated from: connect(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ref2.clone())?); __mm_s.push_str(&*literal!(") and add {0, ..., 0} = equalityConstraint(cr1, cr2) instead.\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((partition.clone(), metamodelica::nil(), list![inDaeEdge.clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outPartition, outConnectedConnections, outBrokenConnections))
}

fn connectCanonicalComponents(mut inPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inRef1: Arc<DAE::ComponentRef>, mut inRef2: Arc<DAE::ComponentRef>) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)> {
    let mut outPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut outReallyConnected: bool;
    (outPartition, outReallyConnected) = 'mc: {
        let __mc_input = (inPartition, inRef1, inRef2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(ref1.clone(), ref2.clone())?) else { bail!("pattern mismatch") };
                    Ok((partition.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (partition, ref1, ref2) => {
                    let mut partition = (*partition).clone();
                    partition = BaseHashTable::add((ref1.clone(), ref2.clone()), partition.clone())?;
                    Ok((partition.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outPartition, outReallyConnected))
}

fn addRootsToTable(mut inTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inRoots: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inFirstRoot: Arc<DAE::ComponentRef>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTable, inRoots, inFirstRoot)) {
        (table, Deref @ metamodelica::List::Cons { head: root, tail: tail }, firstRoot) => {
            let mut table = (*table).clone();
            table = BaseHashTable::add((root.clone(), firstRoot.clone()), table.clone())?;
            { (inTable, inRoots, inFirstRoot) = (table.clone(), tail.clone(), firstRoot.clone()); continue '__tco; }
        },
        (table, Deref @ metamodelica::List::Nil, _) => {
            return Ok(table.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn resultGraphWithRoots(mut roots: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    let mut outTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut table0: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut dummyRoot: Arc<DAE::ComponentRef>;
    dummyRoot = ComponentReferenceBasics::makeCrefIdent((literal!("__DUMMY_ROOT")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    table0 = HashTableCG::emptyHashTable();
    outTable = addRootsToTable(table0, roots, dummyRoot)?;
    Ok(outTable)
}

fn addBranchesToTable(mut inTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inBranches: Edges) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTable, inBranches)) {
        (table, Deref @ metamodelica::List::Cons { head: (ref1, ref2), tail: tail }) => {
            let mut table1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
            let mut table2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
            table1 = connectBranchComponents(table.clone(), ref1.clone(), ref2.clone())?;
            { (inTable, inBranches) = (table1.clone(), tail.clone()); continue '__tco; }
        },
        (table, Deref @ metamodelica::List::Nil) => {
            return Ok(table.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn ord(mut inEl1: PotentialRoot, mut inEl2: PotentialRoot) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (inEl1, inEl2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((c1, r1), (c2, r2)) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let true = (realEq(r1.clone(), r2.clone())) else { bail!("pattern mismatch") };
                    s1 = (ComponentReferenceBasics::printComponentRefStr(c1.clone())?).clone();
                    s2 = (ComponentReferenceBasics::printComponentRefStr(c2.clone())?).clone();
                    let 1 = (stringCompare((s1.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((_, r1), (_, r2)) => {
                    Ok(r1.clone() > r2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

fn addPotentialRootsToTable(mut inTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inPotentialRoots: PotentialRoots, mut inRoots: DefiniteRoots, mut inFirstRoot: Arc<DAE::ComponentRef>) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), DefiniteRoots)> {
    let mut outTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut outRoots: DefiniteRoots;
    (outTable, outRoots) = 'mc: {
        let __mc_input = (inTable, inPotentialRoots, inRoots, inFirstRoot);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (table, Deref @ metamodelica::List::Nil, roots, _) => {
                    Ok((table.clone(), roots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (table, Deref @ metamodelica::List::Cons { head: (potentialRoot, _), tail: tail }, roots, firstRoot) => {
                    let mut canon1: Arc<DAE::ComponentRef>;
                    let mut canon2: Arc<DAE::ComponentRef>;
                    let mut finalRoots: DefiniteRoots;
                    let mut table = (*table).clone();
                    canon1 = canonical(table.clone(), potentialRoot.clone())?;
                    canon2 = canonical(table.clone(), firstRoot.clone())?;
                    let (__pa0, true) = (connectCanonicalComponents(table.clone(), canon1.clone(), canon2.clone())?) else { bail!("pattern mismatch") };
                    table = __pa0.clone();
                    (table, finalRoots) = addPotentialRootsToTable(table.clone(), tail.clone(), metamodelica::cons(potentialRoot.clone(), roots.clone()), firstRoot.clone())?;
                    Ok((table.clone(), finalRoots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (table, Deref @ metamodelica::List::Cons { head: _, tail: tail }, roots, firstRoot) => {
                    let mut finalRoots: DefiniteRoots;
                    let mut table = (*table).clone();
                    (table, finalRoots) = addPotentialRootsToTable(table.clone(), tail.clone(), roots.clone(), firstRoot.clone())?;
                    Ok((table.clone(), finalRoots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTable, outRoots))
}

fn addConnections(mut inTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inConnections: DaeEdges) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), DaeEdges, DaeEdges)> {
    let mut outTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut outConnectedConnections: DaeEdges;
    let mut outBrokenConnections: DaeEdges;
    (outTable, outConnectedConnections, outBrokenConnections) = (::match_deref::match_deref! { match &((inTable, inConnections)) {
        (table, Deref @ metamodelica::List::Nil) => {
            (table.clone(), metamodelica::nil(), metamodelica::nil())
        },
        (table, Deref @ metamodelica::List::Cons { head: e, tail: tail }) => {
            let mut broken1: DaeEdges;
            let mut broken2: DaeEdges;
            let mut broken: DaeEdges;
            let mut connected1: DaeEdges;
            let mut connected2: DaeEdges;
            let mut connected: DaeEdges;
            let mut table = (*table).clone();
            (table, connected1, broken1) = connectComponents(table.clone(), e.clone())?;
            (table, connected2, broken2) = addConnections(table.clone(), tail.clone())?;
            connected = listAppend(connected1.clone(), connected2.clone());
            broken = listAppend(broken1.clone(), broken2.clone());
            (table.clone(), connected.clone(), broken.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outTable, outConnectedConnections, outBrokenConnections))
}

fn findResultGraph(mut inGraph: ConnectionGraph, mut modelNameQualified: ArcStr) -> Result<(DefiniteRoots, DaeEdges, DaeEdges)> {
    let mut outRoots: DefiniteRoots;
    let mut outConnectedConnections: DaeEdges;
    let mut outBrokenConnections: DaeEdges;
    (outRoots, outConnectedConnections, outBrokenConnections) = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ConnectionGraph { definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, .. } => {
                    Ok((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ConnectionGraph { definiteRoots, potentialRoots, uniqueRoots, branches, connections, .. } => {
                    let mut finalRoots: DefiniteRoots;
                    let mut orderedPotentialRoots: PotentialRoots;
                    let mut broken: DaeEdges;
                    let mut connected: DaeEdges;
                    let mut table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut dummyRoot: Arc<DAE::ComponentRef>;
                    let mut brokenConnectsViaGraphViz: ArcStr;
                    let mut userBrokenLst: Arc<metamodelica::List<ArcStr>>;
                    let mut userBrokenLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
                    let mut userBrokenTplLst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
                    let mut connections = (*connections).clone();
                    connections = connections.clone().reverse();
                    table = resultGraphWithRoots(definiteRoots.clone())?;
                    table = addBranchesToTable(table.clone(), branches.clone())?;
                    orderedPotentialRoots = List::sort(potentialRoots.clone(), (std::sync::Arc::new(fnptr!(ord, (Arc<DAE::ComponentRef>, metamodelica::Real), (Arc<DAE::ComponentRef>, metamodelica::Real))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, metamodelica::Real), (Arc<DAE::ComponentRef>, metamodelica::Real)) -> Result<bool> + 'static>))?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Ordered Potential Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(orderedPotentialRoots.clone(), (std::sync::Arc::new(printPotentialRootTuple) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, metamodelica::Real)) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (table, connected, broken) = addConnections(table.clone(), connections.clone())?;
                    dummyRoot = ComponentReferenceBasics::makeCrefIdent((literal!("__DUMMY_ROOT")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
                    (table, finalRoots) = addPotentialRootsToTable(table.clone(), orderedPotentialRoots.clone(), definiteRoots.clone(), dummyRoot.clone())?;
                    brokenConnectsViaGraphViz = (generateGraphViz((modelNameQualified.clone()).clone(), definiteRoots.clone(), potentialRoots.clone(), uniqueRoots.clone(), branches.clone(), connections.clone(), finalRoots.clone(), broken.clone())?).clone();
                    if stringEq((brokenConnectsViaGraphViz.clone()).clone(), (literal!("")).clone()) {
                    } else {
                        userBrokenLst = Util::stringSplitAtChar((brokenConnectsViaGraphViz.clone()).clone(), (literal!("#")).clone())?;
                        userBrokenLstLst = List::map1(userBrokenLst.clone(), (std::sync::Arc::new(Util::stringSplitAtChar) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), (literal!("|")).clone())?;
                        userBrokenTplLst = makeTuple(userBrokenLstLst.clone())?;
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("User selected the following connect edges for breaking:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(userBrokenTplLst.clone(), (std::sync::Arc::new(fnptr!(printTupleStr, (ArcStr, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, ArcStr)) -> Result<ArcStr> + 'static>))?, (literal!("\n\t")).clone())); ArcStr::from(__mm_s) }).clone())?;
                        printDaeEdges(connections.clone())?;
                        connections = orderConnectsGuidedByUser(connections.clone(), userBrokenTplLst.clone())?;
                        connections = connections.clone().reverse();
                        metamodelica::print((literal!("\nAfer ordering:\n")).clone());
                        (finalRoots, connected, broken) = findResultGraph(ConnectionGraph { updateGraph: false, definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }, (modelNameQualified.clone()).clone())?;
                    }
                    Ok((finalRoots.clone(), connected.clone(), broken.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRoots, outConnectedConnections, outBrokenConnections))
}

fn orderConnectsGuidedByUser(mut inConnections: DaeEdges, mut inUserSelectedBreaking: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<DaeEdges> {
    let mut outOrderedConnections: DaeEdges;
    let mut front: DaeEdges = metamodelica::nil();
    let mut back: DaeEdges = metamodelica::nil();
    let mut c1: Arc<DAE::ComponentRef>;
    let mut c2: Arc<DAE::ComponentRef>;
    let mut sc1: ArcStr;
    let mut sc2: ArcStr;
    for mut e in &*inConnections {
        let mut e = e.clone();
        (c1, c2, _) = e.clone();
        sc1 = (ComponentReferenceBasics::printComponentRefStr(c1.clone())?).clone();
        sc2 = (ComponentReferenceBasics::printComponentRefStr(c2.clone())?).clone();
        if listMember((sc1.clone(), sc2.clone()), inUserSelectedBreaking.clone()) || listMember((sc2.clone(), sc1.clone()), inUserSelectedBreaking.clone()) {
            back = metamodelica::cons(e.clone(), back.clone());
        } else {
            front = metamodelica::cons(e.clone(), front.clone());
        }
    }
    outOrderedConnections = List::append_reverse(front, back);
    Ok(outOrderedConnections)
}

fn printTupleStr(mut inTpl: (ArcStr, ArcStr)) -> ArcStr {
    let mut out: ArcStr;
    out = ((match inTpl {
        (mut c1, mut c2) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*c1.clone()); __mm_s.push_str(&*literal!(" -- ")); __mm_s.push_str(&*c2.clone()); ArcStr::from(__mm_s) }
        },
    })).clone();
    out
}

fn makeTuple(mut inLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<Arc<metamodelica::List<(ArcStr, ArcStr)>>> {
    let mut outLst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
    outLst = 'mc: {
        let __mc_input = inLstLst;
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
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: c1, tail: Deref @ metamodelica::List::Cons { head: c2, tail: Deref @ metamodelica::List::Nil } }, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
                    lst = makeTuple(rest.clone())?;
                    Ok(metamodelica::cons((c1.clone(), c2.clone()), lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: Deref @ "", tail: Deref @ metamodelica::List::Nil }, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
                    lst = makeTuple(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
                    lst = makeTuple(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: bad, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following output from GraphViz OpenModelica assistant cannot be parsed:")); __mm_s.push_str(&*stringDelimitList(bad.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\nExpected format from GrapViz: cref1|cref2#cref3|cref4#. Ignoring malformed input.")); ArcStr::from(__mm_s) }).clone())?;
                    lst = makeTuple(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLst)
}

fn printPotentialRootTuple(mut potentialRoot: PotentialRoot) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(potentialRoot) {
        (cr, priority) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

fn setRootDistance(mut finalRoots: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>)), mut distance: i32, mut nextLevel: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut irooted: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))> {
    let mut orooted: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    orooted = 'mc: {
        let __mc_input = (finalRoots, nextLevel.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(irooted.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(setRootDistance(nextLevel.clone(), table.clone(), distance + 1, metamodelica::nil(), irooted.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _) => {
                    let mut rooted: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut next: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let false = (BaseHashTable::hasKey(cr.clone(), irooted.clone())?) else { bail!("pattern mismatch") };
                    rooted = BaseHashTable::add((cr.clone(), distance), irooted.clone())?;
                    next = BaseHashTable::get(cr.clone(), table.clone())?;
                    next = listAppend(nextLevel.clone(), next.clone());
                    Ok(setRootDistance(rest.clone(), table.clone(), distance, next.clone(), rooted.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _) => {
                    let mut rooted: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let false = (BaseHashTable::hasKey(cr.clone(), irooted.clone())?) else { bail!("pattern mismatch") };
                    rooted = BaseHashTable::add((cr.clone(), distance), irooted.clone())?;
                    Ok(setRootDistance(rest.clone(), table.clone(), distance, nextLevel.clone(), rooted.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    Ok(setRootDistance(rest.clone(), table.clone(), distance, nextLevel.clone(), irooted.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(orooted)
}

fn addBranches(mut edge: Edge, mut itable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))> {
    let mut otable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let mut cref1: Arc<DAE::ComponentRef>;
    let mut cref2: Arc<DAE::ComponentRef>;
    (cref1, cref2) = edge;
    otable = addConnectionRooted(cref1.clone(), cref2.clone(), itable)?;
    otable = addConnectionRooted(cref2, cref1, otable)?;
    Ok(otable)
}

fn addConnectionsRooted(mut connection: DaeEdge, mut itable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))> {
    let mut otable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    let mut cref1: Arc<DAE::ComponentRef>;
    let mut cref2: Arc<DAE::ComponentRef>;
    (cref1, cref2, _) = connection;
    otable = addConnectionRooted(cref1.clone(), cref2.clone(), itable)?;
    otable = addConnectionRooted(cref2, cref1, otable)?;
    Ok(otable)
}

fn addConnectionRooted(mut cref1: Arc<DAE::ComponentRef>, mut cref2: Arc<DAE::ComponentRef>, mut itable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))> {
    let mut otable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
    otable = (match itable.clone() {
        _ => {
            let mut table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crefs = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(BaseHashTable::get(cref1.clone(), itable.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
            table = BaseHashTable::add((cref1, metamodelica::cons(cref2, crefs.clone())), itable)?;
            table.clone()
        },
    });
    Ok(otable)
}

fn evalConnectionsOperators(mut inRoots: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut graph: ConnectionGraph, mut inDae: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outDae: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outDae = 'mc: {
        let __mc_input = inDae.clone();
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
                _ => {
                    let mut rooted: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut table: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    let mut branches: Edges;
                    let mut connections: DaeEdges;
                    let mut outDae: Arc<metamodelica::List<Arc<DAE::Element>>> = outDae.clone();
                    table = HashTable3::emptyHashTable();
                    branches = getBranches(graph.clone())?;
                    table = List::fold(branches.clone(), (std::sync::Arc::new(addBranches) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))> + 'static>), table.clone())?;
                    connections = getConnections(graph.clone())?;
                    table = List::fold(connections.clone(), (std::sync::Arc::new(addConnectionsRooted) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>))> + 'static>), table.clone())?;
                    rooted = setRootDistance(inRoots.clone(), table.clone(), 0, metamodelica::nil(), HashTable::emptyHashTable())?;
                    (outDae, _) = DAEUtil::traverseDAEElementList(inDae.clone(), (std::sync::Arc::new(fnptr!(evalConnectionsOperatorsHelper, Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, ConnectionGraph))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, ConnectionGraph)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, ConnectionGraph))> + 'static>), (rooted.clone(), inRoots.clone(), graph.clone()))?;
                    Ok((outDae.clone(), outDae.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outDae = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

fn evalConnectionsOperatorsHelper(mut inExp: Arc<DAE::Exp>, mut inRoots: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, ConnectionGraph)) -> (Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, ConnectionGraph)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outRoots: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, ConnectionGraph);
    (outExp, outRoots) = 'mc: {
        let __mc_input = (inExp.clone(), inRoots.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (rooted, roots, graph)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = false")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: false }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (rooted, roots, graph)) => {
                    let mut cref1: Arc<DAE::ComponentRef>;
                    let mut result: bool;
                    let mut branches: Edges;
                    branches = getBranches(graph.clone())?;
                    cref1 = getEdge(cref.clone(), branches.clone())?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: Found Branche Partner ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref1.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    result = getRooted(cref.clone(), cref1.clone(), rooted.clone());
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: result.clone() }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (rooted, roots @ Deref @ metamodelica::List::Nil, graph)) => {
                    Ok((exp.clone(), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (rooted, roots, graph)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = false")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: false }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, (rooted, roots, graph)) => {
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = false")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: false }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (rooted, roots, graph)) => {
                    let mut result: bool;
                    result = List::isMemberOnTrue(cref.clone(), roots.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: result.clone() }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: Deref @ metamodelica::List::Nil }, .. } }, (rooted, roots, graph)) => {
                    let mut result: bool;
                    result = List::isMemberOnTrue(cref.clone(), roots.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    result = boolNot(result.clone());
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: result.clone() }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "uniqueRootIndices" } }, expLst: Deref @ metamodelica::List::Cons { head: uroots @ Deref @ DAE::Exp::ARRAY { array: lst, .. }, tail: Deref @ metamodelica::List::Cons { head: nodes, tail: Deref @ metamodelica::List::Cons { head: message, tail: Deref @ metamodelica::List::Nil } } }, .. }, (rooted, roots, graph)) => {
                    let mut lst = (*lst).clone();
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.evalConnectionsOperatorsHelper: Connections.uniqueRootsIndicies(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(uroots.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ExpressionBasics::printExpStr(nodes.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ExpressionBasics::printExpStr(message.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    lst = List::fill(Arc::new(DAE::Exp::ICONST { integer: 1 }), (lst.clone().len() as i32));
                    Ok((Arc::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: lst.clone() }), (rooted.clone(), roots.clone(), graph.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inRoots.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outRoots)
}

fn getRooted(mut cref1: Arc<DAE::ComponentRef>, mut cref2: Arc<DAE::ComponentRef>, mut rooted: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))) -> bool {
    let mut result: bool;
    result = 'mc: {
        let __mc_input = rooted.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut i1: i32;
            let mut i2: i32;
            i1 = BaseHashTable::get(cref1.clone(), rooted.clone())?;
            i2 = BaseHashTable::get(cref2.clone(), rooted.clone())?;
            Ok(intLt(i1.clone(), i2.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    result
}

fn getEdge(mut cr: Arc<DAE::ComponentRef>, mut edges: Edges) -> Result<Arc<DAE::ComponentRef>> {
    let mut ocr: Arc<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = edges;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (cref1, cref2), tail: _ } => {
                    let mut cref1 = (*cref1).clone();
                    cref1 = getEdge1(cr.clone(), cref1.clone(), cref2.clone())?;
                    Ok(cref1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getEdge(cr.clone(), rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ocr)
}

fn getEdge1(mut cr: Arc<DAE::ComponentRef>, mut cref1: Arc<DAE::ComponentRef>, mut cref2: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut ocr: Arc<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = cref2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cref1.clone())?) else { bail!("pattern mismatch") };
                    Ok(cref2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cref2.clone())?) else { bail!("pattern mismatch") };
                    Ok(cref1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ocr)
}

fn printConnectionStr(mut connectTuple: DaeEdge, mut ty: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(connectTuple) {
        (c1, c2, _) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ty); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

fn printEdges(mut inEdges: Edges) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inEdges) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (c1, c2), tail: tail } => {
            metamodelica::print((literal!("    ")).clone());
            metamodelica::print((ComponentReferenceBasics::printComponentRefStr(c1.clone())?).clone());
            metamodelica::print((literal!(" -- ")).clone());
            metamodelica::print((ComponentReferenceBasics::printComponentRefStr(c2.clone())?).clone());
            metamodelica::print((literal!("\n")).clone());
            printEdges(tail.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printDaeEdges(mut inEdges: DaeEdges) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inEdges) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (c1, c2, _), tail: tail } => {
            metamodelica::print((literal!("    ")).clone());
            metamodelica::print((ComponentReferenceBasics::printComponentRefStr(c1.clone())?).clone());
            metamodelica::print((literal!(" -- ")).clone());
            metamodelica::print((ComponentReferenceBasics::printComponentRefStr(c2.clone())?).clone());
            metamodelica::print((literal!("\n")).clone());
            printDaeEdges(tail.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printConnectionGraph(mut inGraph: ConnectionGraph) -> Result<()> {
    let () = (match inGraph {
        ConnectionGraph { connections: mut connections, branches: mut branches, .. } => {
            metamodelica::print((literal!("Connections:\n")).clone());
            printDaeEdges(connections.clone())?;
            metamodelica::print((literal!("Branches:\n")).clone());
            printEdges(branches.clone())?;
            ()
        },
    });
    Ok(())
}

fn getDefiniteRoots(mut inGraph: ConnectionGraph) -> Result<DefiniteRoots> {
    let mut outResult: DefiniteRoots;
    outResult = (match inGraph {
        ConnectionGraph { definiteRoots: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getUniqueRoots(mut inGraph: ConnectionGraph) -> Result<UniqueRoots> {
    let mut outResult: UniqueRoots;
    outResult = (match inGraph {
        ConnectionGraph { uniqueRoots: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getPotentialRoots(mut inGraph: ConnectionGraph) -> Result<PotentialRoots> {
    let mut outResult: PotentialRoots;
    outResult = (match inGraph {
        ConnectionGraph { potentialRoots: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getBranches(mut inGraph: ConnectionGraph) -> Result<Edges> {
    let mut outResult: Edges;
    outResult = (match inGraph {
        ConnectionGraph { branches: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getConnections(mut inGraph: ConnectionGraph) -> Result<DaeEdges> {
    let mut outResult: DaeEdges;
    outResult = (match inGraph {
        ConnectionGraph { connections: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

pub(crate) fn merge(mut inGraph1: ConnectionGraph, mut inGraph2: ConnectionGraph) -> Result<ConnectionGraph> {
    let mut outGraph: ConnectionGraph;
    outGraph = 'mc: {
        let __mc_input = (inGraph1.clone(), inGraph2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ConnectionGraph { definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, .. }) => {
                    Ok(inGraph1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ConnectionGraph { definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, .. }, _) => {
                    Ok(inGraph2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (inGraph1.clone() == inGraph2.clone()) else { bail!("pattern mismatch") };
                    Ok(inGraph1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ConnectionGraph { updateGraph: updateGraph1, definiteRoots: definiteRoots1, potentialRoots: potentialRoots1, uniqueRoots: uniqueRoots1, branches: branches1, connections: connections1 }, ConnectionGraph { updateGraph: updateGraph2, definiteRoots: definiteRoots2, potentialRoots: potentialRoots2, uniqueRoots: uniqueRoots2, branches: branches2, connections: connections2 }) => {
                    let mut updateGraph: bool;
                    let mut definiteRoots: DefiniteRoots;
                    let mut uniqueRoots: UniqueRoots;
                    let mut potentialRoots: PotentialRoots;
                    let mut branches: Edges;
                    let mut connections: DaeEdges;
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::trace((literal!("- ConnectionGraph.merge()\n")).clone())?;
                    }
                    updateGraph = boolOr(updateGraph1.clone(), updateGraph2.clone());
                    definiteRoots = List::union(definiteRoots1.clone(), definiteRoots2.clone());
                    potentialRoots = List::union(potentialRoots1.clone(), potentialRoots2.clone());
                    uniqueRoots = List::union(uniqueRoots1.clone(), uniqueRoots2.clone());
                    branches = List::union(branches1.clone(), branches2.clone());
                    connections = List::union(connections1.clone(), connections2.clone());
                    Ok(ConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

/* **********************************************************************************************************************/
/* ****************************************** GraphViz generation *******************************************************/
/* **********************************************************************************************************************/
fn graphVizEdge(mut inEdge: Edge) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = ((::match_deref::match_deref! { match &(inEdge) {
        (c1, c2) => {
            let mut strEdge: ArcStr;
            strEdge = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!("\" -- \"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c2.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [color = blue, dir = \"none\", fontcolor=blue, label = \"branch\"];\n\t")); ArcStr::from(__mm_s) }).clone();
            strEdge.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn graphVizDaeEdge(mut inDaeEdge: DaeEdge, mut inBrokenDaeEdges: DaeEdges) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = ((::match_deref::match_deref! { match &(inDaeEdge.clone()) {
        (c1, c2, _) => {
            let mut sc1: ArcStr;
            let mut sc2: ArcStr;
            let mut strDaeEdge: ArcStr;
            let mut label: ArcStr;
            let mut labelFontSize: ArcStr;
            let mut decorate: ArcStr;
            let mut color: ArcStr;
            let mut style: ArcStr;
            let mut fontColor: ArcStr;
            let mut isBroken: bool;
            isBroken = listMember(inDaeEdge, inBrokenDaeEdges);
            label = (if (isBroken.clone()) {literal!("[[broken connect]]")} else {literal!("connect")}).clone();
            color = (if (isBroken.clone()) {literal!("red")} else {literal!("green")}).clone();
            style = (if (isBroken.clone()) {literal!("\"bold, dashed\"")} else {literal!("solid")}).clone();
            decorate = (boolString(isBroken.clone())).clone();
            fontColor = (if (isBroken.clone()) {literal!("red")} else {literal!("green")}).clone();
            labelFontSize = (if (isBroken.clone()) {literal!("labelfontsize = 20.0, ")} else {literal!("")}).clone();
            sc1 = (ComponentReferenceBasics::printComponentRefStr(c1.clone())?).clone();
            sc2 = (ComponentReferenceBasics::printComponentRefStr(c2.clone())?).clone();
            strDaeEdge = stringAppendList(list![(literal!("\"")).clone(), (sc1.clone()).clone(), (literal!("\" -- \"")).clone(), (sc2.clone()).clone(), (literal!("\" [")).clone(), (literal!("dir = \"none\", ")).clone(), (literal!("style = ")).clone(), (style.clone()).clone(), (literal!(", ")).clone(), (literal!("decorate = ")).clone(), (decorate.clone()).clone(), (literal!(", ")).clone(), (literal!("color = ")).clone(), (color.clone()).clone(), (literal!(", ")).clone(), (labelFontSize.clone()).clone(), (literal!("fontcolor = ")).clone(), (fontColor.clone()).clone(), (literal!(", ")).clone(), (literal!("label = \"")).clone(), (label.clone()).clone(), (literal!("\"")).clone(), (literal!("];\n\t")).clone()]);
            strDaeEdge.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn graphVizDefiniteRoot(mut inDefiniteRoot: DefiniteRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = ((::match_deref::match_deref! { match &(inDefiniteRoot) {
        c => {
            let mut strDefiniteRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = listMember(c.clone(), inFinalRoots);
            strDefiniteRoot = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = red, rank = \"source\", label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c.clone())?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot.clone()) {literal!("shape=polygon, sides=8, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) }).clone();
            strDefiniteRoot.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn graphVizPotentialRoot(mut inPotentialRoot: PotentialRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = ((::match_deref::match_deref! { match &(inPotentialRoot) {
        (c, priority) => {
            let mut strPotentialRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = listMember(c.clone(), inFinalRoots);
            strPotentialRoot = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = orangered, rank = \"min\" label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c.clone())?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot.clone()) {literal!("shape=ploygon, sides=7, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) }).clone();
            strPotentialRoot.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn generateGraphViz(mut modelNameQualified: ArcStr, mut definiteRoots: DefiniteRoots, mut potentialRoots: PotentialRoots, mut uniqueRoots: UniqueRoots, mut branches: Edges, mut connections: DaeEdges, mut finalRoots: DefiniteRoots, mut broken: DaeEdges) -> Result<ArcStr> {
    let mut brokenConnectsViaGraphViz: ArcStr;
    brokenConnectsViaGraphViz = ('mc: {
        let __mc_input = broken.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (boolOr(Flags::isSet(Flags::CGRAPH_GRAPHVIZ_FILE.clone())?, Flags::isSet(Flags::CGRAPH_GRAPHVIZ_SHOW.clone())?)) else { bail!("pattern mismatch") };
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut fileName: ArcStr;
                    let mut i: ArcStr;
                    let mut nrDR: ArcStr;
                    let mut nrPR: ArcStr;
                    let mut nrUR: ArcStr;
                    let mut nrBR: ArcStr;
                    let mut nrCO: ArcStr;
                    let mut nrFR: ArcStr;
                    let mut nrBC: ArcStr;
                    let mut timeStr: ArcStr;
                    let mut infoNodeStr: ArcStr;
                    let mut brokenConnects: ArcStr;
                    let mut tStart: metamodelica::Real;
                    let mut tEnd: metamodelica::Real;
                    let mut t: metamodelica::Real;
                    let mut graphVizStream: IOStream::IOStream;
                    let mut infoNode: Arc<metamodelica::List<ArcStr>>;
                    tStart = clock();
                    i = (literal!("\t")).clone();
                    fileName = (stringAppend((modelNameQualified.clone()).clone(), (literal!(".gv")).clone())).clone();
                    graphVizStream = IOStream::create((fileName.clone()).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
                    nrDR = (intString((definiteRoots.clone().len() as i32))).clone();
                    nrPR = (intString((potentialRoots.clone().len() as i32))).clone();
                    nrUR = (intString((uniqueRoots.clone().len() as i32))).clone();
                    nrBR = (intString((branches.clone().len() as i32))).clone();
                    nrCO = (intString((connections.clone().len() as i32))).clone();
                    nrFR = (intString((finalRoots.clone().len() as i32))).clone();
                    nrBC = (intString((broken.clone().len() as i32))).clone();
                    infoNode = list![(literal!("// Generated by OpenModelica. \n")).clone(), (literal!("// Overconstrained connection graph for model: \n//    ")).clone(), (modelNameQualified.clone()).clone(), (literal!("\n")).clone(), (literal!("// \n")).clone(), (literal!("// Summary: \n")).clone(), (literal!("//   Roots:                      ")).clone(), (nrDR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Potential Roots:    ")).clone(), (nrPR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Unique Roots:       ")).clone(), (nrUR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Branches:           ")).clone(), (nrBR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Connections:        ")).clone(), (nrCO.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Final Roots:        ")).clone(), (nrFR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Broken Connections: ")).clone(), (nrBC.clone()).clone(), (literal!("\n")).clone()];
                    infoNodeStr = stringAppendList(infoNode.clone());
                    infoNodeStr = (System::stringReplace((infoNodeStr.clone()).clone(), (literal!("\n")).clone(), (literal!("\\l")).clone())?).clone();
                    infoNodeStr = (System::stringReplace((infoNodeStr.clone()).clone(), (literal!("\t")).clone(), (literal!(" ")).clone())?).clone();
                    infoNodeStr = (System::stringReplace((infoNodeStr.clone()).clone(), (literal!("/")).clone(), (literal!("")).clone())?).clone();
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), infoNode.clone())?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n\n")).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("graph \"")).clone(), (modelNameQualified.clone()).clone(), (literal!("\"\n{\n\n")).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("overlap=false;\n")).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("layout=dot;\n\n")).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("node [")).clone(), (literal!("fillcolor = \"lightsteelblue1\", ")).clone(), (literal!("shape = box, ")).clone(), (literal!("style = \"bold, filled\", ")).clone(), (literal!("rank = \"max\"")).clone(), (literal!("]\n\n")).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("edge [")).clone(), (literal!("color = \"black\", ")).clone(), (literal!("style = bold")).clone(), (literal!("]\n\n")).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(i.clone()).clone(), (literal!("graph [fontsize=20, fontname = \"Courier Bold\" label= \"\\n\\n")).clone(), (infoNodeStr.clone()).clone(), (literal!("\", size=\"6,6\"];\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Definite Roots (Connections.root)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(definiteRoots.clone(), (std::sync::Arc::new(graphVizDefiniteRoot) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>), finalRoots.clone())?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Potential Roots (Connections.potentialRoot)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(potentialRoots.clone(), (std::sync::Arc::new(graphVizPotentialRoot) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, metamodelica::Real), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<ArcStr> + 'static>), finalRoots.clone())?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Branches (Connections.branch)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map(branches.clone(), (std::sync::Arc::new(graphVizEdge) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<ArcStr> + 'static>))?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Connections (connect)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(connections.clone(), (std::sync::Arc::new(graphVizDaeEdge) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>), Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>)>>) -> Result<ArcStr> + 'static>), broken.clone())?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n}\n")).clone()])?;
                    tEnd = clock();
                    t = tEnd.clone() - tStart.clone();
                    timeStr = (realString(t.clone())).clone();
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n\n\n// graph generation took: ")).clone(), (timeStr.clone()).clone(), (literal!(" seconds\n")).clone()])?;
                    System::writeFile((fileName.clone()).clone(), (IOStream::string(graphVizStream.clone())?).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz with connection graph for model: ")); __mm_s.push_str(&*modelNameQualified.clone()); __mm_s.push_str(&*literal!(" was writen to file: ")); __mm_s.push_str(&*fileName.clone()); ArcStr::from(__mm_s) }).clone())?;
                    brokenConnects = (showGraphViz((fileName.clone()).clone(), (modelNameQualified.clone()).clone())?).clone();
                    Ok(brokenConnects.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(brokenConnectsViaGraphViz)
}

fn showGraphViz(mut fileNameGraphViz: ArcStr, mut modelNameQualified: ArcStr) -> Result<ArcStr> {
    let mut brokenConnectsViaGraphViz: ArcStr;
    brokenConnectsViaGraphViz = ('mc: {
        let __mc_input = modelNameQualified.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::isSet(Flags::CGRAPH_GRAPHVIZ_SHOW.clone())?) else { bail!("pattern mismatch") };
            Ok(literal!(""))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut leftyCMD: ArcStr;
            let mut fileNameTraceRemovedConnections: ArcStr;
            let mut omhome: ArcStr;
            let mut brokenConnects: ArcStr;
            let mut leftyExitStatus: i32;
            fileNameTraceRemovedConnections = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelNameQualified.clone()); __mm_s.push_str(&*literal!("_removed_connections.txt")); ArcStr::from(__mm_s) }).clone();
            Debug::traceln((literal!("Tyring to start GraphViz *lefty* to visualize the graph. You need to have lefty in your PATH variable")).clone())?;
            Debug::traceln((literal!("Make sure you quit GraphViz *lefty* via Right Click->quit to be sure the process will be exited.")).clone())?;
            Debug::traceln((literal!("If you quit the GraphViz *lefty* window via X, please kill the process in task manager to continue.")).clone())?;
            omhome = (Settings::getInstallationDirectoryPath()?).clone();
            omhome = (System::stringReplace((omhome.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
            leftyCMD = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("load('")); __mm_s.push_str(&*omhome.clone()); __mm_s.push_str(&*literal!("/share/omc/scripts/openmodelica.lefty');")); __mm_s.push_str(&*literal!("openmodelica.init();openmodelica.createviewandgraph('")); __mm_s.push_str(&*fileNameGraphViz.clone()); __mm_s.push_str(&*literal!("','file',null,null);txtview('off');")); ArcStr::from(__mm_s) }).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Running command: ")); __mm_s.push_str(&*literal!("lefty -e ")); __mm_s.push_str(&*leftyCMD.clone()); __mm_s.push_str(&*literal!(" > ")); __mm_s.push_str(&*fileNameTraceRemovedConnections.clone()); ArcStr::from(__mm_s) }).clone())?;
            leftyExitStatus = System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lefty -e ")); __mm_s.push_str(&*leftyCMD.clone()); ArcStr::from(__mm_s) }).clone(), (fileNameTraceRemovedConnections.clone()).clone());
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz *lefty* exited with status:")); __mm_s.push_str(&*intString(leftyExitStatus.clone())); ArcStr::from(__mm_s) }).clone())?;
            brokenConnects = (System::readFile((fileNameTraceRemovedConnections.clone()).clone())?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz OpenModelica assistant returned the following broken connects: ")); __mm_s.push_str(&*brokenConnects.clone()); ArcStr::from(__mm_s) }).clone())?;
            Ok(brokenConnects.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(brokenConnectsViaGraphViz)
}

pub(crate) fn removeBrokenConnects(mut inConnects: Arc<metamodelica::List<DAE::Connect::ConnectorElement>>, mut inConnected: DaeEdges, mut inBroken: DaeEdges) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>>>> {
    let mut outConnects: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>>>;
    outConnects = (::match_deref::match_deref! { match &(inBroken.clone()) {
        Deref @ metamodelica::List::Nil => {
            list![inConnects]
        },
        _ => {
            let mut toRemove: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut toKeep: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut intersect: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut cset: Arc<metamodelica::List<DAE::Connect::ConnectorElement>>;
            let mut csets: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>>>;
            toRemove = filterFromSet(inConnects.clone(), inBroken, metamodelica::nil(), (literal!("removed")).clone())?;
            if toRemove.clone().is_empty() {
                csets = list![inConnects];
            } else {
                toKeep = filterFromSet(inConnects.clone(), inConnected.clone(), metamodelica::nil(), (literal!("allowed")).clone())?;
                intersect = List::intersectionOnTrue(toRemove.clone(), toKeep.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: CS: ")); __mm_s.push_str(&*stringDelimitList(List::map(inConnects.clone(), (std::sync::Arc::new(ConnectUtil::printElementStr) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Connect::ConnectorElement) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: keep: ")); __mm_s.push_str(&*stringDelimitList(List::map(toKeep.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: delete: ")); __mm_s.push_str(&*stringDelimitList(List::map(toRemove.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: allow = remove - keep: ")); __mm_s.push_str(&*stringDelimitList(List::map(intersect.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                }
                toRemove = List::setDifference(toRemove.clone(), intersect.clone())?;
                if Flags::isSet(Flags::CGRAPH.clone())? {
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.removeBrokenConnects: allow - delete: ")); __mm_s.push_str(&*stringDelimitList(List::map(toRemove.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                }
                cset = removeFromConnects(inConnects, toRemove.clone())?;
                csets = splitSetByAllowed(cset.clone(), inConnected)?;
            }
            csets.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outConnects)
}

fn splitSetByAllowed(mut inConnects: Arc<metamodelica::List<DAE::Connect::ConnectorElement>>, mut inConnected: DaeEdges) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>>>> {
    let mut outConnects: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>>>;
    let mut cset: Arc<metamodelica::List<DAE::Connect::ConnectorElement>>;
    let mut csets: Arc<metamodelica::List<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>>>;
    let mut e: DaeEdge = (Arc::new(DAE::ComponentRef::WILD), Arc::new(DAE::ComponentRef::WILD), metamodelica::nil());
    let mut cr1: Arc<DAE::ComponentRef>;
    let mut cr2: Arc<DAE::ComponentRef>;
    let mut ce: DAE::Connect::ConnectorElement = <DAE::Connect::ConnectorElement as ::std::default::Default>::default();
    csets = metamodelica::nil();
    for mut e in &*inConnected {
        let mut e = e.clone();
        cset = metamodelica::nil();
        (cr1, cr2, _) = e.clone();
        for mut ce in &*inConnects.clone() {
            let mut ce = ce.clone();
            if ComponentReferenceBasics::crefPrefixOf(cr1.clone(), ce.name.clone())? {
                cset = metamodelica::cons(ce.clone(), cset.clone());
            }
            if ComponentReferenceBasics::crefPrefixOf(cr2.clone(), ce.name.clone())? {
                cset = metamodelica::cons(ce.clone(), cset.clone());
            }
        }
        if !(cset.clone().is_empty()) {
            csets = metamodelica::cons(cset.clone(), csets.clone());
        }
    }
    outConnects = csets;
    Ok(outConnects)
}

fn filterFromSet(mut inConnects: Arc<metamodelica::List<DAE::Connect::ConnectorElement>>, mut inFilter: DaeEdges, mut inAcc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut msg: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut filteredCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    filteredCrefs = 'mc: {
        let __mc_input = inFilter;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(List::unique(inAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (c1, c2, _), tail: rest } => {
                    let mut filtered: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let true = (ConnectUtil::isReferenceInConnects(inConnects.clone(), c1.clone())?) else { bail!("pattern mismatch") };
                    let true = (ConnectUtil::isReferenceInConnects(inConnects.clone(), c2.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::CGRAPH.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectionGraph.filterFromSet: ")); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(" connect(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    filtered = filterFromSet(inConnects.clone(), rest.clone(), metamodelica::cons(c1.clone(), metamodelica::cons(c2.clone(), inAcc.clone())), (msg.clone()).clone())?;
                    Ok(filtered.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut filtered: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    filtered = filterFromSet(inConnects.clone(), rest.clone(), inAcc.clone(), (msg.clone()).clone())?;
                    Ok(filtered.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(filteredCrefs)
}

fn removeFromConnects(mut inConnects: Arc<metamodelica::List<DAE::Connect::ConnectorElement>>, mut inToRemove: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<DAE::Connect::ConnectorElement>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inConnects.clone(), inToRemove)) {
        (_, Deref @ metamodelica::List::Nil) => {
            return Ok(inConnects)
        },
        (cset, Deref @ metamodelica::List::Cons { head: c, tail: rest }) => {
            let mut cset = (*cset).clone();
            let __pa0 = ::match_deref::match_deref! { match &(ConnectUtil::removeReferenceFromConnects(cset.clone(), c.clone())?) {
                (__pa0, true) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cset = __pa0.clone();
            { (inConnects, inToRemove) = (cset.clone(), rest.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn addBrokenEqualityConstraintEquations(mut inDAE: DAE::DAElist, mut inBroken: DaeEdges) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist;
    outDAE = 'mc: {
        let __mc_input = inBroken.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inDAE.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut equalityConstraintElements: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut dae: DAE::DAElist;
                    equalityConstraintElements = List::flatten(List::map(inBroken.clone(), std::sync::Arc::new(fnptr!(Util::tuple33, _)))?)?;
                    dae = DAEUtil::joinDaes(DAE::DAElist { elementLst: equalityConstraintElements.clone() }, inDAE.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDAE)
}

