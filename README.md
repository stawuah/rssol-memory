# Rust-to-Solana Memory

A hands-on learning path for building the Rust knowledge, Solana mental
models, and security habits needed to design, build, and ship production-grade
on-chain programs with Anchor — from first principles up through the
economic, security, and operational judgment expected of a mid-to-senior
protocol engineer.

The repository teaches concepts across four connected stages:

1. `rustlings-core` develops practical Rust fluency through small test-driven
   exercises.
2. `anchor-core` translates those Rust concepts into Solana accounts, state
   transitions, validation, PDAs, CPIs, account sizing, errors, and compute.
3. `protocol-core` builds on Anchor Core to reach production readiness —
   economic modeling, oracle integration, MEV resistance, cross-program
   security, governance, deployment operations, security review methodology,
   fuzz testing, zero-knowledge proofs, and optimization.
4. `systems-core` covers the off-chain and validator side — concurrency,
   message passing, durability, failure handling, consensus, Solana validator
   internals, and async Rust with tokio — taking a learner from junior to
   senior protocol engineer.

The goal is not to memorize framework syntax. It is to understand what the
code does, why the Solana runtime requires each check, why a protocol's
economics can break even when its code compiles cleanly, and how a system
remains correct — and solvent — when callers, markets, and oracles all behave
adversarially.

## Learning tracks

### Rustlings Core

[`rustlings-core`](rustlings-core/README.md) contains focused Rust exercises
with tests, a local README in every exercise folder, and numbered
implementation guidance beside each `todo!()`.

Topics include:

- bindings, mutability, ownership, and borrowing;
- string slices, byte slices, and UTF-8 boundaries;
- structs, methods, enums, and exhaustive state transitions;
- `Option`, `Result`, custom errors, and `?`;
- traits, associated types, generics, and trait bounds;
- bounded vectors, maps, and duplicate detection;
- lazy iterators and checked aggregation;
- lifetimes and borrowed contexts;
- checked arithmetic, wide intermediates, fixed-point units, and rounding.

These concepts are taught with protocol-oriented examples such as vaults,
positions, markets, transfers, authority relationships, capacity limits, and
balance accounting.

### Anchor Core

[`anchor-core`](anchor-core/README.md) maps Rust reasoning to simplified
Solana and Anchor models. Every exercise also includes its own README and
coding steps in the source.

Topics include:

- programs versus persistent state accounts;
- account owners, signers, authorities, and writable accounts;
- deterministic PDA identity and seed design;
- cross-program invocation boundaries;
- Anchor-style account validation;
- privacy properties and metadata leakage;
- lifetimes inside account contexts;
- serialization and account-space calculations;
- protocol error codes and transaction failure;
- compute budgets and worst-case execution.

The exercises use small Rust models before introducing full framework
machinery. This keeps the runtime behavior and security purpose of each
Anchor constraint visible.

### Protocol Core

[`protocol-core`](protocol-core/README.md) is where the curriculum moves from
"writing correct Anchor code" to "reasoning like the engineer responsible for
real money." Every exercise still follows the same `todo!()` + README + test
pattern, but each module is anchored to a real exploit class or production
failure mode, not just an API surface.

Topics include:

- fixed-point economic modeling: rounding direction, share-based vault
  accounting, fee accrual, slippage, interest curves, liquidation math, and
  overcollateralized stablecoin mechanics;
- oracle integration: confidence intervals, staleness defense, TWAP vs spot
  pricing, and multi-oracle aggregation;
- MEV and ordering: sandwich attacks, commit-reveal patterns, confidential
  transfers, and encrypted trigger orders;
- cross-program security: arbitrary CPI validation, multi-hop CPI privilege
  tracking, and account-substitution exploits;
- upgrade authority and governance: multisig gating, timelocks, and the
  immutability/flexibility tradeoff;
- deployment and operations: devnet/mainnet pipelines, IDL versioning,
  transaction failure monitoring, and compute budget tuning;
