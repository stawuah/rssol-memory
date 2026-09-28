//! # `Mutex<T>` and `Condvar` — one-at-a-time access, and "wait until"
//!
//! **Mutex, what it is:** a box around a value with a lock. To touch the
//! value you call `.lock()`, which returns a [`MutexGuard`]. The guard derefs
//! to `&mut T`. When the guard is dropped (end of scope), the lock opens.
//!
//! **Use it when:** several threads must CHANGE the same data: a balance,
//! a map of sessions, an in-memory index, a WAL's current offset.
//!
//! **Don't use it when:** you only read (use `Arc<T>` or `RwLock`), or one
//! thread could own the data and others send it messages (use a channel —
//! usually simpler and never deadlocks on lock order).
//!
//! **How this one works** (Rust Atomics and Locks, ch. 9):
//! `state` is a `u32`:
//!   * `0` = unlocked
//!   * `1` = locked, nobody waiting
//!   * `2` = locked, someone MAY be sleeping
//!
//! Locking tries a quick `0 → 1`. If that fails we spin a little (locks are
//! usually held for nanoseconds), then set `2` and ask the OS to put this
//! thread to sleep (`wait`, a futex on Linux). Unlocking sets `0`; only if
//! the old value was `2` do we pay for a syscall to wake a sleeper.
//!
//! **Condvar, what it is:** "sleep until someone tells me the data changed".
//! Always used with a Mutex, always in a loop that re-checks the condition,
//! because wake-ups can be spurious or "stolen" by another thread.

use atomic_wait::{wait, wake_all, wake_one};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering::*};

/// A lock around a `T`. See module docs.
pub struct Mutex<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}

// SAFETY: the lock guarantees only one thread has `&mut T` at a time, so
// sharing `&Mutex<T>` is fine as long as T may be moved between threads.
// Note: T does NOT need to be Sync. That's the whole point of a Mutex.
unsafe impl<T: Send> Sync for Mutex<T> {}

/// Proof that you hold the lock. Derefs to the value. Unlocks on drop.
///
/// The `'a` lifetime ties the guard to the mutex: the compiler will not let
/// the guard outlive the Mutex it locks.
#[must_use = "if unused the Mutex unlocks immediately"]
pub struct MutexGuard<'a, T> {
    pub(crate) mutex: &'a Mutex<T>,
    /// `*const ()` is !Send, which makes the guard !Send (same as std).
    /// The lock should be released by the thread that took it. This is also
    /// why "holding a std MutexGuard across `.await`" makes a future !Send.
    _not_send: std::marker::PhantomData<*const ()>,
}

// Sharing `&MutexGuard` between threads only exposes `&T`, so T: Sync is enough.
unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self { state: AtomicU32::new(0), value: UnsafeCell::new(value) }
    }

    /// Blocks until the lock is ours.
    ///
    /// ⚠️ Calling `lock()` again on the same Mutex while you still hold the
    /// guard = deadlock with yourself. The thread sleeps forever.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        // Fast path: 0 → 1. Acquire = see all writes made before the last unlock.
        if self.state.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
            lock_contended(&self.state);
        }
        MutexGuard { mutex: self, _not_send: std::marker::PhantomData }
    }

    /// Never blocks. `None` if someone else holds the lock.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.state
            .compare_exchange(0, 1, Acquire, Relaxed)
            .ok()
            .map(|_| MutexGuard { mutex: self, _not_send: std::marker::PhantomData })
    }

    /// Get the value back out (consumes the Mutex, so no lock needed).
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// `&mut self` already proves nobody else can touch it: no locking needed.
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

#[cold]
fn lock_contended(state: &AtomicU32) {
    // 1) Spin briefly: the holder is probably about to unlock.
    let mut spin = 0;
    while state.load(Relaxed) == 1 && spin < 100 {
        spin += 1;
        std::hint::spin_loop();
    }
    if state.compare_exchange(0, 1, Acquire, Relaxed).is_ok() {
        return;
    }
    // 2) Mark "someone is waiting" (2) and sleep until state changes.
    //    swap returns the old value: if it was 0, we just took the lock.
    while state.swap(2, Acquire) != 0 {
        wait(state, 2); // sleeps only if state is still 2 (no lost wake-ups)
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: the guard exists, so we hold the lock.
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the guard exists, so we hold the lock exclusively.
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Release = publish our writes to the next thread that locks.
        if self.mutex.state.swap(0, Release) == 2 {
            wake_one(&self.mutex.state); // syscall only if someone may sleep
        }
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// "Wait until the condition changes". See module docs.
///
/// ```
/// use strand::{Arc, Condvar, Mutex};
/// let pair = Arc::new((Mutex::new(false), Condvar::new()));
/// let p2 = pair.clone();
/// std::thread::spawn(move || {
///     *p2.0.lock() = true;
///     p2.1.notify_one();
/// });
/// let mut ready = pair.0.lock();
/// while !*ready {                 // ALWAYS a loop, never an `if`
///     ready = pair.1.wait(ready); // unlocks, sleeps, re-locks
/// }
/// ```
pub struct Condvar {
    counter: AtomicU32,
    num_waiters: AtomicUsize,
}

impl Condvar {
    pub const fn new() -> Self {
        Self { counter: AtomicU32::new(0), num_waiters: AtomicUsize::new(0) }
    }

    /// Wake one sleeping thread (if any).
    pub fn notify_one(&self) {
        if self.num_waiters.load(Relaxed) > 0 {
            self.counter.fetch_add(1, Relaxed);
            wake_one(&self.counter);
        }
    }

    /// Wake every sleeping thread.
    pub fn notify_all(&self) {
        if self.num_waiters.load(Relaxed) > 0 {
            self.counter.fetch_add(1, Relaxed);
            wake_all(&self.counter);
        }
    }

    /// Unlock, sleep until notified, lock again. Takes the guard by value so
    /// you can't touch the data while unlocked; gives it back when relocked.
    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        self.num_waiters.fetch_add(1, Relaxed);
        // Read the counter BEFORE unlocking: a notify after this point
        // changes it, so `wait` below returns instead of missing it.
        let counter_value = self.counter.load(Relaxed);
        let mutex = guard.mutex;
        drop(guard);
        wait(&self.counter, counter_value);
        self.num_waiters.fetch_sub(1, Relaxed);
        mutex.lock()
    }
}

impl Default for Condvar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn many_threads_count_correctly() {
        let m = Mutex::new(0u64);
        std::thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..100_000 {
                        *m.lock() += 1;
                    }
                });
            }
        });
        assert_eq!(m.into_inner(), 800_000);
    }

    #[test]
    fn try_lock_fails_while_held() {
        let m = Mutex::new(());
        let g = m.lock();
        assert!(m.try_lock().is_none());
        drop(g);
        assert!(m.try_lock().is_some());
    }
}
