# strand

Concurrency in Rust with a small syntax, built from scratch so you can see how it works.

```rust
use strand::*;

fn main() {
    chan!(jobs: String);

    let _crew = spawn!(4, [jobs], |id| {
        send!(jobs, "hello from job {id}");
    });

    for msg in jobs {
        println!("{msg}");
    }
}
```

No `move`, `clone`, `drop(tx)` or `join` by hand, and it's still plain Rust underneath. The macros expand to ordinary code (compare `examples/jobs.rs` with `examples/jobs_raw.rs`).

Every primitive is hand-written on atomics, following *Rust Atomics and Locks*:

- `src/arc.rs`: `Arc` and `Weak`
- `src/mutex.rs`: `Mutex` and `Condvar`
- `src/rwlock.rs`: `RwLock`
- `src/channel.rs`: mpsc channels, bounded or not, with timeouts, built on the three above
- `src/select.rs`: `select!`, `Stop` (graceful shutdown), `Ticker`
- `src/oneshot.rs`, `src/broadcast.rs`, `src/semaphore.rs`, `src/pool.rs`, `src/sharded.rs`

**Start here:**
1. [LESSONS.md](LESSONS.md): every concurrency idea you need, in learning order, explained simply, with when to use each and the real std/tokio equivalent.
2. [GUIDE.md](GUIDE.md): the syntax reference, the Arc questions, and the full error catalogue.

```sh
cargo test                            # stress tests + "this must not compile" tests
cargo run --example jobs
cargo run --release --example stream  # an endless stream through a buffer of 100
cargo run --release --example wal     # a write-ahead log with group commit + crash recovery

# all examples: jobs, jobs_raw, counter, worker_pool, routing_table, stream,
# pipeline, actor, pubsub, limits, std_basics, tiny_async, wal
```
