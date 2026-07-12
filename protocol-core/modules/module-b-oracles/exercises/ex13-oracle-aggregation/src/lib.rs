#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePrice {
    pub source_id: u8,
    pub price_micro_usd: u64,
    pub publish_slot: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateError {
    DuplicateSource,
    InsufficientQuorum,
    SourceDeviation,
    CircuitBreaker,
    FutureSlot,
    Overflow,
}

pub fn aggregate(
    sources: &[SourcePrice],
    current_slot: u64,
    max_age: u64,
    minimum_quorum: usize,
    max_source_deviation_bps: u16,
    last_accepted: u64,
    max_jump_bps: u16,
) -> Result<u64, AggregateError> {
    // Coding map:
    // 1. Reject duplicate source_id values.
    // 2. For each source, checked_sub publish_slot from current_slot; reject
    //    future slots and exclude sources older than max_age.
    // 3. Require at least minimum_quorum fresh prices.
    // 4. Sort copied u64 prices and find median; for even count, checked-average
    //    the two middle values without overflowing (`low + (high-low)/2`).
    // 5. Check every fresh source's absolute deviation from median; reject when
    //    any exceeds max_source_deviation_bps under this strict policy.
    // 6. Check median deviation from last_accepted when last_accepted != 0.
    // 7. Return median or the appropriate circuit error.
    todo!("aggregate independent fresh sources with circuit breakers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_median_for_consistent_quorum() {
        let sources = [
            SourcePrice { source_id: 1, price_micro_usd: 99, publish_slot: 100 },
            SourcePrice { source_id: 2, price_micro_usd: 100, publish_slot: 100 },
            SourcePrice { source_id: 3, price_micro_usd: 101, publish_slot: 99 },
        ];
        assert_eq!(aggregate(&sources, 100, 2, 3, 200, 100, 500), Ok(100));
    }

    #[test]
    fn rejects_duplicate_source_identity() {
        let sources = [
            SourcePrice { source_id: 1, price_micro_usd: 100, publish_slot: 10 },
            SourcePrice { source_id: 1, price_micro_usd: 100, publish_slot: 10 },
        ];
        assert_eq!(aggregate(&sources, 10, 1, 2, 100, 100, 100), Err(AggregateError::DuplicateSource));
    }

    #[test]
    fn aggregate_jump_trips_breaker() {
        let sources = [
            SourcePrice { source_id: 1, price_micro_usd: 120, publish_slot: 10 },
            SourcePrice { source_id: 2, price_micro_usd: 120, publish_slot: 10 },
            SourcePrice { source_id: 3, price_micro_usd: 120, publish_slot: 10 },
        ];
        assert_eq!(aggregate(&sources, 10, 1, 3, 100, 100, 1_000), Err(AggregateError::CircuitBreaker));
    }

    #[test]
    #[ignore = "learner must combine stale and outlier behavior"]
    fn learner_adversarial_test() {
        todo!("define and test stale-plus-outlier quorum policy")
    }
}
