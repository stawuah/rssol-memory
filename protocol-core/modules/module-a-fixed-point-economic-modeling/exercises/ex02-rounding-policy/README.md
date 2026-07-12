# ex02 — Rounding is protocol policy

## Why this exists

Integer division discards the remainder. That is not merely a technical detail:
it determines who receives the fractional dust. Repeatedly giving dust to users
can slowly make a protocol insolvent.

**Floor division** rounds down. **Ceiling division** rounds up whenever a remainder
exists. A common solvency policy is to round assets paid out down and fees or debt
owed to the protocol up. Exact policy depends on the product and must be documented.

## Production Solana application

Vault withdrawals, share conversions, swap output, protocol fees, interest, and
liquidation bonuses all require an intentional rounding direction.

## What this defends against

It prevents “rounding leakage,” where many small operations extract value because
every remainder favors the caller.

## What wrong code looks like

```rust,ignore
let fee = amount * fee_bps / 10_000; // silently charges zero on tiny amounts
let payout = shares * assets / supply + 1; // always overpays, even exact division
```

## How to code it

1. Build one checked wide multiplication helper.
2. Floor by ordinary integer division.
3. Ceiling by adding one only when the remainder is non-zero.
4. Reject zero denominator.
5. Convert back to `u64` explicitly.

## How to test it

Test exact division, one-unit remainder, tiny fee, zero numerator, zero denominator,
and a result too large for `u64`.

## Explain before continuing

Explain who benefits when withdrawal output rounds up and why ceiling division must
not add one to an already exact result.

## Required learner test

Add a test showing that splitting a fee-bearing action into smaller actions cannot
reduce the total fee under the chosen ceiling policy.
