# strand — the guide

This covers what each piece is for, when to reach for it, what it turns into, and the errors you'll meet from single programs up to distributed systems. The yardstick is *Rust Atomics and Locks* by Mara Bos (free at marabos.nl/atomics). Chapter numbers below refer to that book.

---

## 0. Getting started

```toml
# In your own project's Cargo.toml
[dependencies]
strand = { path = "../strand" }
```

```rust
use strand::*;
```

Run the examples in this order. Each one teaches one idea:

| Command | Teaches |
|---|---|
| `cargo run --example jobs` | channels, spawning, how a loop knows it's done |
| `cargo run --example jobs_raw` | the same program without macros (what they expand to) |
| `cargo run --release --example counter` | Mutex vs atomics, and when each wins |
| `cargo run --example worker_pool` | bounded queues, backpressure, many consumers |
| `cargo run --example routing_table` | read-mostly data with RwLock |
| `cargo run --release --example stream` | endless stream, bounded buffer, backpressure vs load shedding, `select!` |
| `cargo run --release --example pipeline` | stages + fan-out/fan-in (validator-shaped) |
| `cargo run --example actor` | one owner thread + oneshot replies |
| `cargo run --example pubsub` | broadcast, slow subscribers |
| `cargo run --release --example limits` | thread pool, panic isolation, semaphore |
| `cargo run --example std_basics` | scoped threads, OnceLock, thread_local, Barrier, Release/Acquire, CAS |
| `cargo run --example tiny_async` | a mini async executor: what tokio does underneath |
| `cargo run --release --example wal` | a write-ahead log: single writer, group commit, crash recovery |
| `cargo test` | stress tests + checks that mistakes fail to compile |

---

## 1. The syntax, one piece at a time

### `chan!` — a channel

```rust
chan!(jobs: String);              // unbounded
chan!(jobs: String, bounded 64);  // at most 64 waiting; senders pause when full
```

**What it is:** a queue. Many threads put values in, one thread takes them out (mpsc = multi-producer, single-consumer).

**Use it when** work flows in one direction:
- request handlers → one WAL/disk writer
- network reader thread → block/transaction processor
- workers → a results collector
- gossip receiver → state machine

**Expands to:** `let jobs = strand::Chan::<String>::new();`

**Bounded or unbounded?** In anything long-running (a server, a node, a validator), use **bounded**. If a producer is faster than the consumer, an unbounded queue grows until the process runs out of memory. A bounded queue makes the producer wait instead. That's called **backpressure**, and it's how a slow disk slows down incoming requests instead of crashing the node.

### `send!` — put a value in

```rust
send!(jobs, "hello from job {id}");  // string literal = format string -> String
send!(jobs, my_value);               // any value of the channel's type
```

It panics if the receiver is gone. If you want to handle that case, use `jobs.send(v)`, which returns a `Result`.

Gotcha: `send!(ch, 5)` fails to compile because the macro treats the literal as a format string. Write `send!(ch, (5))` or use a variable.

### `for msg in chan` — take values out

```rust
for msg in jobs { println!("{msg}"); }
```

The loop ends when the queue is empty **and** every Sender is gone. `for` first drops the Chan's own Sender, so you never need `drop(tx)`. Forgetting that `drop` is the #1 mpsc bug: the program hangs forever.

Other ways to receive (on a `Receiver`, from `chan.into_receiver()`):
- `rx.recv()` waits for one value. `Err` means closed.
- `rx.recv_timeout(d)` waits at most `d`.
- `rx.try_recv()` never waits.
- `select! { recv(a) -> x => .., recv(b) -> y => .., after(d) => .. }` waits on several at once.
- `rx.drain_ready()` takes everything queued right now. Use it for batching (see `wal.rs`).

### `shared!` + `lock!` — data many threads change

```rust
shared!(balance = 0u64);
lock!(balance => b { *b += 10 });           // lock, use, unlock at }
lock!(from, to => a, b { *a -= 5; *b += 5 }); // two locks, safe order
```

**Use it when** several threads must modify the same thing: a session map, an in-memory index, a peer table, a nonce counter with extra state.

