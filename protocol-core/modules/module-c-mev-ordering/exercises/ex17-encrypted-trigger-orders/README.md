# ex17 — Encrypted trigger orders

## Why this exists

Encrypted take-profit and stop-loss orders delay disclosure of trigger price and size until authorized execution.

## Rust / Anchor connection

Commitment-bound encrypted order state.

## Production exploit or bug this defends against

Revealing plaintext before eligibility invites front-running.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
store plaintext trigger and size on-chain
```

## Step-by-step coding approach

bind ciphertext to owner and market; validate reveal; enforce one execution or cancellation

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**An order executes once only when its authenticated reveal and oracle condition match.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
