# Rust-to-Solana Memory

A hands-on learning path from Rust fundamentals to production-grade Solana
infrastructure and smart contract engineering. It starts with Rust syntax,
moves through Anchor and Solana runtime reasoning, and ends with protocol
design, economic modeling, oracle safety, MEV resistance, security review,
fuzzing, and zero-knowledge systems.

The repository teaches concepts in three connected stages:

1. `rustlings-core` develops practical Rust fluency through small test-driven
   exercises.
2. `anchor-core` translates those Rust concepts into Solana accounts, state
   transitions, validation, PDAs, CPIs, account sizing, errors, and compute.
3. `protocol-core` builds on both to teach production protocol math, oracle
   safety, MEV, CPI security, governance, deployment, fuzzing, zero-knowledge
   proofs, and optimization for real Solana systems.

The goal is not to memorize framework syntax. It is to understand what the code
does, why the Solana runtime requires each check, and how a protocol remains
correct when callers provide hostile inputs.

## Learning tracks

### Rustlings Core

[`rustlings-core`](./rustlings-core/README.md) contains focused Rust exercises
with tests, a local README in every exercise folder, and numbered implementation
guidance beside each `todo!()`.

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

[`anchor-core`](./anchor-core/README.md) maps Rust reasoning to simplified
Solana and Anchor models. Every exercise also includes its own README and coding
steps in the source.

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

The exercises use small Rust models before introducing full framework machinery.
This keeps the runtime behavior and security purpose of each Anchor constraint
visible.

### Protocol Core

[`protocol-core`](./protocol-core/README.md) is the third track. It assumes the
learner already understands Rust fundamentals and Anchor basics, then moves
into the design problems that show up in real Solana protocols.

Topics include:

- fixed-point arithmetic, rounding, shares, fees, and liquidation math;
- oracle safety, staleness, manipulation resistance, and circuit breakers;
- MEV-aware ordering, commit-reveal, and encrypted trigger logic;
- CPI security, signer propagation, and hostile account substitution;
- upgrade authority, multisig governance, and timelocked changes;
- deployment pipelines, logs, compute budgets, and mainnet runbooks;
- security review, fuzzing, regression testing, and trust-boundary docs;
- zero-knowledge proof intuition and Solana verification tradeoffs;
- struct packing, zero-copy, batching, and rent-aware optimization.

This is the track that helps a learner move from "I can write a program" to
"I can reason about whether a protocol is safe enough to ship."

## How the three tracks connect

| Stage or skill | What it teaches | Why it matters |
|---|---|---|
| `rustlings-core` | Rust syntax, ownership, errors, collections, and lifetimes | Gives you the language fluency to write correct code without fighting the compiler |
| `anchor-core` | Accounts, PDAs, CPIs, validation, sizing, and compute | Turns Rust knowledge into safe Solana program structure |
| `protocol-core` | Protocol math, security, governance, deployment, fuzzing, and optimization | Teaches you how to design and harden real production smart contracts |
| Fixed-point math and rounding | Stablecoin solvency, fee accounting, and liquidation math | Prevents insolvency and hidden precision loss in financial programs |
| Oracle reads and staleness checks | Price-manipulation defense and fail-closed execution | Keeps protocols from trusting stale or adversarial market data |
| CPI validation and signer tracking | Cross-program exploit prevention | Stops fake-account substitution and privilege escalation across calls |
| Governance and upgrade authority | Controlled upgrades and admin risk management | Makes protocol change management explicit instead of accidental |
| Fuzzing and audits | Adversarial testing and bug documentation | Finds edge cases before they become production incidents |
| Compute and account optimization | Staying inside network and rent limits | Keeps programs usable under real Solana constraints |

## How to study

Start with Rustlings Core and work in numeric order:

```sh
cd rustlings-core
cargo test -p ex01-let-mut
```

For each exercise:

