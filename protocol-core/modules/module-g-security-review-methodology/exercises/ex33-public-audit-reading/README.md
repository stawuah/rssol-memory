# ex33 — Read a public audit

## Why this exists

Audit reports show real failure modes, severity reasoning, exploitability, and remediation tradeoffs.

## Rust / Anchor connection

Evidence-based technical summarization.

## Production exploit or bug this defends against

Copying finding titles produces no transferable understanding.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
summarize without affected invariant or attack path
```

## Step-by-step coding approach

choose Neodyme/OtterSec/Sec3; cite report; restate three findings; map fixes/tests

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Each summary identifies root cause, violated invariant, exploit path, fix, and regression test.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
