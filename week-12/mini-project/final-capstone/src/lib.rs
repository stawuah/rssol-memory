pub fn checklist() -> Vec<&'static str> {
    todo!("return: code, tests, docs, threat model, demo")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_required_artifacts() {
        assert_eq!(checklist(), vec!["code", "tests", "docs", "threat model", "demo"]);
    }
}
