# ex46 — Compute-unit profiling

## Why this exists

Optimization starts with measurement per instruction and input size; logs or checkpoints locate expensive regions.

## Rust / Anchor connection

Benchmark interpretation.

## Production exploit or bug this defends against

Optimizing guessed hot spots adds complexity without lowering worst-case compute.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
benchmark only happy-path tiny input
```

## Step-by-step coding approach

measure baseline/max inputs; attribute deltas; optimize one change; prevent regression

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Worst-case instruction stays below documented CU ceiling with reproducible benchmark.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
