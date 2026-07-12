# ex11 — Traits as behavior contracts

## Rust idea

A trait defines behavior multiple types implement. An associated type lets each
implementation name its error type.

## Solana connection

Anchor uses traits for serialization, account loading, ownership, discriminators,
and instruction data. Small protocol traits can share validation behavior.

## How to solve it

1. Read the `Validate` contract.
2. Reject a zero deposit.
3. For withdrawal, reject zero before checking available funds.
4. Return `Ok(())` only when all rules pass.
5. Add one valid and one invalid test for each implementation.
