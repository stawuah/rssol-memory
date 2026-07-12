#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PythPrice {
    pub price: i64,
    pub confidence: u64,
    pub exponent: i32,
    pub publish_slot: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceError {
    NonPositivePrice,
    ConfidenceTooWide,
    UnsupportedExponent,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedPrice {
    pub price_micro_usd: u64,
    pub confidence_micro_usd: u64,
    pub publish_slot: u64,
}

fn scale_to_micro(value: u64, exponent: i32) -> Result<u64, PriceError> {
    // Coding map:
    // 1. Target exponent is -6 (micro-USD).
    // 2. Compute exponent - (-6), rejecting an absolute difference above 18.
    // 3. If positive, checked_mul by 10^difference as u128.
    // 4. If negative, divide by 10^(-difference), rounding down.
    // 5. Convert to u64 with explicit overflow handling.
    todo!("normalize a signed-decimal feed field to micro-USD")
}

pub fn validate_price(update: PythPrice, max_confidence_bps: u16) -> Result<ValidatedPrice, PriceError> {
    // Coding map:
    // 1. Reject price <= 0 and max_confidence_bps > 10_000.
    // 2. Normalize positive price and confidence with the same exponent.
    // 3. Compute confidence * 10_000 / price using u128.
    // 4. Reject when confidence basis points exceed the configured maximum.
    // 5. Return both normalized values and the original publish slot.
    todo!("validate and normalize a Pyth-style price")
}

pub fn conservative_collateral_price(price: ValidatedPrice) -> Result<u64, PriceError> {
    // For collateral, use the lower confidence bound. checked_sub prevents a
    // confidence interval wider than the price from wrapping below zero.
    todo!("return price minus confidence safely")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_price_and_confidence_together() {
        let update = PythPrice { price: 12_276_250, confidence: 1_500, exponent: -5, publish_slot: 50 };
        let valid = validate_price(update, 100).unwrap();
        assert_eq!(valid.price_micro_usd, 122_762_500);
        assert_eq!(valid.confidence_micro_usd, 15_000);
        assert_eq!(conservative_collateral_price(valid), Ok(122_747_500));
    }

    #[test]
    fn rejects_wide_confidence() {
        let update = PythPrice { price: 100, confidence: 6, exponent: 0, publish_slot: 1 };
        assert_eq!(validate_price(update, 500), Err(PriceError::ConfidenceTooWide));
    }

    #[test]
    #[ignore = "learner must add an exponent boundary test"]
    fn learner_adversarial_test() {
        todo!("test another exponent and remove #[ignore]")
    }
}
