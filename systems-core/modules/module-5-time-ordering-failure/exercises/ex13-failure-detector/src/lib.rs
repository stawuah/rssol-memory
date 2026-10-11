//! ex13 — Heartbeat failure detection with an injected clock.

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub type NodeId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Alive,
    /// Missed heartbeats for `suspect_after`: maybe slow, maybe dead.
    Suspect,
    /// Missed heartbeats for `dead_after`: stop routing to it.
    Dead,
    /// Never heard from.
    Unknown,
}

pub struct FailureDetector {
    suspect_after: Duration,
    dead_after: Duration,
    last_seen: HashMap<NodeId, Instant>,
}

impl FailureDetector {
    /// Panics unless suspect_after < dead_after.
    pub fn new(suspect_after: Duration, dead_after: Duration) -> Self {
        assert!(suspect_after < dead_after, "suspect_after must be < dead_after");
        FailureDetector { suspect_after, dead_after, last_seen: HashMap::new() }
    }

    /// Record a heartbeat. Time must never go backwards for a node: an older
    /// `now` than the stored one is ignored (delayed or reordered packets).
    pub fn heartbeat(&mut self, node: NodeId, now: Instant) {
        // Coding map: store max(existing, now).
        todo!("keep the latest heartbeat time")
    }

    pub fn status(&self, node: NodeId, now: Instant) -> NodeStatus {
        // Coding map:
        // 1. Unknown if never seen.
        // 2. silence = now.saturating_duration_since(last) (now may be < last).
        // 3. silence >= dead_after -> Dead; >= suspect_after -> Suspect; else Alive.
        todo!("classify by silence duration")
    }

    /// All nodes that are Suspect or Dead, sorted by id.
    pub fn unhealthy(&self, now: Instant) -> Vec<(NodeId, NodeStatus)> {
        // Coding map: map every known node through status, keep non-Alive, sort.
        todo!("list suspect and dead nodes")
    }

    /// Forget Dead nodes (bounded memory). Returns how many were removed.
    pub fn evict_dead(&mut self, now: Instant) -> usize {
        // Coding map: retain nodes whose silence < dead_after; return removed count.
        todo!("drop dead nodes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: Duration = Duration::from_millis(500);
    const D: Duration = Duration::from_millis(2_000);

    #[test]
    fn status_thresholds() {
        let t0 = Instant::now();
        let mut fd = FailureDetector::new(S, D);
        assert_eq!(fd.status(1, t0), NodeStatus::Unknown);
        fd.heartbeat(1, t0);
        assert_eq!(fd.status(1, t0 + Duration::from_millis(499)), NodeStatus::Alive);
        assert_eq!(fd.status(1, t0 + S), NodeStatus::Suspect);
        assert_eq!(fd.status(1, t0 + D), NodeStatus::Dead);
    }

    #[test]
    fn late_heartbeats_do_not_rewind_time() {
        let t0 = Instant::now();
        let mut fd = FailureDetector::new(S, D);
        fd.heartbeat(1, t0 + Duration::from_millis(1_000));
        fd.heartbeat(1, t0); // delayed packet from earlier
        assert_eq!(fd.status(1, t0 + Duration::from_millis(1_400)), NodeStatus::Alive);
        assert_eq!(fd.status(1, t0), NodeStatus::Alive); // clock skew: never negative
    }

    #[test]
    fn unhealthy_and_eviction() {
        let t0 = Instant::now();
        let mut fd = FailureDetector::new(S, D);
        fd.heartbeat(3, t0);
        fd.heartbeat(1, t0 + Duration::from_millis(1_000));
        fd.heartbeat(2, t0 + Duration::from_millis(2_000));
        let now = t0 + Duration::from_millis(2_100);
        assert_eq!(fd.unhealthy(now), vec![(1, NodeStatus::Suspect), (3, NodeStatus::Dead)]);
        assert_eq!(fd.evict_dead(now), 1);
        assert_eq!(fd.status(3, now), NodeStatus::Unknown);
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("a node that heartbeats exactly every suspect_after - 1ms is never Suspect over 100 beats")
    }
}
