# ex09 — Pyth price and confidence

## Why this exists

An **oracle** supplies external facts, such as an asset's market price, to a
program. Pyth reports a signed integer price, a base-10 exponent, and a confidence
interval. If `price = 12_276_250` and `exponent = -5`, the human price is
`122.76250`. The confidence uses the same exponent.

A **confidence interval** expresses uncertainty around the aggregate price. A
price of `$100 ± $3` is less precise than `$100 ± $0.05`. Protocols should reject
or conservatively adjust a feed when confidence is too wide for their risk model.

This exercise models decoded Pyth data rather than depending on a specific SDK.
Real integration must also constrain the expected feed account/program and use the
current supported Pyth receiver API.

## Production Solana application

Lending, stablecoins, perps, swaps, and liquidation engines translate oracle data
into one internal scale before applying economic rules.

## Exploit or bug defended against

Wrong exponent handling can misprice collateral by powers of ten. Ignoring wide
confidence can allow risky borrowing or unfair liquidation during volatile markets.

## Wrong code

```rust,ignore
let dollars = update.price as u64; // ignores exponent and negative prices
let collateral_value = amount * dollars; // ignores uncertainty
```

## Coding approach

1. Require positive price and non-negative confidence.
2. Normalize price and confidence to micro-USD (`10^-6`) using powers of ten.
3. Use checked `u128` math for scaling up and explicit division for scaling down.
4. Calculate confidence as basis points of price.
5. Reject confidence above the configured maximum.
6. For collateral valuation, use conservative `price - confidence`.

## Testing

Test exponents above, equal to, and below `-6`; negative/zero price; confidence
wider than price; maximum confidence; and scaling overflow.

## Explain before continuing

Explain why confidence is not the probability that the price is correct, and why
a lending protocol might value collateral at the lower confidence bound.

## Required learner test

Add a different exponent conversion case and prove that price and confidence keep
the same scale.
