use std::collections::HashMap;

pub fn add_unique_member(members: &mut Vec<String>, member: String, capacity: usize) -> Result<(), &'static str> {
    // Coding map: duplicate check -> capacity check -> push -> Ok(()).
    // Do not mutate the vector on either error path.
    todo!("reject duplicates and enforce capacity before pushing")
}

pub fn balances(entries: &[(String, u64)]) -> Result<HashMap<String, u64>, &'static str> {
    // Coding map:
    // 1. Start an empty map.
    // 2. Insert each owner and amount.
    // 3. Detect and reject a key that was already present.
    // 4. Return the completed map.
    todo!("build a map while rejecting duplicate owners")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_is_unique_and_bounded() {
        let mut members = vec!["alice".into()];
        assert_eq!(add_unique_member(&mut members, "alice".into(), 2), Err("duplicate"));
        add_unique_member(&mut members, "bob".into(), 2).unwrap();
        assert_eq!(add_unique_member(&mut members, "carol".into(), 2), Err("capacity"));
    }

    #[test]
    fn map_rejects_ambiguous_input() {
        let entries = vec![("alice".into(), 1), ("alice".into(), 2)];
        assert_eq!(balances(&entries), Err("duplicate"));
    }
}
