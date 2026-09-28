//! # strand — concurrency with a small syntax, built from scratch
//!
//! Everything here is written by hand on top of atomics, following
//! *Rust Atomics and Locks* (Mara Bos):
//!
//! | Module | What you get | Built from |
//! |---|---|---|
//! | [`arc`] | [`Arc`], [`Weak`] | `AtomicUsize` + one heap allocation |
//! | [`mutex`] | [`Mutex`], [`Condvar`] | `AtomicU32` + futex wait/wake |
//! | [`rwlock`] | [`RwLock`] | `AtomicU32` + futex wait/wake |
//! | [`channel`] | [`Sender`], [`Receiver`], [`Chan`] | our `Arc` + `Mutex` + `Condvar` + park/unpark |
//! | [`select`] | [`select!`], [`Stop`], [`Ticker`] | park/unpark + deadlines |
//! | [`oneshot`] | [`oneshot()`] | a channel of capacity 1 |
//! | [`broadcast`] | [`Broadcast`] | one bounded inbox per subscriber |
//! | [`semaphore`] | [`Semaphore`], [`Permit`] | our `Mutex` + `Condvar` |
//! | [`pool`] | [`Pool`] | bounded channel + oneshot + `catch_unwind` |
//! | [`sharded`] | [`Sharded`] | N × `Mutex<HashMap>` |
//!
//! On top sits the **small syntax** (macros). Each one expands to the plain
//! Rust you'd otherwise write by hand (`move`, `clone`, `drop`, `join`).
//!
//! ```
//! use strand::*;
//!
//! chan!(jobs: String);                        // a channel named `jobs`
//!
//! let _crew = spawn!(4, [jobs], |id| {        // 4 threads, each gets a sender
//!     send!(jobs, "hello from job {id}");
//! });
//!
//! for msg in jobs {                           // ends when all 4 are done
//!     println!("{msg}");
//! }
//! ```
//!
//! ## Syntax cheat sheet
//!
//! | You write | Means |
//! |---|---|
//! | `chan!(name: T)` | unbounded channel of `T` |
//! | `chan!(name: T, bounded 64)` | channel that holds at most 64 (backpressure) |
//! | `send!(name, value)` / `send!(name, "fmt {x}")` | send a value / a formatted `String` |
//! | `for x in name { }` | receive until every sender is done |
//! | `shared!(name = value)` | data many threads can change (`Arc<Mutex<T>>`) |
//! | `lock!(name => g { ... })` | lock, use `g`, unlock at `}` |
//! | `lock!(a, b => ga, gb { ... })` | lock two, always in a safe order (no ABBA deadlock) |
//! | `read_mostly!(name = value)` | data read often, written rarely (`Arc<RwLock<T>>`) |
//! | `spawn!(n, [a, b], \|id\| { ... })` | `n` threads; each gets its own handle to `a`, `b` |
//! | `spawn!([a], { ... })` | one thread |
//! | `select! { recv(a) -> x => .., recv(b) -> y => .., after(d) => .. }` | wait for whichever comes first |
//!
//! `spawn!` returns a [`Crew`]. Keep it (`let crew = spawn!(..)`) to run in
//! the background; call `crew.wait()` for the results. If you don't keep it,
//! the statement waits right there for all threads (fork-join). Either way
//! threads are always joined, and a panic in a thread is never silently lost.
//!
//! ## Mistakes that become compile errors
//!
//! A worker can't receive from a captured channel (it only got a Sender):
//! ```compile_fail
//! use strand::*;
//! chan!(jobs: u32);
//! spawn!(2, [jobs], |_id| {
//!     for j in jobs { println!("{j}"); }   // ERROR: Sender is not iterable
//! });
//! ```
//!
//! Capturing something that isn't thread-safe gives a friendly message:
//! ```compile_fail
//! use strand::*;
//! let names = vec!["a", "b"];
//! spawn!(2, [names], |id| { println!("{}", names[id]); }); // ERROR: wrap it
//! ```
//!
//! `Arc` is read-only; you need a lock to change shared data:
//! ```compile_fail
//! use strand::*;
//! let a = Arc::new(5);
//! *a += 1; // ERROR: cannot assign through `Arc`
//! ```
//!
//! A `MutexGuard` can't be sent to another thread:
//! ```compile_fail
//! use strand::*;
//! let m = Mutex::new(1);
//! std::thread::scope(|s| {
//!     let g = m.lock();
//!     s.spawn(move || drop(g)); // ERROR: MutexGuard is not Send
//! });
//! ```
//!
//! A `Receiver` can't be shared between two consumers:
//! ```compile_fail
//! use strand::*;
//! let (_tx, rx) = channel::<u32>();
//! std::thread::scope(|s| {
//!     s.spawn(|| rx.recv());
//!     s.spawn(|| rx.recv()); // ERROR: Receiver is not Sync
//! });
//! ```

