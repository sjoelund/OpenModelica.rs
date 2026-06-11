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

use crate::AbsynUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;

pub type Key = Arc<Absyn::Path>;

pub type Value = Option<DAE::Function>;

pub(crate) fn keyStr(mut inKey: Key) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (AbsynUtil::pathString(inKey, (literal!(".")).clone(), true, false)?).clone();
    Ok(outString)
}

pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inValue {
        Some(DAE::Function::FUNCTION { path: mut path, .. }) => {
            AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?
        },
        Some(DAE::Function::RECORD_CONSTRUCTOR { path: mut path, .. }) => {
            AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?
        },
        Some(DAE::Function::RECORD_CONSTRUCTOR { .. }) => {
            literal!("<SOME_FUNCTION>")
        },
        _ => {
            literal!("<NO_FUNCTION>")
        },
    })).clone();
    Ok(outString)
}

pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
    let mut outResult: i32;
    outResult = AbsynUtil::pathCompareNoQual(inKey1, inKey2)?;
    Ok(outResult)
}

pub use addConflictReplace as addConflictDefault;

pub fn addDaeFunction(mut functions: Arc<metamodelica::List<DAE::Function>>, mut functionTree: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut functionTree: Arc<Tree> = functionTree;
    for mut f in &*functions {
        let mut f = f.clone();
        functionTree = add(functionTree.clone(), functionName(f.clone())?, Some(f.clone()), (std::sync::Arc::new(fnptr!(addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
    }
    Ok(functionTree)
}

pub(crate) fn addDaeExtFunction(mut functions: Arc<metamodelica::List<DAE::Function>>, mut functionTree: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut functionTree: Arc<Tree> = functionTree;
    for mut f in &*functions {
        let mut f = f.clone();
        if isExtFunction(f.clone()) {
            functionTree = add(functionTree.clone(), functionName(f.clone())?, Some(f.clone()), (std::sync::Arc::new(fnptr!(addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
        }
    }
    Ok(functionTree)
}

fn functionName(mut elt: DAE::Function) -> Result<Arc<Absyn::Path>> {
    let mut name: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    name = (match elt {
        DAE::Function::FUNCTION { path: ref __esc_name, .. } => {
            name = __esc_name.clone();
            name.clone()
        },
        DAE::Function::RECORD_CONSTRUCTOR { path: ref __esc_name, .. } => {
            name = __esc_name.clone();
            name.clone()
        },
    });
    Ok(name)
}

fn isExtFunction(mut elt: DAE::Function) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(elt) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

/// The binary tree data structure.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Tree {
    NODE {
        /// The key of the node.
        key: Key,
        value: Value,
        /// Height of tree, used for balancing
        height: i32,
        /// Left subtree.
        left: Arc<Tree>,
        /// Right subtree.
        right: Arc<Tree>,
    },
    LEAF {
        /// The key of the node.
        key: Key,
        value: Value,
    },
    EMPTY,
}
impl metamodelica::gc::MMTrace for Tree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Tree::NODE { key, value, height, left, right } => {
                metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                Ok(())
            }
            Tree::LEAF { key, value } => {
                metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            Tree::EMPTY => Ok(()),
        }
    }
}
impl Tree {
    pub fn interned_EMPTY() -> Arc<Tree> {
        thread_local! {
            static INTERNED: Arc<Tree> = Arc::new(Tree::EMPTY);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_EMPTY() -> Arc<Tree> { Tree::interned_EMPTY() }
impl Default for Tree {
    fn default() -> Self { Self::EMPTY }
}
pub use self::Tree::{NODE,LEAF,EMPTY};

pub type ValueNode = Arc<Absyn::Path>;

pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Option<DAE::Function>, Option<DAE::Function>, Arc<Absyn::Path>) -> Result<Option<DAE::Function>> + 'static>) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = inTree.clone();
    tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: inKey, value: inValue })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value;
            let mut key_comp: i32;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey, inValue, conflictFunc.clone())?);
            } else if key_comp == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey, inValue, conflictFunc.clone())?);
            } else {
                value = conflictFunc(inValue, var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                if !((match (&(var_field!((*tree).value, Tree::NODE).clone()), &(value.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (DAE::Function::FUNCTION { path: __refeq_v0l, functions: __refeq_v1l, type_: __refeq_v2l, visibility: __refeq_v3l, partialPrefix: __refeq_v4l, isImpure: __refeq_v5l, inlineType: __refeq_v6l, unusedInputs: __refeq_v7l, source: __refeq_v8l, comment: __refeq_v9l }, DAE::Function::FUNCTION { path: __refeq_v0r, functions: __refeq_v1r, type_: __refeq_v2r, visibility: __refeq_v3r, partialPrefix: __refeq_v4r, isImpure: __refeq_v5r, inlineType: __refeq_v6r, unusedInputs: __refeq_v7r, source: __refeq_v8r, comment: __refeq_v9r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v1l), &*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)) && (match (&(*__refeq_v3l), &(*__refeq_v3r)) { (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true, (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true, _ => false }) && ((*__refeq_v4l) == (*__refeq_v4r)) && ((*__refeq_v5l) == (*__refeq_v5r)) && (match (&(*__refeq_v6l), &(*__refeq_v6r)) { (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true, (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true, (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true, (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true, (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true, (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true, _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v7l), &*(*__refeq_v7r)) && referenceEq(&*(*__refeq_v8l),&*(*__refeq_v8r)) && (match (&(*__refeq_v9l), &(*__refeq_v9r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }), (DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0l, type_: __refeq_v1l, source: __refeq_v2l }, DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0r, type_: __refeq_v1r, source: __refeq_v2r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && referenceEq(&*(*__refeq_v1l),&*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)), _ => false }), _ => false })) {
                    assign_variant_field!(tree => Tree::NODE; value = value);
                }
            }
            if (key_comp == 0) {tree} else {balance(tree)?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value;
            let mut key_comp: i32;
            let mut outTree: Arc<Tree>;
            key_comp = keyCompare(inKey.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey, value: inValue }), right: crate::AvlTreePathFunction::Tree::interned_EMPTY() });
            } else if key_comp == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: crate::AvlTreePathFunction::Tree::interned_EMPTY(), right: Arc::new(Tree::LEAF { key: inKey, value: inValue }) });
            } else {
                value = conflictFunc(inValue, var_field!((*tree).value, Tree::LEAF).clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
                if !((match (&(var_field!((*tree).value, Tree::LEAF).clone()), &(value.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (DAE::Function::FUNCTION { path: __refeq_v0l, functions: __refeq_v1l, type_: __refeq_v2l, visibility: __refeq_v3l, partialPrefix: __refeq_v4l, isImpure: __refeq_v5l, inlineType: __refeq_v6l, unusedInputs: __refeq_v7l, source: __refeq_v8l, comment: __refeq_v9l }, DAE::Function::FUNCTION { path: __refeq_v0r, functions: __refeq_v1r, type_: __refeq_v2r, visibility: __refeq_v3r, partialPrefix: __refeq_v4r, isImpure: __refeq_v5r, inlineType: __refeq_v6r, unusedInputs: __refeq_v7r, source: __refeq_v8r, comment: __refeq_v9r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v1l), &*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)) && (match (&(*__refeq_v3l), &(*__refeq_v3r)) { (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true, (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true, _ => false }) && ((*__refeq_v4l) == (*__refeq_v4r)) && ((*__refeq_v5l) == (*__refeq_v5r)) && (match (&(*__refeq_v6l), &(*__refeq_v6r)) { (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true, (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true, (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true, (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true, (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true, (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true, _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v7l), &*(*__refeq_v7r)) && referenceEq(&*(*__refeq_v8l),&*(*__refeq_v8r)) && (match (&(*__refeq_v9l), &(*__refeq_v9r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }), (DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0l, type_: __refeq_v1l, source: __refeq_v2l }, DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0r, type_: __refeq_v1r, source: __refeq_v2r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && referenceEq(&*(*__refeq_v1l),&*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)), _ => false }), _ => false })) {
                    assign_variant_field!(tree => Tree::LEAF; value = value);
                }
                outTree = tree;
            }
            if (key_comp == 0) {outTree} else {balance(outTree)?}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

pub(crate) fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
    let mut value: Value;
    bail!("fail");
    Ok(value)
}

pub(crate) fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
    let mut value: Value = oldValue.clone();
    value
}

pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
    let mut value: Value = newValue.clone();
    value
}

pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<DAE::Function>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Option<DAE::Function>, Option<DAE::Function>, Arc<Absyn::Path>) -> Result<Option<DAE::Function>> + 'static>) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = tree;
    let mut key: Key;
    let mut value: Value;
    for mut t in &*inValues {
        let mut t = t.clone();
        (key, value) = t.clone();
        tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
    }
    Ok(tree)
}

pub(crate) fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Option<DAE::Function>>) -> Result<Option<DAE::Function>> + 'static>) -> Result<Arc<Tree>> {
    pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Option<DAE::Function>>) -> Result<Value> + 'static>;

    let mut tree: Arc<Tree> = tree;
    let mut key_comp: i32 = 0;
    let mut new_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
    tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => Arc::new(Tree::LEAF { key: key, value: r#fn(None)? }),
        Deref @ Tree::NODE { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::NODE).clone())?;
            if key_comp == -1 {
                assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key, r#fn.clone())?);
            } else if key_comp == 1 {
                assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key, r#fn.clone())?);
            } else {
                assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
            }
            if (key_comp == 0) {tree} else {balance(tree)?}
        },
        Deref @ Tree::LEAF { .. } => {
            key_comp = keyCompare(key.clone(), var_field!((*tree).key, Tree::LEAF).clone())?;
            if key_comp == -1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: key, value: r#fn(None)? }), right: crate::AvlTreePathFunction::Tree::interned_EMPTY() });
            } else if key_comp == 1 {
                new_tree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: crate::AvlTreePathFunction::Tree::interned_EMPTY(), right: Arc::new(Tree::LEAF { key: key, value: r#fn(None)? }) });
            } else {
                assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                new_tree = tree;
            }
            if (key_comp == 0) {new_tree} else {balance(new_tree)?}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

fn balance(mut inTree: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut outTree: Arc<Tree> = inTree.clone();
    outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::LEAF { .. } => {
            inTree
        },
        Deref @ Tree::NODE { .. } => {
            let mut lh: i32;
            let mut rh: i32;
            let mut diff: i32;
            let mut balanced_tree: Arc<Tree>;
            lh = height(var_field!((*outTree).left, Tree::NODE).clone());
            rh = height(var_field!((*outTree).right, Tree::NODE).clone());
            diff = lh - rh;
            if diff < -1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE).clone()) > 0) {rotateLeft(setTreeLeftRight(outTree.clone(), var_field!((*outTree).left, Tree::NODE).clone(), rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?)?)?} else {rotateLeft(outTree)?};
            } else if diff > 1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE).clone()) < 0) {rotateRight(setTreeLeftRight(outTree.clone(), rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?, var_field!((*outTree).right, Tree::NODE).clone())?)?} else {rotateRight(outTree)?};
            } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh, rh) + 1 {
                assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh, rh) + 1);
                balanced_tree = outTree;
            } else {
                balanced_tree = outTree;
            }
            balanced_tree
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTree)
}

