pub fn distribute_evenly(tasks: &[u64], workers: usize) -> Vec<Vec<u64>> {
    todo!("distribute tasks round-robin into worker buckets")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_robin_distribution() {
        assert_eq!(distribute_evenly(&[1, 2, 3, 4, 5], 2), vec![vec![1, 3, 5], vec![2, 4]]);
    }
}
