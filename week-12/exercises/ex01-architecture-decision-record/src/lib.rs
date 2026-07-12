pub fn adr_sections() -> Vec<&'static str> {
    todo!("return: context, decision, consequences")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_adr_sections() {
        assert_eq!(adr_sections(), vec!["context", "decision", "consequences"]);
    }
}
