# ex12 — Generics and trait bounds

## Rust idea

Generics reuse an algorithm across types. A trait bound states the behavior the
algorithm requires. Returning `&T` avoids copying the selected item.

## Solana connection

Anchor wrappers and helpers are generic. Understanding bounds helps decode what
an API requires, though concrete types often keep security boundaries clearer.

## How to solve it

1. Iterate over borrowed items.
2. Compare values returned by `amount()`.
3. Return `None` for an empty slice and a reference for a non-empty slice.
4. Use a short-circuiting iterator predicate for the limit check.
5. Do not clone or collect.
