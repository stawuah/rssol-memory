# Module 5 — Time, Ordering, and Failure

Level: **Senior**. Distributed systems have no shared clock and can't tell
slow from dead. This module covers logical time, failure detection with
injected clocks, and retry policies that don't turn blips into outages.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex12 | Lamport and vector clocks, causality comparison | ordering events without wall clocks; conflict detection |
| ex13 | injected `Instant`, graded health states, bounded maps | gossip staleness, repair peer selection, health checks |
| ex14 | exponential backoff, full jitter, budgets, injected sleep/rng | transaction submission, RPC clients, keepers |

## Ordered exercises

1. [Logical clocks](./exercises/ex12-logical-clocks/README.md)
2. [Failure detector](./exercises/ex13-failure-detector/README.md)
3. [Retry and backoff](./exercises/ex14-retry-backoff/README.md)

## Coverage checklist

- [x] happened-before, concurrency detection
- [x] alive/suspect/dead, reordered heartbeats
- [x] retryable vs permanent errors, time budgets, deterministic tests

**Common interview question:** Why can't you use wall-clock timestamps to order
events across validators, and what does Solana use instead?
