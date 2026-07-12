pub fn is_authorized(authority: &str, signer: &str) -> bool {
    todo!("authorized only when signer matches authority")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_must_match() {
        assert!(is_authorized("alice", "alice"));
        assert!(!is_authorized("alice", "bob"));
    }
}
