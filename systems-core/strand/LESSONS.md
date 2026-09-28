# Concurrency in Rust, explained like you're five

This is the whole map, from "what is a thread" to "how a validator pipeline works". Each idea has:

- 🧸 **the simple picture**
- **when you need it**
- **how to write it in strand**
- **what real Rust code uses** (std, crossbeam, tokio)
- ⚠️ **the trap**

Every idea marked ▶ has a runnable example: `cargo run --release --example <name>`.

---

## First: "bounded" does NOT mean "I know how many items are coming"

🧸 A restaurant has **8 chairs in the waiting area**. Customers can keep arriving **forever**. Only 8 can *wait* at a time. When the chairs are full, the next customer waits at the door until a chair frees up.

- `chan!(orders: Order, bounded 8)`: the waiting area has 8 chairs. The stream can be infinite.
- `chan!(orders: Order)` (unbounded): an infinite waiting area. If the kitchen is slower than the arrivals, the room fills the whole building (your RAM) and the process dies.

So for a never-ending stream:

1. The consumer is just a loop that never ends by itself (`for x in rx`, or a `select!` loop).
2. The buffer is bounded, so memory stays flat.
3. When the buffer is full you pick one:
   - **wait**, which is backpressure (`send`, so nothing is lost), or
   - **drop**, which is load shedding (`try_send`, where you count what you lost).
4. You stop it on purpose with a `Stop` signal, or by dropping every Sender.

▶ `stream`. 11 thousand items went through a buffer of 100 in strategy A, and 61 **million** were offered in strategy B, and the buffer never held more than 100.

---

## The map (everything, in learning order)

| # | Level | Ideas | In strand | Example |
|---|---|---|---|---|
| 1 | Threads | spawn, join, `move`, `'static`, scoped threads, `Send`/`Sync` | `spawn!`, `Crew` | `jobs`, `std_basics` |
| 2 | Sharing memory | `Arc`, `Weak`, `Mutex`, `RwLock`, atomics, memory ordering, CAS, `Condvar`, `OnceLock`, `thread_local!`, `Barrier` | `shared!`, `lock!`, `read_mostly!`, `Arc`… | `counter`, `routing_table`, `std_basics` |
| 3 | Message passing | mpsc, bounded vs unbounded, `try_send`, `recv_timeout`, oneshot, broadcast, `select!` | `chan!`, `send!`, `select!`, `oneshot()`, `Broadcast` | `jobs`, `actor`, `pubsub`, `stream` |
| 4 | Control | cancellation, graceful shutdown, timers, limits | `Stop`, `Ticker`, `Semaphore` | `stream`, `limits` |
| 5 | Patterns | worker pool, thread pool, pipeline, fan-out/fan-in, actor, single writer, sharding, batching, backpressure vs shedding, WAL | `Pool`, `Sharded` | `worker_pool`, `limits`, `pipeline`, `actor`, `wal` |
| 6 | Async | Future, poll, Waker, executor, tokio | (explained, not rebuilt) | `tiny_async` |
| 7 | Bugs | data race vs race condition, deadlock, livelock, starvation, lost wake-up, false sharing | compile errors + GUIDE §4 | `cargo test` |
| 8 | Testing | stress tests, loom, miri, ThreadSanitizer | — | — |
| 9 | Distributed | timeouts, retries, idempotency, clocks, consensus, gossip | — | next steps |

---

## Level 1 — Threads

### Thread
🧸 A thread is a second pair of hands. Your program starts with one; `spawn` hires more. They all work at the same time.
**When:** there's work that can happen in parallel (CPU-heavy work, or waiting on many things).
**strand:** `spawn!(4, [x], |id| { ... })`. **Real:** `std::thread::spawn`.
⚠️ Starting a thread costs memory (its own stack) and a few microseconds. Don't start one per tiny task; use a `Pool`.

### Join
🧸 "Wait at the door until my helper comes back."
**strand:** `crew.wait()`, or automatically when the `Crew` is dropped. **Real:** `handle.join()`.
⚠️ If you never join, a panic inside the thread disappears silently and `main` may exit while the thread is mid-work.

