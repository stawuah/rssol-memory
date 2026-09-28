//! ex04 — Atomics: CAS loops, and publishing data with Release/Acquire.

use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

/// Tracks the highest slot seen by any thread, without a lock.
pub struct SlotTracker {
    highest: AtomicU64,
    observations: AtomicU64,
}

impl SlotTracker {
    pub const fn new() -> Self {
        SlotTracker { highest: AtomicU64::new(0), observations: AtomicU64::new(0) }
    }

    /// Record `slot`. Returns true iff THIS call raised the highest slot.
    pub fn observe(&self, slot: u64) -> bool {
        // Coding map:
        // 1. Count the observation (fetch_add, Relaxed: it's just a statistic).
        // 2. Load the current highest (Relaxed).
        // 3. Loop: if slot <= current, return false.
        // 4. compare_exchange_weak(current, slot, Relaxed, Relaxed):
        //      Ok  -> we raised it, return true
        //      Err(actual) -> someone changed it; current = actual; retry.
        //    (`fetch_max` does this in one call. Write the loop once to learn CAS.)
        todo!("CAS loop to raise the maximum")
    }

    pub fn highest(&self) -> u64 {
        self.highest.load(Ordering::Relaxed)
    }

    pub fn observations(&self) -> u64 {
        self.observations.load(Ordering::Relaxed)
    }
}

impl Default for SlotTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AlreadyPublished;

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;

/// A value written once by one thread and read by many (a tiny OnceLock).
pub struct Published {
    state: AtomicU8,
    value: AtomicU64,
}

impl Published {
    pub const fn new() -> Self {
        Published { state: AtomicU8::new(EMPTY), value: AtomicU64::new(0) }
    }

    /// Only the first caller wins. Others get Err(AlreadyPublished).
    pub fn publish(&self, v: u64) -> Result<(), AlreadyPublished> {
        // Coding map:
        // 1. Claim the right to write: compare_exchange(EMPTY, WRITING, Relaxed, Relaxed).
        //    Failure -> Err(AlreadyPublished).
        // 2. Store the value (Relaxed is fine, the next step publishes it).
        // 3. store(READY, Release): "everything I wrote before is now visible
        //    to anyone who Acquires READY".
        todo!("claim, write, then release-publish")
    }

    /// None until published; afterwards always the published value.
    pub fn get(&self) -> Option<u64> {
        // Coding map:
        // 1. load(state, Acquire). Only if READY, read the value.
        // 2. Why Acquire: it pairs with the Release in publish, guaranteeing
        //    we see the value stored before READY, not an older one.
        todo!("acquire-load the state, then read")
    }
}

impl Default for Published {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracker_keeps_the_max_across_threads() {
        let t = SlotTracker::new();
        let raised = std::sync::atomic::AtomicU64::new(0);
        std::thread::scope(|s| {
            for th in 0..8u64 {
                let (t, raised) = (&t, &raised);
                s.spawn(move || {
                    for i in 0..10_000u64 {
                        if t.observe(i * 8 + th) {
                            raised.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                });
            }
        });
        assert_eq!(t.highest(), 9_999 * 8 + 7);
        assert_eq!(t.observations(), 80_000);
        assert!(raised.load(Ordering::Relaxed) >= 1);
    }

    #[test]
    fn only_one_raise_for_a_repeated_slot() {
        let t = SlotTracker::new();
        let wins: usize = std::thread::scope(|s| {
            let hs: Vec<_> = (0..8).map(|_| s.spawn(|| t.observe(500))).collect();
            hs.into_iter().map(|h| h.join().unwrap() as usize).sum()
        });
        assert_eq!(wins, 1);
        assert!(!t.observe(10));
    }

    #[test]
    fn exactly_one_publisher_wins_and_readers_agree() {
        let p = Published::new();
        assert_eq!(p.get(), None);
        let winners: Vec<u64> = std::thread::scope(|s| {
            let hs: Vec<_> = (1..=8u64).map(|v| { let p = &p; s.spawn(move || p.publish(v).ok().map(|_| v)) }).collect();
            hs.into_iter().filter_map(|h| h.join().unwrap()).collect()
        });
        assert_eq!(winners.len(), 1);
        assert_eq!(p.get(), Some(winners[0]));
        assert_eq!(p.publish(99), Err(AlreadyPublished));
    }

    #[test]
    #[ignore = "learner must add an ordering test"]
    fn learner_adversarial_test() {
        todo!("spin-read get() from 4 threads while one publishes; assert no reader ever sees Some(0)")
    }
}
