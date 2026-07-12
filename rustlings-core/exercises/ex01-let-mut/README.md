# ex01 — Bindings and mutation

## Rust idea

`let` binds a name to a value. Bindings are immutable unless they use `mut`.
Explicit mutation makes state changes easy to locate and review.

## Solana connection

Rust `mut` permits local mutation. Anchor `#[account(mut)]` declares that a
transaction may write an account. They work at different layers, but both make
mutation explicit.

## How to solve it

1. Find the value that changes after creation.
2. Make only that binding mutable.
3. Add `step` exactly once.
4. Return the final counter.
5. Run `cargo test -p ex01-let-mut`.

Explain afterward why `start` and `step` do not need mutable bindings.
