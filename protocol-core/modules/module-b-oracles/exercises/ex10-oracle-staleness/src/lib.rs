#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleError {
    FuturePublishSlot,
    Stale,
    InsufficientCollateral,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotPrice {
    pub micro_usd: u64,
    pub publish_slot: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowPosition {
    pub collateral_tokens: u64,
    pub debt_micro_usd: u64,
}

pub fn require_fresh(price: SlotPrice, current_slot: u64, max_age: u64) -> Result<(), OracleError> {
    // Coding map:
    // 1. checked_sub publish_slot from current_slot.
    // 2. Map subtraction failure to FuturePublishSlot.
    // 3. Reject age > max_age; equality remains valid.
    todo!("enforce a maximum slot age")
}

pub fn borrow(
    position: &mut BorrowPosition,
    amount_micro_usd: u64,
    price: SlotPrice,
    current_slot: u64,
    max_age: u64,
) -> Result<(), OracleError> {
    // Coding map:
    // 1. Call require_fresh before any mutation.
    // 2. Compute collateral value = tokens * price as u128.
    // 3. checked_add requested debt into a temporary.
    // 4. Require collateral value >= 150% of temporary debt.
    // 5. Convert/check values, then commit debt once.
    todo!("borrow only with fresh price and sufficient backing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_max_age_is_fresh() {
        assert_eq!(require_fresh(SlotPrice { micro_usd: 2_000_000, publish_slot: 90 }, 100, 10), Ok(()));
    }

    #[test]
    fn stale_borrow_is_atomic() {
        let mut p = BorrowPosition { collateral_tokens: 100, debt_micro_usd: 0 };
        let before = p;
        let price = SlotPrice { micro_usd: 2_000_000, publish_slot: 80 };
        assert_eq!(borrow(&mut p, 10_000_000, price, 100, 10), Err(OracleError::Stale));
        assert_eq!(p, before);
    }

    #[test]
    fn future_publish_slot_is_not_fresh() {
        assert_eq!(require_fresh(SlotPrice { micro_usd: 1, publish_slot: 101 }, 100, 10), Err(OracleError::FuturePublishSlot));
    }

    #[test]
    #[ignore = "learner must add another freshness boundary"]
    fn learner_adversarial_test() {
        todo!("add a boundary test and prove failure does not mutate state")
    }
}
