pub const PPM_DENOMINATOR: u128 = 1_000_000;
pub const SECONDS_PER_YEAR: u128 = 31_536_000;
pub const MAX_ACCRUAL_SECONDS: u64 = 31_536_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccrualError {
    TimeWentBackward,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeState {
    pub assets: u64,
    pub annual_rate_ppm: u64,
    pub accrued_fees: u64,
    pub last_update: u64,
}

impl FeeState {
    pub fn accrue(&mut self, now: u64) -> Result<u64, AccrualError> {
        // Coding map:
        // 1. checked_sub last_update from now; map failure to TimeWentBackward.
        // 2. Cap elapsed at MAX_ACCRUAL_SECONDS.
        // 3. checked_mul assets * annual_rate_ppm * elapsed as u128.
        // 4. Divide by PPM_DENOMINATOR * SECONDS_PER_YEAR, rounding up only
        //    when the numerator has a remainder.
        // 5. Convert fee to u64 and checked_add it to accrued_fees.
        // 6. Commit accrued_fees and last_update only after all checks pass.
        todo!("accrue a linear annual fee safely")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> FeeState {
        FeeState { assets: 1_000_000, annual_rate_ppm: 10_000, accrued_fees: 0, last_update: 100 }
    }

    #[test]
    fn one_year_accrues_one_percent() {
        let mut fees = state();
        assert_eq!(fees.accrue(100 + SECONDS_PER_YEAR as u64), Ok(10_000));
    }

    #[test]
    fn backward_time_is_rejected_without_mutation() {
        let mut fees = state();
        let before = fees;
        assert_eq!(fees.accrue(99), Err(AccrualError::TimeWentBackward));
        assert_eq!(fees, before);
    }

    #[test]
    fn elapsed_time_is_capped() {
        let mut fees = state();
        assert_eq!(fees.accrue(100 + MAX_ACCRUAL_SECONDS * 10), Ok(10_000));
    }

    #[test]
    #[ignore = "learner must compare sequential and combined accrual"]
    fn learner_adversarial_test() {
        todo!("test repeated accrual and document the effect of ceiling rounding")
    }
}
