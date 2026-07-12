use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Counter {
    value: RefCell<u64>,
}

impl Counter {
    pub fn increment(&self) {
        todo!("mutate the inner value through RefCell")
    }

    pub fn get(&self) -> u64 {
        todo!("read the inner value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_through_shared_reference() {
        let counter = Counter::default();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }
}
