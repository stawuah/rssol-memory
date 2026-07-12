#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    pub request_id: u64,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct ReplicaLog {
    operations: Vec<Operation>,
}

impl ReplicaLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, _operation: Operation) -> bool {
        todo!("append only unique request ids")
    }

    pub fn replay(&self) -> Vec<String> {
        todo!("return values in append order")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_preserves_order() {
        let mut log = ReplicaLog::new();
        log.append(Operation {
            request_id: 1,
            value: "set a".into(),
        });
        log.append(Operation {
            request_id: 2,
            value: "set b".into(),
        });
        assert_eq!(log.replay(), vec!["set a".to_string(), "set b".to_string()]);
    }

    #[test]
    fn duplicate_requests_do_not_change_replay() {
        let mut log = ReplicaLog::new();
        assert!(log.append(Operation {
            request_id: 9,
            value: "set a".into(),
        }));
        assert!(!log.append(Operation {
            request_id: 9,
            value: "set overwritten".into(),
        }));
        assert_eq!(log.replay(), vec!["set a".to_string()]);
    }
}
