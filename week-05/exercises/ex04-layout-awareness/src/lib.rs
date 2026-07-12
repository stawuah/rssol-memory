pub fn bytes_for_u64_array(len: usize) -> usize {
    todo!("return the number of bytes needed for len u64 values")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_array_bytes() {
        assert_eq!(bytes_for_u64_array(0), 0);
        assert_eq!(bytes_for_u64_array(4), 32);
    }
}
