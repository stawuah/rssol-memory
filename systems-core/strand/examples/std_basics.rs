//! Things Rust's standard library already gives you that you should know
//! (no need to rebuild them). Run: cargo run --example std_basics
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::*};
use std::sync::{Barrier, OnceLock};
use std::cell::Cell;
use std::thread;

// OnceLock: initialize a global exactly once, even if 8 threads race to do it.
static GENESIS_HASH: OnceLock<String> = OnceLock::new();

// thread_local!: every thread gets its OWN copy. No sharing, no locks.
thread_local! {
    static SCRATCH_COUNT: Cell<u32> = const { Cell::new(0) };
}

fn main() {
    // 1) SCOPED THREADS: borrow local data without Arc or move.
    //    The scope guarantees every thread finishes before `scope` returns,
    //    so borrowing `accounts` is safe (the 'static rule is lifted).
    let accounts = vec![100u64, 250, 75, 900];
    let total = AtomicU64::new(0);
    thread::scope(|s| {
        for chunk in accounts.chunks(2) {
            s.spawn(|| {
                total.fetch_add(chunk.iter().sum::<u64>(), Relaxed);
            });
        }
    });
    println!("1. scoped threads summed {} (borrowed, no Arc)", total.load(Relaxed));

    // 2) ONCELOCK
    thread::scope(|s| {
        for i in 0..8 {
            s.spawn(move || {
                GENESIS_HASH.get_or_init(|| format!("computed once, by thread {i}"));
            });
        }
    });
    println!("2. OnceLock: {}", GENESIS_HASH.get().unwrap());

    // 3) THREAD_LOCAL
    thread::scope(|s| {
        for _ in 0..3 {
            s.spawn(|| {
                for _ in 0..5 {
                    SCRATCH_COUNT.with(|c| c.set(c.get() + 1));
                }
                assert_eq!(SCRATCH_COUNT.with(|c| c.get()), 5); // 5, not 15: own copy
            });
        }
    });
    println!("3. thread_local: each thread counted to 5 on its own copy");

    // 4) BARRIER: nobody continues until all have arrived (start a benchmark
    //    or a simulation round at the same moment).
    let barrier = Barrier::new(4);
    thread::scope(|s| {
        for i in 0..4 {
            let barrier = &barrier;
            s.spawn(move || {
                thread::sleep(std::time::Duration::from_millis(i * 10)); // arrive at different times
                barrier.wait(); // ...leave together
            });
        }
    });
    println!("4. Barrier: 4 threads released together");

    // 5) RELEASE/ACQUIRE hand-off: publish data, then raise a flag.
    //    Release on the flag = "everything I wrote before is visible to whoever
    //    Acquires this flag". With Relaxed, the reader could see ready=true but
    //    an OLD value of DATA (on ARM; x86 happens to hide the bug).
    static DATA: AtomicU64 = AtomicU64::new(0);
    static READY: AtomicBool = AtomicBool::new(false);
    thread::scope(|s| {
        s.spawn(|| {
            DATA.store(42, Relaxed);
            READY.store(true, Release);
        });
        s.spawn(|| {
            while !READY.load(Acquire) {
                std::hint::spin_loop();
            }
            assert_eq!(DATA.load(Relaxed), 42); // guaranteed by Release/Acquire
        });
    });
    println!("5. Release/Acquire: reader saw the data the flag promised");

    // 6) COMPARE-AND-SWAP loop: update only if nobody changed it meanwhile.
    //    Here: keep the highest slot number seen by any thread.
    let highest = AtomicU64::new(0);
    thread::scope(|s| {
        for slot in [17u64, 42, 8, 33] {
            let highest = &highest;
            s.spawn(move || {
                let mut current = highest.load(Relaxed);
                while slot > current {
                    match highest.compare_exchange_weak(current, slot, Relaxed, Relaxed) {
                        Ok(_) => break,
                        Err(actual) => current = actual, // someone beat us: re-check
                    }
                }
            });
        }
    });
    println!("6. CAS loop: highest slot = {}", highest.load(Relaxed));
}
