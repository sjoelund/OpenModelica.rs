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

use crate::GraphMLDumpTpl;
use openmodelica_tpl::Tpl;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//TODO: Use HashTable for nodes to prevent duplicates
// -------------------------
// Constant types
// -------------------------
pub const COLOR_BLACK: &'static str = "000000";

pub const COLOR_BLUE: &'static str = "0000FF";

pub const COLOR_GREEN: &'static str = "339966";

pub const COLOR_RED: &'static str = "FF0000";

pub(crate) const COLOR_DARKRED: &'static str = "800000";

pub const COLOR_WHITE: &'static str = "FFFFFF";

pub const COLOR_YELLOW: &'static str = "FFFF00";

pub const COLOR_GRAY: &'static str = "C0C0C0";

pub const COLOR_PURPLE: &'static str = "993366";

pub const COLOR_ORANGE: &'static str = "FFCC00";

pub const COLOR_ORANGE2: &'static str = "FF6600";

pub(crate) const COLOR_DARKGRAY: &'static str = "666666";

pub const COLOR_RED2: &'static str = "F0988E";

pub const COLOR_GREEN2: &'static str = "98B954";

pub const COLOR_CYAN: &'static str = "46BED8";

pub const COLOR_PINK: &'static str = "CF8CB7";

pub(crate) const COLOR_GREEN3: &'static str = "008080";

pub const LINEWIDTH_STANDARD: metamodelica::Real = metamodelica::OrderedFloat(2.0_f64);

pub const LINEWIDTH_BOLD: metamodelica::Real = metamodelica::OrderedFloat(4.0_f64);

pub const FONTSIZE_STANDARD: i32 = 12;

pub(crate) const FONTSIZE_BIG: i32 = 20;

pub(crate) const FONTSIZE_SMALL: i32 = 8;

pub const BORDERWIDTH_STANDARD: metamodelica::Real = metamodelica::OrderedFloat(1.0_f64);

pub const BORDERWIDTH_BOLD: metamodelica::Real = metamodelica::OrderedFloat(4.0_f64);

