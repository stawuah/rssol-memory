//! # `RwLock<T>` — many readers OR one writer
//!
//! **What it is:** like a Mutex, but any number of threads can hold a READ
//! lock at the same time. A WRITE lock is exclusive.
//!
//! **Use it when:** reads are frequent and writes are rare: a routing table,
//! a cluster membership list, a config reloaded every few minutes, a cache of
//! validator stakes read by every request.
//!
//! **Don't use it when:** writes are common or the critical section is tiny;
//! a plain Mutex is faster then (an RwLock has more bookkeeping).
//!
//! **How this one works** (Rust Atomics and Locks, ch. 9):
//! `state` = `2 × readers`, `+1` if a writer is WAITING, `u32::MAX` if a
//! writer HOLDS the lock. An odd number means "writer waiting: new readers
//! must wait too". That prevents *writer starvation*, where a constant stream
//! of readers keeps a writer out forever.

use atomic_wait::{wait, wake_all, wake_one};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering::*};

pub struct RwLock<T> {
    state: AtomicU32,
    writer_wake_counter: AtomicU32,
    value: UnsafeCell<T>,
}

// Readers on different threads get `&T` at the same time, so T must be Sync.
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

pub struct ReadGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

pub struct WriteGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            writer_wake_counter: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    /// Shared access. Many threads can hold this at once.
    pub fn read(&self) -> ReadGuard<'_, T> {
        let mut s = self.state.load(Relaxed);
        loop {
            if s % 2 == 0 {
                // Even: no writer waiting or holding. Add one reader (+2).
                assert!(s != u32::MAX - 2, "too many readers");
                match self.state.compare_exchange_weak(s, s + 2, Acquire, Relaxed) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }
            if s % 2 == 1 {
                // Odd: a writer is waiting or holding. Sleep.
                wait(&self.state, s);
                s = self.state.load(Relaxed);
            }
        }
    }

    /// Exclusive access. Waits for all readers to leave.
    pub fn write(&self) -> WriteGuard<'_, T> {
        let mut s = self.state.load(Relaxed);
        loop {
            // Unlocked (0) or only "writer waiting" (1): take it.
            if s <= 1 {
                match self.state.compare_exchange(s, u32::MAX, Acquire, Relaxed) {
                    Ok(_) => return WriteGuard { rwlock: self },
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }
            // Make the state odd so no NEW readers get in.
            if s % 2 == 0 {
                if let Err(e) = self.state.compare_exchange(s, s + 1, Relaxed, Relaxed) {
                    s = e;
                    continue;
                }
            }
            // Sleep until a reader/writer leaves.
            let w = self.writer_wake_counter.load(Acquire);
            s = self.state.load(Relaxed);
            if s >= 2 {
                wait(&self.writer_wake_counter, w);
                s = self.state.load(Relaxed);
            }
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        // 3 → 1 means: we were the last reader AND a writer is waiting.
        if self.rwlock.state.fetch_sub(2, Release) == 3 {
            self.rwlock.writer_wake_counter.fetch_add(1, Release);
            wake_one(&self.rwlock.writer_wake_counter);
        }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Release);
        self.rwlock.writer_wake_counter.fetch_add(1, Release);
        wake_one(&self.rwlock.writer_wake_counter); // one waiting writer
        wake_all(&self.rwlock.state); // all waiting readers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readers_and_writers() {
        let table = RwLock::new(vec![0u32; 4]);
        std::thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    for _ in 0..10_000 {
                        let t = table.read();
                        // A writer changes all 4 slots together, so a reader
                        // must never see them differ (no torn reads).
                        assert!(t.iter().all(|&x| x == t[0]));
                    }
                });
            }
            s.spawn(|| {
                for _ in 0..1_000 {
                    let mut t = table.write();
                    for x in t.iter_mut() {
                        *x += 1;
                    }
                }
            });
        });
        assert_eq!(table.into_inner(), vec![1000; 4]);
    }
}
