pub fn square_all(values: &[u64]) -> Vec<u64> {
    todo!("simulate a worker-pool style batch by returning squared values in input order")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_input() {
        assert_eq!(square_all(&[1, 2, 3]), vec![1, 4, 9]);
    }
}
