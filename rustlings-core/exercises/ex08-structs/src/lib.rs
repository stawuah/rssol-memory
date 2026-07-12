#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub owner: String,
    pub deposited: u64,
    pub active: bool,
}

impl Position {
    pub fn new(owner: String) -> Self {
        // Coding map: store owner, set deposited to zero, and establish active=true.
        todo!("construct an active position with a zero balance")
    }

    pub fn deposit(&mut self, amount: u64) -> Result<(), &'static str> {
        // Coding map:
        // 1. Reject an inactive position.
        // 2. Calculate with checked_add before changing the field.
        // 3. Assign the checked value and return success.
        todo!("reject inactive positions and update the balance safely")
    }

    pub fn close(self) -> Result<String, &'static str> {
        // Coding map: reject non-zero balance; otherwise move `owner` out of `self`.
        todo!("return the owner only if the balance is zero")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_invariants() {
        let position = Position::new("alice".into());
        assert_eq!(position.deposited, 0);
        assert!(position.active);
    }

    #[test]
    fn methods_preserve_invariants() {
        let mut position = Position::new("alice".into());
        position.deposit(7).unwrap();
        assert_eq!(position.deposited, 7);
        assert_eq!(position.close(), Err("non-zero balance"));
    }
}
