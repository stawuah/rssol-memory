# ex26 — Multisig-gated administration

## Why this exists

A multisig requires a threshold of distinct approved members before an admin action executes.

## Rust / Anchor connection

Distinct signer quorum and proposal binding.

## Production exploit or bug this defends against

Duplicate signatures or proposal substitution bypass governance.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
count approvals without deduplicating members
```

## Step-by-step coding approach

bind proposal payload; validate members; count distinct approvals; execute once

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**A threshold of distinct current members approves the exact action once.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
