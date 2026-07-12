# ex13 — Bounded collections

## Rust idea

`Vec` stores ordered values. `HashMap` stores unique keys. Their APIs can detect
duplicates rather than silently accepting ambiguous input.

## Solana connection

Account space and compute are bounded. On-chain collections need maximum sizes,
paging, or one-PDA-per-item designs. Duplicate identities can corrupt accounting.

## How to solve it

1. Reject an existing member before checking capacity.
2. Reject when length has reached capacity.
3. Push only after validation.
4. Build the map one entry at a time.
5. Use insertion or the entry API to detect duplicate owners.
6. Verify errors do not alter existing collection state.
