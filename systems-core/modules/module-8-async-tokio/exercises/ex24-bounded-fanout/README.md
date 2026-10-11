# ex24 — Bounded concurrent fan-out

## Why this exists

"Fetch these 10,000 accounts / blocks / peer statuses" is everyday
infrastructure work. Spawning 10,000 tasks at once overwhelms the remote
service (and gets you rate-limited), exhausts sockets, and spikes memory.
Doing them one by one is too slow. The answer is **bounded concurrency**:
at most N in flight, each with its own deadline, and each failure isolated
from the others.

Tools:
- **`Semaphore`**: N permits; a task holds one while it runs (compare strand's
  `Semaphore`).
- **`JoinSet`**: owns a group of tasks and yields results as they finish.
- **`timeout`**: every remote call has a deadline.
- **Panic isolation**: a panicking task becomes a `JoinError`, not a crashed
  program.

## Production Solana application

Snapshot and ledger fetchers, RPC fan-out (`getMultipleAccounts` in chunks),
keeper bots scanning obligations, indexers backfilling slots, and health
checkers pinging validators all use this exact shape. Public RPCs rate-limit
aggressively, so the bound is how you stay a good citizen.

## Bug defended against

- Acquiring the permit *inside* the spawned task: all 10,000 tasks exist at
  once and merely wait. Memory still spikes. Acquire **before** spawning.
- No per-call timeout: one hung call holds a permit forever and throughput
  collapses to N-1, then N-2, …
- One panic aborting the whole batch.
- Results returned in completion order when the caller expects id order.

## Wrong code

```rust,ignore
let futs = ids.into_iter().map(|id| fetch(id));
futures::future::join_all(futs).await     // unbounded, no timeouts
```

## Coding approach

Follow the map exactly. The inner `tokio::spawn` is what turns a panic into
a `JoinError` while you still know which id it was.

## Testing

Peak concurrency ≤ 5 with 20 calls (4 waves), timeout/failure/panic
per id, and empty input.

## Explain before continuing

Where exactly is the permit released, and why does holding it inside the
wrapper task guarantee release even on panic or timeout?

## Required learner test

10,000 ids with `max_in_flight` 8: the peak number of live tasks never exceeds 8.
