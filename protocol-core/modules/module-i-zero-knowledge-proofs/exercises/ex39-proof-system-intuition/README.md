# ex39 — Proof-system intuition

## Why this exists

A prover convinces a verifier a statement is true; completeness accepts true statements, soundness rejects false ones, and zero-knowledge hides the witness.

## Rust / Anchor connection

Statement/witness separation.

## Production exploit or bug this defends against

Confusing encrypted data with proof of valid computation accepts false state transitions.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
accept proof-shaped bytes without binding public inputs
```

## Step-by-step coding approach

model statement, witness, proof; test completeness, soundness, and no witness disclosure

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Verifier acceptance is bound to exact public inputs and valid witness relation.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
