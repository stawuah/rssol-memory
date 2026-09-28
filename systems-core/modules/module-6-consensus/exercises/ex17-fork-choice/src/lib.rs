//! ex17 — Fork choice: pick the heaviest fork by stake-weighted latest votes.

use std::collections::{BTreeMap, HashMap};

pub type Slot = u64;
pub type NodeId = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForkError {
    UnknownParent(Slot),
    SlotNotAfterParent { slot: Slot, parent: Slot },
    Duplicate(Slot),
}

/// A tree of slots. Every slot except the root has exactly one parent with a
/// smaller slot number.
pub struct ForkTree {
    root: Slot,
    parent: HashMap<Slot, Slot>,
    children: BTreeMap<Slot, Vec<Slot>>,
}

impl ForkTree {
    pub fn new(root: Slot) -> Self {
        let mut children = BTreeMap::new();
        children.insert(root, Vec::new());
        ForkTree { root, parent: HashMap::new(), children }
    }

    pub fn contains(&self, slot: Slot) -> bool {
        self.children.contains_key(&slot)
    }

    pub fn add(&mut self, slot: Slot, parent: Slot) -> Result<(), ForkError> {
        // Coding map:
        // 1. Duplicate if slot already present; UnknownParent if parent absent.
        // 2. slot <= parent -> SlotNotAfterParent.
        // 3. Record parent[slot], children[parent].push(slot), children[slot] = [].
        todo!("validate and link the new slot")
    }

    /// True if `slot` is `ancestor` or lies below it.
    pub fn is_descendant(&self, slot: Slot, ancestor: Slot) -> bool {
        // Coding map: walk parent links from `slot` until you hit `ancestor`
        // (true) or run out of parents (false). Unknown slot -> false.
        todo!("walk parent pointers")
    }

    /// Stake of all validators whose LATEST vote is `slot` or a descendant.
    /// Votes for slots not in the tree are ignored.
    pub fn subtree_stake(&self, slot: Slot, latest_votes: &HashMap<NodeId, Slot>, stakes: &HashMap<NodeId, u64>) -> u128 {
        // Coding map: sum stakes[v] (missing = 0) over votes where is_descendant(vote, slot).
        todo!("sum stake voting inside this subtree")
    }

    /// Greedy heaviest-subtree fork choice: start at the root; repeatedly move
    /// to the child with the most subtree stake (ties -> lower slot) until a leaf.
    pub fn best_leaf(&self, latest_votes: &HashMap<NodeId, Slot>, stakes: &HashMap<NodeId, u64>) -> Slot {
        // Coding map:
        // 1. cur = root.
        // 2. While children[cur] is non-empty: choose max by (stake, Reverse(slot)).
        // 3. Return cur.
        todo!("descend into the heaviest child until a leaf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //        0
    //      /   \
    //     1     2
    //    / \     \
    //   3   4     5
    fn tree() -> ForkTree {
        let mut t = ForkTree::new(0);
        for (s, p) in [(1, 0), (2, 0), (3, 1), (4, 1), (5, 2)] {
            t.add(s, p).unwrap();
        }
        t
    }

    fn map<K: std::hash::Hash + Eq + Copy, V: Copy>(v: &[(K, V)]) -> HashMap<K, V> {
        v.iter().copied().collect()
    }

    #[test]
    fn add_validation() {
        let mut t = tree();
        assert_eq!(t.add(3, 1), Err(ForkError::Duplicate(3)));
        assert_eq!(t.add(9, 8), Err(ForkError::UnknownParent(8)));
        assert_eq!(t.add(1, 4), Err(ForkError::Duplicate(1)));
        assert_eq!(t.add(6, 6), Err(ForkError::UnknownParent(6)));
        t.add(7, 5).unwrap();
        assert_eq!(t.add(6, 7), Err(ForkError::SlotNotAfterParent { slot: 6, parent: 7 }));
    }

    #[test]
    fn descendants() {
        let t = tree();
        assert!(t.is_descendant(4, 1));
        assert!(t.is_descendant(4, 0));
        assert!(t.is_descendant(2, 2));
        assert!(!t.is_descendant(5, 1));
        assert!(!t.is_descendant(42, 0));
    }

    #[test]
    fn heaviest_fork_wins_not_the_longest_or_most_voters() {
        let t = tree();
        let stakes = map(&[(1, 10), (2, 10), (3, 45)]);
        let votes = map(&[(1, 3), (2, 4), (3, 5)]);
        assert_eq!(t.subtree_stake(1, &votes, &stakes), 20);
        assert_eq!(t.subtree_stake(2, &votes, &stakes), 45);
        assert_eq!(t.best_leaf(&votes, &stakes), 5);
    }

    #[test]
    fn votes_on_inner_slots_and_ties() {
        let t = tree();
        let stakes = map(&[(1, 30), (2, 30)]);
        let votes = map(&[(1, 1), (2, 2)]);
        assert_eq!(t.best_leaf(&votes, &stakes), 3); // tie at root -> slot 1; tie below -> slot 3
        let votes = map(&[(1, 1), (2, 99)]); // unknown slot ignored
        assert_eq!(t.subtree_stake(0, &votes, &stakes), 30);
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("a validator switching its latest vote from fork 1 to fork 2 moves best_leaf accordingly")
    }
}
