#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersionedValue {
    pub value: u64,
    pub version: u64,
}

pub fn apply_if_version_matches(current: &mut VersionedValue, expected_version: u64, new_value: u64) -> bool {
    todo!("update only when expected_version matches current.version; bump version on success")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_only_on_matching_version() {
        let mut current = VersionedValue { value: 1, version: 2 };
        assert!(apply_if_version_matches(&mut current, 2, 9));
        assert_eq!(current, VersionedValue { value: 9, version: 3 });
        assert!(!apply_if_version_matches(&mut current, 2, 10));
    }
}