// -------------------------
// Data structures
// -------------------------
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum GraphInfo {
    GRAPHINFO {
        graphs: Arc<metamodelica::List<Graph>>,
        graphCount: i32,
        nodes: Arc<metamodelica::List<Node>>,
        nodeCount: i32,
        edges: Arc<metamodelica::List<Edge>>,
        edgeCount: i32,
        attributes: Arc<metamodelica::List<Attribute>>,
        graphNodeKey: ArcStr,
        graphEdgeKey: ArcStr,
    },
    GRAPHINFOARR {
        graphs: metamodelica::Array<Graph>,
        nodes: metamodelica::Array<Node>,
        edges: Arc<metamodelica::List<Edge>>,
        attributes: metamodelica::Array<Attribute>,
        graphNodeKey: ArcStr,
        graphEdgeKey: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for GraphInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            GraphInfo::GRAPHINFO { graphs, graphCount, nodes, nodeCount, edges, edgeCount, attributes, graphNodeKey, graphEdgeKey } => {
                metamodelica::gc::MMTrace::mm_accept(graphs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(graphCount, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodeCount, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(edges, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(edgeCount, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(graphNodeKey, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(graphEdgeKey, __mmv)?;
                Ok(())
            }
            GraphInfo::GRAPHINFOARR { graphs, nodes, edges, attributes, graphNodeKey, graphEdgeKey } => {
                metamodelica::gc::MMTrace::mm_accept(graphs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(edges, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(graphNodeKey, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(graphEdgeKey, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for GraphInfo {
    fn default() -> Self {
        Self::GRAPHINFOARR {
            graphs: Default::default(),
            nodes: Default::default(),
            edges: Default::default(),
            attributes: Default::default(),
            graphNodeKey: Default::default(),
            graphEdgeKey: Default::default(),
        }
    }
}
pub use self::GraphInfo::{GRAPHINFO,GRAPHINFOARR};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Graph {
    pub id: ArcStr,
    pub directed: bool,
    pub nodeIdc: Arc<metamodelica::List<i32>>,
    pub attValues: Arc<metamodelica::List<(i32, ArcStr)>>,
}

impl metamodelica::gc::MMTrace for Graph {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.directed, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nodeIdc, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.attValues, __mmv)?;
        Ok(())
    }
}
impl Default for Graph {
    fn default() -> Self {
        Self {
            id: Default::default(),
            directed: Default::default(),
            nodeIdc: Default::default(),
            attValues: Default::default(),
        }
    }
}

pub type GRAPH = Graph;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Node {
    NODE {
        id: ArcStr,
        color: ArcStr,
        border: metamodelica::Real,
        nodeLabels: Arc<metamodelica::List<NodeLabel>>,
        shapeType: ShapeType,
        optDesc: Option<ArcStr>,
        attValues: Arc<metamodelica::List<(i32, ArcStr)>>,
    },
    GROUPNODE {
        id: ArcStr,
        internalGraphIdx: i32,
        isFolded: bool,
        header: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for Node {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Node::NODE { id, color, border, nodeLabels, shapeType, optDesc, attValues } => {
                metamodelica::gc::MMTrace::mm_accept(id, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(color, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(border, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodeLabels, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(shapeType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(optDesc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attValues, __mmv)?;
                Ok(())
            }
            Node::GROUPNODE { id, internalGraphIdx, isFolded, header } => {
                metamodelica::gc::MMTrace::mm_accept(id, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(internalGraphIdx, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFolded, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(header, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Node {
    fn default() -> Self {
        Self::GROUPNODE {
            id: Default::default(),
            internalGraphIdx: Default::default(),
            isFolded: Default::default(),
            header: Default::default(),
        }
    }
}
pub use self::Node::{NODE,GROUPNODE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Edge {
    pub id: ArcStr,
    pub target: ArcStr,
    pub source: ArcStr,
    pub color: ArcStr,
    pub lineType: LineType,
    pub lineWidth: metamodelica::Real,
    pub smooth: bool,
    pub edgeLabels: Arc<metamodelica::List<EdgeLabel>>,
    pub arrows: (ArrowType, ArrowType),
    pub attValues: Arc<metamodelica::List<(i32, ArcStr)>>,
}

impl metamodelica::gc::MMTrace for Edge {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.target, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.color, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineWidth, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.smooth, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.edgeLabels, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arrows, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.attValues, __mmv)?;
        Ok(())
    }
}
impl Default for Edge {
    fn default() -> Self {
        Self {
            id: Default::default(),
            target: Default::default(),
            source: Default::default(),
            color: Default::default(),
            lineType: Default::default(),
            lineWidth: Default::default(),
            smooth: Default::default(),
            edgeLabels: Default::default(),
            arrows: Default::default(),
            attValues: Default::default(),
        }
    }
}

pub type EDGE = Edge;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Attribute {
    pub attIdx: i32,
    pub defaultValue: ArcStr,
    pub name: ArcStr,
    pub attType: AttributeType,
    pub attTarget: AttributeTarget,
}

impl metamodelica::gc::MMTrace for Attribute {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.attIdx, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.defaultValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.attType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.attTarget, __mmv)?;
        Ok(())
    }
}
impl Default for Attribute {
    fn default() -> Self {
        Self {
            attIdx: Default::default(),
            defaultValue: Default::default(),
            name: Default::default(),
            attType: Default::default(),
            attTarget: Default::default(),
        }
    }
}

pub type ATTRIBUTE = Attribute;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NodeLabel {
    NODELABEL_INTERNAL {
        text: ArcStr,
        backgroundColor: Option<ArcStr>,
        fontStyle: FontStyle,
    },
    NODELABEL_CORNER {
        text: ArcStr,
        backgroundColor: Option<ArcStr>,
        fontStyle: FontStyle,
        position: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for NodeLabel {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NodeLabel::NODELABEL_INTERNAL { text, backgroundColor, fontStyle } => {
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(backgroundColor, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fontStyle, __mmv)?;
                Ok(())
            }
            NodeLabel::NODELABEL_CORNER { text, backgroundColor, fontStyle, position } => {
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(backgroundColor, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fontStyle, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(position, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NodeLabel {
    fn default() -> Self {
        Self::NODELABEL_INTERNAL {
            text: Default::default(),
            backgroundColor: Default::default(),
            fontStyle: Default::default(),
        }
    }
}
pub use self::NodeLabel::{NODELABEL_INTERNAL,NODELABEL_CORNER};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EdgeLabel {
    pub text: ArcStr,
    pub backgroundColor: Option<ArcStr>,
    pub fontSize: i32,
}

impl metamodelica::gc::MMTrace for EdgeLabel {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.text, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.backgroundColor, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fontSize, __mmv)?;
        Ok(())
    }
}
impl Default for EdgeLabel {
    fn default() -> Self {
        Self {
            text: Default::default(),
            backgroundColor: Default::default(),
            fontSize: Default::default(),
        }
    }
}

pub type EDGELABEL = EdgeLabel;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum FontStyle {
    FONTPLAIN,
    FONTBOLD,
    FONTITALIC,
    FONTBOLDITALIC,
}
impl metamodelica::gc::MMTrace for FontStyle {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FontStyle::FONTPLAIN => Ok(()),
            FontStyle::FONTBOLD => Ok(()),
            FontStyle::FONTITALIC => Ok(()),
            FontStyle::FONTBOLDITALIC => Ok(()),
        }
    }
}
impl Default for FontStyle {
    fn default() -> Self { Self::FONTPLAIN }
}
pub use self::FontStyle::{FONTPLAIN,FONTBOLD,FONTITALIC,FONTBOLDITALIC};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ShapeType {
    RECTANGLE,
    ROUNDRECTANGLE,
    ELLIPSE,
    PARALLELOGRAM,
    HEXAGON,
    TRIANGLE,
    OCTAGON,
    DIAMOND,
    TRAPEZOID,
    TRAPEZOID2,
}
impl metamodelica::gc::MMTrace for ShapeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ShapeType::RECTANGLE => Ok(()),
            ShapeType::ROUNDRECTANGLE => Ok(()),
            ShapeType::ELLIPSE => Ok(()),
            ShapeType::PARALLELOGRAM => Ok(()),
            ShapeType::HEXAGON => Ok(()),
            ShapeType::TRIANGLE => Ok(()),
            ShapeType::OCTAGON => Ok(()),
            ShapeType::DIAMOND => Ok(()),
            ShapeType::TRAPEZOID => Ok(()),
            ShapeType::TRAPEZOID2 => Ok(()),
        }
    }
}
impl Default for ShapeType {
    fn default() -> Self { Self::RECTANGLE }
}
pub use self::ShapeType::{RECTANGLE,ROUNDRECTANGLE,ELLIPSE,PARALLELOGRAM,HEXAGON,TRIANGLE,OCTAGON,DIAMOND,TRAPEZOID,TRAPEZOID2};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum LineType {
    LINE,
    DASHED,
    DASHEDDOTTED,
}
impl metamodelica::gc::MMTrace for LineType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            LineType::LINE => Ok(()),
            LineType::DASHED => Ok(()),
            LineType::DASHEDDOTTED => Ok(()),
        }
    }
}
impl Default for LineType {
    fn default() -> Self { Self::LINE }
}
pub use self::LineType::{LINE,DASHED,DASHEDDOTTED};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ArrowType {
    ARROWSTANDART,
    ARROWNONE,
    ARROWCONCAVE,
}
impl metamodelica::gc::MMTrace for ArrowType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ArrowType::ARROWSTANDART => Ok(()),
            ArrowType::ARROWNONE => Ok(()),
            ArrowType::ARROWCONCAVE => Ok(()),
        }
    }
}
impl Default for ArrowType {
    fn default() -> Self { Self::ARROWSTANDART }
}
pub use self::ArrowType::{ARROWSTANDART,ARROWNONE,ARROWCONCAVE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum AttributeType {
    TYPE_STRING,
    TYPE_BOOLEAN,
    TYPE_INTEGER,
    TYPE_DOUBLE,
}
impl metamodelica::gc::MMTrace for AttributeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            AttributeType::TYPE_STRING => Ok(()),
            AttributeType::TYPE_BOOLEAN => Ok(()),
            AttributeType::TYPE_INTEGER => Ok(()),
            AttributeType::TYPE_DOUBLE => Ok(()),
        }
    }
}
impl Default for AttributeType {
    fn default() -> Self { Self::TYPE_STRING }
}
pub use self::AttributeType::{TYPE_STRING,TYPE_BOOLEAN,TYPE_INTEGER,TYPE_DOUBLE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum AttributeTarget {
    TARGET_NODE,
    TARGET_EDGE,
    TARGET_GRAPH,
}
impl metamodelica::gc::MMTrace for AttributeTarget {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            AttributeTarget::TARGET_NODE => Ok(()),
            AttributeTarget::TARGET_EDGE => Ok(()),
            AttributeTarget::TARGET_GRAPH => Ok(()),
        }
    }
}
impl Default for AttributeTarget {
    fn default() -> Self { Self::TARGET_NODE }
}
pub use self::AttributeTarget::{TARGET_NODE,TARGET_EDGE,TARGET_GRAPH};

// -------------------------
// Logic
// -------------------------
pub fn createGraphInfo() -> GraphInfo {
    let mut oGraphInfo: GraphInfo;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: metamodelica::nil(), graphCount: 0, nodes: metamodelica::nil(), nodeCount: 0, edges: metamodelica::nil(), edgeCount: 0, attributes: metamodelica::nil(), graphNodeKey: (literal!("gi1")).clone(), graphEdgeKey: (literal!("gi2")).clone() };
    oGraphInfo
}

pub fn addGraph(mut id: ArcStr, mut directed: bool, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Graph, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oGraph: (Graph, i32);
    let mut tmpGraph: Graph;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    graphCount = graphCount + 1;
    tmpGraph = Graph { id: (id).clone(), directed: directed, nodeIdc: metamodelica::nil(), attValues: metamodelica::nil() };
    graphs = metamodelica::cons(tmpGraph.clone(), graphs);
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs, graphCount: graphCount, nodes: nodes, nodeCount: nodeCount, edges: edges, edgeCount: edgeCount, attributes: attributes, graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    oGraph = (tmpGraph, graphCount);
    Ok((oGraphInfo, oGraph))
}

pub fn addNode(mut id: ArcStr, mut backgroundColor: ArcStr, mut borderWidth: metamodelica::Real, mut nodeLabels: Arc<metamodelica::List<NodeLabel>>, mut shapeType: ShapeType, mut optDesc: Option<ArcStr>, mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>, mut iGraphIdx: i32, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Node, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oNode: (Node, i32);
    let mut tmpNode: Node;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let mut iGraph: Graph;
    let mut gid: ArcStr;
    let mut directed: bool;
    let mut nodeIdc: Arc<metamodelica::List<i32>>;
    let mut gAttValues: Arc<metamodelica::List<(i32, ArcStr)>>;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    iGraph = (graphs.clone()).get(graphCount - iGraphIdx + 1)?;
    let Graph { id: __pa9, directed: __pa10, nodeIdc: __pa11, attValues: __pa12 } = (iGraph) else { bail!("pattern mismatch") };
    gid = __pa9.clone();
    directed = __pa10.clone();
    nodeIdc = __pa11.clone();
    gAttValues = __pa12.clone();
    nodeCount = nodeCount + 1;
    tmpNode = Node::NODE { id: (id).clone(), color: (backgroundColor).clone(), border: borderWidth, nodeLabels: nodeLabels, shapeType: shapeType, optDesc: optDesc, attValues: attValues };
    nodes = metamodelica::cons(tmpNode.clone(), nodes);
    nodeIdc = metamodelica::cons(nodeCount, nodeIdc);
    iGraph = Graph { id: (gid).clone(), directed: directed, nodeIdc: nodeIdc, attValues: gAttValues };
    graphs = List::set(graphs, graphCount - iGraphIdx + 1, iGraph)?;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs, graphCount: graphCount, nodes: nodes, nodeCount: nodeCount, edges: edges, edgeCount: edgeCount, attributes: attributes, graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    oNode = (tmpNode, nodeCount);
    Ok((oGraphInfo, oNode))
}

pub fn addGroupNode(mut id: ArcStr, mut iGraphIdx: i32, mut isFolded: bool, mut iHeader: ArcStr, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Node, i32), (Graph, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oNode: (Node, i32);
    let mut oGraph: (Graph, i32);
    let mut tmpGraphInfo: GraphInfo;
    let mut tmpNode: Node;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let mut iGraph: Graph;
    let mut newGraph: Graph;
    let mut gid: ArcStr;
    let mut directed: bool;
    let mut newGraphIdx: i32;
    let mut nodeIdc: Arc<metamodelica::List<i32>>;
    let mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo.clone()) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    iGraph = (graphs).get(graphCount - iGraphIdx + 1)?;
    let Graph { id: __pa9, directed: __pa10, nodeIdc: __pa11, attValues: __pa12 } = (iGraph) else { bail!("pattern mismatch") };
    gid = __pa9.clone();
    directed = __pa10.clone();
    nodeIdc = __pa11.clone();
    attValues = __pa12.clone();
    let (__pa13, (__pa14, __pa15)) = addGraph(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("g")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone(), directed, iGraphInfo)?;
    tmpGraphInfo = __pa13.clone();
    newGraph = __pa14.clone();
    newGraphIdx = __pa15.clone();
    let GraphInfo::GRAPHINFO { graphs: __pa16, graphCount: __pa17, nodes: __pa18, nodeCount: __pa19, edges: __pa20, edgeCount: __pa21, attributes: __pa22, graphNodeKey: __pa23, graphEdgeKey: __pa24 } = (tmpGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa16.clone();
    graphCount = __pa17.clone();
    nodes = __pa18.clone();
    nodeCount = __pa19.clone();
    edges = __pa20.clone();
    edgeCount = __pa21.clone();
    attributes = __pa22.clone();
    graphNodeKey = __pa23.clone();
    graphEdgeKey = __pa24.clone();
    nodeCount = nodeCount + 1;
    tmpNode = Node::GROUPNODE { id: (id).clone(), internalGraphIdx: newGraphIdx, isFolded: isFolded, header: (iHeader).clone() };
    nodes = metamodelica::cons(tmpNode.clone(), nodes);
    nodeIdc = metamodelica::cons(nodeCount, nodeIdc);
    iGraph = Graph { id: (gid).clone(), directed: directed, nodeIdc: nodeIdc, attValues: attValues };
    graphs = List::set(graphs, graphCount - iGraphIdx + 1, iGraph)?;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs, graphCount: graphCount, nodes: nodes, nodeCount: nodeCount, edges: edges, edgeCount: edgeCount, attributes: attributes, graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    oNode = (tmpNode, nodeCount);
    oGraph = (newGraph, newGraphIdx);
    Ok((oGraphInfo, oNode, oGraph))
}

pub fn addEdge(mut id: ArcStr, mut target: ArcStr, mut source: ArcStr, mut color: ArcStr, mut lineType: LineType, mut lineWidth: metamodelica::Real, mut smooth: bool, mut labels: Arc<metamodelica::List<EdgeLabel>>, mut arrows: (ArrowType, ArrowType), mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Edge, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oEdge: (Edge, i32);
    let mut tmpEdge: Edge;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    edgeCount = edgeCount + 1;
    tmpEdge = Edge { id: (id).clone(), target: (target).clone(), source: (source).clone(), color: (color).clone(), lineType: lineType, lineWidth: lineWidth, smooth: smooth, edgeLabels: labels, arrows: arrows, attValues: attValues };
    edges = metamodelica::cons(tmpEdge.clone(), edges);
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs, graphCount: graphCount, nodes: nodes, nodeCount: nodeCount, edges: edges, edgeCount: edgeCount, attributes: attributes, graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    oEdge = (tmpEdge, edgeCount);
    Ok((oGraphInfo, oEdge))
}

pub fn addAttribute(mut defaultValue: ArcStr, mut name: ArcStr, mut attType: AttributeType, mut attTarget: AttributeTarget, mut iGraphInfo: GraphInfo) -> Result<(GraphInfo, (Attribute, i32))> {
    let mut oGraphInfo: GraphInfo;
    let mut oAttribute: (Attribute, i32);
    let mut tmpAttribute: Attribute;
    let mut attIdx: i32;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    attIdx = (attributes.clone().len() as i32) + 1;
    tmpAttribute = Attribute { attIdx: attIdx, defaultValue: (defaultValue).clone(), name: (name).clone(), attType: attType, attTarget: attTarget };
    attributes = metamodelica::cons(tmpAttribute.clone(), attributes);
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs, graphCount: graphCount, nodes: nodes, nodeCount: nodeCount, edges: edges, edgeCount: edgeCount, attributes: attributes, graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    oAttribute = (tmpAttribute, attIdx);
    Ok((oGraphInfo, oAttribute))
}

pub fn addGraphAttributeValue(mut iValue: (i32, ArcStr), mut iGraphIdx: i32, mut iGraphInfo: GraphInfo) -> Result<GraphInfo> {
    let mut oGraphInfo: GraphInfo;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let mut iGraph: Graph;
    let mut gid: ArcStr;
    let mut directed: bool;
    let mut nodeIdc: Arc<metamodelica::List<i32>>;
    let mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    iGraph = (graphs.clone()).get(graphCount - iGraphIdx + 1)?;
    let Graph { id: __pa9, directed: __pa10, nodeIdc: __pa11, attValues: __pa12 } = (iGraph) else { bail!("pattern mismatch") };
    gid = __pa9.clone();
    directed = __pa10.clone();
    nodeIdc = __pa11.clone();
    attValues = __pa12.clone();
    attValues = metamodelica::cons(iValue, attValues);
    iGraph = Graph { id: (gid).clone(), directed: directed, nodeIdc: nodeIdc, attValues: attValues };
    graphs = List::set(graphs, graphCount - iGraphIdx + 1, iGraph)?;
    oGraphInfo = GraphInfo::GRAPHINFO { graphs: graphs, graphCount: graphCount, nodes: nodes, nodeCount: nodeCount, edges: edges, edgeCount: edgeCount, attributes: attributes, graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    Ok(oGraphInfo)
}

// -------------------------
// Helper
// -------------------------
pub(crate) fn getMainGraph(mut iGraphInfo: GraphInfo) -> Result<Option<(i32, Graph)>> {
    let mut oGraph: Option<(i32, Graph)>;
    let mut graphs: Arc<metamodelica::List<Graph>> = metamodelica::nil();
    let mut firstGraph: Graph = <Graph as ::std::default::Default>::default();
    oGraph = (match iGraphInfo {
        GraphInfo::GRAPHINFO { graphCount: 0, .. } => None,
        GraphInfo::GRAPHINFO { graphs: mut __esc_graphs, .. } => {
            graphs = __esc_graphs.clone();
            firstGraph = listHead(graphs.clone())?;
            Some((1, firstGraph))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oGraph)
}

pub fn getAttributeByNameAndTarget(mut iAttributeName: ArcStr, mut iAttributeTarget: AttributeTarget, mut iGraphInfo: GraphInfo) -> Result<Option<(Attribute, i32)>> {
    let mut oAttribute: Option<(Attribute, i32)>;
    let mut attributes: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut tmpRes: Option<(Attribute, i32)> = None;
    oAttribute = (match iGraphInfo {
        GraphInfo::GRAPHINFO { attributes: mut __esc_attributes, .. } => {
            attributes = __esc_attributes.clone();
            tmpRes = getAttributeByNameAndTargetTail(attributes.clone(), (iAttributeName).clone(), iAttributeTarget);
            tmpRes
        },
        GraphInfo::GRAPHINFO { attributes: mut __esc_attributes, .. } => {
            attributes = __esc_attributes.clone();
            tmpRes = getAttributeByNameAndTargetTail(attributes.clone(), (iAttributeName).clone(), iAttributeTarget);
            tmpRes
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oAttribute)
}

fn getAttributeByNameAndTargetTail(mut iList: Arc<metamodelica::List<Attribute>>, mut iAttributeName: ArcStr, mut iAttributeTarget: AttributeTarget) -> Option<(Attribute, i32)> {
    let mut oAttribute: Option<(Attribute, i32)>;
    let mut rest: Arc<metamodelica::List<Attribute>> = metamodelica::nil();
    let mut attIdx: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    let mut head: Attribute = <Attribute as ::std::default::Default>::default();
    let mut attTarget: AttributeTarget = AttributeTarget::TARGET_EDGE;
    let mut tmpAttribute: Option<(Attribute, i32)> = None;
    oAttribute = 'mc: {
        let __mc_input = iList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head @ Attribute { attIdx, name, attTarget, .. }, tail: rest } => {
                    let true = (stringEq((name.clone()).clone(), (iAttributeName.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (compareAttributeTargets(iAttributeTarget.clone(), attTarget.clone())?) else { bail!("pattern mismatch") };
                    Ok(Some((head.clone(), attIdx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut tmpAttribute: Option<(Attribute, i32)> = tmpAttribute.clone();
                    tmpAttribute = getAttributeByNameAndTargetTail(rest.clone(), (iAttributeName.clone()).clone(), iAttributeTarget.clone());
                    Ok((tmpAttribute.clone(), tmpAttribute.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { tmpAttribute = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oAttribute
}

fn compareAttributeTargets(mut iTarget1: AttributeTarget, mut iTarget2: AttributeTarget) -> Result<bool> {
    let mut oEqual: bool;
    let mut tarInt1: i32;
    let mut tarInt2: i32;
    tarInt1 = compareAttributeTarget0(iTarget1)?;
    tarInt2 = compareAttributeTarget0(iTarget2)?;
    oEqual = intEq(tarInt1, tarInt2);
    Ok(oEqual)
}

fn compareAttributeTarget0(mut iTarget: AttributeTarget) -> Result<i32> {
    let mut oCodec: i32;
    oCodec = (match iTarget {
        AttributeTarget::TARGET_NODE { .. } => 0,
        AttributeTarget::TARGET_EDGE { .. } => 1,
        AttributeTarget::TARGET_GRAPH { .. } => 1,
    });
    Ok(oCodec)
}

// -------------------------
// Dump
// -------------------------
pub fn dumpGraph(mut iGraphInfo: GraphInfo, mut iFileName: ArcStr) -> Result<()> {
    let mut iGraphInfoArr: GraphInfo;
    iGraphInfoArr = convertToGraphInfoArr(iGraphInfo)?;
    Tpl::tplNoret2((std::sync::Arc::new(GraphMLDumpTpl::dumpGraphInfo) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, GraphInfo, ArcStr) -> Result<Tpl::Text> + 'static>), iGraphInfoArr, (iFileName).clone())?;
    Ok(())
}

fn convertToGraphInfoArr(mut iGraphInfo: GraphInfo) -> Result<GraphInfo> {
    let mut oGraphInfo: GraphInfo;
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphsArr: metamodelica::Array<Graph>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodesArr: metamodelica::Array<Node>;
    let mut nodeCount: i32;
    let mut edges: Arc<metamodelica::List<Edge>>;
    let mut edgeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut attributesArr: metamodelica::Array<Attribute>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, edges: __pa4, edgeCount: __pa5, attributes: __pa6, graphNodeKey: __pa7, graphEdgeKey: __pa8 } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    edges = __pa4.clone();
    edgeCount = __pa5.clone();
    attributes = __pa6.clone();
    graphNodeKey = __pa7.clone();
    graphEdgeKey = __pa8.clone();
    graphsArr = metamodelica::arrayFromVec(graphs.into_iter().cloned().collect());
    nodesArr = metamodelica::arrayFromVec(nodes.into_iter().cloned().collect());
    attributesArr = List::listArrayReverse(attributes)?;
    oGraphInfo = GraphInfo::GRAPHINFOARR { graphs: graphsArr.clone(), nodes: nodesArr.clone(), edges: edges, attributes: attributesArr.clone(), graphNodeKey: (graphNodeKey).clone(), graphEdgeKey: (graphEdgeKey).clone() };
    Ok(oGraphInfo)
}

// -------------------------
// debug prints
// -------------------------
pub(crate) fn printGraphInfo(mut iGraphInfo: GraphInfo) -> Result<()> {
    let mut graphs: Arc<metamodelica::List<Graph>>;
    let mut graphCount: i32;
    let mut nodes: Arc<metamodelica::List<Node>>;
    let mut nodeCount: i32;
    let mut attributes: Arc<metamodelica::List<Attribute>>;
    let mut graphNodeKey: ArcStr;
    let mut graphEdgeKey: ArcStr;
    let GraphInfo::GRAPHINFO { graphs: __pa0, graphCount: __pa1, nodes: __pa2, nodeCount: __pa3, attributes: __pa4, graphNodeKey: __pa5, graphEdgeKey: __pa6, .. } = (iGraphInfo) else { bail!("pattern mismatch") };
    graphs = __pa0.clone();
    graphCount = __pa1.clone();
    nodes = __pa2.clone();
    nodeCount = __pa3.clone();
    attributes = __pa4.clone();
    graphNodeKey = __pa5.clone();
    graphEdgeKey = __pa6.clone();
    List::map_0(nodes, (std::sync::Arc::new(printNode) as std::sync::Arc<dyn ::std::ops::Fn(Node) -> Result<()> + 'static>))?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nodeCount: ")); __mm_s.push_str(&*intString(nodeCount)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("graphCount: ")); __mm_s.push_str(&*intString(graphCount)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn printNode(mut node: Node) -> Result<()> {
    let mut id: ArcStr;
    let mut atts: ArcStr;
    let mut optDesc: Option<ArcStr>;
    let mut attValues: Arc<metamodelica::List<(i32, ArcStr)>>;
    let Node::NODE { id: __pa0, optDesc: __pa1, attValues: __pa2, .. } = (node) else { bail!("pattern mismatch") };
    id = __pa0.clone();
    optDesc = __pa1.clone();
    attValues = __pa2.clone();
    atts = stringDelimitList(List::map(attValues, std::sync::Arc::new(fnptr!(Util::tuple22, _)))?, (literal!(" | ")).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("node: ")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" desc: ")); __mm_s.push_str(&*Util::getOption(optDesc)?); __mm_s.push_str(&*literal!("\n\tatts: ")); __mm_s.push_str(&*atts); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

