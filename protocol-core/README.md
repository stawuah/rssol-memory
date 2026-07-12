# Protocol Core

`protocol-core` is the production-engineering track after `rustlings-core` and
`anchor-core`. It teaches the invariants behind financial Solana programs before
adding framework and deployment machinery.

The exercises are deliberately incomplete. Exercises are grouped under explicit
`modules/module-*` directories. Each exercise folder contains:

- a plain-language `README.md` explaining the economic and security reason;
- `src/lib.rs` with `todo!()` placeholders and numbered coding maps;
- tests that encode required invariants;
- a required learner-written adversarial or boundary test.

## How to study

1. Read the exercise README completely.
2. Complete its “explain before coding” checkpoint in your own words.
3. Read the tests and predict every assertion.
4. Implement one `todo!()` at a time.
5. Run `cargo test -p <package-name>`.
6. Add the required test marked `#[ignore]`, then remove `#[ignore]`.
7. Do not continue until you can explain the invariant without code.

## [Module A — Fixed-Point and Economic Modeling](./modules/module-a-fixed-point-economic-modeling/README.md)

| Exercise | Rust / math skill | Production Solana application |
|---|---|---|
| ex01 | scaled integers and checked `u128` math | deterministic on-chain prices |
| ex02 | floor and ceiling division | protocol-favorable solvency rounding |
| ex03 | ratio accounting and state transitions | share-based yield vaults |
| ex04 | time-based checked accrual | management and borrow fees |
| ex05 | constant-product invariant | AMM swaps and slippage protection |
| ex06 | piecewise-linear functions | utilization-based lending rates |
| ex07 | value ratios and thresholds | health factors and liquidations |
| ex08 | multi-account accounting model | overcollateralized stablecoins |

**Common interview question:** How do you implement financial calculations on-chain
without floating point, and how can rounding direction or share accounting make a
protocol insolvent?

## [Module B — Oracles](./modules/module-b-oracles/README.md)

| Exercise | Rust / Anchor skill | Production Solana application |
|---|---|---|
| ex09 | signed fixed-point normalization | Pyth price and confidence interpretation |
| ex10 | checked slot age and fail-closed state | stale-price rejection |
| ex11 | bounded weighted accumulation | TWAP manipulation resistance |
| ex12 | attacker-driven state sequencing | reproducing a spot-oracle exploit |
| ex13 | sorting, median, and deviation checks | multi-oracle circuit breakers |

**Common interview question:** What checks must a Solana lending protocol perform
before trusting an oracle price, and why is a fresh spot price still insufficient?

## Remaining modules

- [Module C — MEV and Ordering](./modules/module-c-mev-ordering/README.md)
- [Module D — CPI and Cross-Program Security](./modules/module-d-cpi-cross-program-security/README.md)
- [Module E — Upgrade Authority and Governance](./modules/module-e-upgrade-authority-governance/README.md)
- [Module F — Deployment and Operations](./modules/module-f-deployment-operations/README.md)
- [Module G — Security Review Methodology](./modules/module-g-security-review-methodology/README.md)
- [Module H — Fuzz Testing](./modules/module-h-fuzz-testing/README.md)
- [Module I — Zero-Knowledge Proofs](./modules/module-i-zero-knowledge-proofs/README.md)
- [Module J — Optimization](./modules/module-j-optimization/README.md)

## Appendices and capstones

- [Rust, Solana, security, DeFi, and ZK glossary](./GLOSSARY.md)
- [Ten production-oriented capstone specifications](./CAPSTONES.md)
