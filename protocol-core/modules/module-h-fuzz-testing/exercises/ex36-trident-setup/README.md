# ex36 — Trident vault setup

## Why this exists

Trident drives Solana programs through instruction sequences and checks state properties rather than only example cases.

## Rust / Anchor connection

Fuzz environment and account snapshots.

## Production exploit or bug this defends against

A harness that never reaches valid state reports little useful coverage.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
feed random bytes to one instruction only
```

## Step-by-step coding approach

install compatible Trident; define accounts; initialize valid states; run sequences; assert conservation

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Every generated successful sequence preserves asset and share conservation.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