### `move` and `'static`
🧸 The helper might still be working after you've gone home. So they can't just *point at* your stuff on your desk; they have to *take* it (or their own copy) with them.
**strand:** `[x]` in `spawn!` gives each thread its own handle. **Real:** `let x = x.clone(); thread::spawn(move || ...)`.
⚠️ Error: `closure may outlive the current function`.

### Scoped threads
🧸 "Everybody must come back before I leave the room." Then helpers CAN just point at your stuff, because you'll still be there.
**When:** short parallel work on data you already have (split an array into chunks).
**Real:** `std::thread::scope(|s| { s.spawn(|| ...) })`, which needs no `Arc` and no `move`. ▶ `std_basics`

### `Send` and `Sync`
🧸 Labels on toys. **Send** = "it's okay to *give* this toy to another kid". **Sync** = "it's okay for several kids to *look at it together*".
- `Rc` is not Send: its counter isn't atomic.
- `Cell`/`RefCell` are not Sync: they allow unsynchronized mutation.
- `MutexGuard` is not Send: unlock where you locked.
- Our `Receiver` is not Sync: only one consumer.

The compiler checks these labels, and that's why data races are **compile errors** in Rust.

---

## Level 2 — Sharing memory

### `Arc` (Atomically Reference Counted)
🧸 One toy and a counter saying how many kids are holding it. The last kid to let go puts it away.
**When:** many threads need the same *read-only* thing (config, a lookup table, a handle).
**strand:** `Arc::new(v)` (built in `src/arc.rs`). **Real:** `std::sync::Arc`.
- No size to allocate. One heap allocation, sized by the compiler.
- `clone()` = +1 on the counter, not a copy of the data.
- Deref lets you use it like the value itself (`arc.field`).

⚠️ You can't change what's inside. For that you need a lock inside: `Arc<Mutex<T>>`.

### `Weak`
🧸 A note that says where the toy is, but doesn't count as holding it. When you want to play, ask "is it still there?" (`upgrade()`).
**When:** back-pointers (child → parent), caches, breaking cycles.
⚠️ Two `Arc`s pointing at each other never get freed (a leak).

### `Mutex`
🧸 A bathroom with one key. Take the key, use it, hang the key back. Everyone else waits.
**When:** several threads **change** the same data.
**strand:** `shared!(x = v)` then `lock!(x => g { *g += 1 })`. **Real:** `std::sync::Mutex`, `parking_lot::Mutex`.
⚠️ Traps:
- Locking twice in the same thread hangs it.
- Locking A→B in one thread and B→A in another deadlocks. `lock!(a, b => ..)` prevents this.
- Doing slow things (I/O, sleep, `send` on a bounded channel) while holding the lock.

### `RwLock`
🧸 A museum painting: any number of people can **look** at once, but to **repaint** it you clear the room.
**When:** read constantly, write rarely (routing tables, stake tables, config).
**strand:** `read_mostly!(x = v)`, `x.read()`, `x.write()`. ▶ `routing_table`
⚠️ If writes are common, it's slower than a Mutex.

### Atomics
🧸 A scoreboard number that everyone can bump **in one motion**, so two bumps can't collide. No key needed.
**When:** a single number or flag: counters, IDs, "is shutting down", "highest slot seen".
**Real:** `AtomicU64`, `AtomicBool`, and friends: `fetch_add`, `swap`, `compare_exchange`. ▶ `counter` (about 4× faster than a Mutex for one number)
⚠️ Only works for one value at a time. Two values that must change together need a lock.

### Memory ordering (`Relaxed` / `Release` / `Acquire` / `SeqCst`)
🧸 You write a letter, then raise the mailbox flag. **Release** means "everything I did before raising the flag is done". **Acquire** means "when I see the flag up, I'll also see the letter". **Relaxed** means "I only care about this one flag, not the letter".
**When:** writing your own lock-free code. Every `lock()` is an Acquire and every unlock is a Release, which is why data changed inside a lock is visible to the next locker. ▶ `std_basics` #5
⚠️ Wrong orderings pass tests on x86 laptops and break on ARM servers.

