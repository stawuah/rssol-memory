# ex07 — Byte slices and UTF-8 safety

## Rust idea

`&[u8]` can hold any bytes; `&str` must be valid UTF-8. Safe slicing requires a
validated range and, for strings, a character boundary.

## Solana connection

Instruction data, discriminators, PDA seeds, and account data are bytes. Bad
offsets must return an error or absence rather than panic.

## How to solve it

1. Locate the first `b':'` byte.
2. Exclude the delimiter and use `.get(range)` to borrow the remainder.
3. Return `None` when no delimiter exists.
4. Use `char_indices` to find the byte boundary after `max_chars` characters.
5. Fall back to `text.len()` when the string is shorter.
6. Run `cargo test -p ex07-slices-advanced`.
