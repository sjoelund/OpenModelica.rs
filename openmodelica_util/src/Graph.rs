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

use crate::Error;
use openmodelica_util_datatypes_basic::List;

pub fn buildGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace, ArgType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNodes: Arc<metamodelica::List<NodeType>>, mut inEdgeFunc: Arc<dyn ::std::ops::Fn(NodeType, ArgType) -> Result<Arc<metamodelica::List<NodeType>>> + 'static>, mut inEdgeArg: ArgType) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EdgeFunc<NodeType: Clone + 'static, ArgType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, ArgType) -> Result<Arc<metamodelica::List<NodeType>>> + 'static>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outGraph = List::zip(inNodes.clone(), List::map1(inNodes, inEdgeFunc.clone(), inEdgeArg)?);
    Ok(outGraph)
}

pub fn emptyGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNodes: Arc<metamodelica::List<NodeType>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outGraph = List::map(inNodes, std::sync::Arc::new(fnptr!(emptyGraphHelper, _)))?;
    Ok(outGraph)
}

fn emptyGraphHelper<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut nt: NodeType) -> (NodeType, Arc<metamodelica::List<NodeType>>) {
    let mut out: (NodeType, Arc<metamodelica::List<NodeType>>);
    out = (nt, metamodelica::nil());
    out
}

pub fn topologicalSort<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>)> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outNodes: Arc<metamodelica::List<NodeType>>;
    let mut outRemainingGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    let mut start_nodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    let mut rest_nodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    (rest_nodes, start_nodes) = List::splitOnTrue(inGraph, std::sync::Arc::new(fnptr!(hasOutgoingEdges, _)))?;
    (outNodes, outRemainingGraph) = topologicalSort2(start_nodes, rest_nodes, metamodelica::nil(), inEqualFunc.clone())?;
    Ok((outNodes, outRemainingGraph))
}

