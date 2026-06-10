// Manually written
#![allow(non_snake_case)]
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Mutable<T: Clone>(Arc<std::sync::Mutex<T>>);

// `PartialEq` is a conditional impl rather than a struct-level bound so
// that `Mutable<T>` can store values whose `T` does not implement
// `PartialEq` (notably callbacks: `&impl Fn(...)` and similar). MM-level
// code only invokes structural equality on `Mutable<T>` when `T` itself
// is comparable.
impl<T: Clone + PartialEq> PartialEq for Mutable<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        *self_guard == *other_guard
    }
}

/// `Eq` mirrors `PartialEq` (both are content-based). Implemented as a
/// conditional impl on `T: Eq` so wrappers around non-`Eq` types still
/// compile; only callers that demand `Eq` on the wrapper pay the bound.
impl<T: Clone + Eq> Eq for Mutable<T> {}

/// Content-based ordering. Mirrors `PartialEq`'s "lock both, compare
/// inner values" pattern. Locks are always acquired self-then-other in
/// declaration order so the routine is deadlock-free against itself
/// (cross-thread Mutable comparisons assume no concurrent reordering of
/// the same pair of cells in opposite order, which the codegen never
/// generates).
impl<T: Clone + PartialOrd> PartialOrd for Mutable<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        (*self_guard).partial_cmp(&*other_guard)
    }
}

impl<T: Clone + Ord> Ord for Mutable<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        (*self_guard).cmp(&*other_guard)
    }
}

// `Hash` is deliberately NOT implemented. Like `std::cell::RefCell`, the
// interior value can change after a hash has been computed but before the
// hash is consumed; a hashable cell would silently corrupt hash containers
// when the contents mutated. Codegen excludes `Hash` from the derive set
// for any type that transitively contains a `Mutable` (in fact such types
// only carry `PartialEq` — see `derives_for` in `mmtorust/src/codegen.rs`).

/// `Default` is exposed so records containing a `Mutable<T>` can themselves
/// derive `Default` (used by the codegen lowering of `arrayCreateNoInit`
/// when the type-witness dummy is unassigned). The default `Mutable` holds a
/// freshly-locked `T::default()`.
impl<T: Clone + Default> Default for Mutable<T> {
    fn default() -> Self {
        Mutable(Arc::from(std::sync::Mutex::new(T::default())))
    }
}

pub fn create<T: Clone>(data: T) -> Mutable<T> {
    Mutable(Arc::from(std::sync::Mutex::new(data)))
}

pub fn update<T: Clone>(mutable: Mutable<T>, data: T) {
    let mut guard = mutable.0.lock().unwrap();
    *guard = data;
}

pub fn access<T: Clone>(mutable: Mutable<T>) -> T {
    mutable.0.lock().unwrap().clone()
}

/// MetaModelica `referenceEq` on mutable cells: true iff both handles
/// designate the same cell (same `Arc` allocation). Contents are irrelevant —
/// two distinct cells holding equal values are not reference-equal. Called
/// from generated code (the builtin `referenceEq` lowering dispatches here
/// because the `Arc` field is private). Takes references: the call site only
/// needs identity, never ownership.
pub fn referenceEq<T: Clone>(a: &Mutable<T>, b: &Mutable<T>) -> bool {
    Arc::ptr_eq(&a.0, &b.0)
}

/// Identity of the underlying cell, same as the free [`referenceEq`] the
/// concrete-typed lowering dispatches to.
impl<T: Clone> metamodelica::ReferenceEq for Mutable<T> {
    fn reference_eq(&self, other: &Self) -> bool {
        referenceEq(self, other)
    }
}

/// Transitional tracing through the legacy `Arc`-based cell: the cell itself
/// is not a Gc allocation (cycles through it still leak), but its content may
/// transitively hold `Gc` pointers while the codegen migration is in
/// progress, so tracing must pass through. Mirrors dumpster's `Mutex` impl:
/// a busy lock reports `Err` ("keep alive").
impl<T: Clone + MMTrace> MMTrace for Mutable<T> {
    fn mm_accept<V: metamodelica::gc::dumpster::Visitor>(
        &self,
        visitor: &mut V,
    ) -> Result<(), ()> {
        self.0.try_lock().map_err(|_| ())?.mm_accept(visitor)
    }
}

// ── Gc-backed cell ────────────────────────────────────────────────────────────
//
// `Mutable<T>` above is an `Arc<Mutex<T>>`: cheap, but a cell updated with a
// value that transitively contains the cell forms an `Arc` cycle that is
// never deallocated (see `tests/mutable_cycle_drop_tests.rs`). `GcMutable` is
// the cycle-collecting replacement: the cell is a `dumpster::unsync::Gc`
// allocation, so once codegen also allocates the cycle-capable payload types
// with `Gc`, the collector can trace through cells and free dead cycles.
//
// The two cell types coexist while the codegen migration is in progress;
// codegen keeps `Mutable` for payload types that can never be cyclic.

use metamodelica::gc::{dumpster::unsync::Gc, MMTrace, Traced};
use std::cell::RefCell;

pub struct GcMutable<T: MMTrace + 'static>(Gc<RefCell<Traced<T>>>);

impl<T: MMTrace> Clone for GcMutable<T> {
    fn clone(&self) -> Self {
        GcMutable(Gc::clone(&self.0))
    }
}

impl<T: MMTrace + std::fmt::Debug> std::fmt::Debug for GcMutable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.try_borrow() {
            Ok(v) => write!(f, "GcMutable({:?})", v.0),
            Err(_) => write!(f, "GcMutable(<borrowed>)"),
        }
    }
}

/// Content-based equality, mirroring [`Mutable`]'s `PartialEq`. Unlike the
/// `Mutex` cell (which self-deadlocks when comparing a cell against itself),
/// two shared `RefCell` borrows are fine.
impl<T: MMTrace + PartialEq> PartialEq for GcMutable<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0.borrow() == *other.0.borrow()
    }
}

impl<T: MMTrace + Eq> Eq for GcMutable<T> {}

impl<T: MMTrace + Default> Default for GcMutable<T> {
    fn default() -> Self {
        GcMutable(Gc::new(RefCell::new(Traced(T::default()))))
    }
}

/// Cells are traced by handing the inner `Gc` to the collector's visitor;
/// the payload is then traced through `Traced<T>`'s `TraceWith` bridge.
impl<T: MMTrace> MMTrace for GcMutable<T> {
    fn mm_accept<V: metamodelica::gc::dumpster::Visitor>(
        &self,
        visitor: &mut V,
    ) -> Result<(), ()> {
        visitor.visit_unsync(&self.0);
        Ok(())
    }
}

impl<T: MMTrace> GcMutable<T> {
    pub fn create(data: T) -> GcMutable<T> {
        GcMutable(Gc::new(RefCell::new(Traced(data))))
    }

    pub fn update(mutable: GcMutable<T>, data: T) {
        mutable.0.borrow_mut().0 = data;
    }

    pub fn access(mutable: GcMutable<T>) -> T
    where
        T: Clone,
    {
        mutable.0.borrow().0.clone()
    }

    /// MetaModelica `referenceEq`: same cell allocation, contents irrelevant.
    pub fn referenceEq(a: &GcMutable<T>, b: &GcMutable<T>) -> bool {
        Gc::ptr_eq(&a.0, &b.0)
    }
}

impl<T: MMTrace> metamodelica::ReferenceEq for GcMutable<T> {
    fn reference_eq(&self, other: &Self) -> bool {
        GcMutable::referenceEq(self, other)
    }
}
