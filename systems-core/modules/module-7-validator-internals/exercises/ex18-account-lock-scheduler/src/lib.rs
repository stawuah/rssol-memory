//! ex18 — Sealevel in miniature: schedule transactions into parallel batches
//! using their declared read/write account sets.

use std::collections::{HashMap, HashSet};

pub type Pubkey = u32;
pub type TxId = u64;

pub const MAX_ACCOUNTS_PER_TX: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tx {
    pub id: TxId,
    pub reads: Vec<Pubkey>,
    pub writes: Vec<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockError {
    TooManyAccounts(TxId),
    /// A key listed twice (in reads, in writes, or in both).
    DuplicateAccount { tx: TxId, key: Pubkey },
}

/// Reject malformed transactions before they reach the scheduler.
pub fn validate(tx: &Tx) -> Result<(), LockError> {
    // Coding map:
    // 1. reads.len() + writes.len() > MAX_ACCOUNTS_PER_TX -> TooManyAccounts.
    // 2. Insert every key into a HashSet; a failed insert -> DuplicateAccount.
    todo!("account limit and duplicate keys")
}

/// Two transactions conflict if they touch a common account and at least
/// one of them WRITES it. Read + read never conflicts.
pub fn conflicts(a: &Tx, b: &Tx) -> bool {
    // Coding map: any a.write in (b.reads ∪ b.writes), or any b.write in a.reads.
    todo!("write-write or read-write overlap")
}

/// Assign each tx to the earliest batch after every EARLIER tx it conflicts
/// with. Txs inside one batch are conflict-free and can run in parallel;
/// batches run in order. The result is equivalent to running txs one by one.
/// Returns tx ids per batch, in input order within each batch.
pub fn schedule(txs: &[Tx]) -> Result<Vec<Vec<TxId>>, LockError> {
    // Coding map (O(total accounts), no pairwise comparison):
    // 1. Validate every tx first; any error aborts.
    // 2. Track per account: last_write_batch and last_read_batch (HashMap<Pubkey, usize>).
    // 3. For each tx, batch = the max of:
    //      for each WRITE key: last_write+1 and last_read+1 (if present)
    //      for each READ key:  last_write+1 (if present)
    //    and 0 if nothing applies.
    // 4. Push the id into batches[batch] (grow the Vec as needed).
    // 5. Update last_write for writes and last_read = max(last_read, batch) for reads.
    todo!("greedy batch assignment preserving conflict order")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx(id: TxId, reads: &[Pubkey], writes: &[Pubkey]) -> Tx {
        Tx { id, reads: reads.to_vec(), writes: writes.to_vec() }
    }

    /// Test executor: each tx writes (sum of its reads + id) into each write.
    fn execute(state: &mut HashMap<Pubkey, u64>, t: &Tx) {
        let s: u64 = t.reads.iter().map(|k| state.get(k).copied().unwrap_or(0)).fold(t.id, u64::wrapping_add);
        for &w in &t.writes {
            let e = state.entry(w).or_insert(0);
            *e = e.wrapping_mul(31).wrapping_add(s);
        }
    }

    #[test]
    fn validation() {
        assert_eq!(validate(&tx(1, &[1], &[1])), Err(LockError::DuplicateAccount { tx: 1, key: 1 }));
        let many: Vec<Pubkey> = (0..65).collect();
        assert_eq!(validate(&tx(2, &many, &[])), Err(LockError::TooManyAccounts(2)));
        assert_eq!(schedule(&[tx(1, &[], &[5]), tx(3, &[2, 2], &[])]), Err(LockError::DuplicateAccount { tx: 3, key: 2 }));
    }

    #[test]
    fn conflict_rules() {
        assert!(!conflicts(&tx(1, &[7], &[]), &tx(2, &[7], &[])));
        assert!(conflicts(&tx(1, &[7], &[]), &tx(2, &[], &[7])));
        assert!(conflicts(&tx(1, &[], &[7]), &tx(2, &[], &[7])));
        assert!(!conflicts(&tx(1, &[], &[7]), &tx(2, &[], &[8])));
    }

    #[test]
    fn independent_and_read_only_txs_share_one_batch() {
        let txs = vec![tx(1, &[100], &[1]), tx(2, &[100], &[2]), tx(3, &[100], &[3])];
        assert_eq!(schedule(&txs).unwrap(), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn hot_account_serializes() {
        let txs = vec![tx(1, &[], &[9]), tx(2, &[9], &[1]), tx(3, &[], &[9]), tx(4, &[], &[2])];
        assert_eq!(schedule(&txs).unwrap(), vec![vec![1, 4], vec![2], vec![3]]);
    }

    #[test]
    fn batched_execution_equals_sequential_execution() {
        let mut seed = 7u64;
        let mut rnd = |m: u32| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((seed >> 33) as u32) % m
        };
        let txs: Vec<Tx> = (1..=300)
            .map(|id| {
                let mut keys: Vec<Pubkey> = Vec::new();
                while keys.len() < 4 {
                    let k = rnd(40);
                    if !keys.contains(&k) {
                        keys.push(k);
                    }
                }
                let split = 1 + rnd(3) as usize;
                tx(id, &keys[split..], &keys[..split])
            })
            .collect();
        let batches = schedule(&txs).unwrap();
        let by_id: HashMap<TxId, &Tx> = txs.iter().map(|t| (t.id, t)).collect();
        for b in &batches {
            for (i, a) in b.iter().enumerate() {
                for c in &b[i + 1..] {
                    assert!(!conflicts(by_id[a], by_id[c]), "conflict inside a batch: {a} {c}");
                }
            }
        }
        let mut seq = HashMap::new();
        txs.iter().for_each(|t| execute(&mut seq, t));
        let mut par = HashMap::new();
        for b in &batches {
            for id in b.iter().rev() {
                execute(&mut par, by_id[id]); // any order inside a batch must be equivalent
            }
        }
        assert_eq!(seq, par);
        assert!(batches.len() < txs.len());
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("100 txs all writing one hot account produce 100 batches; explain what that means for throughput and fees")
    }
}
