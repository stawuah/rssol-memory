# ex48 — Rent, closing, and PDA reuse

## Why this exists

Account allocation requires lamports; closing refunds them. Reusing deterministic PDA addresses can enable revival or stale-state confusion unless lifecycle is explicit.

## Rust / Anchor connection

Account lifecycle and sizing.

## Production exploit or bug this defends against

Close to attacker-controlled recipient or reinitialize old identity without versioning.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
close without authority/zero-state checks
```

## Step-by-step coding approach

calculate minimal max space; verify close recipient; terminal marker/version; safe reuse policy

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Closed value is refunded correctly and old logical state cannot be replayed after reuse.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
