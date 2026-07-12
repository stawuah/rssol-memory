pub const Q32_SCALE: u128 = 1_u128 << 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedError {
    DivisionByZero,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q32(pub u64);

/// Q64.64 uses 64 fractional bits and therefore stores its raw representation in
/// u128. Multiplication needs either a 256-bit intermediate, decomposition, or a
/// reviewed big-integer/fixed-point implementation; blindly multiplying two u128
/// raw values is not safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q64(pub u128);

impl Q64 {
    pub const ONE: Self = Self(1_u128 << 64);

    pub fn from_ratio(numerator: u64, denominator: u64) -> Result<Self, FixedError> {
        // Coding map:
        // 1. Reject denominator == 0.
        // 2. Shift numerator as u128 left by 64 with checked_shl.
        // 3. Divide by denominator, documenting floor rounding.
        // 4. Return Q64 without narrowing the raw u128 representation.
        todo!("construct a Q64.64 ratio without floating point")
    }
}

impl Q32 {
    pub const ONE: Self = Self(1_u64 << 32);

    pub fn from_integer(value: u32) -> Self {
        Self((value as u64) << 32)
    }

    pub fn from_ratio(numerator: u64, denominator: u64) -> Result<Self, FixedError> {
        // Coding map:
        // 1. Reject denominator == 0.
        // 2. checked_mul numerator as u128 by Q32_SCALE.
        // 3. Divide the wide product by denominator as u128 (rounding down).
        // 4. Convert to u64, mapping failure to FixedError::Overflow.
        // 5. Wrap the raw result in Q32.
        todo!("construct a Q32.32 value from a rational number")
    }

    pub fn checked_mul(self, rhs: Self) -> Result<Self, FixedError> {
        // Coding map:
        // 1. Multiply both raw u64 values as u128.
        // 2. Divide the product by Q32_SCALE to restore one scale factor.
        // 3. Convert to u64 with an explicit overflow error.
        todo!("multiply two scaled values with a wide intermediate")
    }

    pub fn floor(self) -> u64 {
        self.0 >> 32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn represents_exact_binary_fraction() {
        assert_eq!(Q32::from_ratio(3, 2), Ok(Q32((1_u64 << 32) + (1_u64 << 31))));
    }

    #[test]
    fn multiplication_restores_scale() {
        let one_and_half = Q32::from_ratio(3, 2).unwrap();
        assert_eq!(one_and_half.checked_mul(Q32::from_integer(2)).unwrap().floor(), 3);
    }

    #[test]
    fn rejects_zero_denominator() {
        assert_eq!(Q32::from_ratio(1, 0), Err(FixedError::DivisionByZero));
    }

    #[test]
    fn q64_represents_one_and_a_half() {
        assert_eq!(Q64::from_ratio(3, 2), Ok(Q64((1_u128 << 64) + (1_u128 << 63))));
    }

    #[test]
    #[ignore = "learner must replace this with a real adversarial boundary test"]
    fn learner_adversarial_test() {
        todo!("add a fixed-point boundary test and remove #[ignore]")
    }
}
