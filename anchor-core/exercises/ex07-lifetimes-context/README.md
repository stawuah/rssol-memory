# ex07 — Lifetimes in an account context

## Rust idea

`Context<'info, T>` holds a mutable reference valid for `'info`. Mutation occurs
through that exclusive reference.

## Solana model

Anchor contexts borrow validated account wrappers during one instruction. State
changes are later serialized to persistent account data.

## How to solve it

1. Follow `ctx.accounts` to the borrowed counter.
2. Read its value.
3. Increment through the mutable reference.
4. For production code, use checked addition and return an error.
5. Explain why lifetime safety does not prove account authorization.
