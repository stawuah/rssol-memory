# Module 4 — Durability and Batching

Level: **Mid → Senior**. "Acknowledged" must mean "survives a crash", and it
must be fast. You'll frame a write-ahead log, recover from torn writes, batch
syncs with group commit, and bound replay-protection memory with expiry.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex09 | byte framing, CRC, panic-free parsing of untrusted bytes | WALs, blockstore/snapshot integrity |
| ex10 | deadline-bounded batching with `recv_timeout` | group commit, batched sigverify/DB writes |
| ex11 | two-index cache, `BTreeMap::split_off` expiry | blockhash expiry + status cache (replay protection) |

## Ordered exercises

1. [WAL framing](./exercises/ex09-wal-framing/README.md)
2. [Group commit](./exercises/ex10-group-commit/README.md)
3. [Replay protection](./exercises/ex11-replay-protection/README.md)

## Coverage checklist

- [x] length + checksum framing, truncate-on-recovery
- [x] crash at every byte offset
- [x] batching knobs: size vs latency
- [x] idempotency/replay defense with bounded memory

**Common interview question:** A client says "my transaction was confirmed
twice" and another says "my acknowledged write disappeared after a restart".
Which mechanisms prevent each, and where exactly do they live?
