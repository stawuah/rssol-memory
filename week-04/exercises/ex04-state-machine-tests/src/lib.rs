#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Proposed,
    Acked,
    Completed,
}

pub fn can_transition(from: Status, to: Status) -> bool {
    todo!("Proposed -> Acked -> Completed is allowed; other forward jumps are not")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_transitions() {
        assert!(can_transition(Status::Proposed, Status::Acked));
        assert!(can_transition(Status::Acked, Status::Completed));
    }

    #[test]
    fn rejects_invalid_transitions() {
        assert!(!can_transition(Status::Proposed, Status::Completed));
        assert!(!can_transition(Status::Completed, Status::Proposed));
    }
}
