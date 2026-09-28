# ex16 — Raft leader election

## Why this exists

Raft is the consensus algorithm to learn first. It tolerates *crash* faults
(not Byzantine ones), and its election rules show every core consensus idea:

- **Terms** are logical epochs. A higher term always wins, and anyone who sees
  one steps down to follower.
- **One vote per term** per node, which is how two leaders in one term become
  impossible.
- A **majority** (> n/2) is needed, and any two majorities overlap.
- The **up-to-date log rule**: never elect a node missing committed entries.

## Production Solana application

Solana's consensus is BFT (ex15, ex17), not Raft. But Raft is everywhere around
it: etcd/Consul in validator operations tooling, sequencers, bridges' off-chain
committees, indexers, and databases (TiKV, CockroachDB). "Explain leader
election and split-brain" is a standard senior interview question.

## Bug defended against

- **Split brain:** two leaders in the same term because a node voted twice.
- Electing a node with a stale log, which loses committed data.
- A leader that ignores a higher term and keeps acting as leader.
- Counting a duplicated vote reply twice.

## Wrong code

```rust,ignore
if req.term >= self.term { return VoteReply { granted: true, .. } } // votes for everyone
```

## Coding approach

Implement `log_is_up_to_date`, then the three handlers in the order of the
coding maps. `step_down` is provided.

## Testing

The up-to-date rule, winning with a majority, one vote per term, stale logs
losing, higher-term step-down, and duplicate replies.

## Explain before continuing

Why must a node that *rejects* a vote still adopt the higher term? Why do Raft
implementations randomize election timeouts?

## Required learner test

A split vote in a 4-node cluster: no leader; the next term resolves it.
