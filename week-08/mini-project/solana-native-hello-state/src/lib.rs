#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterAccount {
    pub value: u32,
}

pub fn increment(account: &mut CounterAccount) {
    todo!("increment the stored counter value")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_state() {
        let mut account = CounterAccount { value: 0 };
        increment(&mut account);
        assert_eq!(account.value, 1);
    }
}
