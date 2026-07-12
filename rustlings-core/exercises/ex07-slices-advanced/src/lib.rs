/// Return the bytes after the first `:` without allocating.
pub fn payload_bytes(input: &[u8]) -> Option<&[u8]> {
    // Coding map:
    // 1. Locate the first byte equal to b':'.
    // 2. Advance one byte so the delimiter is excluded.
    // 3. Use safe slice access and return the borrowed remainder.
    // 4. Return None when no delimiter exists.
    todo!("find the delimiter and return a borrowed byte slice")
}

/// Return at most `max_chars` Unicode scalar values without splitting UTF-8.
pub fn safe_prefix(text: &str, max_chars: usize) -> &str {
    // Coding map:
    // 1. Iterate over Unicode character byte positions with `char_indices`.
    // 2. Determine the byte boundary after at most `max_chars` characters.
    // 3. Use text.len() when the requested prefix includes the full string.
    // 4. Slice only at the valid boundary you discovered.
    todo!("find a valid UTF-8 boundary and return a string slice")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_payload_bytes() {
        assert_eq!(payload_bytes(b"vault:deposit"), Some(&b"deposit"[..]));
        assert_eq!(payload_bytes(b"vault"), None);
    }

    #[test]
    fn respects_utf8_boundaries() {
        assert_eq!(safe_prefix("Solana", 3), "Sol");
        assert_eq!(safe_prefix("Accra 🇬🇭", 7), "Accra 🇬");
        assert_eq!(safe_prefix("rust", 20), "rust");
    }
}
