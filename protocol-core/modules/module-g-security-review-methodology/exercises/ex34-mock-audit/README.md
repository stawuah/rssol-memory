# ex34 — Mock audit an open-source program

## Why this exists

A security review traces assets, authorities, trust boundaries, state transitions, CPIs, and arithmetic before hunting isolated bugs.

## Rust / Anchor connection

Audit-report writing.

## Production exploit or bug this defends against

Listing style issues as security findings hides missing exploit reasoning.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
finding has no impact, proof, or remediation
```

## Step-by-step coding approach

scope commit; architecture map; threat model; findings with severity/evidence/repro/fix

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Every finding is reproducible and tied to a concrete invariant or trust boundary.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
