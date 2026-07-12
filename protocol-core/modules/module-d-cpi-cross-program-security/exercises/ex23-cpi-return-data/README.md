# ex23 — CPI return-data validation

## Why this exists

Return data is bytes plus the program ID that set it; both identity and encoding require validation.

## Rust / Anchor connection

Length, owner, discriminator, and value checks.

## Production exploit or bug this defends against

Trusting arbitrary return bytes can fabricate quotes or amounts.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
deserialize return bytes without checking producer
```

## Step-by-step coding approach

check producer ID, exact length, tag, bounds, and semantic relation

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only well-formed data from the expected callee affects state.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
