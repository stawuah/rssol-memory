# ex37 — Meaningful fuzz harnesses

## Why this exists

A useful fuzzer generates structured actions, maintains a model oracle, explores boundaries, and checks invariants after every step.

## Rust / Anchor connection

Stateful model-based testing.

## Production exploit or bug this defends against

Uniform random bytes mostly fail deserialization and never reach deep logic.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
assert only program does not panic
```

## Step-by-step coding approach

weight valid/invalid actions; model balances; include aliases and boundaries; compare model

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Harness reaches deep transitions and detects semantic divergence, not just crashes.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
