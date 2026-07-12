# ex03 — Deterministic PDA identity

## Rust idea

Iterating a borrowed seed slice in stable order can build a deterministic mock
identifier. This exercise does not implement Solana's cryptographic PDA algorithm.

## Solana model

A PDA derives from seeds, program ID, and bump. It has no private key. Seeds form
a protocol namespace and often bind an account to a market or authority.

## How to solve it

1. Begin the mock address with the program ID.
2. Iterate seeds in their given order.
3. Add an unambiguous separator before each seed.
4. Verify same inputs match and changed or reordered seeds differ.
5. In real Anchor code, use `seeds` and `bump` constraints—not this mock format.
