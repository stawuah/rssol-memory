# ex20 — Missing program-ID exploit

## Why this exists

A caller can substitute a malicious executable when a program accepts an unchecked CPI target.

## Rust / Anchor connection

Executable identity validation.

## Production exploit or bug this defends against

A fake token program can report success without transferring value.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
invoke caller-supplied program without address check
```

## Step-by-step coding approach

implement vulnerable call; attacker fake; then require exact program ID

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**No accounting credit occurs unless the canonical external program executed.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
