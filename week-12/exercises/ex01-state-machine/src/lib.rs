#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultInstruction {
    Initialize { opening_balance: u64 },
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VaultState {
    pub initialized: bool,
    pub balance: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultError {
    AlreadyInitialized,
    NotInitialized,
    ZeroAmount,
    InsufficientFunds,
}

impl VaultState {
    pub fn apply(&mut self, instruction: VaultInstruction) -> Result<(), VaultError> {
        match instruction {
            VaultInstruction::Initialize { opening_balance } => {
                if self.initialized {
                    return Err(VaultError::AlreadyInitialized);
                }
                self.initialized = true;
                self.balance = opening_balance;
                Ok(())
            }
            VaultInstruction::Deposit { amount } => {
                self.ensure_initialized()?;
                self.ensure_nonzero(amount)?;
                self.balance = self.balance.saturating_add(amount);
                Ok(())
            }
            VaultInstruction::Withdraw { amount } => {
                self.ensure_initialized()?;
                self.ensure_nonzero(amount)?;
                if self.balance < amount {
                    return Err(VaultError::InsufficientFunds);
                }
                self.balance -= amount;
                Ok(())
            }
        }
    }

    fn ensure_initialized(&self) -> Result<(), VaultError> {
        if self.initialized {
            Ok(())
        } else {
            Err(VaultError::NotInitialized)
        }
    }

    fn ensure_nonzero(&self, amount: u64) -> Result<(), VaultError> {
        if amount == 0 {
            Err(VaultError::ZeroAmount)
        } else {
            Ok(())
        }
    }
}

pub fn explain_state_machine() -> &'static str {
    "A program is a deterministic machine: same state + same instruction = same result."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_sets_state() {
        let mut state = VaultState::default();
        state.apply(VaultInstruction::Initialize {
            opening_balance: 10,
        }).unwrap();
        assert_eq!(state.balance, 10);
        assert!(state.initialized);
    }

    #[test]
    fn cannot_initialize_twice() {
        let mut state = VaultState::default();
        state.apply(VaultInstruction::Initialize {
            opening_balance: 10,
        }).unwrap();
        assert_eq!(
            state.apply(VaultInstruction::Initialize {
                opening_balance: 1
            }),
            Err(VaultError::AlreadyInitialized)
        );
    }

    #[test]
    fn withdraw_requires_funds() {
        let mut state = VaultState::default();
        state.apply(VaultInstruction::Initialize {
            opening_balance: 10,
        }).unwrap();
        assert_eq!(
            state.apply(VaultInstruction::Withdraw { amount: 11 }),
            Err(VaultError::InsufficientFunds)
        );
    }
}

