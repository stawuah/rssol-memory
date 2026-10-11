//! ex08 — A long-running service loop: work, timer ticks, and graceful shutdown.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitReason {
    /// The stop flag was set; queued work was drained first.
    Stopped,
    /// Every sender was dropped and the queue was empty.
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopReport {
    pub processed: u64,
    pub ticks: u64,
    pub drained_on_shutdown: u64,
    pub exit: ExitReason,
}

/// Run until stopped or disconnected.
/// * Each received item goes to `on_item`.
/// * If nothing arrives for `tick`, count a tick (heartbeats, metrics, flushes).
/// * When `stop` is observed: hand every ALREADY-QUEUED item to `on_item`
///   (count them as drained_on_shutdown), then exit with Stopped.
pub fn run_service(
    rx: Receiver<u64>,
    stop: &AtomicBool,
    tick: Duration,
    mut on_item: impl FnMut(u64),
) -> LoopReport {
    // Coding map:
    // 1. Keep processed / ticks counters.
    // 2. Loop:
    //    a. If stop.load(Acquire): drain with try_recv() until it fails,
    //       calling on_item and counting; return Stopped.
    //    b. match rx.recv_timeout(tick):
    //         Ok(v)            -> on_item(v), processed += 1
    //         Err(Timeout)     -> ticks += 1   (this also re-checks stop soon)
    //         Err(Disconnected)-> return Disconnected
    // 3. Never block forever: recv() without timeout would ignore `stop`.
    todo!("service loop with recv_timeout ticks and a draining shutdown")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Instant;

    #[test]
    fn disconnect_ends_after_all_items() {
        let (tx, rx) = mpsc::channel();
        for i in 0..50 {
            tx.send(i).unwrap();
        }
        drop(tx);
        let mut sum = 0;
        let r = run_service(rx, &AtomicBool::new(false), Duration::from_millis(10), |v| sum += v);
        assert_eq!((r.processed, r.exit), (50, ExitReason::Disconnected));
        assert_eq!(sum, (0..50).sum());
    }

    #[test]
    fn ticks_while_idle_and_stops_promptly() {
        let (_tx, rx) = mpsc::channel::<u64>();
        let stop = AtomicBool::new(false);
        let t0 = Instant::now();
        let r = std::thread::scope(|s| {
            s.spawn(|| {
                std::thread::sleep(Duration::from_millis(80));
                stop.store(true, Ordering::Release);
            });
            run_service(rx, &stop, Duration::from_millis(10), |_| {})
        });
        assert_eq!(r.exit, ExitReason::Stopped);
        assert!(r.ticks >= 3, "ticks = {}", r.ticks);
        assert!(t0.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn queued_work_is_drained_not_lost() {
        let (tx, rx) = mpsc::channel();
        for i in 0..20 {
            tx.send(i).unwrap();
        }
        let stop = AtomicBool::new(true);
        let mut seen = Vec::new();
        let r = run_service(rx, &stop, Duration::from_millis(10), |v| seen.push(v));
        assert_eq!((r.drained_on_shutdown, r.processed, r.exit), (20, 0, ExitReason::Stopped));
        assert_eq!(seen, (0..20).collect::<Vec<_>>());
        drop(tx);
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("with a 10 s tick and stop already set, assert run_service returns in well under 1 s")
    }
}
