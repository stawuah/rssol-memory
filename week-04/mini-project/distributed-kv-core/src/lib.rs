pub trait KeyValueStore {
    fn set(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<&str>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set { key: String, value: String },
}

#[derive(Debug, Default)]
pub struct MemoryKv {
    entries: Vec<(String, String)>,
}

impl KeyValueStore for MemoryKv {
    fn set(&mut self, _key: String, _value: String) {
        todo!("insert or replace")
    }

    fn get(&self, _key: &str) -> Option<&str> {
        todo!("return borrowed value if present")
    }
}

pub fn apply(store: &mut impl KeyValueStore, commands: &[Command]) {
    todo!("apply all commands in order")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_populates_store() {
        let mut store = MemoryKv::default();
        apply(
            &mut store,
            &[Command::Set {
                key: "a".into(),
                value: "1".into(),
            }],
        );
        assert_eq!(store.get("a"), Some("1"));
    }
}
