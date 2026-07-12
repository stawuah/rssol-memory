# ex43 — Range proofs and confidential tokens

## Why this exists

A range proof shows an encrypted amount lies in an allowed range without revealing it, preventing negative or overflowing hidden balances.

## Rust / Anchor connection

Hidden-value invariant reasoning.

## Production exploit or bug this defends against

Homomorphic ciphertext math alone cannot prove plaintext is non-negative or bounded.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
accept any ciphertext delta without range proof
```

## Step-by-step coding approach

bind proof to ciphertext and account; verify range; prevent replay; update pending balance

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Every hidden amount is proven within range before confidential state changes.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
