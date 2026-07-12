#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeState {
    pub node_id: String,
    pub term: u64,
}

pub fn encode(state: &NodeState) -> String {
    todo!("encode as `node_id:term`")
}

pub fn decode(input: &str) -> Option<NodeState> {
    todo!("decode from `node_id:term`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_state() {
        let state = NodeState {
            node_id: "node-a".into(),
            term: 7,
        };
        assert_eq!(decode(&encode(&state)), Some(state));
    }
}
