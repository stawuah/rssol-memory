# ex10 — Staleness and fail-closed execution

## Why this exists

A price is **stale** when it is older than the protocol permits. Slot age is
`current_slot - publish_slot`. A feed can stop updating because of market hours,
network problems, publisher failure, or update-delivery failure.

**Fail closed** means the risky operation returns an error instead of continuing
with uncertain data. If a price becomes stale during an instruction's logic, the
instruction must not leave partial state. Solana transaction errors roll changes
back, but pure business logic should still calculate before committing.

## Production Solana application

Borrowing, minting, withdrawals, liquidations, and settlement each need a documented
freshness policy. Emergency actions may intentionally use different policy.

## Exploit or bug defended against

An attacker can borrow or withdraw against an old high collateral price after the
real market crashes. Incorrect subtraction can also treat a future slot as fresh.

## Wrong code

```rust,ignore
position.debt += amount;
if current_slot - price.publish_slot > max_age { return Err(Stale); }
```

## Coding approach

1. Reject publish slots in the future using checked subtraction.
2. Treat age equal to `max_age` as valid and greater as stale.
3. Validate freshness before computing new debt.
4. Compute all new state in temporary values.
5. Commit only after price, confidence, and collateral checks succeed.

## Testing

Test age zero, exact maximum, maximum plus one, future slot, state unchanged on
failure, and a price that is fresh for the first check but stale at supplied
execution slot.

## Explain before continuing

Explain why “last known price” is not automatically safe, and which protocol
operations you would pause when prices are unavailable.

## Required learner test

Add a future-slot or exact-boundary test not already present and assert state is
unchanged.
