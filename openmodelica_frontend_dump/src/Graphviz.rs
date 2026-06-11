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

pub type Type = ArcStr;

pub type Ident = ArcStr;

pub type Label = ArcStr;

/// an Attribute is a pair of name an value.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Attribute {
    /// name
    pub name: ArcStr,
    /// value
    pub value: ArcStr,
}

impl metamodelica::gc::MMTrace for Attribute {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
impl Default for Attribute {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
        }
    }
}

pub type ATTR = Attribute;


pub type Attributes = Arc<metamodelica::List<Attribute>>;

/// A graphviz Node is a node of the graph.
///    It has a type and attributes and children.
///    It can also have a list of labels, provided by the LNODE
///    constructor.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Node {
    NODE {
        type_: Type,
        attributes: Attributes,
        children: Arc<metamodelica::List<Arc<Node>>>,
    },
    LNODE {
        type_: Type,
        labelLst: Arc<metamodelica::List<ArcStr>>,
        attributes: Attributes,
        children: Arc<metamodelica::List<Arc<Node>>>,
    },
}
impl metamodelica::gc::MMTrace for Node {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Node::NODE { type_, attributes, children } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(children, __mmv)?;
                Ok(())
            }
            Node::LNODE { type_, labelLst, attributes, children } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(labelLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(children, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Node {
    fn default() -> Self {
        Self::NODE {
            type_: Default::default(),
            attributes: Default::default(),
            children: Default::default(),
        }
    }
}
pub use self::Node::{NODE,LNODE};

pub type Children = Arc<metamodelica::List<Arc<Node>>>;

pub static r#box: Attribute = Attribute { name: literal!("shape"), value: literal!("box") };

pub fn dump(mut node: Arc<Node>) -> Result<()> {
    let mut nm: Label;
    metamodelica::print((literal!("graph AST {\n")).clone());
    nm = (dumpNode(node)?).clone();
    metamodelica::print((literal!("}\n")).clone());
    Ok(())
}

fn dumpNode(mut inNode: Arc<Node>) -> Result<Ident> {
    let mut outIdent: Ident;
    outIdent = ((::match_deref::match_deref! { match &(inNode) {
        Deref @ Node::NODE { type_: typ, attributes: attr, children } => {
            let mut nm: Label;
            let mut typlbl: Label;
            let mut out: Label;
            let mut newattr: Attributes;
            nm = (nodename((typ.clone()).clone())).clone();
            typlbl = (makeLabel(list![(typ.clone()).clone()])?).clone();
            newattr = metamodelica::cons(Attribute { name: (literal!("label")).clone(), value: (typlbl.clone()).clone() }, attr.clone());
            out = (makeNode((nm.clone()).clone(), newattr.clone())?).clone();
            metamodelica::print((out.clone()).clone());
            dumpChildren((nm.clone()).clone(), children.clone())?;
            nm.clone()
        },
        Deref @ Node::LNODE { type_: typ, labelLst: lbl, attributes: attr, children } => {
            let mut nm: Label;
            let mut out: Label;
            let mut lblstr: Label;
            let mut newattr: Attributes;
            let mut lbl_1: Arc<metamodelica::List<ArcStr>>;
            nm = (nodename((typ.clone()).clone())).clone();
            lbl_1 = metamodelica::cons((typ.clone()).clone(), lbl.clone());
            lblstr = (makeLabel(lbl_1.clone())?).clone();
            newattr = metamodelica::cons(Attribute { name: (literal!("label")).clone(), value: (lblstr.clone()).clone() }, attr.clone());
            out = (makeNode((nm.clone()).clone(), newattr.clone())?).clone();
            metamodelica::print((out.clone()).clone());
            dumpChildren((nm.clone()).clone(), children.clone())?;
            nm.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outIdent)
}

fn makeLabel(mut sl: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut s2: ArcStr;
    let mut s0: Label;
    let mut s1: Label;
    s0 = (makeLabelReq(sl, (literal!("")).clone())?).clone();
    s1 = (stringAppend((literal!("\"")).clone(), (s0).clone())).clone();
    s2 = (stringAppend((s1).clone(), (literal!("\"")).clone())).clone();
    Ok(s2)
}

fn makeLabelReq(mut inStringLst: Arc<metamodelica::List<ArcStr>>, mut inString: ArcStr) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inStringLst) {
        Deref @ metamodelica::List::Cons { head: s, tail: Deref @ metamodelica::List::Nil } => {
            return Ok(stringAppend((inString).clone(), (s.clone()).clone()))
        },
        Deref @ metamodelica::List::Cons { head: s1, tail: Deref @ metamodelica::List::Cons { head: s2, tail: Deref @ metamodelica::List::Nil } } => {
            let mut s: Label;
            s = (stringAppend((inString).clone(), (s1.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("\\n")).clone())).clone();
            return Ok(stringAppend((s.clone()).clone(), (s2.clone()).clone()))
        },
        Deref @ metamodelica::List::Cons { head: s1, tail: rest } => {
            let mut s: Label;
            s = (stringAppend((inString).clone(), (s1.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("\\n")).clone())).clone();
            { (inStringLst, inString) = (rest.clone(), (s.clone()).clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn dumpChildren(mut inIdent: Ident, mut inChildren: Children) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inIdent, inChildren)) {
        (_, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (parent, Deref @ metamodelica::List::Cons { head: node, tail: rest }) => {
            let mut nm: Label;
            nm = (dumpNode(node.clone())?).clone();
            printEdge((nm.clone()).clone(), (parent.clone()).clone());
            dumpChildren((parent.clone()).clone(), rest.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn nodename(mut r#str: ArcStr) -> ArcStr {
    let mut s: ArcStr;
    let mut i: i32;
    let mut is: Label;
    i = tick();
    is = (intString(i)).clone();
    s = (stringAppend((literal!("GVNOD")).clone(), (is).clone())).clone();
    s
}

fn printEdge(mut n1: Ident, mut n2: Ident) -> () {
    let mut r#str: Label;
    r#str = (makeEdge((n1).clone(), (n2).clone())).clone();
    metamodelica::print((r#str).clone());
    metamodelica::print((literal!(";\n")).clone());
    ()
}

fn makeEdge(mut n1: Ident, mut n2: Ident) -> ArcStr {
    let mut r#str: ArcStr;
    let mut s: Label;
    s = (stringAppend((n1).clone(), (literal!(" -- ")).clone())).clone();
    r#str = (stringAppend((s).clone(), (n2).clone())).clone();
    r#str
}

fn makeNode(mut nm: Ident, mut attr: Attributes) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: Label;
    let mut s_1: Label;
    s = (makeAttr(attr)?).clone();
    s_1 = (stringAppend((nm).clone(), (s).clone())).clone();
    r#str = (stringAppend((s_1).clone(), (literal!(";")).clone())).clone();
    Ok(r#str)
}

fn makeAttr(mut l: Arc<metamodelica::List<Attribute>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut res: Label;
    let mut s: Label;
    res = (makeAttrReq(l, (literal!("")).clone())?).clone();
    s = (stringAppend((literal!("[")).clone(), (res).clone())).clone();
    r#str = (stringAppend((s).clone(), (literal!("]")).clone())).clone();
    Ok(r#str)
}

fn makeAttrReq(mut inAttributeLst: Arc<metamodelica::List<Attribute>>, mut inString: ArcStr) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAttributeLst) {
        Deref @ metamodelica::List::Cons { head: Attribute { name, value: v }, tail: Deref @ metamodelica::List::Nil } => {
            let mut s: Label;
            s = (stringAppend((inString).clone(), (name.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("=")).clone())).clone();
            return Ok(stringAppend((s.clone()).clone(), (v.clone()).clone()))
        },
        Deref @ metamodelica::List::Cons { head: Attribute { name, value: v }, tail: rest } => {
            let mut s: Label;
            s = (stringAppend((inString).clone(), (name.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("=")).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (v.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!(",")).clone())).clone();
            { (inAttributeLst, inString) = (rest.clone(), (s.clone()).clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

