#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vault {
    pub balance: u64,
}

pub fn deposit(vault: &mut Vault, amount: u64) {
    todo!("increase balance by amount")
}

pub fn withdraw(vault: &mut Vault, amount: u64, authorized: bool) -> bool {
    todo!("succeed only when authorized and enough balance exists")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_and_withdraw() {
        let mut vault = Vault { balance: 0 };
        deposit(&mut vault, 10);
        assert_eq!(vault.balance, 10);
        assert!(withdraw(&mut vault, 4, true));
        assert_eq!(vault.balance, 6);
    }
}
