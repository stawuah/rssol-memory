# ex40 — Groth16 versus PLONK

## Why this exists

Groth16 often has small proofs and circuit-specific trusted setup; PLONK-family systems offer more universal setup/flexibility with different costs.

## Rust / Anchor connection

Engineering tradeoff evaluation.

## Production exploit or bug this defends against

Choosing by proof size alone ignores setup, prover, verifier, tooling, and upgrades.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
declare one proof system universally best
```

## Step-by-step coding approach

compare setup, proof size, verifier cost, circuit changes, ecosystem, security assumptions

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**Choice document ties proof system properties to product constraints.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
