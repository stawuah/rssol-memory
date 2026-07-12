pub fn first_word(text: &str) -> &str {
    // Search for the first space, use the full length if absent, then borrow the prefix.
    &text[..text.find(' ').unwrap_or(text.len())]
}

pub fn prefix_until(text: &str, delimiter: char) -> &str {
    // Search -> choose delimiter index or text length -> return a zero-copy slice.
    &text[..text.find(delimiter).unwrap_or(text.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_first_word() {
        assert_eq!(first_word("anchor brings order"), "anchor");
    }

    #[test]
    fn returns_prefix_before_delimiter() {
        assert_eq!(prefix_until("token2022:confidential", ':'), "token2022");
    }
}
