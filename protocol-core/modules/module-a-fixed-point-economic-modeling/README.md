# Module A — Fixed-Point and Economic Modeling

This module teaches deterministic financial math and the economic invariants that
keep vaults, AMMs, lending markets, liquidations, and stablecoins solvent.

## Map

| Exercise | Rust / Anchor skill | Solana production application |
|---|---|---|
| ex01 | Q32.32 scaled integers and wide intermediates | deterministic on-chain prices and indexes |
| ex02 | floor/ceiling division | solvency-aware withdrawals and fees |
| ex03 | proportional state accounting | ERC4626-style share vaults and donation defense |
| ex04 | checked time accrual | management fees and lending indexes |
| ex05 | invariant-preserving mutation | AMM swaps and minimum-out enforcement |
| ex06 | piecewise fixed-point functions | kinked utilization interest curves |
| ex07 | threshold ratios | health factors and liquidation quotes |
| ex08 | atomic multi-total accounting | stablecoin mint, burn, and collateral redemption |

## Ordered exercises

1. [Fixed-point Q32.32](./exercises/ex01-fixed-point/README.md)
2. [Rounding policy](./exercises/ex02-rounding-policy/README.md)
3. [Share-based vault](./exercises/ex03-share-vault/README.md)
4. [Per-second fee accrual](./exercises/ex04-fee-accrual/README.md)
5. [Constant-product AMM](./exercises/ex05-constant-product-amm/README.md)
6. [Kinked interest curve](./exercises/ex06-kinked-interest/README.md)
7. [Liquidation health](./exercises/ex07-liquidation-health/README.md)
8. [Overcollateralized stablecoin](./exercises/ex08-stablecoin-engine/README.md)

## Coverage checklist

- [x] Q32.32 representation and why floating point is unsuitable for consensus math
- [x] protocol-favorable rounding direction
- [x] share accounting and first-depositor donation attack
- [x] per-second fee accrual without narrow overflow
- [x] constant-product slippage and minimum output
- [x] utilization and kinked rates
- [x] liquidation and health factors
- [x] stablecoin mint, burn, and collateral redemption mechanics

**Common interview question:** How can rounding direction or share conversion make
an otherwise correct-looking financial program insolvent?
