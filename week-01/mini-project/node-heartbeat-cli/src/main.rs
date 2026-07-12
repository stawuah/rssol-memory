use node_heartbeat_cli::{Heartbeat, summarize_cluster};

fn main() {
    let heartbeats = vec![
        Heartbeat {
            node_id: "node-a".into(),
            tick: 10,
        },
        Heartbeat {
            node_id: "node-b".into(),
            tick: 7,
        },
        Heartbeat {
            node_id: "node-a".into(),
            tick: 12,
        },
    ];

    let statuses = summarize_cluster(&heartbeats, 12, 2);
    println!("{statuses:#?}");
}
