//! Shared counter two ways: Mutex vs a single atomic.
//! Run: cargo run --release --example counter
//!
//! Lesson: a Mutex protects ANY data (maps, vecs, structs). For ONE number,
//! an atomic does the same job without a lock and is usually much faster.
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;
use strand::*;

fn main() {
    const THREADS: usize = 8;
    const EACH: u64 = 200_000;

    // 1) Mutex: correct for any data type.
    shared!(counter = 0u64);
    let t = Instant::now();
    spawn!(THREADS, [counter], |_id| {
        for _ in 0..EACH {
            lock!(counter => c { *c += 1 });
        }
    }); // not kept: waits here for all threads
    println!("mutex : {} in {:?}", *counter.lock(), t.elapsed());

    // 2) Atomic: one hardware instruction, no lock. Arc because threads share it.
    let hits = Arc::new(AtomicU64::new(0));
    let t = Instant::now();
    spawn!(THREADS, [hits], |_id| {
        for _ in 0..EACH {
            hits.fetch_add(1, Relaxed); // Relaxed is fine: we only need the total
        }
    });
    println!("atomic: {} in {:?}", hits.load(Relaxed), t.elapsed());
}
