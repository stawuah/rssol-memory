//! # `select!`, `Stop`, `Ticker` — waiting for whichever comes first
//!
//! **Like you're five:** you're waiting for your friend, the pizza, OR the
//! alarm, whichever comes first. You don't stand at the door, then at the
//! oven, then at the clock. You sit down and whoever arrives rings the bell.
//!
//! ```
//! use strand::*;
//! use std::time::Duration;
//!
//! let (tx, orders) = channel::<u32>();
//! let stop = Stop::new();
//! let mut heartbeat = Ticker::every(Duration::from_millis(10));
//! tx.send(7).unwrap();
//!
//! let mut beats = 0;
//! loop {
//!     select! {
//!         recv(orders) -> order => {           // order: Option<u32>, None = closed
//!             match order {
//!                 Some(o) => println!("order {o}"),
//!                 None => break,
//!             }
//!         },
//!         recv(heartbeat) -> _ => {            // fires every 10 ms
//!             beats += 1;
//!             if beats == 3 { stop.fire(); }
//!         },
//!         recv(stop) -> _ => break,            // someone said "stop"
//!         after(Duration::from_secs(1)) => {   // nothing at all for 1 s
//!             println!("idle");
//!         },
//!     }
//! }
//! ```
//!
//! **Use it for:** a node's main loop (network messages + timer + shutdown),
//! request deadlines, heartbeats and leader-election timeouts, "flush every
//! 10 ms OR when 100 items are waiting".
//!
//! **How it works:** every source implements [`Waitable`]:
//! 1. `register()`: "if something happens, unpark THIS thread".
//! 2. `is_ready()`: "is there something right now?"
//! 3. If nobody is ready: `park()` (or `park_timeout` until the nearest
//!    deadline) and go back to step 1.
//!
//! Register first, then check. The other order has a gap where a message can
//! arrive between the check and the register and nobody wakes us.
//!
//! **Fairness:** if arm 1 is always busy, a naive select always picks it and
//! arm 2 starves. We rotate which arm is checked first on every call.

use crate::{Arc, Mutex};
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering::*};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

/// Something `select!` can wait on.
#[diagnostic::on_unimplemented(
    message = "`select!` can't wait on `{Self}`",
    note = "select! works with a `Receiver`, `OneshotReceiver`, `Stop` or `Ticker`.\n\
            For a `Chan`, call `.into_receiver()` first: that drops the Chan's own \
            Sender, so the channel can actually close"
)]
pub trait Waitable {
    /// True if `take()` would not wait (an item is there, or it's closed).
    fn is_ready(&self) -> bool;
    /// Arrange for the current thread to be unparked when this becomes ready.
    fn register(&self);
    /// For time-based sources: when will this become ready by itself?
    fn deadline(&self) -> Option<Instant> {
        None
    }
}

/// A `Waitable` that produces a value. `None` means "closed, forever".
pub trait Selectable: Waitable {
    type Item;
    fn take(&self) -> Option<Self::Item>;
}

thread_local! {
    static ROTATE: Cell<usize> = const { Cell::new(0) };
}

/// Blocks until one source is ready. Returns its index, or `None` if
/// `deadline` passed first. Used by `select!`.
pub fn wait_any(sources: &[&dyn Waitable], deadline: Option<Instant>) -> Option<usize> {
    let n = sources.len();
    let start = ROTATE.with(|r| {
        let s = r.get();
        r.set(s.wrapping_add(1));
        s
    });
    loop {
        for s in sources {
            s.register();
        }
        for k in 0..n {
            let i = (start + k) % n;
            if sources[i].is_ready() {
                return Some(i);
            }
        }
        // Sleep until the nearest deadline (ours or a Ticker's), or until unparked.
        let nearest = sources.iter().filter_map(|s| s.deadline()).chain(deadline).min();
        match nearest {
            None => thread::park(),
            Some(d) => {
                let now = Instant::now();
                if now < d {
                    thread::park_timeout(d - now);
                }
            }
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            // One last look: something may have arrived at the same moment.
            for k in 0..n {
                let i = (start + k) % n;
                if sources[i].is_ready() {
                    return Some(i);
                }
            }
            return None;
        }
    }
}

/// Wait on several sources at once. See module docs.
///
/// ```text
/// select! {
///     recv(source) -> pattern => { body },    // pattern gets Option<Item>
///     ...
///     after(duration) => { body },            // optional: nothing came in time
/// }
/// ```
/// Arms are separated by commas (also after `{ }` bodies).
///
/// Inside a body, `break` and `continue` act on YOUR surrounding loop.
#[macro_export]
macro_rules! select {
    (
        $( recv($src:expr) -> $pat:pat => $body:expr ),+
        $( , after($dur:expr) => $tbody:expr )?
        $(,)?
    ) => {{
        let __deadline: ::std::option::Option<::std::time::Instant> =
            ::std::option::Option::None $( .or(::std::option::Option::Some(::std::time::Instant::now() + $dur)) )?;
        let __which = $crate::select::wait_any(
            &[ $( &$src as &dyn $crate::select::Waitable ),+ ],
            __deadline,
        );
        #[allow(unused_mut, unused_assignments)]
        let mut __i = 0usize;
        $(
            if __which == ::std::option::Option::Some(__i) {
                let $pat = $crate::select::Selectable::take(&$src);
                $body;
            }
            __i += 1;
        )+
        let _ = __i;
        $( if __which.is_none() { $tbody; } )?
    }};
}

// ───────────────────────────── Stop ─────────────────────────────────────────

