# ex14 — Retries, backoff, jitter, budgets

## Why this exists

Networks fail temporarily, so you retry. Naive retries turn a small outage into
a big one:

- **Immediate retries** hammer an overloaded service harder (a retry storm).
- **Synchronized retries** (every client waits exactly 1 s) arrive together in
  waves. **Jitter** (a random fraction of the delay) spreads them out.
- **Exponential backoff** (100 ms, 200, 400, …, capped) gives the service
  room to recover.
- **Budgets:** a caller with a 5 s deadline must not sleep for 30 s.
- **Only retry what's safe.** Timeouts are usually retryable. "Invalid
  signature" never is. And a retried *write* is only safe if it's idempotent
  (ex11).

## Production Solana application

Sending transactions: RPC `sendTransaction` with retries until the blockhash
expires; `maxRetries` and "rebroadcast yourself" strategies; priority-fee bumps
on retry. Clients to validators, keepers, liquidators, and bots all live or die
by their retry policy.

## Bug defended against

- Retry storms, synchronized waves, unbounded latency.
- Retrying permanent errors and wasting the whole budget.
- `base * 2^attempt` overflowing after about 60 attempts.

## Wrong code

```rust,ignore
loop { match send() { Ok(v) => return v, Err(_) => sleep(Duration::from_secs(1)) } }
```

## Coding approach

A pure `backoff_delay` (easy to test), then the loop. Inject `sleep` and
`rng`, so tests take 0 ms and are deterministic.

## Testing

Growth, cap, huge-attempt overflow, jitter scaling, transient → success,
permanent → one call, exhausted, over budget.

## Explain before continuing

Why "full jitter" rather than "base + small jitter"? When is retrying a
transaction submission safe on Solana, and when could it double-spend
off-chain state?

## Required learner test

With random `rng`, every sleep is ≤ cap and the total is ≤ budget.
