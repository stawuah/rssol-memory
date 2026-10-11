# ex20 — Turbine: fanout trees for block propagation

## Why this exists

A leader can't send every block to 2,000+ validators itself; its uplink would
saturate. Instead it sends each piece (a **shred**) to a few nodes, which
forward it to a few more, and so on: a **tree**. With fanout `k`, `n` nodes
are reached in about `log_k(n)` hops (2,000 nodes with fanout 200 take 2 hops).

The tree must be:
- **deterministic:** every node computes the same tree from public inputs, so it
  knows whom to forward to without coordination;
- **re-shuffled per shred:** a node isn't always near the root, which spreads
  bandwidth load and limits what one malicious node can withhold.

## Production Solana application

This is Turbine. Real Turbine also weights the shuffle by stake (high-stake
nodes nearer the root), uses erasure coding (so missing shreds can be rebuilt),
and falls back to **repair** when shreds are lost. Block propagation latency
directly affects confirmation times and skip rates.

## Bug defended against

- Non-deterministic shuffles, which give inconsistent trees and nodes that
  never receive shreds.
- Overflow in `fanout * index`.
- A fixed tree that turns one node into a permanent bottleneck or censor.

## Wrong code

```rust,ignore
let mut order = nodes.to_vec(); order.shuffle(&mut rand::thread_rng()); // every node gets a different tree
```

## Coding approach

A seeded Fisher–Yates shuffle, then heap-style index math for
`children`/`parent`, then `retransmit_peers` to combine them.

## Testing

Permutation and determinism, tree shape, overflow safety, exactly one parent
per node, the 2-hop claim, and full coverage by retransmission.

## Explain before continuing

What does a larger fanout trade (fewer hops vs. per-node bandwidth)? What does
erasure coding buy you when a subtree's parent is offline?

## Required learner test

A node's position changes across shred seeds.
