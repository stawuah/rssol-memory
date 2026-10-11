//! POOL + SEMAPHORE: reuse a fixed set of threads, and cap how many things
//! happen at once.
//! Run: cargo run --release --example limits
use std::time::Duration;
use strand::*;

fn verify(sig: u64) -> bool {
    let mut h = sig;
    for _ in 0..50_000 {
        h = h.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    }
    h % 7 != 0
}

fn main() {
    // 1) A pool of 4 threads verifies 200 "signatures". 200 jobs, 4 threads.
    let pool = Pool::new(4, 16);
    let answers: Vec<_> = (0..200u64).map(|sig| pool.run(move || verify(sig))).collect();
    let valid = answers.into_iter().map(|a| a.recv()).filter(|r| *r == Ok(true)).count();
    println!("pool: {valid}/200 signatures valid");

    // A job that panics doesn't take a worker down with it.
    // (Quiet panic printing, so the demo output stays readable.)
    std::panic::set_hook(Box::new(|info| eprintln!("  [a job panicked: {}]", info.payload().downcast_ref::<&str>().unwrap_or(&"?"))));
    let crash = pool.run(|| -> bool { panic!("malformed packet") });
    println!("pool: crashing job answered with {:?}; pool still works: {:?}", crash.recv(), pool.run(|| 1 + 1).recv());
    drop(pool); // graceful shutdown: finish queued jobs, join all workers

    // 2) At most 3 "peer connections" at once. 8 peers try to connect.
    //    try_acquire = reject when full ("busy, try later") instead of queueing.
    let slots = Arc::new(Semaphore::new(3));
    let outcomes = spawn!(8, [slots], |peer| {
        match slots.try_acquire() {
            Some(_permit) => {
                std::thread::sleep(Duration::from_millis(30)); // serve the peer
                format!("peer {peer}: served")
            } // `_permit` dropped here -> slot free again
            None => format!("peer {peer}: rejected (busy)"),
        }
    })
    .wait();
    for o in outcomes {
        println!("{o}");
    }
    println!("free connection slots after: {}", slots.available());
}
