# ex38 — Minimize and regress a fuzz bug

## Why this exists

Minimization removes irrelevant actions from a failing sequence; a regression test permanently preserves the smallest reproducer.

## Rust / Anchor connection

Failure reduction and deterministic tests.

## Production exploit or bug this defends against

Leaving a bug only as a random seed makes it flaky and easy to reintroduce.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
store huge nondeterministic fuzz log
```

## Step-by-step coding approach

capture seed; shrink actions/data/accounts; explain root cause; add named deterministic test

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**The minimal sequence fails before patch and passes afterward forever.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
