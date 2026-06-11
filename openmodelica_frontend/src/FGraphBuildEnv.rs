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

use crate::FGraph;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Name = ArcStr;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = FCore::Node;

pub type Data = FCore::Data;

pub type Kind = FCore::Kind;

pub type Ref = metamodelica::Array<FCore::Node>;

pub type Refs = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Children = Arc<FCore::RefTree::Tree>;

pub type Parents = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Graph = FCore::Graph;

pub type Scope = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub(crate) fn mkProgramGraph(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inKind: Kind, mut graph: Graph) -> Result<Graph> {
    let mut graph: Graph = graph;
    let mut topRef: Ref;
    topRef = FGraph::top(graph.clone())?;
    for mut cls in &*inProgram {
        let mut cls = cls.clone();
        graph = mkClassGraph(cls.clone(), topRef.clone(), inKind.clone(), graph.clone(), true)?;
    }
    Ok(graph)
}

fn mkClassGraph(mut inClass: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph, mut checkDuplicate: bool) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inClass.clone(), inGraph)) {
        (Deref @ SCode::Element::CLASS { .. }, g) => {
            let mut g = (*g).clone();
            g = mkClassNode(inClass, openmodelica_frontend_types::DAE::Prefix::NOPRE, openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), inParentRef.clone(), inKind, g.clone(), checkDuplicate)?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkClassNode(mut inClass: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph, mut checkDuplicate: bool) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut cls: Arc<SCode::Element>;
            let mut name: ArcStr;
            let mut n: Node;
            let mut nr: Ref;
            cls = SCodeInstUtil::expandEnumerationClass(inClass)?;
            let __pa0 = ::match_deref::match_deref! { match &(cls.clone()) {
                Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::CL { e: cls, pre: inPrefix, r#mod: inMod, kind: inKind, status: openmodelica_frontend_dump::FCore::Status::CLS_UNTYPED });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (name).clone(), nr.clone(), checkDuplicate)?;
            g.clone()
        },
    });
    Ok(outGraph)
}

pub(crate) fn mkConstrainClass(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inElement, inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) }, .. }, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), list![inParentRef.clone()], FCore::Data::CC { cc: cc.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) }, .. }, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), list![inParentRef.clone()], FCore::Data::CC { cc: cc.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::ccNodeName)).clone(), nr.clone(), false)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outGraph
}

