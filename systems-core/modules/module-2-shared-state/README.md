# Module 2 — Shared State

Level: **Junior → Mid**. When threads must share mutable data, pick the right
tool for the access pattern: per-item locks with a global order, atomics for
single values, and `RwLock<Arc<T>>` snapshots for read-mostly data.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex03 | `Mutex` per item, lock ordering, all-or-nothing updates | per-account locking without deadlock |
| ex04 | atomics, CAS loops, Release/Acquire publication | slot/root tracking, exit flags, one-time init |
| ex05 | `RwLock<Arc<T>>` snapshot swapping | epoch stakes, leader schedule caches, config reload |

## Ordered exercises

1. [Ledger with lock ordering](./exercises/ex03-ledger-lock-ordering/README.md)
2. [Atomics and publication](./exercises/ex04-atomics-and-publication/README.md)
3. [Epoch snapshot](./exercises/ex05-epoch-snapshot/README.md)

## Coverage checklist

- [x] ABBA deadlock and global lock order
- [x] consistent multi-lock snapshot
- [x] compare-exchange loop, 3-state publication flag
- [x] snapshot isolation without holding guards

**Common interview question:** You have a hot `Mutex<HashMap>` in a validator
service. Walk through how you'd diagnose the contention and your options
(sharding, RwLock, snapshot swap, actor), with trade-offs.
