pub const PPM: u64 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateError {
    BorrowExceedsSupply,
    InvalidKink,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KinkModel {
    pub base_rate_ppm: u64,
    pub kink_utilization_ppm: u64,
    pub slope_one_ppm: u64,
    pub slope_two_ppm: u64,
}

pub fn utilization_ppm(supplied: u64, borrowed: u64) -> Result<u64, RateError> {
    // Coding map:
    // 1. Reject borrowed > supplied.
    // 2. Return zero for supplied == 0.
    // 3. Compute borrowed * PPM / supplied with a u128 intermediate.
    todo!("calculate bounded utilization in parts per million")
}

impl KinkModel {
    pub fn borrow_rate(&self, utilization: u64) -> Result<u64, RateError> {
        // Coding map:
        // 1. Validate kink is in 1..=PPM and utilization <= PPM.
        // 2. If utilization <= kink, calculate base + slope_one * util / kink.
        // 3. Otherwise calculate excess = util - kink and add:
        //    base + slope_one + slope_two * excess / (PPM - kink).
        // 4. Handle kink == PPM without a zero denominator in the upper branch.
        // 5. Use checked u128 math and checked conversion throughout.
        todo!("evaluate the continuous piecewise-linear borrow rate")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> KinkModel {
        KinkModel { base_rate_ppm: 20_000, kink_utilization_ppm: 800_000, slope_one_ppm: 100_000, slope_two_ppm: 900_000 }
    }

    #[test]
    fn utilization_is_scaled_before_division() {
        assert_eq!(utilization_ppm(1_000, 800), Ok(800_000));
    }

    #[test]
    fn curve_is_continuous_at_kink() {
        assert_eq!(model().borrow_rate(800_000), Ok(120_000));
        assert!(model().borrow_rate(800_001).unwrap() >= 120_000);
    }

    #[test]
    fn full_utilization_reaches_both_slopes() {
        assert_eq!(model().borrow_rate(PPM), Ok(1_020_000));
    }

    #[test]
    #[ignore = "learner must test monotonicity across the curve"]
    fn learner_adversarial_test() {
        todo!("sample valid utilization points and assert non-decreasing rates")
    }
}
