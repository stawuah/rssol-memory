pub fn required_checks() -> Vec<&'static str> {
    todo!("return: validate program, validate accounts, validate authority, perform cpi")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_expected_checks() {
        assert_eq!(
            required_checks(),
            vec!["validate program", "validate accounts", "validate authority", "perform cpi"]
        );
    }
}
