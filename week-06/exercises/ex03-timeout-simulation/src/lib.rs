#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestState {
    Completed,
    TimedOut,
}

pub fn classify_request(elapsed_ms: u64, timeout_ms: u64) -> RequestState {
    todo!("classify by comparing elapsed time to timeout")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_out_when_elapsed_exceeds_limit() {
        assert_eq!(classify_request(12, 10), RequestState::TimedOut);
        assert_eq!(classify_request(8, 10), RequestState::Completed);
    }
}
