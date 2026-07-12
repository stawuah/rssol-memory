#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryError {
    ExhaustedRetries,
}

pub fn retry_until_ok<T, F>(max_attempts: usize, mut operation: F) -> Result<T, RetryError>
where
    F: FnMut(usize) -> Option<T>,
{
    todo!("call operation with attempt numbers starting at 1 until it returns Some or attempts run out")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeeds_before_limit() {
        let mut seen = Vec::new();
        let result = retry_until_ok(5, |attempt| {
            seen.push(attempt);
            (attempt == 3).then_some("ready")
        });

        assert_eq!(result, Ok("ready"));
        assert_eq!(seen, vec![1, 2, 3]);
    }

    #[test]
    fn returns_error_when_exhausted() {
        let result = retry_until_ok(2, |_| None::<u32>);
        assert_eq!(result, Err(RetryError::ExhaustedRetries));
    }
}
