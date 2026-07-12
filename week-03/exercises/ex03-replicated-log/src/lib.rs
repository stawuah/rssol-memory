#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub request_id: u64,
    pub payload: String,
}

#[derive(Debug, Default)]
pub struct ReplicatedLog {
    entries: Vec<LogEntry>,
}

impl ReplicatedLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, _entry: LogEntry) -> bool {
        todo!("append only if request_id has not been seen before; return true if appended")
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_unique_entries() {
        let mut log = ReplicatedLog::new();
        assert!(log.append(LogEntry {
            request_id: 1,
            payload: "a".into(),
        }));
        assert!(log.append(LogEntry {
            request_id: 2,
            payload: "b".into(),
        }));
        assert_eq!(log.entries().len(), 2);
    }

    #[test]
    fn rejects_duplicate_request_id() {
        let mut log = ReplicatedLog::new();
        assert!(log.append(LogEntry {
            request_id: 7,
            payload: "first".into(),
        }));
        assert!(!log.append(LogEntry {
            request_id: 7,
            payload: "duplicate".into(),
        }));
        assert_eq!(log.entries()[0].payload, "first");
    }
}
