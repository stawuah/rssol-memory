# Rust-to-Solana Memory

A hands-on learning repository for building the Rust knowledge, Solana mental
models, and security habits needed to write reliable on-chain programs with
Anchor.

The repository teaches concepts in two connected stages:

1. `rustlings-core` develops practical Rust fluency through small test-driven
   exercises.
2. `anchor-core` translates those Rust concepts into Solana accounts, state
   transitions, validation, PDAs, CPIs, account sizing, errors, and compute.
3. `protocol-core` applies those foundations to production financial math,
   oracle safety, MEV, CPI security, governance, operations, fuzzing, ZK, and
   optimization. It is being built module by module so each section can be
   reviewed before the next is added.

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

## How the two tracks connect

| Rust skill | Solana and Anchor application |
|---|---|
| Ownership and borrowing | Safely reading and mutating account state |
| Structs | Serialized account and instruction data |
| Enums and `match` | Protocol phases, commands, and legal transitions |
| `Result` and custom errors | Failed instructions and client-readable errors |
| Slices and lifetimes | Borrowed instruction bytes and account contexts |
| Traits and generics | Anchor account, serialization, and validation APIs |
| Collections | Bounded account data and predictable compute |
| Checked arithmetic | Safe balances, fees, shares, prices, and interest |
| Tests | Proving authorization, boundaries, and invariants |

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

Run an entire track only after its unfinished exercises have been completed:

```sh
cargo test --workspace
```

## What this repository seeks to achieve

By the end, a learner should be able to:

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

The expected outcome is readiness to build and test educational Solana programs
such as counters, vaults, escrows, staking models, and simple marketplaces on
localnet or devnet.

## Proficiency this can develop

Completing the exercises thoughtfully can move a learner from basic Rust syntax
toward practical junior-level Solana program development. The learner should be
able to read an Anchor account context, implement small state transitions, write
meaningful tests, and discuss common account-validation risks.

Proficiency depends on how the repository is used. Typing solutions mechanically
will provide limited value. Predicting behavior, explaining compiler errors,
adding adversarial tests, and building a project from scratch will produce much
stronger results.

This repository alone does not make a developer ready to custody real funds.
Production protocols require deeper economic design, token integration, oracle
and timing analysis, deployment operations, extensive integration and fuzz
testing, threat modeling, and independent security review.

## Suggested capstone

Build a localnet vault or escrow containing:

- initialization with a deterministic PDA;
- stored authority and explicit protocol phase;
- deposit, withdrawal or settlement, freeze/cancel, and close instructions;
- checked balance arithmetic;
- correct account allocation;
- one carefully constrained CPI;
- tests for wrong signer, owner, PDA, program ID, mint, duplicate accounts,
  overflow, invalid state, replay, CPI failure, and maximum-size inputs;
- a short document listing every invariant and trust boundary.

If you can implement the capstone without copying, explain every constraint, and
defend each failure test, you have built a solid foundation for more advanced
Solana protocol engineering.

## Safety

Use localnet or devnet while learning. Never commit private keys or seed phrases,
and never treat unaudited educational code as production financial software.
