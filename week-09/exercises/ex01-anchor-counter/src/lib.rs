#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counter {
    pub value: u64,
}

pub fn increment(counter: &mut Counter) {
    todo!("increment the counter, representing a simple Anchor-style handler body")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_counter() {
        let mut counter = Counter { value: 1 };
        increment(&mut counter);
        assert_eq!(counter.value, 2);
    }
}
