# ex02 — Owned threads, `'static`, `Send`, and panics

## Why this exists

`thread::spawn` takes `F: FnOnce() -> T + Send + 'static`. Read it as:
- **`FnOnce`**: the thread runs it once and may consume what it captured.
- **`Send`**: everything it carries can move to another thread (`Rc` can't).
- **`'static`**: it borrows nothing that could die before the thread does.

`join()` returns `Result<T, Box<dyn Any + Send>>`. The `Err` side is the
panic payload. If you drop a `JoinHandle` without joining, a panic in that
thread vanishes silently: the work is lost and nobody knows.

## Production Solana application

Validator services (banking stage, replay, gossip, RPC) each run on
long-lived named threads. A service thread that dies silently is a node that
looks healthy but has stopped voting or stopped processing transactions.
Named threads (`solBanknStgTx`, `solRepairSvc`, ...) are how operators find
which service died.

## Bug defended against

- Joining inside the spawn loop, which turns parallel work into sequential work.
- `handle.join().unwrap()`, which re-panics the caller and loses the other results.
- Ignoring handles, which loses panics.

## Wrong code

```rust,ignore
for job in jobs {
    let r = std::thread::spawn(job).join().unwrap(); // sequential, and crashes on panic
    results.push(r);
}
```

## Coding approach

1. Spawn everything, keeping the handles in order.
2. Join in order and map each panic to `JobError::Panicked`.
3. Downcast the payload: `&str` first, then `String`.

## Testing

Order is preserved, both kinds of panic message are captured, and the jobs
provably overlap in time.

## Explain before continuing

Why does `thread::spawn` need `'static` but `thread::scope` doesn't? What's
in a panic payload, and why is it `dyn Any`?

## Required learner test

Panic with `std::panic::panic_any(7u32)` and assert you get "unknown panic".
