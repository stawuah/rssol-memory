//! # Channels (mpsc) — threads talk by sending values, not sharing them
//!
//! **Like you're five:** a conveyor belt. Many people (Senders) put boxes on
//! it. One person (the Receiver) takes boxes off at the other end.
//!
//! **Use it when:** work flows in one direction. Request handlers → a single
//! WAL writer, network reader → block processor, workers → results collector.
//!
//! **Bounded ≠ "I know how many items will come".** Bounded means how many
//! can WAIT ON THE BELT at once. Items can keep coming forever:
//! * [`channel()`]: the belt is endless. If the receiver is slow, boxes pile
//!   up until memory runs out.
//! * [`bounded(n)`]: the belt holds `n`. When full, `send` waits
//!   (**backpressure**), or `try_send` tells you it's full so you can drop
//!   the item (**load shedding**).
//!
//! **Ways to take items off:**
//! | Method | If empty... |
//! |---|---|
//! | `recv()` | waits as long as it takes |
//! | `recv_timeout(d)` | waits at most `d` |
//! | `try_recv()` | returns `None` immediately |
//! | `select!` | waits on SEVERAL channels at once (see `select.rs`) |
//!
//! **When does receiving end?** When the belt is empty AND every Sender has
//! been dropped. For a never-ending stream the Senders simply never all go
//! away, so the loop runs until you stop it (see `Stop`).
//!
//! **How the receiver sleeps:** it writes its own thread handle into
//! `rx_waiter` and calls `thread::park()`. A sender that adds an item takes
//! that handle and calls `unpark()`. Rust's park has a "token": an unpark that
//! arrives BEFORE the park isn't lost, it makes the next park return at once.

use crate::select::{Selectable, Waitable};
use crate::{Arc, Condvar, Mutex};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
    receiver_alive: bool,
    capacity: Option<usize>,
    rx_waiter: Option<Thread>, // the receiving thread, if it is asleep
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    not_full: Condvar, // senders sleep here when a bounded channel is full
}

/// The sending end. Clone it to get more producers.
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// The receiving end. Exactly one exists, and it can't be cloned.
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    /// `Cell` is Send but !Sync, so the Receiver can MOVE to another thread
    /// but can't be SHARED between two. That's how "single consumer" is
    /// enforced at compile time.
    _not_sync: PhantomData<std::cell::Cell<()>>,
}

/// The receiver was dropped, so nobody will ever read this. You get the
/// value back.
#[derive(Debug, PartialEq, Eq)]
pub struct SendError<T>(pub T);

/// Why `try_send` didn't send. You get the value back either way.
#[derive(Debug, PartialEq, Eq)]
pub enum TrySendError<T> {
    /// The belt is full right now. Drop it, retry later, or count it.
    Full(T),
    /// The receiver is gone.
    Closed(T),
}

/// All senders were dropped and the queue is empty. No more messages, ever.
#[derive(Debug, PartialEq, Eq)]
pub struct RecvError;

#[derive(Debug, PartialEq, Eq)]
pub enum RecvTimeoutError {
    /// Nothing arrived in time. The channel is still open.
    Timeout,
    /// No more messages, ever.
    Closed,
}

fn make<T>(capacity: Option<usize>) -> (Sender<T>, Receiver<T>) {
    if capacity == Some(0) {
        panic!("bounded(0) would deadlock: use at least 1");
    }
    let shared = Arc::new(Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::new(),
            senders: 1,
            receiver_alive: true,
            capacity,
            rx_waiter: None,
        }),
        not_full: Condvar::new(),
    });
    (
        Sender { shared: shared.clone() },
        Receiver { shared, _not_sync: PhantomData },
    )
}

/// Unbounded channel: `send` never waits.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    make(None)
}

/// Bounded channel: at most `capacity` items wait at once.
pub fn bounded<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    make(Some(capacity))
}

