//! # Pool — a fixed team of worker threads
//!
//! **Like you're five:** a kitchen with 4 cooks and a ticket rail. You don't
//! hire a new cook for every order (starting a thread costs ~10-50 µs and
//! memory for its stack); you pin the ticket on the rail and a free cook
//! takes it.
//!
//! **Use it for:** CPU work arriving all the time: signature verification,
//! hashing, compressing, (de)serializing.
//!
//! What it teaches:
//! * a bounded job queue (the rail) gives backpressure to whoever submits;
//! * `run` returns a oneshot, so you can collect the answer later;
//! * one job panicking must NOT kill the cook: we catch it, and the asker
//!   sees "no answer";
//! * graceful shutdown: dropping the Pool closes the queue, cooks finish the
//!   tickets already on the rail, then everyone is joined.

use crate::channel::{bounded, Sender};
use crate::oneshot::{oneshot, OneshotReceiver};
use crate::{Arc, Mutex};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Pool {
    queue: Option<Sender<Job>>,
    workers: Vec<JoinHandle<()>>,
}

impl Pool {
    /// `threads` cooks; at most `queue_size` tickets waiting on the rail.
    pub fn new(threads: usize, queue_size: usize) -> Self {
        let (tx, rx) = bounded::<Job>(queue_size);
        let rx = Arc::new(Mutex::new(rx)); // many cooks share one rail
        let workers = (0..threads)
            .map(|i| {
                let rx = rx.clone();
                std::thread::Builder::new()
                    .name(format!("pool-{i}"))
                    .spawn(move || loop {
                        let job = rx.lock().recv(); // lock only to TAKE a ticket
                        match job {
                            Ok(job) => {
                                let _ = catch_unwind(AssertUnwindSafe(job)); // survive panics
                            }
                            Err(_) => break, // rail closed and empty: go home
                        }
                    })
                    .expect("failed to start worker")
            })
            .collect();
        Pool { queue: Some(tx), workers }
    }

    /// Run `f` on some worker. Returns where the answer will arrive.
    /// Waits if the rail is full (backpressure).
    pub fn run<R, F>(&self, f: F) -> OneshotReceiver<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (reply, answer) = oneshot();
        let job: Job = Box::new(move || {
            let _ = reply.send(f()); // if f panics, `reply` is dropped: asker gets Err
        });
        if self.queue.as_ref().unwrap().send(job).is_err() {
            panic!("pool is shut down");
        }
        answer
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        drop(self.queue.take()); // close the rail
        for w in self.workers.drain(..) {
            let _ = w.join(); // wait for everyone to finish their last ticket
        }
    }
}
