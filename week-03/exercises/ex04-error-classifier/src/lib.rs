#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemError {
    Transport,
    Storage,
    Validation,
}

pub fn is_retryable(error: &SystemError) -> bool {
    todo!("transport and storage are retryable for this exercise; validation is not")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_retryability() {
        assert!(is_retryable(&SystemError::Transport));
        assert!(is_retryable(&SystemError::Storage));
        assert!(!is_retryable(&SystemError::Validation));
    }
}
