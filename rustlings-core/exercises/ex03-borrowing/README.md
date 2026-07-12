# ex03 — Shared and mutable borrowing

## Rust idea

`&T` gives shared read access. `&mut T` gives one exclusive writer. A slice
borrows part of existing data without allocation.

## Solana connection

Instructions borrow account data during execution. Writable account locks help
the runtime schedule transactions, but Rust borrowing does not replace owner,
signer, authority, or PDA validation.

## How to solve it

1. Mutate the string through `&mut String` using its append method.
2. For `head`, return a borrowed range instead of a new `String`.
3. Remember that string ranges use byte offsets.
4. Run `cargo test -p ex03-borrowing`.
5. Add a test that demonstrates a bad UTF-8 byte boundary.
