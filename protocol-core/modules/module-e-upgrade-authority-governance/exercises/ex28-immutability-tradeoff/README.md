# ex28 — Immutability tradeoff

## Why this exists

Renouncing upgrade authority removes one trust assumption but also removes emergency fixes; retaining it requires governance disclosure.

## Rust / Anchor connection

Irreversible state transition analysis.

## Production exploit or bug this defends against

Claiming immutability while an upgrade path remains misleads users.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
display immutable flag unrelated to loader authority
```

## Step-by-step coding approach

model retain, transfer, and renounce decisions; produce trust statement

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Published trust assumptions match actual loader authority state.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