### Compare-and-swap (CAS)
🧸 "Change the number to 42, **but only if it's still 17**. If someone changed it, tell me the new value and I'll rethink."
**When:** lock-free updates like max, increment-if-below-limit, or `Weak::upgrade` (see `src/arc.rs`). ▶ `std_basics` #6

### `Condvar`
🧸 "Wake me up when the cookies are ready." You sleep *next to* the Mutex and someone taps you after changing the data.
**When:** building blocking queues, semaphores, "wait until count == 0". In strand, the `Semaphore` and the bounded channel's "wait for space" are built on it.
⚠️ Always check in a `while` loop, never an `if`. Wake-ups can be spurious, or someone else may take the cookie first.

### `OnceLock` / `LazyLock`
🧸 "Whoever gets here first bakes the cake; everyone else eats that same cake."
**When:** global things initialized once (genesis config, a precomputed table). ▶ `std_basics` #2

### `thread_local!`
🧸 Every kid gets their own crayons. No sharing means no fighting.
**When:** per-thread scratch buffers, per-thread random number generators, per-thread stats you add up later.

### `Barrier`
🧸 "Nobody starts the race until all runners are at the line."
**When:** benchmarks, simulations that go in rounds, tests. ▶ `std_basics` #4

---

## Level 3 — Message passing

### mpsc channel
🧸 A conveyor belt. Many put boxes on (Senders), one takes them off (Receiver).
**When:** work flows one way. It's often simpler and safer than sharing plus locks.
**strand:** `chan!(x: T)`, `send!(x, v)`, `for v in x`. **Real:** `std::sync::mpsc`, `crossbeam_channel`, `tokio::sync::mpsc`.
⚠️ The receive loop ends only when every Sender is dropped. A forgotten Sender means a hang. (`for v in chan` handles it.)

### Bounded / `try_send` / `recv_timeout`
Covered at the top. `try_send` = "don't wait, tell me if it's full". `recv_timeout(d)` = "wait at most d". Both are essential for anything network-facing. ▶ `stream`

### Oneshot
🧸 A self-addressed envelope for **one** reply.
**When:** request → response: "write this, tell me the log position", "what's the balance?".
**strand:** `let (reply, answer) = oneshot();`. **Real:** `tokio::sync::oneshot`.
`send(self)` takes the envelope by value, so you *can't* reply twice; the compiler enforces it. ▶ `actor`, `wal`

### Broadcast
🧸 The school loudspeaker. One announcement, every classroom hears it. The announcer doesn't wait for a class that isn't listening.
**When:** new slot/block, config changed, peer joined, shutdown notice.
**strand:** `Broadcast::new(n)`, `.subscribe()`, `.publish(v)`. **Real:** `tokio::sync::broadcast`. A close cousin is `tokio::sync::watch`: subscribers only see the *latest* value, which suits "current leader" or "current config". ▶ `pubsub`
⚠️ Slow subscribers: either everyone waits for them (bad) or they miss messages (we count these as "lagged").

### `select!`
🧸 Waiting for your friend, the pizza, OR the alarm, whichever comes first.
**When:** every long-running loop: messages + timer + shutdown. Also request deadlines and heartbeats.
**strand:**
```rust
select! {
    recv(inbox) -> msg => { /* msg: Option<T>, None = closed */ },
    recv(heartbeat) -> _ => { /* every N ms */ },
    recv(stop) -> _ => break,
    after(Duration::from_secs(5)) => { /* nothing for 5 s: peer dead? */ },
}
```
**Real:** `crossbeam_channel::select!`, `tokio::select!`.
⚠️ Fairness: if one arm is always busy, it can starve the others. strand rotates which arm it checks first.

---

## Level 4 — Control

### `Stop` (cancellation / graceful shutdown)
🧸 The bell at the end of break. Everyone hears it, finishes what's in their hands, and goes inside.
**Why not just kill a thread?** It might be halfway through writing a file or holding a lock. So threads **check** for the signal at safe points.
**strand:** `let stop = Stop::new(); stop.fire(); stop.is_fired(); stop.sleep(d)`, and `recv(stop)` in `select!`. **Real:** `tokio_util::sync::CancellationToken`, or an `Arc<AtomicBool>`.

