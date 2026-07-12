# ex12 — Build and exploit a vulnerable spot oracle

## Why this exists

Security concepts become concrete when you reproduce the exploit. This exercise
models a lending program that prices collateral directly from an AMM's current
reserve ratio. The learner must implement the vulnerable path, implement the
attacker's reserve manipulation, demonstrate excess borrowing, and then implement
a guarded path using an independent reference and deviation limit.

The model is intentionally small. A real attack may use flash liquidity, multiple
CPIs, transaction ordering, and exact pool mechanics.

## Production Solana application

Protocols often compose with DEX pools. A pool account is not an oracle simply
because it contains a price-like reserve ratio.

## Exploit or bug defended against

An attacker temporarily makes collateral appear expensive, borrows more value than
the true collateral supports, then restores the pool and leaves bad debt.

## Wrong code

The vulnerable function in this exercise is the wrong code: it trusts one mutable
pool and performs no freshness, liquidity, TWAP, confidence, or deviation check.

## Coding approach

1. Implement the pool spot ratio with scaled wide math.
2. Implement a swap-like reserve change that raises collateral's quoted price.
3. Use the manipulated price in `vulnerable_borrow_limit`.
4. Write the exploit test showing limit growth.
5. Implement `guarded_borrow_limit` using an independent price and deviation cap.
6. Ensure guard failure produces no lending-state mutation in your added test.

## Testing

Test normal reserves, manipulation magnitude, insufficient reserve, zero reserve,
deviation exactly at/above limit, and guarded rejection.

## Explain before continuing

Describe the attack transaction step by step, identify which state the attacker
controls, and name the missing trust boundary.

## Required learner test

The ignored exploit test is mandatory: implement the manipulation sequence and
prove vulnerable borrowing exceeds the safe limit while the guarded path rejects.
