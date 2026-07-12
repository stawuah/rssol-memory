# ex05 — Result, errors, and `?`

## Rust idea

`Result<T, E>` makes failure explicit. `map_err` translates lower-level errors.
`?` extracts success or returns the error early.

## Solana connection

Anchor handlers return `Result<()>`. A program error aborts the instruction and
rolls back transaction state. Never `unwrap()` untrusted instruction data.

## How to solve it

1. Trim and reject empty input.
2. Parse `u64` and map the parsing error.
3. Reject zero.
4. Use `?` in `transfer` to obtain the amount.
5. Use checked subtraction and convert `None` into the domain error.
6. Run `cargo test -p ex05-result-and-question`.
