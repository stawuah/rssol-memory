# Systems Core

`systems-core` is the **off-chain and validator-side** track. `protocol-core`
teaches you to reason like the engineer responsible for the money *inside* a
program. `systems-core` teaches you to reason like the engineer responsible for
the **machines, networks and processes** that run it: concurrency, durability,
failure, consensus, and the internals of a Solana validator.

It follows the same format as the other tracks: a README per exercise,
`todo!()` stubs with numbered coding maps, tests that encode the invariants, and
one learner-written adversarial test per exercise (marked `#[ignore]`).

Two companion documents:

- **[SIGNATURES.md](./SIGNATURES.md):** the blueprint. The function signatures
  that matter, what each part of a signature tells you, and when to use or avoid
  each API.
- **[CAPSTONES.md](./CAPSTONES.md):** six senior-level projects.

A hands-on lab crate, **[`strand`](./strand/README.md)**, rebuilds `Arc`,
`Mutex`, `Condvar`, `RwLock`, channels, `select!`, a pool and a tiny async
executor from atomics, with runnable examples. Use it alongside Modules 1–3
whenever you want to see *inside* a primitive.

---

## The roadmap: junior → mid → senior → staff

Levels are defined by **what you can be trusted with**, not by years.

| Level | You can be trusted to… | Modules | Evidence you can show |
|---|---|---|---|
| **Junior** | write multithreaded Rust that compiles for the right reasons: no data races, no deadlocks, no lost panics. Explain `Send`, `Sync`, `'static`. | 1, 2 | ex01–ex05 green + learner tests; explain each lock order and memory ordering out loud |
| **Mid** | design a service: pipelines and actors with bounded queues, clean shutdown, durable writes, batching, replay protection. | 3, 4 | ex06–ex11; a small service with a WAL that survives `kill -9` |
| **Senior** | own a component in a distributed system: reason about partial failure, time, retries, idempotency, quorums and fork choice; build async services with deadlines everywhere. | 5, 6, 8 | ex12–ex17, ex23–ex24; capstone S2 or S5 with a design doc and failure-mode table |
| **Senior → Staff (protocol)** | change validator-level machinery safely: scheduling, leader schedule, propagation, gossip, PoH; measure performance; lead an incident; defend a design against adversarial review. | 7 + capstones | ex18–ex22; capstone S1 or S6; a merged PR or review in Agave/Firedancer/Jito/an infra repo |

Senior is also more than code. It means writing design docs that name the
invariants and failure modes, running load tests with numbers, writing runbooks
(see protocol-core Module F), reviewing other people's concurrency code, and
reading unfamiliar production code (Agave) without getting lost.

---

## How to study

Same rules as the other tracks:

1. Read the exercise README completely.
2. Do the "explain before continuing" part in your own words **before** coding.
3. Read the tests and predict every assertion.
4. Implement one `todo!()` at a time; `cargo test -p <package>`.
5. Remove `#[ignore]` from the learner test and write it.
6. Look up every API you used in [SIGNATURES.md](./SIGNATURES.md) and say
   why it was the right choice, and when it wouldn't be.
7. Don't continue until you can explain the invariant without code.

```sh
cd systems-core
cargo test -p ex01-scoped-parallel-sum
```

**Prerequisite:** `rustlings-core` fully passing. `systems-core` can run **in
parallel** with `anchor-core`/`protocol-core`; the weekly plan below
interleaves them.

---

## Modules

| Module | Level | Exercises | Theme |
|---|---|---|---|
| [1 — Threads and ownership](./modules/module-1-threads-ownership/README.md) | Junior | ex01–ex02 | scoped vs owned threads, `Send`/`'static`, panics |
| [2 — Shared state](./modules/module-2-shared-state/README.md) | Junior → Mid | ex03–ex05 | lock ordering, atomics & ordering, snapshot swapping |
| [3 — Message passing](./modules/module-3-message-passing/README.md) | Mid | ex06–ex08 | pipelines, actors, graceful shutdown |
| [4 — Durability and batching](./modules/module-4-durability-batching/README.md) | Mid → Senior | ex09–ex11 | WAL, group commit, replay protection |
| [5 — Time, ordering, failure](./modules/module-5-time-ordering-failure/README.md) | Senior | ex12–ex14 | logical clocks, failure detection, retries |
| [6 — Consensus](./modules/module-6-consensus/README.md) | Senior | ex15–ex17 | BFT quorums, Raft election, fork choice |
| [7 — Validator internals](./modules/module-7-validator-internals/README.md) | Senior → Staff | ex18–ex22 | Sealevel, leader schedule, Turbine, gossip, PoH |
| [8 — Async with tokio](./modules/module-8-async-tokio/README.md) | Senior | ex23–ex24 | tokio actors, bounded fan-out |

### All exercises

