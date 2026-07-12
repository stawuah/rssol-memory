# ex07 — Health factors and liquidation math

## Why this exists

A **health factor** compares risk-adjusted collateral value with debt. A factor at
or above `1.0` is healthy under the model; below `1.0` is liquidatable. Collateral
is discounted by a liquidation threshold because its market value can fall before
the protocol sells it.

A **liquidation** lets a third party repay unhealthy debt and receive collateral,
usually with a bonus. Close-factor limits prevent one liquidation from repaying
more debt than policy permits.

Prices in this exercise use micro-dollars (`1_000_000` = $1) and ratios use basis
points (`10_000` = 100%).

## Production Solana application

Lending markets, stablecoins, margin accounts, and perpetual futures all depend on
correct health and liquidation calculations.

## What this defends against

It prevents healthy accounts being liquidated, unhealthy accounts escaping
liquidation, excessive collateral seizure, wrong token decimals, and overflow.

## What wrong code looks like

```rust,ignore
let health = collateral_amount * collateral_price / debt_amount * debt_price;
// Wrong precedence/scaling, overflow risk, and division occurs too early.
```

## How to code it

1. Convert token amounts to value using matching base-unit scales.
2. Apply liquidation threshold to collateral value.
3. Represent health in PPM and define zero-debt health explicitly.
4. Permit liquidation only below `1_000_000` PPM.
5. Cap repay amount by debt and close factor.
6. Calculate seized collateral from repaid debt value plus bonus, rounding up.

## How to test it

Test zero debt, exactly healthy boundary, one unit below boundary, price decline,
close-factor cap, collateral cap, decimal mismatch scenarios, and overflow.

## Explain before continuing

Explain why a collateral value of $100 does not support $100 debt when the
liquidation threshold is 80%, and who receives the liquidation bonus.

## Required learner test

Add a boundary test showing an exactly healthy position cannot be liquidated while
the nearest representable unhealthy position can.
