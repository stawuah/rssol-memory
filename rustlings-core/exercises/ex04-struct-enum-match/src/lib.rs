#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vault {
    pub balance: u64,
    pub frozen: bool,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            balance: 0,
            frozen: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Deposit(u64),
    Withdraw(u64),
    Freeze,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    Frozen,
    ZeroAmount,
    InsufficientFunds,
}

impl Vault {
     pub fn apply(&mut self, command: Command) -> Result<(), VaultError> {
    // Coding map:
    // 1. Match every Command variant.
    // 2. Validate frozen state and amount before mutation.
    // 3. For withdrawal, compare amount with the current balance.
    // 4. Apply exactly one legal state transition.
    // 5. Return a precise VaultError or Ok(()).
    match command {
        Command::Deposit(amount) => {
            if self.frozen {
                return Err(VaultError::Frozen);
            }
            if amount == 0 {
                return Err(VaultError::ZeroAmount);
            }
            self.balance += amount;
        }
        Command::Withdraw(amount) => {
            if self.frozen {
                return Err(VaultError::Frozen)
            }
            if amount == 0 {
                return Err(VaultError::ZeroAmount);
            }
            if amount > self.balance {
                return Err(VaultError::InsufficientFunds)
            }

            self.balance -= amount
        }
        Command::Freeze => {
           self.frozen = true;
        }
    }

    Ok(())
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_increases_balance() {
        let mut vault = Vault::new();
        vault.apply(Command::Deposit(10)).unwrap();
        assert_eq!(vault.balance, 10);
    }

    #[test]
    fn withdraw_checks_funds() {
        let mut vault = Vault::new();
        vault.apply(Command::Deposit(10)).unwrap();
        assert_eq!(vault.apply(Command::Withdraw(11)), Err(VaultError::InsufficientFunds));
    }

    #[test]
    fn frozen_vault_rejects_mutation() {
        let mut vault = Vault::new();
        vault.apply(Command::Freeze).unwrap();
        assert_eq!(vault.apply(Command::Deposit(1)), Err(VaultError::Frozen));
    }
}
