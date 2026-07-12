pub fn can_afford(units_needed: u64, units_available: u64) -> bool {
    // Coding map: translate "fits" into one inclusive comparison and return it.
    todo!("compare compute units")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_budget() {
        assert!(can_afford(50_000, 200_000));
    }

    #[test]
    fn does_not_fit_budget() {
        assert!(!can_afford(250_000, 200_000));
    }
}
