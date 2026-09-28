//! ex22 — Proof of History: sequential to generate, parallel to verify.
//!
//! (A toy mixing function stands in for SHA-256. The structure is what matters.)

/// Provided: one "hash" step.
pub fn mix(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Provided: one hash step that also mixes in data (e.g. a batch of transactions).
pub fn record(prev: u64, data: u64) -> u64 {
    mix(prev ^ data.rotate_left(17))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// Hash steps since the previous entry (>= 1).
    pub num_hashes: u64,
    /// Data mixed into the LAST step, if any.
    pub mixin: Option<u64>,
    /// Resulting hash.
    pub hash: u64,
}

/// Apply `mix` n times.
pub fn hash_n(start: u64, n: u64) -> u64 {
    todo!("repeated mixing")
}

/// Build the entry that follows `prev`: (num_hashes - 1) plain steps, then one
/// final step that is `record(h, data)` if there's a mixin, else `mix(h)`.
/// Panics if num_hashes == 0.
pub fn next_entry(prev: u64, num_hashes: u64, mixin: Option<u64>) -> Entry {
    todo!("plain steps then the final (possibly recording) step")
}

/// Check one entry against the hash before it. num_hashes == 0 is invalid.
pub fn verify_entry(prev: u64, e: &Entry) -> bool {
    todo!("recompute and compare")
}

pub fn verify_sequential(start: u64, entries: &[Entry]) -> bool {
    // Coding map: prev = start; each entry must verify against prev; prev = entry.hash.
    todo!("walk the chain")
}

/// Same answer as verify_sequential, using up to `threads` scoped threads.
/// Key insight: every entry's starting hash (the previous entry's `hash`)
/// is already in the list, so entries can be checked independently.
pub fn verify_parallel(start: u64, entries: &[Entry], threads: usize) -> bool {
    // Coding map:
    // 1. threads == 0 -> treat as 1. Empty -> true.
    // 2. Build `prevs`: prevs[0] = start, prevs[i] = entries[i - 1].hash.
    // 3. Split indices into `threads` chunks; in std::thread::scope, each thread
    //    checks verify_entry(prevs[i], &entries[i]) for its chunk.
    // 4. All chunks true -> true.
    todo!("verify entries independently on scoped threads")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(start: u64, n: usize) -> Vec<Entry> {
        let mut prev = start;
        (0..n)
            .map(|i| {
                let mixin = if i % 3 == 0 { Some(i as u64 * 1_000) } else { None };
                let e = next_entry(prev, 1 + (i as u64 % 50) * 20, mixin);
                prev = e.hash;
                e
            })
            .collect()
    }

    #[test]
    fn valid_chain_verifies_both_ways() {
        let es = chain(7, 400);
        assert!(verify_sequential(7, &es));
        for t in [1, 2, 3, 8, 1000] {
            assert!(verify_parallel(7, &es, t), "threads={t}");
        }
        assert!(verify_parallel(7, &[], 4));
    }

    #[test]
    fn tampering_is_detected_anywhere() {
        let es = chain(7, 200);
        for idx in [0, 1, 99, 199] {
            let mut bad = es.clone();
            bad[idx].mixin = Some(bad[idx].mixin.unwrap_or(0) ^ 1); // change the recorded data
            assert!(!verify_sequential(7, &bad));
            assert!(!verify_parallel(7, &bad, 4));
            let mut short = es.clone();
            short[idx].num_hashes += 1; // claim a different amount of elapsed "time"
            assert!(!verify_parallel(7, &short, 4));
        }
        assert!(!verify_parallel(8, &es, 4)); // wrong start
    }

    #[test]
    fn zero_hashes_is_invalid() {
        let e = Entry { num_hashes: 0, mixin: None, hash: 7 };
        assert!(!verify_entry(7, &e));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("reorder two entries in a valid chain and assert verification fails (why does it?)")
    }
}
