//! # Semaphore — at most N at a time
//!
//! **Like you're five:** a bouncy castle that fits 5 kids. Each kid takes a
//! wristband to get in and gives it back when they leave. No wristband left?
//! Wait in line.
//!
//! **Use it for:** limits on concurrency. At most 64 open peer connections,
//! at most 8 disk reads in flight, at most 4 RPCs to one validator, a
//! database connection pool. A Mutex is a semaphore with N = 1.
//!
//! The wristband is a [`Permit`]. It's returned in its `Drop`, so you can't
//! forget to give it back, even if your code returns early or panics.

use crate::{Condvar, Mutex};

pub struct Semaphore {
    available: Mutex<usize>,
    freed: Condvar,
}

/// Proof you're "inside". Dropping it lets the next one in.
#[must_use = "dropping the permit immediately releases it"]
pub struct Permit<'a> {
    sem: &'a Semaphore,
}

impl Semaphore {
    pub const fn new(permits: usize) -> Self {
        Semaphore { available: Mutex::new(permits), freed: Condvar::new() }
    }

    /// Wait for a free permit.
    pub fn acquire(&self) -> Permit<'_> {
        let mut n = self.available.lock();
        while *n == 0 {
            n = self.freed.wait(n);
        }
        *n -= 1;
        Permit { sem: self }
    }

    /// Get a permit only if one is free right now (e.g. reject a new
    /// connection with "server busy" instead of queueing it).
    pub fn try_acquire(&self) -> Option<Permit<'_>> {
        let mut n = self.available.lock();
        if *n == 0 {
            return None;
        }
        *n -= 1;
        Some(Permit { sem: self })
    }

    pub fn available(&self) -> usize {
        *self.available.lock()
    }
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        *self.sem.available.lock() += 1;
        self.sem.freed.notify_one();
    }
}
