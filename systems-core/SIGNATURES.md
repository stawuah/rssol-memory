# The Signature Blueprint

You don't need to memorize every API. You need a **map in your head**: for each
job, which function, what its signature promises, when it's the right tool, and
when it's a trap. A Rust signature is a contract. Learn to read it and the
docs become fast to use.

Each entry below has the signature, **Use when**, **Avoid when**, and where
relevant a **Senior note** (the detail that separates "works" from
"production-safe"). The exercise tags (ex06, …) point to where you practice it.

---

## 0. How to read a signature

| You see | It means | Why it matters for concurrency |
|---|---|---|
| `fn f(self)` | consumes the value; the caller can't use it again | "use once" APIs: `oneshot::Sender::send(self)`, `JoinHandle::join(self)` |
| `fn f(&self)` | shared access; many callers at once | a `&self` method that mutates must use interior mutability (a lock or an atomic) |
| `fn f(&mut self)` | exclusive access | exclusivity is proven at compile time, so no lock is needed (`Mutex::get_mut`) |
| `T: Send` | a `T` may be **moved** to another thread | `Rc`, `MutexGuard` and raw pointers are not `Send` |
| `T: Sync` | a `&T` may be **shared** between threads | `Cell`/`RefCell` are not `Sync`; `Mutex<T>` is `Sync` if `T: Send` |
| `'static` | borrows nothing that can die first (owned data is fine) | required for `thread::spawn` and `tokio::spawn`: the task may outlive the caller |
| `F: FnOnce()` | callable once, may consume captures | threads and tasks run their closure once |
| `F: Fn()` | callable many times through `&` | callbacks called repeatedly or from many tasks (`fetch_all`, ex24) |
| `impl Trait` (arg) | "any type implementing Trait", static dispatch | zero-cost; each call site compiles its own copy |
| `Box<dyn Trait>` | runtime dispatch, one concrete type erased | heterogeneous job queues: `Box<dyn FnOnce() + Send>` (ex02, strand `Pool`) |
| `-> Result<T, E>` | can fail; the caller must decide | services never `unwrap` channel/lock/join results on hot paths |
| `-> Option<T>` | may be absent | `recv()` in tokio returns `None` when closed |
| `-> LockResult<Guard>` | `Err` if a thread panicked while holding it (poisoned) | decide explicitly: `.expect("poisoned")` or recover with `into_inner()` |
| `Guard<'a, T>` | a borrow that ends when dropped | the lifetime ties the guard to its lock; it unlocks at scope end |
| `where` clauses | extra requirements | read them first; they tell you what the API needs from you |

---

## 1. Threads (ex01, ex02)

```rust
std::thread::spawn<F, T>(f: F) -> JoinHandle<T>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static
```
**Use when** a long-lived service thread owns its data (moved in).
**Avoid when** the work is short and borrows local data: use `scope`. Avoid it
for thousands of tiny tasks: use a pool or tokio.
**Senior note:** name it with `Builder::name`, keep the handle, and join it on
shutdown. A dropped handle hides panics.

```rust
std::thread::scope<'env, F, T>(f: F) -> T
    where F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T
Scope::spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    where F: FnOnce() -> T + Send + 'scope, T: Send + 'scope
```
**Use when** splitting work over borrowed data (chunks of a slice). Everything
is joined before `scope` returns.
**Avoid when** threads must outlive the function.

```rust
std::thread::Builder::new().name(String).stack_size(usize).spawn(f) -> io::Result<JoinHandle<T>>
JoinHandle::join(self) -> std::thread::Result<T>        // = Result<T, Box<dyn Any + Send>>
std::thread::available_parallelism() -> io::Result<NonZeroUsize>
```
**Senior note:** `join()`'s `Err` is the panic payload; downcast to `&str` or
`String` (ex02). Size worker pools from `available_parallelism`, not a
hard-coded 8.

---

## 2. Shared state (ex03–ex05)

### Arc / Weak
```rust
Arc::new(value: T) -> Arc<T>
Arc::clone(this: &Arc<T>) -> Arc<T>                     // pointer copy + atomic increment
Arc::get_mut(this: &mut Arc<T>) -> Option<&mut T>       // only if unique
Arc::make_mut(this: &mut Arc<T>) -> &mut T  where T: Clone   // clone-on-write
Arc::try_unwrap(this: Arc<T>) -> Result<T, Arc<T>>
Arc::downgrade(this: &Arc<T>) -> Weak<T>
Weak::upgrade(&self) -> Option<Arc<T>>
```
**Use when** many threads need the same data and none is "the owner".
**Avoid when** a borrow would do (scoped threads), or data is mutated often
(an `Arc<Mutex>` everywhere is a design smell: consider an actor).
**Senior note:** write `Arc::clone(&x)` rather than `x.clone()` so readers see
it's a cheap pointer copy. Use `Weak` to break cycles.