1. Read the exercise folder's `README.md`.
2. Read the function signatures and tests in `src/lib.rs`.
3. Predict ownership, state changes, success, and failure cases.
4. Follow the numbered coding map beside each `todo!()`.
5. Implement one function at a time.
6. Run only that package's tests until it passes.
7. Add at least one boundary or adversarial test.
8. Explain the Solana connection before continuing.

After completing Rustlings Core, follow the same loop in Anchor Core:

```sh
cd ../anchor-core
cargo test -p ex01-account-as-state
```

After Anchor Core is fully passing, move into Protocol Core in order:

```sh
cd ../protocol-core
cargo test -p ex01-fixed-point
```

The same eight-step loop applies there too. The difference is that the exercises
now focus on protocol logic, production risk, and adversarial behavior instead of
basic Rust or framework mechanics.

Run an entire track only after its unfinished exercises have been completed:

```sh
cargo test --workspace
```

## What this repository seeks to achieve

This repository is designed to take a learner from "I can read Rust and copy an
Anchor example" to "I can explain, test, and defend a Solana protocol design."

By the end, the learner should be able to:

- write idiomatic Rust without guessing about moves, borrows, or lifetimes;
- model protocol state and legal transitions with structs and enums;
- handle malformed input through precise errors instead of panics;
- use bounded collections and predictable iteration;
- implement overflow-safe integer and fixed-point arithmetic;
- distinguish an account address, account owner, signer, and protocol authority;
- explain why each Anchor account constraint exists;
- derive and validate PDA-based state identities;
- reason about privileges and failures across a CPI;
- calculate maximum serialized account space;
- identify instruction invariants and test hostile account substitution;
- consider replay, duplicate accounts, compute ceilings, and privacy leakage.

The immediate outcome is readiness to build and test educational Solana
programs such as counters, vaults, escrows, staking models, and simple
marketplaces on localnet or devnet.

The larger outcome is practical mid-level Solana protocol engineering judgment:
the learner should be able to reason about economic invariants, oracle trust,
MEV exposure, cross-program trust boundaries, governance changes, and
operational readiness for production-grade systems.

## Proficiency this can develop

Completing the exercises thoughtfully can move a learner from basic Rust syntax
and framework familiarity toward junior-level Solana development and then into
practical mid-level protocol engineering. The learner should be able to read an
Anchor account context, implement small state transitions, write meaningful
tests, run a fuzz pass, and explain the main trust boundaries in their own code.

Proficiency depends on how the repository is used. Typing solutions mechanically
will provide limited value. Predicting behavior, explaining compiler errors,
adding adversarial tests, and building a project from scratch will produce much
stronger results.

This repository does not guarantee senior production readiness. Real seniority
still depends on shipped systems, incident response, operational ownership, and
independent third-party security review. What this repo does provide is the
judgment-building foundation that makes those responsibilities realistic to
approach.

## Suggested capstone

If you only do one project, start with the overcollateralized stablecoin
protocol in `protocol-core/CAPSTONES.md`. It exercises the widest range of
economic, oracle, and liquidation invariants.

If you want a broader challenge, build one of the other Protocol Core capstones
that leans into MEV resistance, CPI security, governance, or fuzz testing. The
full set of ten specs is the right place to choose from because each one is tied
to a different production failure mode.

Each capstone requires:

- a devnet or localnet deployment plan;
- a written invariant and trust-boundary document;
- a fuzz or adversarial test strategy;
- a short postmortem-style explanation of what it defends against.

If you can implement one capstone without copying, explain every constraint,
and defend the failure tests, you have built a strong foundation for more
advanced Solana protocol engineering.

## Appendix

- [Root glossary](./GLOSSARY.md)
- [Protocol Core glossary](./protocol-core/GLOSSARY.md)
- [Protocol Core capstones](./protocol-core/CAPSTONES.md)

## Safety

Use localnet or devnet while learning. Never commit private keys or seed phrases,
and never treat unaudited educational code as production financial software.
