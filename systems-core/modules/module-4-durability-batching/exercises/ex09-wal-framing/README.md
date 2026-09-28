# ex09 — WAL framing and crash recovery

## Why this exists

A **write-ahead log** (WAL) records an intent durably *before* acting on it. If
the machine loses power, the log is replayed on restart. The hard part is the
**last record**: power can fail halfway through a write, leaving a torn record
on disk. Recovery must:

1. keep every complete, intact record,
2. detect the torn or corrupt tail (length + checksum), and
3. truncate the log to the last valid byte, then continue.

## Production Solana application

The blockstore, accounts DB, and validator snapshots all depend on this
discipline. RocksDB (used by the blockstore) has its own WAL. Every protocol
that says "I acknowledged it, so it survives a crash" is making a WAL promise.
On-chain, you are the WAL's client; off-chain infrastructure (indexers,
sequencers, bridges, relayers) is where you write one.

## Bug defended against

- Trusting a corrupt length field, which allocates 4 GiB or reads out of bounds.
- A checksum mismatch treated as "skip this record": everything after it may be garbage.
- Panicking on untrusted bytes during recovery, which makes the node crash-loop on restart.

## Wrong code

```rust,ignore
let len = u32::from_le_bytes(log[pos..pos+4].try_into().unwrap()) as usize;
records.push(log[pos+8..pos+8+len].to_vec());   // panics on a torn tail
```

## Coding approach

Check every bound before slicing. Stop at the first problem, and report
`valid_len` so the caller can truncate.

## Testing

A known CRC vector, round trip, a crash at **every** byte of the last record,
a flipped bit in the middle record, and an absurd length field.

## Explain before continuing

Why stop at the first bad record instead of skipping it? Where exactly must
`fsync` happen for an "acknowledged" record to survive power loss?

## Required learner test

Fuzz `replay` with 10,000 pseudo-random inputs: it must never panic.
