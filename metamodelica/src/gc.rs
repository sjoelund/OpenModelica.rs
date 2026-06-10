//! Cycle-collection bridge between MetaModelica values and the `dumpster`
//! garbage collector.
//!
//! Immutable MetaModelica values are constructed bottom-up and can never be
//! cyclic on their own; reference cycles only arise when a mutable cell
//! (`Mutable.update` / `Pointer.update`) is updated with a value that
//! transitively contains the cell. Plain `Arc` leaks such cycles. The fix is
//! to allocate cycle-capable values with [`dumpster::unsync::Gc`], whose
//! collector traces the object graph and frees unreachable cycles.
//!
//! Only the *unsync* half of dumpster is used: MetaModelica payloads are
//! deliberately not `Send` (`Array<T>` is `Rc<RefCell<Vec<T>>>`, callbacks
//! are `Arc<dyn Fn>` without `+ Send + Sync`), so cycle-capable values never
//! cross threads — see `System::launchParallelTasks` for the precedent.
//!
//! # Why an own trait instead of `dumpster::Trace`
//!
//! Tracing must traverse field types we do not own (`ArcStr`, `Arc<T>`,
//! tuples containing them, …). Implementing dumpster's `TraceWith` for those
//! is forbidden by the orphan rule. [`MMTrace`] is *our* trait, so it can be
//! implemented for any type; the single newtype [`Traced`] bridges it into
//! `TraceWith` at `Gc` allocation boundaries.
//!
//! # The unchecked assumption: function values are not traced
//!
//! There is deliberately **no** `MMTrace` impl for `dyn Fn` types, and the
//! per-type impls emitted by mmtorust skip function-typed fields. A closure
//! that captured a `Gc` pointer would hide that edge from the collector, and
//! a cycle running through it could be freed while still reachable. This is
//! an accepted, unchecked assumption: MetaModelica callbacks capture
//! expressions, options and scalars, not mutable instance nodes. If that
//! ever changes, the capture list of the offending closure — not this module
//! — is the thing to fix.

pub use dumpster;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use arcstr::ArcStr;
use dumpster::{TraceWith, Visitor};

use crate::List;

/// Structural tracing of MetaModelica values: delegate to every field that
/// can (transitively) contain a `Gc` pointer, skip everything else.
///
/// Generated types receive an impl emitted by mmtorust; runtime container
/// types delegate below; scalar leaves accept trivially. The `Result`
/// mirrors `TraceWith::accept`: `Err(())` means the value could not be
/// proven traceable right now (e.g. a `RefCell` currently borrowed), which
/// the collector treats as "keep alive".
pub trait MMTrace {
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()>;
}

/// Bridge newtype: the only point where `MMTrace` meets dumpster's
/// `TraceWith`. `Gc` payloads are wrapped in `Traced` so the collector can
/// drive our traversal.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Traced<T: MMTrace>(pub T);

unsafe impl<T: MMTrace, V: Visitor> TraceWith<V> for Traced<T> {
    fn accept(&self, visitor: &mut V) -> Result<(), ()> {
        self.0.mm_accept(visitor)
    }
}

// ── scalar leaves ─────────────────────────────────────────────────────────────

macro_rules! mm_trace_leaf {
    ($($t:ty),* $(,)?) => {$(
        impl MMTrace for $t {
            #[inline]
            fn mm_accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), ()> {
                Ok(())
            }
        }
    )*};
}

mm_trace_leaf!(
    (), bool, char,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    String, ArcStr,
);

impl MMTrace for ordered_float::OrderedFloat<f64> {
    #[inline]
    fn mm_accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), ()> {
        Ok(())
    }
}

/// Strings, flags and line numbers only.
impl MMTrace for crate::SourceInfo {
    #[inline]
    fn mm_accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), ()> {
        Ok(())
    }
}

// ── containers: delegate to the payload ───────────────────────────────────────

impl<T: MMTrace + ?Sized> MMTrace for Arc<T> {
    #[inline]
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        (**self).mm_accept(visitor)
    }
}

impl<T: MMTrace + ?Sized> MMTrace for Rc<T> {
    #[inline]
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        (**self).mm_accept(visitor)
    }
}

impl<T: MMTrace + ?Sized> MMTrace for Box<T> {
    #[inline]
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        (**self).mm_accept(visitor)
    }
}

