# ex02 — Ownership and moves

## Rust idea

Passing `String` by value moves ownership. Passing a reference borrows it, so the
caller retains ownership and can use it afterward.

## Solana connection

Rust ownership is a memory rule. Solana account ownership is a runtime field
naming the program allowed to modify account data. Anchor handlers normally
borrow account data temporarily.

## How to solve it

1. Compare the `String` and `&String` parameters.
2. Predict which caller binding remains usable.
3. Read the length through each parameter.
4. Do not clone the borrowed string.
5. Run `cargo test -p ex02-ownership`.

As an extension, change `&String` to `&str` and explain why it is more flexible.
