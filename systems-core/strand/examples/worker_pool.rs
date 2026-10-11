//! Worker pool: 1 producer -> bounded queue -> 4 workers -> results.
//! Run: cargo run --example worker_pool
//!
//! Patterns shown:
//! * bounded channel = backpressure (producer can't run ahead by more than 8)
//! * many consumers on one queue = put the Receiver inside `shared!`
//!   (mpsc has ONE consumer; the Mutex takes turns handing it out)
//! * a second channel to collect results
use strand::*;

fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn main() {
    chan!(jobs: u64, bounded 8);
    chan!(results: (usize, u64, bool));

    // Producer thread: gets a Sender for `jobs`.
    let _producer = spawn!([jobs], {
        for n in 1_000_000..1_000_040 {
            send!(jobs, n); // waits whenever 8 jobs are already queued
        }
    });

    // Turn the rest of `jobs` into a Receiver the workers can share.
    // (This also drops our own Sender, so the queue closes when the producer ends.)
    shared!(queue = jobs.into_receiver());

    let _workers = spawn!(4, [queue, results], |worker| {
        loop {
            // Hold the lock only while taking ONE job, never while working.
            let job = lock!(queue => q { q.recv() });
            let Ok(n) = job else { break }; // queue closed and empty: stop
            send!(results, (worker, n, is_prime(n)));
        }
    });

    let mut primes = Vec::new();
    for (worker, n, prime) in results {
        if prime {
            println!("worker {worker} found prime {n}");
            primes.push(n);
        }
    }
    println!("{} primes", primes.len());
}
