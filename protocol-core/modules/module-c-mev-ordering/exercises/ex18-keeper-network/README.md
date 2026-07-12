# ex18 — Permissionless keeper network

## Why this exists

Keepers are independent actors paid to perform maintenance such as liquidations or order execution.

## Rust / Anchor connection

Idempotency and bounded rewards.

## Production exploit or bug this defends against

A keeper can spam failing work or claim rewards repeatedly.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
pay reward before verifying useful state transition
```

## Step-by-step coding approach

validate eligibility; make execution idempotent; cap rewards; charge spam cost

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only a successful first state transition earns one bounded reward.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
