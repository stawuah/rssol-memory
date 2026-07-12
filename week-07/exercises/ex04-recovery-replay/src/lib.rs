pub fn replay_sum(entries: &[i64]) -> i64 {
    todo!("sum entries in order to rebuild derived state")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebuilds_state_from_log() {
        assert_eq!(replay_sum(&[3, -1, 4]), 6);
    }
}
