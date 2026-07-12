# ex21 — Multi-hop CPI privileges

## Why this exists

Signer and writable privileges propagate through CPI but cannot be escalated beyond what the caller received.

## Rust / Anchor connection

Privilege tracking across call graphs.

## Production exploit or bug this defends against

A downstream program may misuse an unnecessarily forwarded signer.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
forward authority signer through every hop
```

## Step-by-step coding approach

track per-hop metadata; reject escalation; minimize privilege at each edge

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Every hop uses a subset of upstream privileges and only required accounts.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
