#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockAccount {
    pub key: &'static str,
    pub owner: &'static str,
    pub is_signer: bool,
    pub is_writable: bool,
    pub data_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountError {
    MissingSignature { account: &'static str },
    NotWritable { account: &'static str },
    WrongOwner { account: &'static str },
    AccountTooSmall { account: &'static str },
}

pub fn validate_program_context(
    payer: &MockAccount,
    vault: &MockAccount,
    program_id: &'static str,
) -> Result<(), AccountError> {
    if !payer.is_signer {
        return Err(AccountError::MissingSignature { account: payer.key });
    }
    if !payer.is_writable {
        return Err(AccountError::NotWritable { account: payer.key });
    }
    if !vault.is_writable {
        return Err(AccountError::NotWritable { account: vault.key });
    }
    if vault.owner != program_id {
        return Err(AccountError::WrongOwner { account: vault.key });
    }
    if vault.data_len < 8 {
        return Err(AccountError::AccountTooSmall { account: vault.key });
    }
    Ok(())
}

pub fn explain_anchor_checks() -> &'static str {
    "Anchor account constraints are the seatbelt: they check signer, owner, mutability, and shape before logic runs."
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payer() -> MockAccount {
        MockAccount {
            key: "payer",
            owner: "system",
            is_signer: true,
            is_writable: true,
            data_len: 0,
        }
    }

    fn vault() -> MockAccount {
        MockAccount {
            key: "vault",
            owner: "program-1",
            is_signer: false,
            is_writable: true,
            data_len: 16,
        }
    }

    #[test]
    fn accepts_valid_context() {
        assert_eq!(validate_program_context(&payer(), &vault(), "program-1"), Ok(()));
    }

    #[test]
    fn rejects_missing_signature() {
        let mut bad = payer();
        bad.is_signer = false;
        assert_eq!(
            validate_program_context(&bad, &vault(), "program-1"),
            Err(AccountError::MissingSignature { account: "payer" })
        );
    }

    #[test]
    fn rejects_wrong_owner() {
        assert_eq!(
            validate_program_context(&payer(), &vault(), "program-2"),
            Err(AccountError::WrongOwner { account: "vault" })
        );
    }
}

