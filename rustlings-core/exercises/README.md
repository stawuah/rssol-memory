# Exercises

Work through the crates in numeric order. Each crate isolates one Rust idea and
connects it to a habit needed in Solana programs.

Every exercise folder contains its own `README.md`. Read it first, then follow
the numbered guidance beside each `todo!()` in `src/lib.rs`.

Run one exercise while learning:

```sh
cargo test -p ex07-slices-advanced
```

Run the whole set after completing every `todo!()`:

```sh
cargo test --workspace
```

## Core sequence

1. bindings and mutability
2. ownership
3. borrowing
4. structs, enums, and matching
5. errors and `?`
6. slices
7. advanced slices and UTF-8 safety
8. structs and invariant-preserving methods
9. enums as state machines
10. custom error layers
11. traits
12. generics and trait bounds
13. bounded collections
14. iterators and checked folds
15. lifetimes and borrowed contexts
16. checked and fixed-point arithmetic

Each crate here is intentionally incomplete.

## What to do

- open the `src/lib.rs`
- read the test
- replace the `todo!()` with your implementation
- rerun tests until the crate passes

## Hint style

If you get stuck, ask yourself:

- What is owned?
- What is borrowed?
- What can change?
- What must never change?
- What does the test prove?
