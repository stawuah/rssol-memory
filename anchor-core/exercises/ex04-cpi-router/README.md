# ex04 — Cross-program invocation routing

## Rust idea

Validate a data-carrying struct and return explicit success or failure.

## Solana model

A CPI invokes another program while forwarding accounts and privileges. The
target program and forwarded account list are a trust boundary.

## How to solve it

1. Reject a zero amount before routing.
2. Accept only the expected `system` target.
3. Return the specified success value.
4. Reject every unknown target.
5. Add a wrong-program test.

Real CPI code must constrain program ID, accounts, signer seeds, writable flags,
and errors.
