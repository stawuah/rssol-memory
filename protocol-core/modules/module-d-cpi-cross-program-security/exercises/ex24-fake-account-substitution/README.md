# ex24 — Fake account substitution exploit

## Why this exists

A caller may supply a look-alike account they own unless owner, address, type, and relationship are checked.

## Rust / Anchor connection

Account identity and ownership constraints.

## Production exploit or bug this defends against

Reading attacker-controlled bytes as trusted vault state grants fake balances.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
check only deserialized fields
```

## Step-by-step coding approach

exploit fake account; then validate owner, discriminator, PDA, authority relation

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only canonical program-owned state can authorize value movement.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
