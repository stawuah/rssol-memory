#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Increment,
    Set(u32),
}

pub fn decode(input: &[u8]) -> Option<Instruction> {
    todo!("0 means Increment; 1 plus 4 trailing bytes means Set(u32 little-endian)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_increment() {
        assert_eq!(decode(&[0]), Some(Instruction::Increment));
    }

    #[test]
    fn decodes_set() {
        assert_eq!(decode(&[1, 5, 0, 0, 0]), Some(Instruction::Set(5)));
    }
}
