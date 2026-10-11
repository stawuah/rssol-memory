//! ex12 — Logical time: Lamport clocks and vector clocks.

use std::collections::BTreeMap;

pub type NodeId = u32;

/// A Lamport clock: one counter. If event A caused B, then L(A) < L(B).
/// (The reverse is NOT true: L(A) < L(B) doesn't mean A caused B.)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Lamport(pub u64);

impl Lamport {
    /// A local event or a send. Returns the new timestamp.
    pub fn tick(&mut self) -> u64 {
        // Coding map: increment (checked; panic "clock overflow" on overflow), return.
        todo!("advance the clock")
    }

    /// We received a message stamped `remote`. Returns the new timestamp.
    pub fn on_receive(&mut self, remote: u64) -> u64 {
        // Coding map: self = max(self, remote) + 1.
        todo!("jump past the remote clock")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Causality {
    Before,
    After,
    Equal,
    /// Neither happened-before the other: a real conflict to resolve.
    Concurrent,
}

/// A vector clock: one counter per node. Detects concurrency exactly.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VectorClock {
    counts: BTreeMap<NodeId, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, node: NodeId) -> u64 {
        self.counts.get(&node).copied().unwrap_or(0)
    }

    /// `node` did something.
    pub fn increment(&mut self, node: NodeId) {
        // Coding map: counts[node] += 1 (missing = 0).
        todo!("bump this node's entry")
    }

    /// Element-wise max with `other` (what you do on receiving its state).
    pub fn merge(&mut self, other: &VectorClock) {
        // Coding map: for each (node, c) in other: counts[node] = max(counts[node], c).
        todo!("element-wise max")
    }

    /// Before: every entry <= other's, and at least one <.
    /// After: the mirror. Equal: all equal. Otherwise Concurrent.
    pub fn compare(&self, other: &VectorClock) -> Causality {
        // Coding map:
        // 1. Walk the UNION of node ids (a missing entry counts as 0).
        // 2. Track `some_less` (self < other somewhere) and `some_greater`.
        // 3. (false,false)=Equal (true,false)=Before (false,true)=After (true,true)=Concurrent.
        todo!("compare over the union of nodes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lamport_respects_message_causality() {
        let (mut a, mut b) = (Lamport::default(), Lamport::default());
        let sent = a.tick(); // a: 1
        for _ in 0..5 {
            b.tick(); // b: 5
        }
        let recv = b.on_receive(sent);
        assert!(recv > sent);
        assert_eq!(recv, 6);
        let reply = b.tick();
        assert_eq!(a.on_receive(reply), 8);
    }

    #[test]
    fn vector_clock_orders_causal_events() {
        let mut a = VectorClock::new();
        a.increment(1);
        let mut b = a.clone(); // b received a's state
        b.increment(2);
        assert_eq!(a.compare(&b), Causality::Before);
        assert_eq!(b.compare(&a), Causality::After);
        assert_eq!(a.compare(&a.clone()), Causality::Equal);
    }

    #[test]
    fn vector_clock_detects_concurrency() {
        let mut a = VectorClock::new();
        let mut b = VectorClock::new();
        a.increment(1);
        b.increment(2);
        assert_eq!(a.compare(&b), Causality::Concurrent);
        let mut merged = a.clone();
        merged.merge(&b);
        assert_eq!(a.compare(&merged), Causality::Before);
        assert_eq!(b.compare(&merged), Causality::Before);
        assert_eq!((merged.get(1), merged.get(2), merged.get(3)), (1, 1, 0));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("show two events with Lamport a < b that are nevertheless Concurrent by vector clock")
    }
}
