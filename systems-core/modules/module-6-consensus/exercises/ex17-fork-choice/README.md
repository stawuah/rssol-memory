# ex17 — Fork choice

## Why this exists

When leaders build on different parents (network delay, a skipped leader, a
malicious leader), the chain **forks** into a tree. Every validator must pick
**one** branch to build and vote on, using a rule everyone computes the same
way, so the honest majority converges. The fork-choice rule plus the vote
thresholds (ex15) are what make a fork final.

**Heaviest subtree:** each validator's *latest* vote supports its slot and all
ancestors. From the root, always step into the child subtree with the most
supporting stake. This is not "longest chain": weight, not length.

## Production Solana application

Solana's `HeaviestSubtreeForkChoice` in the replay stage does exactly this,
combined with Tower BFT lockouts (a validator can't switch forks freely after
voting). Understanding fork choice is essential for anything touching
consensus, MEV (fork-aware bundles), or finality (processed vs. confirmed vs.
finalized commitment levels).

## Bug defended against

- Longest-chain thinking: a long fork built by one small validator wins.
- Counting a validator's *old* votes too (only the latest vote counts).
- Non-deterministic tie-breaking, which makes validators diverge.
- Accepting a child slot ≤ its parent (an invalid tree).

## Wrong code

```rust,ignore
let best = self.children.keys().max();          // highest slot = "longest", ignores stake
```

## Coding approach

`is_descendant` via parent walks, `subtree_stake` via that, then greedy descent
with a deterministic tie-break. (Production code caches subtree weights and
updates them incrementally; the O(n·v) version is fine for learning.)

## Testing

Tree validation, ancestry, heaviest vs. most voters, votes on inner nodes,
ties, and unknown votes.

## Explain before continuing

What are processed / confirmed / finalized commitment levels in terms of this
tree? Why do lockouts exist: what attack does free fork-switching allow?

## Required learner test

Moving one validator's latest vote from fork 1 to fork 2 changes `best_leaf`.
