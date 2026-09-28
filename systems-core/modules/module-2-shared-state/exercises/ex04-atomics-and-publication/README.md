# ex04 — Atomics: compare-and-swap and safe publication

## Why this exists

An atomic is a number that many threads can update without a lock, one
indivisible step at a time. Two skills matter:

1. **CAS loops** (`compare_exchange`): "set it to X only if it's still Y; if
   not, tell me what it is now and I'll retry". This is how every lock-free
   max, counter-with-limit, and `Arc::upgrade` works.
2. **Memory ordering.** `Release` on a store plus `Acquire` on the matching load
   means "whoever sees my flag also sees everything I wrote before it".
   `Relaxed` only guarantees that single value is correct.

## Production Solana application

- Tracking the highest root/slot seen, vote counters, and metrics: all atomics.
- "Exit" flags that every validator service checks: `Arc<AtomicBool>`.
- Publishing a computed value once (OnceLock / lazy statics).

## Bug defended against

- `if slot > highest.load() { highest.store(slot) }` is a race: two threads
  both pass the check and the smaller value can win.
- Publishing with `Relaxed` on the flag: a reader on ARM can see "ready" but
  the old value. It passes every test on an x86 laptop.

## Wrong code

```rust,ignore
if slot > self.highest.load(Relaxed) {        // check...
    self.highest.store(slot, Relaxed);        // ...then act: not atomic together
}
self.value.store(v, Relaxed);
self.ready.store(true, Relaxed);              // no Release: value may not be visible
```

## Coding approach

Follow the coding maps. For CAS, use `compare_exchange_weak` in a loop (it may
fail spuriously, which the loop handles). For `publish`, use a 3-state flag so
two publishers can't both write.

## Testing

8 threads × 10k observations keep the true max. Exactly one caller "raises"
a repeated slot. Exactly one publisher wins and readers agree with it.

## Explain before continuing

Why is `compare_exchange` needed instead of load-then-store? Draw the Release
→ Acquire "happens-before" arrow for `publish`/`get`.

## Required learner test

Spin-read `get()` from 4 threads while one thread publishes a non-zero value;
assert no reader ever sees `Some(0)`.
