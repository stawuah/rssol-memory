//! ex05 — RwLock<Arc<T>>: cheap consistent snapshots that never block writers for long.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochInfo {
    pub epoch: u64,
    /// validator identity -> stake (lamports)
    pub stakes: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpochError {
    NotNextEpoch { current: u64, got: u64 },
}

/// Holds the current epoch. Readers grab an `Arc` snapshot and release the
/// lock immediately; a writer swaps in a whole new `Arc`.
pub struct EpochCache {
    current: RwLock<Arc<EpochInfo>>,
}

impl EpochCache {
    pub fn new(initial: EpochInfo) -> Self {
        EpochCache { current: RwLock::new(Arc::new(initial)) }
    }

    /// A consistent view of the current epoch. The read lock must be held
    /// only long enough to clone the Arc (a pointer copy + counter bump).
    pub fn snapshot(&self) -> Arc<EpochInfo> {
        // Coding map:
        // 1. Take the read lock.
        // 2. Arc::clone the inner Arc.
        // 3. Return it; the guard drops at the end of the statement.
        todo!("clone the Arc under a short read lock")
    }

    /// Replace the epoch. Only `current.epoch + 1` is accepted.
    /// Returns the previous snapshot.
    pub fn rotate(&self, next: EpochInfo) -> Result<Arc<EpochInfo>, EpochError> {
        // Coding map:
        // 1. Take the write lock.
        // 2. Check next.epoch == current.epoch + 1 (use checked_add).
        // 3. std::mem::replace the Arc with Arc::new(next); return the old one.
        // 4. Don't build `next` while holding the lock: it's passed in ready-made.
        todo!("validate then swap the Arc under the write lock")
    }
}

/// Total stake of a snapshot, overflow-safe.
pub fn total_stake(info: &EpochInfo) -> u128 {
    // Coding map: sum the stakes as u128.
    todo!("sum stakes as u128")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    fn epoch(n: u64) -> EpochInfo {
        // Invariant for tests: every stake equals the epoch number.
        EpochInfo { epoch: n, stakes: (0..5).map(|v| (format!("v{v}"), n)).collect() }
    }

    #[test]
    fn old_snapshots_survive_rotation() {
        let cache = EpochCache::new(epoch(1));
        let old = cache.snapshot();
        let prev = cache.rotate(epoch(2)).unwrap();
        assert_eq!(prev.epoch, 1);
        assert_eq!(old.epoch, 1);
        assert_eq!(total_stake(&old), 5);
        assert_eq!(cache.snapshot().epoch, 2);
    }

    #[test]
    fn rotation_must_be_sequential() {
        let cache = EpochCache::new(epoch(7));
        assert_eq!(cache.rotate(epoch(9)), Err(EpochError::NotNextEpoch { current: 7, got: 9 }));
        assert_eq!(cache.rotate(epoch(7)), Err(EpochError::NotNextEpoch { current: 7, got: 7 }));
        assert_eq!(cache.snapshot().epoch, 7);
    }

    #[test]
    fn holding_a_snapshot_does_not_block_the_writer() {
        let cache = Arc::new(EpochCache::new(epoch(1)));
        let _held = cache.snapshot(); // a slow reader keeps this for a long time
        let (tx, rx) = mpsc::channel();
        let c = cache.clone();
        std::thread::spawn(move || tx.send(c.rotate(epoch(2)).is_ok()).unwrap());
        assert_eq!(rx.recv_timeout(Duration::from_secs(5)), Ok(true));
    }

    #[test]
    fn readers_always_see_consistent_epochs() {
        let cache = EpochCache::new(epoch(1));
        std::thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    for _ in 0..20_000 {
                        let snap = cache.snapshot();
                        assert!(snap.stakes.values().all(|&st| st == snap.epoch), "torn snapshot");
                    }
                });
            }
            s.spawn(|| {
                for n in 2..=500 {
                    cache.rotate(epoch(n)).unwrap();
                }
            });
        });
        assert_eq!(cache.snapshot().epoch, 500);
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("assert rotate from u64::MAX returns NotNextEpoch instead of overflowing")
    }
}