- security review methodology: reading real audit reports and conducting a
  mock audit of an existing open-source program;
- fuzz testing on-chain programs with Trident;
- zero-knowledge proofs: prover/verifier intuition, ZK compression, on-chain
  proof verification, and range proofs in confidential transfers;
- optimization: account layout, zero-copy deserialization, compute unit
  profiling, and instruction batching.

### Systems Core

[`systems-core`](systems-core/README.md) is the track for everything outside
the program: the threads, queues, disks, networks and consensus rules that
production Solana infrastructure is built from. It has a junior → mid → senior
→ staff roadmap, a [function-signature blueprint](systems-core/SIGNATURES.md)
(what to use when, and what not to), 24 exercises in the same `todo!()` +
README + test format, and six [capstones](systems-core/CAPSTONES.md).

Topics include:

- scoped and owned threads, `Send`/`Sync`/`'static`, panic handling;
- lock ordering, atomics and memory ordering, snapshot swapping;
- bounded pipelines, actors with one-shot replies, graceful shutdown;
- write-ahead logs, crash recovery, group commit, replay protection;
- logical clocks, failure detection, retries with backoff and budgets;
- BFT stake quorums, Raft leader election, heaviest-subtree fork choice;
- Sealevel-style account-lock scheduling, leader schedules, Turbine fanout,
  gossip CRDTs, and Proof of History verification;
- tokio actors and bounded concurrent fan-out.

It includes `strand`, a lab crate that rebuilds `Arc`, `Mutex`, channels,
`select!` and a tiny async executor from atomics, with runnable examples.

## How the three tracks connect

| Rust / Anchor skill          | Junior application (Anchor Core)                    | Production application (Protocol Core)                  |
| ----------------------------- | ---------------------------------------------------- | --------------------------------------------------------- |
| Ownership and borrowing       | Safely reading and mutating account state            | Zero-copy account access under compute pressure            |
| Structs                       | Serialized account and instruction data              | Fixed-point balance and share accounting                   |
| Enums and `match`             | Protocol phases, commands, and legal transitions     | Liquidation states, order lifecycle, governance phases      |
| `Result` and custom errors    | Failed instructions and client-readable errors       | Oracle-staleness and solvency-check failure paths           |
| Slices and lifetimes          | Borrowed instruction bytes and account contexts      | Range-proof and ciphertext handling in confidential transfers |
| Traits and generics           | Anchor account, serialization, and validation APIs   | Verifier interfaces for ZK proof programs                    |
| Collections                   | Bounded account data and predictable compute         | Multi-oracle aggregation and keeper registries               |
| Checked arithmetic            | Safe balances, fees, shares, prices, and interest    | Overflow-safe liquidation, funding rate, and peg mechanics    |
| CPI                            | One constrained cross-program call                   | Multi-hop CPI graphs and arbitrary-CPI exploit defense        |
| Tests                          | Proving authorization, boundaries, and invariants    | Fuzzing, adversarial testing, and mock security audits        |

## How to study

Work through the tracks **in order** — Rustlings Core, then Anchor Core, then
Protocol Core. Each track assumes the previous one is fully passing.

```
cd rustlings-core
cargo test -p ex01-let-mut
```

For each exercise, in any track:

1. Read the exercise folder's `README.md`.
2. Read the function signatures and tests in `src/lib.rs`.
3. Predict ownership, state changes, success, and failure cases.
4. Follow the numbered coding map beside each `todo!()`.
5. Implement one function at a time.
6. Run only that package's tests until it passes.
7. Add at least one boundary or adversarial test of your own.
8. Explain the Solana (or, in Protocol Core, the economic/security) connection
   before continuing. If you can't explain *why* the check exists, re-read
   the module README before moving on.

Move to Anchor Core only after Rustlings Core is fully passing:

```
cd ../anchor-core
cargo test -p ex01-account-as-state
```

Move to Protocol Core only after Anchor Core is fully passing:

