# ex06 — Utilization and kinked interest curves

## Why this exists

**Utilization** is the fraction of supplied liquidity currently borrowed. If a
pool has 1,000 supplied and 800 borrowed, utilization is 80%. Lending protocols
raise borrowing rates as utilization rises to encourage repayment and new supply.

A **kinked interest curve** is piecewise linear: rates rise gently below a target
utilization (the kink) and steeply above it. The steep region protects withdrawable
liquidity when the pool becomes heavily utilized.

## Production Solana application

Lending pools calculate borrow and supply rates from current utilization, then
accrue them into indexes used by every depositor and borrower.

## What this defends against

It prevents division by zero, utilization above 100%, discontinuity at the kink,
overflow, and incorrect slope scaling that can charge absurd rates.

## What wrong code looks like

```rust,ignore
let utilization = borrowed / supplied; // integer result is usually zero
let rate = base + utilization * slope;  // scale is missing
```

## How to code it

1. Express utilization in PPM (`1_000_000` means 100%).
2. Reject borrowed greater than supplied; define empty-pool utilization as zero.
3. Below the kink, scale slope-one by utilization divided by kink.
4. Above the kink, add all of slope-one plus a scaled excess using slope-two.
5. Use checked wide intermediates and explicit conversion.

## How to test it

Test empty pool, zero borrowing, just below kink, exactly kink, just above kink,
100% utilization, invalid pool state, and monotonic rate increase.

## Explain before continuing

Draw the two line segments and explain why the function must be continuous at the
kink even though its slope changes.

## Required learner test

Add a property-style loop asserting the rate never decreases as utilization rises
through valid sample points.