**Expands to:** `Arc<Mutex<T>>`, and `lock!` becomes `{ let mut b = balance.lock(); ... }`. The guard is dropped at `}`, which unlocks.

**Two-lock form:** it always locks in the same global order (by memory address), whatever order you write them in. That makes the classic ABBA deadlock impossible (§4).

**Rules inside a `lock!` block:** keep it short. Never sleep, do network or disk I/O, `send!` on a bounded channel, or lock the same thing again.

### `read_mostly!` — data read constantly, written rarely

```rust
read_mostly!(routes = HashMap::new());
let r = routes.read();          // many readers at once
routes.write().insert(k, v);    // one writer, briefly
```

**Use it for** routing tables, cluster membership, stake tables, and config that's hot-reloaded. **Expands to** `Arc<RwLock<T>>`. If writes are frequent, plain `shared!` is faster.

### `spawn!` — start threads

```rust
let crew = spawn!(4, [jobs, counter], |id| { ... });   // 4 threads, id = 0..4
spawn!([jobs], { ... });                                // 1 thread
let results = crew.wait();                              // join, collect return values
```

Everything in `[ ... ]` gets handed to each thread through the `Share` trait:

| You capture | Each thread gets |
|---|---|
| `Chan<T>` | a `Sender<T>` (send only: receiving is a compile error) |
| `Shared<T>`, `ReadMostly<T>`, `Arc<T>` | a pointer clone (cheap, same data) |
| numbers, `bool`, `&'static str`, `Duration` | a copy |
| anything else (`Vec`, `String`, `HashMap`, ...) | **compile error** with advice on what to wrap it in |

**Expands to:**
```rust
for i in 0..4 {
    let jobs = jobs.sender();          // Share::share
    let counter = counter.clone();
    crew.start(i, move || { let id = i; ... });
}
```

**Keep it or not:**
- `let crew = spawn!(...)` runs in the background. The threads are joined when you call `crew.wait()` or when `crew` goes out of scope.
- `spawn!(...);` on its own waits right there for all threads (fork-join).

If a thread panics, the panic is re-raised when the crew is joined. With plain `std::thread::spawn`, ignoring the handle makes panics vanish silently.

---

## 2. Your Arc questions

### "Do I need to allocate a size?"

No. `Arc::new(value)` does exactly one heap allocation, and the compiler computes its size: two counters plus your `T`.

```rust
let a = Arc::new([0u8; 1024]);   // heap: 8 + 8 + 1024 bytes, one allocation
let b = a.clone();               // NO new allocation, just count += 1
```

Look at `Arc::new` in `src/arc.rs`: `Box::new(ArcData { .. })` then `Box::leak`. Box asks for `size_of::<ArcData<T>>()`. When the last pointer drops, `Box::from_raw` gives the memory back.

What you *do* choose is capacity for growable things **inside** it, e.g. `Arc::new(Vec::with_capacity(1000))`. That's the Vec's own buffer, not the Arc.

### "How does referencing / dereferencing work?"

`Arc<T>` implements `Deref<Target = T>`:

```rust
let cfg = Arc::new(Config { port: 8001 });
cfg.port          // auto-deref: (*cfg).port
cfg.validate()    // methods on Config work directly
let r: &Config = &cfg;   // deref coercion: &Arc<Config> -> &Config
```

What you **can't** do:

```rust
*cfg = other;          // ✗ can't assign: other threads may be reading
let c: Config = *cfg;  // ✗ can't move out: others still point at it
cfg.port = 9;          // ✗ same reason
```

