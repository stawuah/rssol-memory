//! # Oneshot — exactly one reply
//!
//! **Like you're five:** you give the shopkeeper a self-addressed envelope.
//! They put ONE answer in it and mail it back. The envelope can't be reused.
//!
//! **Use it for:** request → response between threads. "Write this record,
//! and tell me its log position when it's on disk." "What's the balance of
//! account X?" Every actor-style design (and tokio code) uses this.
//!
//! `send(self)` and `recv(self)` take `self` BY VALUE, so the compiler stops
//! you from sending or receiving twice. The type system encodes "one".
//!
//! (Chapter 5 of the book builds a oneshot from a single atomic flag. This
//! one reuses our channel with capacity 1 to stay short.)

use crate::channel::{bounded, Receiver, RecvTimeoutError, Sender};
use crate::select::{Selectable, Waitable};
use std::time::Duration;

pub struct OneshotSender<T>(Sender<T>);
pub struct OneshotReceiver<T>(Receiver<T>);

/// Make a one-use reply channel: `let (reply_to, answer) = oneshot();`
pub fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let (tx, rx) = bounded(1);
    (OneshotSender(tx), OneshotReceiver(rx))
}

impl<T> OneshotSender<T> {
    /// Send the one answer. `Err(value)` if the asker stopped waiting.
    pub fn send(self, value: T) -> Result<(), T> {
        self.0.send(value).map_err(|e| e.0)
    }
}

impl<T> OneshotReceiver<T> {
    /// Wait for the answer. `Err` = the other side dropped the envelope
    /// without answering (e.g. it crashed or panicked).
    pub fn recv(self) -> Result<T, crate::RecvError> {
        self.0.recv()
    }
    /// Wait at most `d`. Every request to another component should have a
    /// deadline: "no answer" is a real outcome in distributed systems.
    pub fn recv_timeout(self, d: Duration) -> Result<T, RecvTimeoutError> {
        self.0.recv_timeout(d)
    }
}

impl<T> Waitable for OneshotReceiver<T> {
    fn is_ready(&self) -> bool {
        self.0.is_ready()
    }
    fn register(&self) {
        self.0.register()
    }
}
impl<T> Selectable for OneshotReceiver<T> {
    type Item = T;
    fn take(&self) -> Option<T> {
        self.0.take()
    }
}
