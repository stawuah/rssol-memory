#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockAccount {
    pub owner: String,
    pub is_signer: bool,
    pub data_len: usize,
}

pub fn is_valid_state_account(account: &MockAccount, expected_owner: &str, expected_len: usize) -> bool {
    todo!("owner must match, signer is not required, and data length must match")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_owner_and_size() {
        let account = MockAccount {
            owner: "program-1".into(),
            is_signer: false,
            data_len: 32,
        };
        assert!(is_valid_state_account(&account, "program-1", 32));
        assert!(!is_valid_state_account(&account, "program-2", 32));
    }
}
