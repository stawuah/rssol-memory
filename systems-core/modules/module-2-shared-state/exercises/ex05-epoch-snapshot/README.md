# ex05 — Snapshots with `RwLock<Arc<T>>`

## Why this exists

Some data is read on every request and replaced occasionally: the epoch's
stake table, the leader schedule, a config, a routing table. Two naive designs
both hurt:

- `RwLock<T>` with readers holding the guard while they compute: a slow reader
  blocks the writer, and the writer blocks every new reader.
- `Mutex<T>` + clone the whole table per read: slow and memory-heavy.

The pattern: `RwLock<Arc<T>>`. A reader takes the lock for nanoseconds, clones
the **Arc** (not the data), and releases. The writer builds the new `T` outside
the lock and swaps the pointer. Old readers keep their old snapshot until
they drop it. (The `arc-swap` crate does this lock-free.)

## Production Solana application

The epoch stakes, the leader schedule cache, and bank snapshots handed to RPC
are all "immutable snapshot behind an Arc, swapped on change". Consensus
decisions must use ONE consistent epoch view, never half of epoch N and half
of epoch N+1.

## Bug defended against

- Torn reads: a reader sees the epoch number from N+1 and stakes from N.
- Writer starvation or long stalls caused by readers holding guards.
- Skipping or replaying an epoch.

## Wrong code

```rust,ignore
pub fn stakes(&self) -> RwLockReadGuard<'_, EpochInfo> { self.current.read().unwrap() }
// callers hold the guard across slow work -> the writer waits for all of them
```

## Coding approach

`snapshot` is one line. `rotate` validates, then `std::mem::replace`.

## Testing

Old snapshots survive rotation, non-sequential rotation is rejected, a held
snapshot doesn't block the writer, and 4 readers never see a torn epoch.

## Explain before continuing

Why does returning `Arc<EpochInfo>` instead of a guard change who can block
whom? What does an old snapshot cost in memory, and when is it freed?

## Required learner test

Rotate from `u64::MAX` and assert `NotNextEpoch` instead of an overflow.