fn calculateBalance(mut inNode: Arc<Tree>) -> i32 {
    let mut outBalance: i32;
    outBalance = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => height(var_field!((*inNode).left, Tree::NODE).clone()) - height(var_field!((*inNode).right, Tree::NODE).clone()),
        Deref @ Tree::LEAF { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBalance
}

pub(crate) fn fold<FT: Clone + 'static + metamodelica::gc::MMTrace>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { key, value, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult)?;
            outResult = inFunc(key.clone(), value.clone(), outResult)?;
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult)?;
            outResult
        },
        Deref @ Tree::LEAF { key, value } => {
            outResult = inFunc(key.clone(), value.clone(), outResult)?;
            outResult
        },
        _ => {
            outResult
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outResult)
}

pub(crate) fn foldCond<FT: Clone + 'static + metamodelica::gc::MMTrace>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

    let mut value: FT = value;
    value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            let mut c: bool;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), value)?;
            if c {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value)?;
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value)?;
            }
            value
        },
        Deref @ Tree::LEAF { .. } => {
            let mut c: bool;
            (value, c) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), value)?;
            value
        },
        _ => {
            value
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub(crate) fn fold_2<FT1: Clone + 'static + metamodelica::gc::MMTrace, FT2: Clone + 'static + metamodelica::gc::MMTrace>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> Result<(FT1, FT2)> {
    pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut foldArg1: FT1 = foldArg1;
    let mut foldArg2: FT2 = foldArg2;
    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1, foldArg2)?;
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone(), foldArg1, foldArg2)?;
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1, foldArg2)?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone(), foldArg1, foldArg2)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((foldArg1, foldArg2))
}

