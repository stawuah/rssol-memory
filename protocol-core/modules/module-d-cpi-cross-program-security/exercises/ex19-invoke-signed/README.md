# ex19 — invoke and invoke_signed

## Why this exists

A CPI calls another program; invoke_signed lets a program authorize with PDA seeds verified by the runtime.

## Rust / Anchor connection

Account metadata and signer-seed construction.

## Production exploit or bug this defends against

Wrong seed order or forwarded privileges can authorize the wrong action.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
mark every forwarded account writable and signer
```

## Step-by-step coding approach

validate program ID; minimize metadata; reconstruct canonical seeds and bump

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Only the expected PDA receives signer privilege for the expected target program.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
