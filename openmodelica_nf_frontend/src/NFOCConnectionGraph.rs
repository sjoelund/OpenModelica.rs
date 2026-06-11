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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnections;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::DisjointSets;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/// a tuple with two crefs and equation(s) for calling the equalityConstraint function call
pub type FlatEdge = NFConnections::BrokenEdge;

/// a lit of broken edges
pub type FlatEdges = Arc<metamodelica::List<NFConnections::BrokenEdge>>;

/// an edge is a tuple with two component references
pub type Edge = (Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>);

/// A list of edges
pub type Edges = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>;

/// root defined with Connection.root
pub type DefiniteRoot = Arc<ComponentRef::NFComponentRef>;

/// roots defined with Connection.root
pub type DefiniteRoots = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

/// roots defined with Connection.uniqueRoot
pub type UniqueRoots = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>>;

/// potential root defined with Connections.potentialRoot
pub type PotentialRoot = (Arc<ComponentRef::NFComponentRef>, metamodelica::Real);

/// potential roots defined with Connections.potentialRoot
pub type PotentialRoots = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, metamodelica::Real)>>;

/// Input structure for connection breaking algorithm. It is collected during instantiation phase.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFOCConnectionGraph {
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
    pub connections: FlatEdges,
}

impl metamodelica::gc::MMTrace for NFOCConnectionGraph {
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
impl Default for NFOCConnectionGraph {
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

pub type GRAPH = NFOCConnectionGraph;


thread_local! { static __EMPTY_TLS: NFOCConnectionGraph = NFOCConnectionGraph { updateGraph: true, definiteRoots: metamodelica::nil(), potentialRoots: metamodelica::nil(), uniqueRoots: metamodelica::nil(), branches: metamodelica::nil(), connections: metamodelica::nil() }; }
pub(crate) fn EMPTY() -> NFOCConnectionGraph { __EMPTY_TLS.with(|__t| __t.clone()) }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ConnectionsOperator {
    BRANCH = 1,
    ROOT = 2,
    POTENTIAL_ROOT = 3,
    IS_ROOT = 4,
    ROOTED = 5,
    UNIQUE_ROOT = 6,
    UNIQUE_ROOT_INDICES = 7,
    NOT_OPERATOR = 8,
}
impl PartialOrd for ConnectionsOperator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ConnectionsOperator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for ConnectionsOperator {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub type CrefCrefTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;

pub type CrefIndexTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;

pub type CrefRootsTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;

pub mod CrefSets {
    use super::*;
    pub(crate) fn EntryHash(mut entry: Entry) -> Result<i32> {
        let mut hash: i32;
        hash = ComponentRef::hash(entry)?;
        Ok(hash)
    }

    pub(crate) fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> Result<bool> {
        let mut isEqual: bool;
        isEqual = ComponentRef::isEqual(entry1, entry2)?;
        Ok(isEqual)
    }

    pub(crate) fn EntryString(mut entry: Entry) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (ComponentRef::toString(entry)?).clone();
        Ok(r#str)
    }

    pub type Entry = Arc<ComponentRef::NFComponentRef>;

    pub type IndexTable = Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;

    /// This is a disjoint sets data structure. The nodes are stored in an array of
    ///   Integers. The root elements of a set is given a negative value that
    ///   corresponds to its rank, while other elements are given positive values that
    ///   corresponds to the index of their parent in the array. The hashtable is used
    ///   to look up the array index of a entry, and is also used to store the entries.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Sets {
        /// An array of nodes
        pub nodes: metamodelica::Array<i32>,
        /// An Entry->Integer table.
        pub elements: IndexTable,
        /// The number of nodes stored in the sets.
        pub nodeCount: i32,
    }

    impl metamodelica::gc::MMTrace for Sets {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.nodes, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.elements, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nodeCount, __mmv)?;
            Ok(())
        }
    }
    impl Default for Sets {
        fn default() -> Self {
            Self {
                nodes: Default::default(),
                elements: Default::default(),
                nodeCount: Default::default(),
            }
        }
    }

    pub type DISJOINT_SETS = Sets;


