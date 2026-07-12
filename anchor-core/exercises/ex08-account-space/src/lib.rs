pub fn space_for_profile(name_max: usize, tags_max: usize) -> usize {
    // Coding map:
    // 1. Add 8 bytes for the discriminator and 8 for score.
    // 2. Add 4 + name_max for String.
    // 3. Add 4 + tags_max for Vec<u8>.
    // 4. Return the total maximum encoded size.
    todo!("calculate account space")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_space() {
        assert_eq!(space_for_profile(20, 10), 8 + 8 + (4 + 20) + (4 + 10));
    }
}
