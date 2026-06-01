//! Typed global-root variables for the `openmodelica_frontend` crate.
//!
//! This file is **manually maintained**. It declares the `thread_local!`
//! statics for global roots whose value types are defined in this crate
//! (and therefore cannot be declared in `openmodelica_util::Globals` without
//! creating a circular dependency).
//!
//! See `openmodelica_util/src/Globals.rs` for the full design rationale.

#![allow(non_snake_case, non_upper_case_globals, type_complexity)]

use std::cell::RefCell;
use std::sync::Arc;

// ── Thread-local roots (process-global by MetaModelica semantics) ─────────────

thread_local! {
    // Index 9 — instHashIndex
    //
    // The instantiation hash table. Initialised by `InstHashTable.new()`.
    // Uses `todo!()` as placeholder; always set before first read.
    pub static instHashIndex: RefCell<crate::InstHashTable::HashTable> =
        RefCell::new(todo!("instHashIndex must be initialised by InstHashTable::new() before first use"));

    // Indices 10–12 — instNFInstCacheIndex / instNFNodeCacheIndex /
    // instNFLookupCacheIndex
    //
    // These NF caches store `NFInstNode.InstNode` values from
    // `openmodelica_nf_frontend` and are only accessed by `Script/NFApi.mo`.
    // They are declared in `openmodelica_backend_main::Globals` so that the old
    // frontend does not have to depend on the new-frontend crate.

    // Index 13 — builtinIndex
    //
    // Builtin function index: list of (flag × parse functions).
    // Initialised by FBuiltin.mo; reset to nil() between runs.
    pub static builtinIndex: RefCell<Arc<metamodelica::List<(
        (i32, bool),
        (openmodelica_ast::Absyn::Program, Arc<metamodelica::List<Arc<openmodelica_frontend_types::SCode::Element>>>),
    )>>> = RefCell::new(metamodelica::nil());

    // Index 18 — builtinGraphIndex
    //
    // Builtin environment graph index: list of (flag × FCore.Graph).
    // Initialised by Builtin.mo; reset to nil() between runs.
    pub static builtinGraphIndex: RefCell<Arc<metamodelica::List<(i32, crate::FCore::Graph)>>> =
        RefCell::new(metamodelica::nil());

    // Index 22 — inlineHashTable
    //
    // Hash table used during inlining. Set to Some(...) when inlining starts,
    // None when done. Source: Inline.mo.
    pub static inlineHashTable: RefCell<Option<(
        crate::HashTableCG::HashTable,
        crate::VarTransform::VariableReplacements,
    )>> = const { RefCell::new(None) };

    // Index 24 — operatorOverloadingCache
    //
    // Pair of AVL trees caching operator-overloading resolutions.
    // Reset to empty trees by OperatorOverloading.clearCache().
    pub static operatorOverloadingCache: RefCell<(
        Arc<crate::OperatorOverloading::AvlTreePathPathEnv::Tree>,
        Arc<crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree>,
    )> = RefCell::new((
        Arc::new(crate::OperatorOverloading::AvlTreePathPathEnv::Tree::EMPTY),
        Arc::new(crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::EMPTY),
    ));

    // Index 32 — backendCevalInterface
    //
    // Function table populated by backend registration before any ceval-from-
    // backend operation (cevalCallFunction, cevalInteractiveFunctions,
    // elabCallInteractive).  Uses todo!() since there is no meaningful empty
    // state; always set before first use.
    pub static backendCevalInterface: RefCell<crate::BackendCevalInterface::BackendInterfaceFunctions> =
        RefCell::new(todo!("backendCevalInterface must be registered before first use"));
}
