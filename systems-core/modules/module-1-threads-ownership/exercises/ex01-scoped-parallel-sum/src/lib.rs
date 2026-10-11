//! ex01 — Scoped threads: split work, borrow data, combine safely.

use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SumError {
    ZeroWorkers,
    Overflow,
}

/// Split `0..len` into at most `workers` contiguous, non-empty ranges whose
/// sizes differ by at most one (earlier ranges get the extra element).
pub fn chunk_ranges(len: usize, workers: usize) -> Vec<Range<usize>> {
    // Coding map:
    // 1. Return an empty Vec when workers == 0 or len == 0.
    // 2. Use w = min(workers, len) so no range is empty.
    // 3. base = len / w, extra = len % w.
    // 4. Range i has size base + 1 if i < extra, else base.
    // 5. Walk a `start` cursor so ranges are contiguous and cover 0..len.
    todo!("split len into balanced contiguous ranges")
}

/// Sum `values` using up to `workers` threads that BORROW the slice
/// (no Arc, no clone). Any overflow, in a chunk or when combining, is an error.
pub fn parallel_sum(values: &[u64], workers: usize) -> Result<u64, SumError> {
    // Coding map:
    // 1. Reject workers == 0 with ZeroWorkers.
    // 2. Build ranges with chunk_ranges.
    // 3. Inside std::thread::scope, spawn one thread per range. Each thread
    //    borrows &values[range] and returns Option<u64> via checked_add fold.
    // 4. Join every handle inside the scope and collect the partial results.
    // 5. Combine partials with checked_add; any None or overflow -> Overflow.
    // 6. An empty slice sums to 0.
    todo!("sum chunks on scoped threads with checked arithmetic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_are_balanced_and_contiguous() {
        assert_eq!(chunk_ranges(10, 3), vec![0..4, 4..7, 7..10]);
        assert_eq!(chunk_ranges(2, 8), vec![0..1, 1..2]);
        assert!(chunk_ranges(0, 4).is_empty());
        assert!(chunk_ranges(5, 0).is_empty());
    }

    #[test]
    fn parallel_matches_sequential_for_any_worker_count() {
        let values: Vec<u64> = (1..=10_000).collect();
        let expected: u64 = values.iter().sum();
        for workers in 1..=9 {
            assert_eq!(parallel_sum(&values, workers), Ok(expected), "workers={workers}");
        }
        assert_eq!(parallel_sum(&[], 4), Ok(0));
    }

    #[test]
    fn overflow_is_detected_across_chunks() {
        assert_eq!(parallel_sum(&[u64::MAX, 1], 2), Err(SumError::Overflow));
        assert_eq!(parallel_sum(&[u64::MAX, 1], 1), Err(SumError::Overflow));
    }

    #[test]
    fn zero_workers_is_rejected() {
        assert_eq!(parallel_sum(&[1, 2, 3], 0), Err(SumError::ZeroWorkers));
    }

    #[test]
    #[ignore = "learner must add a boundary test"]
    fn learner_adversarial_test() {
        todo!("add a test where workers > len, or where the overflow happens only when combining partials")
    }
}
