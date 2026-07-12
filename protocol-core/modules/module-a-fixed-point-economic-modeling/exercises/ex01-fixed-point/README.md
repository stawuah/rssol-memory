# ex01 — Fixed-point numbers (Q32.32)

## Why this exists

Financial programs need fractions such as `1.25`, but floating-point operations
can introduce architecture-dependent behavior and surprising rounding. Consensus
software needs every validator to calculate the same integer result.

**Fixed-point representation** stores a scaled integer. Q32.32 uses 32 bits for
the whole-number portion and 32 bits for the fractional portion. The stored value
for `1.0` is `2^32`; the stored value for `1.5` is `1.5 * 2^32`.

Q64.64 uses the same idea with 64 fractional bits and usually needs wider
intermediates. This exercise uses Q32.32 so the representation fits in `u64` and
products can be calculated in `u128`.

## Production Solana application

Prices, exchange rates, interest indexes, and collateral ratios need deterministic
fractional math. On-chain code should define scale, overflow behavior, and rounding
instead of relying on implicit conversions.

## What this defends against

It prevents nondeterministic or imprecise floating-point accounting and catches
overflow that could otherwise create or erase protocol value.

## What wrong code looks like

```rust,ignore
let price = 1.10_f64;
let value = amount as f64 * price;

// Also wrong: multiplication can overflow before division reduces it.
let result = a * b / SCALE;
```

## How to code it

1. Reject a zero denominator.
2. Widen operands to `u128` before multiplying by the scale.
3. Divide only after the checked multiplication.
4. Convert the result back to `u64` with `try_from`.
5. For multiplication, multiply two raw values widely, then divide by the scale.
6. Document that integer division rounds down.

## How to test it

Test exact integers, exact binary fractions, a non-exact fraction, zero denominator,
overflow, multiplication by zero, and multiplication by one.

## Explain before continuing

Write two sentences explaining why `a * b / SCALE` can overflow even when the final
answer fits, and why fixed-point division loses fractional remainder.

## Required learner test

Replace the ignored test with a boundary case not already covered and explain which
invariant it protects.