impl<T: MMTrace> MMTrace for Option<T> {
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        match self {
            Some(x) => x.mm_accept(visitor),
            None => Ok(()),
        }
    }
}

impl<T: MMTrace> MMTrace for Vec<T> {
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        for x in self {
            x.mm_accept(visitor)?;
        }
        Ok(())
    }
}

/// Mirrors dumpster's own `RefCell` impl: a cell that is currently borrowed
/// cannot be traced, so report `Err` and let the collector keep it alive.
impl<T: MMTrace + ?Sized> MMTrace for RefCell<T> {
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        self.try_borrow().map_err(|_| ())?.mm_accept(visitor)
    }
}

/// Iterative, not recursive: lists are routinely tens of thousands of
/// elements long and a recursive traversal would overflow the stack.
impl<T: MMTrace + Clone> MMTrace for List<T> {
    fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
        let mut cur = self;
        loop {
            match cur {
                List::Cons { head, tail } => {
                    head.mm_accept(visitor)?;
                    cur = tail;
                }
                List::Nil => return Ok(()),
            }
        }
    }
}

macro_rules! mm_trace_tuple {
    ($($name:ident : $idx:tt),+) => {
        impl<$($name: MMTrace),+> MMTrace for ($($name,)+) {
            fn mm_accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ()> {
                $(self.$idx.mm_accept(visitor)?;)+
                Ok(())
            }
        }
    };
}

mm_trace_tuple!(A: 0);
mm_trace_tuple!(A: 0, B: 1);
mm_trace_tuple!(A: 0, B: 1, C: 2);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10);
mm_trace_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11);

// `Array<T>` (= `Rc<RefCell<Vec<T>>>`) is covered by composing the `Rc`,
// `RefCell` and `Vec` impls above.

// ── function values: the unchecked leaves ─────────────────────────────────────
//
// Codegen lowers MetaModelica function values to `Arc<dyn Fn(..) -> .. +
// 'static>`. A closure's captures cannot be traversed, so these impls accept
// trivially — this is the module-level "captures no Gc pointers" assumption.
// One impl per arity; extend the list if codegen ever emits a higher arity.

macro_rules! mm_trace_dyn_fn {
    ($($args:ident),*) => {
        impl<Ret $(,$args)*> MMTrace for dyn Fn($($args),*) -> Ret + 'static {
            #[inline]
            fn mm_accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), ()> {
                Ok(())
            }
        }
    };
}

/// Plain function pointers carry no captures at all; trivially accepted.
/// (They appear in cell contents, e.g. SimCodeUtil's tuple-shaped caches.)
macro_rules! mm_trace_fn_ptr {
    ($($args:ident),*) => {
        impl<Ret $(,$args)*> MMTrace for fn($($args),*) -> Ret {
            #[inline]
            fn mm_accept<V: Visitor>(&self, _visitor: &mut V) -> Result<(), ()> {
                Ok(())
            }
        }
    };
}

mm_trace_fn_ptr!();
mm_trace_fn_ptr!(A);
mm_trace_fn_ptr!(A, B);
mm_trace_fn_ptr!(A, B, C);
mm_trace_fn_ptr!(A, B, C, D);
mm_trace_fn_ptr!(A, B, C, D, E);
mm_trace_fn_ptr!(A, B, C, D, E, F);
mm_trace_fn_ptr!(A, B, C, D, E, F, G);
mm_trace_fn_ptr!(A, B, C, D, E, F, G, H);

mm_trace_dyn_fn!();
mm_trace_dyn_fn!(A);
mm_trace_dyn_fn!(A, B);
mm_trace_dyn_fn!(A, B, C);
mm_trace_dyn_fn!(A, B, C, D);
mm_trace_dyn_fn!(A, B, C, D, E);
mm_trace_dyn_fn!(A, B, C, D, E, F);
mm_trace_dyn_fn!(A, B, C, D, E, F, G);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I, J);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I, J, K);
mm_trace_dyn_fn!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time checks that the shapes generated code actually embeds are
    // traceable: Array, List-of-tuple, nested options.
    fn assert_mm_trace<T: MMTrace>() {}

    #[test]
    fn representative_shapes_are_traceable() {
        assert_mm_trace::<crate::Array<i32>>();
        assert_mm_trace::<Arc<List<(ArcStr, i32)>>>();
        assert_mm_trace::<Option<Box<(String, Vec<f64>)>>>();
    }
}
