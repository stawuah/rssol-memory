# Module 6 — Consensus

Level: **Senior**. The core of protocol engineering: quorums, elections, and
fork choice. You'll implement the rules and prove the invariants with tests:
no two conflicting confirmations, no two leaders in a term, one deterministic
fork choice.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex15 | u128 threshold math, dedup + equivocation detection | > 2/3 stake confirmation, slashable evidence |
| ex16 | state machine with terms, votes, and log checks | Raft (sequencers, ops tooling, databases) |
| ex17 | tree building, ancestry, stake-weighted descent | heaviest-subtree fork choice (Tower BFT) |

## Ordered exercises

1. [Stake quorum](./exercises/ex15-stake-quorum/README.md)
2. [Raft election](./exercises/ex16-raft-election/README.md)
3. [Fork choice](./exercises/ex17-fork-choice/README.md)

## Coverage checklist

- [x] n ≥ 3f + 1 and the quorum-overlap argument
- [x] stake-weighted, total-stake-denominated thresholds
- [x] one vote per term, up-to-date log rule, step-down
- [x] latest-vote heaviest subtree with deterministic ties

**Common interview question:** Explain why Solana needs > 2/3 of stake to
confirm, what equivocation is, and what the fork-choice rule does when two
forks are close in weight.
