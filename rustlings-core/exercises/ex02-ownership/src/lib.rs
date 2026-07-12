pub fn owned_length(text: String) -> usize {
    // `text` was moved into this function. Read its length before it is dropped.
    text.len()
}

pub fn borrowed_length(text: &String) -> usize {
    // `text` is borrowed. Read through the reference without cloning or moving it.
    text.len()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_value_is_consumed() {
        let text = String::from("anchor");
        assert_eq!(owned_length(text), 6);
    }

    #[test]
    fn borrowed_value_stays_usable() {
        let text = String::from("solana");
        assert_eq!(borrowed_length(&text), 6);
        assert_eq!(text, "solana");
    }
}
