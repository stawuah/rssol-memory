#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateAccount {
    pub key: &'static str,
    pub owner: &'static str,
    pub balance: u64,
}

/// Analogy:
/// A program is the factory machine.
/// The account is the storage bin the machine is allowed to update.
pub fn deposit(program: &Program, account: &mut StateAccount, amount: u64) -> Result<(), &'static str> {
    // Coding map:
    // 1. Reject account.owner != program.id.
    // 2. Reject amount == 0.
    // 3. Calculate with checked_add before mutation.
    // 4. Assign the new balance and return Ok(()).
    todo!("implement the program-state boundary")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_can_update_its_own_account() {
        let program = Program { id: "p1" };
        let mut account = StateAccount {
            key: "vault",
            owner: "p1",
            balance: 10,
        };

        deposit(&program, &mut account, 5).unwrap();
        assert_eq!(account.balance, 15);
    }

    #[test]
    fn wrong_owner_is_rejected() {
        let program = Program { id: "p1" };
        let mut account = StateAccount {
            key: "vault",
            owner: "p2",
            balance: 10,
        };

        assert_eq!(deposit(&program, &mut account, 5), Err("wrong owner"));
    }
}
