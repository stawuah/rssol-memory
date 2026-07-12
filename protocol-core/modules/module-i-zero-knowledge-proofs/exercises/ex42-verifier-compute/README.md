# ex42 — On-chain verifier compute

## Why this exists

Verifier programs check proofs on-chain but consume compute and require exact verifier identity, verifying key, and public inputs.

## Rust / Anchor connection

CPI validation and compute budgeting.

## Production exploit or bug this defends against

Calling arbitrary verifier or omitting public-input binding makes proof meaningless.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
trust boolean returned by caller-supplied program
```

## Step-by-step coding approach

check verifier ID; encode inputs canonically; enforce CU budget; validate return producer/data

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**State changes only after expected verifier confirms exact statement within budget.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
