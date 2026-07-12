pub const PPM: u128 = 1_000_000;
pub const BPS: u128 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidationError {
    InvalidRatio,
    HealthyPosition,
    ZeroPrice,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub collateral_amount: u64,
    pub collateral_price_micro_usd: u64,
    pub debt_amount: u64,
    pub debt_price_micro_usd: u64,
    pub liquidation_threshold_bps: u16,
}

pub fn health_factor_ppm(position: Position) -> Result<u64, LiquidationError> {
    // Coding map:
    // 1. Validate threshold <= 10_000 and non-zero prices.
    // 2. Return u64::MAX for zero debt (no liquidation liability).
    // 3. Compute collateral value and debt value as u128.
    // 4. Apply threshold: adjusted collateral = value * threshold / BPS.
    // 5. Compute adjusted_collateral * PPM / debt_value.
    // 6. Convert to u64 with overflow handling.
    todo!("calculate a scaled health factor")
}

pub fn collateral_to_seize(
    position: Position,
    requested_repay: u64,
    close_factor_bps: u16,
    bonus_bps: u16,
) -> Result<(u64, u64), LiquidationError> {
    // Coding map:
    // 1. Require health_factor_ppm(position) < PPM.
    // 2. Validate close factor and bonus ratios.
    // 3. Cap repay by debt * close_factor / BPS and requested_repay.
    // 4. Convert repay to debt value, add liquidation bonus.
    // 5. Convert value to collateral amount using ceiling division.
    // 6. Cap seized collateral at available collateral.
    // 7. Return (actual_repay, seized_collateral).
    todo!("quote a bounded liquidation")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unhealthy() -> Position {
        Position { collateral_amount: 100, collateral_price_micro_usd: 1_000_000, debt_amount: 90, debt_price_micro_usd: 1_000_000, liquidation_threshold_bps: 8_000 }
    }

    #[test]
    fn health_applies_collateral_threshold() {
        assert_eq!(health_factor_ppm(unhealthy()), Ok(888_888));
    }

    #[test]
    fn close_factor_limits_repayment() {
        let (repaid, seized) = collateral_to_seize(unhealthy(), 90, 5_000, 500).unwrap();
        assert_eq!(repaid, 45);
        assert_eq!(seized, 48); // ceil(45 * 1.05)
    }

    #[test]
    fn zero_debt_is_maximally_healthy() {
        let mut p = unhealthy();
        p.debt_amount = 0;
        assert_eq!(health_factor_ppm(p), Ok(u64::MAX));
    }

    #[test]
    #[ignore = "learner must test the exact health boundary"]
    fn learner_adversarial_test() {
        todo!("test exactly healthy versus nearest unhealthy liquidation eligibility")
    }
}
