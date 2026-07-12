pub fn derive_mock_pda(seed_a: &str, seed_b: &str) -> String {
    todo!("return a deterministic mock address by joining the seeds with `::`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivation_is_deterministic() {
        assert_eq!(derive_mock_pda("board", "user-1"), "board::user-1");
    }
}
