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
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore::RefTree;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub type Name = ArcStr;

pub type Names = Arc<metamodelica::List<ArcStr>>;

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

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub(crate) const extendsPrefix: &'static str = "$ext_";

pub(crate) const topNodeName: &'static str = "$top";

// these names are used mostly for edges in the graph
// the edges are saved inside the AvlTree ("name", Ref)
pub(crate) const tyNodeName: &'static str = "$ty";

pub(crate) const ftNodeName: &'static str = "$ft";

pub(crate) const refNodeName: &'static str = "$ref";

pub(crate) const modNodeName: &'static str = "$mod";

pub(crate) const bndNodeName: &'static str = "$bnd";

pub(crate) const cndNodeName: &'static str = "$cnd";

pub(crate) const dimsNodeName: &'static str = "$dims";

pub(crate) const tydimsNodeName: &'static str = "$tydims";

pub(crate) const subsNodeName: &'static str = "$subs";

pub(crate) const ccNodeName: &'static str = "$cc";

pub(crate) const eqNodeName: &'static str = "$eq";

pub(crate) const ieqNodeName: &'static str = "$ieq";

pub(crate) const alNodeName: &'static str = "$al";

pub(crate) const ialNodeName: &'static str = "$ial";

pub(crate) const optNodeName: &'static str = "$opt";

pub(crate) const edNodeName: &'static str = "$ed";

pub(crate) const forNodeName: &'static str = "$for";

pub(crate) const matchNodeName: &'static str = "$match";

pub(crate) const cloneNodeName: &'static str = "$clone";

pub(crate) const origNodeName: &'static str = "$original";

pub(crate) const feNodeName: &'static str = "$functionEvaluation";

pub(crate) const duNodeName: &'static str = "$definedUnits";

pub(crate) const veNodeName: &'static str = "$ve";

pub(crate) const imNodeName: &'static str = "$imp";

pub(crate) const itNodeName: &'static str = "$it";

pub(crate) const assertNodeName: &'static str = "$assert";

pub(crate) const statusNodeName: &'static str = "$status";

pub(crate) fn toRef(mut inNode: Node) -> Ref {
    let mut outRef: Ref;
    outRef = arrayCreate(1, inNode);
    outRef
}

pub fn fromRef(mut inRef: Ref) -> Result<Node> {
    let mut outNode: Node;
    outNode = metamodelica::arrayGet(inRef.clone(), 1)?;
    Ok(outNode)
}

pub(crate) fn updateRef(mut inRef: Ref, mut inNode: Node) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = metamodelica::arrayUpdate(inRef.clone(), 1, inNode)?;
    Ok(outRef)
}

pub fn id(mut inNode: Node) -> Result<Id> {
    let mut id: Id;
    let FCore::N { id: __pa0, .. } = (inNode) else { bail!("pattern mismatch") };
    id = __pa0.clone();
    Ok(id)
}

pub(crate) fn parents(mut inNode: Node) -> Result<Parents> {
    let mut p: Parents;
    let FCore::N { parents: __pa0, .. } = (inNode) else { bail!("pattern mismatch") };
    p = __pa0.clone();
    Ok(p)
}

pub(crate) fn hasParents(mut inNode: Node) -> Result<bool> {
    let mut b: bool;
    b = !(parents(inNode)?.is_empty());
    Ok(b)
}

pub(crate) fn refParents(mut inRef: Ref) -> Result<Parents> {
    let mut p: Parents;
    let FCore::N { parents: __pa0, .. } = (fromRef(inRef.clone())?) else { bail!("pattern mismatch") };
    p = __pa0.clone();
    Ok(p)
}

pub(crate) fn refPushParents(mut inRef: Ref, mut inParents: Parents) -> Result<Ref> {
    let mut outRef: Ref;
    let mut n: Name;
    let mut i: Id;
    let mut p: Parents;
    let mut c: Children;
    let mut d: Data;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: __pa4 } = (fromRef(inRef.clone())?) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    i = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    d = __pa4.clone();
    p = listAppend(inParents, p);
    outRef = updateRef(inRef.clone(), FCore::Node { name: (n).clone(), id: i, parents: p, children: c, data: d })?;
    Ok(outRef)
}

pub(crate) fn setParents(mut inNode: Node, mut inParents: Parents) -> Result<Node> {
    let mut outNode: Node;
    let mut n: Name;
    let mut i: Id;
    let mut p: Parents;
    let mut c: Children;
    let mut d: Data;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: __pa4 } = (inNode) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    i = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    d = __pa4.clone();
    outNode = FCore::Node { name: (n).clone(), id: i, parents: inParents, children: c, data: d };
    Ok(outNode)
}

