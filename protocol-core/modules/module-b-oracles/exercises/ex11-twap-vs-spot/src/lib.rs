#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observation {
    pub slot: u64,
    pub price_micro_usd: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwapError {
    NotEnoughObservations,
    Unordered,
    InsufficientCoverage,
    ExcessiveSpotDeviation,
    Overflow,
}

pub fn twap(observations: &[Observation], minimum_slots: u64) -> Result<u64, TwapError> {
    // Coding map:
    // 1. Require at least two observations.
    // 2. Iterate adjacent windows with `.windows(2)`.
    // 3. checked_sub slots; reject zero or reversed intervals as Unordered.
    // 4. checked_add price * elapsed into a u128 cumulative value.
    // 5. checked_add total elapsed and require it >= minimum_slots.
    // 6. Divide cumulative price-time by total elapsed and convert to u64.
    todo!("calculate a duration-weighted average price")
}

pub fn require_spot_near_twap(spot: u64, average: u64, max_deviation_bps: u16) -> Result<(), TwapError> {
    // Coding map:
    // 1. Validate max_deviation_bps <= 10_000 and average > 0.
    // 2. Calculate absolute difference without signed conversion.
    // 3. Compute difference * 10_000 / average as u128.
    // 4. Reject deviation above the configured maximum.
    todo!("apply a spot-versus-TWAP circuit breaker")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_by_duration_not_sample_count() {
        let points = [
            Observation { slot: 0, price_micro_usd: 100 },
            Observation { slot: 9, price_micro_usd: 200 },
            Observation { slot: 10, price_micro_usd: 200 },
        ];
        assert_eq!(twap(&points, 10), Ok(110));
    }

    #[test]
    fn circuit_breaker_rejects_large_spot_move() {
        assert_eq!(require_spot_near_twap(120, 100, 1_000), Err(TwapError::ExcessiveSpotDeviation));
    }

    #[test]
    #[ignore = "learner must model a short-lived spot spike"]
    fn learner_adversarial_test() {
        todo!("show a one-slot spike has limited effect on a long TWAP")
    }
}
