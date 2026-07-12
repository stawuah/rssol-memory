# ex14 — Lazy iterators and checked aggregation

## Rust idea

`find` returns the first match. `try_fold` carries an accumulator that can stop
on failure. Both can work without an intermediate allocation.

## Solana connection

Iteration costs compute and must be bounded. Totals used in accounting need
checked arithmetic.

## How to solve it

1. Iterate over borrowed positions.
2. Find the matching owner and return that borrowed position.
3. Begin aggregation at `0_u64`.
4. Use `checked_add` for every fold step.
5. Let overflow stop the fold with `None`.
6. Test empty input and a valid total.
