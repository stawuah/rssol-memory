pub fn encode_u32(value: u32) -> [u8; 4] {
    todo!("encode as little-endian bytes")
}

pub fn decode_u32(bytes: [u8; 4]) -> u32 {
    todo!("decode from little-endian bytes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_u32() {
        let bytes = encode_u32(0x0102_0304);
        assert_eq!(decode_u32(bytes), 0x0102_0304);
    }
}
