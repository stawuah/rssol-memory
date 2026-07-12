#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmountError {
    Empty,
    NotANumber,
    ZeroNotAllowed,
    InsufficientFunds,
}

pub fn parse_amount(input: &str) -> Result<u64, AmountError> {
    // Coding map: normalize -> reject empty -> parse/map error -> reject zero.
    let input = input.trim();
    if input.is_empty() {
        return Err(AmountError::Empty);
    }

    let amount = input
        .parse::<u64>()
        .map_err(|_| AmountError::NotANumber)?;
    if amount == 0 {
        return Err(AmountError::ZeroNotAllowed);
    }

    Ok(amount)
}

pub fn transfer(balance: u64, raw_amount: &str) -> Result<u64, AmountError> {
    // Coding map: use `?` to parse, then checked subtraction to prevent underflow.
    let amount = parse_amount(raw_amount)?;
    balance
        .checked_sub(amount)
        .ok_or(AmountError::InsufficientFunds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_amounts() {
        assert_eq!(parse_amount("42"), Ok(42));
        assert_eq!(parse_amount(" 7 "), Ok(7));
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(parse_amount(""), Err(AmountError::Empty));
        assert_eq!(parse_amount("abc"), Err(AmountError::NotANumber));
        assert_eq!(parse_amount("0"), Err(AmountError::ZeroNotAllowed));
    }

    #[test]
    fn transfer_uses_the_parsed_amount() {
        assert_eq!(transfer(100, "25"), Ok(75));
        assert_eq!(transfer(20, "25"), Err(AmountError::InsufficientFunds));
    }
}
