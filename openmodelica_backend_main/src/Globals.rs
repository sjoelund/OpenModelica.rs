//! Typed global-root variables for the `openmodelica_backend_main` crate.
//!
//! This file is **manually maintained**. It declares the `thread_local!`
//! statics for global roots whose value types are defined in this crate
//! (and therefore cannot be declared in a lower crate's `Globals` without
//! creating a circular dependency).
//!
//! See `openmodelica_util/src/Globals.rs` for the full design rationale.

#![allow(non_snake_case, non_upper_case_globals, type_complexity)]

use std::cell::RefCell;
use std::sync::Arc;

// ── Thread-local roots (process-global by MetaModelica semantics) ─────────────

thread_local! {
    // Index 26 — interactiveCache
    //
    // Cache of interactive lookup results:
    //   list of (program, path, environment).
    // Set to Some(list) when cache is populated; None otherwise.
    //
    // Declared here rather than in `openmodelica_backend` because the tuple's
    // third element, `Interactive.GraphicEnvCache`, is defined in this crate
    // (Script/Interactive.mo); `openmodelica_backend` does not depend on
    // `openmodelica_backend_main`.
    pub static interactiveCache: RefCell<Option<Arc<metamodelica::List<(
        openmodelica_ast::Absyn::Program,
        Arc<openmodelica_ast::Absyn::Path>,
        crate::Interactive::GraphicEnvCache,
    )>>>> = const { RefCell::new(None) };
}