impl<T> Sender<T> {
    /// Put a value on the belt. On a full bounded channel, WAITS for space.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let mut inner = self.shared.inner.lock();
        loop {
            if !inner.receiver_alive {
                return Err(SendError(value));
            }
            match inner.capacity {
                Some(cap) if inner.queue.len() >= cap => {
                    inner = self.shared.not_full.wait(inner); // backpressure
                }
                _ => break,
            }
        }
        inner.queue.push_back(value);
        let waiter = inner.rx_waiter.take();
        drop(inner); // unlock BEFORE waking, so the receiver can lock at once
        if let Some(t) = waiter {
            t.unpark();
        }
        Ok(())
    }

    /// Never waits. If the belt is full you get the value back in
    /// `TrySendError::Full` and decide: drop it, count it, retry later.
    pub fn try_send(&self, value: T) -> Result<(), TrySendError<T>> {
        let mut inner = self.shared.inner.lock();
        if !inner.receiver_alive {
            return Err(TrySendError::Closed(value));
        }
        if inner.capacity.is_some_and(|c| inner.queue.len() >= c) {
            return Err(TrySendError::Full(value));
        }
        inner.queue.push_back(value);
        let waiter = inner.rx_waiter.take();
        drop(inner);
        if let Some(t) = waiter {
            t.unpark();
        }
        Ok(())
    }

    /// Items waiting right now (a snapshot).
    pub fn len(&self) -> usize {
        self.shared.inner.lock().queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Receiver<T> {
    fn take_item(&self) -> Option<T> {
        let v = self.shared.inner.lock().queue.pop_front();
        if v.is_some() {
            self.shared.not_full.notify_one(); // a slot opened up
        }
        v
    }

    /// Core loop shared by recv and recv_timeout.
    fn recv_until(&self, deadline: Option<Instant>) -> Result<T, RecvTimeoutError> {
        loop {
            {
                let mut inner = self.shared.inner.lock();
                if let Some(v) = inner.queue.pop_front() {
                    drop(inner);
                    self.shared.not_full.notify_one();
                    return Ok(v);
                }
                if inner.senders == 0 {
                    return Err(RecvTimeoutError::Closed);
                }
                // Register BEFORE unlocking, so a send right after we unlock
                // will find us and unpark us.
                inner.rx_waiter = Some(thread::current());
            }
            match deadline {
                None => thread::park(),
                Some(d) => {
                    let now = Instant::now();
                    if now >= d {
                        return Err(RecvTimeoutError::Timeout);
                    }
                    thread::park_timeout(d - now);
                }
            }
            // Woken (or spuriously woken): loop and check again.
        }
    }

    /// Waits as long as it takes. `Err` = all senders gone, belt empty.
    pub fn recv(&self) -> Result<T, RecvError> {
        self.recv_until(None).map_err(|_| RecvError)
    }

    /// Waits at most `timeout`. The building block of heartbeats, request
    /// deadlines and "flush every N ms" loops.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self.recv_until(Some(Instant::now() + timeout))
    }

    /// Never waits. `None` if nothing is queued right now.
    pub fn try_recv(&self) -> Option<T> {
        self.take_item()
    }

    /// Takes everything queued right now (batching: e.g. WAL group commit).
    pub fn drain_ready(&self) -> Vec<T> {
        let v: Vec<T> = self.shared.inner.lock().queue.drain(..).collect();
        if !v.is_empty() {
            self.shared.not_full.notify_all();
        }
        v
    }

    /// Items waiting right now (a snapshot).
    pub fn len(&self) -> usize {
        self.shared.inner.lock().queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { rx: self }
    }
}

// select! support: "is something here (or closed)?" + "wake me".
impl<T> Waitable for Receiver<T> {
    fn is_ready(&self) -> bool {
        let inner = self.shared.inner.lock();
        !inner.queue.is_empty() || inner.senders == 0
    }
    fn register(&self) {
        self.shared.inner.lock().rx_waiter = Some(thread::current());
    }
}
impl<T> Selectable for Receiver<T> {
    type Item = T;
    fn take(&self) -> Option<T> {
        self.take_item() // None here means "closed"
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.inner.lock().senders += 1;
        Sender { shared: self.shared.clone() }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock();
        inner.senders -= 1;
        let waiter = if inner.senders == 0 { inner.rx_waiter.take() } else { None };
        drop(inner);
        // Last sender gone: wake the receiver so it can see "closed" and stop.
        if let Some(t) = waiter {
            t.unpark();
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.inner.lock().receiver_alive = false;
        // Wake blocked senders so they get SendError instead of sleeping forever.
        self.shared.not_full.notify_all();
    }
}

pub struct Iter<'a, T> {
    rx: &'a Receiver<T>,
}
impl<T> Iterator for Iter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.rx.recv().ok()
    }
}

