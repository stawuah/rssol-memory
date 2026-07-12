# ex08 — Overcollateralized stablecoin engine

## Why this exists

A **stablecoin** aims to track a reference value, commonly one US dollar. An
**overcollateralized** design requires collateral worth more than the debt minted.
For example, a 150% minimum ratio requires $150 of collateral for $100 debt.

Minting creates stablecoin debt against collateral. Burning destroys stablecoins
and reduces debt. Redemption or collateral withdrawal must leave the position
above its required collateral ratio. The **peg** is the target market price; this
small model enforces backing rules but does not by itself guarantee market demand
will keep the token at the peg.

## Production Solana application

Collateralized stablecoins combine token mint authority, vault custody, oracle
valuation, debt accounting, liquidation, governance, and emergency controls.

## What this defends against

It prevents unbacked minting, withdrawing locked collateral, debt underflow,
inconsistent global supply, zero-value operations, and partial state mutation.

## What wrong code looks like

```rust,ignore
position.debt += mint_amount;
global_supply += mint_amount;
// Collateral ratio checked afterward: failure may leave inconsistent state.
```

## How to code it

1. Calculate collateral value with checked wide arithmetic.
2. Calculate maximum debt allowed by the minimum ratio.
3. For mint, calculate position debt and global supply first, then verify backing.
4. For burn, cap or reject amounts above position debt and update both totals.
5. For withdrawal, calculate remaining collateral first and re-check backing.
6. Commit related state fields atomically.

## How to test it

Test mint exactly at the ratio, one unit above, burn, full debt repayment, collateral
withdrawal with/without debt, price decline, global supply consistency, zero input,
and overflow.

## Explain before continuing

Explain the difference between “the protocol enforces 150% backing at its oracle
price” and “the stablecoin is guaranteed to trade at $1.”

## Required learner test

Write a sequence test with two positions and assert that global stablecoin supply
always equals the sum of their debts after successful mint and burn operations.
