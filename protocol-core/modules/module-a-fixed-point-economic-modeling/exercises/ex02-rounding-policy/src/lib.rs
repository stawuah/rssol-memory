#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    DivisionByZero,
    Overflow,
}

pub fn mul_div_floor(a: u64, b: u64, denominator: u64) -> Result<u64, MathError> {
    // Coding map:
    // 1. Reject denominator == 0.
    // 2. checked_mul a and b as u128.
    // 3. Divide by denominator as u128; this floors naturally.
    // 4. Convert the quotient back to u64.
    todo!("implement checked floor multiplication/division")
}

pub fn mul_div_ceil(a: u64, b: u64, denominator: u64) -> Result<u64, MathError> {
    // Coding map:
    // 1. Reject denominator == 0 and compute the wide checked product.
    // 2. Calculate quotient and remainder separately.
    // 3. Add one only when remainder != 0, using checked_add.
    // 4. Convert the rounded quotient back to u64.
    todo!("implement checked ceiling multiplication/division")
}

pub fn fee_ceil(amount: u64, fee_bps: u64) -> Result<u64, MathError> {
    // One basis point is 1 / 10_000. Reuse the ceiling helper rather than
    // duplicating rounding logic.
    todo!("charge a basis-point fee rounded toward protocol solvency")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_results_do_not_gain_an_extra_unit() {
        assert_eq!(mul_div_floor(10, 3, 5), Ok(6));
        assert_eq!(mul_div_ceil(10, 3, 5), Ok(6));
    }

    #[test]
    fn remainder_changes_rounding_direction() {
        assert_eq!(mul_div_floor(10, 1, 3), Ok(3));
        assert_eq!(mul_div_ceil(10, 1, 3), Ok(4));
    }

    #[test]
    fn tiny_nonzero_fee_is_not_free() {
        assert_eq!(fee_ceil(1, 1), Ok(1));
    }

    #[test]
    #[ignore = "learner must test fee splitting resistance"]
    fn learner_adversarial_test() {
        todo!("compare one large fee with the sum of several smaller fees")
    }
}
