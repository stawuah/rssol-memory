//! A PIPELINE with fan-out and fan-in, shaped like a Solana validator's
//! transaction path (fetch -> sigverify -> banking), heavily simplified.
//! Run: cargo run --release --example pipeline
//!
//! ```text
//!  fetch ──► [q1] ──► verify ×4 ──► [q2] ──► execute ──► stats
//!   1 thread         (fan-out)      (fan-in)  1 thread (owns the state)
//! ```
//! Each arrow is a bounded channel. If "execute" is slow, q2 fills, the
//! verifiers wait, q1 fills, fetch waits. Backpressure flows upstream
//! automatically, no coordination code needed.
use std::collections::HashMap;
use strand::*;

#[derive(Clone, Debug)]
struct Tx {
    id: u64,
    from: u32,
    to: u32,
    amount: u64,
    signature_ok: bool,
}

fn main() {
    chan!(fetched: Tx, bounded 64);
    chan!(verified: Tx, bounded 64);

    // Stage 1: fetch (pretend packets arrive from the network).
    let _fetch = spawn!([fetched], {
        for id in 0..20_000u64 {
            let bad = id % 97 == 0; // some forged transactions
            send!(fetched, Tx { id, from: (id % 10) as u32, to: ((id + 3) % 10) as u32, amount: 1, signature_ok: !bad });
        }
    }); // fetch ends -> its Sender drops -> q1 closes when drained

    // Stage 2: 4 verifiers share q1 (fan-out) and all feed q2 (fan-in).
    shared!(q1 = fetched.into_receiver());
    let _verify = spawn!(4, [q1, verified], |_worker| {
        loop {
            let next = lock!(q1 => q { q.recv() });
            let Ok(tx) = next else { break };
            // "expensive" signature check
            let mut h = tx.id;
            for _ in 0..200 {
                h = h.wrapping_mul(6364136223846793005).wrapping_add(1);
            }
            std::hint::black_box(h);
            if tx.signature_ok {
                send!(verified, tx);
            }
        }
    });

    // Stage 3: ONE executor owns the balances. No lock needed: single owner.
    let mut balances: HashMap<u32, i64> = (0..10).map(|a| (a, 1_000)).collect();
    let (mut executed, mut rejected) = (0, 0);
    for tx in verified {
        let from = balances[&tx.from];
        if from >= tx.amount as i64 {
            *balances.get_mut(&tx.from).unwrap() -= tx.amount as i64;
            *balances.get_mut(&tx.to).unwrap() += tx.amount as i64;
            executed += 1;
        } else {
            rejected += 1; // insufficient funds
        }
    }
    let total: i64 = balances.values().sum();
    println!("executed {executed}, rejected {rejected}, forged dropped {}", 20_000 - executed - rejected);
    println!("total money still {total} (conservation check: must be 10000)");
    assert_eq!(total, 10_000);
}
