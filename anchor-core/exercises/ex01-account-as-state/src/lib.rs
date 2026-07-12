#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Account {
    pub lamports: u64,
    pub data_len: usize,
    pub owner: &'static str,
}

/// Analogy:
/// A Solana account is like a labeled locker.
/// The label is the address, the contents are the state, and the owner decides who may edit it.
pub fn can_be_modified_by(account: &Account, program_id: &'static str) -> bool {
    // Coding map: compare account.owner with program_id and return that boolean.
    todo!("check ownership")
}

pub fn is_rent_like(account: &Account) -> bool {
    // Coding map: this learning model considers any data_len > 0 to store state.
    todo!("check whether the account stores state")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_controls_mutation() {
        let account = Account {
            lamports: 10,
            data_len: 8,
            owner: "program-a",
        };
        assert!(can_be_modified_by(&account, "program-a"));
        assert!(!can_be_modified_by(&account, "program-b"));
    }

    #[test]
    fn data_length_marks_state() {
        let account = Account {
            lamports: 1,
            data_len: 0,
            owner: "program-a",
        };
        assert!(!is_rent_like(&account));
        let account = Account { data_len: 64, ..account };
        assert!(is_rent_like(&account));
    }
}
