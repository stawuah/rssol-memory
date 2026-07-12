# ex31 — Compute budgets and priority fees

## Why this exists

Transactions request a compute-unit limit and optional unit price; congestion affects scheduling but higher fees do not fix incorrect limits.

## Rust / Anchor connection

Resource estimation and fee caps.

## Production exploit or bug this defends against

Underestimated limits fail; unlimited fee bidding burns funds under load.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
always request maximum units and uncapped price
```

## Step-by-step coding approach

measure worst case; add margin; cap total fee; retry with fresh blockhash

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Transaction stays within measured compute and configured fee-risk limits.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
