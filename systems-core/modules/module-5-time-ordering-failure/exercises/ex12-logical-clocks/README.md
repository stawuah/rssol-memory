# ex12 — Logical clocks

## Why this exists

In a distributed system there is **no shared clock**. Wall clocks drift, jump
(NTP corrections), and disagree between machines. So "which happened first?"
has to be answered with **causality**: A *happened before* B if A could have
influenced B (same node earlier, or A's message reached B's node before B).

- **Lamport clock:** one number. It guarantees A→B ⇒ L(A) < L(B). It's cheap,
  and enough for a consistent total order (tie-break by node id).
- **Vector clock:** one number per node. It can tell "A before B" apart from
  "concurrent", which is what you need to **detect conflicts**.

## Production Solana application

Solana replaces wall-clock agreement with **Proof of History** (a verifiable
sequence of hashes, ex22) and slots. Gossip values carry wallclocks but resolve
conflicts deterministically (ex21). Off-chain (bridges, sequencers, CRDT-based
indexers, multi-region services) you'll use Lamport or hybrid logical clocks.
Interviewers ask this constantly.

## Bug defended against

- Ordering events by `SystemTime::now()` across machines.
- Last-writer-wins by wall clock silently discarding concurrent writes.

## Wrong code

```rust,ignore
if msg.timestamp > local.timestamp { local = msg }  // wall-clock LWW: skew decides
```

## Coding approach

Lamport is two lines. For vector clocks, compare over the **union** of keys;
a missing key means 0.

## Testing

Lamport receive jumps past the sender, and vector clocks give Before, After,
Equal, and Concurrent, plus merge behavior.

## Explain before continuing

Why can't a Lamport clock detect concurrency? What does a vector clock cost
as the cluster grows to 2,000 validators?

## Required learner test

Two events with Lamport a < b that a vector clock proves are Concurrent.