```
cd ../protocol-core
cargo test -p ex01-fixed-point-basics
```

Start Systems Core once Rustlings Core is passing. It can run in parallel
with Anchor Core and Protocol Core (its README has a 16-week plan that
interleaves them):

```
cd ../systems-core
cargo test -p ex01-scoped-parallel-sum
```

Run an entire track only after its unfinished exercises have been completed:

```
cargo test --workspace
```

## Appendix

A full glossary of Rust, Solana, Anchor, DeFi, oracle, MEV, cryptography, and
testing terminology used across all three tracks lives in
[`GLOSSARY.md`](GLOSSARY.md). Each term is defined in plain language, followed
by a one-line note on why it matters in this curriculum. Consult it whenever
an exercise README uses a term you haven't seen defined yet — the exercises
assume you'll look terms up rather than front-loading a glossary read before
you start.

## What this repository seeks to achieve

By the end of Rustlings Core and Anchor Core, a learner should be able to:

- write idiomatic Rust without guessing about moves, borrows, or lifetimes;
- model protocol state and legal transitions with structs and enums;
- handle malformed input through precise errors instead of panics;
- use bounded collections and predictable iteration;
- implement overflow-safe integer and fixed-point arithmetic;
- distinguish an account address, account owner, signer, and protocol
  authority;
- explain why each Anchor account constraint exists;
- derive and validate PDA-based state identities;
- reason about privileges and failures across a CPI;
- calculate maximum serialized account space;
- identify instruction invariants and test hostile account substitution;
- consider replay, duplicate accounts, compute ceilings, and privacy leakage.

By the end of Protocol Core, a learner should additionally be able to:

- model share-based vault accounting and defend against donation attacks;
- defend an oracle-dependent protocol against staleness and price
  manipulation;
- recognize and defend against sandwich attacks and other MEV, and explain
  when confidential transfers or commit-reveal patterns are the right
  mitigation;
- trace signer privilege and validate account identity across multi-hop CPI;
- design multisig- and timelock-gated governance for sensitive instructions;
- run a real devnet deployment pipeline and monitor it in production;
- read a professional security audit and conduct a credible mock audit of
  someone else's program;
- write and run a Trident fuzz suite against their own protocol, and turn a
  discovered bug into a permanent regression test;
- explain what a zero-knowledge proof does and does not prove, and reason
  about when ZK is worth its cost versus a simpler mechanism;
- profile and reduce an instruction's compute unit cost.

Completing all three tracks thoughtfully can move a learner from basic Rust
syntax through junior-level Anchor development into practical **mid-level
Solana protocol engineering** — able to reason about economic invariants,
oracle risk, MEV, and cross-program trust boundaries, and to run a fuzzing and
mock-audit pass on their own code before anyone else sees it.

Reaching **senior-level, production-custody readiness** additionally requires
real deployment history, incident response experience under live conditions,
and independent third-party security review. This repository is designed to
build the judgment those experiences require — it is not a substitute for
having lived through them. Typing solutions mechanically will provide limited
value at any level. Predicting behavior, explaining compiler and runtime
errors, adding adversarial tests, and building a project from scratch will
produce much stronger results than following the numbered hints alone.

## Suggested capstones

Protocol Core includes ten capstone specifications, ranging from an
overcollateralized stablecoin to a fuzzed-and-audited "mystery vault." See
[`protocol-core/CAPSTONES.md`](protocol-core/CAPSTONES.md) for the full list.

Each capstone requires, at minimum:

- deployment to devnet, not just localnet;
- a written document listing every invariant and trust boundary;
- a Trident fuzz suite run against the finished program;
- tests for wrong signer, owner, PDA, program ID, mint, duplicate accounts,
  overflow, invalid state, replay, CPI failure, and maximum-size inputs.

If you can implement a capstone without copying, explain every constraint,
defend each failure test, and describe what would need to happen before you'd
trust it with real funds, you have built a solid foundation for
production-oriented Solana protocol engineering.
