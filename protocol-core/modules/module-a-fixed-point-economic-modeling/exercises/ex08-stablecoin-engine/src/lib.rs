pub const BPS: u128 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StableError {
    ZeroAmount,
    InvalidRatio,
    InsufficientCollateral,
    InsufficientDebt,
    InsufficientCollateralBalance,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub collateral: u64,
    pub debt: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Engine {
    pub total_supply: u64,
    pub collateral_price_micro_usd: u64,
    pub minimum_collateral_ratio_bps: u16,
}

impl Engine {
    pub fn max_debt(&self, collateral: u64) -> Result<u64, StableError> {
        // Coding map:
        // 1. Validate minimum ratio >= 10_000 and price > 0.
        // 2. Compute collateral * price as u128.
        // 3. Compute maximum debt value = collateral_value * BPS / ratio.
        // 4. Prices and stable debt both use micro-USD units in this model.
        // 5. Convert to u64 with overflow handling.
        todo!("calculate maximum safely backed debt")
    }

    pub fn mint(&mut self, position: &mut Position, amount: u64) -> Result<(), StableError> {
        // Coding map:
        // 1. Reject zero.
        // 2. checked_add amount to position debt and total supply into temporaries.
        // 3. Require new debt <= max_debt(current collateral).
        // 4. Commit both fields atomically.
        todo!("mint only overcollateralized debt")
    }

    pub fn burn(&mut self, position: &mut Position, amount: u64) -> Result<(), StableError> {
        // Coding map:
        // 1. Reject zero and amount > position.debt.
        // 2. checked_sub from position debt and total supply into temporaries.
        // 3. Commit both fields atomically.
        todo!("burn stablecoins and reduce matching debt")
    }

    pub fn withdraw_collateral(&self, position: &mut Position, amount: u64) -> Result<(), StableError> {
        // Coding map:
        // 1. Reject zero and amount > position.collateral.
        // 2. Compute remaining collateral without mutation.
        // 3. Require current debt <= max_debt(remaining collateral).
        // 4. Commit remaining collateral only after the backing check passes.
        todo!("withdraw only collateral not required to back debt")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        Engine { total_supply: 0, collateral_price_micro_usd: 2_000_000, minimum_collateral_ratio_bps: 15_000 }
    }

    #[test]
    fn rejects_unbacked_mint_without_mutation() {
        let mut e = engine();
        let mut p = Position { collateral: 150, debt: 0 };
        let before_e = e;
        let before_p = p;
        assert_eq!(e.mint(&mut p, 200_000_001), Err(StableError::InsufficientCollateral));
        assert_eq!((e, p), (before_e, before_p));
    }

    #[test]
    fn mint_and_burn_keep_supply_equal_to_debt() {
        let mut e = engine();
        let mut p = Position { collateral: 150, debt: 0 };
        e.mint(&mut p, 100_000_000).unwrap();
        e.burn(&mut p, 40_000_000).unwrap();
        assert_eq!(e.total_supply, p.debt);
    }

    #[test]
    fn collateral_cannot_be_withdrawn_below_ratio() {
        let mut e = engine();
        let mut p = Position { collateral: 150, debt: 0 };
        e.mint(&mut p, 100_000_000).unwrap();
        assert_eq!(e.withdraw_collateral(&mut p, 76), Err(StableError::InsufficientCollateral));
        assert_eq!(p.collateral, 150);
    }

    #[test]
    #[ignore = "learner must test two-position global accounting"]
    fn learner_adversarial_test() {
        todo!("assert global supply equals the sum of multiple position debts")
    }
}