To change shared data, put a lock inside: `Arc<Mutex<T>>` (that's `shared!`). If you're provably the only owner, `Arc::get_mut(&mut a)` gives `&mut T`.

The same Deref trick powers the guards: `MutexGuard` derefs to `T` and also implements `DerefMut`, which is why `*c += 1` works inside `lock!`.

### Weak: when you need it

Two `Arc`s pointing at each other (peer ↔ peer, parent ↔ child) never reach count 0, so they leak. Make one direction `Weak`:

```rust
let parent = Arc::new(Node { .. });
let back: Weak<Node> = Arc::downgrade(&parent);
if let Some(p) = back.upgrade() { /* still alive */ }
```

Typical uses are a child pointing back at its parent, a cache that shouldn't keep entries alive, and observers.

---

## 3. Which tool? (use cases)

| Situation | Reach for | Why |
|---|---|---|
| One thread produces, another consumes | `chan!` | no shared state, no lock ordering to get wrong |
| Many producers, one owner of a resource (file, socket, DB) | `chan!` + single owner thread | the owner needs no lock at all (`wal.rs`) |
| Many workers take jobs from one queue | `chan!` + `shared!(queue = rx)` | mpsc has one consumer; the mutex takes turns (`worker_pool.rs`) |
| Several threads modify a structure | `shared!` | the general tool |
| Read by everyone, updated occasionally | `read_mostly!` | readers don't block each other |
| A single counter or flag | `std::sync::atomic` in an `Arc` | no lock; ~4× faster here (`counter.rs`) |
| Immutable data every thread reads | `Arc::new(value)` | no lock needed at all |
| Need a reply to a request | a channel per request (oneshot) | `wal.rs` acks |
| Wait until some condition is true | `Mutex` + `Condvar` (in a `while` loop) | the building block under channels |

A good default for protocol and distributed-systems code: **give each resource one owner thread and talk to it with channels.** Reach for locks when that gets awkward, not first.

---

## 4. The errors: a catalogue

### A. Compile-time errors (the compiler protecting you)

| Error you'll see | What it means | Fix |
|---|---|---|
| `closure may outlive the current function, but it borrows x` | a thread might run after `x` is gone (it needs a `'static` lifetime) | `move` + a clone per thread (`spawn!` does this), or `std::thread::scope` |
| `use of moved value` / `value moved here, in previous iteration of loop` | the first thread took ownership, so the second has nothing | clone per thread, or wrap in `Arc`/`shared!` |
| `` `Rc<..>` cannot be sent between threads safely `` | `Rc`'s counter isn't atomic | use `Arc` |
| `` `Cell`/`RefCell` cannot be shared between threads safely `` | not thread-safe interior mutability | `Mutex`/`RwLock`/atomics |
| `cannot assign to data in an Arc` / `cannot move out of an Arc` | Arc is shared and read-only | put a `Mutex` inside, or `Arc::get_mut` |
| `` `MutexGuard` cannot be sent between threads safely `` | the lock must be released by the thread that took it | lock inside the thread |
| `future is not Send` (async) | a std `MutexGuard` is held across `.await` | drop the guard before `.await`, or use tokio's Mutex |
| `borrowed value does not live long enough` (with guards) | the guard's lifetime `'a` is tied to the mutex | keep the mutex alive longer than the guard |

strand adds: capturing a non-thread-safe value in `spawn!` produces a custom message naming the right wrapper. Receiving in a worker from a captured channel is a type error. Sharing one `Receiver` between two threads is a type error.

### B. Runtime bugs (compiles fine, still wrong)

| Bug | Symptom | How it happens | Prevention |
|---|---|---|---|
| **Forgotten sender** | program hangs at the end | `for msg in rx` waits for a Sender still alive in `main` | `for msg in chan` drops it for you |
| **Self-deadlock** | hangs | `lock()` twice on the same mutex in one thread (often via a helper function) | keep lock blocks small; never call out while locked |
| **ABBA deadlock** | hangs under load only | T1 locks A then B, T2 locks B then A | `lock!(a, b => ...)` fixes the order |
| **Lock held during I/O or sleep** | terrible throughput, timeouts | disk/network inside `lock!` | copy data out, unlock, then do I/O |
| **Bounded send while locked** | hangs | the consumer needs the same lock to make room | never `send!` inside `lock!` |
| **Unbounded queue growth** | memory climbs until the process is killed | producer faster than consumer | `bounded N` |
| **Lost panic** | a thread dies silently and work is missing | `JoinHandle` ignored | `Crew` re-raises panics |
| **Arc cycle** | memory leak | two Arcs point at each other | one direction `Weak` |
| **Check-then-act race** | rare wrong answers | `if map.contains(k)` then later `insert`, with the lock released in between | do both inside ONE `lock!` |
| **Condvar `if` instead of `while`** | rare crashes/wrong state | spurious or stolen wake-ups | always `while !cond { g = cv.wait(g) }` |
| **Writer starvation** | updates never apply | endless readers on an RwLock | this RwLock blocks new readers when a writer waits |
| **Contention** | adding threads makes it slower | everyone fights for one lock | shard the data (N locks by key hash), or use a channel to one owner |

Poisoning: std's Mutex marks itself "poisoned" if a thread panics while holding it. This one (like the book's) doesn't. After a panic the data might be half-updated, so treat a panicked worker as a failed operation.

### C. Distributed-systems failures (the same ideas, across machines)

Everything above has a network version, and the patterns in this crate are the local training ground for them:

| Local concept | Distributed version | What to do |
|---|---|---|
| Bounded channel | request queues, TCP windows, mempools | bound everything; shed load or return "busy" when full |
| Forgotten sender / hang | a peer that never replies | **timeouts on every remote call**, then retry or give up |
| Thread panic | node crash, partial failure | assume any step can fail halfway; design for recovery |
| Lock | leader lease, distributed lock | locks can expire while you still think you hold them; use fencing tokens / epochs |
| Check-then-act race | two nodes both think they're leader | consensus (Raft, Tower BFT on Solana) or compare-and-swap on shared storage |
| Message order within one channel | order across machines is NOT guaranteed | sequence numbers, logical clocks |
| `send` succeeded | the other side processed it? unknown | idempotent operations + request IDs, so retries are safe |
| WAL fsync | durability | ack only after fsync (`wal.rs` does this); checksum every record |
| Torn write on crash | half a record on disk | length + CRC framing, truncate on replay (`wal.rs`) |

---

## 5. Memory ordering on one page (ch. 3)

When you read `src/`, you'll see `Relaxed`, `Acquire`, and `Release`:

- **Relaxed:** "just make this one number correct". Used for counters where nothing else depends on the value (`Arc::clone`, the `counter.rs` atomic).
- **Release** (on unlock / on drop): "everything I wrote before this is published".
- **Acquire** (on lock / before freeing): "I see everything published by whoever did the Release".
- A Release–Acquire pair is a hand-off. It's why data written inside `lock!` in one thread is visible in the next thread that locks.

If you get these wrong, the code works on your laptop (x86 is forgiving) and breaks on ARM servers. To test, run under `cargo +nightly miri test` and try the `loom` crate, which explores thread interleavings.

---

## 6. Reading the source (suggested order)

1. `src/arc.rs`: counters, `Deref`, `Drop`, `Send`/`Sync` (ch. 1, 2, 6)
2. `src/mutex.rs`: `UnsafeCell`, guards, futex wait/wake, Condvar (ch. 4, 8, 9)
3. `src/rwlock.rs`: packing state into one integer, starvation (ch. 9)
4. `src/channel.rs`: building a channel from the three pieces above (ch. 5)
5. `src/lib.rs`: the syntax layer: the `Share` trait, `Crew`, and `macro_rules!`

6. `src/select.rs`, `src/oneshot.rs`, `src/broadcast.rs`, `src/semaphore.rs`, `src/pool.rs`, `src/sharded.rs`

Exercises that make it stick:
- Make the Mutex poison itself on panic (check `std::thread::panicking()` in the guard's Drop).
- Build a `watch` channel: subscribers only ever see the latest value (like `tokio::sync::watch`).
- Replace the oneshot with the book's ch. 5 version: one `AtomicBool` + `UnsafeCell<MaybeUninit<T>>`.
- Add `send_timeout` to the bounded channel (a timed Condvar wait).
- Rewrite `wal.rs` with a segment file per N MB and a checkpoint.
- Port `actor.rs` to tokio: `tokio::sync::mpsc` + `oneshot` + `tokio::spawn`.

## 7. Where this stops, and what's next

- **No async runtime.** `tiny_async` shows how one works; real network services use tokio. LESSONS.md has a strand → tokio translation table. The ideas map one-to-one; only "block the thread" becomes "yield the task".
- **`select!` wakes by thread park/unpark.** That's fine for learning. crossbeam's select is the hardened version.
- These primitives are for learning. They are correct and tested, but in production use `std::sync` or `crossbeam`/`parking_lot`, which have years of hardening.
