pub fn append_suffix(text: &mut String, suffix: &str) {
    // Coding map:
    // 1. Use the mutable reference; do not create a replacement String.
    // 2. Append the borrowed suffix with String's in-place API.
     text.push_str(suffix);

}

pub fn head(text: &str, count: usize) -> &str {
    // Coding map:
    // 1. Treat `count` as a byte endpoint.
    // 2. Borrow the range from byte zero to that endpoint.
    // 3. Return the slice, not a newly allocated String.
    &text[..count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_borrow_edits_in_place() {
        let mut text = String::from("anchor");
        append_suffix(&mut text, "-school");
        assert_eq!(text, "anchor-school");
    }

    #[test]
    fn slice_borrows_without_copying() {
        let text = String::from("solana");
        assert_eq!(head(&text, 3), "sol");
        assert_eq!(text, "solana");
    }
}
