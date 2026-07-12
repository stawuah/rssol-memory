# ex08 — Invariant-preserving structs

## Rust idea

Constructors establish valid starting state. Methods control updates. A method
taking `self` consumes a value and can represent a terminal operation.

## Solana connection

An Anchor account struct stores protocol state. Initialization and every update
must preserve its invariants; closing is a terminal transition.

## How to solve it

1. Initialize the supplied owner, zero balance, and active status.
2. Reject deposits to inactive positions.
3. Use checked addition before assigning the new balance.
4. Permit close only at zero balance.
5. Return the owned owner string from the consumed struct.
6. Add overflow and successful-close tests.
