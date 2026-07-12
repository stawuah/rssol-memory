#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferError {
    InvalidNumber,
    ZeroAmount,
    InsufficientFunds,
    Overflow,
}

pub fn parse_nonzero(raw: &str) -> Result<u64, TransferError> {
    // Coding map: parse/map error -> reject zero -> return the parsed amount.
    todo!("convert the parse error and reject zero")
}

pub fn transfer(source: &mut u64, destination: &mut u64, raw: &str) -> Result<(), TransferError> {
    // Coding map:
    // 1. Parse with `?`.
    // 2. Compute new_source with checked_sub.
    // 3. Compute new_destination with checked_add.
    // 4. Mutate both only after both computations succeed.
    todo!("use ? and checked arithmetic, then mutate only after all checks pass")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propagates_specific_errors() {
        assert_eq!(parse_nonzero("nope"), Err(TransferError::InvalidNumber));
        assert_eq!(parse_nonzero("0"), Err(TransferError::ZeroAmount));
    }

    #[test]
    fn failure_is_atomic() {
        let mut source = 5;
        let mut destination = u64::MAX;
        assert_eq!(transfer(&mut source, &mut destination, "1"), Err(TransferError::Overflow));
        assert_eq!((source, destination), (5, u64::MAX));
    }
}
