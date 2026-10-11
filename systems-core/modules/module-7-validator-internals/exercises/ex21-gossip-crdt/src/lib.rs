//! ex21 — Gossip state as a CRDT: last-writer-wins per (origin, label),
//! deterministic tie-breaks, and future-timestamp protection.

use std::collections::BTreeMap;

pub type NodeId = u32;

/// Values more than this far in the future (ms) are rejected.
pub const MAX_CLOCK_DRIFT_MS: u64 = 10_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrdsValue {
    pub origin: NodeId,
    pub label: String,
    pub wallclock: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Upsert {
    Inserted,
    Updated,
    /// Older, equal, or from the future: ignored.
    Ignored,
}

/// Does `a` supersede `b`? Compare (wallclock, data) lexicographically.
/// The data tie-break makes the result identical on every node.
pub fn supersedes(a: &CrdsValue, b: &CrdsValue) -> bool {
    todo!("strict (wallclock, data) comparison")
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Crds {
    table: BTreeMap<(NodeId, String), CrdsValue>,
}

impl Crds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, origin: NodeId, label: &str) -> Option<&CrdsValue> {
        self.table.get(&(origin, label.to_string()))
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Insert if new, replace if it supersedes, else ignore.
    /// `now` is our local clock (ms); reject wallclock > now + MAX_CLOCK_DRIFT_MS.
    pub fn upsert(&mut self, v: CrdsValue, now: u64) -> Upsert {
        // Coding map:
        // 1. Future check with saturating_add.
        // 2. key = (origin, label). Vacant -> insert, Inserted.
        // 3. Occupied: if supersedes(&v, existing) replace -> Updated, else Ignored.
        todo!("LWW upsert with future protection")
    }

    /// Pull everything from `other`. Returns how many entries changed.
    pub fn merge(&mut self, other: &Crds, now: u64) -> usize {
        // Coding map: upsert a clone of every value; count Inserted + Updated.
        todo!("upsert everything from the other replica")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_000_000;

    fn v(origin: NodeId, label: &str, wallclock: u64, data: &[u8]) -> CrdsValue {
        CrdsValue { origin, label: label.into(), wallclock, data: data.to_vec() }
    }

    #[test]
    fn last_writer_wins() {
        let mut c = Crds::new();
        assert_eq!(c.upsert(v(1, "contact", 10, b"a"), NOW), Upsert::Inserted);
        assert_eq!(c.upsert(v(1, "contact", 9, b"z"), NOW), Upsert::Ignored);
        assert_eq!(c.upsert(v(1, "contact", 11, b"b"), NOW), Upsert::Updated);
        assert_eq!(c.upsert(v(1, "contact", 11, b"b"), NOW), Upsert::Ignored); // idempotent
        assert_eq!(c.upsert(v(2, "contact", 1, b"x"), NOW), Upsert::Inserted);
        assert_eq!(c.get(1, "contact").unwrap().data, b"b");
    }

    #[test]
    fn ties_break_the_same_way_everywhere() {
        let (a, b) = (v(1, "x", 5, b"aaa"), v(1, "x", 5, b"bbb"));
        let mut c1 = Crds::new();
        c1.upsert(a.clone(), NOW);
        c1.upsert(b.clone(), NOW);
        let mut c2 = Crds::new();
        c2.upsert(b, NOW);
        c2.upsert(a, NOW);
        assert_eq!(c1, c2);
    }

    #[test]
    fn far_future_values_cannot_poison_the_table() {
        let mut c = Crds::new();
        assert_eq!(c.upsert(v(1, "x", NOW + MAX_CLOCK_DRIFT_MS + 1, b"evil"), NOW), Upsert::Ignored);
        assert_eq!(c.upsert(v(1, "x", NOW + MAX_CLOCK_DRIFT_MS, b"ok"), NOW), Upsert::Inserted);
        assert_eq!(c.upsert(v(1, "x", u64::MAX, b"evil"), u64::MAX - 5), Upsert::Updated); // saturating: no overflow
    }

    #[test]
    fn replicas_converge_regardless_of_merge_order() {
        let mut seed = 3u64;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            seed >> 40
        };
        let mut replicas = vec![Crds::new(), Crds::new(), Crds::new()];
        for _ in 0..300 {
            let r = (next() % 3) as usize;
            let val = v((next() % 5) as u32, ["a", "b"][(next() % 2) as usize], next() % 50, &[(next() % 4) as u8]);
            replicas[r].upsert(val, NOW);
        }
        let orders = [[0, 1, 2], [2, 1, 0], [1, 0, 2], [2, 0, 1]];
        let results: Vec<Crds> = orders
            .iter()
            .map(|o| {
                let mut acc = Crds::new();
                for &i in o {
                    acc.merge(&replicas[i], NOW);
                }
                acc.merge(&replicas[o[0]], NOW); // idempotent re-merge
                acc
            })
            .collect();
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("explain and test: what can a malicious node do with wallclock = now + MAX_CLOCK_DRIFT_MS?")
    }
}
