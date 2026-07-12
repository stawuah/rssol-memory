# ex41 — Light-style ZK compression

## Why this exists

Compressed state stores commitments in Merkle trees; validity proofs show a state transition relates to an accepted root without storing every account directly.

## Rust / Anchor connection

Merkle root and nullifier modeling.

## Production exploit or bug this defends against

Accepting an old root or reused leaf permits replay/double spend.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
verify Merkle path but not current root or nullifier
```

## Step-by-step coding approach

bind leaf, root, owner, transition, and nullifier; update root atomically

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only a valid proof against an accepted root consumes each state item once.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