    pub(crate) fn add(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
        let mut sets: Sets = sets;
        let mut index: i32;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut node_count: i32;
        let Sets { nodes: __pa0, elements: __pa1, nodeCount: __pa2 } = (sets) else { bail!("pattern mismatch") };
        nodes = __pa0.clone();
        elements = __pa1.clone();
        node_count = __pa2.clone();
        index = node_count + 1;
        if index > metamodelica::arrayLength(nodes.clone()) {
            nodes = Array::expand(((intReal(index) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32), nodes.clone(), -1)?;
        }
        UnorderedMap::addNew(entry, index, elements.clone())?;
        sets = Sets { nodes: nodes.clone(), elements: elements, nodeCount: index };
        Ok((sets, index))
    }

    pub(crate) fn addList(mut entries: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut node_count: i32;
        let mut sz: i32;
        let mut index: i32;
        let Sets { nodes: __pa0, elements: __pa1, nodeCount: __pa2 } = (sets) else { bail!("pattern mismatch") };
        nodes = __pa0.clone();
        elements = __pa1.clone();
        node_count = __pa2.clone();
        sz = (entries.clone().len() as i32);
        index = node_count + 1;
        node_count = node_count + sz;
        if node_count > metamodelica::arrayLength(nodes.clone()) {
            nodes = Array::expand(((intReal(node_count) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32), nodes.clone(), -1)?;
        }
        for mut e in &*entries {
            let mut e = e.clone();
            UnorderedMap::addNew(e.clone(), index, elements.clone())?;
            index = index + 1;
        }
        sets = Sets { nodes: nodes.clone(), elements: elements, nodeCount: node_count };
        Ok(sets)
    }

    pub(crate) fn contains(mut entry: Entry, mut sets: Sets) -> Result<bool> {
        let mut found: bool;
        found = isSome(UnorderedMap::get(entry, sets.elements.clone())?);
        Ok(found)
    }

    pub(crate) fn emptySets(mut setCount: i32) -> Sets {
        let mut sets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut elements: IndexTable;
        let mut sz: i32;
        sz = std::cmp::max(setCount, 3);
        nodes = arrayCreate(sz, -1);
        elements = UnorderedMap::new((std::sync::Arc::new(EntryHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(EntryEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        sets = Sets { nodes: nodes.clone(), elements: elements, nodeCount: 0 };
        sets
    }

    pub(crate) fn extractSets(mut sets: Sets) -> (metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>, Sets) {
        let mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut assignedSets: Sets;
        let mut nodes: metamodelica::Array<i32>;
        let mut set_idx: i32 = 0;
        let mut idx: i32;
        let mut entries: metamodelica::Array<(Arc<ComponentRef::NFComponentRef>, i32)>;
        let mut e: Entry;
        nodes = sets.nodes.clone();
        for mut i in 1..=sets.nodeCount.clone() {
            if ({let __elt = nodes.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0 {
                set_idx = set_idx + 1;
                {
                    let __cell0 = -(set_idx);
                    let __idx0 = i.clone();
                    nodes.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
                }
            }
        }
        setsArray = arrayCreate(set_idx, metamodelica::nil());
        entries = UnorderedMap::toArray(sets.elements.clone());
        for mut i in ({let __s=metamodelica::arrayLength(entries.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            (e, idx) = metamodelica::Dangerous::arrayGetNoBoundsChecking(entries.clone(), i.clone());
            set_idx = ({let __elt = nodes.borrow()[(idx-1) as usize].clone(); __elt});
            while set_idx > 0 {
                set_idx = ({let __elt = nodes.borrow()[(set_idx-1) as usize].clone(); __elt});
            }
            set_idx = -(set_idx);
            {
                let __cell1 = metamodelica::cons(e.clone(), ({let __elt = setsArray.borrow()[(set_idx-1) as usize].clone(); __elt}));
                let __idx1 = set_idx;
                setsArray.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
            }
        }
        assignedSets = Sets { nodes: nodes.clone(), elements: sets.elements.clone(), nodeCount: sets.nodeCount.clone() };
        (setsArray, assignedSets)
    }

    pub(crate) fn find(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
        let mut sets: Sets = sets;
        let mut index: i32;
        let mut oindex: Option<i32>;
        oindex = UnorderedMap::get(entry.clone(), sets.elements.clone())?;
        if isSome(oindex.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(oindex) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            index = __pa0.clone();
        } else {
            (sets, index) = add(entry, sets)?;
        }
        Ok((sets, index))
    }

    pub(crate) fn findRoot(mut nodeIndex: i32, mut nodes: metamodelica::Array<i32>) -> Result<i32> {
        let mut rootIndex: i32 = nodeIndex;
        let mut parent: i32 = ({let __elt = nodes.borrow()[(nodeIndex-1) as usize].clone(); __elt});
        let mut idx: i32 = nodeIndex;
        while parent > 0 {
            rootIndex = parent;
            parent = ({let __elt = nodes.borrow()[(parent-1) as usize].clone(); __elt});
        }
        parent = ({let __elt = nodes.borrow()[(nodeIndex-1) as usize].clone(); __elt});
        while parent > 0 {
            metamodelica::arrayUpdate(nodes.clone(), idx, rootIndex)?;
            idx = parent;
            parent = ({let __elt = nodes.borrow()[(parent-1) as usize].clone(); __elt});
        }
        Ok(rootIndex)
    }

    pub(crate) fn findSet(mut entry: Entry, mut sets: Sets) -> Result<(i32, Sets)> {
        let mut set: i32;
        let mut updatedSets: Sets;
        let mut index: i32;
        (updatedSets, index) = find(entry, sets)?;
        set = findRoot(index, updatedSets.nodes.clone())?;
        Ok((set, updatedSets))
    }

    pub(crate) fn findSetArrayIndex(mut entry: Entry, mut sets: Sets) -> Result<i32> {
        let mut set: i32;
        set = UnorderedMap::getOrFail(entry, sets.elements.clone())?;
        while set > 0 {
            set = ({let __elt = sets.nodes.borrow()[(set-1) as usize].clone(); __elt});
        }
        set = -(set);
        Ok(set)
    }

    pub(crate) fn getEntry(mut entry: Entry, mut sets: Sets) -> Result<Option<Arc<ComponentRef::NFComponentRef>>> {
        let mut outEntry: Option<Arc<ComponentRef::NFComponentRef>>;
        outEntry = UnorderedMap::getKey(entry, sets.elements.clone())?;
        Ok(outEntry)
    }

    pub(crate) fn getNodeCount(mut sets: Sets) -> i32 {
        let mut nodeCount: i32 = sets.nodeCount.clone();
        nodeCount
    }

    pub(crate) fn merge(mut entry1: Entry, mut entry2: Entry, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut set1: i32;
        let mut set2: i32;
        (set1, sets) = findSet(entry1, sets)?;
        (set2, sets) = findSet(entry2, sets)?;
        sets = union(set1, set2, sets)?;
        Ok(sets)
    }

    pub(crate) fn printSets(mut sets: Sets) -> Result<()> {
        let mut nodes: metamodelica::Array<i32>;
        let mut entries: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, i32)>>;
        let mut e: Entry;
        let mut i: i32;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(sets.nodeCount.clone())); __mm_s.push_str(&*literal!(" sets:\n")); ArcStr::from(__mm_s) }).clone());
        nodes = sets.nodes.clone();
        entries = UnorderedMap::toList(sets.elements.clone());
        for mut p in &*entries {
            let mut p = p.clone();
            (e, i) = p.clone();
            metamodelica::print((literal!("[")).clone());
            metamodelica::print(ArcStr::from(::std::format!("{}", i)));
            metamodelica::print((literal!("]")).clone());
            metamodelica::print((EntryString(e.clone())?).clone());
            metamodelica::print((literal!(" -> ")).clone());
            metamodelica::print(ArcStr::from(::std::format!("{}", ({let __elt = nodes.borrow()[(i-1) as usize].clone(); __elt}))));
            metamodelica::print((literal!("\n")).clone());
        }
        Ok(())
    }

    pub(crate) fn union(mut set1: i32, mut set2: i32, mut sets: Sets) -> Result<Sets> {
        let mut sets: Sets = sets;
        let mut rank1: i32;
        let mut rank2: i32;
        if set1 != set2 {
            rank1 = ({let __elt = sets.nodes.borrow()[(set1-1) as usize].clone(); __elt});
            rank2 = ({let __elt = sets.nodes.borrow()[(set2-1) as usize].clone(); __elt});
            if rank1 > rank2 {
                metamodelica::arrayUpdate(sets.nodes.clone(), set2, set1)?;
            } else if rank1 < rank2 {
                metamodelica::arrayUpdate(sets.nodes.clone(), set1, set2)?;
            } else {
                metamodelica::arrayUpdate(sets.nodes.clone(), set1, ({let __elt = sets.nodes.borrow()[(set1-1) as usize].clone(); __elt}) - 1)?;
                metamodelica::arrayUpdate(sets.nodes.clone(), set2, set1)?;
            }
        }
        Ok(sets)
    }

}

pub type IsDeletedFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>;

pub(crate) fn handleOverconstrainedConnections(mut flatModel: Arc<FlatModel::NFFlatModel>, mut conns: Arc<NFConnections::NFConnections>, mut isDeleted: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>) -> Result<(Arc<FlatModel::NFFlatModel>, FlatEdges)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut broken: FlatEdges;
    let mut graph: NFOCConnectionGraph = EMPTY().clone();
    let mut connected: FlatEdges;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut print_trace: bool = Flags::isSet(Flags::CGRAPH.clone())?;
    graph = addBreakableBranches(conns.connections.clone(), isDeleted.clone(), print_trace, graph)?;
    (eql, graph) = addRootsAndBranches(flatModel.equations.clone(), print_trace, graph)?;
    assign_field!(flatModel.equations = eql);
    (flatModel, connected, broken) = handleOverconstrainedConnections_dispatch(graph, flatModel)?;
    assign_field!(flatModel.equations = removeBrokenConnects(flatModel.equations.clone(), connected, broken.clone(), isDeleted.clone())?);
    Ok((flatModel, broken))
}

fn addBreakableBranches(mut connections: Arc<metamodelica::List<Arc<Connection::NFConnection>>>, mut isDeleted: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    let mut breakable: CrefSets::Sets;
    let mut c1: Arc<Connector::NFConnector>;
    let mut c2: Arc<Connector::NFConnector>;
    let mut lhs_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut rhs_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut rhs: Arc<ComponentRef::NFComponentRef>;
    let mut lhs_set: i32;
    let mut rhs_set: i32;
    breakable = CrefSets::emptySets(3);
    for mut conn in &*connections {
        let mut conn = conn.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
            Deref @ Connection::CONNECTION { lhs: __pa0, rhs: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        c1 = __pa0.clone();
        c2 = __pa1.clone();
        lhs_crefs = getOverconstrainedCrefs(c1.clone(), isDeleted.clone())?;
        rhs_crefs = getOverconstrainedCrefs(c2.clone(), isDeleted.clone())?;
        for mut lhs in &*lhs_crefs.clone() {
            let mut lhs = lhs.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rhs_crefs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs = __pa2.clone();
            rhs_crefs = __pa3.clone();
            (lhs_set, breakable) = CrefSets::findSet(lhs.clone(), breakable.clone())?;
            (rhs_set, breakable) = CrefSets::findSet(rhs.clone(), breakable.clone())?;
            if lhs_set != rhs_set {
                graph = addConnection(lhs.clone(), rhs.clone(), c1.source.clone(), printTrace, graph.clone())?;
                breakable = CrefSets::union(lhs_set, rhs_set, breakable.clone())?;
            }
        }
    }
    Ok(graph)
}

fn addRootsAndBranches(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, NFOCConnectionGraph)> {
    let mut outEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut graph: NFOCConnectionGraph = graph;
    let mut call: Arc<Call::NFCall> = Arc::new(<Call::NFCall as ::std::default::Default>::default());
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut root: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut msg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut priority: i32 = 0;
    for mut eq in &*equations {
        let mut eq = eq.clone();
        outEquations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { arguments: __esc_args, .. } }, .. } => {
            call = (*__esc_call).clone();
            args = (*__esc_args).clone();
            (match identifyConnectionsOperator(Function::name(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone())) {
        ConnectionsOperator::ROOT => {
            let __pa0 = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa0.clone();
            graph = addDefiniteRoot(cref.clone(), printTrace, graph.clone())?;
            outEquations.clone()
        },
        ConnectionsOperator::POTENTIAL_ROOT => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa0.clone();
            arg2 = __pa1.clone();
            let __pa3 = ::match_deref::match_deref! { match &(arg1.clone()) {
                Deref @ Expression::CREF { cref: __pa3, .. } => __pa3.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa3.clone();
            let __pa4 = ::match_deref::match_deref! { match &(Ceval::evalExp(arg2.clone(), Ceval::noTarget().clone())?) {
                Deref @ Expression::INTEGER { value: __pa4 } => __pa4.clone(),
                _ => bail!("pattern mismatch"),
            } };
            priority = __pa4.clone();
            graph = addPotentialRoot(cref.clone(), metamodelica::OrderedFloat((priority) as f64), printTrace, graph.clone())?;
            outEquations.clone()
        },
        ConnectionsOperator::UNIQUE_ROOT => {
            graph = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_root @ Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::List::Nil } => {
            root = (*__esc_root).clone();
            cref = (*__esc_cref).clone();
            addUniqueRoots(root.clone(), Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() }), printTrace, graph.clone())?
        },
        Deref @ metamodelica::List::Cons { head: __esc_root @ Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::List::Cons { head: __esc_msg, tail: Deref @ metamodelica::List::Nil } } => {
            root = (*__esc_root).clone();
            cref = (*__esc_cref).clone();
            msg = (*__esc_msg).clone();
            addUniqueRoots(root.clone(), msg.clone(), printTrace, graph.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
            outEquations.clone()
        },
        ConnectionsOperator::BRANCH { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lhs = __pa0.clone();
            rhs = __pa1.clone();
            graph = addBranch(lhs.clone(), rhs.clone(), printTrace, graph.clone())?;
            outEquations.clone()
        },
        _ => metamodelica::cons(eq.clone(), outEquations.clone()),
    })
        },
        _ => metamodelica::cons(eq.clone(), outEquations.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outEquations = metamodelica::Dangerous::listReverseInPlace(outEquations);
    Ok((outEquations, graph))
}

fn generateEqualityConstraintEquation(mut lhs: Arc<ComponentRef::NFComponentRef>, mut rhs: Arc<ComponentRef::NFComponentRef>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<Equation::NFEquation>> {
    let mut equalityConstraintEq: Arc<Equation::NFEquation>;
    let mut context: i32;
    let mut fcref_rhs: Arc<ComponentRef::NFComponentRef>;
    let mut fcref_lhs: Arc<ComponentRef::NFComponentRef>;
    let mut fn_node_rhs: Arc<InstNode::InstNode>;
    let mut fn_node_lhs: Arc<InstNode::InstNode>;
    let mut exp_rhs: Arc<Expression::NFExpression>;
    let mut exp_lhs: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut info: SourceInfo = ElementSource::getInfo(source.clone());
    context = intBitOr(InstContext::EQUATION.clone(), InstContext::CONNECT.clone());
    fcref_rhs = Function::lookupFunctionSimple((literal!("equalityConstraint")).clone(), InstNode::classScope(ComponentRef::node(lhs.clone())?), context)?;
    (fcref_rhs, fn_node_rhs, _) = Function::instFunctionRef(fcref_rhs, context, Absyn::dummyInfo.clone())?;
    exp_rhs = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: fcref_rhs, arguments: list![Expression::fromCref(lhs.clone(), false)?, Expression::fromCref(rhs, false)?], named_args: metamodelica::nil(), call_scope: fn_node_rhs }) });
    (exp_rhs, ty, _, _) = Typing::typeExp(exp_rhs, context, info.clone(), false)?;
    fcref_lhs = Function::lookupFunctionSimple((literal!("fill")).clone(), InstNode::topScope(ComponentRef::node(lhs)?)?, context)?;
    (fcref_lhs, fn_node_lhs, _) = Function::instFunctionRef(fcref_lhs, context, Absyn::dummyInfo.clone())?;
    exp_lhs = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: fcref_lhs, arguments: metamodelica::cons(Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }), ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut d in (Type::arrayDims(ty)).into_iter().cloned() {
            let __x = Dimension::sizeExp(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), named_args: metamodelica::nil(), call_scope: fn_node_lhs }) });
    (exp_lhs, ty, _, _) = Typing::typeExp(exp_lhs, context, info, false)?;
    equalityConstraintEq = Equation::makeEquality(exp_rhs, exp_lhs, ty, source, crate::NFInstNode::InstNode::interned_EMPTY_NODE(), Equation::ScalarizeMode::NO_PREFERENCE.clone());
    Ok(equalityConstraintEq)
}

