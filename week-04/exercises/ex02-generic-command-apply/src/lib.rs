pub trait CounterStore {
    fn add(&mut self, value: i64);
    fn total(&self) -> i64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Add(i64),
}

pub fn apply_commands<S: CounterStore>(store: &mut S, commands: &[Command]) {
    todo!("apply every command to the store")
}

#[derive(Debug, Default)]
pub struct SimpleCounter(i64);

impl CounterStore for SimpleCounter {
    fn add(&mut self, value: i64) {
        self.0 += value;
    }

    fn total(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_all_commands() {
        let mut store = SimpleCounter::default();
        apply_commands(&mut store, &[Command::Add(2), Command::Add(3), Command::Add(-1)]);
        assert_eq!(store.total(), 4);
    }
}
