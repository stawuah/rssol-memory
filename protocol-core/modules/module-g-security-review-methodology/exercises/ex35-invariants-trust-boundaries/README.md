# ex35 — Capstone invariants and trust boundaries

## Why this exists

An invariant must always hold; a trust boundary marks where untrusted data or authority enters the system.

## Rust / Anchor connection

Pre-code security specification.

## Production exploit or bug this defends against

Writing invariants after implementation rationalizes existing behavior.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
document vague goal such as funds are safe
```

## Step-by-step coding approach

enumerate assets, roles, entrypoints, external programs, oracle assumptions, invariants, failure modes

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Capstone cannot be complete until tests map to every invariant and boundary.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
