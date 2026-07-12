# ex13 — Multi-oracle aggregation and circuit breakers

## Why this exists

No oracle is failure-proof. **Median-of-N aggregation** sorts independent valid
prices and chooses the middle value, limiting the influence of a minority of
extreme outliers. For an even number, this exercise uses the checked average of
the two middle prices.

A **deviation threshold** rejects sources too far from the median. A **circuit
breaker** pauses a risky action when inputs disagree, too few sources remain, or
the aggregate moves too far from the last accepted value.

## Production Solana application

High-value lending, stablecoin, and derivative systems may combine Pyth,
Switchboard, DEX TWAPs, or other independently governed sources.

## Exploit or bug defended against

It limits damage from one compromised source, stale source, configuration error,
or sudden implausible aggregate jump. “Multiple” sources do not help if they share
the same underlying dependency.

## Wrong code

```rust,ignore
let average = sources.iter().map(|p| p.price).sum::<u64>() / sources.len() as u64;
// One extreme value can dominate; stale sources are included; sum can overflow.
```

## Coding approach

1. Filter sources by freshness without mutating input.
2. Require a quorum.
3. Sort copied prices and calculate median safely.
4. Reject any accepted source whose deviation from median exceeds policy.
5. Compare new median with last accepted value.
6. Return a circuit-breaker error instead of a questionable price.

## Testing

Test odd/even counts, one extreme outlier, stale sources, insufficient quorum,
duplicate sources, exact deviation boundary, and aggregate jump breaker.

## Explain before continuing

Explain why three feeds that all derive from the same exchange are not three
independent trust sources, and compare median with arithmetic mean under one outlier.

## Required learner test

Add a scenario where one source is stale and one is an outlier; assert whether the
remaining quorum should succeed according to your documented policy.
