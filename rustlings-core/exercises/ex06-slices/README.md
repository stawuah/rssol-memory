# ex06 — Borrowed string slices

## Rust idea

`&str` borrows UTF-8 text. `find` returns a byte boundary, and a range can return
a prefix without allocating.

## Solana connection

Account and instruction data are byte sequences. Borrowed views can reduce
allocation and compute, but every boundary must be validated.

## How to solve it

1. Search for the delimiter.
2. Use the input length when it is absent.
3. Slice from zero to the discovered endpoint.
4. Return the borrowed view directly.
5. Test delimiter-first, delimiter-last, and delimiter-absent cases.
