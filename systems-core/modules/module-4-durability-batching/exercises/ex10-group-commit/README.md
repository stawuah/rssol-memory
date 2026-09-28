# ex10 — Group commit

## Why this exists

`fsync` (making data survive power loss) costs roughly 0.1–10 ms, whatever the
size of the write. One fsync per record caps a disk at a few hundred to a few
thousand durable writes per second. **Group commit** writes everything that's
waiting and then fsyncs **once** for the whole group. Throughput goes up by
orders of magnitude, while every record still gets its durability guarantee.

The two knobs are a trade-off:
- `max_items`: a bigger batch means fewer syncs, but more memory and a longer tail.
- `max_wait`: a longer wait means bigger batches under light load, but adds latency to each write.

## Production Solana application

The same idea is batching across the stack: sigverify batches signatures for
the GPU/SIMD, banking stage takes batches of transactions, entries batch
transactions under one PoH hash, and RPC/indexers batch database writes.
Knowing when to batch, and how to bound latency while doing it, is a senior skill.

## Bug defended against

- A timer that starts when you *start waiting* instead of when the first item
  arrives: an idle writer then returns empty batches in a loop.
- Unbounded waiting for "one more item" under light load.
- Acknowledging writes before the sync.

## Wrong code

```rust,ignore
for item in rx { write(&item); fsync(); }          // one sync per item
```

## Coding approach

Block for the first item, set the deadline, then `recv_timeout(remaining)`
until full, deadline, or disconnect.

## Testing

A full batch is immediate, a partial batch comes after the deadline,
close/empty semantics hold, and 5,000 items need far fewer than 5,000 syncs,
in order.

## Explain before continuing

What latency does a single write see under light load? Under heavy load? How
would you choose `max_wait` for a validator vs. an analytics pipeline?

## Required learner test

A slow producer with a tiny `max_wait` produces batches of size 1.
