#[derive(Debug, Default)]
pub struct DurableEventLog {
    entries: Vec<String>,
}

impl DurableEventLog {
    pub fn append(&mut self, value: String) {
        todo!("append event")
    }

    pub fn snapshot(&self) -> String {
        todo!("join entries with newline separators")
    }

    pub fn from_snapshot(_snapshot: &str) -> Self {
        todo!("rebuild log from newline-separated snapshot")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_snapshot() {
        let mut log = DurableEventLog::default();
        log.append("a".into());
        log.append("b".into());
        let rebuilt = DurableEventLog::from_snapshot(&log.snapshot());
        assert_eq!(rebuilt.snapshot(), "a\nb");
    }
}
