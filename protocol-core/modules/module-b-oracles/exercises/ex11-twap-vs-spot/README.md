# ex11 — TWAP versus spot price

## Why this exists

A **spot price** is the price at one moment. A **time-weighted average price**
(TWAP) averages prices according to how long each price lasted. A brief manipulated
spot can strongly affect one transaction but should have limited effect on a long
TWAP window.

A TWAP is not automatically safe. The observation source, window length, update
frequency, gaps, overflow, and minimum covered duration all matter.

## Production Solana application

Perpetual funding, liquidation marks, lending collateral, and execution guards may
compare spot, EMA/TWAP, and confidence rather than trust one instantaneous value.

## Exploit or bug defended against

A flash-funded attacker can move a thin AMM's spot price, perform a valuable action,
then reverse the trade in the same transaction or short interval.

## Wrong code

```rust,ignore
let collateral_price = amm.reserve_quote / amm.reserve_base;
mint(collateral_amount * collateral_price); // one manipulable observation
```

## Coding approach

1. Require at least two ordered observations.
2. For each adjacent pair, weight the earlier price by elapsed slots.
3. Stop exactly at the requested window start/end.
4. Require sufficient covered duration.
5. Divide cumulative price-time by total duration.
6. Compare spot deviation from TWAP against a circuit threshold.

## Testing

Test constant price, uneven durations, out-of-order observations, insufficient
history, overflow, and a one-slot spike inside a long stable window.

## Explain before continuing

Explain why averaging three samples is not necessarily time-weighted and how a
longer window trades responsiveness for manipulation resistance.

## Required learner test

Add a one-slot price spike and show that spot moves much more than the TWAP.
