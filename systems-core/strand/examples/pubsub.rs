//! BROADCAST / pub-sub: a slot leader announces new slots to every
//! subscriber. One subscriber is slow; it misses some, and nobody else waits.
//! Run: cargo run --example pubsub
use std::time::Duration;
use strand::*;

fn main() {
    let slots: Broadcast<u64> = Broadcast::new(4); // each inbox holds 4
    let stop = Stop::new();

    // Subscribe BEFORE publishing starts (you only get messages sent after).
    let rpc = slots.subscribe();
    let indexer = slots.subscribe();
    let slow_archiver = slots.subscribe();

    let listeners = [("rpc", rpc, 0u64), ("indexer", indexer, 0), ("slow archiver", slow_archiver, 15)]
        .into_iter()
        .map(|(name, inbox, delay_ms)| {
            std::thread::spawn(move || {
                let mut seen = Vec::new();
                for slot in inbox {
                    seen.push(slot);
                    std::thread::sleep(Duration::from_millis(delay_ms));
                }
                (name, seen)
            })
        })
        .collect::<Vec<_>>();

    // Publisher: one new slot every 2 ms (think 400 ms on mainnet).
    let publisher = spawn!([slots, stop], {
        for slot in 1..=50u64 {
            slots.publish(slot);
            if stop.sleep(Duration::from_millis(2)) {
                break;
            }
        }
    });
    publisher.wait();
    let lagged = slots.lagged();
    drop(slots); // closing the broadcast ends every subscriber's loop

    for l in listeners {
        let (name, seen) = l.join().unwrap();
        println!("{name:>14}: got {:>2} slots, e.g. {:?}", seen.len(), &seen[..seen.len().min(8)]);
    }
    println!("messages dropped for slow subscribers: {lagged}");
}
