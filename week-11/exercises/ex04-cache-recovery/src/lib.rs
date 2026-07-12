pub fn missing_range(last_seen: u64, latest: u64) -> Vec<u64> {
    todo!("return the missing slots that need to be replayed, exclusive of last_seen and inclusive of latest")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_missing_slots() {
        assert_eq!(missing_range(7, 10), vec![8, 9, 10]);
    }
}
