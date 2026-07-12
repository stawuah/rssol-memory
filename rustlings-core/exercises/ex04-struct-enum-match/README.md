# ex04 — Structs, enums, and state transitions

## Rust idea

Structs group state. Enums represent one of several commands or errors. `match`
forces every command variant to be handled.

## Solana connection

Account data is commonly a struct. Instructions and protocol phases form a
state machine: validated current state plus a command produces new state or an
explicit error.

## How to solve it

1. Write the invariants: no zero amount, no balance changes while frozen, and no
   withdrawal above the balance.
2. Match every `Command` variant.
3. Validate before changing fields.
4. Return the matching `VaultError` when a rule fails.
5. Return success after a legal transition.
6. Run `cargo test -p ex04-struct-enum-match`.
