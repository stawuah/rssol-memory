# ex32 — Mainnet readiness runbook

## Why this exists

A runbook is an executable human procedure for deployment, monitoring, incidents, rollback, and authority custody.

## Rust / Anchor connection

Operational checklist design.

## Production exploit or bug this defends against

Teams discover missing keys, alerts, or rollback steps during an incident.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
deploy checklist contains only build and deploy
```

## Step-by-step coding approach

write owners, prerequisites, go/no-go, verification, rollback, alerts, incident contacts

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Another engineer can execute and verify deployment without tribal knowledge.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
