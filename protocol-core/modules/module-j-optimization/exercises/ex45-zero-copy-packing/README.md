# ex45 — Struct packing and zero-copy

## Why this exists

Zero-copy reads account bytes without full deserialization but requires stable layout, alignment, padding, and safe access discipline.

## Rust / Anchor connection

Layout-aware account design.

## Production exploit or bug this defends against

Reordering fields changes padding/layout and can corrupt upgrades.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
assume Rust field sizes simply sum with no padding
```

## Step-by-step coding approach

compare layouts; order fields; validate discriminator/owner/length; document versioning

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Zero-copy type has verified stable layout and rejects malformed account bytes.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
