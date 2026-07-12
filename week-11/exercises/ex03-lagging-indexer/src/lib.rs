pub fn lag(current_slot: u64, indexed_slot: u64) -> u64 {
    todo!("return how many slots behind the indexer is")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_slot_lag() {
        assert_eq!(lag(20, 17), 3);
    }
}
