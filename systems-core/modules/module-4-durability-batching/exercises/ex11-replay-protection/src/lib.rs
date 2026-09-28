//! ex11 — Replay protection with bounded memory (a mini status cache).
//!
//! A transaction references a recent blockhash (here: the slot it came from).
//! Accept it only if that blockhash is recent AND the signature is new.
//! Because old blockhashes are rejected anyway, old signatures can be forgotten.

use std::collections::{BTreeMap, HashMap};

pub type Signature = u64;
pub type Slot = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    AlreadyProcessed,
    BlockhashExpired,
    BlockhashFromFuture,
}

pub struct StatusCache {
    max_age: u64,
    seen: HashMap<Signature, Slot>,
    by_slot: BTreeMap<Slot, Vec<Signature>>,
}

impl StatusCache {
    pub fn new(max_age: u64) -> Self {
        StatusCache { max_age, seen: HashMap::new(), by_slot: BTreeMap::new() }
    }

    /// Accept a transaction exactly once while its blockhash is valid.
    /// Age = current_slot - blockhash_slot; age == max_age is still valid.
    pub fn check_and_insert(&mut self, sig: Signature, blockhash_slot: Slot, current_slot: Slot) -> Result<(), ReplayError> {
        // Coding map:
        // 1. age = current_slot.checked_sub(blockhash_slot) else BlockhashFromFuture.
        // 2. age > max_age -> BlockhashExpired.
        // 3. seen contains sig -> AlreadyProcessed.
        // 4. Insert into BOTH maps (seen, and by_slot[blockhash_slot]).
        // 5. Check age before looking up sig: expired txs are rejected even if
        //    their signature was already purged from memory.
        todo!("validate age, reject duplicates, record the signature")
    }

    /// Forget every signature whose blockhash has expired at `current_slot`.
    /// Returns how many signatures were removed.
    pub fn purge(&mut self, current_slot: Slot) -> usize {
        // Coding map:
        // 1. Oldest still-valid slot: keep_from = current_slot.saturating_sub(max_age).
        // 2. let keep = self.by_slot.split_off(&keep_from); the old map now holds
        //    only expired slots. Swap so by_slot = keep.
        // 3. Remove each expired signature from `seen`; count them.
        todo!("drop expired slots and their signatures")
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates_are_rejected() {
        let mut c = StatusCache::new(150);
        assert_eq!(c.check_and_insert(1, 100, 120), Ok(()));
        assert_eq!(c.check_and_insert(1, 100, 121), Err(ReplayError::AlreadyProcessed));
        assert_eq!(c.check_and_insert(2, 100, 121), Ok(()));
    }

    #[test]
    fn age_boundaries() {
        let mut c = StatusCache::new(150);
        assert_eq!(c.check_and_insert(1, 50, 200), Ok(()));
        assert_eq!(c.check_and_insert(2, 49, 200), Err(ReplayError::BlockhashExpired));
        assert_eq!(c.check_and_insert(3, 201, 200), Err(ReplayError::BlockhashFromFuture));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn purged_signatures_still_cannot_replay() {
        let mut c = StatusCache::new(10);
        c.check_and_insert(7, 100, 100).unwrap();
        assert_eq!(c.purge(111), 1);
        assert!(c.is_empty());
        assert_eq!(c.check_and_insert(7, 100, 111), Err(ReplayError::BlockhashExpired));
    }

    #[test]
    fn memory_stays_bounded_forever() {
        let mut c = StatusCache::new(150);
        for slot in 0..20_000u64 {
            for k in 0..5 {
                c.check_and_insert(slot * 10 + k, slot, slot).unwrap();
            }
            c.purge(slot);
            assert!(c.len() <= 151 * 5);
        }
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("purge with current_slot < max_age must not underflow or remove valid entries")
    }
}
