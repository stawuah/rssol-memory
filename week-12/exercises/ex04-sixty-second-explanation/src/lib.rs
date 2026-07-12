pub fn summary_sentence(project_name: &str) -> String {
    todo!("return a one-sentence summary beginning with the project name")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_project_name() {
        assert!(summary_sentence("Capstone").starts_with("Capstone"));
    }
}
