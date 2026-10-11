//! # Broadcast — one message, everyone gets a copy
//!
//! **Like you're five:** the school loudspeaker. One announcement, every
//! classroom hears it. A classroom that isn't listening misses it; the
//! announcer does NOT wait for them.
//!
//! **Use it for:** "new slot/block arrived" to every subscriber, config
//! changed, peer joined/left, shutdown notices, pub/sub.
//!
//! **The hard part is slow subscribers.** Options:
//! 1. wait for them: one slow subscriber slows EVERYONE (bad for a node);
//! 2. drop messages for them and count it ("lagged"), which is what we do,
//!    and what `tokio::sync::broadcast` does;
//! 3. disconnect them.
//!
//! Each subscriber has its own bounded inbox. Dead subscribers (Receiver
//! dropped) are removed automatically on the next publish.

use crate::channel::{bounded, Receiver, Sender, TrySendError};
use crate::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

pub struct Broadcast<T> {
    inner: Arc<Inner<T>>,
}

struct Inner<T> {
    subscribers: Mutex<Vec<Sender<T>>>,
    inbox_size: usize,
    lagged: AtomicU64,
}

impl<T: Clone> Broadcast<T> {
    /// Each subscriber can fall `inbox_size` messages behind before losing some.
    pub fn new(inbox_size: usize) -> Self {
        Broadcast {
            inner: Arc::new(Inner {
                subscribers: Mutex::new(Vec::new()),
                inbox_size,
                lagged: AtomicU64::new(0),
            }),
        }
    }

    /// Start listening. You only get messages published AFTER this call.
    pub fn subscribe(&self) -> Receiver<T> {
        let (tx, rx) = bounded(self.inner.inbox_size);
        self.inner.subscribers.lock().push(tx);
        rx
    }

    /// Send a copy to every subscriber. Never waits. Returns how many got it.
    pub fn publish(&self, msg: T) -> usize {
        let mut subs = self.inner.subscribers.lock();
        let mut delivered = 0;
        subs.retain(|tx| match tx.try_send(msg.clone()) {
            Ok(()) => {
                delivered += 1;
                true
            }
            Err(TrySendError::Full(_)) => {
                self.inner.lagged.fetch_add(1, Relaxed); // too slow: they miss this one
                true
            }
            Err(TrySendError::Closed(_)) => false, // unsubscribed: remove
        });
        delivered
    }

    /// Total messages dropped because a subscriber was too slow.
    pub fn lagged(&self) -> u64 {
        self.inner.lagged.load(Relaxed)
    }

    pub fn subscriber_count(&self) -> usize {
        self.inner.subscribers.lock().len()
    }
}

impl<T> Clone for Broadcast<T> {
    fn clone(&self) -> Self {
        Broadcast { inner: self.inner.clone() }
    }
}
