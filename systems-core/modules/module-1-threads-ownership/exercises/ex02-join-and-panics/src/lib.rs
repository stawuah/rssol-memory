//! ex02 — Owned threads: 'static, Send, join, and panics that must not vanish.

use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobError {
    /// The job panicked. Contains the panic message when it was a string.
    Panicked(String),
}

/// A unit of work that can be moved to another thread.
/// `Send`: may cross to another thread. `'static`: borrows nothing short-lived.
pub type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

/// Turn a panic payload into a readable message.
pub fn panic_message(payload: &(dyn Any + Send)) -> String {
    // Coding map:
    // 1. `panic!("literal")` produces a `&'static str` payload.
    // 2. `panic!("fmt {x}")` produces a `String` payload.
    // 3. Try downcast_ref::<&str>() then downcast_ref::<String>().
    // 4. Anything else -> "unknown panic".
    todo!("downcast the panic payload")
}

/// Run every job on its own thread AT THE SAME TIME, wait for all of them,
/// and return results in the same order as `jobs`. A panicking job becomes
/// `Err(JobError::Panicked(msg))`; it must not crash the caller or hide.
pub fn run_jobs<T: Send + 'static>(jobs: Vec<Job<T>>) -> Vec<Result<T, JobError>> {
    // Coding map:
    // 1. Spawn ALL threads first (collect the JoinHandles), then join.
    //    Joining inside the spawn loop would run jobs one at a time.
    // 2. Name threads with std::thread::Builder (e.g. "job-3") for debugging.
    // 3. join() returns Err(payload) if the thread panicked.
    // 4. Map that payload through panic_message.
    todo!("spawn all, then join all, capturing panics")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    #[test]
    fn results_keep_order() {
        let jobs: Vec<Job<u32>> = (0..5u32).map(|i| Box::new(move || i * 10) as Job<u32>).collect();
        let got: Vec<_> = run_jobs(jobs).into_iter().map(Result::unwrap).collect();
        assert_eq!(got, vec![0, 10, 20, 30, 40]);
    }

    #[test]
    fn panics_are_captured_with_message() {
        let jobs: Vec<Job<u32>> = vec![
            Box::new(|| 1),
            Box::new(|| panic!("static message")),
            Box::new(|| {
                let slot = 42;
                panic!("bad slot {slot}")
            }),
        ];
        let got = run_jobs(jobs);
        assert_eq!(got[0], Ok(1));
        assert_eq!(got[1], Err(JobError::Panicked("static message".into())));
        assert_eq!(got[2], Err(JobError::Panicked("bad slot 42".into())));
    }

    #[test]
    fn jobs_really_run_concurrently() {
        // Each job waits until all 4 have started. Run one at a time, that never happens.
        let started = Arc::new(AtomicUsize::new(0));
        let jobs: Vec<Job<bool>> = (0..4)
            .map(|_| {
                let started = started.clone();
                Box::new(move || {
                    started.fetch_add(1, SeqCst);
                    let deadline = Instant::now() + Duration::from_secs(2);
                    while started.load(SeqCst) < 4 && Instant::now() < deadline {
                        std::thread::yield_now();
                    }
                    started.load(SeqCst) == 4
                }) as Job<bool>
            })
            .collect();
        assert!(run_jobs(jobs).into_iter().all(|r| r == Ok(true)));
    }

    #[test]
    #[ignore = "learner must add a payload test"]
    fn learner_adversarial_test() {
        todo!("panic with a non-string payload (std::panic::panic_any(7u32)) and assert the message")
    }
}