### Mutex
```rust
Mutex::new(value: T) -> Mutex<T>
Mutex::lock(&self) -> LockResult<MutexGuard<'_, T>>
Mutex::try_lock(&self) -> TryLockResult<MutexGuard<'_, T>>
Mutex::get_mut(&mut self) -> LockResult<&mut T>          // no locking: &mut proves exclusivity
Mutex::into_inner(self) -> LockResult<T>
```
**Use when** several threads **modify** the same data and each critical section is short.
**Avoid when** only one thread needs the data (move it or use an actor), it's a
single number (atomics), or it's read-mostly (a snapshot).
**Senior note:** never hold a guard across I/O, `sleep`, a blocking `send`, or
`.await`. Document the lock order when you take more than one (ex03).

### RwLock
```rust
RwLock::read(&self) -> LockResult<RwLockReadGuard<'_, T>>
RwLock::write(&self) -> LockResult<RwLockWriteGuard<'_, T>>
```
**Use when** reads vastly outnumber writes **and** reads are short.
**Avoid when** writes are frequent (a `Mutex` is faster), or readers hold the
guard for long. Use `RwLock<Arc<T>>` snapshots instead (ex05).

### Condvar
```rust
Condvar::wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> LockResult<MutexGuard<'a, T>>
Condvar::wait_while<'a, T, F>(&self, guard: MutexGuard<'a, T>, condition: F)
    -> LockResult<MutexGuard<'a, T>>  where F: FnMut(&mut T) -> bool
Condvar::wait_timeout<'a, T>(&self, guard: MutexGuard<'a, T>, dur: Duration)
    -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>
Condvar::notify_one(&self)
Condvar::notify_all(&self)
```
**Use when** building a blocking primitive (a queue, a semaphore, "wait until
count == 0").
**Avoid when** a channel already expresses it (almost always).
**Senior note:** always wait in a loop (or `wait_while`). Wake-ups can be
spurious or stolen.

### Atomics
```rust
AtomicU64::load(&self, order: Ordering) -> u64
AtomicU64::store(&self, val: u64, order: Ordering)
AtomicU64::fetch_add(&self, val: u64, order: Ordering) -> u64          // returns the OLD value
AtomicU64::swap(&self, val: u64, order: Ordering) -> u64
AtomicU64::fetch_max(&self, val: u64, order: Ordering) -> u64
AtomicU64::compare_exchange(&self, current: u64, new: u64, success: Ordering, failure: Ordering)
    -> Result<u64, u64>                                                 // Err(actual)
AtomicU64::compare_exchange_weak(...)                                   // may fail spuriously: use in loops
AtomicU64::fetch_update<F>(&self, set: Ordering, fetch: Ordering, f: F) -> Result<u64, u64>
    where F: FnMut(u64) -> Option<u64>
```
**Use when** you have a single counter, flag, max/min, or an ID generator.
**Avoid when** two values must change together: that needs a lock.
**Senior note on ordering:**
`Relaxed` is fine for statistics and counters nobody synchronizes on. Use
`Release` on the store that publishes data, and `Acquire` on the load that
consumes it (ex04). `SeqCst` is for when you can't articulate the pairing; then
revisit.

### One-time init
```rust
OnceLock::get_or_init<F>(&self, f: F) -> &T   where F: FnOnce() -> T
OnceLock::get(&self) -> Option<&T>
OnceLock::set(&self, value: T) -> Result<(), T>
static X: LazyLock<T> = LazyLock::new(|| ...);
```
**Use when** a global or config is computed once.
**Avoid when** the value changes over time (use snapshots).

### Barrier / thread_local
```rust
Barrier::new(n: usize) -> Barrier;  Barrier::wait(&self) -> BarrierWaitResult
thread_local! { static X: Cell<u32> = const { Cell::new(0) }; }   X.with(|x| ...)
```
**Use when** you need to start rounds together (Barrier), or want per-thread
scratch space with no sharing (thread_local).

---

## 3. Channels (ex06–ex10)

### std::sync::mpsc
```rust
mpsc::channel<T>() -> (Sender<T>, Receiver<T>)                    // unbounded
mpsc::sync_channel<T>(bound: usize) -> (SyncSender<T>, Receiver<T>)  // bounded; 0 = rendezvous
Sender::send(&self, t: T) -> Result<(), SendError<T>>             // Err gives the value back
SyncSender::send(&self, t: T) -> Result<(), SendError<T>>         // BLOCKS when full
SyncSender::try_send(&self, t: T) -> Result<(), TrySendError<T>>  // Full(t) | Disconnected(t)
Receiver::recv(&self) -> Result<T, RecvError>                     // Err = all senders gone, empty
Receiver::recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>
Receiver::try_recv(&self) -> Result<T, TryRecvError>
Receiver::iter(&self) -> Iter<'_, T>                              // ends when all senders drop
```
**Use when** threads (not async) pass work in one direction.
**Avoid when** you need multiple consumers or `select`: use crossbeam.
**Senior note:** in servers, **bounded** by default. Drop your own sender
before draining. Drain before joining (ex06). `recv_timeout` is how loops
notice shutdown (ex08).

### crossbeam-channel (MPMC + select)
```rust
crossbeam_channel::bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>)   // Receiver: Clone (multi-consumer)
crossbeam_channel::unbounded<T>() -> (Sender<T>, Receiver<T>)
crossbeam_channel::after(duration: Duration) -> Receiver<Instant>
crossbeam_channel::tick(duration: Duration) -> Receiver<Instant>
select! { recv(rx) -> msg => ..., recv(tick) -> _ => ..., default(timeout) => ... }
```
**Use when** you need worker pools sharing one queue, waiting on several
channels, or timers in thread-based code. Solana's validator uses crossbeam
channels heavily.

---

## 4. Tokio (ex23, ex24)

```rust
tokio::spawn<F>(future: F) -> JoinHandle<F::Output>
    where F: Future + Send + 'static, F::Output: Send + 'static
tokio::task::spawn_blocking<F, R>(f: F) -> JoinHandle<R>
    where F: FnOnce() -> R + Send + 'static, R: Send + 'static
JoinHandle<T>: Future<Output = Result<T, JoinError>>              // Err on panic or abort
JoinSet::spawn<F>(&mut self, task: F) -> AbortHandle
JoinSet::join_next(&mut self) -> Option<Result<T, JoinError>>     // async
```
**Use `spawn`** for I/O-bound tasks. **Use `spawn_blocking`** for blocking calls
(file I/O, a sync DB driver). **Use rayon** for CPU-heavy parallel work
(signature verification, hashing).
**Avoid** `std::thread::sleep` or heavy CPU inside async tasks. It freezes
every task on that worker thread.

```rust
tokio::sync::mpsc::channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>)
Sender::send(&self, value: T) -> Result<(), SendError<T>>          // async; waits when full
Sender::try_send(&self, message: T) -> Result<(), TrySendError<T>>
Receiver::recv(&mut self) -> Option<T>                             // async; &mut: single consumer
tokio::sync::oneshot::channel<T>() -> (Sender<T>, Receiver<T>)
oneshot::Sender::send(self, t: T) -> Result<(), T>                // self: once
oneshot::Receiver<T>: Future<Output = Result<T, RecvError>>
tokio::sync::broadcast::channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>)
broadcast::Receiver::recv(&mut self) -> Result<T, RecvError>       // RecvError::Lagged(n) | Closed
tokio::sync::watch::channel<T>(init: T) -> (Sender<T>, Receiver<T>)
watch::Receiver::changed(&mut self) -> Result<(), RecvError>;  borrow(&self) -> Ref<'_, T>
```
| Need | Channel |
|---|---|
| work queue, many producers → one consumer | `mpsc` (bounded) |
| one reply to one request | `oneshot` |
| every subscriber gets every event (lossy for slow ones) | `broadcast` |
| subscribers only need the **latest** value (config, current leader, slot) | `watch` |

```rust
tokio::sync::Semaphore::new(permits: usize)
Semaphore::acquire(&self) -> Result<SemaphorePermit<'_>, AcquireError>            // async
Semaphore::acquire_owned(self: Arc<Self>) -> Result<OwnedSemaphorePermit, AcquireError>
tokio::sync::Mutex::lock(&self) -> MutexGuard<'_, T>                              // async
tokio::sync::Notify::notified(&self) -> Notified<'_>;  notify_one(&self);  notify_waiters(&self)
tokio_util::sync::CancellationToken::{new, cancel(&self), cancelled(&self), child_token(&self)}
```
**Senior notes:**
- Acquire permits *before* spawning (ex24).
- Use `tokio::sync::Mutex` **only** when the guard must live across `.await`;
  otherwise a std `Mutex` is faster.
- Use `CancellationToken` trees for shutdown of task hierarchies.

```rust
tokio::time::timeout<F>(duration: Duration, future: F) -> Timeout<F>   // -> Result<F::Output, Elapsed>
tokio::time::sleep(duration: Duration) -> Sleep
tokio::time::interval(period: Duration) -> Interval;  Interval::tick(&mut self) -> Instant  // first tick is immediate
tokio::select! { biased; _ = &mut shutdown => ..., Some(m) = rx.recv() => ..., _ = tick.tick() => ... }
```
**Senior note:** every remote call gets a `timeout`. In `select!`, branches
that lose are **cancelled** (dropped), so only put cancel-safe futures in a
loop's `select!` (`mpsc::Receiver::recv` is cancel-safe; a half-read from a
socket may not be).

