pub fn nth_fibonacci(n: usize) -> u64 {
    todo!("return the nth Fibonacci number where F(0) = 0 and F(1) = 1")
}

pub fn fibonacci_series(len: usize) -> Vec<u64> {
    todo!("return the first len Fibonacci numbers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_first_values() {
        assert_eq!(nth_fibonacci(0), 0);
        assert_eq!(nth_fibonacci(1), 1);
        assert_eq!(nth_fibonacci(2), 1);
        assert_eq!(nth_fibonacci(7), 13);
    }

    #[test]
    fn builds_series() {
        assert_eq!(fibonacci_series(0), Vec::<u64>::new());
        assert_eq!(fibonacci_series(1), vec![0]);
        assert_eq!(fibonacci_series(6), vec![0, 1, 1, 2, 3, 5]);
    }
}