Graceful shutdown order in a real service:
1. Stop accepting new work.
2. Close the input channels (drop the Senders).
3. Let the workers drain what's queued.
4. Flush and fsync.
5. Join the threads.

`Pool`'s `Drop` does exactly this.

### `Ticker`
🧸 A clock that dings every minute. If you were busy and missed three dings, you get one ding, not a burst of three.
**When:** heartbeats, "flush every 10 ms", metrics, retry timers, leader-election timeouts.
**strand:** `Ticker::every(d)` in `select!`. **Real:** `crossbeam_channel::tick`, `tokio::time::interval`.

### `Semaphore`
🧸 A bouncy castle that fits 5 kids, with a wristband to get in. No wristband left means wait (or go home).
**When:** cap concurrency: 64 peer connections, 8 disk reads in flight, a DB connection pool.
**strand:** `sem.acquire()` / `sem.try_acquire()` gives you a `Permit`, which is returned automatically when dropped. **Real:** `tokio::sync::Semaphore`. ▶ `limits`

---

## Level 5 — Patterns (how real systems are shaped)

| Pattern | 🧸 Picture | When | Example |
|---|---|---|---|
| **Worker pool** | 4 cooks, 1 ticket rail | many independent jobs | `worker_pool` |
| **Thread pool** | same cooks reused all day | CPU work arriving constantly (sig verify, hashing) | `limits` (`Pool`) |
| **Pipeline** | car factory: each station does one step, belt between them | multi-step processing (fetch → verify → execute) | `pipeline` |
| **Fan-out / fan-in** | one station has 4 workers because it's slow | one stage is the bottleneck | `pipeline` (4 verifiers) |
| **Actor** | only the teller touches the money drawer | state that many need to change: give it ONE owner + a mailbox | `actor` |
| **Single writer** | one person writes in the logbook | files, WAL, anything with ordering | `wal` |
| **Sharding** | 16 checkout lanes by first letter | one lock is the bottleneck | `Sharded` (test in `lib.rs`) |
| **Batching / group commit** | the bus leaves with everyone at the stop, not one per person | expensive per-operation cost (fsync, network round trip) | `wal`, `stream` |
| **Backpressure** | "kitchen full, please wait" | data you must not lose | `stream` A |
| **Load shedding** | "kitchen full, come back later" | data you can lose (metrics, gossip) or overload protection | `stream` B |
| **Write-ahead log** | write it in the diary before doing it | durability, crash recovery | `wal` |

**Rule of thumb for protocol code:** give each piece of state **one owner thread** and talk to it with channels (actor/pipeline). Add locks only where that gets awkward. Solana's validator follows the pipeline shape (fetch → sigverify → banking → broadcast), and most tokio services follow the actor shape.

For **data parallelism** (the same operation on a million items), don't hand-roll threads. Use **rayon**: `items.par_iter().map(verify).collect()`.

---

## Level 6 — Async (what tokio is)

🧸 Threads are like hiring one worker per customer. Async is **one worker with a notepad**: "A is waiting for the oven, so I'll help B and C, and come back to A when the oven dings."

- A `Future` is a paused task. `poll()` asks "can you move forward?" and gets back `Ready(value)` or `Pending`.
- When it returns `Pending`, the future keeps a **Waker** and calls `wake()` when it can move forward (the oven dinged).
- An **executor** polls futures again after they're woken. strand's `tiny_async` executor uses `thread.unpark()` as the waker, the same trick our channels use.
- `.await` = "poll this; if Pending, pause me too".

▶ `tiny_async`: two 300 ms "network calls" finish in 300 ms on **one** thread.

**When to use async:** thousands of connections that are mostly *waiting* (network servers, RPC, gossip). **When to use threads:** CPU-heavy work (hashing, signature verification). Real systems use both: tokio for I/O, plus a thread pool (`spawn_blocking` or rayon) for CPU work.

