pub trait Validate {
    type Error;

    fn validate(&self) -> Result<(), Self::Error>;
}

pub struct Deposit {
    pub amount: u64,
}

pub struct Withdrawal {
    pub amount: u64,
    pub available: u64,
}

impl Validate for Deposit {
    type Error = &'static str;

    fn validate(&self) -> Result<(), Self::Error> {
        // Coding map: reject amount == 0, otherwise return Ok(()).
        todo!("reject a zero deposit")
    }
}

impl Validate for Withdrawal {
    type Error = &'static str;

    fn validate(&self) -> Result<(), Self::Error> {
        // Coding map: reject zero first, then reject amount > available, else succeed.
        todo!("reject zero and amounts above the available balance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_type_implements_the_same_contract() {
        assert_eq!(Deposit { amount: 0 }.validate(), Err("zero amount"));
        assert_eq!(Withdrawal { amount: 8, available: 7 }.validate(), Err("insufficient funds"));
        assert_eq!(Withdrawal { amount: 7, available: 7 }.validate(), Ok(()));
    }
}
