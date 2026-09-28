# ex06 — A bounded pipeline with fan-out and fan-in

## Why this exists

A pipeline is a chain of stages joined by queues. Each stage does one job, and a
slow stage gets more workers (**fan-out**) that feed one downstream queue
(**fan-in**). **Bounded** queues are what make it production-safe: when a
downstream stage is slow, upstream stages block on `send` (**backpressure**)
instead of buffering until the process runs out of memory.

Bounded means "at most `capacity` waiting at once", not "a known total".
The input could be infinite.

## Production Solana application

The TPU (Transaction Processing Unit) is exactly this: fetch stage →
**sigverify** (parallel, the expensive stage) → banking stage → broadcast.
Each arrow is a channel. Overload handling ("what happens when sigverify can't
keep up?") is a core protocol-engineering question.

## Bug defended against

- **Join-before-drain deadlock:** the collector joins the workers first; the
  workers are blocked sending into a full output queue; nobody makes progress.
- **Forgotten sender:** the collector's own `out_tx` is still alive, so the
  `for` loop never ends.
- **Holding the shared-receiver lock while verifying:** fan-out collapses back to
  one worker at a time.

## Wrong code

```rust,ignore
for w in workers { w.join().unwrap(); }        // deadlock with bounded queues
for (id, ok) in out_rx { /* never reached */ }
```

## Coding approach

Build both channels, spawn the producer and the workers, drop your own `out_tx`,
drain `out_rx`, and only then join.

## Testing

Exact counts for 1/2/8 workers × capacity 0/1/64, plus a watchdog on a
capacity-1 run of 20,000 packets.

## Explain before continuing

Trace what blocks on what in the join-before-drain bug. With capacity 0, how
many packets can be "in flight" at once?

## Required learner test

An empty packet list returns zero stats promptly.
