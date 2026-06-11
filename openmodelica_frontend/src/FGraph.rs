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

use crate::FGraphBuildEnv;
use crate::FNode;
use crate::InnerOuter;
use crate::Mod;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore::RefTree;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
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

pub type Scope = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Top = FCore::Top;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Status = FCore::Status;

pub const fn emptyGraph() -> FCore::Graph { FCore::Graph::EG { name: literal!("empty") } }

pub(crate) fn top(mut inGraph: Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).node.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outRef)
}

pub(crate) fn extra(mut inGraph: Graph) -> Result<Extra> {
    let mut outExtra: Extra;
    outExtra = (match inGraph.clone() {
        FCore::Graph::G { .. } => var_field!(inGraph.top, FCore::Graph::G).extra.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outExtra)
}

pub(crate) fn currentScope(mut inGraph: Graph) -> Result<Scope> {
    let mut outScope: Scope = metamodelica::nil();
    outScope = (match inGraph {
        FCore::Graph::G { scope: ref __esc_outScope, .. } => {
            outScope = __esc_outScope.clone();
            outScope.clone()
        },
        FCore::Graph::EG { name: _ } => metamodelica::nil(),
    });
    Ok(outScope)
}

pub fn lastScopeRef(mut inGraph: Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = listHead(currentScope(inGraph)?)?;
    Ok(outRef)
}

pub(crate) fn setLastScopeRef(mut inRef: Ref, mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph = inGraph.clone();
    outGraph = (match outGraph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = metamodelica::cons(inRef.clone(), listRest(var_field!(outGraph.scope, FCore::Graph::G).clone())?);
            if let FCore::Graph::G { scope, .. } = &mut outGraph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            outGraph
        },
        _ => outGraph,
    });
    Ok(outGraph)
}

pub(crate) fn stripLastScopeRef(mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    let mut t: Top;
    let mut s: Scope;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inGraph) {
        FCore::Graph::G { top: __pa0, scope: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    t = __pa0.clone();
    outRef = __pa1.clone();
    s = __pa2.clone();
    outGraph = FCore::Graph::G { top: t, scope: s };
    Ok((outGraph, outRef))
}

pub(crate) fn topScope(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        FCore::Graph::G { .. } => metamodelica::arrayGet(var_field!(inGraph.top, FCore::Graph::G).graph.clone(), 1)?,
        _ => bail!("match: no arm matched"),
    });
    Ok(outGraph)
}

pub fn empty() -> Graph {
    let mut outGraph: Graph;
    outGraph = emptyGraph().clone();
    outGraph
}

pub fn new(mut inGraphName: Name, mut inPath: Arc<Absyn::Path>) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut n: Node;
    let mut s: Scope;
    let mut nr: Ref;
    let mut id: Id;
    let mut ag: metamodelica::Array<FCore::Graph>;
    let mut top: Top;
    id = System::tmpTickIndex(Global::fgraph_nextId.clone());
    n = FNode::new((arcstr::literal!(FNode::topNodeName)).clone(), id, metamodelica::nil(), openmodelica_frontend_dump::FCore::Data::TOP);
    nr = FNode::toRef(n);
    s = list![nr.clone()];
    ag = metamodelica::arrayCreate(1, emptyGraph().clone());
    top = FCore::Top { graph: ag.clone(), name: (inGraphName).clone(), node: nr.clone(), extra: FCore::Extra { topModel: inPath } };
    outGraph = FCore::Graph::G { top: top.clone(), scope: s };
    unsafe { metamodelica::Dangerous::arrayInitSlot(ag.clone(), 1, FCore::Graph::G { top: top, scope: list![nr.clone()] }) };
    Ok(outGraph)
}

pub(crate) fn node(mut inGraph: Graph, mut inName: Name, mut inParents: Parents, mut inData: Data) -> (Graph, Node) {
    let mut outGraph: Graph;
    let mut outNode: Node;
    (outGraph, outNode) = (match inGraph {
        mut g => {
            let mut i: i32;
            let mut n: Node;
            i = System::tmpTickIndex(Global::fgraph_nextId.clone());
            n = FNode::new((inName).clone(), i.clone(), inParents, inData);
            (g.clone(), n.clone())
        },
    });
    (outGraph, outNode)
}

