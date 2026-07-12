# ex10 — Layered errors and atomic updates

## Rust idea

A custom error enum describes parsing, validation, and arithmetic failures. New
values should be calculated before mutation so failure cannot partially update
state.

## Solana connection

Returning an error rolls back a transaction. Atomic pure logic is still valuable
around CPIs, serialization, and tests.

## How to solve it

1. Parse and map invalid-number errors.
2. Reject zero.
3. Compute the new source with checked subtraction.
4. Compute the new destination with checked addition.
5. Assign both only after both calculations succeed.
6. Confirm the failure test leaves both values unchanged.
