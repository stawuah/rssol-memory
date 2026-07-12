#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    MissingSignature,
    WrongOwner,
    AmountTooLarge,
}

pub fn check_amount(amount: u64, max: u64) -> Result<(), ProtocolError> {
    // Coding map: return AmountTooLarge when amount > max; equality is valid;
    // return Ok(()) for every value within the inclusive limit.
    todo!("return a custom error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_too_large_amounts() {
        assert_eq!(check_amount(101, 100), Err(ProtocolError::AmountTooLarge));
    }
}
