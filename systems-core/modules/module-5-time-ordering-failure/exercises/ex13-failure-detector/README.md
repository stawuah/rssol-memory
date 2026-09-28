# ex13 — Failure detection

## Why this exists

Across a network you **cannot tell** a dead node from a slow one or from a
broken link. All you have is "I haven't heard from it for T". A failure
detector turns silence into a *suspicion level*. The trade-off is fundamental:
a short timeout detects failures fast but falsely accuses slow nodes; a long
timeout is accurate but slow. Real systems use graded states (alive → suspect
→ dead) and adaptive timeouts (the phi-accrual detector).

**Inject the clock** (`now: Instant` parameters) instead of calling
`Instant::now()` inside. That makes timing logic deterministic and testable,
which is a senior habit.

## Production Solana application

Gossip marks peers stale when their contact info stops refreshing. Repair
avoids unresponsive peers. RPC load balancers, Jito relayers, and your own
infrastructure all need health checks with the same states.

## Bug defended against

- `now - last` panicking or underflowing when timestamps arrive out of order.
- Delayed heartbeats moving `last_seen` backwards.
- Unbounded maps of nodes that left forever.

## Wrong code

```rust,ignore
let silence = now - self.last_seen[&node];   // panics if unknown or if now < last
```

## Coding approach

Keep the max timestamp per node, use `saturating_duration_since`, and classify
with `>=` thresholds.

## Testing

Exact thresholds, reordered heartbeats, and unhealthy listing plus eviction.

## Explain before continuing

Why can no timeout be both fast and never wrong (the FLP / partial-synchrony
intuition)? What should a node do with a *Suspect* peer that it wouldn't do
with a *Dead* one?

## Required learner test

Heartbeats every `suspect_after - 1ms` never produce Suspect.