pub(crate) fn target(mut inNode: Node) -> Result<Ref> {
    let mut outRef: Ref;
    let __pa0 = ::match_deref::match_deref! { match &(targetScope(inNode)?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRef = __pa0.clone();
    Ok(outRef)
}

pub(crate) fn targetScope(mut inNode: Node) -> Result<Scope> {
    let mut outScope: Scope = metamodelica::nil();
    outScope = (match inNode {
        FCore::Node { data: FCore::Data::REF { target: ref __esc_outScope }, .. } => {
            outScope = __esc_outScope.clone();
            outScope.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outScope)
}

pub(crate) fn new(mut inName: Name, mut inId: Id, mut inParents: Parents, mut inData: Data) -> Node {
    let mut node: Node;
    node = FCore::Node { name: (inName).clone(), id: inId, parents: inParents, children: FCore::RefTree::new(), data: inData };
    node
}

pub(crate) fn addImport(mut inImport: Arc<SCode::Element>, mut inImportTable: ImportTable) -> Result<ImportTable> {
    let mut outImportTable: ImportTable;
    outImportTable = (::match_deref::match_deref! { match &((inImport, inImportTable)) {
        (Deref @ SCode::Element::IMPORT { imp: imp @ Absyn::Import::UNQUAL_IMPORT { .. }, .. }, FCore::ImportTable { hidden, qualifiedImports: qual_imps, unqualifiedImports: unqual_imps }) => {
            let mut unqual_imps = (*unqual_imps).clone();
            unqual_imps = List::unionElt(imp.clone(), unqual_imps.clone());
            FCore::ImportTable { hidden: hidden.clone(), qualifiedImports: qual_imps.clone(), unqualifiedImports: unqual_imps.clone() }
        },
        (Deref @ SCode::Element::IMPORT { imp, info, .. }, FCore::ImportTable { hidden, qualifiedImports: qual_imps, unqualifiedImports: unqual_imps }) => {
            let mut imp = (*imp).clone();
            let mut qual_imps = (*qual_imps).clone();
            imp = translateQualifiedImportToNamed(imp.clone())?;
            checkUniqueQualifiedImport(imp.clone(), qual_imps.clone(), info.clone())?;
            qual_imps = List::unionElt(imp.clone(), qual_imps.clone());
            FCore::ImportTable { hidden: hidden.clone(), qualifiedImports: qual_imps.clone(), unqualifiedImports: unqual_imps.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outImportTable)
}

fn translateQualifiedImportToNamed(mut inImport: Import) -> Result<Import> {
    let mut outImport: Import;
    outImport = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            inImport
        },
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut name: Name;
            name = (AbsynUtil::pathLastIdent(path.clone())?).clone();
            Absyn::Import::NAMED_IMPORT { name: (name.clone()).clone(), path: path.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outImport)
}

fn checkUniqueQualifiedImport(mut inImport: Import, mut inImports: Arc<metamodelica::List<Absyn::Import>>, mut inInfo: SourceInfo) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inImport.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (List::isMemberOnTrue(inImport.clone(), inImports.clone(), (std::sync::Arc::new(fnptr!(compareQualifiedImportNames, Absyn::Import, Absyn::Import)) as std::sync::Arc<dyn ::std::ops::Fn(Absyn::Import, Absyn::Import) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let Absyn::Import::NAMED_IMPORT { name: mut name, .. } = __mc_input.clone() else { bail!("nomatch") };
            Error::addSourceMessage(Error::MULTIPLE_QUALIFIED_IMPORTS_WITH_SAME_NAME.clone(), list![(name.clone()).clone()], inInfo.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn compareQualifiedImportNames(mut inImport1: Import, mut inImport2: Import) -> bool {
    let mut outEqual: bool;
    outEqual = (match (inImport1, inImport2) {
        (Absyn::Import::NAMED_IMPORT { name: mut name1, .. }, Absyn::Import::NAMED_IMPORT { name: mut name2, .. }) if (stringEqual((name1.clone()).clone(), (name2.clone()).clone())) => {
            true
        },
        _ => {
            false
        },
    });
    outEqual
}

pub(crate) fn addChildRef(mut inParentRef: Ref, mut inName: Name, mut inChildRef: Ref, mut checkDuplicate: bool) -> Result<()> {
    let mut n: Name;
    let mut i: i32;
    let mut p: Parents;
    let mut c: Children;
    let mut d: Data;
    let mut parent: Ref;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: __pa4 } = (fromRef(inParentRef.clone())?) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    i = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    d = __pa4.clone();
    c = FCore::RefTree::add(c, (inName).clone(), inChildRef.clone(), (if (checkDuplicate) { ((std::sync::Arc::new(printElementConflictError) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>, ArcStr) -> Result<metamodelica::Array<FCore::Node>> + 'static>) as _) } else { ((std::sync::Arc::new(fnptr!(FCore::RefTree::addConflictReplace, metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>, ArcStr) -> Result<metamodelica::Array<FCore::Node>> + 'static>) as _) }))?;
    parent = updateRef(inParentRef.clone(), FCore::Node { name: (n).clone(), id: i, parents: p, children: c, data: d })?;
    Ok(())
}

fn printElementConflictError(mut newRef: Ref, mut oldRef: Ref, mut name: ArcStr) -> Result<Ref> {
    let mut dummy: Ref;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    if Config::acceptMetaModelicaGrammar()? {
        dummy = newRef.clone();
    } else {
        info1 = SCodeUtil::elementInfo(getElementFromRef(newRef.clone())?);
        info2 = SCodeUtil::elementInfo(getElementFromRef(oldRef.clone())?);
        Error::addMultiSourceMessage(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone(), list![(name).clone()], list![info2, info1])?;
        bail!("fail");
    }
    Ok(dummy)
}

pub(crate) fn addImportToRef(mut r#ref: Ref, mut imp: Arc<SCode::Element>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: Parents;
    let mut c: Children;
    let mut it: ImportTable;
    let mut r: Ref;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::IM { i: __pa4 } } = (fromRef(r#ref.clone())?) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    id = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    it = __pa4.clone();
    it = addImport(imp, it)?;
    r = updateRef(r#ref.clone(), FCore::Node { name: (n).clone(), id: id, parents: p, children: c, data: FCore::Data::IM { i: it } })?;
    Ok(())
}

pub(crate) fn addTypesToRef(mut r#ref: Ref, mut inTys: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: Parents;
    let mut c: Children;
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>;
    let mut r: Ref;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::FT { tys: __pa4 } } = (fromRef(r#ref.clone())?) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    id = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    tys = __pa4.clone();
    tys = List::unique(listAppend(inTys, tys));
    r = updateRef(r#ref.clone(), FCore::Node { name: (n).clone(), id: id, parents: p, children: c, data: FCore::Data::FT { tys: tys } })?;
    Ok(())
}

pub(crate) fn addIteratorsToRef(mut r#ref: Ref, mut inIterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: Parents;
    let mut c: Children;
    let mut it: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
    let mut r: Ref;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::FS { fis: __pa4 } } = (fromRef(r#ref.clone())?) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    id = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    it = __pa4.clone();
    r = updateRef(r#ref.clone(), FCore::Node { name: (n).clone(), id: id, parents: p, children: c, data: FCore::Data::FS { fis: listAppend(it, inIterators) } })?;
    Ok(())
}

pub(crate) fn addDefinedUnitToRef(mut r#ref: Ref, mut du: Arc<SCode::Element>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: Parents;
    let mut c: Children;
    let mut r: Ref;
    let mut dus: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: FCore::DU { els: __pa4 } } = (fromRef(r#ref.clone())?) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    id = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    dus = __pa4.clone();
    r = updateRef(r#ref.clone(), FCore::Node { name: (n).clone(), id: id, parents: p, children: c, data: FCore::Data::DU { els: metamodelica::cons(du, dus) } })?;
    Ok(())
}

pub fn name(mut n: Node) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = ((match n {
        FCore::Node { name: mut s, .. } => {
            s.clone()
        },
    })).clone();
    Ok(name)
}

pub(crate) fn refName(mut r: Ref) -> Result<ArcStr> {
    let mut n: ArcStr;
    n = (name(fromRef(r.clone())?)?).clone();
    Ok(n)
}

pub(crate) fn data(mut n: Node) -> Result<Data> {
    let mut d: Data = FCore::Data::TOP;
    d = (match n {
        FCore::Node { data: mut __esc_d, .. } => {
            d = __esc_d.clone();
            d.clone()
        },
    });
    Ok(d)
}

pub(crate) fn refData(mut r: Ref) -> Result<Data> {
    let mut outData: Data;
    outData = data(fromRef(r.clone())?)?;
    Ok(outData)
}

pub(crate) fn top(mut inRef: Ref) -> Result<Ref> {
    let mut outTop: Ref;
    outTop = inRef.clone();
    while hasParents(fromRef(outTop.clone())?)? {
        outTop = original(parents(fromRef(outTop.clone())?)?)?;
    }
    Ok(outTop)
}

pub fn children(mut inNode: Node) -> Result<Children> {
    let mut outChildren: Children;
    let FCore::N { children: __pa0, .. } = (inNode) else { bail!("pattern mismatch") };
    outChildren = __pa0.clone();
    Ok(outChildren)
}

pub(crate) fn hasChild(mut inNode: Node, mut inName: Name) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = inName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            childFromNode(inNode.clone(), (inName.clone()).clone())?;
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    b
}

pub(crate) fn refHasChild(mut inRef: Ref, mut inName: Name) -> Result<bool> {
    let mut b: bool;
    b = hasChild(fromRef(inRef.clone())?, (inName).clone());
    Ok(b)
}

pub(crate) fn setChildren(mut inNode: Node, mut inChildren: Children) -> Result<Node> {
    let mut outNode: Node;
    let mut n: Name;
    let mut i: Id;
    let mut p: Parents;
    let mut c: Children;
    let mut d: Data;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: __pa4 } = (inNode) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    i = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    d = __pa4.clone();
    outNode = FCore::Node { name: (n).clone(), id: i, parents: p, children: inChildren, data: d };
    Ok(outNode)
}

pub(crate) fn setData(mut inNode: Node, mut inData: Data) -> Result<Node> {
    let mut outNode: Node;
    let mut n: Name;
    let mut i: Id;
    let mut p: Parents;
    let mut c: Children;
    let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: _ } = (inNode) else { bail!("pattern mismatch") };
    n = __pa0.clone();
    i = __pa1.clone();
    p = __pa2.clone();
    c = __pa3.clone();
    outNode = FCore::Node { name: (n).clone(), id: i, parents: p, children: c, data: inData };
    Ok(outNode)
}

pub(crate) fn child(mut inParentRef: Ref, mut inName: Name) -> Result<Ref> {
    let mut outChildRef: Ref;
    outChildRef = childFromNode(fromRef(inParentRef.clone())?, (inName).clone())?;
    Ok(outChildRef)
}

pub(crate) fn childFromNode(mut inNode: Node, mut inName: Name) -> Result<Ref> {
    let mut outChildRef: Ref;
    let mut c: Children;
    c = children(inNode)?;
    outChildRef = FCore::RefTree::get(c, (inName).clone())?;
    Ok(outChildRef)
}

pub(crate) fn element2Data(mut inElement: Arc<SCode::Element>, mut inKind: Kind) -> Result<(Data, Arc<DAE::Var>)> {
    let mut outData: Data;
    let mut outVar: Arc<DAE::Var>;
    (outData, outVar) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name: n, prefixes: Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: _, finalPrefix: _, innerOuter: io, replaceablePrefix: _ }, attributes: SCode::Attributes { arrayDims: _, connectorType: ct, parallelism: prl, variability: var, direction: dir, .. }, typeSpec: _, modifications: _, comment: _, condition: _, info: _ } => {
            let mut nd: Data;
            let mut i: Arc<DAE::Var>;
            nd = FCore::Data::CO { e: inElement, r#mod: openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), kind: inKind, status: openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED };
            i = Arc::new(DAE::Var { name: (n.clone()).clone(), attributes: Arc::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
            (nd.clone(), i.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outData, outVar))
}

pub(crate) fn dataStr(mut inData: Data) -> ArcStr {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(inData) {
        FCore::Data::TOP { .. } => {
            literal!("TOP")
        },
        FCore::Data::IT { i: _ } => {
            literal!("I")
        },
        FCore::Data::CL { e: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, .. } => {
            literal!("CE")
        },
        FCore::Data::CL { .. } => {
            literal!("C")
        },
        FCore::Data::CO { .. } => {
            literal!("c")
        },
        FCore::Data::EX { .. } => {
            literal!("E")
        },
        FCore::Data::DU { els: _ } => {
            literal!("U")
        },
        FCore::Data::FT { tys: _ } => {
            literal!("FT")
        },
        FCore::Data::AL { name: _, a: _ } => {
            literal!("ALG")
        },
        FCore::Data::EQ { name: _, e: _ } => {
            literal!("EQ")
        },
        FCore::Data::OT { constrainLst: _, clsAttrs: _ } => {
            literal!("OPT")
        },
        FCore::Data::ED { ed: _ } => {
            literal!("ED")
        },
        FCore::Data::FS { fis: _ } => {
            literal!("FS")
        },
        FCore::Data::FI { fi: _ } => {
            literal!("FI")
        },
        FCore::Data::MS { e: _ } => {
            literal!("MS")
        },
        FCore::Data::MO { m: _ } => {
            literal!("M")
        },
        FCore::Data::EXP { name: n, .. } => {
            n.clone()
        },
        FCore::Data::DIMS { name: n, .. } => {
            n.clone()
        },
        FCore::Data::CR { r: _ } => {
            literal!("r")
        },
        FCore::Data::CC { cc: _ } => {
            literal!("CC")
        },
        FCore::Data::ND { scopeType: _ } => {
            literal!("ND")
        },
        FCore::Data::REF { target: _ } => {
            literal!("REF")
        },
        FCore::Data::VR { .. } => {
            literal!("VR")
        },
        FCore::Data::IM { i: _ } => {
            literal!("IM")
        },
        FCore::Data::ASSERT { message: m } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("assert(")); __mm_s.push_str(&*m.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("UKNOWN NODE DATA")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outStr
}

pub(crate) fn toStr(mut inNode: Node) -> ArcStr {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ('mc: {
        let __mc_input = inNode.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let FCore::Node { name: _, id: mut i, parents: ref p, children: _, data: mut d } = __mc_input.clone() else { bail!("nomatch") };
            let mut outStr: ArcStr = outStr.clone();
            outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[i:")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*literal!("[p:")); __mm_s.push_str(&*stringDelimitList(List::map(List::map(List::map(p.clone(), (std::sync::Arc::new(fromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<FCore::Node> + 'static>))?, (std::sync::Arc::new(id) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Node) -> Result<i32> + 'static>))?, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*literal!("[n:")); __mm_s.push_str(&*name(inNode.clone())?); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*literal!("[d:")); __mm_s.push_str(&*dataStr(d.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            Ok((outStr.clone(), outStr.clone()))
        })() { outStr = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!("Unhandled node!"))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    outStr
}

pub(crate) fn toPathStr(mut inNode: Node) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ('mc: {
        let __mc_input = inNode.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: Deref @ metamodelica::List::Nil, children: _, data: _ } => {
                    let mut outStr: ArcStr = outStr.clone();
                    outStr = (name(inNode.clone())?).clone();
                    Ok((outStr.clone(), outStr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outStr = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: p, children: _, data: _ } => {
                    let mut nr: Ref;
                    let mut s: ArcStr;
                    let mut outStr: ArcStr = outStr.clone();
                    nr = contextual(p.clone())?;
                    let true = (hasParents(fromRef(nr.clone())?)?) else { bail!("pattern mismatch") };
                    s = (toPathStr(fromRef(nr.clone())?)?).clone();
                    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name(inNode.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((outStr.clone(), outStr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outStr = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                FCore::Node { name: _, id: _, parents: p, children: _, data: _ } => {
                    let mut nr: Ref;
                    let mut outStr: ArcStr = outStr.clone();
                    nr = contextual(p.clone())?;
                    let false = (hasParents(fromRef(nr.clone())?)?) else { bail!("pattern mismatch") };
                    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name(inNode.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok((outStr.clone(), outStr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outStr = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outStr)
}

pub(crate) fn scopeStr(mut sc: Scope) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(sc.reverse(), (std::sync::Arc::new(refName) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<ArcStr> + 'static>))?, (literal!("/")).clone());
    Ok(s)
}

pub(crate) fn isImplicitScope(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::TOP { .. }, .. } => false,
        FCore::Node { data: FCore::Data::CL { .. }, .. } => false,
        FCore::Node { data: FCore::Data::CO { .. }, .. } => false,
        FCore::Node { data: FCore::Data::CC { .. }, .. } => false,
        FCore::Node { data: FCore::Data::FS { .. }, .. } => false,
        FCore::Node { data: FCore::Data::MS { .. }, .. } => false,
        FCore::Node { data: FCore::Data::VR { .. }, .. } => false,
        _ => true,
    });
    b
}

pub(crate) fn isRefImplicitScope(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isImplicitScope(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isEncapsulated(mut inNode: Node) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inNode) {
        FCore::Node { data: FCore::Data::CL { e: Deref @ SCode::Element::CLASS { encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, .. }, .. }, .. } => true,
        FCore::Node { data: FCore::Data::CO { .. }, .. } if (boolEq(Config::acceptMetaModelicaGrammar()?, false) && boolNot(Flags::isSet(Flags::GRAPH_INST.clone())?)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isReference(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::REF { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isUserDefined(mut inNode: Node) -> Result<bool> {
    let mut b: bool = false;
    b = (match inNode.clone() {
        FCore::Node { data: FCore::Data::CL { kind: FCore::Kind::USERDEFINED { .. }, .. }, .. } => {
            true
        },
        FCore::Node { data: FCore::Data::CO { kind: FCore::Kind::USERDEFINED { .. }, .. }, .. } => {
            true
        },
        _ if (hasParents(inNode.clone())?) => {
            let mut p: Ref;
            let __pa0 = ::match_deref::match_deref! { match &(parents(inNode.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            b = isRefUserDefined(p.clone())?;
            b
        },
        _ => {
            false
        },
    });
    Ok(b)
}

pub(crate) fn isTop(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::TOP { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isExtends(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::EX { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isDerived(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { e: mut e, .. }, .. } => {
            SCodeUtil::isDerivedClass(e.clone())
        },
        _ => {
            false
        },
    });
    b
}

pub(crate) fn isClass(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isInstance(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: _ }, .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isRedeclare(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inNode) {
        FCore::Node { data: FCore::Data::CL { e: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE { .. }, .. }, .. }, .. }, .. } => true,
        FCore::Node { data: FCore::Data::CO { e: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE { .. }, .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClassExtends(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inNode) {
        FCore::Node { data: FCore::Data::CL { e: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isComponent(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CO { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isConstrainClass(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CC { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isCref(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CR { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isBasicType(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { kind: FCore::Kind::BASIC_TYPE { .. }, .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isBuiltin(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { kind: FCore::Kind::BUILTIN { .. }, .. }, .. } => true,
        FCore::Node { data: FCore::Data::CO { kind: FCore::Kind::BUILTIN { .. }, .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isFunction(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { e: mut e, .. }, .. } if (SCodeUtil::isFunction(e.clone()) || SCodeUtil::isOperator(e.clone())) => {
            true
        },
        _ => {
            false
        },
    });
    b
}

pub(crate) fn isRecord(mut inNode: Node) -> bool {
    let mut b: bool = false;
    b = (match inNode {
        FCore::Node { data: FCore::Data::CL { e: mut e, .. }, .. } if (SCodeUtil::isRecord(e.clone())) => {
            true
        },
        _ => {
            false
        },
    });
    b
}

pub(crate) fn isSection(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::AL { .. }, .. } => true,
        FCore::Node { data: FCore::Data::EQ { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isMod(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::MO { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isModHolder(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { name: mut n, data: FCore::Data::MO { .. }, .. } => {
            stringEq((n.clone()).clone(), (arcstr::literal!(modNodeName)).clone())
        },
        _ => {
            false
        },
    });
    b
}

pub(crate) fn isClone(mut inNode: Node) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inNode) {
        FCore::Node { parents: Deref @ metamodelica::List::Cons { head: r, tail: _ }, .. } => {
            b = isRefVersion(r.clone())?;
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isVersion(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::VR { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isDims(mut inNode: Node) -> bool {
    let mut b: bool;
    b = (match inNode {
        FCore::Node { data: FCore::Data::DIMS { .. }, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isIn(mut inNode: Node, mut inFunctionRefIs: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type FunctionRefIs = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut b: bool = false;
    b = (match inFunctionRefIs.clone() {
        _ => {
            let mut s: Scope;
            let mut b1: bool;
            let mut b2: bool;
            s = originalScope(toRef(inNode.clone()))?;
            b1 = List::applyAndFold(s.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), inFunctionRefIs.clone(), false)?;
            s = contextualScope(toRef(inNode))?;
            b2 = List::applyAndFold(s.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), inFunctionRefIs.clone(), false)?;
            b = boolOr(b1.clone(), b2.clone());
            b
        },
    });
    Ok(b)
}

pub(crate) fn nonImplicitRefFromScope(mut inScope: Scope) -> Result<Ref> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inScope) {
        Deref @ metamodelica::List::Nil => {
            return Ok(bail!("fail"))
        },
        Deref @ metamodelica::List::Cons { head: r, tail: _ } if (!(isRefImplicitScope(r.clone())?)) => {
            return Ok(r.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            { inScope = rest.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn namesUpToParentName(mut inRef: Ref, mut inName: Name) -> Result<Names> {
    let mut outNames: Names;
    outNames = namesUpToParentName_dispatch(inRef.clone(), (inName).clone(), metamodelica::nil())?;
    Ok(outNames)
}

fn namesUpToParentName_dispatch(mut inRef: Ref, mut inName: Name, mut acc: Names) -> Result<Names> {
    '__tco: loop {
        match (inRef.clone(), inName.clone()) {
        (mut r, _) if (isRefTop(r.clone())?) => {
            return Ok(metamodelica::nil())
        },
        (mut r, _) if (stringEq((inName.clone()).clone(), (refName(r.clone())?).clone())) => {
            return Ok(acc)
        },
        (mut r, mut name) => {
            { (inRef, inName, acc) = (original(refParents(r.clone())?)?, (name.clone()).clone(), metamodelica::cons((refName(r.clone())?).clone(), acc)); continue '__tco; }
        },
    }
    }
}

pub(crate) fn getModifierTarget(mut inRef: Ref) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else { bail!("nomatch") };
            if !((isRefTop(r.clone())?)) { bail!("guard") }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else { bail!("nomatch") };
            if !((isRefModHolder(r.clone())?)) { bail!("guard") }
            r = original(refParents(r.clone())?)?;
            let __pa0 = ::match_deref::match_deref! { match &(refRefTargetScope(r.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            Ok(r.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(getModifierTarget(original(refParents(inRef.clone())?)?)?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRef)
}

pub(crate) fn originalScope(mut inRef: Ref) -> Result<Scope> {
    let mut outScope: Scope;
    outScope = originalScope_dispatch(inRef.clone(), metamodelica::nil())?;
    Ok(outScope)
}

pub(crate) fn originalScope_dispatch(mut inRef: Ref, mut inAcc: Scope) -> Result<Scope> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAcc) {
        acc if (isTop(fromRef(inRef.clone())?)) => {
            return Ok(metamodelica::cons(inRef.clone(), acc.clone()).reverse())
        },
        acc => {
            let mut r: Ref;
            r = original(parents(fromRef(inRef.clone())?)?)?;
            { (inRef, inAcc) = (r.clone(), metamodelica::cons(inRef.clone(), acc.clone())); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn original(mut inParents: Parents) -> Result<Ref> {
    let mut outOriginal: Ref;
    outOriginal = List::last(inParents)?;
    Ok(outOriginal)
}

pub(crate) fn contextualScope(mut inRef: Ref) -> Result<Scope> {
    let mut outScope: Scope;
    outScope = contextualScope_dispatch(inRef.clone(), metamodelica::nil())?;
    Ok(outScope)
}

pub(crate) fn contextualScope_dispatch(mut inRef: Ref, mut inAcc: Scope) -> Result<Scope> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAcc) {
        acc if (isTop(fromRef(inRef.clone())?)) => {
            return Ok(metamodelica::cons(inRef.clone(), acc.clone()).reverse())
        },
        acc => {
            let mut r: Ref;
            r = contextual(parents(fromRef(inRef.clone())?)?)?;
            { (inRef, inAcc) = (r.clone(), metamodelica::cons(inRef.clone(), acc.clone())); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn contextual(mut inParents: Parents) -> Result<Ref> {
    let mut outContextual: Ref;
    outContextual = listHead(inParents)?;
    Ok(outContextual)
}

pub(crate) fn lookupRef(mut inRef: Ref, mut inScope: Scope) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inScope;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(inRef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                s => {
                    let mut r: Ref;
                    let mut s = (*s).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(s.clone().reverse()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    s = __pa0.clone();
                    r = lookupRef_dispatch(inRef.clone(), s.clone())?;
                    Ok(r.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRef)
}

pub(crate) fn lookupRef_dispatch(mut inRef: Ref, mut inScope: Scope) -> Result<Ref> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inScope) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inRef.clone())
        },
        Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
            let mut n: Name;
            let mut r = (*r).clone();
            n = (name(fromRef(r.clone())?)?).clone();
            r = child(inRef.clone(), (n.clone()).clone())?;
            { (inRef, inScope) = (r.clone(), rest.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn filter(mut inRef: Ref, mut inFilter: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>) -> Result<Refs> {
    pub type Filter = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut filtered: Refs;
    let mut c: Children;
    c = children(fromRef(inRef.clone())?)?;
    filtered = FCore::RefTree::fold(c, (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static> = inFilter.clone(); move |__pe_a0, __pe_a1, __pe_a3| filter_work(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>) -> Result<Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>> + 'static>), metamodelica::nil())?;
    filtered = filtered.reverse();
    Ok(filtered)
}

fn filter_work(mut name: Name, mut r#ref: Ref, mut filter: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>, mut accum: Refs) -> Result<Refs> {
    pub type Filter = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut refs: Refs = accum.clone();
    if filter(r#ref.clone())? {
        refs = metamodelica::cons(r#ref.clone(), refs);
    }
    Ok(refs)
}

pub(crate) fn isRefExtends(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isExtends(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefDerived(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isDerived(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefComponent(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isComponent(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefConstrainClass(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isConstrainClass(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefClass(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isClass(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefInstance(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isInstance(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefRedeclare(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isRedeclare(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefClassExtends(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isClassExtends(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefCref(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isCref(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefReference(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isReference(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefUserDefined(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isUserDefined(fromRef(inRef.clone())?)?;
    Ok(b)
}

pub(crate) fn isRefTop(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isTop(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefBasicType(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isBasicType(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefBuiltin(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isBuiltin(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefFunction(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isFunction(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefRecord(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isRecord(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefSection(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isSection(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefMod(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isMod(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefModHolder(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isModHolder(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefClone(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isClone(fromRef(inRef.clone())?)?;
    Ok(b)
}

pub(crate) fn isRefVersion(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isVersion(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefDims(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isDims(fromRef(inRef.clone())?);
    Ok(b)
}

pub(crate) fn isRefIn(mut inRef: Ref, mut inFunctionRefIs: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type FunctionRefIs = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut b: bool;
    b = isIn(fromRef(inRef.clone())?, inFunctionRefIs.clone())?;
    Ok(b)
}

pub(crate) fn dfs(mut inRef: Ref) -> Result<Refs> {
    let mut outRefs: Refs;
    outRefs = (match inRef.clone() {
        _ => {
            let mut refs: Refs;
            let mut c: Children;
            c = children(fromRef(inRef.clone())?)?;
            refs = FCore::RefTree::listValues(c.clone(), metamodelica::nil());
            refs = List::flatten(List::map(refs.clone(), (std::sync::Arc::new(dfs) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>> + 'static>))?)?;
            refs = metamodelica::cons(inRef.clone(), refs.clone());
            refs.clone()
        },
    });
    Ok(outRefs)
}

pub(crate) fn apply1<ExtraArg: Clone + 'static + metamodelica::gc::MMTrace>(mut inRef: Ref, mut inApply: Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, ExtraArg) -> Result<ExtraArg> + 'static>, mut inExtraArg: ExtraArg) -> Result<ExtraArg> {
    pub type Apply<ExtraArg: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Name, Ref, ExtraArg) -> Result<ExtraArg> + 'static>;

    let mut outExtraArg: ExtraArg;
    outExtraArg = FCore::RefTree::fold(children(fromRef(inRef.clone())?)?, inApply.clone(), inExtraArg)?;
    outExtraArg = inApply((refName(inRef.clone())?).clone(), inRef.clone(), outExtraArg)?;
    Ok(outExtraArg)
}

pub(crate) fn hasImports(mut inNode: Node) -> Result<bool> {
    let mut b: bool = false;
    b = (match inNode.clone() {
        _ => {
            let mut qi: Arc<metamodelica::List<Absyn::Import>>;
            let mut uqi: Arc<metamodelica::List<Absyn::Import>>;
            let FCore::IMPORT_TABLE { hidden: _, qualifiedImports: __pa0, unqualifiedImports: __pa1 } = (importTable(fromRef(refImport(toRef(inNode))?)?)?) else { bail!("pattern mismatch") };
            qi = __pa0.clone();
            uqi = __pa1.clone();
            b = boolOr(!(qi.clone().is_empty()), !(uqi.clone().is_empty()));
            b
        },
        _ => {
            false
        },
    });
    Ok(b)
}

pub(crate) fn imports(mut inNode: Node) -> Result<(Arc<metamodelica::List<Absyn::Import>>, Arc<metamodelica::List<Absyn::Import>>)> {
    let mut outQualifiedImports: Arc<metamodelica::List<Absyn::Import>>;
    let mut outUnQualifiedImports: Arc<metamodelica::List<Absyn::Import>>;
    (outQualifiedImports, outUnQualifiedImports) = (match inNode.clone() {
        _ => {
            let mut qi: Arc<metamodelica::List<Absyn::Import>>;
            let mut uqi: Arc<metamodelica::List<Absyn::Import>>;
            let FCore::IMPORT_TABLE { hidden: _, qualifiedImports: __pa0, unqualifiedImports: __pa1 } = (importTable(fromRef(refImport(toRef(inNode))?)?)?) else { bail!("pattern mismatch") };
            qi = __pa0.clone();
            uqi = __pa1.clone();
            (qi.clone(), uqi.clone())
        },
        _ => {
            (metamodelica::nil(), metamodelica::nil())
        },
    });
    Ok((outQualifiedImports, outUnQualifiedImports))
}

pub(crate) fn derivedRef(mut inRef: Ref) -> Result<Refs> {
    let mut outRefs: Refs;
    outRefs = (match inRef.clone() {
        _ if (isRefDerived(inRef.clone())?) => list![child(inRef.clone(), (arcstr::literal!(refNodeName)).clone())?],
        _ => metamodelica::nil(),
    });
    Ok(outRefs)
}

pub(crate) fn extendsRefs(mut inRef: Ref) -> Result<Refs> {
    let mut outRefs: Refs;
    outRefs = (match inRef.clone() {
        _ if (isRefClass(inRef.clone())?) => {
            let mut refs: Refs;
            let mut rd: Refs;
            rd = derivedRef(inRef.clone())?;
            refs = filter(inRef.clone(), (std::sync::Arc::new(isRefExtends) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>))?;
            refs = List::flatten(List::map1(refs.clone(), (std::sync::Arc::new(filter) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>> + 'static>), (std::sync::Arc::new(isRefReference) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>) -> Result<bool> + 'static>))?)?;
            refs = listAppend(rd.clone(), refs.clone());
            refs.clone()
        },
        _ => {
            metamodelica::nil()
        },
    });
    Ok(outRefs)
}

pub(crate) fn cloneRef(mut inName: Name, mut inRef: Ref, mut inParentRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            (g, r) = clone(fromRef(inRef.clone())?, inParentRef.clone(), g.clone())?;
            addChildRef(inParentRef.clone(), (inName).clone(), r.clone(), false)?;
            (g.clone(), r.clone())
        },
    });
    Ok((outGraph, outRef))
}

pub(crate) fn clone(mut inNode: Node, mut inParentRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match (inNode, inGraph) {
        (FCore::Node { name: mut name, id: mut id, parents: mut parents, children: mut children, data: mut data }, mut g) => {
            let mut n: Node;
            let mut r: Ref;
            let mut parents = parents.clone();
            let mut children = children.clone();
            parents = metamodelica::cons(inParentRef.clone(), parents.clone());
            let (__pa0, ref __pa5 @ FCore::N { name: ref __pa1, id: ref __pa2, parents: ref __pa3, children: _, data: ref __pa4 }) = (FGraph::node(g.clone(), (name.clone()).clone(), parents.clone(), data.clone())) else { bail!("pattern mismatch") };
            g = __pa0.clone();
            name = __pa1.clone();
            id = __pa2.clone();
            parents = __pa3.clone();
            data = __pa4.clone();
            n = __pa5.clone();
            r = toRef(n.clone());
            (g, children) = cloneTree(children.clone(), r.clone(), g.clone())?;
            r = updateRef(r.clone(), FCore::Node { name: (name.clone()).clone(), id: id.clone(), parents: parents.clone(), children: children.clone(), data: data.clone() })?;
            (g.clone(), r.clone())
        },
    });
    Ok((outGraph, outRef))
}

pub(crate) fn cloneTree(mut inChildren: Children, mut inParentRef: Ref, mut inGraph: Graph) -> Result<(Graph, Children)> {
    let mut outGraph: Graph;
    let mut outChildren: Children;
    (outChildren, outGraph) = FCore::RefTree::mapFold(inChildren, (std::sync::Arc::new({ let __pe_b1 = inParentRef.clone(); move |__pe_a0, __pe_a2, __pe_a3| cloneChild(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, FCore::Graph) -> Result<(metamodelica::Array<FCore::Node>, FCore::Graph)> + 'static>), inGraph)?;
    Ok((outGraph, outChildren))
}

fn cloneChild(mut name: Name, mut parentRef: Ref, mut inRef: Ref, mut inGraph: Graph) -> Result<(Ref, Graph)> {
    let mut r#ref: Ref;
    let mut graph: Graph;
    (graph, r#ref) = cloneRef((name).clone(), inRef.clone(), parentRef.clone(), inGraph)?;
    Ok((r#ref, graph))
}

pub(crate) fn copyRef(mut inRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = copyRefNoUpdate(inRef.clone())?;
            (g, r) = updateRefs(r.clone(), g.clone())?;
            (g.clone(), r.clone())
        },
    });
    Ok((outGraph, outRef))
}

pub(crate) fn updateRefs(mut inRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            (r, g) = apply1(inRef.clone(), (std::sync::Arc::new(updateRefInGraph) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, (metamodelica::Array<FCore::Node>, FCore::Graph)) -> Result<(metamodelica::Array<FCore::Node>, FCore::Graph)> + 'static>), (inRef.clone(), g.clone()))?;
            (g.clone(), r.clone())
        },
    });
    Ok((outGraph, outRef))
}

fn updateRefInGraph(mut name: Name, mut inRef: Ref, mut inTopRefAndGraph: (metamodelica::Array<FCore::Node>, FCore::Graph)) -> Result<(metamodelica::Array<FCore::Node>, FCore::Graph)> {
    let mut outTopRefAndGraph: (metamodelica::Array<FCore::Node>, FCore::Graph);
    outTopRefAndGraph = (match inTopRefAndGraph {
        (mut t, mut g) => {
            let mut n: Name;
            let mut i: Id;
            let mut p: Parents;
            let mut c: Children;
            let mut d: Data;
            let FCore::N { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: __pa4 } = (fromRef(inRef.clone())?) else { bail!("pattern mismatch") };
            n = __pa0.clone();
            i = __pa1.clone();
            p = __pa2.clone();
            c = __pa3.clone();
            d = __pa4.clone();
            p = List::map1r(p.clone(), (std::sync::Arc::new(lookupRefFromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>) -> Result<metamodelica::Array<FCore::Node>> + 'static>), t.clone())?;
            d = updateRefInData(d.clone(), t.clone())?;
            updateRef(inRef.clone(), FCore::Node { name: (n.clone()).clone(), id: i.clone(), parents: p.clone(), children: c.clone(), data: d.clone() })?;
            (t.clone(), g.clone())
        },
    });
    Ok(outTopRefAndGraph)
}

pub(crate) fn lookupRefFromRef(mut inRef: Ref, mut inOldRef: Ref) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (match inOldRef.clone() {
        _ => {
            let mut r: Ref;
            let mut s: Scope;
            s = originalScope(inOldRef.clone())?;
            r = lookupRef(inRef.clone(), s.clone())?;
            r.clone()
        },
    });
    Ok(outRef)
}

fn updateRefInData(mut inData: Data, mut inRef: Ref) -> Result<Data> {
    let mut outData: Data;
    outData = (match inData.clone() {
        FCore::Data::REF { target: ref sc } => {
            let mut sc = sc.clone();
            sc = List::map1r(sc.clone(), (std::sync::Arc::new(lookupRefFromRef) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Node>, metamodelica::Array<FCore::Node>) -> Result<metamodelica::Array<FCore::Node>> + 'static>), inRef.clone())?;
            FCore::Data::REF { target: sc.clone() }
        },
        _ => {
            inData
        },
    });
    Ok(outData)
}

pub(crate) fn copyRefNoUpdate(mut inRef: Ref) -> Result<Ref> {
    let mut outRef: Ref = copy(fromRef(inRef.clone())?)?;
    Ok(outRef)
}

fn copy(mut inNode: Node) -> Result<Ref> {
    let mut outRef: Ref;
    let mut node: Node = inNode.clone();
    outRef = (match node.clone() {
        FCore::Node { .. } => {
            node.children = FCore::RefTree::map(node.children.clone(), (std::sync::Arc::new(copyChild) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>) -> Result<metamodelica::Array<FCore::Node>> + 'static>))?;
            toRef(node)
        },
    });
    Ok(outRef)
}

fn copyChild(mut name: Name, mut inRef: Ref) -> Result<Ref> {
    let mut r#ref: Ref = copyRefNoUpdate(inRef.clone())?;
    Ok(r#ref)
}

pub(crate) fn getElement(mut inNode: Node) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (match inNode {
        FCore::Node { data: FCore::Data::CL { e: mut e, .. }, .. } => {
            e.clone()
        },
        FCore::Node { data: FCore::Data::CO { e: mut e, .. }, .. } => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outElement)
}

pub(crate) fn getElementFromRef(mut inRef: Ref) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = getElement(fromRef(inRef.clone())?)?;
    Ok(outElement)
}

pub(crate) fn isImplicitRefName(mut r: Ref) -> Result<bool> {
    let mut b: bool;
    b = (match r.clone() {
        _ if (!(isRefTop(r.clone())?)) => FCore::isImplicitScope((refName(r.clone())?).clone()),
        _ => false,
    });
    Ok(b)
}

pub(crate) fn refInstVar(mut inRef: Ref) -> Result<Arc<DAE::Var>> {
    let mut v: Arc<DAE::Var>;
    let mut r: Ref;
    r = refInstance(inRef.clone())?;
    let FCore::IT { i: __pa0 } = (refData(r.clone())?) else { bail!("pattern mismatch") };
    v = __pa0.clone();
    Ok(v)
}

pub(crate) fn refInstance(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = child(inRef.clone(), (arcstr::literal!(itNodeName)).clone())?;
    Ok(r)
}

pub(crate) fn isRefRefUnresolved(mut inRef: Ref) -> bool {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = b.clone();
            refRef(inRef.clone())?;
            b = refRefTargetScope(inRef.clone())?.is_empty();
            Ok((b, b.clone()))
        })() { b = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    b
}

pub(crate) fn isRefRefResolved(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = !(isRefRefUnresolved(inRef.clone()));
    b
}

pub(crate) fn refRef(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = child(inRef.clone(), (arcstr::literal!(refNodeName)).clone())?;
    Ok(r)
}

pub(crate) fn refRefTargetScope(mut inRef: Ref) -> Result<Scope> {
    let mut sc: Scope;
    let mut r: Ref;
    r = refRef(inRef.clone())?;
    sc = targetScope(fromRef(r.clone())?)?;
    Ok(sc)
}

pub(crate) fn refImport(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = child(inRef.clone(), (arcstr::literal!(imNodeName)).clone())?;
    Ok(r)
}

pub(crate) fn importTable(mut inNode: Node) -> Result<ImportTable> {
    let mut it: ImportTable = <FCore::ImportTable as ::std::default::Default>::default();
    it = (match inNode {
        FCore::Node { data: FCore::Data::IM { i: mut __esc_it }, .. } => {
            it = __esc_it.clone();
            it.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(it)
}

pub(crate) fn mkExtendsName(mut inPath: Arc<Absyn::Path>) -> Result<Name> {
    let mut outName: Name;
    outName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(extendsPrefix)); __mm_s.push_str(&*AbsynUtil::pathString(inPath, (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
    Ok(outName)
}

pub(crate) fn scopeHashWork(mut scope: Scope, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    for mut r in &*scope {
        let mut r = r.clone();
        hash = 31 * hash + stringHashDjb2((refName(r.clone())?).clone());
    }
    Ok(hash)
}

pub(crate) fn scopePathEq(mut scope1: Scope, mut scope2: Scope) -> Result<bool> {
    let mut eq: bool;
    eq = ({
        let mut __acc: Option<bool> = None;
        let __thr_src0 = scope1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = scope2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(r1), Some(r2)) => {
                    let __x = refName(r1.clone())? == refName(r2.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.unwrap_or(true)
    });
    Ok(eq)
}

