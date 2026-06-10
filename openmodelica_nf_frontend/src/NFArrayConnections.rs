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

use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Purity;
use crate::NFSBGraphUtil as SBGraphUtil;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::SBAtomicSet;
use openmodelica_util::SBFunctions;
use openmodelica_util::SBGraph::IncidenceList;
use openmodelica_util::SBGraph::VertexDescriptor;
use openmodelica_util::SBInterval;
use openmodelica_util::SBMultiInterval;
use openmodelica_util::SBPWLinearMap;
use openmodelica_util::SBSet;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub mod SetVertex {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SetVertex {
        pub name: Arc<Connector::NFConnector>,
        pub vs: Arc<SBSet::SBSet>,
    }

    impl metamodelica::gc::MMTrace for SetVertex {
        fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vs, __mmv)?;
            Ok(())
        }
    }
    impl Default for SetVertex {
        fn default() -> Self {
            Self {
                name: Default::default(),
                vs: Default::default(),
            }
        }
    }

    pub type SET_VERTEX = SetVertex;

    pub fn isEqual(mut v1: Arc<SetVertex>, mut v2: Arc<SetVertex>) -> Result<bool> {
        let mut equal: bool = Connector::isEqual(v1.name.clone(), v2.name.clone())?;
        Ok(equal)
    }

    pub fn isNamed(mut v: Arc<SetVertex>, mut name: Arc<Connector::NFConnector>) -> Result<bool> {
        let mut equal: bool = Connector::isEqual(v.name.clone(), name.clone())?;
        Ok(equal)
    }

    pub fn toString(mut v: Arc<SetVertex>) -> Result<ArcStr> {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*Connector::toString(v.name.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SBSet::toString(v.vs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
        Ok(r#str)
    }

}

pub mod SetEdge {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SetEdge {
        pub name: ArcStr,
        pub es1: Arc<SBPWLinearMap::SBPWLinearMap>,
        pub es2: Arc<SBPWLinearMap::SBPWLinearMap>,
    }

    impl metamodelica::gc::MMTrace for SetEdge {
        fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.es1, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.es2, __mmv)?;
            Ok(())
        }
    }
    impl Default for SetEdge {
        fn default() -> Self {
            Self {
                name: Default::default(),
                es1: Default::default(),
                es2: Default::default(),
            }
        }
    }

    pub type SET_EDGE = SetEdge;

    pub fn isEqual(mut e1: Arc<SetEdge>, mut e2: Arc<SetEdge>) -> bool {
        let mut equal: bool = e1.name.clone() == e2.name.clone();
        equal
    }

    pub fn toString(mut e: Arc<SetEdge>) -> Result<ArcStr> {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*e.name.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SetVertex 1:\t")); __mm_s.push_str(&*SBPWLinearMap::toString(e.es1.clone())?); __mm_s.push_str(&*literal!("\nSetVertex 2:\t")); __mm_s.push_str(&*SBPWLinearMap::toString(e.es2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
        Ok(r#str)
    }

}

pub type NameVertexTable = Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<SBMultiInterval::SBMultiInterval>>>;

pub type SBGraph = Arc<IncidenceList::IncidenceList<Arc<SetVertex::SetVertex>, Arc<SetEdge::SetEdge>>>;

pub fn resolve(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut max_dim: i32 = 1;
    let mut v_count: Arc<Vector::Vector<i32>>;
    let mut e_count: Arc<Vector::Vector<i32>>;
    let mut conns: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut graph: SBGraph;
    let mut vss: Arc<SBSet::SBSet>;
    let mut res: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut emap1: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut nmv_table: NameVertexTable;
    for mut var in &*flatModel.variables.clone() {
        let mut var = var.clone();
        max_dim = std::cmp::max(max_dim.clone(), Type::dimensionCount(var.ty.clone()));
    }
    v_count = Vector::newFill(max_dim.clone(), 1);
    e_count = Vector::newFill(max_dim.clone(), 1);
    (flatModel, conns) = collect(flatModel.clone())?;
    graph = IncidenceList::new((std::sync::Arc::new(SetVertex::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetVertex::SetVertex>, Arc<SetVertex::SetVertex>) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(SetEdge::isEqual, Arc<SetEdge::SetEdge>, Arc<SetEdge::SetEdge>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetEdge::SetEdge>, Arc<SetEdge::SetEdge>) -> Result<bool> + 'static>), (std::sync::Arc::new(SetVertex::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetVertex::SetVertex>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(SetEdge::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetEdge::SetEdge>) -> Result<ArcStr> + 'static>));
    nmv_table = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    createGraph(flatModel.variables.clone(), conns.clone(), graph.clone(), v_count.clone(), e_count.clone(), nmv_table.clone())?;
    if Flags::isSet(Flags::DUMP_SET_BASED_GRAPHS.clone())? {
        metamodelica::print((IncidenceList::toString(graph.clone())?).clone());
    }
    (vss, emap1, emap2) = createMaps(graph.clone())?;
    res = SBFunctions::connectedComponents(vss.clone(), emap1.clone(), emap2.clone())?;
    if Flags::isSet(Flags::DUMP_SET_BASED_GRAPHS.clone())? {
        metamodelica::print((IncidenceList::toString(graph.clone())?).clone());
    }
    conns = generateEquations(res.clone(), flatModel.clone(), graph.clone(), v_count.clone(), nmv_table.clone())?;
    eql = listAppend(flatModel.equations.clone(), conns.clone());
    assign_field!(flatModel.equations = eql.clone());
    Ok(flatModel)
}

fn collect(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut conns: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    (conns, eql) = List::splitOnTrue(flatModel.equations.clone(), (std::sync::Arc::new(fnptr!(isConnection, Arc<Equation::NFEquation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?;
    assign_field!(flatModel.equations = eql.clone());
    Ok((flatModel, conns))
}

fn isConnection(mut eq: Arc<Equation::NFEquation>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { .. } => {
            return true
        },
        Deref @ Equation::FOR { body: Deref @ metamodelica::List::Cons { head: e, tail: _ }, .. } => {
            { eq = e.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn createGraph(mut variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut eCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<()> {
    addFlowsToGraph(variables.clone(), graph.clone(), vCount.clone(), nmvTable.clone())?;
    addConnectionsToGraph(equations.clone(), graph.clone(), vCount.clone(), eCount.clone(), nmvTable.clone())?;
    Ok(())
}

fn addFlowsToGraph(mut variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<()> {
    let mut conn: Arc<Connector::NFConnector>;
    let mut parent_cr: Arc<ComponentRef::NFComponentRef>;
    for mut var in &*variables.clone() {
        let mut var = var.clone();
        if Variable::isFlow(var.clone()) {
            parent_cr = ComponentRef::rest(var.name.clone())?;
            conn = Connector::fromFacedCref(parent_cr.clone(), ComponentRef::nodeType(parent_cr.clone())?, Connector::Face::INSIDE.clone(), ElementSource::createElementSource(var.info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?)?;
            createVertex(conn.clone(), graph.clone(), vCount.clone(), nmvTable.clone())?;
        }
    }
    Ok(())
}

fn addConnectionsToGraph(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut eCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<()> {
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { .. } => {
            createConnection(var_field!((*eq).lhs, Equation::NFEquation::CONNECT).clone(), var_field!((*eq).rhs, Equation::NFEquation::CONNECT).clone(), var_field!((*eq).source, Equation::NFEquation::CONNECT).clone(), graph.clone(), vCount.clone(), eCount.clone(), nmvTable.clone())?;
            ()
        },
        Deref @ Equation::FOR { range: Some(__esc_range), .. } => {
            range = (*__esc_range).clone();
            range = Ceval::evalExp(range.clone(), Ceval::EvalTarget::new(Equation::info(eq.clone())?, NFInstContext::ITERATION_RANGE.clone(), None))?;
            body = Equation::replaceIteratorList(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(), range.clone())?;
            addConnectionsToGraph(body.clone(), graph.clone(), vCount.clone(), eCount.clone(), nmvTable.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFArrayConnections.addConnectionsToGraph")); __mm_s.push_str(&*literal!(" got unknown equation ")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFArrayConnections.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn createConnection(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut source: Arc<DAE::ElementSource>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut eCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<()> {
    let mut lhs_cr: Arc<ComponentRef::NFComponentRef>;
    let mut rhs_cr: Arc<ComponentRef::NFComponentRef>;
    let mut lhs_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut rhs_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut mi1: Arc<SBMultiInterval::SBMultiInterval>;
    let mut mi2: Arc<SBMultiInterval::SBMultiInterval>;
    let mut d1: i32;
    let mut d2: i32;
    let mut lhs_conn: Arc<Connector::NFConnector>;
    let mut rhs_conn: Arc<Connector::NFConnector>;
    (lhs_cr, lhs_subs) = separate(Expression::toCref(lhs.clone())?)?;
    (rhs_cr, rhs_subs) = separate(Expression::toCref(rhs.clone())?)?;
    lhs_conn = Connector::fromCref(lhs_cr.clone(), ComponentRef::nodeType(lhs_cr.clone())?, source.clone())?;
    rhs_conn = Connector::fromCref(rhs_cr.clone(), ComponentRef::nodeType(rhs_cr.clone())?, source.clone())?;
    (mi1, d1) = getConnectIntervals(lhs_conn.clone(), lhs_subs.clone(), graph.clone(), vCount.clone(), nmvTable.clone())?;
    (mi2, d2) = getConnectIntervals(rhs_conn.clone(), rhs_subs.clone(), graph.clone(), vCount.clone(), nmvTable.clone())?;
    updateGraph(d1.clone(), d2.clone(), mi1.clone(), mi2.clone(), graph.clone(), eCount.clone())?;
    Ok(())
}

fn separate(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    cref = ComponentRef::fillSubscripts(cref.clone());
    cref = ComponentRef::replaceWholeSubscripts(cref.clone())?;
    subs = ComponentRef::subscriptsAllFlat(cref.clone())?;
    cref = ComponentRef::stripSubscriptsAll(cref.clone());
    Ok((cref, subs))
}

fn getConnectIntervals(mut conn: Arc<Connector::NFConnector>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<(Arc<SBMultiInterval::SBMultiInterval>, i32)> {
    let mut outMI: Arc<SBMultiInterval::SBMultiInterval>;
    let mut d: i32;
    (outMI, d) = createVertex(conn.clone(), graph.clone(), vCount.clone(), nmvTable.clone())?;
    outMI = SBGraphUtil::multiIntervalFromSubscripts(subs.clone(), vCount.clone(), outMI.clone())?;
    Ok((outMI, d))
}

fn createVertex(mut conn: Arc<Connector::NFConnector>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<(Arc<SBMultiInterval::SBMultiInterval>, i32)> {
    let mut mi: Arc<SBMultiInterval::SBMultiInterval>;
    let mut d: i32;
    let mut od: Option<i32>;
    let mut v: Arc<SetVertex::SetVertex>;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>;
    let mut s: Arc<SBSet::SBSet>;
    let mut name: ArcStr;
    od = IncidenceList::findVertex(graph.clone(), (std::sync::Arc::new({ let __pe_b1 = conn.clone(); move |__pe_a0| SetVertex::isNamed(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SetVertex::SetVertex>) -> Result<bool> + 'static>))?;
    if isSome(od.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(od.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        d = __pa0.clone();
        v = IncidenceList::getVertex(graph.clone(), d.clone())?;
        mi = SBAtomicSet::aset(UnorderedSet::first(SBSet::asets(v.vs.clone()))?);
        return Ok((mi.clone(), d.clone()));
    }
    dims = crefDims(Connector::name(conn.clone()))?;
    mi = SBGraphUtil::multiIntervalFromDimensions(dims.clone(), vCount.clone())?;
    s = SBSet::newEmpty();
    s = SBSet::addAtomicSet(SBAtomicSet::new(mi.clone()), s.clone())?;
    v = Arc::new(SetVertex::SetVertex { name: conn.clone(), vs: s.clone() });
    d = IncidenceList::addVertex(graph.clone(), v.clone());
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Connector::toString(conn.clone())?); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*Connector::faceString(conn.clone())); ArcStr::from(__mm_s) }).clone();
    UnorderedMap::addUnique((name.clone()).clone(), mi.clone(), nmvTable.clone())?;
    Ok((mi, d))
}

fn crefDims(mut cr: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut c: Arc<ComponentRef::NFComponentRef> = cr.clone();
    while !(ComponentRef::isEmpty(c.clone())) {
        dims = listAppend(Type::arrayDims(ComponentRef::nodeType(c.clone())?), dims.clone());
        c = ComponentRef::rest(c.clone())?;
    }
    Ok(dims)
}

fn updateGraph(mut d1: i32, mut d2: i32, mut mi1: Arc<SBMultiInterval::SBMultiInterval>, mut mi2: Arc<SBMultiInterval::SBMultiInterval>, mut graph: SBGraph, mut eCount: Arc<Vector::Vector<i32>>) -> Result<()> {
    let mut pw1: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut pw2: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut name: ArcStr;
    let mut se: Arc<SetEdge::SetEdge>;
    (name, pw1, pw2) = SBGraphUtil::linearMapFromIntervals(d1.clone(), d2.clone(), mi1.clone(), mi2.clone(), eCount.clone())?;
    se = Arc::new(SetEdge::SetEdge { name: (name.clone()).clone(), es1: pw1.clone(), es2: pw2.clone() });
    IncidenceList::addEdge(graph.clone(), d1.clone(), d2.clone(), se.clone())?;
    Ok(())
}

fn createMaps(mut graph: SBGraph) -> Result<(Arc<SBSet::SBSet>, Arc<SBPWLinearMap::SBPWLinearMap>, Arc<SBPWLinearMap::SBPWLinearMap>)> {
    let mut vss: Arc<SBSet::SBSet>;
    let mut emap1: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: Arc<SBPWLinearMap::SBPWLinearMap>;
    let mut vs: Arc<metamodelica::List<Arc<SetVertex::SetVertex>>>;
    let mut es: Arc<metamodelica::List<Arc<SetEdge::SetEdge>>>;
    let mut e: Arc<SetEdge::SetEdge>;
    vss = SBSet::newEmpty();
    for mut v in &*IncidenceList::vertices(graph.clone()) {
        let mut v = v.clone();
        vss = SBSet::union(vss.clone(), v.vs.clone())?;
    }
    es = IncidenceList::edges(graph.clone());
    if es.clone().is_empty() {
        emap1 = SBPWLinearMap::newEmpty();
        emap2 = SBPWLinearMap::newEmpty();
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(IncidenceList::edges(graph.clone())) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        es = __pa1.clone();
        emap1 = e.es1.clone();
        emap2 = e.es2.clone();
        for mut e in &*es.clone() {
            let mut e = e.clone();
            emap1 = SBPWLinearMap::combine(e.es1.clone(), emap1.clone())?;
            emap2 = SBPWLinearMap::combine(e.es2.clone(), emap2.clone())?;
        }
    }
    Ok((vss, emap1, emap2))
}

fn generateEquations(mut pw: Arc<SBPWLinearMap::SBPWLinearMap>, mut flatModel: Arc<FlatModel::NFFlatModel>, mut graph: SBGraph, mut vCount: Arc<Vector::Vector<i32>>, mut nmvTable: NameVertexTable) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut vc_dom: Arc<SBSet::SBSet>;
    let mut vc_im: Arc<SBSet::SBSet>;
    let mut aux_s: Arc<SBSet::SBSet>;
    let mut vc_domi: Arc<SBSet::SBSet>;
    let mut vc_domi_aux: Arc<SBSet::SBSet>;
    let mut iterators: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut pot_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut flow_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut iter_expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    vc_dom = SBPWLinearMap::wholeDom(pw.clone())?;
    vc_im = SBPWLinearMap::image(pw.clone(), vc_dom.clone())?;
    iterators = arrayCreate(Vector::size(vCount.clone()), crate::NFInstNode::InstNode::interned_EMPTY_NODE());
    for mut i in 1..=metamodelica::arrayLength(iterators.clone()) {
        {
            let __cell0 = InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), crate::NFType::interned_INTEGER());
            let __idx0 = i.clone();
            iterators.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
    }
    iter_expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (iterators.clone()).borrow().iter() {
            let __x = Expression::fromCref(ComponentRef::makeIterator(i.clone(), crate::NFType::interned_INTEGER())?, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (pot_vars, flow_vars) = getConnectors(flatModel.clone());
    let __range1 = UnorderedSet::toArray(SBSet::asets(vc_im.clone())).borrow().iter().cloned().collect::<Vec<_>>();
    for mut aset in __range1 {
        aux_s = SBSet::newEmpty();
        aux_s = SBSet::addAtomicSet(aset.clone(), aux_s.clone())?;
        vc_domi = SBPWLinearMap::preImage(pw.clone(), aux_s.clone())?;
        vc_domi_aux = SBSet::complement(vc_domi.clone(), aux_s.clone())?;
        vars = getVars(pot_vars.clone(), aux_s.clone(), graph.clone())?;
        equations = generatePotentialEquations(aset.clone(), vc_domi_aux.clone(), vars.clone(), iterators.clone(), iter_expl.clone(), pot_vars.clone(), graph.clone(), nmvTable.clone(), equations.clone())?;
        equations = generateFlowEquation(aset.clone(), vc_domi.clone(), iterators.clone(), flow_vars.clone(), graph.clone(), nmvTable.clone(), equations.clone())?;
    }
    equations = metamodelica::Dangerous::listReverseInPlace(equations.clone());
    Ok(equations)
}

fn intervalToRange(mut interval: Arc<SBInterval::SBInterval>) -> Result<Arc<Expression::NFExpression>> {
    let mut range: Arc<Expression::NFExpression>;
    let mut lo: i32 = SBInterval::lowerBound(interval.clone());
    let mut hi: i32 = SBInterval::upperBound(interval.clone());
    if lo.clone() == hi.clone() {
        range = Arc::new(Expression::NFExpression::INTEGER { value: lo.clone() });
    } else {
        range = Expression::makeIntegerRange(lo.clone(), SBInterval::stepValue(interval.clone()), hi.clone())?;
    }
    Ok(range)
}

fn generatePotentialEquations(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut dom: Arc<SBSet::SBSet>, mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut iterators: metamodelica::Array<Arc<InstNode::InstNode>>, mut iterExps: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut potVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut graph: SBGraph, mut nmvTable: NameVertexTable, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut sauxi: Arc<SBSet::SBSet>;
    let mut mi: Arc<SBMultiInterval::SBMultiInterval>;
    let mut mi_range: Arc<SBMultiInterval::SBMultiInterval>;
    let mut aux_mi: Arc<SBMultiInterval::SBMultiInterval>;
    let mut inters: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut ranges: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut vars1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut inds: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let __range0 = UnorderedSet::toArray(SBSet::asets(dom.clone())).borrow().iter().cloned().collect::<Vec<_>>();
    for mut auxi in __range0 {
        mi = SBAtomicSet::aset(auxi.clone());
        mi_range = applyOffset(mi.clone(), getOffset(mi.clone(), nmvTable.clone())?)?;
        inters = SBMultiInterval::intervals(mi_range.clone());
        ranges = Array::map(inters.clone(), (std::sync::Arc::new(intervalToRange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        sauxi = SBSet::newEmpty();
        sauxi = SBSet::addAtomicSet(auxi.clone(), sauxi.clone())?;
        vars1 = getVars(potVars.clone(), sauxi.clone(), graph.clone())?;
        mi = SBAtomicSet::aset(aset.clone());
        aux_mi = applyOffset(mi.clone(), getOffset(mi.clone(), nmvTable.clone())?)?;
        (inds, _) = transMulti(mi_range.clone(), aux_mi.clone(), iterators.clone(), false)?;
        eql = generatePotentialEquations2(vars1.clone(), vars.clone(), iterExps.clone(), inds.clone())?;
        equations = generateForLoop(eql.clone(), iterators.clone(), ranges.clone(), equations.clone())?;
    }
    Ok(equations)
}

fn generatePotentialEquations2(mut vars1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut vars2: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut inds1: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inds2: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut l: Arc<Expression::NFExpression>;
    let mut r: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut eq: Arc<Equation::NFEquation>;
    for mut var1 in &*vars1.clone() {
        let mut var1 = var1.clone();
        for mut var2 in &*vars2.clone() {
            let mut var2 = var2.clone();
            if Type::isEqual(ComponentRef::nodeType(var1.clone())?, ComponentRef::nodeType(var2.clone())?)? {
                l = generateConnector(var1.clone(), inds1.clone())?;
                r = generateConnector(var2.clone(), inds2.clone())?;
                ty = Expression::typeOf(l.clone());
                eq = Equation::makeEquality(l.clone(), r.clone(), ty.clone(), DAE::emptyElementSource().clone(), crate::NFInstNode::InstNode::interned_EMPTY_NODE(), Equation::ScalarizeMode::DONT_SCALARIZE.clone());
                equations = metamodelica::cons(eq.clone(), equations.clone());
            }
        }
    }
    equations = metamodelica::Dangerous::listReverseInPlace(equations.clone());
    Ok(equations)
}

fn generateFlowEquation(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut dom: Arc<SBSet::SBSet>, mut iterators: metamodelica::Array<Arc<InstNode::InstNode>>, mut flowVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut graph: SBGraph, mut nmvTable: NameVertexTable, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut mi: Arc<SBMultiInterval::SBMultiInterval>;
    let mut mi_range: Arc<SBMultiInterval::SBMultiInterval>;
    let mut mi_range2: Arc<SBMultiInterval::SBMultiInterval>;
    let mut sauxi: Arc<SBSet::SBSet>;
    let mut inters: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut ranges: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut inds: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut is_sum: bool;
    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut e: Arc<Expression::NFExpression>;
    let mut sum_exp: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut eq: Arc<Equation::NFEquation>;
    mi = SBAtomicSet::aset(aset.clone());
    mi_range = applyOffset(mi.clone(), getOffset(mi.clone(), nmvTable.clone())?)?;
    inters = SBMultiInterval::intervals(mi_range.clone());
    ranges = Array::map(inters.clone(), (std::sync::Arc::new(intervalToRange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    expl = metamodelica::nil();
    let __range0 = UnorderedSet::toArray(SBSet::asets(dom.clone())).borrow().iter().cloned().collect::<Vec<_>>();
    for mut auxi in __range0 {
        mi = SBAtomicSet::aset(auxi.clone());
        mi_range2 = applyOffset(mi.clone(), getOffset(mi.clone(), nmvTable.clone())?)?;
        (inds, is_sum) = transMulti(mi_range.clone(), mi_range2.clone(), iterators.clone(), true)?;
        sauxi = SBSet::newEmpty();
        sauxi = SBSet::addAtomicSet(auxi.clone(), sauxi.clone())?;
        vars = getVars(flowVars.clone(), sauxi.clone(), graph.clone())?;
        for mut var in &*vars.clone() {
            let mut var = var.clone();
            e = generateConnector(var.clone(), inds.clone())?;
            if is_sum.clone() {
                e = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::SUM().clone(), list![e.clone()], Expression::variability(e.clone())?, Purity::PURE.clone(), Type::arrayElementType(Expression::typeOf(e.clone()))) });
            }
            expl = metamodelica::cons(e.clone(), expl.clone());
        }
    }
    if !(expl.clone().is_empty()) {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(expl.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        sum_exp = __pa1.clone();
        expl = __pa2.clone();
        while !(expl.clone().is_empty()) {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(expl.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa3.clone();
            expl = __pa4.clone();
            sum_exp = Arc::new(Expression::NFExpression::BINARY { exp1: e.clone(), operator: Operator::makeAdd(Expression::typeOf(e.clone())), exp2: sum_exp.clone() });
        }
        ty = Expression::typeOf(sum_exp.clone());
        eq = Equation::makeEquality(sum_exp.clone(), Expression::makeZero(ty.clone())?, ty.clone(), DAE::emptyElementSource().clone(), crate::NFInstNode::InstNode::interned_EMPTY_NODE(), Equation::ScalarizeMode::NO_PREFERENCE.clone());
        equations = generateForLoop(list![eq.clone()], iterators.clone(), ranges.clone(), equations.clone())?;
    }
    Ok(equations)
}

fn generateConnector(mut cr: Arc<ComponentRef::NFComponentRef>, mut indices: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    outExp = Expression::fromCref(cr.clone(), false)?;
    if Type::isArray(Expression::typeOf(outExp.clone())) {
        subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut i in (indices.clone()).into_iter().cloned() {
            let __x = Subscript::fromTypedExp(i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        subs = List::firstN(subs.clone(), Type::dimensionCount(Expression::typeOf(outExp.clone())))?;
        outExp = Expression::applySubscripts(subs.clone(), outExp.clone(), false)?;
    }
    Ok(outExp)
}

fn generateForLoop(mut connects: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut iterators: metamodelica::Array<Arc<InstNode::InstNode>>, mut ranges: metamodelica::Array<Arc<Expression::NFExpression>>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = connects.clone();
    for mut i in ({let __s=metamodelica::arrayLength(iterators.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        if Expression::isInteger(({let __elt = ranges.borrow()[(i.clone()-1) as usize].clone(); __elt})) {
            body = Equation::replaceIteratorList(body.clone(), ({let __elt = iterators.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = ranges.borrow()[(i.clone()-1) as usize].clone(); __elt}))?;
        } else {
            body = list![Arc::new(Equation::NFEquation::FOR { iterator: ({let __elt = iterators.borrow()[(i.clone()-1) as usize].clone(); __elt}), range: Some(({let __elt = ranges.borrow()[(i.clone()-1) as usize].clone(); __elt})), body: body.clone(), scope: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), source: DAE::emptyElementSource().clone() })];
        }
    }
    equations = List::append_reverse(body.clone(), equations.clone());
    Ok(equations)
}

fn getConnectors(mut flatModel: Arc<FlatModel::NFFlatModel>) -> (Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>) {
    let mut effVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut flowVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    for mut v in &*flatModel.variables.clone() {
        let mut v = v.clone();
        if Variable::isPotential(v.clone()) {
            effVars = metamodelica::cons(v.clone(), effVars.clone());
        } else if Variable::isFlow(v.clone()) {
            flowVars = metamodelica::cons(v.clone(), flowVars.clone());
        }
    }
    effVars = metamodelica::Dangerous::listReverseInPlace(effVars.clone());
    flowVars = metamodelica::Dangerous::listReverseInPlace(flowVars.clone());
    (effVars, flowVars)
}

fn getOffset(mut mi: Arc<SBMultiInterval::SBMultiInterval>, mut nmvTable: NameVertexTable) -> Result<metamodelica::Array<i32>> {
    let mut res: metamodelica::Array<i32>;
    let mut i: Arc<SBMultiInterval::SBMultiInterval> = Arc::new(<SBMultiInterval::SBMultiInterval as ::std::default::Default>::default());
    let mut aux: Arc<SBMultiInterval::SBMultiInterval>;
    res = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    for mut i in &*UnorderedMap::valueList(nmvTable.clone()) {
        let mut i = i.clone();
        aux = SBMultiInterval::intersection(mi.clone(), i.clone())?;
        if !(SBMultiInterval::isEmpty(aux.clone())) {
            res = SBMultiInterval::minElem(i.clone())?;
        }
    }
    Ok(res)
}

fn applyOffset(mut mi: Arc<SBMultiInterval::SBMultiInterval>, mut off: metamodelica::Array<i32>) -> Result<Arc<SBMultiInterval::SBMultiInterval>> {
    let mut outMI: Arc<SBMultiInterval::SBMultiInterval>;
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut res: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut i: Arc<SBInterval::SBInterval>;
    let mut o: i32;
    if SBMultiInterval::ndim(mi.clone()) != metamodelica::arrayLength(off.clone()) || off.clone().borrow().is_empty() {
        outMI = SBMultiInterval::newEmpty();
    } else {
        ints = SBMultiInterval::intervals(mi.clone());
        res = metamodelica::arrayCreate(metamodelica::arrayLength(ints.clone()), ({let __elt = ints.borrow()[(1-1) as usize].clone(); __elt}));
        for mut j in 1..=metamodelica::arrayLength(ints.clone()) {
            i = ({let __elt = ints.borrow()[(j.clone()-1) as usize].clone(); __elt});
            o = ({let __elt = off.borrow()[(j.clone()-1) as usize].clone(); __elt});
            {
                let __cell0 = SBInterval::new(SBInterval::lowerBound(i.clone()) - o.clone() + 1, SBInterval::stepValue(i.clone()), SBInterval::upperBound(i.clone()) - o.clone() + 1);
                let __idx0 = j.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(res.clone().clone(), __idx0, __cell0); }
            }
        }
        outMI = SBMultiInterval::fromArray(res.clone())?;
    }
    Ok(outMI)
}

fn getVars(mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut sauxi: Arc<SBSet::SBSet>, mut graph: SBGraph) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut res: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut vl: Arc<metamodelica::List<Arc<SetVertex::SetVertex>>>;
    vl = IncidenceList::vertices(graph.clone());
    for mut v in &*vl.clone() {
        let mut v = v.clone();
        if !(SBSet::isEmpty(SBSet::intersection(v.vs.clone(), sauxi.clone())?)) {
            for mut var in &*vars.clone() {
                let mut var = var.clone();
                if ComponentRef::isPrefix(Connector::name(v.name.clone()), var.name.clone())? {
                    res = metamodelica::cons(var.name.clone(), res.clone());
                }
            }
        }
    }
    res = metamodelica::Dangerous::listReverseInPlace(res.clone());
    Ok(res)
}

fn transMulti(mut mi1: Arc<SBMultiInterval::SBMultiInterval>, mut mi2: Arc<SBMultiInterval::SBMultiInterval>, mut iterators: metamodelica::Array<Arc<InstNode::InstNode>>, mut forFlow: bool) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, bool)> {
    let mut outExpl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut flowRange: bool = false;
    let mut ints1: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut ints2: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut i1: Arc<SBInterval::SBInterval>;
    let mut i2: Arc<SBInterval::SBInterval>;
    let mut i1_sz: i32;
    let mut i2_sz: i32;
    let mut m_int: i32;
    let mut x: Arc<Expression::NFExpression>;
    let mut m: Arc<Expression::NFExpression>;
    let mut h: Arc<Expression::NFExpression>;
    let mut e: Arc<Expression::NFExpression>;
    if SBMultiInterval::ndim(mi1.clone()) != SBMultiInterval::ndim(mi2.clone()) {
        return Ok((outExpl.clone(), flowRange.clone()));
    }
    ints1 = SBMultiInterval::intervals(mi1.clone());
    ints2 = SBMultiInterval::intervals(mi2.clone());
    for mut i in 1..=metamodelica::arrayLength(ints1.clone()) {
        i1 = ({let __elt = ints1.borrow()[(i.clone()-1) as usize].clone(); __elt});
        i2 = ({let __elt = ints2.borrow()[(i.clone()-1) as usize].clone(); __elt});
        i1_sz = SBInterval::size(i1.clone());
        i2_sz = SBInterval::size(i2.clone());
        x = Expression::fromCref(ComponentRef::makeIterator(({let __elt = iterators.borrow()[(i.clone()-1) as usize].clone(); __elt}), crate::NFType::interned_INTEGER())?, false)?;
        if i1_sz.clone() == i2_sz.clone() {
            m_int = intDiv(SBInterval::stepValue(i2.clone()), SBInterval::stepValue(i1.clone()));
            m = Arc::new(Expression::NFExpression::INTEGER { value: m_int.clone() });
            h = Arc::new(Expression::NFExpression::INTEGER { value: -(m_int.clone() * SBInterval::lowerBound(i1.clone())) + SBInterval::lowerBound(i2.clone()) });
            e = Arc::new(Expression::NFExpression::BINARY { exp1: Arc::new(Expression::NFExpression::BINARY { exp1: m.clone(), operator: Operator::makeMul(crate::NFType::interned_INTEGER()), exp2: x.clone() }), operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: h.clone() });
            outExpl = metamodelica::cons(e.clone(), outExpl.clone());
        } else if i2_sz.clone() == 1 && !(forFlow.clone()) {
            outExpl = metamodelica::cons(Arc::new(Expression::NFExpression::INTEGER { value: SBInterval::lowerBound(i2.clone()) }), outExpl.clone());
        } else if i1_sz.clone() == 1 && forFlow.clone() {
            e = Expression::makeIntegerRange(SBInterval::lowerBound(i2.clone()), SBInterval::stepValue(i2.clone()), SBInterval::upperBound(i2.clone()))?;
            outExpl = metamodelica::cons(e.clone(), outExpl.clone());
            flowRange = true;
        } else {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFArrayConnections.transMulti")); __mm_s.push_str(&*literal!(" got invalid intervals.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFArrayConnections.mo"))?;
        }
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl.clone());
    Ok((outExpl, flowRange))
}

