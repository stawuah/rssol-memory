# ex44 — When ZK is the wrong tool

## Why this exists

ZK adds proving latency, verifier compute, setup/tooling, circuit maintenance, and audit complexity; simpler mechanisms may solve the actual threat.

## Rust / Anchor connection

Architecture decision writing.

## Production exploit or bug this defends against

Using ZK for public facts or ordering problems creates cost without privacy benefit.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
choose ZK because privacy sounds desirable
```

## Step-by-step coding approach

define secret, statement, adversary, latency/cost; compare oracle and commit-reveal alternatives

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**ZK is selected only when a hidden witness and verifiable relation are truly required.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