pub mod arc;
pub mod broadcast;
pub mod channel;
pub mod mutex;
pub mod oneshot;
pub mod pool;
pub mod rwlock;
pub mod select;
pub mod semaphore;
pub mod sharded;

pub use arc::{Arc, Weak};
pub use broadcast::Broadcast;
pub use channel::{
    bounded, channel, Chan, Receiver, RecvError, RecvTimeoutError, SendError, Sender, TrySendError,
};
pub use mutex::{Condvar, Mutex, MutexGuard};
pub use oneshot::{oneshot, OneshotReceiver, OneshotSender};
pub use pool::Pool;
pub use rwlock::{ReadGuard, RwLock, WriteGuard};
pub use select::{Selectable, Stop, Ticker, Waitable};
pub use semaphore::{Permit, Semaphore};
pub use sharded::Sharded;

use std::thread::JoinHandle;

// ───────────────────────────── Shared / ReadMostly ──────────────────────────

/// Data that many threads CHANGE. Just `Arc<Mutex<T>>` with a nicer name.
/// Create with `shared!(name = value)`, use with `lock!(name => g { .. })`.
pub struct Shared<T>(Arc<Mutex<T>>);

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Shared(Arc::new(Mutex::new(value)))
    }
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock()
    }
    fn addr(&self) -> usize {
        &*self.0 as *const Mutex<T> as usize
    }
    /// Lock two values in a fixed global order (by memory address).
    ///
    /// Why: if thread 1 locks A then B while thread 2 locks B then A, each
    /// waits for the other forever (ABBA deadlock). Sorting by address makes
    /// every thread use the same order, so that cycle can't happen.
    pub fn lock_two<'a, U>(
        a: &'a Shared<T>,
        b: &'a Shared<U>,
    ) -> (MutexGuard<'a, T>, MutexGuard<'a, U>) {
        if a.addr() == b.addr() {
            panic!("strand: lock!(a, b) on the same value twice would deadlock");
        }
        if a.addr() < b.addr() {
            let ga = a.lock();
            (ga, b.lock())
        } else {
            let gb = b.lock();
            (a.lock(), gb)
        }
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}

/// Data read by many threads, written rarely. `Arc<RwLock<T>>`.
pub struct ReadMostly<T>(Arc<RwLock<T>>);

impl<T> ReadMostly<T> {
    pub fn new(value: T) -> Self {
        ReadMostly(Arc::new(RwLock::new(value)))
    }
    pub fn read(&self) -> ReadGuard<'_, T> {
        self.0.read()
    }
    pub fn write(&self) -> WriteGuard<'_, T> {
        self.0.write()
    }
}

impl<T> Clone for ReadMostly<T> {
    fn clone(&self) -> Self {
        ReadMostly(self.0.clone())
    }
}

// ───────────────────────────── Share: what spawn! may capture ───────────────

