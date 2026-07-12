# ex05 — Constant-product AMM and slippage

## Why this exists

An automated market maker (AMM) swaps against token reserves. A constant-product
AMM tries to preserve `x * y = k`, where `x` and `y` are reserve amounts. A larger
trade moves the reserve ratio more, producing **price impact**.

**Slippage** is the difference between expected output and actual output. A caller
sets `minimum_out`; the swap must fail atomically when output is lower. Without it,
network ordering or an attacker can move the price before execution.

## Production Solana application

AMMs, routing, liquidations, vault rebalancing, and any CPI swap need explicit
minimum-output protection.

## What this defends against

It defends against unlimited slippage, zero-output trades, arithmetic overflow,
reserve corruption, and some consequences of sandwich attacks.

## What wrong code looks like

```rust,ignore
let out = reserve_y - (reserve_x * reserve_y / (reserve_x + amount_in));
reserve_x += amount_in;
reserve_y -= out; // no fee, no minimum_out, mutations may partially succeed
```

## How to code it

1. Reject zero input and invalid fee basis points.
2. Apply the fee to input with floor rounding.
3. Calculate output widely with `dy = y * dx / (x + dx)`.
4. Reject zero output and output below `minimum_out`.
5. Compute both reserve changes before committing.
6. Verify the product does not decrease under the model's rounding.

## How to test it

Test normal swap, excessive minimum-out, zero input, tiny input, large price impact,
invalid fee, unchanged reserves on failure, and product preservation.

## Explain before continuing

Explain why `minimum_out` protects the individual transaction but does not remove
all MEV, and why the fee-adjusted input differs from reserve input.

## Required learner test

Add a test that compares one large swap with a much smaller swap and demonstrates
the larger trade's worse effective price.