---

## 5. Data parallelism (rayon)

```rust
use rayon::prelude::*;
slice.par_iter().map(f).collect::<Vec<_>>()
slice.par_chunks(n).map(f).reduce(|| identity, combine)
rayon::join<A, B, RA, RB>(a: A, b: B) -> (RA, RB)
    where A: FnOnce() -> RA + Send, B: FnOnce() -> RB + Send, RA: Send, RB: Send
rayon::ThreadPoolBuilder::new().num_threads(n).thread_name(f).build() -> Result<ThreadPool, _>
```
**Use when** you do the same CPU work over many items (verify 10k signatures,
hash accounts).
**Avoid when** the work is I/O, or inside a tokio task without
`spawn_blocking`/a dedicated pool.

---

## 6. Design signatures (your own APIs)

These are the shapes senior engineers reach for. Each exercise uses one.

| Shape | Example | Why |
|---|---|---|
| **Handle + task** | `fn spawn_kv(..) -> KvService` / `fn spawn_bank(..) -> (BankHandle, JoinHandle<State>)` | clone-able client, one owner, the final state returned on shutdown (ex07, ex23) |
| **Pure core, thin shell** | `fn handle_command(state: &mut S, cmd: Command) -> bool` | business logic testable without threads (ex07) |
| **Injected clock** | `fn status(&self, node: NodeId, now: Instant) -> NodeStatus` | deterministic tests of timing logic (ex13) |
| **Injected effects** | `fn retry(policy, op, is_retryable, sleep: impl FnMut(Duration), rng: impl FnMut() -> u32)` | zero-time, reproducible tests (ex14) |
| **Stop parameter** | `fn run_service(rx, stop: &AtomicBool, tick: Duration, ..) -> LoopReport` | cooperative cancellation is visible in the signature (ex08) |
| **Snapshot getter** | `fn snapshot(&self) -> Arc<T>` (never a guard) | callers can't block writers (ex05) |
| **Validate, then commit** | `fn transfer(&self, ..) -> Result<(), LedgerError>` computing temps first | all-or-nothing updates (ex03, and protocol-core ex10) |
| **Canonical input** | `fn canonical_stakes(&[(NodeId, u64)]) -> Vec<(NodeId, u64)>` | cluster-wide determinism (ex19) |
| **Errors that tell the story** | `enum RetryError<E> { Permanent(E), Exhausted(E), OverBudget(E) }` | callers can react differently to each case |
| **Report structs** | `-> LoopReport`, `-> CommitStats`, `-> Replay { valid_len, torn_tail, .. }` | observability and testable outcomes |