/// Things `spawn!` is allowed to hand to a thread. `share()` makes the
/// per-thread copy (a cheap pointer clone, never a deep copy).
#[diagnostic::on_unimplemented(
    message = "`{Self}` can't be captured by `spawn!`: it isn't a thread-safe handle",
    label = "this value would be moved into the first thread, leaving nothing for the others",
    note = "pick the tool that matches what threads do with it:\n  \
            * threads CHANGE it     -> `shared!(name = value)`\n  \
            * threads only READ it  -> `let name = Arc::new(value);`\n  \
            * threads PRODUCE items -> `chan!(name: T)` and `send!`\n  \
            * each thread needs its own copy -> clone it inside the closure body"
)]
pub trait Share {
    type Handle: Send + 'static;
    fn share(&self) -> Self::Handle;
}

impl<T: Send + 'static> Share for Chan<T> {
    type Handle = Sender<T>; // threads get the sending end only
    fn share(&self) -> Sender<T> {
        self.sender()
    }
}
impl<T: Send + 'static> Share for Sender<T> {
    type Handle = Sender<T>;
    fn share(&self) -> Sender<T> {
        self.clone()
    }
}
impl<T: Send + 'static> Share for Shared<T> {
    type Handle = Shared<T>;
    fn share(&self) -> Shared<T> {
        self.clone()
    }
}
impl<T: Send + Sync + 'static> Share for ReadMostly<T> {
    type Handle = ReadMostly<T>;
    fn share(&self) -> ReadMostly<T> {
        self.clone()
    }
}
impl<T: Send + Sync + 'static> Share for Arc<T> {
    type Handle = Arc<T>;
    fn share(&self) -> Arc<T> {
        self.clone()
    }
}
impl Share for Stop {
    type Handle = Stop;
    fn share(&self) -> Stop {
        self.clone()
    }
}
impl<T: Clone + Send + 'static> Share for Broadcast<T> {
    type Handle = Broadcast<T>;
    fn share(&self) -> Broadcast<T> {
        self.clone()
    }
}
impl<K: Send + 'static, V: Send + 'static> Share for Sharded<K, V> {
    type Handle = Sharded<K, V>;
    fn share(&self) -> Sharded<K, V> {
        self.clone()
    }
}
macro_rules! share_copy {
    ($($t:ty),*) => {$(
        impl Share for $t {
            type Handle = $t;
            fn share(&self) -> $t { *self }
        }
    )*};
}
share_copy!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
            bool, char, &'static str, std::time::Duration);

// ───────────────────────────── Crew: joined threads ─────────────────────────

/// The threads started by one `spawn!`. Always joined: by [`Crew::wait`], or
/// automatically when the Crew is dropped. A panic inside a thread is
/// re-raised here instead of vanishing (a plain `thread::spawn` whose handle
/// you ignore swallows panics silently).
pub struct Crew<R> {
    handles: Vec<JoinHandle<R>>,
}

impl<R: Send + 'static> Crew<R> {
    #[doc(hidden)]
    pub fn new() -> Self {
        Crew { handles: Vec::new() }
    }

    #[doc(hidden)]
    pub fn start(&mut self, id: usize, f: impl FnOnce() -> R + Send + 'static) {
        let handle = std::thread::Builder::new()
            .name(format!("strand-{id}")) // shows up in panics and debuggers
            .spawn(f)
            .expect("strand: the OS refused to start a thread");
        self.handles.push(handle);
    }

    /// Wait for all threads; results come back in id order.
    pub fn wait(mut self) -> Vec<R> {
        self.join_all()
    }

    fn join_all(&mut self) -> Vec<R> {
        self.handles
            .drain(..)
            .map(|h| match h.join() {
                Ok(r) => r,
                Err(panic) => std::panic::resume_unwind(panic),
            })
            .collect()
    }
}

impl<R> Drop for Crew<R> {
    fn drop(&mut self) {
        for h in self.handles.drain(..) {
            let result = h.join();
            if let Err(panic) = result {
                if !std::thread::panicking() {
                    std::panic::resume_unwind(panic);
                }
            }
        }
    }
}