pub struct IntoIter<T> {
    rx: Receiver<T>,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.rx.recv().ok()
    }
}
impl<T> IntoIterator for Receiver<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { rx: self }
    }
}

// ───────────────────────────── Chan: the easy handle ─────────────────────────

/// A channel as ONE named thing, used by the `chan!` / `spawn!` / `send!`
/// syntax.
///
/// * In the thread that created it, a `Chan` can send AND receive.
/// * When `spawn!` captures it, each thread gets only a [`Sender`], so a
///   worker trying to receive is a compile error, not a runtime surprise.
/// * `for msg in chan` drops the Chan's own Sender first, so the loop ends
///   when the spawned senders finish. No manual `drop(tx)`.
/// * To use it in `select!`, call `.into_receiver()` first (same reason).
pub struct Chan<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Chan<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Chan { tx, rx }
    }
    pub fn bounded(capacity: usize) -> Self {
        let (tx, rx) = bounded(capacity);
        Chan { tx, rx }
    }
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.tx.send(value)
    }
    pub fn try_send(&self, value: T) -> Result<(), TrySendError<T>> {
        self.tx.try_send(value)
    }
    /// A new Sender for this channel (what `spawn!` gives each thread).
    pub fn sender(&self) -> Sender<T> {
        self.tx.clone()
    }
    /// Back to the raw parts.
    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        (self.tx, self.rx)
    }
    /// Stop sending from here and give back the Receiver.
    pub fn into_receiver(self) -> Receiver<T> {
        self.rx // self.tx is dropped here
    }
}

impl<T> Default for Chan<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Chan<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    /// Drops our own Sender, then yields messages until all spawned senders
    /// are gone.
    fn into_iter(self) -> IntoIter<T> {
        self.into_receiver().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_ends_when_senders_gone() {
        let (tx, rx) = channel();
        for i in 0..3 {
            let tx = tx.clone();
            std::thread::spawn(move || tx.send(i).unwrap());
        }
        drop(tx);
        let mut got: Vec<i32> = rx.into_iter().collect();
        got.sort();
        assert_eq!(got, vec![0, 1, 2]);
    }

    #[test]
    fn bounded_applies_backpressure() {
        let (tx, rx) = bounded(2);
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        assert_eq!(tx.try_send(9), Err(TrySendError::Full(9)));
        let t = std::thread::spawn(move || tx.send(3).unwrap()); // must wait
        std::thread::sleep(Duration::from_millis(50));
        assert!(!t.is_finished(), "third send should be blocked");
        assert_eq!(rx.recv(), Ok(1));
        t.join().unwrap();
        assert_eq!(rx.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn send_after_receiver_dropped_errors() {
        let (tx, rx) = channel();
        drop(rx);
        assert_eq!(tx.send(7), Err(SendError(7)));
    }

    #[test]
    fn recv_timeout_times_out_then_receives() {
        let (tx, rx) = channel();
        let t0 = Instant::now();
        assert_eq!(rx.recv_timeout(Duration::from_millis(30)), Err(RecvTimeoutError::Timeout));
        assert!(t0.elapsed() >= Duration::from_millis(30));
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            tx.send(5).unwrap();
        });
        assert_eq!(rx.recv_timeout(Duration::from_secs(5)), Ok(5));
        assert_eq!(rx.recv_timeout(Duration::from_secs(5)), Err(RecvTimeoutError::Closed));
    }

    #[test]
    fn infinite_stream_through_small_buffer() {
        // 100_000 items through a belt that holds 4: bounded is about
        // "waiting at once", not "total".
        let (tx, rx) = bounded(4);
        std::thread::spawn(move || {
            for i in 0..100_000u64 {
                tx.send(i).unwrap();
            }
        });
        let sum: u64 = rx.into_iter().sum();
        assert_eq!(sum, 100_000 * 99_999 / 2);
    }
}
