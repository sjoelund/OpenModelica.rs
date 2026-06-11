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

use crate::NFClass as Class;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::NFType;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFAttributes {
    pub connectorType: i32,
    pub parallelism: Prefixes::Parallelism,
    pub variability: Prefixes::Variability,
    pub direction: Prefixes::Direction,
    pub innerOuter: Prefixes::InnerOuter,
    pub isFinal: bool,
    pub isRedeclare: bool,
    pub isReplaceable: Prefixes::Replaceable,
    pub isResizable: bool,
}

impl metamodelica::gc::MMTrace for NFAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.connectorType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.parallelism, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.direction, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isFinal, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isRedeclare, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isReplaceable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isResizable, __mmv)?;
        Ok(())
    }
}
impl Default for NFAttributes {
    fn default() -> Self {
        Self {
            connectorType: Default::default(),
            parallelism: Default::default(),
            variability: Default::default(),
            direction: Default::default(),
            innerOuter: Default::default(),
            isFinal: Default::default(),
            isRedeclare: Default::default(),
            isReplaceable: Default::default(),
            isResizable: Default::default(),
        }
    }
}

pub type ATTRIBUTES = NFAttributes;

thread_local! { static __DEFAULT_ATTR_TLS: Arc<NFAttributes> = Arc::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub fn DEFAULT_ATTR() -> Arc<NFAttributes> { __DEFAULT_ATTR_TLS.with(|__t| __t.clone()) }

thread_local! { static __INPUT_ATTR_TLS: Arc<NFAttributes> = Arc::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::INPUT.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn INPUT_ATTR() -> Arc<NFAttributes> { __INPUT_ATTR_TLS.with(|__t| __t.clone()) }

thread_local! { static __OUTPUT_ATTR_TLS: Arc<NFAttributes> = Arc::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::OUTPUT.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn OUTPUT_ATTR() -> Arc<NFAttributes> { __OUTPUT_ATTR_TLS.with(|__t| __t.clone()) }

thread_local! { static __CONSTANT_ATTR_TLS: Arc<NFAttributes> = Arc::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONSTANT.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn CONSTANT_ATTR() -> Arc<NFAttributes> { __CONSTANT_ATTR_TLS.with(|__t| __t.clone()) }

thread_local! { static __IMPL_DISCRETE_ATTR_TLS: Arc<NFAttributes> = Arc::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::IMPLICITLY_DISCRETE.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub fn IMPL_DISCRETE_ATTR() -> Arc<NFAttributes> { __IMPL_DISCRETE_ATTR_TLS.with(|__t| __t.clone()) }

thread_local! { static __AUGMENTED_ATTR_TLS: Arc<NFAttributes> = Arc::new(NFAttributes { connectorType: ConnectorType::AUGMENTED.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn AUGMENTED_ATTR() -> Arc<NFAttributes> { __AUGMENTED_ATTR_TLS.with(|__t| __t.clone()) }

pub(crate) fn fromSCode(mut compAttr: SCode::Attributes, mut compPrefs: Arc<SCode::Prefixes>) -> Result<Arc<NFAttributes>> {
    let mut attributes: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut par: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    let mut io: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
    let mut fin: bool = false;
    let mut redecl: bool = false;
    let mut repl: Prefixes::Replaceable = Prefixes::Replaceable::NOT_REPLACEABLE;
    attributes = (::match_deref::match_deref! { match &((compAttr.clone(), compPrefs.clone())) {
        (SCode::Attributes { connectorType: SCode::ConnectorType::POTENTIAL { .. }, parallelism: SCode::Parallelism::NON_PARALLEL { .. }, variability: SCode::Variability::VAR { .. }, direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::NOT_REDECLARE { .. }, finalPrefix: SCode::Final::NOT_FINAL { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }) => DEFAULT_ATTR().clone(),
        _ => {
            cty = Prefixes::ConnectorType::fromSCode(compAttr.connectorType.clone())?;
            par = Prefixes::parallelismFromSCode(compAttr.parallelism.clone())?;
            var = Prefixes::variabilityFromSCode(compAttr.variability.clone())?;
            dir = Prefixes::directionFromSCode(compAttr.direction.clone());
            io = Prefixes::innerOuterFromSCode(compPrefs.innerOuter.clone())?;
            fin = SCodeUtil::finalBool(compPrefs.finalPrefix.clone())?;
            redecl = SCodeUtil::redeclareBool(compPrefs.redeclarePrefix.clone())?;
            repl = crate::NFPrefixes::Replaceable::NOT_REPLACEABLE;
            Arc::new(NFAttributes { connectorType: cty, parallelism: par, variability: var, direction: dir, innerOuter: io, isFinal: fin, isRedeclare: redecl, isReplaceable: repl, isResizable: false })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attributes)
}

pub(crate) fn fromDerivedSCode(mut scodeAttr: SCode::Attributes) -> Result<Arc<NFAttributes>> {
    let mut attributes: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    attributes = (match scodeAttr.clone() {
        SCode::Attributes { connectorType: SCode::ConnectorType::POTENTIAL { .. }, variability: SCode::Variability::VAR { .. }, direction: Absyn::Direction::BIDIR { .. }, .. } => DEFAULT_ATTR().clone(),
        _ => {
            cty = Prefixes::ConnectorType::fromSCode(scodeAttr.connectorType.clone())?;
            var = Prefixes::variabilityFromSCode(scodeAttr.variability.clone())?;
            dir = Prefixes::directionFromSCode(scodeAttr.direction.clone());
            Arc::new(NFAttributes { connectorType: cty, parallelism: Parallelism::NON_PARALLEL.clone(), variability: var, direction: dir, innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false })
        },
    });
    Ok(attributes)
}

pub(crate) fn mergeComponentAttributes(mut outerAttr: Arc<NFAttributes>, mut innerAttr: Arc<NFAttributes>, mut node: Arc<InstNode::InstNode>, mut parentRestriction: Arc<Restriction::NFRestriction>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes>;
    let mut cty: i32;
    let mut par: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut fin: bool;
    let mut redecl: bool;
    let mut resize: bool;
    let mut repl: Prefixes::Replaceable;
    if referenceEq(&*(outerAttr.clone()),&*(DEFAULT_ATTR().clone())) && innerAttr.connectorType.clone() == 0 {
        attr = innerAttr;
    } else if referenceEq(&*(innerAttr.clone()),&*(DEFAULT_ATTR().clone())) {
        cty = Prefixes::ConnectorType::merge(outerAttr.connectorType.clone(), innerAttr.connectorType.clone(), node, false)?;
        attr = Arc::new(NFAttributes { connectorType: cty, parallelism: outerAttr.parallelism.clone(), variability: outerAttr.variability.clone(), direction: outerAttr.direction.clone(), innerOuter: innerAttr.innerOuter.clone(), isFinal: outerAttr.isFinal.clone(), isRedeclare: innerAttr.isRedeclare.clone(), isReplaceable: innerAttr.isReplaceable.clone(), isResizable: innerAttr.isResizable.clone() });
    } else {
        cty = Prefixes::ConnectorType::merge(outerAttr.connectorType.clone(), innerAttr.connectorType.clone(), node.clone(), false)?;
        par = Prefixes::mergeParallelism(outerAttr.parallelism.clone(), innerAttr.parallelism.clone(), node.clone())?;
        var = Prefixes::variabilityMin(outerAttr.variability.clone(), innerAttr.variability.clone());
        if Restriction::isFunction(parentRestriction) {
            dir = innerAttr.direction.clone();
        } else {
            dir = Prefixes::mergeDirection(outerAttr.direction.clone(), innerAttr.direction.clone(), node, false)?;
        }
        fin = outerAttr.isFinal.clone() || innerAttr.isFinal.clone();
        redecl = innerAttr.isRedeclare.clone();
        repl = innerAttr.isReplaceable.clone();
        resize = innerAttr.isResizable.clone();
        attr = Arc::new(NFAttributes { connectorType: cty, parallelism: par, variability: var, direction: dir, innerOuter: innerAttr.innerOuter.clone(), isFinal: fin, isRedeclare: redecl, isReplaceable: repl, isResizable: resize });
    }
    Ok(attr)
}

pub(crate) fn mergeDerivedAttributes(mut outerAttr: Arc<NFAttributes>, mut innerAttr: Arc<NFAttributes>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes>;
    let mut cty: i32;
    let mut par: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut io: Prefixes::InnerOuter;
    let mut fin: bool;
    let mut redecl: bool;
    let mut resize: bool;
    let mut repl: Prefixes::Replaceable;
    if referenceEq(&*(innerAttr.clone()),&*(DEFAULT_ATTR().clone())) && outerAttr.connectorType.clone() == 0 {
        attr = outerAttr;
    } else if referenceEq(&*(outerAttr.clone()),&*(DEFAULT_ATTR().clone())) && innerAttr.connectorType.clone() == 0 {
        attr = innerAttr;
    } else {
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(outerAttr) {
            Deref @ ATTRIBUTES { connectorType: __pa0, parallelism: __pa1, variability: __pa2, direction: __pa3, innerOuter: __pa4, isFinal: __pa5, isRedeclare: __pa6, isReplaceable: __pa7, isResizable: __pa8 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cty = __pa0.clone();
        par = __pa1.clone();
        var = __pa2.clone();
        dir = __pa3.clone();
        io = __pa4.clone();
        fin = __pa5.clone();
        redecl = __pa6.clone();
        repl = __pa7.clone();
        resize = __pa8.clone();
        cty = Prefixes::ConnectorType::merge(cty, innerAttr.connectorType.clone(), node.clone(), true)?;
        var = Prefixes::variabilityMin(var, innerAttr.variability.clone());
        dir = Prefixes::mergeDirection(dir, innerAttr.direction.clone(), node, true)?;
        attr = Arc::new(NFAttributes { connectorType: cty, parallelism: par, variability: var, direction: dir, innerOuter: innerAttr.innerOuter.clone(), isFinal: fin, isRedeclare: redecl, isReplaceable: repl, isResizable: resize });
    }
    Ok(attr)
}

pub(crate) fn mergeRedeclaredComponentAttributes(mut origAttr: Arc<NFAttributes>, mut redeclAttr: Arc<NFAttributes>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes>;
    let mut cty: i32;
    let mut rcty: i32;
    let mut cty_fs: i32;
    let mut rcty_fs: i32;
    let mut par: Prefixes::Parallelism;
    let mut rpar: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut rvar: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut rdir: Prefixes::Direction;
    let mut io: Prefixes::InnerOuter;
    let mut rio: Prefixes::InnerOuter;
    let mut fin: bool;
    let mut redecl: bool;
    let mut resize: bool;
    let mut repl: Prefixes::Replaceable;
    if referenceEq(&*(origAttr.clone()),&*(DEFAULT_ATTR().clone())) {
        attr = redeclAttr;
    } else if referenceEq(&*(redeclAttr.clone()),&*(DEFAULT_ATTR().clone())) {
        attr = origAttr;
    } else {
        let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(origAttr) {
            Deref @ ATTRIBUTES { connectorType: __pa0, parallelism: __pa1, variability: __pa2, direction: __pa3, innerOuter: __pa4, isFinal: _, isRedeclare: _, isReplaceable: _, isResizable: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cty = __pa0.clone();
        par = __pa1.clone();
        var = __pa2.clone();
        dir = __pa3.clone();
        io = __pa4.clone();
        let (__pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(redeclAttr) {
            Deref @ ATTRIBUTES { connectorType: __pa5, parallelism: __pa6, variability: __pa7, direction: __pa8, innerOuter: __pa9, isFinal: __pa10, isRedeclare: __pa11, isReplaceable: __pa12, isResizable: __pa13 } => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
            _ => bail!("pattern mismatch"),
        } };
        rcty = __pa5.clone();
        rpar = __pa6.clone();
        rvar = __pa7.clone();
        rdir = __pa8.clone();
        rio = __pa9.clone();
        fin = __pa10.clone();
        redecl = __pa11.clone();
        repl = __pa12.clone();
        resize = __pa13.clone();
        rcty_fs = intBitAnd(rcty, ConnectorType::FLOW_STREAM_MASK.clone());
        cty_fs = intBitAnd(cty, ConnectorType::FLOW_STREAM_MASK.clone());
        if rcty_fs > 0 {
            if cty_fs > 0 && rcty_fs != cty_fs {
                printRedeclarePrefixError(node.clone(), (Prefixes::ConnectorType::toString(rcty)).clone(), (Prefixes::ConnectorType::toString(cty)).clone())?;
            }
        }
        cty = intBitOr(rcty, cty_fs);
        if rpar != Parallelism::NON_PARALLEL.clone() {
            if par != Parallelism::NON_PARALLEL.clone() && par != rpar {
                printRedeclarePrefixError(node.clone(), (Prefixes::parallelismString(rpar)).clone(), (Prefixes::parallelismString(par)).clone())?;
            }
            par = rpar;
        }
        if rvar != Variability::CONTINUOUS.clone() {
            if rvar > var {
                printRedeclarePrefixError(node.clone(), (Prefixes::variabilityString(rvar)?).clone(), (Prefixes::variabilityString(var)?).clone())?;
            }
            var = rvar;
        }
        if rdir != Direction::NONE.clone() {
            if dir != Direction::NONE.clone() && rdir != dir {
                printRedeclarePrefixError(node.clone(), (Prefixes::directionString(rdir)).clone(), (Prefixes::directionString(dir)).clone())?;
            }
            dir = rdir;
        }
        if rio != InnerOuter::NOT_INNER_OUTER.clone() {
            if io != InnerOuter::NOT_INNER_OUTER.clone() && rio != io {
                printRedeclarePrefixError(node, (Prefixes::innerOuterString(rio)).clone(), (Prefixes::innerOuterString(io)).clone())?;
            }
            io = rio;
        }
        attr = Arc::new(NFAttributes { connectorType: cty, parallelism: par, variability: var, direction: dir, innerOuter: io, isFinal: fin, isRedeclare: redecl, isReplaceable: repl, isResizable: resize });
    }
    Ok(attr)
}

pub(crate) fn mergeRedeclaredClassPrefixes(mut origPrefs: Arc<Class::Prefixes::Prefixes>, mut redeclPrefs: Arc<Class::Prefixes::Prefixes>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<Class::Prefixes::Prefixes>> {
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    let mut enc: SCode::Encapsulated;
    let mut par: SCode::Partial;
    let mut fin: SCode::Final;
    let mut io: Absyn::InnerOuter;
    let mut rio: Absyn::InnerOuter;
    let mut repl: Arc<SCode::Replaceable>;
    if referenceEq(&*(origPrefs.clone()),&*(Class::DEFAULT_PREFIXES.clone())) {
        prefs = redeclPrefs;
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(origPrefs) {
            Deref @ Class::Prefixes::PREFIXES { innerOuter: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        io = __pa0.clone();
        let (__pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(redeclPrefs) {
            Deref @ Class::Prefixes::PREFIXES { encapsulatedPrefix: __pa1, partialPrefix: __pa2, finalPrefix: __pa3, innerOuter: __pa4, replaceablePrefix: __pa5 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        enc = __pa1.clone();
        par = __pa2.clone();
        fin = __pa3.clone();
        rio = __pa4.clone();
        repl = __pa5.clone();
        io = (match (io.clone(), rio.clone()) {
        (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, _) => rio,
        (_, Absyn::InnerOuter::NOT_INNER_OUTER { .. }) => io,
        (Absyn::InnerOuter::INNER { .. }, Absyn::InnerOuter::INNER { .. }) => io,
        (Absyn::InnerOuter::OUTER { .. }, Absyn::InnerOuter::OUTER { .. }) => io,
        (Absyn::InnerOuter::INNER_OUTER { .. }, Absyn::InnerOuter::INNER_OUTER { .. }) => io,
        _ => {
            printRedeclarePrefixError(node, (Prefixes::innerOuterString(Prefixes::innerOuterFromSCode(rio)?)).clone(), (Prefixes::innerOuterString(Prefixes::innerOuterFromSCode(io)?)).clone())?;
            bail!("fail")
        },
    });
        prefs = Arc::new(Class::Prefixes::Prefixes { encapsulatedPrefix: enc, partialPrefix: par, finalPrefix: fin, innerOuter: io, replaceablePrefix: repl });
    }
    Ok(prefs)
}

pub(crate) fn printRedeclarePrefixError(mut node: Arc<InstNode::InstNode>, mut prefix1: ArcStr, mut prefix2: ArcStr) -> Result<()> {
    Error::addSourceMessageAndFail(Error::REDECLARE_MISMATCHED_PREFIX.clone(), list![(prefix1).clone(), (InstNode::name(node.clone())?).clone(), (prefix2).clone()], InstNode::info(node))?;
    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    Ok(())
}

pub(crate) fn checkDeclaredComponentAttributes(mut attr: Arc<NFAttributes>, mut parentRestriction: Arc<Restriction::NFRestriction>, mut component: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes> = attr;
    let () = (::match_deref::match_deref! { match &(parentRestriction.clone()) {
        Deref @ Restriction::CONNECTOR { .. } => {
            assertNotInnerOuter(attr.innerOuter.clone(), component.clone(), parentRestriction.clone())?;
            if var_field!((*parentRestriction).isExpandable, Restriction::NFRestriction::CONNECTOR).clone() {
                assertNotFlowStream(attr.connectorType.clone(), component, parentRestriction)?;
                assign_field!(attr.connectorType = intBitOr(attr.connectorType.clone(), ConnectorType::POTENTIALLY_PRESENT.clone()));
            }
            ()
        },
        Deref @ Restriction::RECORD { .. } => {
            assertNotInputOutput(attr.direction.clone(), component.clone(), parentRestriction.clone())?;
            assertNotInnerOuter(attr.innerOuter.clone(), component.clone(), parentRestriction.clone())?;
            assertNotFlowStream(attr.connectorType.clone(), component, parentRestriction)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attr)
}

pub(crate) fn invalidComponentPrefixError(mut prefix: ArcStr, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    Error::addSourceMessage(Error::INVALID_COMPONENT_PREFIX.clone(), list![(prefix).clone(), (InstNode::name(node.clone())?).clone(), (Restriction::toString(restriction)).clone()], InstNode::info(node))?;
    Ok(())
}

pub(crate) fn assertNotInputOutput(mut dir: Prefixes::Direction, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    if dir != Direction::NONE.clone() {
        invalidComponentPrefixError((Prefixes::directionString(dir)).clone(), node, restriction)?;
        bail!("fail");
    }
    Ok(())
}

pub(crate) fn assertNotInnerOuter(mut io: Prefixes::InnerOuter, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    if io != InnerOuter::NOT_INNER_OUTER.clone() {
        invalidComponentPrefixError((Prefixes::innerOuterString(io)).clone(), node, restriction)?;
        bail!("fail");
    }
    Ok(())
}

pub(crate) fn assertNotFlowStream(mut cty: i32, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    if Prefixes::ConnectorType::isFlowOrStream(cty) {
        invalidComponentPrefixError((Prefixes::ConnectorType::toString(cty)).clone(), node, restriction)?;
        bail!("fail");
    }
    Ok(())
}

pub(crate) fn updateComponentConnectorType(mut attributes: Arc<NFAttributes>, mut restriction: Arc<Restriction::NFRestriction>, mut context: i32, mut component: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attributes: Arc<NFAttributes> = attributes;
    let mut cty: i32 = attributes.connectorType.clone();
    if Prefixes::ConnectorType::isConnectorType(cty) {
        if Restriction::isConnector(restriction.clone()) {
            if attributes.variability.clone() < Variability::DISCRETE.clone() && !(InstContext::inRelaxed(context)) && !(Class::isBuiltin(InstNode::getClass(component.clone())?)?) {
                Error::addSourceMessage(Error::INVALID_CONNECTOR_VARIABILITY.clone(), list![(Prefixes::variabilityString(attributes.variability.clone())?).clone(), (InstNode::name(component.clone())?).clone()], InstNode::info(component))?;
                bail!("fail");
            }
            if Restriction::isExpandableConnector(restriction) {
                cty = Prefixes::ConnectorType::setPresent(cty);
            } else {
                cty = intBitAnd(cty, intBitNot(ConnectorType::EXPANDABLE.clone()));
            }
        } else {
            cty = intBitAnd(cty, intBitNot(intBitOr(ConnectorType::CONNECTOR.clone(), ConnectorType::EXPANDABLE.clone())));
        }
        if !(Prefixes::ConnectorType::isFlowOrStream(cty)) {
            cty = Prefixes::ConnectorType::setPotential(cty);
        }
        if cty != attributes.connectorType.clone() {
            assign_field!(attributes.connectorType = cty);
        }
    } else if Prefixes::ConnectorType::isFlowOrStream(cty) && !(InstContext::inRedeclared(context)) {
        Error::addStrictMessage(Error::CONNECTOR_PREFIX_OUTSIDE_CONNECTOR.clone(), list![(Prefixes::ConnectorType::toString(cty)).clone()], InstNode::info(component))?;
        assign_field!(attributes.connectorType = Prefixes::ConnectorType::unsetFlowStream(cty));
    }
    Ok(attributes)
}

pub(crate) fn updateClassConnectorType(mut res: Arc<Restriction::NFRestriction>, mut attrs: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attrs: Arc<NFAttributes> = attrs;
    if Restriction::isExpandableConnector(res.clone()) {
        assign_field!(attrs.connectorType = Prefixes::ConnectorType::setExpandable(attrs.connectorType.clone()));
    } else if Restriction::isConnector(res) {
        assign_field!(attrs.connectorType = Prefixes::ConnectorType::setConnector(attrs.connectorType.clone()));
    }
    attrs
}

pub(crate) fn updateVariability(mut attr: Arc<NFAttributes>, mut cls: Arc<Class::NFClass>, mut clsNode: Arc<InstNode::InstNode>, mut compNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes> = attr;
    let mut var: Prefixes::Variability = attr.variability.clone();
    if referenceEq(&*(attr.clone()),&*(DEFAULT_ATTR().clone())) && InstNode::isDiscreteClass(clsNode.clone())? {
        attr = IMPL_DISCRETE_ATTR().clone();
    } else if var == Variability::CONTINUOUS.clone() && InstNode::isDiscreteClass(clsNode)? {
        assign_field!(attr.variability = Variability::IMPLICITLY_DISCRETE.clone());
    } else if var < Variability::CONTINUOUS.clone() && InstContext::inFunction(context) && attr.direction.clone() != Direction::NONE.clone() && SCodeUtil::isEmptyMod((InstNode::getAnnotation((literal!("__OpenModelica_functionVariability")).clone(), compNode.clone())?).0) {
        assign_field!(attr.variability = Variability::CONTINUOUS.clone());
    } else if var == Variability::PARAMETER.clone() && !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) && Util::getOptionOrDefault(SCodeUtil::lookupBooleanAnnotationMod((InstNode::getAnnotation((literal!("__OpenModelica_resizable")).clone(), compNode)?).0), false) {
        assign_field!(
            attr.variability = Variability::NON_STRUCTURAL_PARAMETER.clone(),
            attr.isResizable = true
        );
    }
    Ok(attr)
}

pub(crate) fn setConnectorType(mut cty: i32, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.connectorType = cty);
    attr
}

pub(crate) fn setVariability(mut var: Prefixes::Variability, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.variability = var);
    attr
}

pub(crate) fn setDirection(mut dir: Prefixes::Direction, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.direction = dir);
    attr
}

pub(crate) fn setInnerOuter(mut io: Prefixes::InnerOuter, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.innerOuter = io);
    attr
}

pub(crate) fn setFinal(mut fin: bool, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.isFinal = fin);
    attr
}

pub(crate) fn setRedeclare(mut redecl: bool, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.isRedeclare = redecl);
    attr
}

pub(crate) fn setReplaceable(mut repl: Prefixes::Replaceable, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.isReplaceable = repl);
    attr
}

pub(crate) fn toDAE(mut ina: Arc<NFAttributes>, mut vis: Prefixes::Visibility) -> Result<Arc<DAE::Attributes>> {
    let mut outa: Arc<DAE::Attributes>;
    outa = Arc::new(DAE::Attributes { connectorType: Prefixes::ConnectorType::toDAE(ina.connectorType.clone()), parallelism: parallelismToSCode(ina.parallelism.clone())?, variability: variabilityToSCode(ina.variability.clone()), direction: directionToAbsyn(ina.direction.clone()), innerOuter: innerOuterToAbsyn(ina.innerOuter.clone())?, visibility: visibilityToSCode(vis) });
    Ok(outa)
}

pub(crate) fn toString(mut attr: Arc<NFAttributes>, mut ty: Arc<NFType::NFType>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (attr.isRedeclare.clone()) {literal!("redeclare ")} else {literal!("")}); __mm_s.push_str(&*if (attr.isFinal.clone()) {literal!("final ")} else {literal!("")}); __mm_s.push_str(&*Prefixes::unparseInnerOuter(attr.innerOuter.clone())); __mm_s.push_str(&*Prefixes::unparseReplaceable(attr.isReplaceable.clone())); __mm_s.push_str(&*Prefixes::unparseParallelism(attr.parallelism.clone())); __mm_s.push_str(&*Prefixes::ConnectorType::unparse(attr.connectorType.clone())); __mm_s.push_str(&*Prefixes::unparseVariability(attr.variability.clone(), ty)?); __mm_s.push_str(&*Prefixes::unparseDirection(attr.direction.clone())); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub(crate) fn toFlatStream(mut attr: Arc<NFAttributes>, mut ty: Arc<NFType::NFType>, mut s: IOStream::IOStream, mut isTopLevel: bool) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = IOStream::append(s, (Prefixes::unparseVariability(attr.variability.clone(), ty)?).clone())?;
    if isTopLevel {
        s = IOStream::append(s, (Prefixes::unparseDirection(attr.direction.clone())).clone())?;
    }
    Ok(s)
}


