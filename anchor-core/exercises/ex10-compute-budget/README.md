# ex10 — Compute budget boundaries

## Rust idea

Translate “fits within the available budget” into one inclusive comparison.

## Solana model

Execution is metered in compute units. Unbounded loops, scans, allocation, logs,
PDA derivation, serialization, and CPIs can make an instruction unusable at its
maximum state size.

## How to solve it

1. Decide whether equality fits.
2. Return the comparison directly.
3. Test zero, one below, equal, and one above.
4. Remember that estimation is not runtime measurement; profile real worst cases.
