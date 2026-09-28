//! ex24 — Bounded concurrent fan-out: JoinSet + Semaphore + per-call timeouts,
//! with panics isolated per task.

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    Timeout,
    Failed(String),
    /// The fetch task panicked; other fetches are unaffected.
    Panicked,
}

/// Call `fetch(id)` for every id with at most `max_in_flight` calls running
/// at once, each limited to `per_call`. Returns one result per id, sorted by id.
pub async fn fetch_all<F, Fut>(
    ids: Vec<u64>,
    max_in_flight: usize,
    per_call: Duration,
    fetch: F,
) -> Vec<(u64, Result<u64, FetchError>)>
where
    F: Fn(u64) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<u64, FetchError>> + Send + 'static,
{
    // Coding map:
    // 1. assert!(max_in_flight > 0). fetch = Arc::new(fetch); sem = Arc::new(Semaphore::new(max_in_flight)).
    // 2. For each id:
    //    a. permit = sem.clone().acquire_owned().await: wait HERE, so we never
    //       even create more than max_in_flight tasks.
    //    b. Build the call future: tokio::time::timeout(per_call, fetch(id)),
    //       mapping Elapsed -> Err(Timeout).
    //    c. set.spawn(async move {
    //           let _permit = permit;                       // released when this task ends
    //           let inner = tokio::spawn(call).await;       // inner spawn isolates panics
    //           (id, inner.unwrap_or(Err(FetchError::Panicked)))
    //       })
    // 3. Drain with while let Some(res) = set.join_next().await; push res.expect(..).
    // 4. Sort by id and return.
    todo!("semaphore-bounded JoinSet with timeouts and panic isolation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    use std::time::Instant;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn respects_the_concurrency_limit() {
        let now = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let (n2, p2) = (now.clone(), peak.clone());
        let t0 = Instant::now();
        let out = fetch_all((0..20).collect(), 5, Duration::from_secs(2), move |id| {
            let (now, peak) = (n2.clone(), p2.clone());
            async move {
                let cur = now.fetch_add(1, SeqCst) + 1;
                peak.fetch_max(cur, SeqCst);
                tokio::time::sleep(Duration::from_millis(50)).await;
                now.fetch_sub(1, SeqCst);
                Ok(id * 2)
            }
        })
        .await;
        assert_eq!(out.len(), 20);
        assert!(out.iter().enumerate().all(|(i, (id, r))| *id == i as u64 && *r == Ok(i as u64 * 2)));
        assert!(peak.load(SeqCst) <= 5, "peak {}", peak.load(SeqCst));
        assert!(t0.elapsed() >= Duration::from_millis(190)); // 20 / 5 = 4 waves of 50 ms
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn timeouts_failures_and_panics_are_per_call() {
        let out = fetch_all(vec![3, 1, 2, 4], 2, Duration::from_millis(100), |id| async move {
            match id {
                1 => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    Ok(1)
                }
                2 => Err(FetchError::Failed("rpc 503".into())),
                3 => panic!("malformed response"),
                _ => Ok(id),
            }
        })
        .await;
        assert_eq!(
            out,
            vec![
                (1, Err(FetchError::Timeout)),
                (2, Err(FetchError::Failed("rpc 503".into()))),
                (3, Err(FetchError::Panicked)),
                (4, Ok(4)),
            ]
        );
    }

    #[tokio::test]
    async fn empty_input() {
        let out = fetch_all(vec![], 3, Duration::from_millis(10), |id| async move { Ok(id) }).await;
        assert!(out.is_empty());
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("10_000 ids with max_in_flight 8: assert the peak number of live tasks never exceeds 8")
    }
}
