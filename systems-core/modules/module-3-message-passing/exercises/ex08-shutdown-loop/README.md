# ex08 — Service loops and graceful shutdown

## Why this exists

A node's services run for weeks. Each one is a loop that must:

1. do work when it arrives,
2. do periodic things when idle (heartbeats, flushes, metrics), and
3. **shut down cleanly** when asked, without losing work already accepted.

You can't safely kill a thread from outside; it could be holding a lock or
be halfway through a write. So threads **check a stop flag** at safe points.
That's cooperative cancellation. `recv()` without a timeout is a trap: it can
block forever and never see the flag.

## Production Solana application

Every validator service thread takes an `exit: Arc<AtomicBool>` and checks it
in its loop, usually using `recv_timeout` so it wakes up regularly. The order of
a validator shutdown (stop accepting, drain, flush, join) is what prevents
corrupted ledgers on restart.

## Bug defended against

- A loop blocked in `recv()` that never notices shutdown (the process hangs on exit).
- Exiting immediately and dropping queued work that was already acknowledged.
- A busy-spin `try_recv` loop burning a whole core while idle.

## Wrong code

```rust,ignore
for v in rx {                         // blocks forever if producers never disconnect
    if stop.load(Relaxed) { return; } // only checked when a message arrives
    handle(v);
}
```

## Coding approach

`recv_timeout(tick)` gives you work, idle ticks, and regular stop checks from
one call. On stop, drain with `try_recv`.

## Testing

Disconnect exits after all items, an idle loop ticks and stops within 2 s, and
queued items are drained in order on shutdown.

## Explain before continuing

What is the worst-case delay between setting `stop` and the loop noticing?
How would you pick `tick` for a heartbeat vs. a WAL flush?

## Required learner test

With a 10 s tick and `stop` already set, assert `run_service` returns in
well under 1 s. Then write down: if a producer never stopped sending, could
the drain run forever? How would you bound it?