pub(crate) fn mkModNode(mut inName: Name, mut inMod: Arc<SCode::Mod>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inName, inMod.clone(), inGraph);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::NOMOD { .. }, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, binding: None, .. }, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, binding: b @ Some(_), .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::MO { m: inMod.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
                    g = mkBindingNode(b.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::MOD { subModLst: sm, binding: b, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::MO { m: inMod.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
                    g = mkSubMods(sm.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    g = mkBindingNode(b.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, Deref @ SCode::Mod::REDECL { element: e, .. }, g) => {
                    let mut n: Node;
                    let mut nr: Ref;
                    let mut g = (*g).clone();
                    (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::MO { m: inMod.clone() });
                    nr = FNode::toRef(n.clone());
                    FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
                    g = mkElementNode(e.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (name, _, g) => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraphBuildEnv.mkModNode failed with: ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" mod: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn mkSubMods(mut inSubMod: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inSubMod, inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            return Ok(g.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: id, r#mod: m }, tail: rest }, g) => {
            let mut g = (*g).clone();
            g = mkModNode((id.clone()).clone(), m.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            { (inSubMod, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn mkBindingNode(mut inBinding: Option<Arc<Absyn::Exp>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inBinding, inGraph)) {
        (None, g) => {
            g.clone()
        },
        (Some(e), g) => {
            let mut g = (*g).clone();
            g = mkExpressionNode((arcstr::literal!(FNode::bndNodeName)).clone(), e.clone(), inParentRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

fn mkClassChildren(mut inClassDef: Arc<SCode::ClassDef>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inClassDef, inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: el, normalEquationLst: eqs, initialEquationLst: ieqs, normalAlgorithmLst: als, initialAlgorithmLst: ials, constraintLst, clsattrs, externalDecl }, g) => {
                    let mut g = (*g).clone();
                    g = List::fold2(el.clone(), (std::sync::Arc::new(mkElementNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkEqNode((arcstr::literal!(FNode::eqNodeName)).clone(), eqs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkEqNode((arcstr::literal!(FNode::ieqNodeName)).clone(), ieqs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkAlNode((arcstr::literal!(FNode::alNodeName)).clone(), als.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkAlNode((arcstr::literal!(FNode::ialNodeName)).clone(), ials.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkOptNode((arcstr::literal!(FNode::optNodeName)).clone(), constraintLst.clone(), clsattrs.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkExternalNode((arcstr::literal!(FNode::edNodeName)).clone(), externalDecl.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: cdef, modifications: m }, g) => {
                    let mut g = (*g).clone();
                    g = mkClassChildren(cdef.clone(), inParentRef.clone(), inKind.clone(), g.clone());
                    g = mkModNode((arcstr::literal!(FNode::modNodeName)).clone(), m.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
                    g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), inParentRef.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::DERIVED { typeSpec: ts, modifications: m, .. }, g) => {
                    let mut nr: Ref;
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut g = (*g).clone();
                    nr = inParentRef.clone();
                    g = mkModNode((arcstr::literal!(FNode::modNodeName)).clone(), m.clone(), nr.clone(), inKind.clone(), g.clone())?;
                    ad = AbsynUtil::typeSpecDimensions(ts.clone());
                    g = mkDimsNode((arcstr::literal!(FNode::tydimsNodeName)).clone(), Some(ad.clone()), nr.clone(), inKind.clone(), g.clone())?;
                    g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), nr.clone(), g.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::OVERLOAD { pathLst: _ }, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PDER { functionPath: _, derivedVariables: _ }, g) => {
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outGraph
}

pub(crate) fn mkElementNode(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inElement.clone(), inGraph)) {
        (Deref @ SCode::Element::COMPONENT { .. }, g) => {
            let mut g = (*g).clone();
            g = mkCompNode(inElement, inParentRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        (Deref @ SCode::Element::CLASS { .. }, g) => {
            let mut g = (*g).clone();
            g = mkClassNode(inElement, openmodelica_frontend_types::DAE::Prefix::NOPRE, openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), inParentRef.clone(), inKind, g.clone(), false)?;
            g.clone()
        },
        (Deref @ SCode::Element::EXTENDS { baseClassPath: p, modifications: m, .. }, g) => {
            let mut name: ArcStr;
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            name = (FNode::mkExtendsName(p.clone())?).clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::EX { e: inElement, r#mod: openmodelica_frontend_types::DAE::Mod::interned_NOMOD() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (name).clone(), nr.clone(), false)?;
            g = mkModNode((arcstr::literal!(FNode::modNodeName)).clone(), m.clone(), nr.clone(), inKind, g.clone())?;
            g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), nr.clone(), g.clone())?;
            g.clone()
        },
        (Deref @ SCode::Element::IMPORT { .. }, g) => {
            let mut g = (*g).clone();
            g = mkImportNode(inElement, inParentRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        (Deref @ SCode::Element::DEFINEUNIT { .. }, g) => {
            let mut g = (*g).clone();
            g = mkUnitsNode(inElement, inParentRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkUnitsNode(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::duNodeName)).clone())?;
            FNode::addDefinedUnitToRef(r.clone(), inElement.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut n: Node;
            let mut r: Ref;
            (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::duNodeName)).clone(), list![inParentRef.clone()], FCore::Data::DU { els: list![inElement.clone()] });
            r = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::duNodeName)).clone(), r.clone(), false)?;
            Ok(g.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn mkImportNode(mut inElement: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::imNodeName)).clone())?;
            FNode::addImportToRef(r.clone(), inElement.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut n: Node;
            let mut r: Ref;
            (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::imNodeName)).clone(), list![inParentRef.clone()], FCore::Data::IM { i: FCore::emptyImportTable.clone() });
            r = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::imNodeName)).clone(), r.clone(), false)?;
            FNode::addImportToRef(r.clone(), inElement.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn mkDimsNode(mut inName: Name, mut inArrayDims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inArrayDims, inGraph)) {
        (None, g) => {
            g.clone()
        },
        (Some(Deref @ metamodelica::List::Nil), g) => {
            g.clone()
        },
        (Some(a @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::DIMS { name: (inName.clone()).clone(), dims: a.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), nr.clone(), false)?;
            g = mkDimsNode_helper(0, a.clone(), nr.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkDimsNode_helper(mut inStartWith: i32, mut inArrayDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inStartWith, inArrayDims, inGraph)) {
        (_, Deref @ metamodelica::List::Nil, g) => {
            return Ok(g.clone())
        },
        (i, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: rest }, g) => {
            let mut name: Name;
            let mut g = (*g).clone();
            name = (intString(i.clone())).clone();
            g = mkExpressionNode((name).clone(), openmodelica_ast::Absyn::Exp::interned_END(), inParentRef.clone(), inKind.clone(), g.clone())?;
            { (inStartWith, inArrayDims, inParentRef, inKind, inGraph) = (i.clone() + 1, rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        (i, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: rest }, g) => {
            let mut name: Name;
            let mut g = (*g).clone();
            name = (intString(i.clone())).clone();
            g = mkExpressionNode((name).clone(), e.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            { (inStartWith, inArrayDims, inParentRef, inKind, inGraph) = (i.clone() + 1, rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn mkCompNode(mut inComp: Arc<SCode::Element>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut name: ArcStr;
    let mut g: Graph;
    let mut n: Node;
    let mut nr: Ref;
    let mut m: Arc<SCode::Mod>;
    let mut cnd: Option<Arc<Absyn::Exp>>;
    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    let mut ts: Arc<Absyn::TypeSpec>;
    let mut nd: Data;
    let mut i: Arc<DAE::Var>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, attributes: SCode::Attributes { arrayDims: __pa1, .. }, typeSpec: __pa2, modifications: __pa3, condition: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ad = __pa1.clone();
    ts = __pa2.clone();
    m = __pa3.clone();
    cnd = __pa4.clone();
    (nd, i) = FNode::element2Data(inComp, inKind)?;
    (g, n) = FGraph::node(inGraph, (name.clone()).clone(), list![inParentRef.clone()], nd);
    nr = FNode::toRef(n);
    FNode::addChildRef(inParentRef.clone(), (name).clone(), nr.clone(), false)?;
    g = mkInstNode(i, nr.clone(), g)?;
    g = mkRefNode((arcstr::literal!(FNode::refNodeName)).clone(), metamodelica::nil(), nr.clone(), g)?;
    outGraph = g;
    Ok(outGraph)
}

pub(crate) fn mkInstNode(mut inVar: Arc<DAE::Var>, mut inParentRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut nr: Ref;
    let mut n: Node;
    let mut g: Graph;
    (g, n) = FGraph::node(inGraph, (arcstr::literal!(FNode::itNodeName)).clone(), list![inParentRef.clone()], FCore::Data::IT { i: inVar });
    nr = FNode::toRef(n);
    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::itNodeName)).clone(), nr.clone(), false)?;
    outGraph = g;
    Ok(outGraph)
}

pub(crate) fn mkConditionNode(mut inCondition: Option<Arc<Absyn::Exp>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inCondition, inGraph)) {
        (None, g) => {
            g.clone()
        },
        (Some(e), g) => {
            let mut g = (*g).clone();
            g = mkExpressionNode((arcstr::literal!(FNode::cndNodeName)).clone(), e.clone(), inParentRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkExpressionNode(mut inName: Name, mut inExp: Arc<Absyn::Exp>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inExp, inGraph)) {
        (e, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::EXP { name: (inName.clone()).clone(), e: e.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), nr.clone(), false)?;
            g = analyseExp(e.clone(), nr.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkCrefsNodes(mut inCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCrefs, inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            return Ok(g.clone())
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, g) => {
            let mut g = (*g).clone();
            g = mkCrefNode(cr.clone(), inParentRef.clone(), inKind.clone(), g.clone())?;
            { (inCrefs, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn mkCrefNode(mut inCref: Arc<Absyn::ComponentRef>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut n: Node;
            let mut nr: Ref;
            let mut name: Name;
            name = (Dump::printComponentRefStr(inCref.clone())?).clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::CR { r: inCref.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (name).clone(), nr.clone(), false)?;
            g = mkDimsNode((arcstr::literal!(FNode::subsNodeName)).clone(), List::mkOption(AbsynUtil::getSubsFromCref(inCref, true, true)?), nr.clone(), inKind, g.clone())?;
            g.clone()
        },
    });
    Ok(outGraph)
}

pub(crate) fn mkTypeNode(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inParentRef: Ref, mut inName: Name, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nr: Ref;
            let mut pr: Ref;
            pr = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone())?;
            nr = FNode::child(pr.clone(), (inName.clone()).clone())?;
            FNode::addTypesToRef(nr.clone(), inTypes.clone())?;
            Ok(inGraph.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut nr: Ref;
            let mut pr: Ref;
            let mut n: Node;
            if '__try0: {
                unwrap_break_err!(FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::tyNodeName)).clone(), list![inParentRef.clone()], FCore::Data::ND { scopeType: None });
            pr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone(), pr.clone(), false)?;
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![pr.clone()], FCore::Data::FT { tys: inTypes.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(pr.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut nr: Ref;
            let mut pr: Ref;
            let mut n: Node;
            pr = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::tyNodeName)).clone())?;
            if '__try0: {
                unwrap_break_err!(FNode::child(pr.clone(), (inName.clone()).clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![pr.clone()], FCore::Data::FT { tys: inTypes.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(pr.clone(), (inName.clone()).clone(), nr.clone(), false)?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut pr: Ref;
            pr = FGraph::top(inGraph.clone())?;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraphBuildEnv.mkTypeNode: Error making type node: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!(" in parent: ")); __mm_s.push_str(&*FNode::name(FNode::fromRef(pr.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(inGraph.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn mkEqNode(mut inName: Name, mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inEqs.clone(), inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            g.clone()
        },
        (_, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::EQ { name: (inName.clone()).clone(), e: inEqs.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), nr.clone(), false)?;
            g = List::fold2(inEqs, (std::sync::Arc::new(analyseEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), nr.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkAlNode(mut inName: Name, mut inAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inAlgs.clone(), inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            g.clone()
        },
        (_, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::AL { name: (inName.clone()).clone(), a: inAlgs.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), nr.clone(), false)?;
            g = List::fold2(inAlgs, (std::sync::Arc::new(analyseAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), nr.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkOptNode(mut inName: Name, mut inConstraintLst: Arc<metamodelica::List<SCode::ConstraintSection>>, mut inClsAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inConstraintLst.clone(), inClsAttrs.clone(), inGraph)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, g) => {
            g.clone()
        },
        (_, _, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::OT { constrainLst: inConstraintLst, clsAttrs: inClsAttrs });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), nr.clone(), false)?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkExternalNode(mut inName: Name, mut inExternalDeclOpt: Option<Arc<SCode::ExternalDecl>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inExternalDeclOpt, inGraph)) {
        (None, g) => {
            g.clone()
        },
        (Some(ed @ Deref @ SCode::ExternalDecl { output_: ocr, args: exps, .. }), g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut oae: Option<Arc<Absyn::Exp>>;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::ED { ed: ed.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), nr.clone(), false)?;
            oae = Util::applyOption(ocr.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefExp, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
            g = mkCrefsFromExps(List::consOption(oae, exps.clone()), nr.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

pub(crate) fn mkCrefsFromExps(mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExps, inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            return Ok(g.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, g) => {
            let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut g = (*g).clone();
            crefs = AbsynUtil::getCrefFromExp(e.clone(), true, true)?;
            g = mkCrefsNodes(crefs, inParentRef.clone(), inKind.clone(), g.clone())?;
            { (inExps, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn analyseExp(mut inExp: Arc<Absyn::Exp>, mut inRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = AbsynUtil::traverseExpBidir(inExp, (std::sync::Arc::new({ let __pe_b1 = inRef.clone(); let __pe_b2 = inKind; move |__pe_a0, __pe_a3| analyseExpTraverserEnter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), (std::sync::Arc::new(fnptr!(analyseExpTraverserExit, Arc<Absyn::Exp>, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), inGraph)?;
    Ok(outGraph)
}

fn analyseOptExp(mut inExp: Option<Arc<Absyn::Exp>>, mut inRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inExp, inGraph)) {
        (None, g) => {
            g.clone()
        },
        (Some(exp), g) => {
            let mut g = (*g).clone();
            g = analyseExp(exp.clone(), inRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

fn analyseExpTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut r#ref: Ref, mut kind: Kind, mut graph: Graph) -> Result<(Arc<Absyn::Exp>, Graph)> {
    let mut inExp: Arc<Absyn::Exp> = inExp;
    let mut graph: Graph = graph;
    graph = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: cref } => {
            analyseCref(cref.clone(), r#ref.clone(), kind, graph)?
        },
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: iters, .. }, .. } => {
            addIterators(iters.clone(), r#ref.clone(), kind, graph)?
        },
        Deref @ Absyn::Exp::CALL { function_: cref, .. } => {
            analyseCref(cref.clone(), r#ref.clone(), kind, graph)?
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, .. } => {
            analyseCref(cref.clone(), r#ref.clone(), kind, graph)?
        },
        Deref @ Absyn::Exp::MATCHEXP { .. } => {
            addMatchScope(inExp.clone(), r#ref.clone(), kind, graph)?
        },
        _ => {
            graph
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((inExp, graph))
}

fn analyseCref(mut inCref: Arc<Absyn::ComponentRef>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (::match_deref::match_deref! { match &((inCref.clone(), inGraph)) {
        (Deref @ Absyn::ComponentRef::WILD { .. }, g) => {
            g.clone()
        },
        (_, g) => {
            let mut g = (*g).clone();
            g = mkCrefNode(inCref, inParentRef.clone(), inKind, g.clone())?;
            g.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraph)
}

fn analyseExpTraverserExit(mut exp: Arc<Absyn::Exp>, mut graph: Graph) -> (Arc<Absyn::Exp>, Graph) {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut graph: Graph = graph;
    (exp, graph)
}

fn analyseEquation(mut inEquation: Arc<SCode::Equation>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = SCodeUtil::mapFoldEquations(inEquation, (std::sync::Arc::new({ let __pe_b1 = inParentRef.clone(); let __pe_b2 = inKind; move |__pe_a0, __pe_a3| analyseEquationTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, FCore::Graph) -> Result<(Arc<SCode::Equation>, FCore::Graph)> + 'static>), inGraph)?;
    Ok(outGraph)
}

fn analyseEquationTraverser(mut eq: Arc<SCode::Equation>, mut r#ref: Ref, mut kind: Kind, mut graph: Graph) -> Result<(Arc<SCode::Equation>, Graph)> {
    let mut eq: Arc<SCode::Equation> = eq;
    let mut graph: Graph = graph;
    (eq, graph) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { index: iter_name, .. } => {
            graph = addIterators(list![Arc::new(Absyn::ForIterator { name: (iter_name.clone()).clone(), guardExp: None, range: None })], r#ref.clone(), kind.clone(), graph)?;
            SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, .. } => {
            graph = analyseCref(cref1.clone(), r#ref.clone(), kind.clone(), graph)?;
            SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?
        },
        _ => {
            SCodeUtil::getEquationInfo(eq.clone())?;
            SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, graph))
}

fn traverseExp(mut exp: Arc<Absyn::Exp>, mut graph: Graph, mut r#ref: Ref, mut kind: Kind) -> Result<(Arc<Absyn::Exp>, Graph)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut graph: Graph = graph;
    (exp, graph) = AbsynUtil::traverseExpBidir(exp, (std::sync::Arc::new({ let __pe_b1 = r#ref.clone(); let __pe_b2 = kind; move |__pe_a0, __pe_a3| analyseExpTraverserEnter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), (std::sync::Arc::new(fnptr!(analyseExpTraverserExit, Arc<Absyn::Exp>, FCore::Graph)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?;
    Ok((exp, graph))
}

fn analyseAlgorithm(mut inAlgorithm: Arc<SCode::AlgorithmSection>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm) {
        Deref @ SCode::AlgorithmSection { statements: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    outGraph = List::fold2(stmts, (std::sync::Arc::new(analyseStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), inParentRef.clone(), inKind, inGraph)?;
    Ok(outGraph)
}

fn analyseStatement(mut inStatement: Arc<SCode::Statement>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    (_, outGraph) = SCodeUtil::mapFoldStatements(inStatement, (std::sync::Arc::new({ let __pe_b1 = inParentRef.clone(); let __pe_b2 = inKind; move |__pe_a0, __pe_a3| analyseStatementTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, FCore::Graph) -> Result<(Arc<SCode::Statement>, FCore::Graph)> + 'static>), inGraph)?;
    Ok(outGraph)
}

fn analyseStatementTraverser(mut stmt: Arc<SCode::Statement>, mut r#ref: Ref, mut kind: Kind, mut graph: Graph) -> Result<(Arc<SCode::Statement>, Graph)> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut graph: Graph = graph;
    (stmt, graph) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            graph = addIterators(list![Arc::new(Absyn::ForIterator { name: (var_field!((*stmt).index, SCode::Statement::ALG_FOR).clone()).clone(), guardExp: None, range: None })], r#ref.clone(), kind.clone(), graph)?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(stmt.clone(), (std::sync::Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?;
            (stmt, graph)
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            graph = addIterators(list![Arc::new(Absyn::ForIterator { name: (var_field!((*stmt).index, SCode::Statement::ALG_PARFOR).clone()).clone(), guardExp: None, range: None })], r#ref.clone(), kind.clone(), graph)?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(stmt.clone(), (std::sync::Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?;
            (stmt, graph)
        },
        _ => {
            SCodeUtil::getStatementInfo(stmt.clone())?;
            (_, graph) = SCodeUtil::mapFoldStatementExps(stmt.clone(), (std::sync::Arc::new({ let __pe_b2 = r#ref.clone(); let __pe_b3 = kind; move |__pe_a0, __pe_a1| traverseExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, FCore::Graph) -> Result<(Arc<Absyn::Exp>, FCore::Graph)> + 'static>), graph)?;
            (stmt, graph)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, graph))
}

pub(crate) fn addIterators(mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut nr: Ref;
            nr = FNode::child(inParentRef.clone(), (arcstr::literal!(FNode::forNodeName)).clone())?;
            FNode::addIteratorsToRef(nr.clone(), inIterators.clone())?;
            g = addIterators_helper(inIterators.clone(), nr.clone(), inKind.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut n: Node;
            let mut nr: Ref;
            (g, n) = FGraph::node(g.clone(), (arcstr::literal!(FNode::forNodeName)).clone(), list![inParentRef.clone()], FCore::Data::FS { fis: inIterators.clone() });
            nr = FNode::toRef(n.clone());
            FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::forNodeName)).clone(), nr.clone(), false)?;
            g = addIterators_helper(inIterators.clone(), nr.clone(), inKind.clone(), g.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn addIterators_helper(mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inIterators, inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            return Ok(g.clone())
        },
        (Deref @ metamodelica::List::Cons { head: i @ Deref @ Absyn::ForIterator { name, .. }, tail: rest }, g) => {
            let mut n: Node;
            let mut nr: Ref;
            let mut g = (*g).clone();
            (g, n) = FGraph::node(g.clone(), (name.clone()).clone(), list![inParentRef.clone()], FCore::Data::FI { fi: i.clone() });
            nr = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (name.clone()).clone(), nr.clone(), false)?;
            { (inIterators, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn addMatchScope(mut inMatchExp: Arc<Absyn::Exp>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut n: Node;
    let mut nr: Ref;
    let mut local_decls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut g: Graph;
    (g, n) = FGraph::node(inGraph, (arcstr::literal!(FNode::matchNodeName)).clone(), list![inParentRef.clone()], FCore::Data::MS { e: inMatchExp.clone() });
    nr = FNode::toRef(n);
    FNode::addChildRef(inParentRef.clone(), (arcstr::literal!(FNode::matchNodeName)).clone(), nr.clone(), false)?;
    let __pa0 = ::match_deref::match_deref! { match &(inMatchExp) {
        Deref @ Absyn::Exp::MATCHEXP { localDecls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    local_decls = __pa0.clone();
    outGraph = addMatchScope_helper(local_decls, nr.clone(), inKind, g)?;
    Ok(outGraph)
}

pub(crate) fn addMatchScope_helper(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inParentRef: Ref, mut inKind: Kind, mut inGraph: Graph) -> Result<Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElements, inGraph)) {
        (Deref @ metamodelica::List::Nil, g) => {
            return Ok(g.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element }, tail: rest }, g) => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut g = (*g).clone();
            el = AbsynToSCode::translateElement(element.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
            g = List::fold2(el, (std::sync::Arc::new(mkElementNode) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, metamodelica::Array<FCore::Node>, FCore::Kind, FCore::Graph) -> Result<FCore::Graph> + 'static>), inParentRef.clone(), inKind.clone(), g.clone())?;
            { (inElements, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, g) => {
            let mut g = (*g).clone();
            { (inElements, inParentRef, inKind, inGraph) = (rest.clone(), inParentRef.clone(), inKind, g.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn mkRefNode(mut inName: Name, mut inTargetScope: Scope, mut inParentRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut n: Node;
            let mut rn: Ref;
            (g, n) = FGraph::node(g.clone(), (inName.clone()).clone(), list![inParentRef.clone()], FCore::Data::REF { target: inTargetScope });
            rn = FNode::toRef(n);
            FNode::addChildRef(inParentRef.clone(), (inName).clone(), rn.clone(), false)?;
            g.clone()
        },
    });
    Ok(outGraph)
}