**strand → tokio translation table:**

| strand | tokio |
|---|---|
| `spawn!` / `Crew` | `tokio::spawn` / `JoinSet` |
| `chan!(x: T, bounded n)` | `tokio::sync::mpsc::channel(n)` |
| `oneshot()` | `tokio::sync::oneshot::channel()` |
| `Broadcast` | `tokio::sync::broadcast::channel(n)` |
| `shared!` / `lock!` | `std::sync::Mutex` (short locks) or `tokio::sync::Mutex` (lock held across `.await`) |
| `Semaphore` | `tokio::sync::Semaphore` |
| `select!` | `tokio::select!` |
| `Ticker` | `tokio::time::interval` |
| `after(d)` / `recv_timeout` | `tokio::time::timeout(d, fut)` |
| `Stop` | `tokio_util::sync::CancellationToken` |
| `Pool` | `tokio::task::spawn_blocking`, or rayon |

⚠️ Async traps:
- Blocking inside async (`thread::sleep`, heavy CPU work) freezes every task on that thread.
- Holding a std `MutexGuard` across `.await` gives the error "future is not Send".

---

## Level 7 — Bugs, named

| Name | 🧸 Picture |
|---|---|
| **Data race** | two kids writing on the same spot of paper at the same instant. Rust makes this a *compile error* (Send/Sync). |
| **Race condition** | "I checked the fridge had milk, then went to get cereal, and someone drank it." Logic that's still wrong even though every step is safe. Fix: check and act inside ONE lock (or one actor). Rust can't catch this for you. |
| **Deadlock** | two kids each holding one crayon and waiting for the other's. Forever. |
| **Livelock** | two people in a corridor both stepping aside the same way, again and again. Busy, but no progress. |
| **Starvation** | the quiet kid never gets a turn. (Writer starvation in RwLock, unfair select.) |
| **Lost wake-up** | you said "wake me" *after* the alarm already rang. strand registers before checking to prevent it. |
| **False sharing** | two kids at the same small table bump elbows even though they're drawing different pictures. Two atomics on the same 64-byte cache line slow each other down. Fix: pad them apart. |

Full catalogue with fixes: `GUIDE.md` §4.

---

## Level 8 — Testing concurrent code

- **Stress tests:** run it with many threads, many times (`cargo test` here does 800k lock operations).
- **miri** (`cargo +nightly miri test`): catches undefined behavior in `unsafe` code.
- **loom:** runs your code under *every possible* thread interleaving (for small tests). This is how tokio tests its primitives.
- **ThreadSanitizer** (`RUSTFLAGS=-Zsanitizer=thread`): detects data races at runtime.
- **Deterministic simulation:** run a whole distributed system in one thread with a fake clock and fake network, so a failing run can be replayed exactly (TigerBeetle, FoundationDB and Sui do this).

---

## Level 9 — What's next for distributed systems

The ideas above, stretched across machines. Learn these next, in roughly this order:

1. **Timeouts on everything.** A peer can be slow, dead, or partitioned, and you can't tell which.
2. **Retries + idempotency.** Retry safely by giving each request an ID so doing it twice is harmless.
3. **Framing and serialization.** Length-prefixed messages (like the WAL), bincode/borsh.
4. **Durability.** WAL, fsync, checksums, snapshots/checkpoints. ▶ `wal`
5. **Time and ordering.** Wall clocks lie. Use sequence numbers, Lamport and vector clocks, slots/epochs.
6. **Failure detection.** Heartbeats plus timeouts (`Ticker` + `after`).
7. **Gossip.** How information spreads (Solana's gossip, turbine for blocks).
8. **Consensus.** Raft first (the easiest to learn), then PBFT/Tower BFT/Alpenglow for Solana.
9. **async networking with tokio.** TCP/QUIC servers built from actors + channels + select.

Books:
- *Rust Atomics and Locks* (levels 1–4, and how this crate is built)
- *Designing Data-Intensive Applications* (level 9)
- The Raft paper, "In Search of an Understandable Consensus Algorithm"
