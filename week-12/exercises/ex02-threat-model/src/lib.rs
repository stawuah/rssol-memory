pub fn minimum_threat_model_sections() -> Vec<&'static str> {
    todo!("return: assets, actors, assumptions, invariants, worst case bugs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_sections() {
        assert_eq!(
            minimum_threat_model_sections(),
            vec!["assets", "actors", "assumptions", "invariants", "worst case bugs"]
        );
    }
}
