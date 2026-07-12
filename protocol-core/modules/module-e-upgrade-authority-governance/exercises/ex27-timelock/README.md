# ex27 — Governance timelock

## Why this exists

A timelock delays sensitive actions so users and monitors can react before execution.

## Rust / Anchor connection

Slot windows and immutable queued payloads.

## Production exploit or bug this defends against

Admin queues a safe action then changes parameters at execution.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
store mutable pointer to proposed parameters
```

## Step-by-step coding approach

hash payload at queue; enforce earliest slot and expiry; cancel/execute once

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Executed payload exactly matches the queued payload after the full delay.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