| # | Exercise | Invariant the tests enforce |
|---|---|---|
| ex01 | scoped parallel sum | same result for any worker count; overflow never silent |
| ex02 | join and panics | all jobs run concurrently; every panic is reported, in order |
| ex03 | ledger lock ordering | money conserved; no deadlock under opposite-direction load |
| ex04 | atomics and publication | the true max wins; exactly one publisher; readers see its value |
| ex05 | epoch snapshot | no torn snapshot; readers never block the writer |
| ex06 | bounded pipeline | exact counts for any shape; no deadlock at capacity 0/1 |
| ex07 | actor + oneshot | no overdraw under 25 concurrent withdrawals; dead actor → error |
| ex08 | shutdown loop | stop is noticed within one tick; queued work drained |
| ex09 | WAL framing | crash at any byte → every complete record kept, tail cut |
| ex10 | group commit | far fewer syncs than writes; order kept; batch bounds respected |
| ex11 | replay protection | a transaction is accepted exactly once; memory bounded forever |
| ex12 | logical clocks | causality preserved; concurrency detected |
| ex13 | failure detector | exact thresholds; reordered heartbeats harmless |
| ex14 | retry & backoff | capped, jittered, budgeted; permanent errors not retried |
| ex15 | stake quorum | > 2/3 of *total* stake; duplicates once; equivocation caught |
| ex16 | Raft election | one vote per term; stale logs can't win; higher term steps down |
| ex17 | fork choice | heaviest subtree by latest votes; deterministic ties |
| ex18 | account-lock scheduler | batched execution ≡ sequential execution |
| ex19 | leader schedule | identical on every machine; frequency ∝ stake |
| ex20 | Turbine tree | everyone receives each shred exactly once; ~log_k(n) hops |
| ex21 | gossip CRDT | any merge order converges; future timestamps rejected |
| ex22 | PoH verification | parallel verification ≡ sequential; any tampering detected |
| ex23 | tokio actor | every request answers or times out; clean exits |
| ex24 | bounded fan-out | ≤ N in flight; per-call timeouts; panics isolated |

---

## A 16-week plan (about 10–12 h/week)

Each week pairs a `systems-core` block with reading. The protocol-core column
keeps your on-chain skills moving in parallel.

| Week | systems-core | Read | protocol-core in parallel |
|---|---|---|---|
| 1 | ex01–ex02; strand `jobs`, `std_basics` | *Rust Atomics and Locks* (RA&L) ch. 1 | anchor-core lessons 1–3 |
| 2 | ex03–ex04; strand `counter` | RA&L ch. 2–3 (atomics, memory ordering) | anchor-core lessons 4–6 |
| 3 | ex05; strand `routing_table` | RA&L ch. 4, 9 (locks) | anchor-core lessons 7–10 |
| 4 | ex06–ex07; strand `pipeline`, `actor` | RA&L ch. 5 (channels) | Module A ex01–ex03 |
| 5 | ex08; strand `stream` | tokio tutorial (tokio.rs), sections up to "Channels" | Module A ex04–ex06 |
| 6 | ex09–ex10; strand `wal` | *Designing Data-Intensive Applications* (DDIA) ch. 3, 7 | Module A ex07–ex08 |
| 7 | ex11 | Solana docs: transactions, recent blockhash, durable nonces | Module B |
| 8 | **checkpoint:** a mini service (actor + WAL + graceful shutdown) | DDIA ch. 8 (trouble with distributed systems) | Module C |
| 9 | ex12–ex13 | Lamport, *Time, Clocks, and the Ordering of Events* (1978) | Module D |
| 10 | ex14; ex23 | *Rust for Rustaceans* ch. on async; tokio `select!`/`timeout` docs | Module E |
| 11 | ex15–ex16 | Raft paper (Ongaro & Ousterhout, 2014); DDIA ch. 9 | Module F |
| 12 | ex17 | Solana docs on Tower BFT and commitment levels; Alpenglow proposal (check its current status) | Module G |
| 13 | ex18; ex24 | Agave: banking-stage scheduler (search `prio_graph_scheduler.rs`) | Module H |
| 14 | ex19–ex20 | Solana whitepaper (PoH), Turbine docs; Agave `cluster_nodes.rs` | Module I |
| 15 | ex21–ex22 | Agave `crds.rs`, `entry.rs`; PBFT paper (Castro & Liskov, 1999) | Module J |
| 16 | start a capstone (S1, S5 or S6) | your own design doc | one protocol-core capstone |

---

## Interview questions by level

**Junior**
- Why does `thread::spawn` need `F: Send + 'static`? What's the difference from `thread::scope`?
- `Rc` vs `Arc`; `RefCell` vs `Mutex`. What do `Send` and `Sync` mean?
- What happens to a panic in a thread whose `JoinHandle` you dropped?

**Mid**
- Design a pipeline for 100k msgs/s where one stage is 10× slower. Where do you put bounds, and what happens on overload?
- How does a service shut down without losing acknowledged work?
- What is group commit and what does it trade?
- How does Solana stop a transaction from executing twice, with bounded memory?

**Senior**
- Why can't you order events across machines with wall clocks? What are Lamport/vector clocks and PoH for?
- Prove that two conflicting blocks can't both get > 2/3 stake with < 1/3 Byzantine.
- Raft: how do terms and the up-to-date log rule prevent split brain and data loss?
- Design retries for transaction submission. What is retryable, and what's the budget?
- In tokio, when do you use `spawn`, `spawn_blocking`, or rayon? What's wrong with a std `MutexGuard` across `.await`?

**Senior → Staff (protocol)**
- Walk a transaction from client to finalization, naming every stage and its concurrency model.
- Why do hot accounts limit throughput, and how do local fee markets respond?
- How are the leader schedule and the Turbine tree made deterministic across 2,000 machines?
- What does PoH prove and not prove? Why is verification parallel?
- A validator falls behind during congestion. How do you find out whether it's replay, networking, or disk?

---

## How this connects to the rest of the repository

| systems-core | connects to |
|---|---|
| ex03 lock ordering, ex18 account locks | anchor-core ex04 CPI router; protocol-core ex22 account locking |
| ex11 replay protection | protocol-core Module C (ordering), ex31 compute & priority fees |
| ex14 retries | protocol-core ex30 logs & simulation, ex32 mainnet runbook |
| ex15 quorums | protocol-core ex26 multisig admin (the same threshold math) |
| ex18 scheduler | protocol-core ex47 batching & address lookup tables (how many accounts a tx can lock) |
| ex22 PoH, Module 7 | protocol-core ex31 (why fees and compute limits exist at the cluster level) |
