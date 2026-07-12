# ex04 — Per-second fee accrual

## Why this exists

Protocols often quote an annual fee but accrue it as time passes. **Accrual** means
recognizing an amount that has accumulated since the last recorded timestamp.
Multiplying assets, rate, and elapsed seconds in `u64` can overflow before division.

This simplified exercise uses linear simple interest rather than compounding. The
rate is parts-per-million (PPM) per year: `10_000 PPM` means one percent annually.

## Production Solana application

Vault management fees, lending interest, funding payments, and reward indexes all
use time- or slot-based accrual.

## What this defends against

It prevents overflow, time moving backward, double accrual, free tiny fees caused
by floor rounding, and unbounded catch-up intervals.

## What wrong code looks like

```rust,ignore
fee = assets * annual_rate * elapsed / 1_000_000 / SECONDS_PER_YEAR;
last_update = now; // changed before calculation is known to succeed
```

## How to code it

1. Reject `now < last_update`.
2. Cap elapsed time to the configured maximum interval.
3. Widen and checked-multiply assets, rate, and elapsed.
4. Divide with ceiling rounding so non-zero accrued liability is not lost.
5. Update accrued fees and timestamp only after success.

## How to test it

Test one year, zero elapsed time, tiny non-zero fee, backward time, capped elapsed
time, overflow/conversion, and two sequential accrual calls.

## Explain before continuing

Explain why updating the timestamp before fee math succeeds can forgive debt, and
why a maximum accrual interval may be an operational safety control.

## Required learner test

Add a sequential-accrual test and compare it with one combined interval under the
documented rounding policy.
