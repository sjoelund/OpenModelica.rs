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
use crate::SBSet;
use crate::StringUtil;
use crate::Vector;
use openmodelica_util_datatypes_basic::List;

pub type VertexEq<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT, VertexT) -> Result<bool> + 'static>;

pub type EdgeEq<EdgeT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>;

pub type VertexStr<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<ArcStr> + 'static>;

pub type EdgeStr<EdgeT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(EdgeT) -> Result<ArcStr> + 'static>;

pub type VertexDescriptor = i32;

// types of sets
// V - generic vertex set (F u U)
// F - function/equation vertex set
// U - unknown/variable vertex set
// E - edge set
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum SetType {
    V = 1,
    F = 2,
    U = 3,
    E = 4,
}
impl PartialOrd for SetType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SetType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for SetType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub(crate) fn edge_finder<EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut index: i32, mut e: EdgeT, mut edges: Arc<Vector::Vector<EdgeT>>, mut eqFn: Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>) -> Result<bool> {
    let mut matching: bool = eqFn(e.clone(), Vector::get(edges.clone(), index)?)?;
    Ok(matching)
}

pub mod IncidenceList {
    use super::*;
#[derive(Clone, metamodelica::ReferenceEq)]
    pub struct IncidenceList<VertexT: Clone, EdgeT: Clone> {
        pub vertices: Arc<Vector::Vector<VertexT>>,
        pub edges: Arc<Vector::Vector<EdgeT>>,
        pub graph: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>>,
        pub vertEqFn: VertexEq<VertexT>,
        pub edgeEqFn: EdgeEq<EdgeT>,
        pub vertToString: VertexStr<VertexT>,
        pub edgeToString: EdgeStr<EdgeT>,
    }

