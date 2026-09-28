//! A TINY ASYNC RUNTIME in ~80 lines, to see what tokio does underneath.
//! Run: cargo run --example tiny_async
//!
//! Like you're five: threads are like hiring one worker per customer.
//! async is ONE worker with a notepad: "customer A is waiting for the oven,
//! I'll help B, then C, and come back to A when the oven dings".
//!
//! * A `Future` is a paused task. `poll()` = "can you make progress?"
//!   It answers `Ready(value)` or `Pending`.
//! * If Pending, the future keeps the `Waker` and calls `wake()` later
//!   ("the oven dinged").
//! * The executor runs `poll` again only after a wake. Our Waker is just
//!   `thread.unpark()`, the same trick the channels in this crate use.
//!
//! Real runtimes (tokio) add: many tasks per thread, a pool of threads,
//! one epoll/kqueue for all sockets, a timer wheel instead of a thread per
//! sleep. The contract (poll / Pending / Waker) is exactly this.
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

// ── the executor ──
struct ThreadWaker(Thread);
impl Wake for ThreadWaker {
    fn wake(self: std::sync::Arc<Self>) {
        self.0.unpark(); // "come back and poll me again"
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut); // futures must not move once polled (self-references)
    let waker = Waker::from(std::sync::Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => thread::park(), // sleep until some wake()
        }
    }
}

// ── a leaf future: sleep ──
struct Sleep {
    until: Instant,
    timer_started: bool,
}
fn sleep(d: Duration) -> Sleep {
    Sleep { until: Instant::now() + d, timer_started: false }
}
impl Future for Sleep {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.until {
            return Poll::Ready(());
        }
        if !self.timer_started {
            self.timer_started = true;
            let (waker, until) = (cx.waker().clone(), self.until);
            thread::spawn(move || {
                thread::sleep(until.saturating_duration_since(Instant::now()));
                waker.wake(); // the oven dings
            });
        }
        Poll::Pending
    }
}

// ── a combinator: run two futures CONCURRENTLY on ONE thread ──
struct Join2<A: Future, B: Future> {
    a: Pin<Box<A>>,
    b: Pin<Box<B>>,
    ra: Option<A::Output>,
    rb: Option<B::Output>,
}
fn join2<A: Future, B: Future>(a: A, b: B) -> Join2<A, B> {
    Join2 { a: Box::pin(a), b: Box::pin(b), ra: None, rb: None }
}
impl<A: Future, B: Future> Future for Join2<A, B>
where
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        if this.ra.is_none() {
            if let Poll::Ready(v) = this.a.as_mut().poll(cx) {
                this.ra = Some(v);
            }
        }
        if this.rb.is_none() {
            if let Poll::Ready(v) = this.b.as_mut().poll(cx) {
                this.rb = Some(v);
            }
        }
        if this.ra.is_some() && this.rb.is_some() {
            Poll::Ready((this.ra.take().unwrap(), this.rb.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

// ── normal async code on top ──
async fn fetch_block(slot: u64, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // pretend network I/O
    format!("block {slot}")
}

fn main() {
    let t0 = Instant::now();
    let (a, b) = block_on(join2(fetch_block(100, 300), fetch_block(101, 300)));
    println!("{a} and {b} in {:?} on ONE thread (not 600 ms: they waited together)", t0.elapsed());

    let t0 = Instant::now();
    let seq = block_on(async {
        let a = fetch_block(200, 150).await; // .await one after another = sequential
        let b = fetch_block(201, 150).await;
        (a, b)
    });
    println!("{seq:?} in {:?} (sequential awaits add up)", t0.elapsed());
}
