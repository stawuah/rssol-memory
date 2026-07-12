#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmError {
    ZeroInput,
    InvalidFee,
    ZeroOutput,
    SlippageExceeded,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pool {
    pub reserve_x: u64,
    pub reserve_y: u64,
    pub fee_bps: u16,
}

impl Pool {
    pub fn quote_x_for_y(&self, amount_in: u64) -> Result<u64, AmmError> {
        // Coding map:
        // 1. Reject zero input and fee_bps >= 10_000.
        // 2. Compute fee-adjusted input = amount_in * (10_000 - fee_bps) / 10_000.
        // 3. Reject adjusted input == 0.
        // 4. Compute reserve_y * adjusted_input / (reserve_x + adjusted_input)
        //    with checked u128 operations and floor rounding.
        // 5. Convert to u64 and reject zero output.
        todo!("quote constant-product output safely")
    }

    pub fn swap_x_for_y(&mut self, amount_in: u64, minimum_out: u64) -> Result<u64, AmmError> {
        // Coding map:
        // 1. Quote output before mutation.
        // 2. Reject output < minimum_out.
        // 3. Compute new reserve_x using the full amount_in (fee remains in pool).
        // 4. Compute new reserve_y by checked subtraction.
        // 5. Commit both reserves atomically and return output.
        todo!("enforce slippage and update both reserves atomically")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool() -> Pool { Pool { reserve_x: 1_000_000, reserve_y: 1_000_000, fee_bps: 30 } }

    #[test]
    fn swap_respects_minimum_out_and_product() {
        let mut p = pool();
        let old_k = (p.reserve_x as u128) * (p.reserve_y as u128);
        let quote = p.quote_x_for_y(10_000).unwrap();
        assert_eq!(p.swap_x_for_y(10_000, quote), Ok(quote));
        let new_k = (p.reserve_x as u128) * (p.reserve_y as u128);
        assert!(new_k >= old_k);
    }

    #[test]
    fn slippage_failure_is_atomic() {
        let mut p = pool();
        let before = p;
        let quote = p.quote_x_for_y(10_000).unwrap();
        assert_eq!(p.swap_x_for_y(10_000, quote + 1), Err(AmmError::SlippageExceeded));
        assert_eq!(p, before);
    }

    #[test]
    #[ignore = "learner must demonstrate price impact"]
    fn learner_adversarial_test() {
        todo!("compare effective output rates for small and large swaps")
    }
}