fn getOverconstrainedCrefs(mut conn: Arc<Connector::NFConnector>, mut isDeleted: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut conns: Arc<metamodelica::List<Arc<Connector::NFConnector>>>;
    conns = Connector::split(conn)?;
    conns = List::mapFlat(conns, (std::sync::Arc::new(Connector::scalarizePrefix) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<Arc<metamodelica::List<Arc<Connector::NFConnector>>>> + 'static>))?;
    crefs = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (conns).into_iter().cloned() {
            if !(!(isDeleted(c.name.clone())?) && isOverconstrainedCref(c.name.clone())?) { continue; }
            let __x = getOverconstrainedCref(c.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    crefs = List::uniqueOnTrue(crefs, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    Ok(crefs)
}

fn isOverconstrainedCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rest: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    b = (::match_deref::match_deref! { match &(cref) {
        Deref @ ComponentRef::CREF { node: __esc_node, origin: ComponentRef::Origin::CREF, restCref: __esc_rest, .. } => {
            node = (*__esc_node).clone();
            rest = (*__esc_rest).clone();
            Class::isOverdetermined(InstNode::getClass(node.clone())?) || isOverconstrainedCref(rest.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn getOverconstrainedCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    '__tco: loop {
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut rest: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { node: __esc_node, origin: ComponentRef::Origin::CREF, restCref: __esc_rest, .. } => {
            node = (*__esc_node).clone();
            rest = (*__esc_rest).clone();
            if (Class::isOverdetermined(InstNode::getClass(node.clone())?)) {return Ok(cref)} else {{ cref = rest.clone(); continue '__tco; }}
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn handleOverconstrainedConnections_dispatch(mut graph: NFOCConnectionGraph, mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<(Arc<FlatModel::NFFlatModel>, FlatEdges, FlatEdges)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut connected: FlatEdges;
    let mut broken: FlatEdges;
    let mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut rooted: CrefIndexTable;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Summary:\n\t")); __mm_s.push_str(&*literal!("Nr Roots:           ")); __mm_s.push_str(&*intString((unwrap_break_err!(getDefiniteRoots(graph.clone()), '__try0).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Potential Roots: ")); __mm_s.push_str(&*intString((unwrap_break_err!(getPotentialRoots(graph.clone()), '__try0).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Unique Roots:    ")); __mm_s.push_str(&*intString((unwrap_break_err!(getUniqueRoots(graph.clone()), '__try0).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Branches:        ")); __mm_s.push_str(&*intString((unwrap_break_err!(getBranches(graph.clone()), '__try0).len() as i32))); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("Nr Connections:     ")); __mm_s.push_str(&*intString((unwrap_break_err!(getConnections(graph.clone()), '__try0).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        (roots, connected, broken) = unwrap_break_err!(findResultGraph(graph.clone(), (unwrap_break_err!(FlatModel::fullName(flatModel.clone()), '__try0)).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Roots: ")); __mm_s.push_str(&*stringDelimitList(unwrap_break_err!(List::map(roots.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>)), '__try0), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Broken connections: ")); __mm_s.push_str(&*stringDelimitList(unwrap_break_err!(List::map1(broken.clone(), (std::sync::Arc::new(printConnectionStr) as std::sync::Arc<dyn ::std::ops::Fn(NFConnections::BrokenEdge, ArcStr) -> Result<ArcStr> + 'static>), (literal!("broken")).clone()), '__try0), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowed connections: ")); __mm_s.push_str(&*stringDelimitList(unwrap_break_err!(List::map1(connected.clone(), (std::sync::Arc::new(printConnectionStr) as std::sync::Arc<dyn ::std::ops::Fn(NFConnections::BrokenEdge, ArcStr) -> Result<ArcStr> + 'static>), (literal!("allowed")).clone()), '__try0), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        rooted = unwrap_break_err!(buildRootedTable(roots.clone(), graph.clone()), '__try0);
        assign_field!(
            flatModel.variables = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(evalConnectionsOperatorsVar(roots.clone(), rooted.clone(), graph.clone(), v.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
            flatModel.equations = unwrap_break_err!(evalConnectionsOperatorsEqs(roots.clone(), rooted.clone(), graph.clone(), flatModel.equations.clone()), '__try0),
            flatModel.initialEquations = unwrap_break_err!(evalConnectionsOperatorsEqs(roots.clone(), rooted.clone(), graph.clone(), flatModel.initialEquations.clone()), '__try0)
        );
        Ok::<_, anyhow::Error>((broken.clone(), connected.clone(), flatModel.clone(), rooted.clone(), roots.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            broken = __try0_o0;
            connected = __try0_o1;
            flatModel = __try0_o2;
            rooted = __try0_o3;
            roots = __try0_o4;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::CGRAPH.clone())?) else { bail!("pattern mismatch") };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.handleOverconstrainedConnections failed for model: ")); __mm_s.push_str(&*FlatModel::fullName(flatModel.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            return Err(__try0_err);
        }
    }
    Ok((flatModel, connected, broken))
}

fn addDefiniteRoot(mut root: Arc<ComponentRef::NFComponentRef>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addDefiniteRoot(")); __mm_s.push_str(&*ComponentRef::toString(root.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.definiteRoots = metamodelica::cons(root, graph.definiteRoots.clone());
    Ok(graph)
}

fn addPotentialRoot(mut root: Arc<ComponentRef::NFComponentRef>, mut priority: metamodelica::Real, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addPotentialRoot(")); __mm_s.push_str(&*ComponentRef::toString(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*realString(priority)); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.potentialRoots = metamodelica::cons((root, priority), graph.potentialRoots.clone());
    Ok(graph)
}

fn addUniqueRoots(mut roots: Arc<Expression::NFExpression>, mut message: Arc<Expression::NFExpression>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    let mut unique_roots: UniqueRoots = graph.uniqueRoots.clone();
    for mut root in &*Expression::arrayScalarElements(roots) {
        let mut root = root.clone();
        unique_roots = (::match_deref::match_deref! { match &(root.clone()) {
        Deref @ Expression::CREF { .. } => {
            if printTrace {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addUniqueRoots(")); __mm_s.push_str(&*Expression::toString(root.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
            }
            metamodelica::cons((var_field!((*root).cref, Expression::NFExpression::CREF).clone(), message.clone()), unique_roots.clone())
        },
        _ => unique_roots.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(graph)
}

fn addBranch(mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addBranch(")); __mm_s.push_str(&*ComponentRef::toString(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(ref2.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.branches = metamodelica::cons((ref1, ref2), graph.branches.clone());
    Ok(graph)
}

fn addConnection(mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>, mut source: Arc<DAE::ElementSource>, mut printTrace: bool, mut graph: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut graph: NFOCConnectionGraph = graph;
    if printTrace {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.addConnection(")); __mm_s.push_str(&*ComponentRef::toString(ref1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(ref2.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    graph.connections = metamodelica::cons(NFConnections::BrokenEdge { lhs: ref1, rhs: ref2, source: source, brokenEquations: metamodelica::nil() }, graph.connections.clone());
    Ok(graph)
}

// ************************************* //
// ********* protected section ********* //
// ************************************* //
fn canonical(mut inPartition: CrefCrefTable, mut inRef: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut outCanonical: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cref_opt: Option<Arc<ComponentRef::NFComponentRef>>;
    cref_opt = UnorderedMap::get(inRef.clone(), inPartition.clone())?;
    outCanonical = (::match_deref::match_deref! { match &(cref_opt) {
        Some(__esc_outCanonical) => {
            outCanonical = (*__esc_outCanonical).clone();
            canonical(inPartition, outCanonical.clone())?
        },
        _ => inRef,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCanonical)
}

fn areInSameComponent(mut partition: CrefCrefTable, mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut outResult: bool;
    outResult = ComponentRef::isEqual(canonical(partition.clone(), ref1)?, canonical(partition, ref2)?)?;
    Ok(outResult)
}

fn connectBranchComponents(mut partition: CrefCrefTable, mut ref1: Arc<ComponentRef::NFComponentRef>, mut ref2: Arc<ComponentRef::NFComponentRef>) -> Result<()> {
    connectCanonicalComponents(partition.clone(), canonical(partition.clone(), ref1)?, canonical(partition, ref2)?)?;
    Ok(())
}

fn connectComponents(mut partition: CrefCrefTable, mut edge: FlatEdge) -> (FlatEdges, FlatEdges) {
    let mut outConnectedConnections: FlatEdges;
    let mut outBrokenConnections: FlatEdges;
    let mut canon1: Arc<ComponentRef::NFComponentRef>;
    let mut canon2: Arc<ComponentRef::NFComponentRef>;
    let mut eq: Arc<Equation::NFEquation>;
    match '__try0: {
        canon1 = unwrap_break_err!(canonical(partition.clone(), edge.lhs.clone()), '__try0);
        canon2 = unwrap_break_err!(canonical(partition.clone(), edge.rhs.clone()), '__try0);
        let false = (unwrap_break_err!(connectCanonicalComponents(partition.clone(), canon1.clone(), canon2.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
            unwrap_break_err!(Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.connectComponents: should remove equations generated from: connect(")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(edge.lhs.clone()), '__try0)); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(edge.rhs.clone()), '__try0)); __mm_s.push_str(&*literal!(") and add {0, ..., 0} = equalityConstraint(cr1, cr2) instead.\n")); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        outConnectedConnections = metamodelica::nil();
        eq = unwrap_break_err!(generateEqualityConstraintEquation(edge.lhs.clone(), edge.rhs.clone(), edge.source.clone()), '__try0);
        outBrokenConnections = list![NFConnections::BrokenEdge { lhs: edge.lhs.clone(), rhs: edge.rhs.clone(), source: edge.source.clone(), brokenEquations: list![eq.clone()] }];
        Ok::<_, anyhow::Error>((outBrokenConnections.clone(), outConnectedConnections.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outBrokenConnections = __try0_o0;
            outConnectedConnections = __try0_o1;
        }
        Err(_) => {
            outConnectedConnections = list![edge.clone()];
            outBrokenConnections = metamodelica::nil();
        }
    }
    (outConnectedConnections, outBrokenConnections)
}

fn connectCanonicalComponents(mut inPartition: CrefCrefTable, mut inRef1: Arc<ComponentRef::NFComponentRef>, mut inRef2: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut outReallyConnected: bool;
    outReallyConnected = !(ComponentRef::isEqual(inRef1.clone(), inRef2.clone())?);
    if outReallyConnected {
        UnorderedMap::add(inRef1, inRef2, inPartition)?;
    }
    Ok(outReallyConnected)
}

fn addRootsToTable(mut table: CrefCrefTable, mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut firstRoot: Arc<ComponentRef::NFComponentRef>) -> Result<()> {
    let mut root: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    for mut root in &*roots {
        let mut root = root.clone();
        UnorderedMap::add(root.clone(), firstRoot.clone(), table.clone())?;
    }
    Ok(())
}

fn resultGraphWithRoots(mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<CrefCrefTable> {
    let mut outTable: CrefCrefTable;
    let mut dummyRoot: Arc<ComponentRef::NFComponentRef>;
    dummyRoot = NFBuiltin::TIME_CREF().clone();
    outTable = newCrefCrefTable();
    addRootsToTable(outTable.clone(), roots, dummyRoot)?;
    Ok(outTable)
}

fn addBranchesToTable(mut table: CrefCrefTable, mut branches: Edges) -> Result<()> {
    let mut ref1: Arc<ComponentRef::NFComponentRef>;
    let mut ref2: Arc<ComponentRef::NFComponentRef>;
    for mut branch in &*branches {
        let mut branch = branch.clone();
        (ref1, ref2) = branch.clone();
        connectBranchComponents(table.clone(), ref1.clone(), ref2.clone())?;
    }
    Ok(())
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
                    s1 = (ComponentRef::toString(c1.clone())?).clone();
                    s2 = (ComponentRef::toString(c2.clone())?).clone();
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

fn addPotentialRootsToTable(mut table: CrefCrefTable, mut potentialRoots: PotentialRoots, mut roots: DefiniteRoots, mut firstRoot: Arc<ComponentRef::NFComponentRef>) -> DefiniteRoots {
    let mut outRoots: DefiniteRoots;
    outRoots = 'mc: {
        let __mc_input = potentialRoots;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(roots.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (potentialRoot, _), tail: tail } => {
                    let mut canon1: Arc<ComponentRef::NFComponentRef>;
                    let mut canon2: Arc<ComponentRef::NFComponentRef>;
                    let mut finalRoots: DefiniteRoots;
                    canon1 = canonical(table.clone(), potentialRoot.clone())?;
                    canon2 = canonical(table.clone(), firstRoot.clone())?;
                    let true = (connectCanonicalComponents(table.clone(), canon1.clone(), canon2.clone())?) else { bail!("pattern mismatch") };
                    finalRoots = addPotentialRootsToTable(table.clone(), tail.clone(), metamodelica::cons(potentialRoot.clone(), roots.clone()), firstRoot.clone());
                    Ok(finalRoots.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: tail } => {
                    let mut finalRoots: DefiniteRoots;
                    finalRoots = addPotentialRootsToTable(table.clone(), tail.clone(), roots.clone(), firstRoot.clone());
                    Ok(finalRoots.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outRoots
}

fn addConnections(mut table: CrefCrefTable, mut inConnections: FlatEdges) -> (FlatEdges, FlatEdges) {
    let mut outConnectedConnections: FlatEdges = metamodelica::nil();
    let mut outBrokenConnections: FlatEdges = metamodelica::nil();
    let mut connected: FlatEdges;
    let mut broken: FlatEdges;
    for mut c in &*inConnections {
        let mut c = c.clone();
        (connected, broken) = connectComponents(table.clone(), c.clone());
        outConnectedConnections = listAppend(connected.clone(), outConnectedConnections.clone());
        outBrokenConnections = listAppend(broken.clone(), outBrokenConnections.clone());
    }
    (outConnectedConnections, outBrokenConnections)
}

fn findResultGraph(mut inGraph: NFOCConnectionGraph, mut modelNameQualified: ArcStr) -> Result<(DefiniteRoots, FlatEdges, FlatEdges)> {
    let mut outRoots: DefiniteRoots;
    let mut outConnectedConnections: FlatEdges;
    let mut outBrokenConnections: FlatEdges;
    (outRoots, outConnectedConnections, outBrokenConnections) = (::match_deref::match_deref! { match &(inGraph) {
        NFOCConnectionGraph { definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, .. } => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        NFOCConnectionGraph { definiteRoots, potentialRoots, uniqueRoots, branches, connections, .. } => {
            let mut finalRoots: DefiniteRoots;
            let mut orderedPotentialRoots: PotentialRoots;
            let mut broken: FlatEdges;
            let mut connected: FlatEdges;
            let mut table: CrefCrefTable;
            let mut dummyRoot: Arc<ComponentRef::NFComponentRef>;
            let mut brokenConnectsViaGraphViz: ArcStr;
            let mut userBrokenLst: Arc<metamodelica::List<ArcStr>>;
            let mut userBrokenLstLst: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
            let mut userBrokenTplLst: Arc<metamodelica::List<(ArcStr, ArcStr)>>;
            let mut connections = (*connections).clone();
            connections = connections.clone().reverse();
            table = resultGraphWithRoots(definiteRoots.clone())?;
            addBranchesToTable(table.clone(), branches.clone())?;
            orderedPotentialRoots = List::sort(potentialRoots.clone(), (std::sync::Arc::new(fnptr!(ord, (Arc<ComponentRef::NFComponentRef>, metamodelica::Real), (Arc<ComponentRef::NFComponentRef>, metamodelica::Real))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, metamodelica::Real), (Arc<ComponentRef::NFComponentRef>, metamodelica::Real)) -> Result<bool> + 'static>))?;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Ordered Potential Roots: ")); __mm_s.push_str(&*stringDelimitList(List::map(orderedPotentialRoots.clone(), (std::sync::Arc::new(printPotentialRootTuple) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, metamodelica::Real)) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (connected, broken) = addConnections(table.clone(), connections.clone());
            dummyRoot = NFBuiltin::TIME_CREF().clone();
            finalRoots = addPotentialRootsToTable(table.clone(), orderedPotentialRoots.clone(), definiteRoots.clone(), dummyRoot.clone());
            brokenConnectsViaGraphViz = (generateGraphViz((modelNameQualified.clone()).clone(), definiteRoots.clone(), potentialRoots.clone(), uniqueRoots.clone(), branches.clone(), connections.clone(), finalRoots.clone(), broken.clone())?).clone();
            if stringEq((brokenConnectsViaGraphViz.clone()).clone(), (literal!("")).clone()) {
            } else {
                userBrokenLst = Util::stringSplitAtChar((brokenConnectsViaGraphViz.clone()).clone(), (literal!("#")).clone())?;
                userBrokenLstLst = List::map1(userBrokenLst.clone(), (std::sync::Arc::new(Util::stringSplitAtChar) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), (literal!("|")).clone())?;
                userBrokenTplLst = makeTuple(userBrokenLstLst.clone())?;
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("User selected the following connect edges for breaking:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(userBrokenTplLst.clone(), (std::sync::Arc::new(fnptr!(printTupleStr, (ArcStr, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, ArcStr)) -> Result<ArcStr> + 'static>))?, (literal!("\n\t")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                printFlatEdges(connections.clone())?;
                connections = orderConnectsGuidedByUser(connections.clone(), userBrokenTplLst.clone())?;
                connections = connections.clone().reverse();
                metamodelica::print((literal!("\nAfer ordering:\n")).clone());
                (finalRoots, connected, broken) = findResultGraph(NFOCConnectionGraph { updateGraph: false, definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }, (modelNameQualified).clone())?;
            }
            (finalRoots.clone(), connected.clone(), broken.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRoots, outConnectedConnections, outBrokenConnections))
}

fn orderConnectsGuidedByUser(mut inConnections: FlatEdges, mut inUserSelectedBreaking: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<FlatEdges> {
    let mut outOrderedConnections: FlatEdges;
    let mut front: FlatEdges = metamodelica::nil();
    let mut back: FlatEdges = metamodelica::nil();
    let mut sc1: ArcStr;
    let mut sc2: ArcStr;
    for mut e in &*inConnections {
        let mut e = e.clone();
        sc1 = (ComponentRef::toString(e.lhs.clone())?).clone();
        sc2 = (ComponentRef::toString(e.rhs.clone())?).clone();
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
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following output from GraphViz OpenModelica assistant cannot be parsed:")); __mm_s.push_str(&*stringDelimitList(bad.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\nExpected format from GrapViz: cref1|cref2#cref3|cref4#. Ignoring malformed input.\n")); ArcStr::from(__mm_s) }).clone());
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
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cr.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

fn buildRootedTable(mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut graph: NFOCConnectionGraph) -> Result<CrefIndexTable> {
    let mut rooted: CrefIndexTable;
    let mut table: CrefRootsTable;
    table = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    List::map1_0(getBranches(graph.clone())?, (std::sync::Arc::new(addBranches) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>), Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<()> + 'static>), table.clone())?;
    List::map1_0(getConnections(graph)?, (std::sync::Arc::new(addConnectionsRooted) as std::sync::Arc<dyn ::std::ops::Fn(NFConnections::BrokenEdge, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<()> + 'static>), table.clone())?;
    rooted = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    setRootDistance(roots, table, 0, metamodelica::nil(), rooted.clone())?;
    Ok(rooted)
}

fn setRootDistance(mut finalRoots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut table: CrefRootsTable, mut distance: i32, mut nextLevel: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut rooted: CrefIndexTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((finalRoots, nextLevel.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            setRootDistance(nextLevel, table, distance + 1, metamodelica::nil(), rooted.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _) if (!(UnorderedMap::contains(cr.clone(), rooted.clone())?)) => {
            let mut next: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            UnorderedMap::addNew(cr.clone(), distance, rooted.clone())?;
            next = (::match_deref::match_deref! { match &(UnorderedMap::get(cr.clone(), table.clone())?) {
        Some(__esc_next) => {
            next = (*__esc_next).clone();
            listAppend(nextLevel, next.clone())
        },
        _ => nextLevel,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            setRootDistance(rest.clone(), table, distance, next.clone(), rooted.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            setRootDistance(rest.clone(), table, distance, nextLevel, rooted.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn addBranches(mut edge: Edge, mut table: CrefRootsTable) -> Result<()> {
    let mut cref1: Arc<ComponentRef::NFComponentRef>;
    let mut cref2: Arc<ComponentRef::NFComponentRef>;
    (cref1, cref2) = edge;
    addConnectionRooted(cref1.clone(), cref2.clone(), table.clone())?;
    addConnectionRooted(cref2, cref1, table)?;
    Ok(())
}

fn addConnectionsRooted(mut connection: FlatEdge, mut table: CrefRootsTable) -> Result<()> {
    addConnectionRooted(connection.lhs.clone(), connection.rhs.clone(), table.clone())?;
    addConnectionRooted(connection.rhs.clone(), connection.lhs.clone(), table)?;
    Ok(())
}

fn addConnectionRooted(mut cref1: Arc<ComponentRef::NFComponentRef>, mut cref2: Arc<ComponentRef::NFComponentRef>, mut table: CrefRootsTable) -> Result<()> {
    pub(crate) fn updateRooted(mut roots: Option<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>, mut newRoot: Arc<ComponentRef::NFComponentRef>) -> DefiniteRoots {
        let mut outRoots: DefiniteRoots = metamodelica::nil();
        outRoots = (::match_deref::match_deref! { match &(roots) {
        Some(__esc_outRoots) => {
            outRoots = (*__esc_outRoots).clone();
            metamodelica::cons(newRoot, outRoots.clone())
        },
        _ => list![newRoot],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outRoots
    }

    UnorderedMap::addUpdate(cref1.clone(), (std::sync::Arc::new({ let __pe_b1 = cref2.clone(); move |__pe_a0| Ok(updateRooted(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> + 'static>), table.clone())?;
    Ok(())
}

fn evalConnectionsOperatorsEqs(mut inRoots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut rooted: CrefIndexTable, mut graph: NFOCConnectionGraph, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    equations = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (equations).into_iter().cloned() {
            let __x = Equation::mapExpShallow(eq.clone(), (std::sync::Arc::new({ let __pe_b1 = rooted.clone(); let __pe_b2 = inRoots.clone(); let __pe_b3 = graph.clone(); let __pe_b4 = Equation::info(eq.clone())?; move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(equations)
}

fn evalConnectionsOperatorsVar(mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut rooted: CrefIndexTable, mut graph: NFOCConnectionGraph, mut var: Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    assign_field!(var.binding = Binding::mapExpShallow(var.binding.clone(), (std::sync::Arc::new({ let __pe_b1 = rooted; let __pe_b2 = roots; let __pe_b3 = graph; let __pe_b4 = var.info.clone(); move |__pe_a0| evaluateOperators(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
    Ok(var)
}

fn evaluateOperators(mut exp: Arc<Expression::NFExpression>, mut rooted: CrefIndexTable, mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut graph: NFOCConnectionGraph, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp, (std::sync::Arc::new({ let __pe_b1 = rooted; let __pe_b2 = roots; let __pe_b3 = graph; let __pe_b4 = info; move |__pe_a0| evalConnectionsOperatorsHelper(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

fn evalConnectionsOperatorsHelper(mut exp: Arc<Expression::NFExpression>, mut rooted: CrefIndexTable, mut roots: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut graph: NFOCConnectionGraph, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut uroots: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut nodes: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut message: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut cref1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut result: bool = false;
            let mut branches: Edges = metamodelica::nil();
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            (match identifyConnectionsOperator(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())) {
        ConnectionsOperator::ROOTED => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        _ if (Expression::isEmptyArray(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(" = false\n")); ArcStr::from(__mm_s) }).clone());
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: false })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::List::Nil } => {
            cref = (*__esc_cref).clone();
            branches = getBranches(graph)?;
            cref = ComponentRef::stripIteratorSubscripts(cref.clone())?;
            match '__try0: {
                cref1 = unwrap_break_err!(getEdge(cref.clone(), branches.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: Found Branche Partner ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(cref.clone()), '__try0)); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(cref1.clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                result = getRooted(cref.clone(), cref1.clone(), rooted.clone());
                if unwrap_break_err!(Flags::isSet(Flags::CGRAPH.clone()), '__try0) {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*unwrap_break_err!(Expression::toString(exp.clone()), '__try0)); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                Ok::<_, anyhow::Error>((result.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    result = __try0_o0;
                }
                Err(_) => {
                    r#str = (ComponentRef::toString(cref.clone())?).clone();
                    Error::addSourceMessage(Error::OCG_MISSING_BRANCH.clone(), list![(r#str.clone()).clone(), (r#str.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    result = false;
                }
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: result.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res.clone()
        },
        ConnectionsOperator::IS_ROOT => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        _ if (Expression::isEmptyArray(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)) => {
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(" = false\n")); ArcStr::from(__mm_s) }).clone());
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: false })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __esc_cref, .. }, tail: Deref @ metamodelica::List::Nil } => {
            cref = (*__esc_cref).clone();
            cref = ComponentRef::stripIteratorSubscripts(cref.clone())?;
            result = List::isMemberOnTrue(cref.clone(), roots, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*boolString(result.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            Arc::new(Expression::NFExpression::BOOLEAN { value: result.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res.clone()
        },
        ConnectionsOperator::UNIQUE_ROOT_INDICES => {
            res = (::match_deref::match_deref! { match &(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_uroots, tail: Deref @ metamodelica::List::Cons { head: __esc_nodes, tail: Deref @ metamodelica::List::Cons { head: __esc_message, tail: Deref @ metamodelica::List::Nil } } } => {
            uroots = (*__esc_uroots).clone();
            nodes = (*__esc_nodes).clone();
            message = (*__esc_message).clone();
            if Flags::isSet(Flags::CGRAPH.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.evalConnectionsOperatorsHelper: Connections.uniqueRootsIndices(")); __mm_s.push_str(&*Expression::toString(uroots.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*Expression::toString(nodes.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
            }
            dim = Type::nthDimension(Expression::typeOf(uroots.clone()), 1)?;
            if !(Dimension::isKnown(dim.clone(), false)) {
                Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(Expression::toString(exp)?).clone()], info)?;
                bail!("fail");
            }
            Expression::fillArray(Dimension::size(dim.clone(), false)?, Arc::new(Expression::NFExpression::INTEGER { value: 1 }))?
        },
        _ => bail!("match: no arm matched"),
    } });
            res.clone()
        },
        _ => exp,
    })
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn getRooted(mut cref1: Arc<ComponentRef::NFComponentRef>, mut cref2: Arc<ComponentRef::NFComponentRef>, mut rooted: CrefIndexTable) -> bool {
    let mut result: bool;
    result = 'mc: {
        let __mc_input = rooted.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut i1: i32;
                    let mut i2: i32;
                    i1 = UnorderedMap::getOrFail(cref1.clone(), rooted.clone())?;
                    i2 = UnorderedMap::getOrFail(cref2.clone(), rooted.clone())?;
                    Ok(intLt(i1.clone(), i2.clone()))
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
    result
}

fn getEdge(mut cr: Arc<ComponentRef::NFComponentRef>, mut edges: Edges) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut ocr: Arc<ComponentRef::NFComponentRef>;
    let mut cref1: Arc<ComponentRef::NFComponentRef>;
    let mut cref2: Arc<ComponentRef::NFComponentRef>;
    for mut edge in &*edges {
        let mut edge = edge.clone();
        (cref1, cref2) = edge.clone();
        if ComponentRef::isEqual(cr.clone(), cref1.clone())? {
            ocr = cref2.clone();
            return Ok(ocr.clone());
        } else if ComponentRef::isEqual(cr.clone(), cref2.clone())? {
            ocr = cref1.clone();
            return Ok(ocr.clone());
        }
    }
    bail!("fail");
    Ok(ocr)
}

fn printConnectionStr(mut edge: FlatEdge, mut ty: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ty); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(edge.lhs.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(edge.rhs.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(outStr)
}

fn printEdges(mut inEdges: Edges) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inEdges) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (c1, c2), tail: tail } => {
            metamodelica::print((literal!("    ")).clone());
            metamodelica::print((ComponentRef::toString(c1.clone())?).clone());
            metamodelica::print((literal!(" -- ")).clone());
            metamodelica::print((ComponentRef::toString(c2.clone())?).clone());
            metamodelica::print((literal!("\n")).clone());
            printEdges(tail.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printFlatEdges(mut inEdges: FlatEdges) -> Result<()> {
    for mut edge in &*inEdges {
        let mut edge = edge.clone();
        metamodelica::print((literal!("    ")).clone());
        metamodelica::print((ComponentRef::toString(edge.lhs.clone())?).clone());
        metamodelica::print((literal!(" -- ")).clone());
        metamodelica::print((ComponentRef::toString(edge.rhs.clone())?).clone());
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

fn printNFOCConnectionGraph(mut inGraph: NFOCConnectionGraph) -> Result<()> {
    let () = (match inGraph {
        NFOCConnectionGraph { connections: mut connections, branches: mut branches, .. } => {
            metamodelica::print((literal!("Connections:\n")).clone());
            printFlatEdges(connections.clone())?;
            metamodelica::print((literal!("Branches:\n")).clone());
            printEdges(branches.clone())?;
            ()
        },
    });
    Ok(())
}

fn getDefiniteRoots(mut inGraph: NFOCConnectionGraph) -> Result<DefiniteRoots> {
    let mut outResult: DefiniteRoots;
    outResult = (match inGraph {
        NFOCConnectionGraph { definiteRoots: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getUniqueRoots(mut inGraph: NFOCConnectionGraph) -> Result<UniqueRoots> {
    let mut outResult: UniqueRoots;
    outResult = (match inGraph {
        NFOCConnectionGraph { uniqueRoots: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getPotentialRoots(mut inGraph: NFOCConnectionGraph) -> Result<PotentialRoots> {
    let mut outResult: PotentialRoots;
    outResult = (match inGraph {
        NFOCConnectionGraph { potentialRoots: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getBranches(mut inGraph: NFOCConnectionGraph) -> Result<Edges> {
    let mut outResult: Edges;
    outResult = (match inGraph {
        NFOCConnectionGraph { branches: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn getConnections(mut inGraph: NFOCConnectionGraph) -> Result<FlatEdges> {
    let mut outResult: FlatEdges;
    outResult = (match inGraph {
        NFOCConnectionGraph { connections: ref result, .. } => {
            result.clone()
        },
    });
    Ok(outResult)
}

fn merge(mut inGraph1: NFOCConnectionGraph, mut inGraph2: NFOCConnectionGraph) -> Result<NFOCConnectionGraph> {
    let mut outGraph: NFOCConnectionGraph;
    outGraph = (::match_deref::match_deref! { match &((inGraph1.clone(), inGraph2.clone())) {
        (_, NFOCConnectionGraph { definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, .. }) => {
            inGraph1.clone()
        },
        (NFOCConnectionGraph { definiteRoots: Deref @ metamodelica::List::Nil, potentialRoots: Deref @ metamodelica::List::Nil, uniqueRoots: Deref @ metamodelica::List::Nil, branches: Deref @ metamodelica::List::Nil, connections: Deref @ metamodelica::List::Nil, .. }, _) => {
            inGraph2.clone()
        },
        (_, _) if (inGraph1.clone() == inGraph2.clone()) => {
            inGraph1.clone()
        },
        (NFOCConnectionGraph { updateGraph: updateGraph1, definiteRoots: definiteRoots1, potentialRoots: potentialRoots1, uniqueRoots: uniqueRoots1, branches: branches1, connections: connections1 }, NFOCConnectionGraph { updateGraph: updateGraph2, definiteRoots: definiteRoots2, potentialRoots: potentialRoots2, uniqueRoots: uniqueRoots2, branches: branches2, connections: connections2 }) => {
            let mut updateGraph: bool;
            let mut definiteRoots: DefiniteRoots;
            let mut uniqueRoots: UniqueRoots;
            let mut potentialRoots: PotentialRoots;
            let mut branches: Edges;
            let mut connections: FlatEdges;
            if Flags::isSet(Flags::CGRAPH.clone())? {
                Debug::trace((literal!("- NFOCConnectionGraph.merge()\n")).clone())?;
            }
            updateGraph = boolOr(updateGraph1.clone(), updateGraph2.clone());
            definiteRoots = List::union(definiteRoots1.clone(), definiteRoots2.clone());
            potentialRoots = List::union(potentialRoots1.clone(), potentialRoots2.clone());
            uniqueRoots = List::union(uniqueRoots1.clone(), uniqueRoots2.clone());
            branches = List::union(branches1.clone(), branches2.clone());
            connections = List::union(connections1.clone(), connections2.clone());
            NFOCConnectionGraph { updateGraph: updateGraph.clone(), definiteRoots: definiteRoots.clone(), potentialRoots: potentialRoots.clone(), uniqueRoots: uniqueRoots.clone(), branches: branches.clone(), connections: connections.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
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
            strEdge = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c1.clone())?); __mm_s.push_str(&*literal!("\" -- \"")); __mm_s.push_str(&*ComponentRef::toString(c2.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [color = blue, dir = \"none\", fontcolor=blue, label = \"branch\"];\n\t")); ArcStr::from(__mm_s) }).clone();
            strEdge.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn graphVizFlatEdge(mut edge: FlatEdge, mut inBrokenFlatEdges: FlatEdges) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut sc1: ArcStr;
    let mut sc2: ArcStr;
    let mut label: ArcStr;
    let mut labelFontSize: ArcStr;
    let mut decorate: ArcStr;
    let mut color: ArcStr;
    let mut style: ArcStr;
    let mut fontColor: ArcStr;
    let mut isBroken: bool;
    isBroken = List::isMemberOnTrue(edge.clone(), inBrokenFlatEdges, (std::sync::Arc::new(FlatEdgeIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(NFConnections::BrokenEdge, NFConnections::BrokenEdge) -> Result<bool> + 'static>))?;
    label = (if (isBroken) {literal!("[[broken connect]]")} else {literal!("connect")}).clone();
    color = (if (isBroken) {literal!("red")} else {literal!("green")}).clone();
    style = (if (isBroken) {literal!("\"bold, dashed\"")} else {literal!("solid")}).clone();
    decorate = (boolString(isBroken)).clone();
    fontColor = (if (isBroken) {literal!("red")} else {literal!("green")}).clone();
    labelFontSize = (if (isBroken) {literal!("labelfontsize = 20.0, ")} else {literal!("")}).clone();
    sc1 = (ComponentRef::toString(edge.lhs.clone())?).clone();
    sc2 = (ComponentRef::toString(edge.rhs.clone())?).clone();
    out = stringAppendList(list![(literal!("\"")).clone(), (sc1).clone(), (literal!("\" -- \"")).clone(), (sc2).clone(), (literal!("\" [")).clone(), (literal!("dir = \"none\", ")).clone(), (literal!("style = ")).clone(), (style).clone(), (literal!(", ")).clone(), (literal!("decorate = ")).clone(), (decorate).clone(), (literal!(", ")).clone(), (literal!("color = ")).clone(), (color).clone(), (literal!(", ")).clone(), (labelFontSize).clone(), (literal!("fontcolor = ")).clone(), (fontColor).clone(), (literal!(", ")).clone(), (literal!("label = \"")).clone(), (label).clone(), (literal!("\"")).clone(), (literal!("];\n\t")).clone()]);
    Ok(out)
}

fn FlatEdgeIsEqual(mut inEdge1: FlatEdge, mut inEdge2: FlatEdge) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = ComponentRef::isEqual(inEdge1.lhs.clone(), inEdge2.lhs.clone())? && ComponentRef::isEqual(inEdge1.rhs.clone(), inEdge2.rhs.clone())?;
    Ok(isEqual)
}

fn graphVizDefiniteRoot(mut inDefiniteRoot: DefiniteRoot, mut inFinalRoots: DefiniteRoots) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = ((::match_deref::match_deref! { match &(inDefiniteRoot) {
        c => {
            let mut strDefiniteRoot: ArcStr;
            let mut isSelectedRoot: bool;
            isSelectedRoot = List::isMemberOnTrue(c.clone(), inFinalRoots, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            strDefiniteRoot = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = red, rank = \"source\", label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot.clone()) {literal!("shape=polygon, sides=8, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) }).clone();
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
            isSelectedRoot = List::isMemberOnTrue(c.clone(), inFinalRoots, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            strPotentialRoot = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*literal!(" [fillcolor = orangered, rank = \"min\" label = ")); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(c.clone())?); __mm_s.push_str(&*literal!("\\n")); __mm_s.push_str(&*realString(priority.clone())); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*if (isSelectedRoot.clone()) {literal!("shape=ploygon, sides=7, distortion=\"0.265084\", orientation=26, skew=\"0.403659\"")} else {literal!("shape=box")}); __mm_s.push_str(&*literal!("];\n\t")); ArcStr::from(__mm_s) }).clone();
            strPotentialRoot.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(out)
}

fn generateGraphViz(mut modelNameQualified: ArcStr, mut definiteRoots: DefiniteRoots, mut potentialRoots: PotentialRoots, mut uniqueRoots: UniqueRoots, mut branches: Edges, mut connections: FlatEdges, mut finalRoots: DefiniteRoots, mut broken: FlatEdges) -> Result<ArcStr> {
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
                    infoNode = list![(literal!("// Generated by OpenModelica.\n")).clone(), (literal!("// Overconstrained connection graph for model:\n//    ")).clone(), (modelNameQualified.clone()).clone(), (literal!("\n")).clone(), (literal!("//\n")).clone(), (literal!("// Summary:\n")).clone(), (literal!("//   Roots:                      ")).clone(), (nrDR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Potential Roots:    ")).clone(), (nrPR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Unique Roots:       ")).clone(), (nrUR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Branches:           ")).clone(), (nrBR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Connections:        ")).clone(), (nrCO.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Final Roots:        ")).clone(), (nrFR.clone()).clone(), (literal!("\n")).clone(), (literal!("//   Broken Connections: ")).clone(), (nrBC.clone()).clone(), (literal!("\n")).clone()];
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
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(definiteRoots.clone(), (std::sync::Arc::new(graphVizDefiniteRoot) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> + 'static>), finalRoots.clone())?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Potential Roots (Connections.potentialRoot)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(potentialRoots.clone(), (std::sync::Arc::new(graphVizPotentialRoot) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, metamodelica::Real), Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> + 'static>), finalRoots.clone())?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Branches (Connections.branch)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map(branches.clone(), (std::sync::Arc::new(graphVizEdge) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)) -> Result<ArcStr> + 'static>))?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n")).clone(), (i.clone()).clone(), (literal!("// Connections (connect)")).clone(), (literal!("\n")).clone(), (i.clone()).clone()])?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), List::map1(connections.clone(), (std::sync::Arc::new(graphVizFlatEdge) as std::sync::Arc<dyn ::std::ops::Fn(NFConnections::BrokenEdge, Arc<metamodelica::List<NFConnections::BrokenEdge>>) -> Result<ArcStr> + 'static>), broken.clone())?)?;
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n}\n")).clone()])?;
                    tEnd = clock();
                    t = tEnd.clone() - tStart.clone();
                    timeStr = (realString(t.clone())).clone();
                    graphVizStream = IOStream::appendList(graphVizStream.clone(), list![(literal!("\n\n\n// graph generation took: ")).clone(), (timeStr.clone()).clone(), (literal!(" seconds\n")).clone()])?;
                    System::writeFile((fileName.clone()).clone(), (IOStream::string(graphVizStream.clone())?).clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz with connection graph for model: ")); __mm_s.push_str(&*modelNameQualified.clone()); __mm_s.push_str(&*literal!(" was writen to file: ")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
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
            metamodelica::print((literal!("Tyring to start GraphViz *lefty* to visualize the graph. You need to have lefty in your PATH variable\n")).clone());
            metamodelica::print((literal!("Make sure you quit GraphViz *lefty* via Right Click->quit to be sure the process will be exited.\n")).clone());
            metamodelica::print((literal!("If you quit the GraphViz *lefty* window via X, please kill the process in task manager to continue.\n")).clone());
            omhome = (Settings::getInstallationDirectoryPath()?).clone();
            omhome = (System::stringReplace((omhome.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone())?).clone();
            leftyCMD = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("load('")); __mm_s.push_str(&*omhome.clone()); __mm_s.push_str(&*literal!("/share/omc/scripts/openmodelica.lefty');")); __mm_s.push_str(&*literal!("openmodelica.init();openmodelica.createviewandgraph('")); __mm_s.push_str(&*fileNameGraphViz.clone()); __mm_s.push_str(&*literal!("','file',null,null);txtview('off');")); ArcStr::from(__mm_s) }).clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Running command: ")); __mm_s.push_str(&*literal!("lefty -e ")); __mm_s.push_str(&*leftyCMD.clone()); __mm_s.push_str(&*literal!(" > ")); __mm_s.push_str(&*fileNameTraceRemovedConnections.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            leftyExitStatus = System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lefty -e ")); __mm_s.push_str(&*leftyCMD.clone()); ArcStr::from(__mm_s) }).clone(), (fileNameTraceRemovedConnections.clone()).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz *lefty* exited with status:")); __mm_s.push_str(&*intString(leftyExitStatus.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            brokenConnects = (System::readFile((fileNameTraceRemovedConnections.clone()).clone())?).clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("GraphViz OpenModelica assistant returned the following broken connects: ")); __mm_s.push_str(&*brokenConnects.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(brokenConnects.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(brokenConnectsViaGraphViz)
}

fn removeBrokenConnects(mut inEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut inConnected: FlatEdges, mut inBroken: FlatEdges, mut isDeleted: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    outEquations = ({
        let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(inBroken.clone()) {
        Deref @ metamodelica::List::Nil => {
            inEquations
        },
        _ => {
            let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut isThere: bool = false;
            let mut r#str: ArcStr;
            let mut source: Arc<DAE::ElementSource>;
            for mut eq in &*inEquations {
                let mut eq = eq.clone();
                eql = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { lhs: Deref @ Expression::CREF { ty: _, cref: __esc_lhs }, rhs: Deref @ Expression::CREF { ty: _, cref: __esc_rhs }, source: __esc_source, .. } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            source = (*__esc_source).clone();
            if !(isDeleted(lhs.clone())? || isDeleted(rhs.clone())?) {
                isThere = false;
                for mut b in &*inBroken.clone() {
                    let mut b = b.clone();
                    if ComponentRef::isEqual(b.lhs.clone(), lhs.clone())? && ComponentRef::isEqual(b.rhs.clone(), rhs.clone())? || ComponentRef::isEqual(b.rhs.clone(), lhs.clone())? && ComponentRef::isEqual(b.lhs.clone(), rhs.clone())? {
                        isThere = true;
                        break;
                    }
                }
            }
            if !(isThere.clone()) {
                eql = metamodelica::cons(eq.clone(), eql.clone());
            }
            eql.clone()
        },
        _ => metamodelica::cons(eq.clone(), eql.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            eql = metamodelica::Dangerous::listReverseInPlace(eql.clone());
            if Flags::isSet(Flags::CGRAPH.clone())? {
                r#str = (literal!("")).clone();
                for mut b in &*inBroken {
                    let mut b = b.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*ComponentRef::toString(b.lhs.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ComponentRef::toString(b.rhs.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                }
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFOCConnectionGraph.removeBrokenConnects:\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            eql.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(outEquations)
}

fn identifyConnectionsOperator(mut functionName: Arc<Absyn::Path>) -> ConnectionsOperator {
    let mut call: ConnectionsOperator;
    call = (::match_deref::match_deref! { match &(functionName) {
        Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name } } => {
            (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "branch" => ConnectionsOperator::BRANCH.clone(),
        Deref @ "root" => ConnectionsOperator::ROOT.clone(),
        Deref @ "potentialRoot" => ConnectionsOperator::POTENTIAL_ROOT.clone(),
        Deref @ "isRoot" => ConnectionsOperator::IS_ROOT.clone(),
        Deref @ "rooted" => ConnectionsOperator::ROOTED.clone(),
        Deref @ "uniqueRoot" => ConnectionsOperator::UNIQUE_ROOT.clone(),
        Deref @ "uniqueRootIndices" => ConnectionsOperator::UNIQUE_ROOT_INDICES.clone(),
        _ => ConnectionsOperator::NOT_OPERATOR.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "rooted" } => {
            ConnectionsOperator::ROOTED.clone()
        },
        _ => {
            ConnectionsOperator::NOT_OPERATOR.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

fn newCrefCrefTable() -> CrefCrefTable {
    let mut table: CrefCrefTable;
    table = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    table
}