pub(crate) fn clone(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        FCore::Graph::G { top: mut t, scope: ref s } => {
            let mut g: Graph;
            let mut nt: Ref;
            let mut ag: metamodelica::Array<FCore::Graph>;
            let mut s = s.clone();
            nt = FNode::toRef(FNode::fromRef(t.node.clone())?);
            (g, nt) = FNode::copyRef(nt.clone(), inGraph)?;
            s = List::map1r(s.clone(), (std::sync::Arc::new(FNode::lookupRefFromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>) -> Result<metamodelica::Array<FCore::Node>> + 'static>), nt.clone())?;
            ag = arrayCreate(1, emptyGraph().clone());
            t = FCore::Top { graph: ag.clone(), name: (t.name.clone()).clone(), node: nt.clone(), extra: t.extra.clone() };
            g = FCore::Graph::G { top: t.clone(), scope: s.clone() };
            metamodelica::arrayUpdate(ag.clone(), 1, g.clone())?;
            g.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outGraph)
}

pub fn updateComp(mut inGraph: Graph, mut inVar: Arc<DAE::Var>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id;
                    let mut p: Parents;
                    let mut c: Children;
                    let mut e: Arc<SCode::Element>;
                    let mut m: Arc<DAE::Mod>;
                    let mut k: Kind;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(g.clone())?;
                    r = FNode::child(pr.clone(), (n.clone()).clone())?;
                    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::CO { e: __pa4, r#mod: __pa5, kind: __pa6, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    id = __pa1.clone();
                    p = __pa2.clone();
                    c = __pa3.clone();
                    e = __pa4.clone();
                    m = __pa5.clone();
                    k = __pa6.clone();
                    r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CO { e: e.clone(), r#mod: m.clone(), kind: k.clone(), status: instStatus.clone() } })?;
                    r = updateSourceTargetScope(r.clone(), currentScope(inTargetGraph.clone())?)?;
                    r = updateInstance(r.clone(), v.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(g.clone())?;
                    let true = (FNode::isImplicitRefName(pr.clone())?) else { bail!("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateComp(g.clone(), v.clone(), instStatus.clone(), inTargetGraph.clone());
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

pub(crate) fn updateSourceTargetScope(mut inRef: Ref, mut inTargetScope: Scope) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else { bail!("nomatch") };
            r = FNode::refRef(r.clone())?;
            r = FNode::updateRef(r.clone(), FNode::setData(FNode::fromRef(r.clone())?, FCore::Data::REF { target: inTargetScope.clone() })?)?;
            Ok(inRef.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else { bail!("nomatch") };
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FNode.updateSourceTargetScope: node does not yet have a reference child: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(r.clone())?)?); __mm_s.push_str(&*literal!(" target scope: ")); __mm_s.push_str(&*FNode::scopeStr(inTargetScope.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(inRef.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRef)
}

pub(crate) fn updateInstance(mut inRef: Ref, mut inVar: Arc<DAE::Var>) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else { bail!("nomatch") };
            r = FNode::refInstance(r.clone())?;
            r = FNode::updateRef(r.clone(), FNode::setData(FNode::fromRef(r.clone())?, FCore::Data::IT { i: inVar.clone() })?)?;
            Ok(inRef.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.updateInstance failed for node: ")); __mm_s.push_str(&*FNode::toPathStr(FNode::fromRef(inRef.clone())?)?); __mm_s.push_str(&*literal!(" variable:")); __mm_s.push_str(&*TypesDump::printVarStr(inVar.clone())?); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRef)
}

fn updateVarAndMod(mut inGraph: Graph, mut inVar: Arc<DAE::Var>, mut inMod: Arc<DAE::Mod>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inVar);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id;
                    let mut p: Parents;
                    let mut c: Children;
                    let mut e: Arc<SCode::Element>;
                    let mut k: Kind;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(g.clone())?;
                    r = FNode::child(pr.clone(), (n.clone()).clone())?;
                    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::CO { e: __pa4, r#mod: _, kind: __pa5, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    id = __pa1.clone();
                    p = __pa2.clone();
                    c = __pa3.clone();
                    e = __pa4.clone();
                    k = __pa5.clone();
                    r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CO { e: e.clone(), r#mod: inMod.clone(), kind: k.clone(), status: instStatus.clone() } })?;
                    r = updateSourceTargetScope(r.clone(), currentScope(inTargetGraph.clone())?)?;
                    r = updateInstance(r.clone(), v.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(g.clone())?;
                    let true = (FNode::isImplicitRefName(pr.clone())?) else { bail!("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateVarAndMod(g.clone(), v.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone());
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

pub(crate) fn updateClass(mut inGraph: Graph, mut inElement: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph, inElement);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, e @ Deref @ SCode::Element::CLASS { name: n, .. }) => {
                    let mut pr: Ref;
                    let mut r: Ref;
                    let mut id: Id;
                    let mut p: Parents;
                    let mut c: Children;
                    let mut k: Kind;
                    let mut n = (*n).clone();
                    pr = lastScopeRef(g.clone())?;
                    r = FNode::child(pr.clone(), (n.clone()).clone())?;
                    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::CL { e: _, pre: _, r#mod: _, kind: __pa4, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    n = __pa0.clone();
                    id = __pa1.clone();
                    p = __pa2.clone();
                    c = __pa3.clone();
                    k = __pa4.clone();
                    r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CL { e: e.clone(), pre: inPrefix.clone(), r#mod: inMod.clone(), kind: k.clone(), status: instStatus.clone() } })?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, e) => {
                    let mut pr: Ref;
                    let mut g = (*g).clone();
                    pr = lastScopeRef(g.clone())?;
                    let true = (FNode::isImplicitRefName(pr.clone())?) else { bail!("pattern mismatch") };
                    (g, _) = stripLastScopeRef(g.clone())?;
                    g = updateClass(g.clone(), e.clone(), inPrefix.clone(), inMod.clone(), instStatus.clone(), inTargetGraph.clone())?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn updateClassElement(mut inRef: Ref, mut inElement: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut instStatus: FCore::Status, mut inTargetGraph: Graph) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (::match_deref::match_deref! { match &((inRef.clone(), inElement)) {
        (r, e @ Deref @ SCode::Element::CLASS { name: n, .. }) => {
            let mut id: Id;
            let mut p: Parents;
            let mut c: Children;
            let mut k: Kind;
            let mut r = (*r).clone();
            let FCore::N { name: _, id: __pa0, parents: __pa1, children: __pa2, data: FCore::CL { e: _, pre: _, r#mod: _, kind: __pa3, status: _ } } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
            id = __pa0.clone();
            p = __pa1.clone();
            c = __pa2.clone();
            k = __pa3.clone();
            r = FNode::updateRef(r.clone(), FCore::Node { name: (n.clone()).clone(), id: id.clone(), parents: p.clone(), children: c.clone(), data: FCore::Data::CL { e: e.clone(), pre: inPrefix, r#mod: inMod, kind: k.clone(), status: instStatus } })?;
            r.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRef)
}

pub(crate) fn addForIterator(mut inGraph: Graph, mut name: ArcStr, mut ty: Arc<DAE::Type>, mut binding: Arc<DAE::Binding>, mut variability: SCode::Variability, mut constOfForIteratorRange: Option<DAE::Const>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            let mut c: Arc<SCode::Element>;
            let mut v: Arc<DAE::Var>;
            c = Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
            v = Arc::new(DAE::Var { name: (name).clone(), attributes: Arc::new(DAE::Attributes { connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: variability, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC }), ty: ty, binding: binding, bind_from_outside: false, constOfForIteratorRange: constOfForIteratorRange });
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkCompNode(c.clone(), r.clone(), openmodelica_frontend_dump::FCore::Kind::BUILTIN, g.clone())?;
            g = updateVarAndMod(g.clone(), v.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED, empty());
            g.clone()
        },
    });
    Ok(outGraph)
}

pub(crate) fn printGraphPathStr(mut inGraph: Graph) -> ArcStr {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Graph::G { scope: s @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. } => {
                    let mut r#str: ArcStr;
                    let mut s = (*s).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(s.clone().reverse()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    s = __pa0.clone();
                    r#str = stringDelimitList(List::map(s.clone(), (std::sync::Arc::new(FNode::refName) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<ArcStr> + 'static>))?, (literal!(".")).clone());
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("<global scope>"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    outString
}

pub(crate) fn openNewScope(mut inGraph: Graph, mut encapsulatedPrefix: SCode::Encapsulated, mut inName: Option<ArcStr>, mut inScopeType: Option<FCore::ScopeType>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, Some(mut n)) = __mc_input.clone() else { bail!("nomatch") };
            let mut no: Node;
            let mut r: Ref;
            let mut p: Ref;
            p = lastScopeRef(g.clone())?;
            (g, no) = node(g.clone(), (n.clone()).clone(), list![p.clone()], FCore::Data::ND { scopeType: inScopeType.clone() });
            r = FNode::toRef(no.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.openNewScope: failed to open new scope in scope: ")); __mm_s.push_str(&*getGraphNameStr(inGraph.clone())); __mm_s.push_str(&*literal!(" name: ")); __mm_s.push_str(&*Util::getOptionOrDefault(inName.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub fn openScope(mut inGraph: Graph, mut encapsulatedPrefix: SCode::Encapsulated, mut inName: Name, mut inScopeType: Option<FCore::ScopeType>) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut p: Ref;
    p = lastScopeRef(inGraph.clone())?;
    outGraph = 'mc: {
        let __mc_input = (inGraph.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, mut n) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(p.clone(), (n.clone()).clone())?;
            let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            FNode::addChildRef(p.clone(), (n.clone()).clone(), r.clone(), false)?;
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, mut n) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            r = FNode::child(p.clone(), (n.clone()).clone())?;
            r = FNode::copyRefNoUpdate(r.clone())?;
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, mut n) = __mc_input.clone() else { bail!("nomatch") };
            let mut no: Node;
            let mut r: Ref;
            (g, no) = node(g.clone(), (n.clone()).clone(), list![p.clone()], FCore::Data::ND { scopeType: inScopeType.clone() });
            r = FNode::toRef(no.clone());
            g = pushScopeRef(g.clone(), r.clone())?;
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.openScope: failed to open new scope in scope: ")); __mm_s.push_str(&*getGraphNameStr(inGraph.clone())); __mm_s.push_str(&*literal!(" name: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn inForLoopScope(mut inGraph: Graph) -> bool {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr;
            name = (FNode::refName(listHead(currentScope(inGraph.clone())?)?)?).clone();
            let true = (stringEq((name.clone()).clone(), (arcstr::literal!(FCore::forScopeName)).clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    res
}

pub(crate) fn inForOrParforIterLoopScope(mut inGraph: Graph) -> bool {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut name: ArcStr;
            name = (FNode::refName(listHead(currentScope(inGraph.clone())?)?)?).clone();
            let true = (stringEq((name.clone()).clone(), (arcstr::literal!(FCore::forIterScopeName)).clone()) || stringEq((name.clone()).clone(), (arcstr::literal!(FCore::parForIterScopeName)).clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    res
}

pub fn getScopePath(mut inGraph: Graph) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outPath: Option<Arc<Absyn::Path>>;
    outPath = 'mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r: Ref;
            let __pa0 = ::match_deref::match_deref! { match &(currentScope(inGraph.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            let true = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            Ok(None)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut p: Arc<Absyn::Path>;
            p = getGraphName(inGraph.clone())?;
            Ok(Some(p.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

pub(crate) fn getGraphNameStr(mut inGraph: Graph) -> ArcStr {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = inGraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(AbsynUtil::pathString(getGraphName(inGraph.clone())?, (literal!(".")).clone(), true, false)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!("."))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    outString
}

pub fn getGraphName(mut inGraph: Graph) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut p: Arc<Absyn::Path>;
    let mut s: Scope;
    let mut r: Ref;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(currentScope(inGraph)?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r = __pa0.clone();
    s = __pa1.clone();
    p = AbsynUtil::makeIdentPathFromString((FNode::refName(r.clone())?).clone());
    for mut r in &*s {
        let mut r = r.clone();
        p = Arc::new(Absyn::Path::QUALIFIED { name: (FNode::refName(r.clone())?).clone(), path: p.clone() });
    }
    let __pa2 = ::match_deref::match_deref! { match &(p) {
        Deref @ Absyn::Path::QUALIFIED { name: _, path: __pa2 } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outPath = __pa2.clone();
    Ok(outPath)
}

pub(crate) fn getGraphNameNoImplicitScopes(mut inGraph: Graph) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut s: Scope;
    let __pa0 = ::match_deref::match_deref! { match &(currentScope(inGraph)?.reverse()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    s = __pa0.clone();
    outPath = AbsynUtil::stringListPath(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut r#str in (({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (s).into_iter().cloned() {
            let __x = FNode::refName(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })).into_iter().cloned() {
            if !(stringGet((r#str.clone()).clone(),1)? != 36) { continue; }
            let __x = r#str.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
    Ok(outPath)
}

pub(crate) fn pushScopeRef(mut graph: Graph, mut inRef: Ref) -> Result<Graph> {
    let mut graph: Graph = graph;
    let () = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = metamodelica::cons(inRef.clone(), var_field!(graph.scope, FCore::Graph::G).clone());
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(graph)
}

pub(crate) fn pushScope(mut graph: Graph, mut inScope: Scope) -> Result<Graph> {
    let mut graph: Graph = graph;
    let () = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = listAppend(inScope, var_field!(graph.scope, FCore::Graph::G).clone());
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(graph)
}

pub(crate) fn setScope(mut graph: Graph, mut inScope: Scope) -> Result<Graph> {
    let mut graph: Graph = graph;
    let () = (match graph.clone() {
        FCore::Graph::G { .. } => {
            let __owned_variant_scope_0 = inScope;
            if let FCore::Graph::G { scope, .. } = &mut graph {
                *scope = __owned_variant_scope_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Graph::G"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(graph)
}

pub fn restrictionToScopeType(mut inRestriction: SCode::Restriction) -> Option<FCore::ScopeType> {
    let mut outType: Option<FCore::ScopeType>;
    outType = (match inRestriction {
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } } => Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE),
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. } } => Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE),
        SCode::Restriction::R_FUNCTION { functionRestriction: _ } => Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE),
        _ => Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE),
    });
    outType
}

pub(crate) fn scopeTypeToRestriction(mut inScopeType: FCore::ScopeType) -> SCode::Restriction {
    let mut outRestriction: SCode::Restriction;
    outRestriction = (match inScopeType {
        FCore::ScopeType::PARALLEL_SCOPE { .. } => SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_PARALLEL_FUNCTION },
        FCore::ScopeType::FUNCTION_SCOPE { .. } => SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: openmodelica_ast::Absyn::FunctionPurity::NO_PURITY } },
        _ => openmodelica_frontend_types::SCode::Restriction::R_CLASS,
    });
    outRestriction
}

pub(crate) fn isTopScope(mut graph: Graph) -> bool {
    let mut isTop: bool;
    isTop = 'mc: {
        let __mc_input = graph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (FNode::isRefTop(lastScopeRef(graph.clone())?)?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    isTop
}

pub(crate) fn crefStripGraphScopePrefix(mut inCref: Arc<Absyn::ComponentRef>, mut inEnv: Graph, mut stripPartial: bool) -> Arc<Absyn::ComponentRef> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = stripPartial;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { bail!("pattern mismatch") };
            Ok(inCref.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut env_path: Arc<Absyn::Path>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(getScopePath(inEnv.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            env_path = __pa0.clone();
            cref1 = AbsynUtil::unqualifyCref(inCref.clone());
            env_path = AbsynUtil::makeNotFullyQualified(env_path.clone());
            cref2 = crefStripGraphScopePrefix2(cref1.clone(), env_path.clone(), stripPartial)?;
            let false = (AbsynUtil::crefEqual(cref1.clone(), cref2.clone())?) else { bail!("pattern mismatch") };
            Ok(cref2.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inCref.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn crefStripGraphScopePrefix2(mut inCref: Arc<Absyn::ComponentRef>, mut inEnvPath: Arc<Absyn::Path>, mut stripPartial: bool) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inEnvPath, stripPartial);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::List::Nil, componentRef: cref }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: env_path }, _) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(crefStripGraphScopePrefix2(cref.clone(), env_path.clone(), stripPartial)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::List::Nil, componentRef: cref }, Deref @ Absyn::Path::IDENT { name: id2 }, _) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::List::Nil, .. }, env_path, true) => {
                    let false = (stringEqual((id1.clone()).clone(), (AbsynUtil::pathFirstIdent(env_path.clone())?).clone())) else { bail!("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

pub(crate) fn pathStripGraphScopePrefix(mut inPath: Arc<Absyn::Path>, mut inEnv: Graph, mut stripPartial: bool) -> Arc<Absyn::Path> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = stripPartial;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { bail!("pattern mismatch") };
            Ok(inPath.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut env_path: Arc<Absyn::Path>;
            let mut path1: Arc<Absyn::Path>;
            let mut path2: Arc<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(getScopePath(inEnv.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            env_path = __pa0.clone();
            path1 = AbsynUtil::makeNotFullyQualified(inPath.clone());
            env_path = AbsynUtil::makeNotFullyQualified(env_path.clone());
            path2 = pathStripGraphScopePrefix2(path1.clone(), env_path.clone(), stripPartial)?;
            let false = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
            Ok(path2.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inPath.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outPath
}

fn pathStripGraphScopePrefix2(mut inPath: Arc<Absyn::Path>, mut inEnvPath: Arc<Absyn::Path>, mut stripPartial: bool) -> Result<Arc<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPath.clone(), inEnvPath, stripPartial)) {
        (Deref @ Absyn::Path::QUALIFIED { name: id1, path }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: env_path }, _) if (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) => {
            { (inPath, inEnvPath, stripPartial) = (path.clone(), env_path.clone(), stripPartial); continue '__tco; }
        },
        (Deref @ Absyn::Path::QUALIFIED { name: id1, path }, Deref @ Absyn::Path::IDENT { name: id2 }, _) if (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) => {
            return Ok(path.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { name: id1, .. }, env_path, true) if (!(stringEqual((id1.clone()).clone(), (AbsynUtil::pathFirstIdent(env_path.clone())?).clone()))) => {
            return Ok(inPath)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn mkComponentNode(mut inGraph: Graph, mut inVar: Arc<DAE::Var>, mut inVarEl: Arc<SCode::Element>, mut inMod: Arc<DAE::Mod>, mut instStatus: Status, mut inCompGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph, inVar, inVarEl, inMod, instStatus, inCompGraph);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Var { name: n, .. }, c, _, _, _) => {
                    let false = (stringEq((n.clone()).clone(), (SCodeUtil::elementName(c.clone())?).clone())) else { bail!("pattern mismatch") };
                    Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.mkComponentNode: The component name: ")); __mm_s.push_str(&*SCodeUtil::elementName(c.clone())?); __mm_s.push_str(&*literal!(" is not the same as its DAE.TYPES_VAR: ")); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, v @ Deref @ DAE::Var { name: n, .. }, c, m, i, cg) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let true = (stringEq((n.clone()).clone(), (SCodeUtil::elementName(c.clone())?).clone())) else { bail!("pattern mismatch") };
                    r = lastScopeRef(g.clone())?;
                    g = FGraphBuildEnv::mkCompNode(c.clone(), r.clone(), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone())?;
                    g = updateVarAndMod(g.clone(), v.clone(), m.clone(), i.clone(), cg.clone());
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn mkClassNode(mut inGraph: Graph, mut inClass: Arc<SCode::Element>, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut checkDuplicate: bool) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = 'mc: {
        let __mc_input = (inGraph, inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ SCode::Element::CLASS { name: n, .. }) => {
                    let mut r: Ref;
                    r = lastScopeRef(g.clone())?;
                    r = FNode::child(r.clone(), (n.clone()).clone())?;
                    let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ SCode::Element::CLASS { .. }) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = lastScopeRef(g.clone())?;
                    g = FGraphBuildEnv::mkClassNode(inClass.clone(), inPrefix.clone(), inMod.clone(), r.clone(), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone(), checkDuplicate)?;
                    Ok(g.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

pub(crate) fn mkTypeNode(mut inGraph: Graph, mut inName: Name, mut inType: Arc<DAE::Type>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkTypeNode(list![inType], r.clone(), (inName).clone(), g.clone())?;
            g.clone()
        },
    });
    Ok(outGraph)
}

pub(crate) fn mkImportNode(mut inGraph: Graph, mut inImport: Arc<SCode::Element>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkElementNode(inImport, r.clone(), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone())?;
            g.clone()
        },
    });
    Ok(outGraph)
}

pub(crate) fn mkDefunitNode(mut inGraph: Graph, mut inDu: Arc<SCode::Element>) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = lastScopeRef(g.clone())?;
            g = FGraphBuildEnv::mkElementNode(inDu, r.clone(), openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone())?;
            g.clone()
        },
    });
    Ok(outGraph)
}

pub(crate) fn classInfToScopeType(mut inState: ClassInf::State) -> Option<FCore::ScopeType> {
    let mut outType: Option<FCore::ScopeType>;
    outType = (match inState {
        ClassInf::State::FUNCTION { .. } => Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE),
        _ => Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE),
    });
    outType
}

pub(crate) fn isEmpty(mut inGraph: Graph) -> bool {
    let mut b: bool;
    b = (match inGraph {
        FCore::Graph::EG { name: _ } => true,
        _ => false,
    });
    b
}

pub fn isNotEmpty(mut inGraph: Graph) -> bool {
    let mut b: bool;
    b = !(isEmpty(inGraph));
    b
}

pub(crate) fn isEmptyScope(mut graph: Graph) -> bool {
    let mut isEmpty: bool;
    match '__try0: {
        isEmpty = FCore::RefTree::isEmpty(unwrap_break_err!(FNode::children(unwrap_break_err!(FNode::fromRef(unwrap_break_err!(lastScopeRef(graph.clone()), '__try0)), '__try0)), '__try0));
        Ok::<_, anyhow::Error>((isEmpty.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isEmpty = __try0_o0;
        }
        Err(_) => {
            isEmpty = true;
        }
    }
    isEmpty
}

pub(crate) fn printGraphStr(mut inGraph: Graph) -> ArcStr {
    let mut s: ArcStr;
    s = (literal!("NOT IMPLEMENTED YET")).clone();
    s
}

pub(crate) fn inFunctionScope(mut inGraph: Graph) -> bool {
    let mut inFunction: bool;
    inFunction = (match inGraph {
        FCore::Graph::G { scope: ref s, .. } if (checkScopeType(s.clone(), Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE)) || checkScopeType(s.clone(), Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE))) => {
            true
        },
        _ => {
            false
        },
    });
    inFunction
}

pub(crate) fn getScopeName(mut inGraph: Graph) -> Result<Name> {
    let mut name: Name = arcstr::literal!("");
    name = ((match inGraph.clone() {
        _ => {
            let mut r: Ref;
            r = lastScopeRef(inGraph)?;
            let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            name = (FNode::refName(r.clone())?).clone();
            name
        },
    })).clone();
    Ok(name)
}

pub(crate) fn checkScopeType(mut inScope: Scope, mut inScopeType: Option<FCore::ScopeType>) -> bool {
    let mut yes: bool;
    yes = 'mc: {
        let __mc_input = inScope;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let mut restr: SCode::Restriction;
                    let true = (FNode::isRefClass(r.clone())?) else { bail!("pattern mismatch") };
                    restr = SCodeUtil::getClassRestriction(FNode::getElement(FNode::fromRef(r.clone())?)?)?;
                    let true = (restrictionToScopeType(restr.clone()) == inScopeType.clone()) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let mut st: Option<FCore::ScopeType>;
                    let FCore::N { data: FCore::ND { scopeType: __pa0 }, .. } = (FNode::fromRef(r.clone())?) else { bail!("pattern mismatch") };
                    st = __pa0.clone();
                    let true = (st.clone() == inScopeType.clone()) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(checkScopeType(rest.clone(), inScopeType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    yes
}

pub(crate) fn lastScopeRestriction(mut inGraph: Graph) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    let mut s: Scope;
    let FCore::G { scope: __pa0, .. } = (inGraph) else { bail!("pattern mismatch") };
    s = __pa0.clone();
    outRestriction = getScopeRestriction(s)?;
    Ok(outRestriction)
}

pub(crate) fn getScopeRestriction(mut inScope: Scope) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    outRestriction = 'mc: {
        let __mc_input = inScope.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    if !((FNode::isRefClass(r.clone())?)) { bail!("guard") }
                    Ok(SCodeUtil::getClassRestriction(FNode::getElement(FNode::fromRef(r.clone())?)?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: _ } => {
                    let mut st: FCore::ScopeType;
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())?) {
                        FCore::Node { data: FCore::Data::ND { scopeType: Some(__pa0) }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    st = __pa0.clone();
                    Ok(scopeTypeToRestriction(st.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(getScopeRestriction(listRest(inScope.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRestriction)
}

pub(crate) fn getGraphPathNoImplicitScope(mut inGraph: Graph) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outAbsynPathOption: Option<Arc<Absyn::Path>>;
    outAbsynPathOption = getGraphPathNoImplicitScope_dispatch(currentScope(inGraph)?);
    Ok(outAbsynPathOption)
}

fn getGraphPathNoImplicitScope_dispatch(mut inScope: Scope) -> Option<Arc<Absyn::Path>> {
    let mut outAbsynPathOption: Option<Arc<Absyn::Path>>;
    let mut opath: Option<Arc<Absyn::Path>> = None;
    outAbsynPathOption = 'mc: {
        let __mc_input = inScope;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#ref, tail: rest } => {
                    if !((!(FNode::isRefTop(r#ref.clone())?))) { bail!("guard") }
                    let mut id: Name;
                    let mut path: Arc<Absyn::Path>;
                    let mut path_1: Arc<Absyn::Path>;
                    let mut opath: Option<Arc<Absyn::Path>> = opath.clone();
                    id = (FNode::refName(r#ref.clone())?).clone();
                    if isImplicitScope((id.clone()).clone()) {
                        opath = getGraphPathNoImplicitScope_dispatch(rest.clone());
                    } else {
                        opath = getGraphPathNoImplicitScope_dispatch(rest.clone());
                        if isSome(opath.clone()) {
                            let __pa0 = ::match_deref::match_deref! { match &(opath.clone()) {
                                        Some(__pa0) => __pa0.clone(),
                                        _ => bail!("pattern mismatch"),
                            } };
                            path = __pa0.clone();
                            path_1 = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                            opath = Some(path_1.clone());
                        } else {
                            opath = Some(Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }));
                        }
                    }
                    Ok((opath.clone(), opath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { opath = __wb0; break 'mc __v; }
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
    outAbsynPathOption
}

pub(crate) fn isImplicitScope(mut inName: Name) -> bool {
    let mut isImplicit: bool;
    isImplicit = FCore::isImplicitScope((inName).clone());
    isImplicit
}

pub(crate) fn joinScopePath(mut inGraph: Graph, mut inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut opath: Option<Arc<Absyn::Path>>;
    let mut envPath: Arc<Absyn::Path>;
    opath = getScopePath(inGraph)?;
    if isSome(opath.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(opath) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        envPath = __pa0.clone();
        outPath = AbsynUtil::joinPaths(envPath, inPath)?;
    } else {
        outPath = inPath;
    }
    Ok(outPath)
}

pub(crate) fn splitGraphScope(mut inGraph: Graph) -> Result<(Graph, Scope)> {
    let mut outRealGraph: Graph;
    let mut outForScope: Scope;
    (outRealGraph, outForScope) = splitGraphScope_dispatch(inGraph, metamodelica::nil())?;
    Ok((outRealGraph, outForScope))
}

pub(crate) fn splitGraphScope_dispatch(mut inGraph: Graph, mut inAcc: Scope) -> Result<(Graph, Scope)> {
    let mut outRealGraph: Graph;
    let mut outForScope: Scope;
    (outRealGraph, outForScope) = (::match_deref::match_deref! { match &(inGraph.clone()) {
        FCore::Graph::EG { name: _ } => {
            (inGraph, inAcc.reverse())
        },
        FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. } => {
            let mut g: Graph;
            let mut s: Scope;
            if FNode::isImplicitRefName(r.clone())? {
                (g, _) = stripLastScopeRef(inGraph)?;
                (g, s) = splitGraphScope_dispatch(g.clone(), metamodelica::cons(r.clone(), inAcc))?;
            } else {
                g = inGraph;
                s = inAcc.reverse();
            }
            (g.clone(), s.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outRealGraph, outForScope))
}

pub(crate) fn getVariablesFromGraphScope(mut inGraph: Graph) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut variables: Arc<metamodelica::List<ArcStr>>;
    variables = (::match_deref::match_deref! { match &(inGraph) {
        FCore::Graph::EG { name: _ } => {
            metamodelica::nil()
        },
        FCore::Graph::G { scope: Deref @ metamodelica::List::Nil, .. } => {
            metamodelica::nil()
        },
        FCore::Graph::G { scope: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. } => {
            let mut lst: Arc<metamodelica::List<ArcStr>>;
            lst = List::map(FNode::filter(r.clone(), (std::sync::Arc::new(FNode::isRefComponent) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>))?, (std::sync::Arc::new(FNode::refName) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<ArcStr> + 'static>))?;
            lst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(variables)
}

pub(crate) fn removeComponentsFromScope(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut r: Ref;
    let mut n: Node;
    r = lastScopeRef(inGraph.clone())?;
    r = FNode::copyRefNoUpdate(r.clone())?;
    n = FNode::fromRef(r.clone())?;
    n = FNode::setChildren(n, FCore::RefTree::new())?;
    r = FNode::updateRef(r.clone(), n)?;
    (outGraph, _) = stripLastScopeRef(inGraph)?;
    outGraph = pushScopeRef(outGraph, r.clone())?;
    Ok(outGraph)
}

pub(crate) fn cloneLastScopeRef(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    let mut r: Ref;
    (outGraph, r) = stripLastScopeRef(inGraph)?;
    r = FNode::copyRefNoUpdate(r.clone())?;
    outGraph = pushScopeRef(outGraph, r.clone())?;
    Ok(outGraph)
}

pub(crate) fn updateScope(mut inGraph: Graph) -> Graph {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        _ => inGraph,
    });
    outGraph
}

pub(crate) fn mkVersionNode(mut inSourceEnv: Graph, mut inSourceName: Name, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inTargetClassEnv: Graph, mut inTargetClass: Arc<SCode::Element>, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>) -> Result<(Graph, Arc<SCode::Element>, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outVersionedTargetClassEnv: Graph;
    let mut outVersionedTargetClass: Arc<SCode::Element>;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>>;
    (outVersionedTargetClassEnv, outVersionedTargetClass, outIH) = 'mc: {
        let __mc_input = inIH.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut gclass: Graph;
                    let mut classRef: Ref;
                    let mut sourceRef: Ref;
                    let mut targetClassParentRef: Ref;
                    let mut crefPrefix: DAE::Prefix;
                    let mut c: Arc<SCode::Element>;
                    let mut targetClassName: Name;
                    let mut newTargetClassName: Name;
                    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>;
                    c = inTargetClass.clone();
                    gclass = inTargetClassEnv.clone();
                    targetClassName = (SCodeUtil::elementName(c.clone())?).clone();
                    (newTargetClassName, crefPrefix) = mkVersionName(inSourceEnv.clone(), (inSourceName.clone()).clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), (targetClassName.clone()).clone())?;
                    sourceRef = FNode::child(lastScopeRef(inSourceEnv.clone())?, (inSourceName.clone()).clone())?;
                    targetClassParentRef = lastScopeRef(inTargetClassEnv.clone())?;
                    classRef = FNode::child(targetClassParentRef.clone(), (targetClassName.clone()).clone())?;
                    classRef = FNode::copyRefNoUpdate(classRef.clone())?;
                    let FCore::CL { e: __pa0, .. } = (FNode::refData(classRef.clone())?) else { bail!("pattern mismatch") };
                    c = __pa0.clone();
                    c = SCodeUtil::setClassName((newTargetClassName.clone()).clone(), c.clone())?;
                    classRef = updateClassElement(classRef.clone(), c.clone(), crefPrefix.clone(), inMod.clone(), FCore::Status::CLS_INSTANCE { instanceOf: (targetClassName.clone()).clone() }, empty())?;
                    FNode::addChildRef(targetClassParentRef.clone(), (newTargetClassName.clone()).clone(), classRef.clone(), false)?;
                    sourceRef = updateSourceTargetScope(sourceRef.clone(), metamodelica::cons(classRef.clone(), currentScope(gclass.clone())?))?;
                    ih = inIH.clone();
                    Ok((gclass.clone(), c.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut c: Arc<SCode::Element>;
                    let mut targetClassName: Name;
                    let mut newTargetClassName: Name;
                    c = inTargetClass.clone();
                    targetClassName = (SCodeUtil::elementName(c.clone())?).clone();
                    (newTargetClassName, _) = mkVersionName(inSourceEnv.clone(), (inSourceName.clone()).clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), (targetClassName.clone()).clone())?;
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.mkVersionNode: failed to create version node:\n")); __mm_s.push_str(&*literal!("Instance: CL(")); __mm_s.push_str(&*getGraphNameStr(inSourceEnv.clone())); __mm_s.push_str(&*literal!(").CO(")); __mm_s.push_str(&*inSourceName.clone()); __mm_s.push_str(&*literal!(").CL(")); __mm_s.push_str(&*getGraphNameStr(inTargetClassEnv.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*targetClassName.clone()); __mm_s.push_str(&*SCodeDump::printModStr(Mod::unelabMod(inMod.clone())?, SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(")\n\t")); __mm_s.push_str(&*newTargetClassName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVersionedTargetClassEnv, outVersionedTargetClass, outIH))
}

pub(crate) fn createVersionScope(mut inSourceEnv: Graph, mut inSourceName: Name, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inTargetClassEnv: Graph, mut inTargetClass: Arc<SCode::Element>, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>) -> Result<(Graph, Arc<SCode::Element>, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outVersionedTargetClassEnv: Graph;
    let mut outVersionedTargetClass: Arc<SCode::Element>;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outVersionedTargetClassEnv, outVersionedTargetClass, outIH) = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::NOMOD { .. } => {
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. } => {
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Config::acceptMetaModelicaGrammar()? || isTargetClassBuiltin(inTargetClassEnv.clone(), inTargetClass.clone()) || inFunctionScope(inSourceEnv.clone()) || SCodeUtil::isOperatorRecord(inTargetClass.clone())) else { bail!("pattern mismatch") };
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (stringEq((AbsynUtil::pathFirstIdent(getGraphName(inTargetClassEnv.clone())?)?).clone(), (literal!("OpenModelica")).clone())) else { bail!("pattern mismatch") };
                    Ok((inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut gclass: Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    (gclass, c, outIH) = mkVersionNode(inSourceEnv.clone(), (inSourceName.clone()).clone(), inPrefix.clone(), inMod.clone(), inTargetClassEnv.clone(), inTargetClass.clone(), inIH.clone())?;
                    Ok(((gclass.clone(), c.clone(), outIH.clone()), outIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outIH = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVersionedTargetClassEnv, outVersionedTargetClass, outIH))
}

pub(crate) fn isTargetClassBuiltin(mut inGraph: Graph, mut inClass: Arc<SCode::Element>) -> bool {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r: Ref;
                    let mut yes: bool = yes.clone();
                    r = FNode::child(lastScopeRef(inGraph.clone())?, (SCodeUtil::elementName(inClass.clone())?).clone())?;
                    yes = FNode::isRefBasicType(r.clone())? || FNode::isRefBuiltin(r.clone())?;
                    Ok((yes, yes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { yes = __wb0; break 'mc __v; }
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
    yes
}

pub(crate) fn mkVersionName(mut inSourceEnv: Graph, mut inSourceName: Name, mut inPrefix: DAE::Prefix, mut inMod: Arc<DAE::Mod>, mut inTargetClassEnv: Graph, mut inTargetClassName: Name) -> Result<(Name, DAE::Prefix)> {
    let mut outName: Name;
    let mut outCrefPrefix: DAE::Prefix;
    (outName, outCrefPrefix) = (match inTargetClassName.clone() {
        _ => {
            let mut crefPrefix: DAE::Prefix;
            let mut name: Name;
            crefPrefix = PrefixUtil::prefixAdd((inSourceName).clone(), metamodelica::nil(), metamodelica::nil(), inPrefix, openmodelica_frontend_types::SCode::Variability::CONST, ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, Absyn::dummyInfo.clone())?;
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inTargetClassName); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*AbsynUtil::pathString(AbsynUtil::stringListPath(AbsynUtil::pathToStringList(PrefixUtil::prefixToPath(crefPrefix.clone())?)?.reverse())?, (literal!("$")).clone(), false, false)?); ArcStr::from(__mm_s) }).clone();
            (name.clone(), crefPrefix.clone())
        },
    });
    Ok((outName, outCrefPrefix))
}

pub(crate) fn getClassPrefix(mut inEnv: FCore::Graph, mut inClassName: Name) -> DAE::Prefix {
    let mut outPrefix: DAE::Prefix;
    outPrefix = 'mc: {
        let __mc_input = inClassName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut p: DAE::Prefix;
            let mut r: Ref;
            r = FNode::child(lastScopeRef(inEnv.clone())?, (inClassName.clone()).clone())?;
            let FCore::CL { pre: __pa0, .. } = (FNode::refData(r.clone())?) else { bail!("pattern mismatch") };
            p = __pa0.clone();
            Ok(p.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(openmodelica_frontend_types::DAE::Prefix::NOPRE)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outPrefix
}

pub(crate) fn isInstance(mut inEnv: FCore::Graph, mut inName: ArcStr) -> bool {
    let mut yes: bool;
    yes = 'mc: {
        let __mc_input = inName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: _ }, .. } = (FNode::refData(FNode::child(lastScopeRef(inEnv.clone())?, (inName.clone()).clone())?)?) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    yes
}

pub(crate) fn getInstanceOriginalName(mut inEnv: FCore::Graph, mut inName: ArcStr) -> ArcStr {
    let mut outName: ArcStr = arcstr::literal!("");
    outName = ('mc: {
        let __mc_input = inName.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut outName: ArcStr = outName.clone();
            let FCore::CL { status: FCore::CLS_INSTANCE { instanceOf: __pa0 }, .. } = (FNode::refData(FNode::child(lastScopeRef(inEnv.clone())?, (inName.clone()).clone())?)?) else { bail!("pattern mismatch") };
            outName = __pa0.clone();
            Ok((outName.clone(), outName.clone()))
        })() { outName = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inName.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    outName
}

pub(crate) fn graphPrefixOf(mut inPrefixEnv: Graph, mut inEnv: Graph) -> Result<bool> {
    let mut outIsPrefix: bool;
    outIsPrefix = graphPrefixOf2(currentScope(inPrefixEnv)?.reverse(), currentScope(inEnv)?.reverse())?;
    Ok(outIsPrefix)
}

pub(crate) fn graphPrefixOf2(mut inPrefixEnv: Scope, mut inEnv: Scope) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPrefixEnv, inEnv)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            return Ok(true)
        },
        (Deref @ metamodelica::List::Cons { head: r1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: r2, tail: rest2 }) if (stringEq((FNode::refName(r1.clone())?).clone(), (FNode::refName(r2.clone())?).clone())) => {
            { (inPrefixEnv, inEnv) = (rest1.clone(), rest2.clone()); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn setStatus(mut inEnv: Graph, mut inName: Name, mut inStatus: FCore::Data) -> Result<Graph> {
    let mut outEnv: Graph;
    outEnv = 'mc: {
        let __mc_input = inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            let mut n: Node;
            let mut r#ref: Ref;
            let mut refParent: Ref;
            refParent = lastScopeRef(g.clone())?;
            if FNode::refHasChild(refParent.clone(), (inName.clone()).clone())? {
                r#ref = FNode::child(refParent.clone(), (inName.clone()).clone())?;
                if FNode::refHasChild(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())? {
                    r#ref = FNode::child(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())?;
                    n = FNode::setData(FNode::fromRef(r#ref.clone())?, inStatus.clone())?;
                    r#ref = FNode::updateRef(r#ref.clone(), n.clone())?;
                } else {
                    (g, n) = node(g.clone(), (arcstr::literal!(FNode::statusNodeName)).clone(), list![r#ref.clone()], inStatus.clone());
                    FNode::addChildRef(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone(), FNode::toRef(n.clone()), false)?;
                }
            }
            Ok(g.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph.setStatus failed on: ")); __mm_s.push_str(&*getGraphNameStr(g.clone())); __mm_s.push_str(&*literal!(" element: ")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(g.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

pub(crate) fn getStatus(mut inEnv: Graph, mut inName: Name) -> Result<FCore::Data> {
    let mut outStatus: FCore::Data;
    outStatus = (match inEnv {
        mut g => {
            let mut r#ref: Ref;
            let mut refParent: Ref;
            let mut s: FCore::Data;
            refParent = lastScopeRef(g.clone())?;
            let true = (FNode::refHasChild(refParent.clone(), (inName.clone()).clone())?) else { bail!("pattern mismatch") };
            r#ref = FNode::child(refParent.clone(), (inName).clone())?;
            let true = (FNode::refHasChild(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())?) else { bail!("pattern mismatch") };
            r#ref = FNode::child(r#ref.clone(), (arcstr::literal!(FNode::statusNodeName)).clone())?;
            s = FNode::refData(r#ref.clone())?;
            s.clone()
        },
        _ => {
            bail!("fail")
        },
    });
    Ok(outStatus)
}

pub(crate) fn selectScope(mut inEnv: Graph, mut inPath: Arc<Absyn::Path>) -> Result<Graph> {
    let mut outEnv: Graph;
    outEnv = (::match_deref::match_deref! { match &(inPath.clone()) {
        _ => {
            let mut env: Graph;
            let mut pl: Arc<metamodelica::List<ArcStr>>;
            let mut lp: i32;
            let mut le: i32;
            let mut diff: i32;
            let mut cs: Scope;
            let mut p: Arc<Absyn::Path>;
            p = AbsynUtil::stripLast(inPath)?;
            let true = (AbsynUtil::pathPrefixOf(p.clone(), getGraphName(inEnv.clone())?)) else { bail!("pattern mismatch") };
            pl = AbsynUtil::pathToStringList(p.clone())?;
            lp = (pl.clone().len() as i32);
            cs = currentScope(inEnv.clone())?;
            le = (cs.clone().len() as i32) - 1;
            diff = le.clone() - lp.clone();
            cs = List::stripN(cs.clone(), diff.clone())?;
            env = setScope(inEnv, cs.clone())?;
            env.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

pub(crate) fn makeScopePartial(mut inEnv: Graph) -> Graph {
    let mut outEnv: Graph = inEnv.clone();
    let mut node: Node;
    let mut data: Data = FCore::Data::TOP;
    let mut el: Arc<SCode::Element> = Arc::new(<SCode::Element as ::std::default::Default>::default());
    if '__try0: {
        node = unwrap_break_err!(FNode::fromRef(unwrap_break_err!(lastScopeRef(inEnv.clone()), '__try0)), '__try0);
        node = (match node.clone() {
        FCore::Node { data: ref __esc_data @ FCore::Data::CL { e: ref __esc_el, .. }, .. } => {
            data = __esc_data.clone();
            el = __esc_el.clone();
            el = SCodeUtil::makeClassPartial(el.clone());
            let __owned_variant_e_0 = el.clone();
            if let FCore::Data::CL { e, .. } = &mut data {
                *e = __owned_variant_e_0;
            } else { panic!("owned-variant field-assign: value held a different variant than FCore::Data::CL"); }
            node.data = data.clone();
            node.clone()
        },
        _ => node.clone(),
    });
        outEnv = unwrap_break_err!(setLastScopeRef(FNode::toRef(node.clone()), outEnv.clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    outEnv
}

pub(crate) fn isPartialScope(mut inEnv: Graph) -> bool {
    let mut outIsPartial: bool;
    let mut el: Arc<SCode::Element>;
    match '__try0: {
        let FCore::N { data: FCore::CL { e: __pa1, .. }, .. } = (unwrap_break_err!(FNode::fromRef(unwrap_break_err!(lastScopeRef(inEnv.clone()), '__try0)), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        el = __pa1.clone();
        outIsPartial = SCodeUtil::isPartial(el.clone());
        Ok::<_, anyhow::Error>((outIsPartial.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outIsPartial = __try0_o0;
        }
        Err(_) => {
            outIsPartial = false;
        }
    }
    outIsPartial
}

