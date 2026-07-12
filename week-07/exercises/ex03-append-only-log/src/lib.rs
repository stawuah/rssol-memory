#[derive(Debug, Default)]
pub struct AppendOnlyLog {
    entries: Vec<String>,
}

impl AppendOnlyLog {
    pub fn append(&mut self, value: String) {
        todo!("push value to the end")
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_append_order() {
        let mut log = AppendOnlyLog::default();
        log.append("a".into());
        log.append("b".into());
        assert_eq!(log.entries(), &["a".to_string(), "b".to_string()]);
    }
}