pub(crate) fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>) -> Result<()> + 'static>) -> Result<()> {
    pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func(var_field!((*tree).key, Tree::NODE).clone(), var_field!((*tree).value, Tree::NODE).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            func(var_field!((*tree).key, Tree::LEAF).clone(), var_field!((*tree).value, Tree::LEAF).clone())?;
            ()
        },
        Deref @ Tree::EMPTY { .. } => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn fromList(mut inValues: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<DAE::Function>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Option<DAE::Function>, Option<DAE::Function>, Arc<Absyn::Path>) -> Result<Option<DAE::Function>> + 'static>) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = crate::AvlTreePathFunction::Tree::interned_EMPTY();
    let mut key: Key;
    let mut value: Value;
    for mut t in &*inValues {
        let mut t = t.clone();
        (key, value) = t.clone();
        tree = add(tree.clone(), key.clone(), value.clone(), conflictFunc.clone())?;
    }
    Ok(tree)
}

pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
    let mut value: Value;
    let mut k: Key;
    k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
    value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), key)?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), key)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

pub(crate) fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Result<Option<Option<DAE::Function>>> {
    '__tco: loop {
        let mut k: Key;
        k = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } });
        ::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => return Ok(Some(var_field!((*tree).value, Tree::LEAF).clone())),
        (0, Deref @ Tree::NODE { .. }) => return Ok(Some(var_field!((*tree).value, Tree::NODE).clone())),
        (1, Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((*tree).right, Tree::NODE).clone(), key); continue '__tco; },
        ((-1), Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((*tree).left, Tree::NODE).clone(), key); continue '__tco; },
        _ => return Ok(None),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn hasKey(mut inTree: Arc<Tree>, mut inKey: Key) -> Result<bool> {
    let mut comp: bool = false;
    let mut key: Key;
    let mut key_comp: i32;
    let mut tree: Arc<Tree> = Arc::new(Tree::EMPTY);
    key = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inTree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*inTree).key, Tree::LEAF).clone(),
        Deref @ Tree::EMPTY { .. } => {
            return Ok(comp.clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    key_comp = keyCompare(inKey.clone(), key)?;
    comp = (::match_deref::match_deref! { match &((key_comp, inTree)) {
        (0, _) => true,
        (1, Deref @ Tree::NODE { right: __esc_tree, .. }) => {
            tree = (*__esc_tree).clone();
            hasKey(tree.clone(), inKey)?
        },
        ((-1), Deref @ Tree::NODE { left: __esc_tree, .. }) => {
            tree = (*__esc_tree).clone();
            hasKey(tree.clone(), inKey)?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

fn height(mut inNode: Arc<Tree>) -> i32 {
    let mut outHeight: i32;
    outHeight = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inNode).height, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHeight
}

pub(crate) fn intersection() -> Result<()> {
    bail!("fail");
    Ok(())
}

pub(crate) fn isEmpty(mut tree: Arc<Tree>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match &(tree) {
        Deref @ Tree::EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Option<DAE::Function>, Option<DAE::Function>, Arc<Absyn::Path>) -> Result<Option<DAE::Function>> + 'static>) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = tree;
    tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree,
        Deref @ Tree::NODE { .. } => {
            tree = add(tree, var_field!((*treeToJoin).key, Tree::NODE).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree, var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree, var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc.clone())?;
            tree
        },
        Deref @ Tree::LEAF { .. } => add(tree, var_field!((*treeToJoin).key, Tree::LEAF).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

pub(crate) fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
    lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { key, .. } => {
            lst = listKeys(var_field!((*tree).right, Tree::NODE).clone(), lst);
            lst = metamodelica::cons(key.clone(), lst);
            lst = listKeys(var_field!((*tree).left, Tree::NODE).clone(), lst);
            lst
        },
        Deref @ Tree::LEAF { key, .. } => {
            metamodelica::cons(key.clone(), lst)
        },
        _ => {
            lst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub(crate) fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
    lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::LEAF { .. } => metamodelica::cons(var_field!((*inTree).key, Tree::LEAF).clone(), lst),
        Deref @ Tree::NODE { .. } => {
            lst = listKeysReverse(var_field!((*inTree).left, Tree::NODE).clone(), lst);
            lst = metamodelica::cons(var_field!((*inTree).key, Tree::NODE).clone(), lst);
            lst = listKeysReverse(var_field!((*inTree).right, Tree::NODE).clone(), lst);
            lst
        },
        _ => lst,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Option<DAE::Function>>>) -> Arc<metamodelica::List<Option<DAE::Function>>> {
    let mut lst: Arc<metamodelica::List<Option<DAE::Function>>> = lst;
    lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { value, .. } => {
            lst = listValues(var_field!((*tree).right, Tree::NODE).clone(), lst);
            lst = metamodelica::cons(value.clone(), lst);
            lst = listValues(var_field!((*tree).left, Tree::NODE).clone(), lst);
            lst
        },
        Deref @ Tree::LEAF { value, .. } => {
            metamodelica::cons(value.clone(), lst)
        },
        _ => {
            lst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>) -> Result<Option<DAE::Function>> + 'static>) -> Result<Arc<Tree>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

    let mut outTree: Arc<Tree> = inTree.clone();
    outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { key, value, .. } => {
            let mut new_value: Value;
            let mut new_left: Arc<Tree>;
            let mut new_right: Arc<Tree>;
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone())?;
            new_value = inFunc(key.clone(), value.clone())?;
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone())?;
            if !(referenceEq(&*(new_left.clone()),&*(var_field!((*outTree).left, Tree::NODE).clone()))) || !((match (&(value.clone()), &(new_value.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (DAE::Function::FUNCTION { path: __refeq_v0l, functions: __refeq_v1l, type_: __refeq_v2l, visibility: __refeq_v3l, partialPrefix: __refeq_v4l, isImpure: __refeq_v5l, inlineType: __refeq_v6l, unusedInputs: __refeq_v7l, source: __refeq_v8l, comment: __refeq_v9l }, DAE::Function::FUNCTION { path: __refeq_v0r, functions: __refeq_v1r, type_: __refeq_v2r, visibility: __refeq_v3r, partialPrefix: __refeq_v4r, isImpure: __refeq_v5r, inlineType: __refeq_v6r, unusedInputs: __refeq_v7r, source: __refeq_v8r, comment: __refeq_v9r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v1l), &*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)) && (match (&(*__refeq_v3l), &(*__refeq_v3r)) { (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true, (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true, _ => false }) && ((*__refeq_v4l) == (*__refeq_v4r)) && ((*__refeq_v5l) == (*__refeq_v5r)) && (match (&(*__refeq_v6l), &(*__refeq_v6r)) { (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true, (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true, (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true, (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true, (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true, (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true, _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v7l), &*(*__refeq_v7r)) && referenceEq(&*(*__refeq_v8l),&*(*__refeq_v8r)) && (match (&(*__refeq_v9l), &(*__refeq_v9r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }), (DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0l, type_: __refeq_v1l, source: __refeq_v2l }, DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0r, type_: __refeq_v1r, source: __refeq_v2r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && referenceEq(&*(*__refeq_v1l),&*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)), _ => false }), _ => false })) || !(referenceEq(&*(new_right.clone()),&*(var_field!((*outTree).right, Tree::NODE).clone()))) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value, height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left, right: new_right });
            }
            outTree
        },
        Deref @ Tree::LEAF { key, value } => {
            let mut new_value: Value;
            new_value = inFunc(key.clone(), value.clone())?;
            if !((match (&(value.clone()), &(new_value.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (DAE::Function::FUNCTION { path: __refeq_v0l, functions: __refeq_v1l, type_: __refeq_v2l, visibility: __refeq_v3l, partialPrefix: __refeq_v4l, isImpure: __refeq_v5l, inlineType: __refeq_v6l, unusedInputs: __refeq_v7l, source: __refeq_v8l, comment: __refeq_v9l }, DAE::Function::FUNCTION { path: __refeq_v0r, functions: __refeq_v1r, type_: __refeq_v2r, visibility: __refeq_v3r, partialPrefix: __refeq_v4r, isImpure: __refeq_v5r, inlineType: __refeq_v6r, unusedInputs: __refeq_v7r, source: __refeq_v8r, comment: __refeq_v9r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v1l), &*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)) && (match (&(*__refeq_v3l), &(*__refeq_v3r)) { (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true, (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true, _ => false }) && ((*__refeq_v4l) == (*__refeq_v4r)) && ((*__refeq_v5l) == (*__refeq_v5r)) && (match (&(*__refeq_v6l), &(*__refeq_v6r)) { (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true, (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true, (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true, (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true, (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true, (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true, _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v7l), &*(*__refeq_v7r)) && referenceEq(&*(*__refeq_v8l),&*(*__refeq_v8r)) && (match (&(*__refeq_v9l), &(*__refeq_v9r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }), (DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0l, type_: __refeq_v1l, source: __refeq_v2l }, DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0r, type_: __refeq_v1r, source: __refeq_v2r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && referenceEq(&*(*__refeq_v1l),&*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)), _ => false }), _ => false })) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value);
            }
            outTree
        },
        _ => {
            inTree
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTree)
}

pub fn mapFold<FT: Clone + 'static + metamodelica::gc::MMTrace>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Option<DAE::Function>, FT) -> Result<(Option<DAE::Function>, FT)> + 'static>, mut inStartValue: FT) -> Result<(Arc<Tree>, FT)> {
    pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

    let mut outTree: Arc<Tree> = inTree.clone();
    let mut outResult: FT = inStartValue.clone();
    outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { key, value, .. } => {
            let mut new_value: Value;
            let mut new_left: Arc<Tree>;
            let mut new_right: Arc<Tree>;
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult)?;
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult)?;
            if !(referenceEq(&*(new_left.clone()),&*(var_field!((*outTree).left, Tree::NODE).clone()))) || !((match (&(value.clone()), &(new_value.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (DAE::Function::FUNCTION { path: __refeq_v0l, functions: __refeq_v1l, type_: __refeq_v2l, visibility: __refeq_v3l, partialPrefix: __refeq_v4l, isImpure: __refeq_v5l, inlineType: __refeq_v6l, unusedInputs: __refeq_v7l, source: __refeq_v8l, comment: __refeq_v9l }, DAE::Function::FUNCTION { path: __refeq_v0r, functions: __refeq_v1r, type_: __refeq_v2r, visibility: __refeq_v3r, partialPrefix: __refeq_v4r, isImpure: __refeq_v5r, inlineType: __refeq_v6r, unusedInputs: __refeq_v7r, source: __refeq_v8r, comment: __refeq_v9r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v1l), &*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)) && (match (&(*__refeq_v3l), &(*__refeq_v3r)) { (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true, (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true, _ => false }) && ((*__refeq_v4l) == (*__refeq_v4r)) && ((*__refeq_v5l) == (*__refeq_v5r)) && (match (&(*__refeq_v6l), &(*__refeq_v6r)) { (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true, (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true, (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true, (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true, (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true, (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true, _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v7l), &*(*__refeq_v7r)) && referenceEq(&*(*__refeq_v8l),&*(*__refeq_v8r)) && (match (&(*__refeq_v9l), &(*__refeq_v9r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }), (DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0l, type_: __refeq_v1l, source: __refeq_v2l }, DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0r, type_: __refeq_v1r, source: __refeq_v2r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && referenceEq(&*(*__refeq_v1l),&*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)), _ => false }), _ => false })) || !(referenceEq(&*(new_right.clone()),&*(var_field!((*outTree).right, Tree::NODE).clone()))) {
                outTree = Arc::new(Tree::NODE { key: key.clone(), value: new_value, height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left, right: new_right });
            }
            outTree
        },
        Deref @ Tree::LEAF { key, value } => {
            let mut new_value: Value;
            (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
            if !((match (&(value.clone()), &(new_value.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => (match (&(*__refeq_l), &(*__refeq_r)) { (DAE::Function::FUNCTION { path: __refeq_v0l, functions: __refeq_v1l, type_: __refeq_v2l, visibility: __refeq_v3l, partialPrefix: __refeq_v4l, isImpure: __refeq_v5l, inlineType: __refeq_v6l, unusedInputs: __refeq_v7l, source: __refeq_v8l, comment: __refeq_v9l }, DAE::Function::FUNCTION { path: __refeq_v0r, functions: __refeq_v1r, type_: __refeq_v2r, visibility: __refeq_v3r, partialPrefix: __refeq_v4r, isImpure: __refeq_v5r, inlineType: __refeq_v6r, unusedInputs: __refeq_v7r, source: __refeq_v8r, comment: __refeq_v9r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v1l), &*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)) && (match (&(*__refeq_v3l), &(*__refeq_v3r)) { (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true, (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true, _ => false }) && ((*__refeq_v4l) == (*__refeq_v4r)) && ((*__refeq_v5l) == (*__refeq_v5r)) && (match (&(*__refeq_v6l), &(*__refeq_v6r)) { (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true, (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true, (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true, (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true, (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true, (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true, _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(*__refeq_v7l), &*(*__refeq_v7r)) && referenceEq(&*(*__refeq_v8l),&*(*__refeq_v8r)) && (match (&(*__refeq_v9l), &(*__refeq_v9r)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }), (DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0l, type_: __refeq_v1l, source: __refeq_v2l }, DAE::Function::RECORD_CONSTRUCTOR { path: __refeq_v0r, type_: __refeq_v1r, source: __refeq_v2r }) => referenceEq(&*(*__refeq_v0l),&*(*__refeq_v0r)) && referenceEq(&*(*__refeq_v1l),&*(*__refeq_v1r)) && referenceEq(&*(*__refeq_v2l),&*(*__refeq_v2r)), _ => false }), _ => false })) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value);
            }
            outTree
        },
        _ => {
            inTree
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outTree, outResult))
}

pub fn new() -> Arc<Tree> {
    let mut outTree: Arc<Tree> = crate::AvlTreePathFunction::Tree::interned_EMPTY();
    outTree
}

pub(crate) fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub(crate) fn printTreeStr(mut inTree: Arc<Tree>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut left: Arc<Tree> = Arc::new(Tree::EMPTY);
    let mut right: Arc<Tree> = Arc::new(Tree::EMPTY);
    outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::EMPTY { .. } => literal!("EMPTY()"),
        Deref @ Tree::LEAF { .. } => printNodeStr(inTree)?,
        Deref @ Tree::NODE { left: __esc_left, right: __esc_right, .. } => {
            left = (*__esc_left).clone();
            right = (*__esc_right).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(left.clone(), true, (literal!("")).clone())?); __mm_s.push_str(&*printNodeStr(inTree)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(right.clone(), false, (literal!("")).clone())?); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn printTreeStr2(mut inTree: Arc<Tree>, mut isLeft: bool, mut inIndent: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut left: Option<Arc<Tree>>;
    let mut right: Option<Arc<Tree>>;
    outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).left, Tree::NODE).clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft) {literal!("     ")} else {literal!(" │   ")}); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).right, Tree::NODE).clone(), false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent); __mm_s.push_str(&*if (isLeft) {literal!(" │   ")} else {literal!("     ")}); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent); __mm_s.push_str(&*if (isLeft) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn referenceEqOrEmpty(mut t1: Arc<Tree>, mut t2: Arc<Tree>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
        _ => referenceEq(&*(t1),&*(t2)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn rotateLeft(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut outNode: Arc<Tree> = inNode.clone();
    outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), var_field!((**child).left, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), node, var_field!((**child).right, Tree::NODE).clone())?
        },
        Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), crate::AvlTreePathFunction::Tree::interned_EMPTY())?;
            setTreeLeftRight(child.clone(), node, crate::AvlTreePathFunction::Tree::interned_EMPTY())?
        },
        _ => {
            inNode
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

fn rotateRight(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut outNode: Arc<Tree> = inNode.clone();
    outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), var_field!((**child).right, Tree::NODE).clone(), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node)?
        },
        Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree>;
            node = setTreeLeftRight(outNode.clone(), crate::AvlTreePathFunction::Tree::interned_EMPTY(), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), crate::AvlTreePathFunction::Tree::interned_EMPTY(), node)?
        },
        _ => {
            inNode
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

pub(crate) fn setTreeLeftRight(mut orig: Arc<Tree>, mut left: Arc<Tree>, mut right: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut res: Arc<Tree>;
    res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig,
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left, right: right })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left, right: right }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

pub(crate) fn smallestKey(mut tree: Arc<Tree>) -> Result<Key> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => return Ok(var_field!((*tree).key, Tree::NODE).clone()),
        Deref @ Tree::NODE { .. } => { tree = var_field!((*tree).right, Tree::NODE).clone(); continue '__tco; },
        Deref @ Tree::LEAF { .. } => return Ok(var_field!((*tree).key, Tree::LEAF).clone()),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<DAE::Function>)>>) -> Arc<metamodelica::List<(Arc<Absyn::Path>, Option<DAE::Function>)>> {
    let mut lst: Arc<metamodelica::List<(Arc<Absyn::Path>, Option<DAE::Function>)>> = lst;
    lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { key, value, .. } => {
            lst = toList(var_field!((*inTree).right, Tree::NODE).clone(), lst);
            lst = metamodelica::cons((key.clone(), value.clone()), lst);
            lst = toList(var_field!((*inTree).left, Tree::NODE).clone(), lst);
            lst
        },
        Deref @ Tree::LEAF { key, value } => {
            metamodelica::cons((key.clone(), value.clone()), lst)
        },
        _ => {
            lst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub(crate) fn update(mut tree: Arc<Tree>, mut key: Key, mut value: Value) -> Result<Arc<Tree>> {
    let mut outTree: Arc<Tree> = add(tree.clone(), key.clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Option<DAE::Function>, Option<DAE::Function>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Option<DAE::Function>, Option<DAE::Function>, Arc<Absyn::Path>) -> Result<Option<DAE::Function>> + 'static>))?;
    Ok(outTree)
}

