# ex18 — Account-lock scheduling (Sealevel)

## Why this exists

Solana transactions **declare every account they'll touch** and whether they
write it. That's what lets the runtime run many transactions **in parallel**:
two transactions can run together unless they share an account that at least
one of them writes. Reads never conflict with reads.

A scheduler turns an ordered list of transactions into batches that can run
in parallel, while producing exactly the same result as running them one by
one. That equivalence is the invariant.

## Production Solana application

This is the banking stage's scheduler (the central scheduler / prio-graph in
Agave). It explains:
- why **hot accounts** (a popular AMM pool, a mint) serialize everything that
  writes them;
- why **local fee markets** exist (priority fees are per contended account);
- why programs should split state across accounts (sharding again).

Your on-chain design choices in protocol-core decide how this scheduler treats
your users.

## Bug defended against

- Letting two writers of one account run together, which gives nondeterministic
  state and a validator that disagrees with the cluster.
- Treating read/read as a conflict, which loses parallelism.
- Allowing duplicate or oversized account lists.

## Wrong code

```rust,ignore
// Pairwise O(n²) with no order preservation: a later tx can land in an earlier batch
for t in txs { let b = batches.iter().position(|b| b.iter().all(|x| !conflicts(x, t))); ... }
```

## Coding approach

Track the last batch that wrote and read each account. A tx's batch is one
past the latest conflicting use. That's O(total accounts), with no pairwise work.

## Testing

Validation, the conflict matrix, all-independent → one batch, a hot account
serializing, and a randomized check that batched execution (even reversed
inside batches) equals sequential execution.

## Explain before continuing

Why must a tx go *after* earlier conflicting txs rather than into any free
batch? How do address lookup tables (protocol-core ex47) affect the number of
accounts a tx can lock?

## Required learner test

100 transactions writing one hot account produce 100 batches. Explain what
that means for throughput and fees.
