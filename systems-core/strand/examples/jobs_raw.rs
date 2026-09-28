//! The same program as jobs.rs, written by hand with the crate's own
//! primitives. This is roughly what the macros expand to.
//! Run: cargo run --example jobs_raw
use strand::channel;

fn main() {
    let (tx, rx) = channel::<String>();             // chan!(jobs: String)
    let mut handles = Vec::new();

    for id in 0..4 {                                  // spawn!(4, [jobs], |id| ...)
        let tx = tx.clone();                          //   Share::share: one Sender per thread
        handles.push(std::thread::spawn(move || {     //   `move`: the thread owns tx and id
            tx.send(format!("hello from job {id}"))  //   send!(jobs, "...")
                .expect("receiver dropped");
        }));
    }

    drop(tx);                                         // `for msg in jobs` does this for you

    for msg in rx {                                   // for msg in jobs
        println!("{msg}");
    }

    for h in handles {                                // Crew's Drop does this
        h.join().unwrap();                            // and re-raises panics
    }
}
