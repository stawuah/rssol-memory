# Systems Core Capstones

Each capstone is a real project you can show. Pick one after Module 6, and a
second after Module 7.

**Every capstone must include:**

- a **design doc** (1–3 pages): components, the concurrency model of each
  (thread, task, actor), channel bounds, the lock order if any;
- an **invariants list**: what must always be true, and which test proves it;
- a **failure-mode table**: for each dependency, what happens when it's slow,
  down, or lying, and what the system does;
- **shutdown behavior**: what is drained, flushed, and joined, in what order;
- **numbers**: a load test with throughput and p50/p99 latency, and where the
  bottleneck is;
- a short **runbook**: how an operator knows it's unhealthy and what to do.

---

## S1 — Mini TPU (transaction ingest pipeline)

Build a transaction-processing pipeline: UDP or QUIC ingest (tokio) → dedup →
signature verification on a rayon pool (`ed25519-dalek`) → account-lock
scheduler (ex18) → parallel executor over a simple accounts map → a results
stream.

- Bounded queues between stages with explicit overload behavior (drop or backpressure, per stage).
- Metrics per stage (queue depth, throughput, drops).
- Load test: find the bottleneck and explain it.
- Stretch: priority by fee per compute unit under contention.

Uses: ex06, ex10, ex11, ex18, ex24.

## S2 — Raft-replicated key-value store

Extend ex16 into full Raft: log replication, commit index, applying to a state
machine, persistence through your WAL (ex09) with group commit (ex10), and
snapshots/log compaction.

- A **deterministic simulation** test harness: N nodes in one process, a fake
  network (delay, drop, reorder, partition) and a fake clock, driven by a seed.
  A failing seed reproduces exactly.
- Invariant checks after every step: election safety, log matching, leader
  completeness, state machine safety.

Uses: ex09, ex10, ex12, ex13, ex14, ex16.

## S3 — Gossip network

Extend ex21 into a working gossip protocol over UDP: signed values (ed25519),
push to random peers, pull with a digest (bloom filter or hash list), pruning,
and failure detection (ex13).

- Measure convergence time vs cluster size and fanout.
- Simulate a partition and heal it; show the replicas converge.
- Show that forged, unsigned, and far-future values are rejected.

Uses: ex12, ex13, ex20, ex21.

## S4 — Fork-aware indexer

Consume account or transaction updates (Yellowstone gRPC / a Geyser stream, or
a simulator you write that emits forks) into a database.

- Handle commitment levels: data at `processed` can be rolled back, so track
  slots and parents and undo orphaned forks (ex17).
- Bounded pipeline with batched writes (ex10) and exactly-once effects through
  slot checkpoints (ex11 thinking).
- Restart safety: resume from the last committed checkpoint with no gaps and
  no duplicates.

Uses: ex06, ex08, ex10, ex11, ex17, ex23.

## S5 — Transaction sender service

A service that lands transactions reliably under congestion:

- Fetch the leader schedule (ex19) and forward to current and upcoming leaders.
- Retries with backoff and a budget (ex14), stopping at blockhash expiry (ex11).
- A priority-fee strategy based on recent fees for the accounts the transaction
  writes (local fee markets, ex18).
- Measure landing rate and time-to-confirmation, and compare strategies.

Uses: ex11, ex14, ex18, ex19, ex23, ex24.

## S6 — Consensus simulator

Simulate N validators with stake, a leader schedule (ex19), blocks forming a
fork tree (ex17), votes with > 2/3 thresholds (ex15), network delay and
partitions, and some Byzantine behaviors (equivocating leader, withheld votes).

- Safety invariant: no two conflicting blocks ever both reach the finality
  threshold, across thousands of random seeds.
- Liveness measurement: time to finality vs. network delay and % offline stake.
- Write up what breaks at exactly 1/3 Byzantine stake, and why.

Uses: ex12, ex15, ex17, ex19, ex20, ex22.
