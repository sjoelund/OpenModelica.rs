//! Drop-detection harness for reference cycles built through `Mutable`.
//!
//! The MetaModelica frontend frequently creates a mutable cell with a
//! placeholder and then `Mutable.update`s it with a value that (transitively)
//! contains the cell itself — e.g. `NFInst.mo` updating `cls_ptr` with an
//! `InstNode` whose class points back through that same pointer. In the Rust
//! port both the cell (`Arc<Mutex<T>>`) and uniontype values (`Arc<enum>`)
//! are strong references, so such cycles are never deallocated.
//!
//! These tests make the leak *observable*: a payload with a `Drop` impl that
//! increments a shared counter, plus a `Weak` handle to the payload. If the
//! cycle is collected, the counter fires and the `Weak` dangles; if it leaks,
//! neither happens.
//!
//! Status: cycle collection is NOT implemented yet. The `*_is_dropped` tests
//! state the desired behavior and are `#[ignore]`d; the `*_currently_leaks`
//! tests pin the status quo. When cycle handling lands, un-ignore the former
//! and delete the latter (they will start failing, which is the signal).

use openmodelica_util_datatypes_basic::Mutable;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

/// Payload whose destruction is observable from the outside.
#[derive(Debug)]
struct DropProbe {
    drops: Arc<AtomicUsize>,
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

/// Creates a probe plus the two observation handles: the drop counter and a
/// `Weak` to the probe allocation itself.
fn probe() -> (Arc<DropProbe>, Weak<DropProbe>, Arc<AtomicUsize>) {
    let drops = Arc::new(AtomicUsize::new(0));
    let p = Arc::new(DropProbe { drops: drops.clone() });
    let weak = Arc::downgrade(&p);
    (p, weak, drops)
}

/// Minimal stand-in for a generated uniontype that stores a `Mutable` cell,
/// shaped like `InstNode.CLASS_NODE { cls: Mutable<Arc<...>>, ... }`.
/// Generated uniontypes are `Arc<enum>`, so the cell holds `Arc<Node>`.
#[derive(Clone, Debug)]
enum Node {
    Empty,
    Link {
        probe: Arc<DropProbe>,
        next: Mutable::Mutable<Arc<Node>>,
    },
}

// ── harness sanity: the probe itself works ──────────────────────────────

// No cycle: a cell points at a Link whose `next` is a *different* cell
// holding Empty. Dropping the outer cell must free the payload — this
// validates that the probe/Weak machinery can actually observe a drop.
#[test]
fn acyclic_mutable_chain_is_dropped() {
    let (p, weak, drops) = probe();
    let tail = Mutable::create(Arc::new(Node::Empty));
    let head = Mutable::create(Arc::new(Node::Link { probe: p, next: tail }));
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(weak.upgrade().is_some());
    drop(head);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert!(weak.upgrade().is_none());
}

// ── the cyclic cases ────────────────────────────────────────────────────

/// Builds the create-then-update self-cycle used throughout the frontend:
/// `cell := Mutable.create(placeholder); Mutable.update(cell, value(cell))`.
/// Returns only the observation handles; every strong reference except the
/// in-cycle one has been dropped by the time this returns.
fn build_self_cycle() -> (Weak<DropProbe>, Arc<AtomicUsize>) {
    let (p, weak, drops) = probe();
    let cell = Mutable::create(Arc::new(Node::Empty));
    Mutable::update(
        cell.clone(),
        Arc::new(Node::Link { probe: p, next: cell.clone() }),
    );
    // `cell` (the last external handle) is dropped here.
    (weak, drops)
}

/// Two-cell cycle: a → b → a, mirroring parent/child node pairs that point
/// at each other's cells.
fn build_two_cell_cycle() -> (Weak<DropProbe>, Arc<AtomicUsize>) {
    let (p, weak, drops) = probe();
    let a = Mutable::create(Arc::new(Node::Empty));
    let b = Mutable::create(Arc::new(Node::Link { probe: p, next: a.clone() }));
    Mutable::update(a.clone(), Arc::new(Node::Empty)); // exercise update on a too
    Mutable::update(
        a,
        Arc::new(Node::Link {
            probe: Arc::new(DropProbe { drops: drops.clone() }),
            next: b,
        }),
    );
    (weak, drops)
}

// Desired behavior once cyclic-drop handling exists: dropping the last
// external handle collects the cycle.
#[test]
#[ignore = "Arc cycles through Mutable are not collected yet; un-ignore when cycle handling is implemented"]
fn self_cycle_through_mutable_update_is_dropped() {
    let (weak, drops) = build_self_cycle();
    assert_eq!(drops.load(Ordering::SeqCst), 1, "cycle payload was not dropped");
    assert!(weak.upgrade().is_none(), "cycle payload is still alive");
}

#[test]
#[ignore = "Arc cycles through Mutable are not collected yet; un-ignore when cycle handling is implemented"]
fn two_cell_cycle_through_mutable_update_is_dropped() {
    let (weak, drops) = build_two_cell_cycle();
    // Both Links (one probe each) must be freed.
    assert_eq!(drops.load(Ordering::SeqCst), 2, "cycle payloads were not dropped");
    assert!(weak.upgrade().is_none(), "cycle payload is still alive");
}

// Pin the current (leaking) behavior so the harness is exercised in CI today.
// DELETE these two tests when the `_is_dropped` tests above are enabled —
// they assert the opposite and will fail once cycles are collected.
#[test]
fn self_cycle_through_mutable_update_currently_leaks() {
    let (weak, drops) = build_self_cycle();
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(weak.upgrade().is_some());
}

#[test]
fn two_cell_cycle_through_mutable_update_currently_leaks() {
    let (weak, drops) = build_two_cell_cycle();
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(weak.upgrade().is_some());
}
