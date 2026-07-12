#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Proposed,
    Acked,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub status: Status,
}

pub fn acknowledge(task: &mut Task) -> bool {
    todo!("move Proposed to Acked and return true; otherwise return false")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acknowledges_proposed_task() {
        let mut task = Task {
            title: "sync".into(),
            status: Status::Proposed,
        };
        assert!(acknowledge(&mut task));
        assert_eq!(task.status, Status::Acked);
    }
}
