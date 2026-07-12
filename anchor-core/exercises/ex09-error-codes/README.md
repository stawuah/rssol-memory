# ex09 — Protocol error codes

## Rust idea

`Result<(), ProtocolError>` returns either success or one domain-specific failure.

## Solana model

Anchor errors become program error codes clients can identify. Returning an error
aborts the instruction and rolls back transaction state.

## How to solve it

1. Compare `amount` with the inclusive maximum.
2. Return `AmountTooLarge` only when amount is greater.
3. Treat equality as valid.
4. Return `Ok(())` otherwise.
5. Test zero, exact maximum, and maximum plus one.
