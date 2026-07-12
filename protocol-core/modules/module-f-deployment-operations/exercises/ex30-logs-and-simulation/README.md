# ex30 — Logs and simulate-before-send

## Why this exists

Simulation estimates execution, logs, errors, account effects, and compute before a transaction is signed and sent.

## Rust / Anchor connection

Structured log parsing.

## Production exploit or bug this defends against

Bots retry permanent failures or miss invariant alarms hidden in text logs.

## What wrong code looks like

```rust,ignore
// Wrong because it omits the invariant and trust-boundary checks:
send first and parse logs with substring guesses
```

## Step-by-step coding approach

classify simulation errors; parse stable event fields; alert; decide retry policy

Implement the small policy model in `src/lib.rs` first. Then translate each boolean
control into the concrete accounts, identities, proofs, slots, or limits described
above. Calculate every new value before committing state.

## How to test it correctly

Test the valid path, then turn off each control independently. Test equality at the
limit, one unit above it, repeated execution, and state unchanged on failure. The
required learner test must model the attacker action described above rather than
only passing random invalid input.

## Invariant

**No production send occurs after failed simulation, and failures are classified.**

## Explain before continuing

In your own words, identify the protected asset, attacker-controlled input, trusted
identity or assumption, exact failure path, and why the chosen test would catch a
regression. Do not continue until another developer can understand your explanation.

## Required learner test

Replace the ignored test in `src/lib.rs` with an adversarial or boundary test that
is different from the supplied cases, then remove `#[ignore]`.
