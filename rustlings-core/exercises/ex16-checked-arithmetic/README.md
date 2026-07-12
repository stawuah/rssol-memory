# ex16 — Checked fixed-point arithmetic

## Rust idea

Checked operations report overflow. A `u128` intermediate can safely hold many
`u64` products, while conversion back to `u64` can still fail.

## Solana connection

Balances, fees, shares, and prices are integer state. Units, precision, overflow,
and rounding direction are economic and security decisions.

## How to solve it

1. Convert operands to `u128` before multiplication.
2. Calculate basis-point growth using denominator `10_000`.
3. Use checked multiply, add, and divide.
4. Reject a zero denominator explicitly.
5. Convert back with `u64::try_from` and map conversion failure.
6. Document that integer division rounds down.
