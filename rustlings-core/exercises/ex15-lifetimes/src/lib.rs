pub struct AccountView<'info> {
    pub key: &'info str,
    pub owner: &'info str,
}

pub struct Context<'info> {
    pub authority: AccountView<'info>,
    pub vault: AccountView<'info>,
}

pub fn account_for_owner<'a>(accounts: &'a [AccountView<'a>], owner: &str) -> Option<&'a AccountView<'a>> {
    // Coding map: search within the borrowed slice and return its existing reference.
    // The result lifetime must remain tied to `accounts`.
    todo!("return a reference tied to the account slice")
}

pub fn vault_key<'info>(ctx: &'info Context<'info>) -> &'info str {
    // Coding map: follow ctx -> vault -> key and return that borrowed string slice.
    todo!("return the borrowed key from the borrowed context")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returned_references_borrow_the_input() {
        let accounts = [
            AccountView { key: "a", owner: "system" },
            AccountView { key: "vault", owner: "program" },
        ];
        assert_eq!(account_for_owner(&accounts, "program").map(|a| a.key), Some("vault"));
    }

    #[test]
    fn context_models_anchor_style_borrows() {
        let ctx = Context {
            authority: AccountView { key: "alice", owner: "system" },
            vault: AccountView { key: "vault", owner: "program" },
        };
        assert_eq!(vault_key(&ctx), "vault");
    }
}
