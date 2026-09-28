# Module 7 — Validator Internals

Level: **Senior → Staff**. Miniature versions of the machinery inside a
Solana validator. Each is small enough to finish in an evening and faithful
enough that the real code (Agave, Firedancer) becomes readable afterwards.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex18 | read/write-set conflict scheduling in O(accounts) | Sealevel / banking-stage scheduler, hot accounts |
| ex19 | canonical ordering, seeded PRNG, weighted sampling | leader schedule, Gulf Stream forwarding |
| ex20 | seeded shuffle, k-ary tree index math | Turbine shred propagation |
| ex21 | LWW CRDT, deterministic ties, drift guard | gossip CRDS |
| ex22 | hash chains, parallel verification with scoped threads | Proof of History entry verification |

## Ordered exercises

1. [Account-lock scheduler](./exercises/ex18-account-lock-scheduler/README.md)
2. [Leader schedule](./exercises/ex19-leader-schedule/README.md)
3. [Turbine tree](./exercises/ex20-turbine-tree/README.md)
4. [Gossip CRDT](./exercises/ex21-gossip-crdt/README.md)
5. [PoH verification](./exercises/ex22-poh-verify/README.md)

## Coverage checklist

- [x] parallel execution equivalent to sequential execution
- [x] cluster-wide determinism (no HashMap order, no thread_rng, no floats)
- [x] log-depth fanout with rotation per shred
- [x] commutative/associative/idempotent merge
- [x] sequential generation vs parallel verification

## Read the real thing afterwards

In the [Agave repository](https://github.com/anza-xyz/agave), search by file
name (paths move between versions): `prio_graph_scheduler.rs` / `account_locks.rs`,
`leader_schedule.rs`, `cluster_nodes.rs`, `crds.rs` / `crds_value.rs`,
`entry.rs` (PoH verification), `heaviest_subtree_fork_choice.rs`,
`status_cache.rs`.

**Common interview question:** Walk a transaction from the moment a client
sends it to the moment it is finalized, naming each validator stage and the
concurrency primitive or protocol that stage relies on.
