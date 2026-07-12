# ex25 — Program upgrade authority

## Why this exists

An upgrade replaces executable code at the same program address while persistent accounts remain; authority controls who may do it.

## Rust / Anchor connection

Authority-state modeling.

## Production exploit or bug this defends against

A compromised upgrade key can replace all program rules.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
treat deployed code as immutable while authority exists
```

## Step-by-step coding approach

model buffer, authority signature, program-data address, and authority rotation

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only the recorded authority can upgrade, and state migration is explicit.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
