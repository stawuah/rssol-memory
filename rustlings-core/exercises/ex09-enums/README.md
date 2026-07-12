# ex09 — Enums as protocol phases

## Rust idea

Enum variants can carry state. Matching `(state, action)` creates an exhaustive
transition table.

## Solana connection

Clients can call public instructions in any order. The program—not the UI—must
reject illegal transitions such as depositing into a paused market.

## How to solve it

1. List every legal `(MarketState, Action)` pair.
2. Initialize only from `Uninitialized`.
3. Deposit only while active and use checked addition.
4. Preserve deposits when pausing or resuming.
5. Close only when deposits are zero.
6. Reject all remaining pairs as invalid transitions.
