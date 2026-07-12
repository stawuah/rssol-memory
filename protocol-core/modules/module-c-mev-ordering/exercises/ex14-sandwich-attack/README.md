# ex14 — Sandwich attack simulation

## Why this exists

Transaction ordering lets an attacker trade before and after a victim, worsening the victim price.

## Rust / Anchor connection

AMM reserve math and minimum-out enforcement.

## Production exploit or bug this defends against

An unbounded victim swap can be surrounded for profit.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
swap without minimum_out
```

## Step-by-step coding approach

model front-run, victim, back-run; compare victim output; enforce minimum out

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Victim execution must revert atomically below minimum output.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
