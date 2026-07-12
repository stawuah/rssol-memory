# ex15 — Commit-reveal ordering

## Why this exists

A commitment hides a choice until a later reveal binds the sender, payload, nonce, and domain.

## Rust / Anchor connection

Hash preimages and replay-safe state.

## Production exploit or bug this defends against

A copied reveal or changed payload can steal execution priority.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
commit only hashes value without signer or nonce
```

## Step-by-step coding approach

construct domain-separated commitment; enforce delay/expiry; consume once

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only the committer can reveal the exact payload once in the valid window.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