#[doc(hidden)]
pub trait SendTo<T> {
    fn send_to(&self, value: T);
}
impl<T> SendTo<T> for Chan<T> {
    fn send_to(&self, value: T) {
        if self.send(value).is_err() {
            panic!("strand: send! failed, the receiving side of this channel was dropped");
        }
    }
}
impl<T> SendTo<T> for Sender<T> {
    fn send_to(&self, value: T) {
        if self.send(value).is_err() {
            panic!("strand: send! failed, the receiving side of this channel was dropped");
        }
    }
}

// ───────────────────────────── The syntax ───────────────────────────────────

/// `chan!(name: T)` — make a channel called `name` carrying `T`.
///
/// `chan!(name: T, bounded N)` — at most `N` queued; senders wait when full.
/// Prefer bounded in servers: it stops a fast producer from eating all RAM.
#[macro_export]
macro_rules! chan {
    ($name:ident : $t:ty) => {
        let $name = $crate::Chan::<$t>::new();
    };
    ($name:ident : $t:ty, bounded $cap:expr) => {
        let $name = $crate::Chan::<$t>::bounded($cap);
    };
}

/// `send!(ch, value)` or `send!(ch, "text {x}")`.
///
/// A string literal is treated as a format string and produces a `String`.
/// Panics if the receiver is gone (use `ch.send(v)` to get a `Result`).
/// To send a number literal, write `send!(ch, (5))` or use a variable.
#[macro_export]
macro_rules! send {
    ($ch:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::SendTo::send_to(&$ch, ::std::format!($fmt $(, $arg)*))
    };
    ($ch:expr, $value:expr) => {
        $crate::SendTo::send_to(&$ch, $value)
    };
}

/// `shared!(name = value)` — data that several threads change.
#[macro_export]
macro_rules! shared {
    ($name:ident = $value:expr) => {
        let $name = $crate::Shared::new($value);
    };
}

/// `read_mostly!(name = value)` — data read constantly, written rarely.
/// Use `name.read()` and `name.write()`.
#[macro_export]
macro_rules! read_mostly {
    ($name:ident = $value:expr) => {
        let $name = $crate::ReadMostly::new($value);
    };
}

/// `lock!(name => g { ... })` — lock, work with `g`, unlock at the `}`.
///
/// `lock!(a, b => ga, gb { ... })` — lock two in a deadlock-free order.
///
/// Keep the block short. Never block inside it (no sleeping, no network, no
/// `send!` on a bounded channel, no locking the same value again).
#[macro_export]
macro_rules! lock {
    ($a:expr, $b:expr => $ga:ident, $gb:ident $body:block) => {{
        #[allow(unused_mut)]
        let (mut $ga, mut $gb) = $crate::Shared::lock_two(&$a, &$b);
        $body
    }};
    ($name:expr => $g:ident $body:block) => {{
        #[allow(unused_mut)]
        let mut $g = $name.lock();
        $body
    }};
}

