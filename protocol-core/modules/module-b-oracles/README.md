# Module B — Oracles

This module treats every external price as untrusted input. It teaches decoding,
freshness, uncertainty, time-weighting, manipulation reproduction, source
independence, and circuit breakers.

## Map

| Exercise | Rust / Anchor skill | Solana production application |
|---|---|---|
| ex09 | signed decimal normalization | Pyth price and confidence interpretation |
| ex10 | checked slot arithmetic and atomic failure | max-age enforcement and fail-closed borrowing |
| ex11 | bounded weighted accumulation | TWAP versus spot manipulation resistance |
| ex12 | adversarial state sequencing | hands-on spot-oracle manipulation exploit |
| ex13 | identity checks, sorting, median | multi-oracle aggregation and circuit breakers |

## Ordered exercises

1. [Pyth price and confidence](./exercises/ex09-pyth-price-confidence/README.md)
2. [Oracle staleness](./exercises/ex10-oracle-staleness/README.md)
3. [TWAP versus spot](./exercises/ex11-twap-vs-spot/README.md)
4. [Oracle manipulation exploit](./exercises/ex12-oracle-manipulation/README.md)
5. [Multi-oracle aggregation](./exercises/ex13-oracle-aggregation/README.md)

## Coverage checklist

- [x] Pyth-style price, exponent, and confidence interval
- [x] maximum slot age and stale-mid-operation failure
- [x] TWAP versus manipulable spot
- [x] learner-written exploit against a vulnerable price consumer
- [x] median-of-N, source deviation, quorum, and aggregate circuit breaker

**Common interview question:** What must a lending program validate before using an
oracle price, and why is “fresh” not synonymous with “safe”?
