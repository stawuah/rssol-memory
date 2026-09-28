//! ex15 — Byzantine quorums: fault tolerance, stake-weighted supermajority,
//! duplicate votes, and equivocation.

use std::collections::{BTreeMap, HashMap};

pub type NodeId = u32;
pub type BlockId = u64;

/// With n equal-weight nodes, BFT tolerates f faulty nodes where n >= 3f + 1.
pub fn max_faulty(n: u64) -> u64 {
    // Coding map: 0 for n == 0, else (n - 1) / 3.
    todo!("largest f with n >= 3f + 1")
}

/// Strictly more than 2/3 of total stake. Use u128 so it can't overflow.
pub fn has_supermajority(voted: u64, total: u64) -> bool {
    // Coding map: total > 0 && voted * 3 > total * 2, computed in u128.
    todo!("voted > 2/3 of total without floats or overflow")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TallyError {
    ZeroTotalStake,
    UnknownVoter(NodeId),
    /// The same validator voted for two different blocks: slashable evidence.
    Equivocation { voter: NodeId, first: BlockId, second: BlockId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tally {
    pub stake_by_block: BTreeMap<BlockId, u128>,
    /// The block with a supermajority, if any (at most one can have one).
    pub confirmed: Option<BlockId>,
}

/// Count stake-weighted votes.
/// * Total stake is the stake of ALL validators, not just the ones who voted.
/// * The same (voter, block) delivered twice counts once (network duplicates).
/// * The same voter on two different blocks is Equivocation.
/// * A voter not in `stakes` is UnknownVoter.
pub fn tally(votes: &[(NodeId, BlockId)], stakes: &HashMap<NodeId, u64>) -> Result<Tally, TallyError> {
    // Coding map:
    // 1. total = sum of all stakes as u128; 0 -> ZeroTotalStake.
    // 2. voted: HashMap<NodeId, BlockId>. For each vote:
    //    - unknown voter -> error
    //    - already voted same block -> skip; different block -> Equivocation
    //    - else record, add stake to stake_by_block[block]
    // 3. confirmed = the block whose stake * 3 > total * 2 (u128 math).
    todo!("stake-weighted tally with duplicate and equivocation handling")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stakes(v: &[(NodeId, u64)]) -> HashMap<NodeId, u64> {
        v.iter().copied().collect()
    }

    #[test]
    fn fault_tolerance_table() {
        let got: Vec<u64> = [0, 1, 3, 4, 6, 7, 100].iter().map(|&n| max_faulty(n)).collect();
        assert_eq!(got, vec![0, 0, 0, 1, 1, 2, 33]);
    }

    #[test]
    fn supermajority_is_strict() {
        assert!(!has_supermajority(66, 99));
        assert!(has_supermajority(67, 100));
        assert!(!has_supermajority(2, 3));
        assert!(has_supermajority(u64::MAX, u64::MAX));
        assert!(!has_supermajority(0, 0));
    }

    #[test]
    fn stake_not_headcount_decides() {
        let s = stakes(&[(1, 70), (2, 10), (3, 10), (4, 10)]);
        let t = tally(&[(1, 9)], &s).unwrap();
        assert_eq!(t.confirmed, Some(9)); // 70% of stake, 1 of 4 nodes
        let t = tally(&[(2, 9), (3, 9), (4, 9)], &s).unwrap();
        assert_eq!(t.confirmed, None); // 3 of 4 nodes, only 30% of stake
    }

    #[test]
    fn non_voters_still_count_in_total() {
        let s = stakes(&[(1, 40), (2, 40), (3, 20)]);
        assert_eq!(tally(&[(1, 5), (2, 5)], &s).unwrap().confirmed, Some(5));
        assert_eq!(tally(&[(1, 5), (3, 5)], &s).unwrap().confirmed, None);
    }

    #[test]
    fn duplicates_count_once_and_equivocation_is_caught() {
        let s = stakes(&[(1, 50), (2, 50)]);
        let t = tally(&[(1, 5), (1, 5), (1, 5)], &s).unwrap();
        assert_eq!(t.stake_by_block[&5], 50);
        assert_eq!(tally(&[(1, 5), (1, 6)], &s), Err(TallyError::Equivocation { voter: 1, first: 5, second: 6 }));
        assert_eq!(tally(&[(9, 5)], &s), Err(TallyError::UnknownVoter(9)));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("prove two different blocks can never both be confirmed: an attacker with 1/3 stake equivocating is caught")
    }
}
