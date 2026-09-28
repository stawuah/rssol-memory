//! ex10 — Group commit: batch many writes behind one expensive sync.

use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

/// Wait for at least one item, then keep collecting until either `max_items`
/// are collected or `max_wait` has passed since the FIRST item arrived.
/// Returns None only when the channel is closed and empty.
pub fn next_batch<T>(rx: &Receiver<T>, max_items: usize, max_wait: Duration) -> Option<Vec<T>> {
    // Coding map:
    // 1. assert!(max_items > 0).
    // 2. first = rx.recv().ok()? : block for the first item (None if closed).
    // 3. deadline = Instant::now() + max_wait.
    // 4. While batch.len() < max_items:
    //      remaining = deadline.saturating_duration_since(now); if zero -> break
    //      match rx.recv_timeout(remaining): Ok -> push; Timeout | Disconnected -> break
    // 5. Some(batch).
    todo!("block for one item, then fill until full or deadline")
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CommitStats {
    pub batches: u64,
    pub items: u64,
    pub syncs: u64,
    pub largest_batch: usize,
}

/// The writer loop: for each batch, `write` it, then `sync` ONCE.
/// Runs until the channel is closed and empty.
pub fn commit_loop<T>(
    rx: &Receiver<T>,
    max_items: usize,
    max_wait: Duration,
    mut write: impl FnMut(&[T]),
    mut sync: impl FnMut(),
) -> CommitStats {
    // Coding map:
    // 1. while let Some(batch) = next_batch(...)
    // 2. write(&batch); sync(); then update the stats.
    // 3. Order matters: acknowledge callers only AFTER sync (not modeled here).
    todo!("write each batch then sync once")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn full_batch_returns_immediately() {
        let (tx, rx) = mpsc::channel();
        for i in 0..10 {
            tx.send(i).unwrap();
        }
        let t0 = Instant::now();
        assert_eq!(next_batch(&rx, 4, Duration::from_secs(10)), Some(vec![0, 1, 2, 3]));
        assert!(t0.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn partial_batch_after_deadline() {
        let (tx, rx) = mpsc::channel();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        let t0 = Instant::now();
        assert_eq!(next_batch(&rx, 100, Duration::from_millis(30)), Some(vec![1, 2]));
        assert!(t0.elapsed() >= Duration::from_millis(30));
        drop(tx);
    }

    #[test]
    fn closed_and_empty_is_none_but_closed_with_items_is_some() {
        let (tx, rx) = mpsc::channel();
        tx.send(5).unwrap();
        drop(tx);
        assert_eq!(next_batch(&rx, 8, Duration::from_secs(10)), Some(vec![5]));
        assert_eq!(next_batch(&rx, 8, Duration::from_secs(10)), None);
    }

    #[test]
    fn group_commit_uses_far_fewer_syncs() {
        let (tx, rx) = mpsc::channel();
        let producer = std::thread::spawn(move || {
            for i in 0..5_000u32 {
                tx.send(i).unwrap();
            }
        });
        let mut written = Vec::new();
        let stats = commit_loop(&rx, 64, Duration::from_millis(2), |b| written.extend_from_slice(b), || {});
        producer.join().unwrap();
        assert_eq!(stats.items, 5_000);
        assert_eq!(written, (0..5_000).collect::<Vec<_>>());
        assert!(stats.largest_batch <= 64);
        assert!(stats.syncs < 5_000);
        assert_eq!(stats.syncs, stats.batches);
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("a slow producer (1 item every 5 ms) with max_wait 1 ms: assert every batch has size 1")
    }
}
