# ex15 — Lifetimes and borrowed contexts

## Rust idea

Lifetimes describe how references relate. They do not keep values alive. A
returned reference must be tied to the input containing the referenced value.

## Solana connection

Anchor uses `'info` to relate account references valid during one instruction.
Memory safety does not prove account authorization; constraints still must.

## How to solve it

1. Read `'a` as the lifetime of the borrowed account slice.
2. Find an account by owner and return its existing reference.
3. Follow the context reference to the vault and then its key.
4. Return the borrowed `&str` without allocating.
5. Remove optional lifetime annotations to explore elision.
