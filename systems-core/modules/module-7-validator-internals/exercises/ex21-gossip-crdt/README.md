# ex21 — Gossip state as a CRDT

## Why this exists

Gossip spreads cluster metadata (who is where, which version, recent votes)
without a coordinator: nodes periodically exchange what they have. Messages
arrive late, duplicated, and in any order. The data structure must make
**every order of merges produce the same state**. That's a **CRDT**
(conflict-free replicated data type). Merge must be:

- **commutative:** A∪B = B∪A
- **associative:** (A∪B)∪C = A∪(B∪C)
- **idempotent:** A∪A = A

Last-writer-wins per key, with a **deterministic** tie-break, satisfies all three.

## Production Solana application

This is Solana's gossip, the **CRDS** (Cluster Replicated Data Store). Values
are keyed by origin pubkey and label (ContactInfo, Vote, EpochSlots, …), signed
by their origin, and resolved by wallclock. Push and pull gossip, bloom-filter
pull requests, and pruning are built on top.

## Bug defended against

- A tie-break by arrival order: replicas diverge forever.
- **Far-future timestamps:** a value stamped `u64::MAX` can never be superseded,
  so one bad node freezes its key cluster-wide. Reject values beyond a drift bound.
- Overflow in `now + drift`.

## Wrong code

```rust,ignore
if v.wallclock >= existing.wallclock { replace }   // >= : order-dependent on ties; no future check
```

## Coding approach

`supersedes` is one tuple comparison. `upsert` uses the `BTreeMap` entry API.
`merge` is repeated upsert.

## Testing

LWW and idempotence, tie symmetry, the future-timestamp guard with saturation,
and random replicas converging under 4 merge orders.

## Explain before continuing

Why does a deterministic tie-break make LWW a CRDT? Real CRDS values are
**signed** by their origin: which attack does that prevent that this model
doesn't?

## Required learner test

What can a malicious node do with `wallclock = now + MAX_CLOCK_DRIFT_MS`, and
how long does the effect last?
