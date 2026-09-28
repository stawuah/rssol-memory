# ex23 — Actors in tokio

## Why this exists

Network services (RPC, relayers, indexers, block engines) handle thousands of
connections that mostly *wait*. Threads per connection are too heavy, so Rust
uses **async** with **tokio**. The shapes you learned with threads carry over
directly:

| threads (ex07) | tokio |
|---|---|
| `std::thread::spawn` | `tokio::spawn` |
| `mpsc::sync_channel(n)` | `tokio::sync::mpsc::channel(n)` |
| `sync_channel(1)` as a reply | `tokio::sync::oneshot` |
| `recv_timeout` | `tokio::time::timeout(d, fut)` |
| checking a stop flag | `tokio::select!` on a shutdown signal |

## Production Solana application

The Jito block engine and relayer, Geyser gRPC plugins (Yellowstone), RPC
servers, indexers and keeper bots are tokio actor systems. The validator's
QUIC TPU ingestion is tokio too. Senior interviews go beyond "can you use
tokio" to "can you design timeouts, backpressure and shutdown correctly".

## Bug defended against

- Awaiting a reply with no timeout: one stuck actor hangs every caller.
- An unbounded mailbox: memory grows under load.
- An actor that never exits (it leaks) because shutdown and "all handles
  dropped" weren't both handled.
- Holding `std::sync::MutexGuard` across `.await`. The actor owns its state,
  so it needs no mutex at all.

## Wrong code

```rust,ignore
self.tx.send(cmd).await.unwrap();
reply_rx.await.unwrap()          // no deadline, panics instead of reporting ActorGone
```

## Coding approach

`run_actor` with `select!` (use `biased;` so shutdown wins), `spawn_kv`,
`shutdown`, then `ask` with ONE timeout wrapping both send and receive.

## Testing

Get/set/previous value, 100 concurrent clients, a stalled actor → `Timeout`
then recovery, and exit when every handle is dropped.

## Explain before continuing

What happens to a command still in the mailbox when shutdown wins the
`select!`? When would you drain it first (compare ex08)?

## Required learner test

A full mailbox (capacity 1) plus stalls: `set()` returns `Timeout` and doesn't block.
