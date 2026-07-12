pub const VIRTUAL_ASSETS: u128 = 1;
pub const VIRTUAL_SHARES: u128 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    ZeroAmount,
    ZeroShares,
    InsufficientShares,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vault {
    pub total_assets: u64,
    pub total_shares: u64,
}

impl Vault {
    pub fn preview_deposit(&self, assets: u64) -> Result<u64, VaultError> {
        // Coding map:
        // 1. Reject assets == 0.
        // 2. Compute assets * (total_shares + VIRTUAL_SHARES) as u128.
        // 3. Divide by total_assets + VIRTUAL_ASSETS (round down).
        // 4. Convert to u64 and reject a zero share result.
        todo!("quote shares using virtual offset accounting")
    }

    pub fn deposit(&mut self, assets: u64) -> Result<u64, VaultError> {
        // Coding map:
        // 1. Preview shares before any mutation.
        // 2. checked_add assets and shares into temporary new totals.
        // 3. Commit both totals only after every operation succeeds.
        // 4. Return minted shares.
        todo!("mint proportional shares atomically")
    }

    pub fn donate(&mut self, assets: u64) -> Result<(), VaultError> {
        // A donation increases total assets but mints no shares.
        // Reject zero, checked_add assets, then commit.
        todo!("model direct asset donation")
    }

    pub fn redeem(&mut self, shares: u64) -> Result<u64, VaultError> {
        // Coding map:
        // 1. Reject zero and shares > total_shares.
        // 2. Quote assets = shares * (total_assets + virtual assets) /
        //    (total_shares + virtual shares), rounding down.
        // 3. checked_sub shares and quoted assets into temporary totals.
        // 4. Commit atomically and return assets.
        todo!("burn shares for proportional assets")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_depositors_receive_equal_shares() {
        let mut vault = Vault::default();
        let alice = vault.deposit(100).unwrap();
        let bob = vault.deposit(100).unwrap();
        assert_eq!(alice, bob);
    }

    #[test]
    fn donation_attack_does_not_silently_take_victim_deposit() {
        let mut vault = Vault::default();
        vault.deposit(1).unwrap();
        vault.donate(1_000_000).unwrap();
        let before = vault;
        let result = vault.deposit(100);
        assert!(matches!(result, Ok(shares) if shares > 0) || result == Err(VaultError::ZeroShares));
        if result.is_err() {
            assert_eq!(vault, before);
        }
    }

    #[test]
    fn yield_increases_redemption_value() {
        let mut vault = Vault::default();
        let shares = vault.deposit(100).unwrap();
        vault.donate(20).unwrap();
        assert!(vault.redeem(shares).unwrap() >= 100);
    }

    #[test]
    #[ignore = "learner must add a distinct donation attack boundary"]
    fn learner_adversarial_test() {
        todo!("prove non-zero deposits never mutate state while minting zero shares")
    }
}
