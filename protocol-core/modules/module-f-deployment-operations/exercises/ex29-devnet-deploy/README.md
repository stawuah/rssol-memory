# ex29 — Anchor devnet deployment

## Why this exists

A reproducible deployment pins tool versions, cluster, program ID, binary hash, IDL, and migration steps.

## Rust / Anchor connection

Deployment manifest validation.

## Production exploit or bug this defends against

Deploying wrong keypair or stale IDL breaks clients and authority control.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
run anchor deploy without cluster or artifact verification
```

## Step-by-step coding approach

build; verify IDs; simulate; deploy devnet; upload/version IDL; record hashes

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Deployed binary, declared ID, IDL version, cluster, and authority all match manifest.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
