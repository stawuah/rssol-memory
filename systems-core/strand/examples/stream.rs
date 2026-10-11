//! An ENDLESS stream through a SMALL buffer.
//! Run: cargo run --release --example stream
//!
//! Answers: "bounded means I know how many items come, right?" No.
//! `bounded 100` = at most 100 WAITING at once. Items can come forever.
//!
//! Setup: transactions arrive forever (faster than we can process them).
//! We run two strategies for 1 second each:
//!   A) backpressure: producer WAITS when the buffer is full (nothing lost,
//!      producer slowed down to our speed)
//!   B) load shedding: producer DROPS when full (producer never slows,
//!      but some items are lost; count them!)
//! The consumer processes in batches and also flushes on a timer, so a
//! half-full batch never sits around forever.
use std::time::{Duration, Instant};
use strand::*;

fn run(name: &str, shed_load: bool) {
    chan!(txs: u64, bounded 100);
    let stop = Stop::new();

    // PRODUCER: an infinite loop. It only ends because `stop` fires.
    let producer = spawn!([txs, stop], {
        let (mut made, mut dropped) = (0u64, 0u64);
        while !stop.is_fired() {
            made += 1;
            if shed_load {
                if let Err(TrySendError::Full(_)) = txs.try_send(made) {
                    dropped += 1; // B: full? throw it away and move on
                }
            } else if txs.send(made).is_err() {
                break; // A: full? wait here. (Err = consumer is gone)
            }
        }
        (made, dropped)
    });

    // CONSUMER: also infinite, until stop. Batch up to 50, or flush every 5 ms.
    let rx = txs.into_receiver();
    let flush = Ticker::every(Duration::from_millis(5));
    let (mut batch, mut processed, mut flushes, mut max_waiting) = (Vec::new(), 0u64, 0u64, 0);
    let started = Instant::now();
    loop {
        select! {
            recv(rx) -> tx => match tx {
                Some(tx) => {
                    std::thread::sleep(Duration::from_micros(2)); // "work" per item
                    batch.push(tx);
                    max_waiting = max_waiting.max(rx.len());
                    if batch.len() == 50 {
                        processed += batch.len() as u64;
                        flushes += 1;
                        batch.clear();
                    }
                }
                None => break,
            },
            recv(flush) -> _ => {
                processed += batch.len() as u64; // timer flush: don't let items sit
                flushes += 1;
                batch.clear();
            },
            recv(stop) -> _ => break,
        }
        if started.elapsed() > Duration::from_secs(1) {
            stop.fire(); // demo ends after 1 s; in a real node: Ctrl-C / shutdown RPC
        }
    }
    drop(rx); // wake a producer blocked in send, so it can see we're gone
    let (made, dropped) = producer.wait()[0];
    println!("{name}");
    println!("  produced {made:>9}, processed {processed:>7}, dropped {dropped:>9}, flushes {flushes}");
    println!("  most ever waiting in the buffer: {max_waiting} (never more than 100)");
    println!("  (produced - processed - dropped = what was still in the buffer when we stopped)\n");
}

fn main() {
    run("A) backpressure (send waits)", false);
    run("B) load shedding (try_send drops)", true);
    println!("A kept memory flat by slowing the producer. B kept the producer fast by losing data.");
    println!("Pick per use case: a WAL must use A. Metrics/gossip often use B.");
}
