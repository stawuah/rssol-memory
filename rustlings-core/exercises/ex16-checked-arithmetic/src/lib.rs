#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    Overflow,
    DivisionByZero,
    Conversion,
}

pub fn accrue(balance: u64, rate_bps: u64) -> Result<u64, MathError> {
    // Coding map:
    // 1. Widen operands to u128.
    // 2. checked_mul balance by rate_bps, then divide by 10_000.
    // 3. checked_add the increase to balance.
    // 4. Try converting back to u64 and map failure.
    todo!("calculate balance + balance * rate_bps / 10_000 with a u128 intermediate")
}

pub fn proportional(amount: u64, numerator: u64, denominator: u64) -> Result<u64, MathError> {
    // Coding map: reject zero denominator -> widen -> checked multiply -> divide ->
    // checked conversion to u64. Remember integer division rounds down.
    todo!("multiply widely, divide safely, and convert back to u64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basis_points_use_integer_math() {
        assert_eq!(accrue(10_000, 500), Ok(10_500));
    }

    #[test]
    fn wide_intermediate_avoids_early_overflow() {
        assert_eq!(proportional(u64::MAX, 2, 2), Ok(u64::MAX));
    }

    #[test]
    fn errors_are_explicit() {
        assert_eq!(proportional(10, 1, 0), Err(MathError::DivisionByZero));
        assert_eq!(accrue(u64::MAX, 10_000), Err(MathError::Conversion));
    }
}