fn topologicalSort2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inStartNodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inRestNodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inAccumNodes: Arc<metamodelica::List<NodeType>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>)> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    '__tco: loop {
        ::match_deref::match_deref! { match &((inStartNodes, inRestNodes.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((inAccumNodes.reverse(), inRestNodes))
        },
        (rest_start, Deref @ metamodelica::List::Nil) => {
            let mut node1: NodeType;
            let mut result: Arc<metamodelica::List<NodeType>>;
            result = inAccumNodes;
            for mut n in &*rest_start.clone() {
                let mut n = n.clone();
                let __pa0 = ::match_deref::match_deref! { match &(n.clone()) {
                    (__pa0, Deref @ metamodelica::List::Nil) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                node1 = __pa0.clone();
                result = metamodelica::cons(node1.clone(), result.clone());
            }
            result = result.reverse();
            return Ok((result, metamodelica::nil()))
        },
        (Deref @ metamodelica::List::Cons { head: (node1, Deref @ metamodelica::List::Nil), tail: rest_start }, rest_rest) => {
            let mut rest_start_: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
            let mut new_start: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
            let mut result: Arc<metamodelica::List<NodeType>>;
            let mut rest_rest = (*rest_rest).clone();
            rest_rest = List::map2(rest_rest.clone(), (std::sync::Arc::new(removeEdge) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), node1.clone(), inEqualFunc.clone())?;
            (rest_rest, new_start) = List::splitOnTrue(rest_rest.clone(), std::sync::Arc::new(fnptr!(hasOutgoingEdges, _)))?;
            rest_start_ = listAppend(rest_start.clone(), new_start);
            { (inStartNodes, inRestNodes, inAccumNodes, inEqualFunc) = (rest_start_, rest_rest.clone(), metamodelica::cons(node1.clone(), inAccumNodes), inEqualFunc.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn hasOutgoingEdges<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>)) -> bool {
    let mut outHasOutEdges: bool;
    outHasOutEdges = (::match_deref::match_deref! { match &(inNode) {
        (_, Deref @ metamodelica::List::Nil) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHasOutEdges
}

fn removeEdge<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inRemovedNode: NodeType, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(NodeType, Arc<metamodelica::List<NodeType>>)> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outNode: (NodeType, Arc<metamodelica::List<NodeType>>);
    let mut node: NodeType;
    let mut edges: Arc<metamodelica::List<NodeType>>;
    (node, edges) = inNode;
    (edges, _) = List::deleteMemberOnTrue(inRemovedNode, edges, inEqualFunc.clone())?;
    outNode = (node, edges);
    Ok(outNode)
}

pub fn findCycles<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>;
    outCycles = findCycles2(inGraph.clone(), inGraph, inEqualFunc.clone())?;
    Ok(outCycles)
}

pub(crate) fn findCycles2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNodes: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>;
    outCycles = 'mc: {
        let __mc_input = inNodes;
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
                Deref @ metamodelica::List::Cons { head: node, tail: rest_nodes } => {
                    let mut cycle: Arc<metamodelica::List<NodeType>>;
                    let mut rest_cycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>;
                    let mut rest_nodes = (*rest_nodes).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(findCycleForNode(node.clone(), inGraph.clone(), metamodelica::nil(), inEqualFunc.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cycle = __pa0.clone();
                    rest_nodes = removeNodesFromGraph(cycle.clone(), rest_nodes.clone(), inEqualFunc.clone())?;
                    rest_cycles = findCycles2(rest_nodes.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    Ok(metamodelica::cons(cycle.clone(), rest_cycles.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_nodes } => {
                    let mut rest_cycles: Arc<metamodelica::List<Arc<metamodelica::List<NodeType>>>>;
                    rest_cycles = findCycles2(rest_nodes.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    Ok(rest_cycles.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCycles)
}

fn findCycleForNode<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inVisitedNodes: Arc<metamodelica::List<NodeType>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Option<Arc<metamodelica::List<NodeType>>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycle: Option<Arc<metamodelica::List<NodeType>>>;
    outCycle = 'mc: {
        let __mc_input = (inNode, inVisitedNodes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, _), Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut start_node: NodeType;
                    let mut is_start_node: bool;
                    let mut opt_cycle: Option<Arc<metamodelica::List<NodeType>>>;
                    let true = (List::isMemberOnTrue(node.clone(), inVisitedNodes.clone(), inEqualFunc.clone())?) else { bail!("pattern mismatch") };
                    start_node = List::last(inVisitedNodes.clone())?;
                    is_start_node = inEqualFunc(node.clone(), start_node.clone())?;
                    opt_cycle = if (is_start_node.clone()) {Some(inVisitedNodes.clone())} else {None};
                    Ok(opt_cycle.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((node, edges), _) => {
                    let mut visited_nodes: Arc<metamodelica::List<NodeType>>;
                    let mut cycle: Arc<metamodelica::List<NodeType>>;
                    visited_nodes = metamodelica::cons(node.clone(), inVisitedNodes.clone());
                    cycle = findCycleForNode2(edges.clone(), inGraph.clone(), visited_nodes.clone(), inEqualFunc.clone())?;
                    Ok(Some(cycle.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCycle)
}

fn findCycleForNode2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNodes: Arc<metamodelica::List<NodeType>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inVisitedNodes: Arc<metamodelica::List<NodeType>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outCycle: Arc<metamodelica::List<NodeType>>;
    outCycle = 'mc: {
        let __mc_input = inNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: node, tail: _ } => {
                    let mut cycle: Arc<metamodelica::List<NodeType>>;
                    let mut graph_node: (NodeType, Arc<metamodelica::List<NodeType>>);
                    graph_node = findNodeInGraph(node.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(findCycleForNode(graph_node.clone(), inGraph.clone(), inVisitedNodes.clone(), inEqualFunc.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cycle = __pa0.clone();
                    Ok(cycle.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_nodes } => {
                    let mut cycle: Arc<metamodelica::List<NodeType>>;
                    cycle = findCycleForNode2(rest_nodes.clone(), inGraph.clone(), inVisitedNodes.clone(), inEqualFunc.clone())?;
                    Ok(cycle.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCycle)
}

fn findNodeInGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: NodeType, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<(NodeType, Arc<metamodelica::List<NodeType>>)> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outNode: (NodeType, Arc<metamodelica::List<NodeType>>);
    outNode = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: graph_node @ (node, _), tail: _ } => {
                    let true = (inEqualFunc(inNode.clone(), node.clone())?) else { bail!("pattern mismatch") };
                    Ok(graph_node.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_graph } => {
                    Ok(findNodeInGraph(inNode.clone(), rest_graph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn findIndexofNodeInGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: NodeType, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inIndex: i32) -> Result<i32> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outIndex: i32;
    outIndex = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (node, _), tail: _ } => {
                    let true = (inEqualFunc(inNode.clone(), node.clone())?) else { bail!("pattern mismatch") };
                    Ok(inIndex)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_graph } => {
                    Ok(findIndexofNodeInGraph(inNode.clone(), rest_graph.clone(), inEqualFunc.clone(), inIndex + 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIndex)
}

fn removeNodesFromGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNodes: Arc<metamodelica::List<NodeType>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outGraph = 'mc: {
        let __mc_input = (inNodes.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (node, _), tail: rest_graph }) => {
                    let mut rest_nodes: Arc<metamodelica::List<NodeType>>;
                    let __pa0 = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(node.clone(), inNodes.clone(), inEqualFunc.clone())?) {
                        (__pa0, Some(_)) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rest_nodes = __pa0.clone();
                    Ok(removeNodesFromGraph(rest_nodes.clone(), rest_graph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: graph_node, tail: rest_graph }) => {
                    let mut rest_graph = (*rest_graph).clone();
                    rest_graph = removeNodesFromGraph(inNodes.clone(), rest_graph.clone(), inEqualFunc.clone())?;
                    Ok(metamodelica::cons(graph_node.clone(), rest_graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn transposeGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(mut intmpGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(intmpGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (node, nodeList), tail: restGraph } => {
                    let mut tmpGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
                    tmpGraph = List::fold2(nodeList.clone(), (std::sync::Arc::new(insertNodetoGraph) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _, _) -> Result<_> + 'static>), node.clone(), inEqualFunc.clone(), intmpGraph.clone())?;
                    tmpGraph = transposeGraph(tmpGraph.clone(), restGraph.clone(), inEqualFunc.clone())?;
                    Ok(tmpGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.transpose failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

fn insertNodetoGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(mut inNode: NodeType, mut inVertex: NodeType, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outGraph = 'mc: {
        let __mc_input = inGraph;
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
                Deref @ metamodelica::List::Cons { head: (node, rest), tail: restGraph } => {
                    let mut rest = (*rest).clone();
                    let mut restGraph = (*restGraph).clone();
                    let true = (inEqualFunc(node.clone(), inNode.clone())?) else { bail!("pattern mismatch") };
                    rest = List::unionList(list![rest.clone(), list![inVertex.clone()]])?;
                    restGraph = insertNodetoGraph(inNode.clone(), inVertex.clone(), inEqualFunc.clone(), restGraph.clone())?;
                    Ok(metamodelica::cons((node.clone(), rest.clone()), restGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (node, rest), tail: restGraph } => {
                    let mut restGraph = (*restGraph).clone();
                    let false = (inEqualFunc(node.clone(), inNode.clone())?) else { bail!("pattern mismatch") };
                    restGraph = insertNodetoGraph(inNode.clone(), inVertex.clone(), inEqualFunc.clone(), restGraph.clone())?;
                    Ok(metamodelica::cons((node.clone(), rest.clone()), restGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn allReachableNodes<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut intmpstorage: (Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<NodeType>>), mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<NodeType>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut reachableNodes: Arc<metamodelica::List<NodeType>>;
    let __pa0 = ::match_deref::match_deref! { match &(allReachableNodesWork(intmpstorage, inGraph, inEqualFunc.clone())?) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    reachableNodes = __pa0.clone();
    Ok(reachableNodes)
}

fn allReachableNodesWork<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut intmpstorage: (Arc<metamodelica::List<NodeType>>, Arc<metamodelica::List<NodeType>>), mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<Option<Arc<metamodelica::List<NodeType>>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut reachableNodes: Option<Arc<metamodelica::List<NodeType>>>;
    reachableNodes = 'mc: {
        let __mc_input = intmpstorage;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, L) => {
                    let mut L = (*L).clone();
                    L = L.clone().reverse();
                    Ok(Some(L.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: M }, L) => {
                    List::getMemberOnTrue(node.clone(), L.clone(), inEqualFunc.clone())?;
                    Ok(allReachableNodesWork((M.clone(), L.clone()), inGraph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: M }, L) => {
                    let mut edges: Arc<metamodelica::List<NodeType>>;
                    let mut M = (*M).clone();
                    let mut L = (*L).clone();
                    L = metamodelica::cons(node.clone(), L.clone());
                    (_, edges) = findNodeInGraph(node.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    M = listAppend(edges.clone(), M.clone());
                    Ok(allReachableNodesWork((M.clone(), L.clone()), inGraph.clone(), inEqualFunc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.allReachableNodes failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(reachableNodes)
}

pub(crate) fn partialDistance2color<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut toColorNodes: Arc<metamodelica::List<NodeType>>, mut inforbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inColors: Arc<metamodelica::List<i32>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inGraphT: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inColored: metamodelica::Array<i32>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>) -> Result<metamodelica::Array<i32>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type PrintFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>;

    let mut outColored: metamodelica::Array<i32>;
    outColored = 'mc: {
        let __mc_input = toColorNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inColored.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: node, tail: rest } => {
                    let mut nodes: Arc<metamodelica::List<NodeType>>;
                    let mut forbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>;
                    let mut colored: metamodelica::Array<i32>;
                    let mut color: i32;
                    let mut index: i32;
                    index = metamodelica::arrayLength(inColored.clone()) - (rest.clone().len() as i32);
                    (_, nodes) = findNodeInGraph(node.clone(), inGraphT.clone(), inEqualFunc.clone())?;
                    forbiddenColor = addForbiddenColors(node.clone(), nodes.clone(), inColored.clone(), inforbiddenColor.clone(), inGraph.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
                    color = arrayFindMinColorIndex(forbiddenColor.clone(), node.clone(), 1, metamodelica::arrayLength(inColored.clone()) + 1, inEqualFunc.clone(), inPrintFunc.clone())?;
                    colored = metamodelica::arrayUpdate(inColored.clone(), index.clone(), color.clone())?;
                    colored = partialDistance2color(rest.clone(), forbiddenColor.clone(), inColors.clone(), inGraph.clone(), inGraphT.clone(), colored.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
                    Ok(colored.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.partialDistance2color failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outColored)
}

fn addForbiddenColors<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: NodeType, mut inNodes: Arc<metamodelica::List<NodeType>>, mut inColored: metamodelica::Array<i32>, mut inForbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>) -> Result<metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type PrintFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>;

    let mut outForbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>;
    outForbiddenColor = 'mc: {
        let __mc_input = (inNodes, inForbiddenColor.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inForbiddenColor.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest }, forbiddenColor) => {
                    let mut nodes: Arc<metamodelica::List<NodeType>>;
                    let mut indexes: Arc<metamodelica::List<i32>>;
                    let mut indexesColor: Arc<metamodelica::List<i32>>;
                    let mut forbiddenColor1: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>;
                    (_, nodes) = findNodeInGraph(node.clone(), inGraph.clone(), inEqualFunc.clone())?;
                    indexes = List::map3(nodes.clone(), (std::sync::Arc::new(findIndexofNodeInGraph) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _, i32) -> Result<i32> + 'static>), inGraph.clone(), inEqualFunc.clone(), 1)?;
                    indexes = List::select1(indexes.clone(), (std::sync::Arc::new(arrayElemetGtZero) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), inColored.clone())?;
                    indexesColor = List::map1(indexes.clone(), (std::sync::Arc::new(getArrayElem) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), inColored.clone())?;
                    List::map2_0(indexesColor.clone(), (std::sync::Arc::new(arrayUpdateListAppend) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), forbiddenColor.clone(), Some(list![inNode.clone()]))?;
                    forbiddenColor1 = addForbiddenColors(inNode.clone(), rest.clone(), inColored.clone(), forbiddenColor.clone(), inGraph.clone(), inEqualFunc.clone(), inPrintFunc.clone())?;
                    Ok(forbiddenColor1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.addForbiddenColors failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outForbiddenColor)
}

fn getArrayElem<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(mut inIndex: i32, mut inArray: metamodelica::Array<Type_a>) -> Result<Type_a> {
    let mut outElem: Type_a;
    outElem = metamodelica::arrayGet(inArray.clone(), inIndex)?;
    Ok(outElem)
}

fn arrayUpdateListAppend<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inIndex: i32, mut inArray: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inNode: Option<Arc<metamodelica::List<NodeType>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inArray.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::arrayUpdate(inArray.clone(), inIndex, inNode.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.arrayUpdateListAppend failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn arrayElemetGtZero(mut inIndex: i32, mut inArray: metamodelica::Array<i32>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = intGt(metamodelica::arrayGet(inArray.clone(), inIndex)?, 0);
    Ok(outBoolean)
}

fn arrayFindMinColorIndex<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inForbiddenColor: metamodelica::Array<Option<Arc<metamodelica::List<NodeType>>>>, mut inNode: NodeType, mut inIndex: i32, mut inmaxIndex: i32, mut inEqualFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>) -> Result<i32> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type PrintFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<NodeType>>, ArcStr) -> Result<()> + 'static>;

    let mut outColor: i32;
    outColor = 'mc: {
        let __mc_input = inPrintFunc.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(metamodelica::arrayGet(inForbiddenColor.clone(), inIndex)?) {
                None => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(inIndex)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nodes: Arc<metamodelica::List<NodeType>>;
            let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(inForbiddenColor.clone(), inIndex)?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            nodes = __pa0.clone();
            if '__try1: {
                unwrap_break_err!(List::getMemberOnTrue(inNode.clone(), nodes.clone(), inEqualFunc.clone()), '__try1);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Ok(inIndex)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nodes: Arc<metamodelica::List<NodeType>>;
            let mut index: i32;
            let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(inForbiddenColor.clone(), inIndex)?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            nodes = __pa0.clone();
            List::getMemberOnTrue(inNode.clone(), nodes.clone(), inEqualFunc.clone())?;
            index = arrayFindMinColorIndex(inForbiddenColor.clone(), inNode.clone(), inIndex + 1, inmaxIndex, inEqualFunc.clone(), inPrintFunc.clone())?;
            Ok(index.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outColor)
}

pub fn printGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type NodeToString<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = stringDelimitList(List::map1(inGraph, (std::sync::Arc::new(printNode) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<ArcStr> + 'static>), inPrintFunc.clone())?, (literal!("\n")).clone());
    Ok(outString)
}

pub(crate) fn printNode<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inPrintFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type NodeToString<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    let mut node: NodeType;
    let mut edges: Arc<metamodelica::List<NodeType>>;
    let mut node_str: ArcStr;
    let mut edges_str: ArcStr;
    (node, edges) = inNode;
    node_str = (inPrintFunc(node)?).clone();
    edges_str = stringDelimitList(List::map(edges, inPrintFunc.clone())?, (literal!(", ")).clone());
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*node_str); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*edges_str); ArcStr::from(__mm_s) }).clone();
    Ok(outString)
}

/* Functions for Integer graphs */
pub fn printGraphInt(mut inGraph: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inGraph) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (node, edges), tail: restGraph } => {
            let mut strEdges: Arc<metamodelica::List<ArcStr>>;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node : ")); __mm_s.push_str(&*intString(node.clone())); __mm_s.push_str(&*literal!(" Edges: ")); ArcStr::from(__mm_s) }).clone());
            strEdges = List::map(edges.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            strEdges = List::map1(strEdges, (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" ")).clone())?;
            List::map_0(strEdges, Arc::new(fnptr!(print, ArcStr)))?;
            metamodelica::print((literal!("\n")).clone());
            printGraphInt(restGraph.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printNodesInt(mut inListNodes: Arc<metamodelica::List<i32>>, mut inName: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inListNodes.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            ()
        },
        _ => {
            let mut strNodes: Arc<metamodelica::List<ArcStr>>;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!(" : ")); ArcStr::from(__mm_s) }).clone());
            strNodes = List::map(inListNodes, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            strNodes = List::map1(strNodes, (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" ")).clone())?;
            List::map_0(strNodes, Arc::new(fnptr!(print, ArcStr)))?;
            metamodelica::print((literal!("\n")).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn allReachableNodesInt(mut intmpstorage: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), mut inGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>, mut inMaxGraphNode: i32, mut inMaxNodexIndex: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut reachableNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    reachableNodes = 'mc: {
        let __mc_input = intmpstorage;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, L) => {
                    Ok(L.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: M }, L) => {
                    let mut edges: Arc<metamodelica::List<i32>>;
                    let mut M = (*M).clone();
                    let mut L = (*L).clone();
                    let mut reachableNodes: Arc<metamodelica::List<i32>> = reachableNodes.clone();
                    L = List::union(L.clone(), list![node.clone()]);
                    let false = (intGe(node.clone(), inMaxGraphNode)) else { bail!("pattern mismatch") };
                    (_, edges) = metamodelica::arrayGet(inGraph.clone(), node.clone())?;
                    edges = List::filter1OnTrue(edges.clone(), std::sync::Arc::new(fnptr!(List::notMember, _, _)), L.clone())?;
                    M = List::union(M.clone(), edges.clone());
                    reachableNodes = allReachableNodesInt((M.clone(), L.clone()), inGraph.clone(), inMaxGraphNode, inMaxNodexIndex)?;
                    Ok((reachableNodes.clone(), reachableNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { reachableNodes = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: M }, L) => {
                    let mut L = (*L).clone();
                    let mut reachableNodes: Arc<metamodelica::List<i32>> = reachableNodes.clone();
                    L = List::union(L.clone(), list![node.clone()]);
                    let true = (intGe(node.clone(), inMaxGraphNode)) else { bail!("pattern mismatch") };
                    reachableNodes = allReachableNodesInt((M.clone(), L.clone()), inGraph.clone(), inMaxGraphNode, inMaxNodexIndex)?;
                    Ok((reachableNodes.clone(), reachableNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { reachableNodes = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.allReachableNodesInt failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(reachableNodes)
}

pub fn partialDistance2colorInt(mut inGraphT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut inforbiddenColor: metamodelica::Array<i32>, mut inColors: Arc<metamodelica::List<i32>>, mut inGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>, mut inColored: metamodelica::Array<i32>) -> Result<()> {
    let mut node: i32;
    let mut color: i32;
    let mut nodes: Arc<metamodelica::List<i32>>;
    if '__try0: {
        for mut tpl in &*inGraphT.clone() {
            let mut tpl = tpl.clone();
            (node, nodes) = tpl.clone();
            unwrap_break_err!(addForbiddenColorsInt(node, nodes.clone(), inColored.clone(), inforbiddenColor.clone(), inGraph.clone()), '__try0);
            color = unwrap_break_err!(arrayFindMinColorIndexInt(inforbiddenColor.clone(), node), '__try0);
            unwrap_break_err!(metamodelica::arrayUpdate(inColored.clone(), node, color), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.partialDistance2colorInt failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
    }
    Ok(())
}

fn addForbiddenColorsInt(mut inNode: i32, mut nodes: Arc<metamodelica::List<i32>>, mut inColored: metamodelica::Array<i32>, mut forbiddenColor: metamodelica::Array<i32>, mut inGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>) -> Result<()> {
    let mut indexes: Arc<metamodelica::List<i32>>;
    match '__try0: {
        for mut node in &*nodes.clone() {
            let mut node = node.clone();
            (_, indexes) = unwrap_break_err!(metamodelica::arrayGet(inGraph.clone(), node.clone()), '__try0);
            unwrap_break_err!(updateForbiddenColorArrayInt(indexes.clone(), inColored.clone(), forbiddenColor.clone(), inNode), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Graph.addForbiddenColorsInt failed.")).clone()], metamodelica::sourceInfo!("Util/Graph.mo"))?;
            return Err(__try0_err);
        }
    }
    Ok(())
}

fn updateForbiddenColorArrayInt(mut inIndexes: Arc<metamodelica::List<i32>>, mut inColored: metamodelica::Array<i32>, mut inForbiddenColor: metamodelica::Array<i32>, mut inNode: i32) -> Result<()> {
    let mut colorIndex: i32;
    for mut index in &*inIndexes {
        let mut index = index.clone();
        colorIndex = metamodelica::arrayGet(inColored.clone(), index.clone())?;
        if colorIndex > 0 {
            metamodelica::arrayUpdate(inForbiddenColor.clone(), colorIndex, inNode)?;
        }
    }
    Ok(())
}

fn arrayFindMinColorIndexInt(mut inForbiddenColor: metamodelica::Array<i32>, mut inNode: i32) -> Result<i32> {
    let mut outColor: i32 = 1;
    loop {
        if metamodelica::arrayGet(inForbiddenColor.clone(), outColor)? != inNode {
            return Ok(outColor.clone());
        } else {
            outColor = outColor + 1;
        }
    }
    Ok(outColor)
}

pub fn filterGraph<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut inCondFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type CondFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>;

    let mut outGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outGraph = List::accumulateMapAccum(inGraph, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = inCondFunc.clone(); move |__pe_a0, __pe_a2| filterGraph2(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>))?;
    Ok(outGraph)
}

fn filterGraph2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inNode: (NodeType, Arc<metamodelica::List<NodeType>>), mut inCondFunc: Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>, mut inAccumGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type CondFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType) -> Result<bool> + 'static>;

    let mut outNode: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    outNode = 'mc: {
        let __mc_input = inNode;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (node, _) => {
                    let false = (inCondFunc(node.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAccumGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (node, edges) => {
                    let mut edges = (*edges).clone();
                    edges = List::filterOnTrue(edges.clone(), inCondFunc.clone())?;
                    Ok(metamodelica::cons((node.clone(), edges.clone()), inAccumGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

pub fn merge<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut graph1: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut graph2: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut eqFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut compareFunc: Arc<dyn ::std::ops::Fn((NodeType, Arc<metamodelica::List<NodeType>>), (NodeType, Arc<metamodelica::List<NodeType>>)) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    pub type CompareFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn((NodeType, Arc<metamodelica::List<NodeType>>), (NodeType, Arc<metamodelica::List<NodeType>>)) -> Result<bool> + 'static>;

    let mut graph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    graph = merge2(List::sort(listAppend(graph1, graph2), compareFunc.clone())?, eqFunc.clone(), metamodelica::nil())?;
    Ok(graph)
}

fn merge2<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut inGraph: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut eqFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>, mut inAcc: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>) -> Result<Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    '__tco: loop {
        ::match_deref::match_deref! { match &(inGraph) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inAcc.reverse())
        },
        Deref @ metamodelica::List::Cons { head: node, tail: Deref @ metamodelica::List::Nil } => {
            return Ok(metamodelica::cons(node.clone(), inAcc).reverse())
        },
        Deref @ metamodelica::List::Cons { head: (n1, e1), tail: Deref @ metamodelica::List::Cons { head: (n2, e2), tail: rest } } => {
            let mut node: (NodeType, Arc<metamodelica::List<NodeType>>);
            let mut b: bool;
            let mut rest = (*rest).clone();
            b = eqFunc(n1.clone(), n2.clone())?;
            (node, rest) = merge3(b, n1.clone(), e1.clone(), n2.clone(), e2.clone(), rest.clone(), eqFunc.clone())?;
            { (inGraph, eqFunc, inAcc) = (rest.clone(), eqFunc.clone(), metamodelica::cons(node.clone(), inAcc)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn merge3<NodeType: Clone + 'static + metamodelica::gc::MMTrace>(mut b: bool, mut n1: NodeType, mut e1: Arc<metamodelica::List<NodeType>>, mut n2: NodeType, mut e2: Arc<metamodelica::List<NodeType>>, mut rest: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>, mut eqFunc: Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>) -> Result<((NodeType, Arc<metamodelica::List<NodeType>>), Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>)> {
    pub type EqualFunc<NodeType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(NodeType, NodeType) -> Result<bool> + 'static>;

    let mut elt: (NodeType, Arc<metamodelica::List<NodeType>>);
    let mut outRest: Arc<metamodelica::List<(NodeType, Arc<metamodelica::List<NodeType>>)>>;
    (elt, outRest) = (match b {
        true => ((n1, List::unionOnTrue(e1, e2, eqFunc.clone())?), rest),
        false => ((n1, e1), metamodelica::cons((n2, e2), rest)),
    });
    Ok((elt, outRest))
}

