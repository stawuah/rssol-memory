#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    pub owner: String,
    pub amount: u64,
}

pub fn find_owner<'a>(positions: &'a [Position], owner: &str) -> Option<&'a Position> {
    // Coding map: iterate borrowed positions and `find` the matching owner string.
    todo!("find and borrow the matching position")
}

pub fn checked_total(positions: &[Position]) -> Option<u64> {
    // Coding map: `try_fold` from 0_u64 and return checked_add at every step.
    todo!("use try_fold and checked_add without allocating")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_without_copying() {
        let positions = vec![
            Position { owner: "alice".into(), amount: 3 },
            Position { owner: "bob".into(), amount: 7 },
        ];
        assert_eq!(find_owner(&positions, "bob").map(|p| p.amount), Some(7));
    }

    #[test]
    fn aggregation_detects_overflow() {
        let positions = vec![
            Position { owner: "a".into(), amount: u64::MAX },
            Position { owner: "b".into(), amount: 1 },
        ];
        assert_eq!(checked_total(&positions), None);
    }
}
