pub fn transaction_steps() -> Vec<&'static str> {
    todo!("return a high-level ordered trace: validate, load accounts, execute instruction, commit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_expected_trace() {
        assert_eq!(
            transaction_steps(),
            vec!["validate", "load accounts", "execute instruction", "commit"]
        );
    }
}
