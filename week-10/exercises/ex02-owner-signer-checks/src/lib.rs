#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockAccount {
    pub owner: String,
    pub is_signer: bool,
}

pub fn can_mutate(account: &MockAccount, expected_owner: &str) -> bool {
    todo!("mutation requires both matching owner and signer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_owner_and_signer() {
        let valid = MockAccount {
            owner: "vault-program".into(),
            is_signer: true,
        };
        let invalid = MockAccount {
            owner: "vault-program".into(),
            is_signer: false,
        };
        assert!(can_mutate(&valid, "vault-program"));
        assert!(!can_mutate(&invalid, "vault-program"));
    }
}
