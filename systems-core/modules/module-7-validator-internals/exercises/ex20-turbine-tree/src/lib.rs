//! ex20 — Turbine-style block propagation: a deterministic k-ary fanout tree.

pub type NodeId = u32;

/// Provided: SplitMix64 PRNG.
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic Fisher–Yates shuffle. Same seed -> same order on every node.
pub fn shuffle(nodes: &[NodeId], seed: u64) -> Vec<NodeId> {
    // Coding map:
    // 1. v = nodes.to_vec(); state = seed.
    // 2. for i in (1..v.len()).rev(): j = splitmix64(&mut state) % (i + 1); v.swap(i, j).
    todo!("seeded Fisher-Yates")
}

/// Position `index` in the tree (0 = root) forwards to these positions.
/// Children of i: fanout*i + 1 ..= fanout*i + fanout, limited to < n.
pub fn children(index: usize, fanout: usize, n: usize) -> Vec<usize> {
    // Coding map: compute the range with checked_mul/checked_add; clamp to n.
    todo!("k-ary children indices")
}

/// The position that forwards to `index`. The root has none.
pub fn parent(index: usize, fanout: usize) -> Option<usize> {
    todo!("inverse of children")
}

/// Hops from the root (root = 0).
pub fn layer(index: usize, fanout: usize) -> usize {
    // Coding map: count parent() steps until None.
    todo!("depth by walking parents")
}

/// For one shred: shuffle all nodes with the shred's seed, find `me`, and
/// return the NodeIds I must retransmit to. Empty if I'm not in the list.
pub fn retransmit_peers(me: NodeId, nodes: &[NodeId], shred_seed: u64, fanout: usize) -> Vec<NodeId> {
    // Coding map: order = shuffle(nodes, seed); pos = position of me; map children(pos) to ids.
    todo!("shuffle, locate myself, map children to ids")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_is_a_deterministic_permutation() {
        let nodes: Vec<NodeId> = (0..500).collect();
        let a = shuffle(&nodes, 9);
        assert_eq!(a, shuffle(&nodes, 9));
        assert_ne!(a, shuffle(&nodes, 10));
        let mut sorted = a.clone();
        sorted.sort();
        assert_eq!(sorted, nodes);
    }

    #[test]
    fn tree_shape() {
        assert_eq!(children(0, 3, 100), vec![1, 2, 3]);
        assert_eq!(children(1, 3, 100), vec![4, 5, 6]);
        assert_eq!(children(2, 3, 8), vec![7]);
        assert!(children(5, 3, 8).is_empty());
        assert_eq!(parent(0, 3), None);
        assert_eq!(parent(6, 3), Some(1));
        assert!(children(usize::MAX / 2, 4, usize::MAX).is_empty()); // no overflow
    }

    #[test]
    fn every_node_has_exactly_one_parent() {
        let (n, fanout) = (1_000, 7);
        let mut received = vec![0u32; n];
        received[0] = 1;
        for i in 0..n {
            for c in children(i, fanout, n) {
                assert_eq!(parent(c, fanout), Some(i));
                received[c] += 1;
            }
        }
        assert!(received.iter().all(|&r| r == 1));
    }

    #[test]
    fn two_thousand_nodes_in_two_hops_with_fanout_200() {
        assert_eq!(layer(1_999, 200), 2);
        assert_eq!(layer(200, 200), 1);
        assert_eq!(layer(0, 200), 0);
    }

    #[test]
    fn retransmit_covers_everyone_once() {
        let nodes: Vec<NodeId> = (100..400).collect();
        let mut got: Vec<NodeId> = nodes.iter().flat_map(|&me| retransmit_peers(me, &nodes, 77, 8)).collect();
        got.sort();
        let root = shuffle(&nodes, 77)[0];
        let mut expected: Vec<NodeId> = nodes.iter().copied().filter(|&n| n != root).collect();
        expected.sort();
        assert_eq!(got, expected);
        assert!(retransmit_peers(9_999, &nodes, 77, 8).is_empty());
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("show a node's tree position changes across shred seeds (no fixed 'always near the root' node)")
    }
}
