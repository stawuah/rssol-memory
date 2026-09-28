//! Your first example: 4 jobs send "hello" through a channel.
//! Run: cargo run --example jobs
//! Compare with examples/jobs_raw.rs, which is what this expands to.
use strand::*;

fn main() {
    chan!(jobs: String); // a channel named `jobs` carrying Strings

    // 4 threads. `[jobs]` = each thread gets its own Sender for `jobs`.
    // Keeping the Crew in `_crew` lets the threads run while we read below.
    let _crew = spawn!(4, [jobs], |id| {
        send!(jobs, "hello from job {id}");
    });

    // Ends by itself once all 4 threads are done sending.
    for msg in jobs {
        println!("{msg}");
    }
} // `_crew` dropped here: every thread is joined
