# ex22 — Account locking and reentrancy analogues

## Why this exists

Solana locks transaction accounts, preventing classic EVM callbacks into newly introduced accounts, but stale assumptions and CPI state inconsistency remain.

## Rust / Anchor connection

Checks-effects-interactions around CPI.

## Production exploit or bug this defends against

A CPI mutates shared accounts and the caller continues using cached pre-CPI values.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
cache balance, CPI, then overwrite callee changes
```

## Step-by-step coding approach

model account set; commit safe effects; reload/verify CPI-mutated state

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Post-CPI logic must use authoritative current state and preserve invariants.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