/// Start threads.
///
/// * `spawn!(n, [a, b], |id| { .. })` — `n` threads, `id` = 0..n
/// * `spawn!([a, b], { .. })` — one thread
/// * `spawn!(n, |id| { .. })` / `spawn!({ .. })` — nothing captured
///
/// Everything in `[ .. ]` is handed to each thread with [`Share::share`]:
/// a `Chan` becomes a `Sender`, a `Shared`/`Arc` is pointer-cloned.
/// This replaces writing `let x = x.clone();` + `move ||` by hand.
#[macro_export]
macro_rules! spawn {
    ($n:expr, [$($cap:ident),* $(,)?], |$id:pat_param| $body:block) => {{
        let mut __crew = $crate::Crew::new();
        for __i in 0..($n as usize) {
            $( let $cap = $crate::Share::share(&$cap); )*
            __crew.start(__i, move || {
                #[allow(unused_variables)]
                let $id = __i;
                $body
            });
        }
        __crew
    }};
    ($n:expr, |$id:pat_param| $body:block) => {
        $crate::spawn!($n, [], |$id| $body)
    };
    ([$($cap:ident),* $(,)?], $body:block) => {
        $crate::spawn!(1, [$($cap),*], |_| $body)
    };
    ($body:block) => {
        $crate::spawn!(1, [], |_| $body)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_jobs() {
        chan!(jobs: String);
        let _crew = spawn!(4, [jobs], |id| {
            send!(jobs, "hello from job {id}");
        });
        let mut got: Vec<String> = jobs.into_iter().collect();
        got.sort();
        assert_eq!(got.len(), 4);
        assert_eq!(got[0], "hello from job 0");
    }

    #[test]
    fn syntax_counter() {
        shared!(counter = 0);
        spawn!(8, [counter], |_id| {
            for _ in 0..1000 {
                lock!(counter => c { *c += 1 });
            }
        }); // not kept -> waits here
        assert_eq!(*counter.lock(), 8000);
    }

    #[test]
    fn syntax_two_locks_no_deadlock() {
        shared!(a = 0);
        shared!(b = 0);
        let x = spawn!(4, [a, b], |_| {
            for _ in 0..10_000 {
                lock!(a, b => ga, gb { *ga += 1; *gb += 1; });
            }
        });
        let y = spawn!(4, [a, b], |_| {
            for _ in 0..10_000 {
                lock!(b, a => gb, ga { *gb += 1; *ga += 1; }); // opposite order: still safe
            }
        });
        x.wait();
        y.wait();
        assert_eq!((*a.lock(), *b.lock()), (80_000, 80_000));
    }

    #[test]
    fn crew_returns_results() {
        let crew = spawn!(3, |id| { id * 10 });
        assert_eq!(crew.wait(), vec![0, 10, 20]);
    }

    #[test]
    fn oneshot_and_pool() {
        let pool = Pool::new(3, 8);
        let answers: Vec<_> = (0..10u64).map(|i| pool.run(move || i * i)).collect();
        let total: u64 = answers.into_iter().map(|a| a.recv().unwrap()).sum();
        assert_eq!(total, 285);
        // A panicking job doesn't kill the pool; its asker just gets Err.
        let bad = pool.run(|| -> u32 { panic!("job failed") });
        assert!(bad.recv().is_err());
        assert_eq!(pool.run(|| 7).recv(), Ok(7));
    }

    #[test]
    fn semaphore_limits_concurrency() {
        use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
        let sem = Arc::new(Semaphore::new(3));
        let inside = Arc::new(AtomicUsize::new(0));
        let max_seen = Arc::new(AtomicUsize::new(0));
        spawn!(10, [sem, inside, max_seen], |_| {
            let _permit = sem.acquire();
            let now = inside.fetch_add(1, SeqCst) + 1;
            max_seen.fetch_max(now, SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(5));
            inside.fetch_sub(1, SeqCst);
        });
        assert!(max_seen.load(SeqCst) <= 3);
        assert_eq!(sem.available(), 3);
    }

    #[test]
    fn broadcast_everyone_gets_it_slow_ones_lag() {
        let news = Broadcast::new(2);
        let fast = news.subscribe();
        let slow = news.subscribe();
        for i in 0..5 {
            news.publish(i);
            fast.try_recv();
        }
        assert_eq!(slow.drain_ready(), vec![0, 1]); // missed 2, 3, 4
        assert_eq!(news.lagged(), 3);
        drop(fast);
        news.publish(9);
        assert_eq!(news.subscriber_count(), 1);
    }

    #[test]
    fn sharded_map() {
        let accounts: Sharded<u32, u64> = Sharded::new(8);
        spawn!(4, [accounts], |t| {
            for k in 0..100u32 {
                *accounts.shard(&k).entry(k).or_insert(0) += t as u64 + 1;
            }
        });
        assert_eq!(accounts.len(), 100);
        assert_eq!(accounts.shard(&42)[&42], 1 + 2 + 3 + 4);
    }

    #[test]
    #[should_panic(expected = "boom")]
    fn thread_panic_is_not_lost() {
        spawn!({ panic!("boom") });
    }
}
