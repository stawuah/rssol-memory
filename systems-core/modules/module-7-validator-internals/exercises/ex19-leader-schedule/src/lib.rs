//! ex19 — Deterministic stake-weighted leader schedule.

pub type NodeId = u32;

/// Provided: a small, fast, deterministic PRNG (SplitMix64).
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleError {
    NoStake,
    ZeroSlotsPerLeader,
}

/// Canonical order: drop zero stakes, sort by stake descending, then id
/// ascending. Every validator must get the same order from any input order.
pub fn canonical_stakes(stakes: &[(NodeId, u64)]) -> Vec<(NodeId, u64)> {
    // Coding map: filter stake > 0; sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0))).
    todo!("filter and sort deterministically")
}

/// Pick a node with probability proportional to stake.
/// `point` must be < total stake; walk cumulative sums and return the first
/// node whose cumulative stake exceeds `point`.
pub fn pick_weighted(sorted: &[(NodeId, u64)], point: u128) -> NodeId {
    // Coding map: cum = 0u128; for each (id, s): cum += s; if point < cum return id.
    // Unreachable if point < total: panic with a clear message.
    todo!("walk cumulative stake")
}

/// A schedule of `num_slots` leaders. Each chosen leader holds
/// `slots_per_leader` consecutive slots (Solana uses 4). Same inputs + seed
/// must give the same schedule on every machine.
pub fn leader_schedule(
    stakes: &[(NodeId, u64)],
    seed: u64,
    num_slots: usize,
    slots_per_leader: usize,
) -> Result<Vec<NodeId>, ScheduleError> {
    // Coding map:
    // 1. slots_per_leader == 0 -> ZeroSlotsPerLeader.
    // 2. sorted = canonical_stakes; total = sum as u128; total == 0 -> NoStake.
    // 3. rng_state = seed. While out.len() < num_slots:
    //      point = splitmix64(&mut rng_state) as u128 % total
    //      leader = pick_weighted(&sorted, point)
    //      push leader slots_per_leader times (don't exceed num_slots).
    todo!("seeded weighted sampling in canonical order")
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAKES: &[(NodeId, u64)] = &[(1, 300), (2, 100), (3, 0), (4, 600)];

    #[test]
    fn canonical_order_ignores_input_order() {
        let mut rev = STAKES.to_vec();
        rev.reverse();
        assert_eq!(canonical_stakes(STAKES), vec![(4, 600), (1, 300), (2, 100)]);
        assert_eq!(canonical_stakes(&rev), canonical_stakes(STAKES));
    }

    #[test]
    fn weighted_pick_boundaries() {
        let s = canonical_stakes(STAKES);
        assert_eq!(pick_weighted(&s, 0), 4);
        assert_eq!(pick_weighted(&s, 599), 4);
        assert_eq!(pick_weighted(&s, 600), 1);
        assert_eq!(pick_weighted(&s, 999), 2);
    }

    #[test]
    fn deterministic_and_seed_dependent() {
        let a = leader_schedule(STAKES, 42, 400, 4).unwrap();
        let mut shuffled = STAKES.to_vec();
        shuffled.swap(0, 3);
        assert_eq!(a, leader_schedule(&shuffled, 42, 400, 4).unwrap());
        assert_ne!(a, leader_schedule(STAKES, 43, 400, 4).unwrap());
    }

    #[test]
    fn leaders_hold_consecutive_slots_and_zero_stake_never_leads() {
        let s = leader_schedule(STAKES, 7, 402, 4).unwrap();
        assert_eq!(s.len(), 402);
        for chunk in s.chunks(4) {
            assert!(chunk.iter().all(|&l| l == chunk[0]));
        }
        assert!(!s.contains(&3));
    }

    #[test]
    fn frequency_tracks_stake() {
        let s = leader_schedule(STAKES, 1, 40_000, 1).unwrap();
        let share = |id| s.iter().filter(|&&l| l == id).count() as f64 / s.len() as f64;
        assert!((share(4) - 0.6).abs() < 0.02, "{}", share(4));
        assert!((share(2) - 0.1).abs() < 0.02, "{}", share(2));
    }

    #[test]
    fn errors() {
        assert_eq!(leader_schedule(&[(1, 0)], 1, 10, 4), Err(ScheduleError::NoStake));
        assert_eq!(leader_schedule(STAKES, 1, 10, 0), Err(ScheduleError::ZeroSlotsPerLeader));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("stakes near u64::MAX for several nodes must not overflow (the total needs u128)")
    }
}