/// A "please stop" signal many threads can watch. Fire it once; it stays
/// fired. Clone it freely (it's an `Arc` inside).
///
/// **Like you're five:** the bell at the end of break time. Everybody hears
/// it, finishes what they're holding, and goes back to class.
///
/// **Why not just kill threads?** Rust (and every sane system) can't safely
/// kill a thread from outside: it might be halfway through writing a file or
/// holding a lock. So threads CHECK for the signal at safe points and exit
/// cleanly. That's called cooperative cancellation / graceful shutdown.
#[derive(Clone)]
pub struct Stop {
    inner: Arc<StopInner>,
}

struct StopInner {
    fired: AtomicBool,
    waiters: Mutex<Vec<Thread>>,
}

impl Stop {
    pub fn new() -> Self {
        Stop {
            inner: Arc::new(StopInner { fired: AtomicBool::new(false), waiters: Mutex::new(Vec::new()) }),
        }
    }

    /// Ring the bell. Wakes everyone waiting in `select!` or `sleep`.
    pub fn fire(&self) {
        self.inner.fired.store(true, Release);
        for t in self.inner.waiters.lock().drain(..) {
            t.unpark();
        }
    }

    /// Cheap check for loops: `while !stop.is_fired() { ... }`
    pub fn is_fired(&self) -> bool {
        self.inner.fired.load(Acquire)
    }

    /// Sleep for `d`, but wake immediately if the stop fires.
    /// Returns `true` if it was stopped.
    pub fn sleep(&self, d: Duration) -> bool {
        let deadline = Instant::now() + d;
        loop {
            self.register();
            if self.is_fired() {
                return true;
            }
            let now = Instant::now();
            if now >= deadline {
                return false;
            }
            thread::park_timeout(deadline - now);
        }
    }
}

impl Default for Stop {
    fn default() -> Self {
        Self::new()
    }
}

impl Waitable for Stop {
    fn is_ready(&self) -> bool {
        self.is_fired()
    }
    fn register(&self) {
        let me = thread::current();
        let mut w = self.inner.waiters.lock();
        if !w.iter().any(|t| t.id() == me.id()) {
            w.push(me);
        }
    }
}
impl Selectable for Stop {
    type Item = ();
    fn take(&self) -> Option<()> {
        Some(())
    }
}

// ───────────────────────────── Ticker ───────────────────────────────────────

/// Becomes ready every `period`. For heartbeats, periodic flushes, metrics.
///
/// **Like you're five:** a clock that goes "ding" every minute. If you were
/// busy and missed 3 dings, you get ONE ding, not 3 (we skip missed ticks, so
/// a slow loop doesn't get a burst of catch-up ticks).
pub struct Ticker {
    period: Duration,
    next: Cell<Instant>,
}

impl Ticker {
    pub fn every(period: Duration) -> Self {
        Ticker { period, next: Cell::new(Instant::now() + period) }
    }
}

impl Waitable for Ticker {
    fn is_ready(&self) -> bool {
        Instant::now() >= self.next.get()
    }
    fn register(&self) {} // time wakes us: see `deadline`
    fn deadline(&self) -> Option<Instant> {
        Some(self.next.get())
    }
}
impl Selectable for Ticker {
    type Item = Instant;
    fn take(&self) -> Option<Instant> {
        let now = Instant::now();
        let mut next = self.next.get() + self.period;
        if next <= now {
            next = now + self.period; // skip missed ticks
        }
        self.next.set(next);
        Some(now)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::{Duration, Instant};

    #[test]
    fn select_picks_ready_channel() {
        let (_ta, a) = channel::<u32>();
        let (tb, b) = channel::<u32>();
        tb.send(2).unwrap();
        let mut got = 0;
        select! {
            recv(a) -> x => { got = x.unwrap(); },
            recv(b) -> x => { got = x.unwrap() * 10; },
        }
        assert_eq!(got, 20);
    }

    #[test]
    fn select_wakes_on_send_from_other_thread() {
        let (ta, a) = channel::<u32>();
        let (_tb, b) = channel::<u32>();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(20));
            ta.send(1).unwrap();
        });
        let mut which = "";
        select! {
            recv(a) -> _ => { which = "a"; },
            recv(b) -> _ => { which = "b"; },
            after(Duration::from_secs(5)) => { which = "timeout"; },
        }
        assert_eq!(which, "a");
    }

    #[test]
    fn select_timeout_and_stop() {
        let (_t, a) = channel::<u32>();
        let t0 = Instant::now();
        let mut timed_out = false;
        select! {
            recv(a) -> _ => {},
            after(Duration::from_millis(30)) => { timed_out = true; },
        }
        assert!(timed_out && t0.elapsed() >= Duration::from_millis(30));

        let stop = Stop::new();
        let s2 = stop.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            s2.fire();
        });
        let mut stopped = false;
        select! {
            recv(a) -> _ => {},
            recv(stop) -> _ => { stopped = true; },
        }
        assert!(stopped);
    }

    #[test]
    fn ticker_ticks() {
        let t = Ticker::every(Duration::from_millis(5));
        let t0 = Instant::now();
        let mut n = 0;
        while n < 4 {
            select! { recv(t) -> _ => { n += 1; } }
        }
        assert!(t0.elapsed() >= Duration::from_millis(20));
    }

    #[test]
    fn stop_sleep_wakes_early() {
        let stop = Stop::new();
        let s2 = stop.clone();
        let t0 = Instant::now();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            s2.fire();
        });
        assert!(stop.sleep(Duration::from_secs(10)));
        assert!(t0.elapsed() < Duration::from_secs(2));
    }
}
