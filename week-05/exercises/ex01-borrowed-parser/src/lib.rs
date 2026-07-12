pub fn parse_pair(input: &str) -> Option<(&str, &str)> {
    todo!("split `key=value` into borrowed slices without allocating")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_borrowed_pair() {
        assert_eq!(parse_pair("role=leader"), Some(("role", "leader")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_pair("roleleader"), None);
    }
}
