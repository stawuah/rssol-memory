#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    pub node_id: String,
    pub tick: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeStatus {
    pub node_id: String,
    pub last_seen_tick: u64,
    pub is_healthy: bool,
}

pub fn summarize_cluster(
    _heartbeats: &[Heartbeat],
    _current_tick: u64,
    _max_staleness: u64,
) -> Vec<NodeStatus> {
    todo!("group by node, keep the most recent tick, and mark health by staleness")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_latest_heartbeat_per_node() {
        let heartbeats = vec![
            Heartbeat {
                node_id: "node-a".into(),
                tick: 10,
            },
            Heartbeat {
                node_id: "node-b".into(),
                tick: 8,
            },
            Heartbeat {
                node_id: "node-a".into(),
                tick: 14,
            },
        ];

        let statuses = summarize_cluster(&heartbeats, 15, 3);
        assert_eq!(statuses.len(), 2);
        assert!(statuses.iter().any(|status| {
            status.node_id == "node-a" && status.last_seen_tick == 14 && status.is_healthy
        }));
        assert!(statuses.iter().any(|status| {
            status.node_id == "node-b" && status.last_seen_tick == 8 && !status.is_healthy
        }));
    }
}
