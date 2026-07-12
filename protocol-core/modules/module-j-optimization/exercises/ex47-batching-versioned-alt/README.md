# ex47 — Batching, v0 transactions, and ALTs

## Why this exists

Batching reduces round trips; versioned transactions and address lookup tables compress account addresses but add lookup lifecycle and integrity concerns.

## Rust / Anchor connection

Transaction planning.

## Production exploit or bug this defends against

Oversized batches exceed packet, account-lock, or compute limits and fail atomically.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
put every operation in one transaction
```

## Step-by-step coding approach

partition by locks/CU/size; validate ALT authority/content; handle expiry/blockhash and partial workflow

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Each batch fits limits and ALT-resolved addresses match expected accounts.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
