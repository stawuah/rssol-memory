/// Analogy:
/// A PDA is like a locker number generated from the program name plus a recipe of seeds.
/// Same recipe, same locker. No private key sits behind it.
pub fn derive_address(program_id: &str, seeds: &[&str]) -> String {
    // Coding map:
    // 1. Start an owned String containing program_id.
    // 2. Iterate seeds in stable order.
    // 3. Append an unambiguous separator and each seed.
    // 4. Return the completed mock address.
    // This is a teaching model, not Solana's real PDA derivation algorithm.
    todo!("derive a deterministic mock address")
}

pub fn same_address(a: &str, b: &str) -> bool {
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seeds_same_address() {
        let a = derive_address("prog", &["vault", "alice"]);
        let b = derive_address("prog", &["vault", "alice"]);
        assert!(same_address(&a, &b));
    }

    #[test]
    fn different_seeds_different_address() {
        let a = derive_address("prog", &["vault", "alice"]);
        let b = derive_address("prog", &["vault", "bob"]);
        assert!(!same_address(&a, &b));
    }
}
