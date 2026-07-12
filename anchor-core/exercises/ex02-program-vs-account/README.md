# ex02 — Program code versus account state

## Rust idea

The program is borrowed read-only; the state account is mutably borrowed. A
`Result` reports a rejected state update.

## Solana model

Programs are executable code. Persistent mutable state lives in accounts supplied
by the caller, so every account boundary must be validated.

## How to solve it

1. Reject when the account owner differs from the program ID.
2. Reject zero amount.
3. Calculate the balance with checked addition.
4. Assign only after every check succeeds.
5. Test wrong owner, zero, overflow, and success.
