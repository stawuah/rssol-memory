//! ex06 — A bounded multi-stage pipeline: producer -> N verifiers -> collector.

use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet {
    pub id: u64,
    pub signature_valid: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PipelineStats {
    pub received: u64,
    pub verified: u64,
    pub rejected: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineError {
    ZeroVerifiers,
}

/// The "expensive" stage. (Real code: ed25519 verification.)
pub fn verify(packet: &Packet) -> bool {
    packet.signature_valid
}

/// Take the next packet from a queue shared by several verifier threads.
/// The lock must be held ONLY while receiving, never while verifying.
pub fn next_packet(queue: &Mutex<Receiver<Packet>>) -> Option<Packet> {
    // Coding map:
    // 1. Lock the mutex, call recv(), and let the guard drop in the same statement.
    // 2. Err from recv means "all senders gone and queue empty" -> None.
    todo!("receive one packet under a short lock")
}

/// Run all `packets` through: producer thread -> bounded queue -> `verifiers`
/// threads -> bounded queue -> this thread (collector).
/// Returns stats and the sorted ids of verified packets.
/// `capacity` bounds BOTH queues (0 = rendezvous: every send waits for a recv).
pub fn run_pipeline(
    packets: Vec<Packet>,
    verifiers: usize,
    capacity: usize,
) -> Result<(PipelineStats, Vec<u64>), PipelineError> {
    // Coding map:
    // 1. Reject verifiers == 0.
    // 2. (in_tx, in_rx) = mpsc::sync_channel(capacity); same for (out_tx, out_rx)
    //    carrying (id, bool).
    // 3. Spawn a producer thread that MOVES packets and in_tx; it sends every packet.
    // 4. Wrap in_rx in Arc<Mutex<_>>. Spawn `verifiers` threads, each with a
    //    clone of that Arc and a clone of out_tx. Loop on next_packet, verify,
    //    send (id, ok).
    // 5. drop(out_tx) in this thread, or the collector loop below never ends.
    // 6. Collect from out_rx BEFORE joining the threads. With bounded queues,
    //    joining first deadlocks: the workers wait for space, and you wait for them.
    // 7. Join everything, sort the verified ids, return.
    todo!("wire producer, verifiers and collector with bounded channels")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn packets(n: u64) -> Vec<Packet> {
        (0..n).map(|id| Packet { id, signature_valid: id % 7 != 0 }).collect()
    }

    #[test]
    fn counts_are_exact_for_any_shape() {
        for verifiers in [1, 2, 8] {
            for capacity in [0, 1, 64] {
                let (stats, ids) = run_pipeline(packets(1_000), verifiers, capacity).unwrap();
                assert_eq!(stats.received, 1_000);
                assert_eq!(stats.rejected, 143);
                assert_eq!(stats.verified, 857);
                assert_eq!(ids.len(), 857);
                assert!(ids.windows(2).all(|w| w[0] < w[1]));
            }
        }
    }

    #[test]
    fn tiny_buffers_do_not_deadlock() {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || tx.send(run_pipeline(packets(20_000), 4, 1).unwrap().0).unwrap());
        let stats = rx.recv_timeout(Duration::from_secs(20)).expect("pipeline deadlocked");
        assert_eq!(stats.received, 20_000);
    }

    #[test]
    fn zero_verifiers_rejected() {
        assert_eq!(run_pipeline(packets(3), 0, 4), Err(PipelineError::ZeroVerifiers));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("an empty packet list must return zero stats promptly (no hang)")
    }
}
