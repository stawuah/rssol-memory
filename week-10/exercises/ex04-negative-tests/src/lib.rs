pub fn withdraw(balance: u64, amount: u64, authorized: bool) -> Result<u64, &'static str> {
    todo!("reject unauthorized callers and insufficient balance")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unauthorized_withdrawal() {
        assert_eq!(withdraw(10, 3, false), Err("unauthorized"));
    }

    #[test]
    fn rejects_insufficient_balance() {
        assert_eq!(withdraw(2, 3, true), Err("insufficient balance"));
    }
}
