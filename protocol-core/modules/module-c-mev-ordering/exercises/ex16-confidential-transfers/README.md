# ex16 — Token-2022 confidential transfers

## Why this exists

ElGamal encryption hides amounts while homomorphic operations and range proofs support confidential balances.

## Rust / Anchor connection

Ciphertext-state validation.

## Production exploit or bug this defends against

Treating encryption as total anonymity leaks participants, timing, and intent.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
assume encrypted amount hides all metadata
```

## Step-by-step coding approach

classify visible metadata; require proof and auditor policy; model pending/available balance

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Confidential balance changes require valid proofs and still expose documented metadata.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
