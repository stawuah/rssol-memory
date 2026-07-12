# Rustlings Core

This folder is the Rust memory builder before Anchor.

## What this is for

- learn how Rust controls data access
- stop guessing about ownership and borrowing
- get comfortable with enums, `match`, `Result`, and slices
- build the Rust habits that make Anchor feel readable later

## How to use it

1. Open one exercise at a time.
2. Read the test first.
3. Predict the behavior before editing code.
4. Replace the `todo!()` with your answer.
5. Run `cargo test --workspace`.
6. Do not move to the next exercise until the current one passes.

## Exercise order

- `ex01-let-mut`: mutable bindings
- `ex02-ownership`: move semantics and borrowing by reference
- `ex03-borrowing`: shared and mutable borrows
- `ex04-struct-enum-match`: state machines with enums and `match`
- `ex05-result-and-question`: explicit error handling and `?`
- `ex06-slices`: borrowed views into strings
- `ex07-slices-advanced`: safe byte slices, UTF-8 boundaries, and zero-copy parsing
- `ex08-structs`: constructors, methods, updates, and protocol invariants
- `ex09-enums`: data-carrying variants and exhaustive state transitions
- `ex10-errors`: layered custom errors, `map_err`, and error propagation
- `ex11-traits`: shared validation behavior and trait implementations
- `ex12-generics`: reusable functions with meaningful trait bounds
- `ex13-collections`: bounded vectors, maps, and duplicate detection
- `ex14-iterators`: allocation-free searching and checked aggregation
- `ex15-lifetimes`: returned borrows and Anchor-style context references
- `ex16-checked-arithmetic`: overflow-safe balances and fixed-point calculations

## Further recommended exercises

These are useful follow-ups after the core exercises:

- `serialization-space`: byte layouts, discriminators, and account sizing
- `modules-tests`: crate structure, unit tests, and failure-path tests
- `security-invariants`: authority, owner, signer, PDA, and account-alias checks

Use the [formatted Rust + Anchor Field Guide](./RUST-ANCHOR-CHEAT-SHEET.pdf) while
working through both `rustlings-core` and `anchor-core`. The editable Markdown
source is [here](./RUST-ANCHOR-CHEAT-SHEET.md).

## Rules

- no copying answers from elsewhere
- read the compiler error carefully
- keep the code deterministic
- prefer the smallest correct fix

## Why this matters for Anchor

Anchor programs are mostly Rust state machines with account validation.
If you can reason about ownership, borrowing, and state transitions, Anchor becomes a thin layer instead of a wall.

## Analogies

- `let x = ...` is placing a label on a box.
- `mut` is a box you are allowed to reopen and edit.
- ownership is the one key that can move from hand to hand.
- `&T` is a read-only library loan card.
- `&mut T` is the one person in the room allowed to edit the book.
- `struct` is a form with fixed fields.
- `enum` is a door with labeled branches.
- `match` is the security guard that checks every branch.
- `Result` is a transaction receipt: success or failure, no guessing.
- `&str` and slices are borrowed views into existing data, not copies.

## Anchor translation

- a Solana account is a shared record onchain
- a program is the logic that updates records
- a PDA is a derived address without a private key
- account validation is Rust plus constraints plus trust boundaries
- instruction handlers are state transitions with explicit inputs