    impl<VertexT: Clone + metamodelica::gc::MMTrace, EdgeT: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace for IncidenceList<VertexT, EdgeT> {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.vertices, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.edges, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.graph, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vertEqFn, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.edgeEqFn, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vertToString, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.edgeToString, __mmv)?;
            Ok(())
        }
    }
    impl<VertexT: Clone + 'static + PartialEq, EdgeT: Clone + 'static + PartialEq> PartialEq for IncidenceList<VertexT, EdgeT> {
        fn eq(&self, other: &Self) -> bool {
            self.vertices == other.vertices && self.edges == other.edges && self.graph == other.graph && std::sync::Arc::ptr_eq((&self.vertEqFn), (&other.vertEqFn)) && std::sync::Arc::ptr_eq((&self.edgeEqFn), (&other.edgeEqFn)) && std::sync::Arc::ptr_eq((&self.vertToString), (&other.vertToString)) && std::sync::Arc::ptr_eq((&self.edgeToString), (&other.edgeToString))
        }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq, EdgeT: Clone + 'static + PartialEq + Eq> Eq for IncidenceList<VertexT, EdgeT> {}
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> PartialOrd for IncidenceList<VertexT, EdgeT> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> Ord for IncidenceList<VertexT, EdgeT> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.vertices.cmp(&other.vertices).then_with(|| self.edges.cmp(&other.edges).then_with(|| self.graph.cmp(&other.graph).then_with(|| (std::sync::Arc::as_ptr((&self.vertEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.vertToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertToString)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeToString)) as *const ()))))))))
        }
    }
    impl<VertexT: Clone + 'static + std::fmt::Debug, EdgeT: Clone + 'static + std::fmt::Debug> std::fmt::Debug for IncidenceList<VertexT, EdgeT> {
        fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut __ds = __f.debug_struct("IncidenceList");
            __ds.field("vertices", &self.vertices);
            __ds.field("edges", &self.edges);
            __ds.field("graph", &self.graph);
            __ds.field("vertEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertEqFn))));
            __ds.field("edgeEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeEqFn))));
            __ds.field("vertToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertToString))));
            __ds.field("edgeToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeToString))));
            __ds.finish()
        }
    }

    pub type INCIDENCE_LIST<VertexT, EdgeT> = IncidenceList<VertexT, EdgeT>;

    pub fn new<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut vertexEq: Arc<dyn ::std::ops::Fn(VertexT, VertexT) -> Result<bool> + 'static>, mut edgeEq: Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>, mut vertexStr: Arc<dyn ::std::ops::Fn(VertexT) -> Result<ArcStr> + 'static>, mut edgeStr: Arc<dyn ::std::ops::Fn(EdgeT) -> Result<ArcStr> + 'static>) -> Arc<IncidenceList<VertexT, EdgeT>> {
        pub(crate) type Indices = Arc<metamodelica::List<i32>>;

        let mut il: Arc<IncidenceList<VertexT, EdgeT>>;
        il = Arc::new(IncidenceList { vertices: Vector::new(0), edges: Vector::new(0), graph: Vector::new(0), vertEqFn: vertexEq.clone(), edgeEqFn: edgeEq.clone(), vertToString: vertexStr.clone(), edgeToString: edgeStr.clone() });
        il
    }

    pub(crate) fn getRow<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<Arc<metamodelica::List<i32>>> {
        let mut row: Arc<metamodelica::List<i32>>;
        row = Vector::get(il.graph.clone(), d)?;
        Ok(row)
    }

    pub fn addVertex<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut v: VertexT) -> i32 {
        let mut d: i32;
        Vector::push(il.vertices.clone(), v);
        Vector::push(il.graph.clone(), metamodelica::nil());
        d = Vector::size(il.vertices.clone());
        d
    }

    pub fn findVertex<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut predFn: Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>) -> Result<Option<i32>> {
        pub type PredFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>;

        let mut od: Option<i32>;
        let mut index: i32;
        (_, index) = Vector::find(il.vertices.clone(), predFn.clone())?;
        od = if (index > 0) {Some(index)} else {None};
        Ok(od)
    }

    pub fn getVertex<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<VertexT> {
        let mut v: VertexT;
        v = Vector::get(il.vertices.clone(), d)?;
        Ok(v)
    }

    pub(crate) fn getVerticesFromSet<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut set: Arc<SBSet::SBSet>, mut getSet: Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>) -> Result<Arc<metamodelica::List<VertexT>>> {
        pub type getSetFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>;

        let mut set_vertices: Arc<metamodelica::List<VertexT>> = metamodelica::nil();
        for mut v in &*vertices(il) {
            let mut v = v.clone();
            if !(SBSet::isEmpty(SBSet::intersection(getSet(v.clone())?, set.clone())?)) {
                set_vertices = metamodelica::cons(v.clone(), set_vertices.clone());
            }
        }
        Ok(set_vertices)
    }

    pub fn addEdge<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d1: i32, mut d2: i32, mut e: EdgeT) -> Result<i32> {
        let mut ei: i32;
        let mut eil: Arc<metamodelica::List<i32>>;
        eil = Vector::get(il.graph.clone(), d1)?;
        ei = List::positionOnTrue(eil.clone(), (std::sync::Arc::new({ let __pe_b1 = e.clone(); let __pe_b2 = il.edges.clone(); let __pe_b3 = il.edgeEqFn.clone(); move |__pe_a0| edge_finder(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
        if ei == -1 {
            Vector::push(il.edges.clone(), e);
            ei = Vector::size(il.edges.clone());
            Vector::update(il.graph.clone(), d1, metamodelica::cons(ei, eil))?;
            Vector::update(il.graph.clone(), d2, metamodelica::cons(ei, Vector::get(il.graph.clone(), d2)?))?;
        } else {
            Vector::update(il.edges.clone(), ei, e)?;
        }
        Ok(ei)
    }

    pub(crate) fn getEdge<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<EdgeT> {
        let mut e: EdgeT;
        e = Vector::get(il.edges.clone(), d)?;
        Ok(e)
    }

    pub(crate) fn isEmpty<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> bool {
        let mut empty: bool = Vector::size(il.vertices.clone()) == 0;
        empty
    }

    pub(crate) fn vertexCount<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> i32 {
        let mut count: i32 = Vector::size(il.vertices.clone());
        count
    }

    pub(crate) fn edgeCount<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> i32 {
        let mut count: i32 = Vector::size(il.edges.clone());
        count
    }

    pub fn vertices<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> Arc<metamodelica::List<VertexT>> {
        let mut vl: Arc<metamodelica::List<VertexT>> = Vector::toList(il.vertices.clone());
        vl
    }

    pub fn edges<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> Arc<metamodelica::List<EdgeT>> {
        let mut el: Arc<metamodelica::List<EdgeT>> = Vector::toList(il.edges.clone());
        el
    }

    pub fn toString<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut vertToString: VertexStr<VertexT> = il.vertToString.clone();
        let mut edgeToString: EdgeStr<EdgeT> = il.edgeToString.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Set-Based Graph")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (Vector::toList(il.vertices.clone())).into_iter().cloned() {
            let __x = vertToString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (Vector::toList(il.edges.clone())).into_iter().cloned() {
            let __x = edgeToString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

}

pub mod BipartiteIncidenceList {
    use super::*;
#[derive(Clone, metamodelica::ReferenceEq)]
    pub struct BipartiteIncidenceList<VertexT: Clone, EdgeT: Clone> {
        pub F_vertices: Arc<Vector::Vector<VertexT>>,
        pub U_vertices: Arc<Vector::Vector<VertexT>>,
        pub edges: Arc<Vector::Vector<EdgeT>>,
        pub graph: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>>,
        pub vertEqFn: VertexEq<VertexT>,
        pub edgeEqFn: EdgeEq<EdgeT>,
        pub vertToString: VertexStr<VertexT>,
        pub edgeToString: EdgeStr<EdgeT>,
    }

    impl<VertexT: Clone + metamodelica::gc::MMTrace, EdgeT: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace for BipartiteIncidenceList<VertexT, EdgeT> {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.F_vertices, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.U_vertices, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.edges, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.graph, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vertEqFn, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.edgeEqFn, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vertToString, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.edgeToString, __mmv)?;
            Ok(())
        }
    }
    impl<VertexT: Clone + 'static + PartialEq, EdgeT: Clone + 'static + PartialEq> PartialEq for BipartiteIncidenceList<VertexT, EdgeT> {
        fn eq(&self, other: &Self) -> bool {
            self.F_vertices == other.F_vertices && self.U_vertices == other.U_vertices && self.edges == other.edges && self.graph == other.graph && std::sync::Arc::ptr_eq((&self.vertEqFn), (&other.vertEqFn)) && std::sync::Arc::ptr_eq((&self.edgeEqFn), (&other.edgeEqFn)) && std::sync::Arc::ptr_eq((&self.vertToString), (&other.vertToString)) && std::sync::Arc::ptr_eq((&self.edgeToString), (&other.edgeToString))
        }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq, EdgeT: Clone + 'static + PartialEq + Eq> Eq for BipartiteIncidenceList<VertexT, EdgeT> {}
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> PartialOrd for BipartiteIncidenceList<VertexT, EdgeT> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> Ord for BipartiteIncidenceList<VertexT, EdgeT> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.F_vertices.cmp(&other.F_vertices).then_with(|| self.U_vertices.cmp(&other.U_vertices).then_with(|| self.edges.cmp(&other.edges).then_with(|| self.graph.cmp(&other.graph).then_with(|| (std::sync::Arc::as_ptr((&self.vertEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.vertToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertToString)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeToString)) as *const ())))))))))
        }
    }
    impl<VertexT: Clone + 'static + std::fmt::Debug, EdgeT: Clone + 'static + std::fmt::Debug> std::fmt::Debug for BipartiteIncidenceList<VertexT, EdgeT> {
        fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut __ds = __f.debug_struct("BipartiteIncidenceList");
            __ds.field("F_vertices", &self.F_vertices);
            __ds.field("U_vertices", &self.U_vertices);
            __ds.field("edges", &self.edges);
            __ds.field("graph", &self.graph);
            __ds.field("vertEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertEqFn))));
            __ds.field("edgeEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeEqFn))));
            __ds.field("vertToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertToString))));
            __ds.field("edgeToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeToString))));
            __ds.finish()
        }
    }

    pub type BIPARTITE_INCIDENCE_LIST<VertexT, EdgeT> = BipartiteIncidenceList<VertexT, EdgeT>;

    pub(crate) fn new<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut vertexEq: Arc<dyn ::std::ops::Fn(VertexT, VertexT) -> Result<bool> + 'static>, mut edgeEq: Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>, mut vertexStr: Arc<dyn ::std::ops::Fn(VertexT) -> Result<ArcStr> + 'static>, mut edgeStr: Arc<dyn ::std::ops::Fn(EdgeT) -> Result<ArcStr> + 'static>) -> Arc<BipartiteIncidenceList<VertexT, EdgeT>> {
        pub(crate) type Indices = Arc<metamodelica::List<i32>>;

        let mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>;
        il = Arc::new(BipartiteIncidenceList { F_vertices: Vector::new(0), U_vertices: Vector::new(0), edges: Vector::new(0), graph: Vector::new(0), vertEqFn: vertexEq.clone(), edgeEqFn: edgeEq.clone(), vertToString: vertexStr.clone(), edgeToString: edgeStr.clone() });
        il
    }

    pub(crate) fn getRow<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<Arc<metamodelica::List<i32>>> {
        let mut row: Arc<metamodelica::List<i32>> = Vector::get(il.graph.clone(), d)?;
        Ok(row)
    }

    pub(crate) fn addVertex<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut v: VertexT, mut ST: SetType) -> Result<i32> {
        let mut d: i32;
        d = (match ST {
        SetType::F => {
            Vector::push(il.F_vertices.clone(), v);
            Vector::push(il.graph.clone(), metamodelica::nil());
            Vector::size(il.F_vertices.clone())
        },
        SetType::U => {
            Vector::push(il.U_vertices.clone(), v);
            Vector::push(il.graph.clone(), metamodelica::nil());
            Vector::size(il.U_vertices.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.addVertex")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST)); __mm_s.push_str(&*literal!("\nAllowed: F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Util/SBGraph.mo"))?;
            bail!("fail")
        },
    });
        Ok(d)
    }

    pub(crate) fn findVertex<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut ST: SetType, mut predFn: Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>) -> Result<Option<i32>> {
        pub type PredFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>;

        let mut od: Option<i32>;
        let mut index: i32 = 0;
        index = (match ST {
        SetType::F => {
            (_, index) = Vector::find(il.F_vertices.clone(), predFn.clone())?;
            index
        },
        SetType::U => {
            (_, index) = Vector::find(il.U_vertices.clone(), predFn.clone())?;
            index
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.findVertex")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST)); __mm_s.push_str(&*literal!("\nAllowed: F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Util/SBGraph.mo"))?;
            bail!("fail")
        },
    });
        od = if (index > 0) {Some(index)} else {None};
        Ok(od)
    }

    pub(crate) fn getVertex<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d: i32, mut ST: SetType) -> Result<VertexT> {
        let mut v: VertexT;
        v = (match ST {
        SetType::F => Vector::get(il.F_vertices.clone(), d)?,
        SetType::U => Vector::get(il.U_vertices.clone(), d)?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.getVertex")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST)); __mm_s.push_str(&*literal!("\nAllowed: F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Util/SBGraph.mo"))?;
            bail!("fail")
        },
    });
        Ok(v)
    }

    pub(crate) fn getVerticesFromSet<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut set: Arc<SBSet::SBSet>, mut ST: SetType, mut getSet: Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>) -> Result<Arc<metamodelica::List<VertexT>>> {
        pub type getSetFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>;

        let mut set_vertices: Arc<metamodelica::List<VertexT>> = metamodelica::nil();
        for mut v in &*vertices(il, ST)? {
            let mut v = v.clone();
            if !(SBSet::isEmpty(SBSet::intersection(getSet(v.clone())?, set.clone())?)) {
                set_vertices = metamodelica::cons(v.clone(), set_vertices.clone());
            }
        }
        Ok(set_vertices)
    }

    pub(crate) fn addEdge<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d1: i32, mut d2: i32, mut e: EdgeT) -> Result<i32> {
        let mut ei: i32;
        let mut eil: Arc<metamodelica::List<i32>>;
        eil = getRow(il.clone(), d1)?;
        ei = List::positionOnTrue(eil.clone(), (std::sync::Arc::new({ let __pe_b1 = e.clone(); let __pe_b2 = il.edges.clone(); let __pe_b3 = il.edgeEqFn.clone(); move |__pe_a0| edge_finder(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
        if ei == -1 {
            Vector::push(il.edges.clone(), e);
            ei = Vector::size(il.edges.clone());
            Vector::update(il.graph.clone(), d1, metamodelica::cons(ei, eil))?;
            Vector::update(il.graph.clone(), d2, metamodelica::cons(ei, Vector::get(il.graph.clone(), d2)?))?;
        } else {
            Vector::update(il.edges.clone(), ei, e)?;
        }
        Ok(ei)
    }

    pub(crate) fn getEdgesFromSet<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut set: Arc<SBSet::SBSet>, mut getSet: Arc<dyn ::std::ops::Fn(EdgeT) -> Result<Arc<SBSet::SBSet>> + 'static>) -> Result<Arc<metamodelica::List<EdgeT>>> {
        pub type getSetFn<EdgeT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(EdgeT) -> Result<Arc<SBSet::SBSet>> + 'static>;

        let mut set_edges: Arc<metamodelica::List<EdgeT>> = metamodelica::nil();
        for mut e in &*edges(il) {
            let mut e = e.clone();
            if !(SBSet::isEmpty(SBSet::intersection(getSet(e.clone())?, set.clone())?)) {
                set_edges = metamodelica::cons(e.clone(), set_edges.clone());
            }
        }
        Ok(set_edges)
    }

    pub(crate) fn getEdge<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<EdgeT> {
        let mut e: EdgeT = Vector::get(il.edges.clone(), d)?;
        Ok(e)
    }

    pub(crate) fn isEmpty<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> bool {
        let mut empty: bool = Vector::size(il.F_vertices.clone()) == 0 && Vector::size(il.U_vertices.clone()) == 0;
        empty
    }

    pub(crate) fn vertexCount<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut ST: SetType) -> Result<i32> {
        let mut count: i32;
        count = (match ST {
        SetType::V { .. } => Vector::size(il.F_vertices.clone()) + Vector::size(il.U_vertices.clone()),
        SetType::F => Vector::size(il.F_vertices.clone()),
        SetType::U => Vector::size(il.U_vertices.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.vertexCount")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST)); __mm_s.push_str(&*literal!("\nAllowed: V,F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Util/SBGraph.mo"))?;
            bail!("fail")
        },
    });
        Ok(count)
    }

    pub(crate) fn edgeCount<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> i32 {
        let mut count: i32 = Vector::size(il.edges.clone());
        count
    }

    pub(crate) fn vertices<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut ST: SetType) -> Result<Arc<metamodelica::List<VertexT>>> {
        let mut vl: Arc<metamodelica::List<VertexT>>;
        vl = (match ST {
        SetType::V { .. } => listAppend(Vector::toList(il.F_vertices.clone()), Vector::toList(il.U_vertices.clone())),
        SetType::F => Vector::toList(il.F_vertices.clone()),
        SetType::U => Vector::toList(il.U_vertices.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.vertices")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST)); __mm_s.push_str(&*literal!("\nAllowed: V,F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Util/SBGraph.mo"))?;
            bail!("fail")
        },
    });
        Ok(vl)
    }

    pub(crate) fn edges<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> Arc<metamodelica::List<EdgeT>> {
        let mut el: Arc<metamodelica::List<EdgeT>> = Vector::toList(il.edges.clone());
        el
    }

    pub(crate) fn toString<VertexT: Clone + 'static + metamodelica::gc::MMTrace, EdgeT: Clone + 'static + metamodelica::gc::MMTrace>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut vertToString: VertexStr<VertexT>;
        let mut edgeToString: EdgeStr<EdgeT>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(il.clone()) {
            Deref @ BipartiteIncidenceList { vertToString: __pa0, edgeToString: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vertToString = __pa0.clone();
        edgeToString = __pa1.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Set-Based Graph")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("F-Vertices")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (Vector::toList(il.F_vertices.clone())).into_iter().cloned() {
            let __x = vertToString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("U-Vertices")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (Vector::toList(il.U_vertices.clone())).into_iter().cloned() {
            let __x = vertToString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("Edges")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (Vector::toList(il.edges.clone())).into_iter().cloned() {
            let __x = edgeToString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub(crate) fn setTypeString(mut ST: SetType) -> ArcStr {
        let mut r#str: ArcStr;
        r#str = ((match ST {
        SetType::V { .. } => literal!("V (generic vertex set)"),
        SetType::F => literal!("F (function vertex set)"),
        SetType::U => literal!("U (unknown vertex set)"),
        SetType::E => literal!("E (edge set)"),
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.setTypeString")); __mm_s.push_str(&*literal!(" ERROR")); ArcStr::from(__mm_s) },
    })).clone();
        r#str
    }

}

