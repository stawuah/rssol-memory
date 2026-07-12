pub fn architecture_layers() -> Vec<&'static str> {
    todo!("return: program, client, indexer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_expected_layers() {
        assert_eq!(architecture_layers(), vec!["program", "client", "indexer"]);
    }
}
