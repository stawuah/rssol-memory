//! # `Arc<T>` and `Weak<T>` — shared ownership across threads
//!
//! **What it is:** a pointer to ONE value on the heap that many threads can
//! own at the same time. The value is freed when the last `Arc` goes away.
//!
//! **Use it when:** several threads need to read the same thing and none of
//! them is "the owner": a config, a connection pool, a routing table, a
//! `Mutex` that guards shared state.
//!
//! **Do you allocate a size?** No. `Arc::new(value)` asks the compiler for
//! `size_of::<ArcData<T>>()` (two counters + your value) and does ONE heap
//! allocation of exactly that size. You never pass a size.
//!
//! ```text
//!   stack (thread A)        heap (one allocation)            stack (thread B)
//!   ┌───────────┐          ┌─────────────────────────┐       ┌───────────┐
//!   │ Arc { ptr }├────────► │ data_ref_count:  2      │ ◄─────┤ Arc { ptr }│
//!   └───────────┘          │ alloc_ref_count: 2      │       └───────────┘
//!                          │ data: Some(your value)  │
//!                          └─────────────────────────┘
//! ```
//!
//! **How does dereferencing work?** `Arc<T>` implements `Deref<Target = T>`,
//! so `arc.field` and `arc.method()` go "through" the pointer automatically.
//! `*arc` gives you a `&T`-like place, but you can NOT move the value out
//! (others may still be using it) and you can NOT mutate it (others may be
//! reading it). To mutate shared data, put a `Mutex` inside: `Arc<Mutex<T>>`.
//!
//! **`Weak<T>`:** a pointer that does NOT keep the value alive. Use it to
//! break cycles (parent ↔ child, peer ↔ peer). Two `Arc`s pointing at each
//! other never reach count 0, so they leak forever.

use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{fence, AtomicUsize, Ordering::*};

struct ArcData<T> {
    /// Number of `Arc`s. When it hits 0 the VALUE is dropped.
    data_ref_count: AtomicUsize,
    /// Number of `Arc`s + `Weak`s. When it hits 0 the MEMORY is freed.
    alloc_ref_count: AtomicUsize,
    /// `None` once the last `Arc` is gone (but `Weak`s may still exist).
    data: UnsafeCell<Option<T>>,
}

/// Shared, thread-safe, reference-counted pointer. See the module docs.
pub struct Arc<T> {
    weak: Weak<T>,
}

/// Non-owning pointer. Call [`Weak::upgrade`] to get an `Arc` if the value
/// is still alive.
pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

// SAFETY: sending a Weak/Arc to another thread lets that thread (a) read the
// T through `&T` (needs T: Sync) and (b) possibly be the one that drops T
// (needs T: Send). So both bounds are required.
unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}
// Arc<T> gets Send/Sync automatically because its only field is a Weak<T>.

impl<T> Arc<T> {
    /// Moves `data` onto the heap. One allocation, size chosen by the compiler.
    pub fn new(data: T) -> Arc<T> {
        let boxed = Box::new(ArcData {
            data_ref_count: AtomicUsize::new(1),
            alloc_ref_count: AtomicUsize::new(1),
            data: UnsafeCell::new(Some(data)),
        });
        Arc {
            weak: Weak {
                // `Box::leak` = "stop managing this memory, I'll free it myself".
                ptr: NonNull::from(Box::leak(boxed)),
            },
        }
    }

    /// Mutable access, but ONLY if this is the last pointer (no other Arc or
    /// Weak). Otherwise `None`, because someone else could be reading.
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
            // Pairs with the Release in Drop: see every write made by the
            // threads that dropped their pointers before us.
            fence(Acquire);
            // SAFETY: we are the only pointer, and we have `&mut self`.
            let arcdata = unsafe { arc.weak.ptr.as_mut() };
            arcdata.data.get_mut().as_mut()
        } else {
            None
        }
    }

    /// Makes a `Weak` pointer to the same value.
    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }

    /// How many `Arc`s exist right now (a snapshot, may change immediately).
    pub fn strong_count(arc: &Self) -> usize {
        arc.weak.data().data_ref_count.load(Relaxed)
    }

    /// True if both point at the same allocation.
    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        a.weak.ptr == b.weak.ptr
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        // SAFETY: the allocation lives as long as any Arc or Weak exists,
        // and `self` is one of them.
        unsafe { self.ptr.as_ref() }
    }

    /// Returns an `Arc` if the value is still alive, `None` if it was dropped.
    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None; // value already dropped: can't resurrect it
            }
            assert!(n < usize::MAX);
            // Only increment if it's still `n`. If another thread changed it
            // (e.g. dropped the last Arc) we retry with the new value.
            match self.data().data_ref_count.compare_exchange_weak(n, n + 1, Relaxed, Relaxed) {
                Ok(_) => return Some(Arc { weak: self.clone() }),
                Err(actual) => n = actual,
            }
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    /// This is what makes `arc.field` work.
    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        // SAFETY: while an Arc exists, data is Some and nobody mutates it
        // (get_mut requires being the ONLY pointer).
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        // Relaxed is enough: we already hold a pointer, so the data can't
        // disappear under us; we just need the count to be correct.
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort(); // someone leaked billions of clones
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Clone for Arc<T> {
    /// Cloning an Arc does NOT copy your data. It copies the pointer and
    /// bumps the counter. Cheap: one atomic add.
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { weak }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        // Release: publish all our writes before we let go.
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            // Acquire: see everyone else's writes before we free memory.
            fence(Acquire);
            // SAFETY: we were the last pointer of any kind.
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())) };
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr = self.weak.data().data.get();
            // SAFETY: last Arc, so nobody else can be reading the value.
            // Weak pointers may still exist, so memory stays; value goes.
            unsafe { *ptr = None };
        }
        // Then `self.weak` is dropped, which frees memory if it was last.
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Arc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    static DROPS: AtomicUsize = AtomicUsize::new(0);
    struct Probe;
    impl Drop for Probe {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Relaxed);
        }
    }

    #[test]
    fn value_dropped_once_after_last_arc() {
        let a = Arc::new(("hello", Probe));
        let b = a.clone();
        let w = Arc::downgrade(&a);
        let t = std::thread::spawn(move || assert_eq!(b.0, "hello"));
        t.join().unwrap();
        assert_eq!(DROPS.load(Relaxed), 0);
        assert!(w.upgrade().is_some());
        drop(a);
        assert_eq!(DROPS.load(Relaxed), 1);
        assert!(w.upgrade().is_none());
    }

    #[test]
    fn get_mut_only_when_unique() {
        let mut a = Arc::new(5);
        *Arc::get_mut(&mut a).unwrap() += 1;
        let b = a.clone();
        assert!(Arc::get_mut(&mut a).is_none());
        drop(b);
        assert_eq!(*a, 6);
    }
}
