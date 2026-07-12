#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub cost: u64,
}

pub fn assign_tasks(tasks: &[Task], workers: usize) -> Vec<Vec<Task>> {
    todo!("assign tasks round-robin, cloning tasks into worker queues for simplicity")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_every_task() {
        let tasks = vec![
            Task { id: 1, cost: 3 },
            Task { id: 2, cost: 5 },
            Task { id: 3, cost: 2 },
        ];
        let assigned = assign_tasks(&tasks, 2);
        assert_eq!(assigned.len(), 2);
        assert_eq!(assigned[0][0].id, 1);
        assert_eq!(assigned[1][0].id, 2);
    }
}
