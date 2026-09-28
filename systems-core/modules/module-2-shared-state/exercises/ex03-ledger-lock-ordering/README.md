# ex03 — Per-account locks and lock ordering

## Why this exists

One big `Mutex<Vec<u64>>` is simple, but it makes every transfer wait for every
other transfer. One lock **per account** lets unrelated transfers run in
parallel. The price: a transfer now needs **two** locks, and two locks taken in
different orders by different threads is the classic **ABBA deadlock**:

```text
thread 1: lock(A) ... waiting for B
thread 2: lock(B) ... waiting for A      -> both wait forever
```

The fix is a **global lock order**: always lock the lower index first.

## Production Solana application

Sealevel runs transactions in parallel because each declares which accounts it
reads and writes, and the runtime takes per-account locks. Anything that locks
more than one resource (accounts DB, bank forks, a cache) needs a documented
lock order. Validators have shipped real deadlocks from getting this wrong.

## Bug defended against

- ABBA deadlock under load; it never shows up in a single-threaded test.
- Partial transfers: debit written, then the credit overflows.
- Self-deadlock on `from == to`.
- A total computed while transfers run: money appears to vanish mid-sum.

## Wrong code

```rust,ignore
let mut a = self.accounts[from].lock().unwrap();
let mut b = self.accounts[to].lock().unwrap();   // order depends on arguments
*a -= amount;                                    // written before checking b
*b = b.checked_add(amount).ok_or(Overflow)?;     // returns early, debit already applied
```

## Coding approach

Validate first, lock low then high, compute both new values, then write both.

## Testing

A 20 s watchdog around 8 threads doing opposite-direction transfers. It's
green only if nothing deadlocks and money is conserved.

## Explain before continuing

Why does a *consistent* order prevent deadlock? What would you do if you
needed to lock accounts whose set you only learn while holding the first lock?

## Required learner test

Run `total()` concurrently with transfers and assert every snapshot equals the
initial total.
