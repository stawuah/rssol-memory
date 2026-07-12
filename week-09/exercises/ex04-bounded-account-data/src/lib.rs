pub fn fits_in_account(input: &str, max_len: usize) -> bool {
    todo!("return whether the input fits within max_len bytes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_max_length() {
        assert!(fits_in_account("task", 8));
        assert!(!fits_in_account("task payload", 4));
    }
}
