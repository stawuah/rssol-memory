# ex15 — Byzantine quorums and stake

## Why this exists

**Byzantine** nodes can lie, crash, or equivocate (say different things to
different peers). The classic result: with `n` nodes you can tolerate `f`
Byzantine ones only if `n >= 3f + 1`. Why 2/3? Any two groups that each hold
more than 2/3 must **overlap** in more than 1/3. If fewer than 1/3 are faulty,
the overlap contains an honest node, and an honest node never votes for two
conflicting blocks. So two conflicting blocks can't both be confirmed.

In proof of stake, you count **stake**, not nodes.

## Production Solana application

Tower BFT and Alpenglow confirm blocks with > 2/3 of stake. The same threshold
decides optimistic confirmation, supermajority roots, and "is the cluster
halted?" (restarts need 80% of stake). **Equivocation** (duplicate blocks
from a leader, or conflicting votes) is detected and handled as evidence.
Bridges and light clients check the same supermajority against a validator set.

## Bug defended against

- Counting heads instead of stake.
- Using only the voters' stake as the total: 3 voters out of 100 validators
  would "confirm".
- Floats (`0.6667`) or `u64` overflow in `voted * 3`.
- Counting a duplicated network message twice.
- Accepting two votes from one validator for different blocks.

## Wrong code

```rust,ignore
let total: u64 = votes.iter().map(|v| stake[&v.0]).sum();   // voters only
if voted as f64 / total as f64 >= 0.66 { ... }               // float, and >= 0.66
```

## Coding approach

Integer math in `u128`, a `HashMap<voter, block>` for dedup and equivocation,
and a `BTreeMap` for deterministic output.

## Testing

The fault-tolerance table, strictness at exactly 2/3, stake vs. headcount,
non-voters in the total, duplicates, equivocation, and unknown voters.

## Explain before continuing

Prove the overlap argument with numbers (total 100, two sets of 67). Why
strictly greater than 2/3?

## Required learner test

Show that an attacker with 1/3 of stake can't get two blocks confirmed without
equivocating, and that `tally` catches the equivocation.
