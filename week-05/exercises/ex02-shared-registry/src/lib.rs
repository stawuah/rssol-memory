use std::sync::{Arc, Mutex};

pub fn record_node(shared: &Arc<Mutex<Vec<String>>>, node_id: &str) {
    todo!("push the node id into the shared registry")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_shared_state() {
        let shared = Arc::new(Mutex::new(Vec::new()));
        record_node(&shared, "node-a");
        assert_eq!(&*shared.lock().unwrap(), &vec!["node-a".to_string()]);
    }
}
