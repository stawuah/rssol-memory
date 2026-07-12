pub fn deduplicate_preserving_order(input: &[u64]) -> Vec<u64> {
    todo!("return only the first occurrence of each id, preserving original order")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_duplicates_but_keeps_order() {
        assert_eq!(
            deduplicate_preserving_order(&[4, 1, 4, 2, 1, 3, 2]),
            vec![4, 1, 2, 3]
        );
    }

    #[test]
    fn empty_input_stays_empty() {
        assert_eq!(deduplicate_preserving_order(&[]), Vec::<u64>::new());
    }
}
