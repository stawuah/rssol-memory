//! Read-mostly data: many threads look up routes, one thread updates them.
//! Run: cargo run --example routing_table
use std::collections::HashMap;
use std::time::Duration;
use strand::*;

fn main() {
    read_mostly!(routes = HashMap::from([("validator-1".to_string(), "10.0.0.1:8001".to_string())]));

    // 4 readers: lots of concurrent lookups, none block each other.
    let readers = spawn!(4, [routes], |id| {
        let mut seen = 0;
        for _ in 0..50 {
            let table = routes.read(); // shared read lock
            seen = seen.max(table.len());
            drop(table); // release before sleeping: never sleep holding a lock
            std::thread::sleep(Duration::from_millis(1));
        }
        (id, seen)
    });

    // 1 writer: occasionally adds a peer (exclusive lock, briefly).
    spawn!([routes], {
        for i in 2..=5 {
            std::thread::sleep(Duration::from_millis(8));
            routes.write().insert(format!("validator-{i}"), format!("10.0.0.{i}:8001"));
        }
    });

    for (id, seen) in readers.wait() {
        println!("reader {id} saw up to {seen} routes");
    }
    println!("final table: {} routes", routes.read().len());
}
