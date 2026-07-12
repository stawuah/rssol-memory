#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AmmReserves {
    pub collateral: u64,
    pub quote_micro_usd: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExploitError {
    EmptyReserve,
    InsufficientReserve,
    ExcessiveDeviation,
    Overflow,
}

pub fn spot_price(pool: AmmReserves) -> Result<u64, ExploitError> {
    // Coding map: reject empty collateral, then calculate
    // quote_micro_usd / collateral with the units already represented per token.
    todo!("read the manipulable reserve ratio")
}

pub fn buy_collateral(pool: &mut AmmReserves, collateral_out: u64, quote_in: u64) -> Result<(), ExploitError> {
    // Coding map:
    // 1. Reject collateral_out >= pool.collateral.
    // 2. Compute new collateral reserve by subtraction.
    // 3. Compute new quote reserve by checked addition.
    // 4. Commit both together. This deliberately simplified action raises spot.
    todo!("manipulate the modeled pool reserves")
}

pub fn vulnerable_borrow_limit(collateral_tokens: u64, pool: AmmReserves) -> Result<u64, ExploitError> {
    // Deliberately vulnerable design: calculate spot, multiply by collateral,
    // and permit debt up to 2/3 of that value. Do not add safeguards here.
    todo!("implement the insecure spot-only borrow limit")
}

pub fn guarded_borrow_limit(
    collateral_tokens: u64,
    pool: AmmReserves,
    independent_price: u64,
    max_deviation_bps: u16,
) -> Result<u64, ExploitError> {
    // Coding map:
    // 1. Read spot and calculate absolute deviation from independent_price.
    // 2. Reject deviation above max_deviation_bps.
    // 3. Use the lower of spot and independent price for collateral valuation.
    // 4. Return 2/3 of checked collateral value.
    todo!("bound a manipulable source with an independent reference")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_pool_matches_independent_price() {
        let pool = AmmReserves { collateral: 100, quote_micro_usd: 100_000_000 };
        assert_eq!(spot_price(pool), Ok(1_000_000));
        assert_eq!(guarded_borrow_limit(10, pool, 1_000_000, 500), Ok(6_666_666));
    }

    #[test]
    #[ignore = "required learner exploit: implement and then remove #[ignore]"]
    fn learner_demonstrates_oracle_manipulation_exploit() {
        // Required sequence:
        // 1. Record safe borrow limit at the independent $1 price.
        // 2. Buy collateral from the modeled pool to raise its reserve ratio.
        // 3. Prove vulnerable limit is now greater than safe limit.
        // 4. Prove guarded_borrow_limit rejects the manipulated pool.
        todo!("write the hands-on exploit")
    }
}
