#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingAccount {
    pub key: &'static str,
    pub owner: &'static str,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    MissingSignature,
    NotWritable,
    WrongOwner,
}

/// Analogy:
/// Anchor constraints are the bouncer at the door.
/// They check the guest list before the party starts.
pub fn validate(accounts: &[IncomingAccount], expected_owner: &'static str) -> Result<(), CheckError> {
    // Coding map:
    // 1. Safely get the first account.
    // 2. Reject it when !is_signer, then when !is_writable.
    // 3. Iterate all accounts and reject any wrong owner.
    // 4. Return Ok(()) only after every check passes.
    todo!("validate the account list")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_accounts() {
        let accounts = vec![IncomingAccount {
            key: "payer",
            owner: "program-a",
            is_signer: true,
            is_writable: true,
        }];

        assert_eq!(validate(&accounts, "program-a"), Ok(()));
    }

    #[test]
    fn rejects_missing_signer() {
        let accounts = vec![IncomingAccount {
            key: "payer",
            owner: "program-a",
            is_signer: false,
            is_writable: true,
        }];

        assert_eq!(validate(&accounts, "program-a"), Err(CheckError::MissingSignature));
    }
}