---

## 7. Red flags in code review

| You see | Why it's a problem | Prefer |
|---|---|---|
| `pub fn data(&self) -> MutexGuard<'_, T>` | callers hold your lock for arbitrary time | a closure API or snapshots |
| `Arc<Mutex<...>>` on every field | shared-everything design; lock soup | ownership + messages (actor) |
| `.lock().unwrap()` / `.send().unwrap()` / `.join().unwrap()` in services | one failure cascades into a crash | map to typed errors; decide policy |
| `mpsc::channel()` / `unbounded_channel()` in a server | unbounded memory under load | bounded + backpressure or shedding |
| `recv()` with no timeout in a service loop | never notices shutdown | `recv_timeout` or `select!` on a stop signal |
| `Instant::now()` / `SystemTime::now()` deep in logic | untestable; clock-skew bugs | inject `now` |
| iterating a `HashMap` for consensus/scheduling output | order differs per process | `BTreeMap` or sort canonically |
| `f64` in consensus, fees, stake math | rounding differs; non-deterministic | integers, `u128` intermediates |
| `std::thread::sleep` or long CPU in `async fn` | stalls the runtime | `tokio::time::sleep`, `spawn_blocking`, rayon |
| `x.lock()` then calling a method that locks `y` | hidden lock order | document and enforce the order |
| a remote call without a deadline | one slow peer hangs you | `timeout` everywhere |
| retry loops without backoff/jitter/budget | retry storms | ex14's policy |
