pub trait Storage {
    fn put(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<&str>;
}

#[derive(Debug, Default)]
pub struct MemoryStorage {
    entries: Vec<(String, String)>,
}

impl Storage for MemoryStorage {
    fn put(&mut self, _key: String, _value: String) {
        todo!("insert or replace by key")
    }

    fn get(&self, _key: &str) -> Option<&str> {
        todo!("return borrowed value if present")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_retrieves_values() {
        let mut store = MemoryStorage::default();
        store.put("user:1".into(), "online".into());
        assert_eq!(store.get("user:1"), Some("online"));
    }

    #[test]
    fn put_replaces_existing_value() {
        let mut store = MemoryStorage::default();
        store.put("user:1".into(), "offline".into());
        store.put("user:1".into(), "online".into());
        assert_eq!(store.get("user:1"), Some("online"));
    }
}
