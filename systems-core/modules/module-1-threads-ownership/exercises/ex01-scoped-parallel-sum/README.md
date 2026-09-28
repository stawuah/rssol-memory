# ex01 — Scoped threads and balanced work splitting

## Why this exists

`std::thread::spawn` requires everything the thread touches to be `'static`,
because the thread might outlive the function that started it. That's why
beginners end up wrapping everything in `Arc` and cloning. `std::thread::scope`
guarantees every thread is joined before the scope returns, so threads can
**borrow** local data directly: no `Arc`, no `move` of owned data, no clones.

## Production Solana application

Validators split work across cores constantly: hashing account data for the
accounts hash, verifying signatures in a batch, checking PoH entries. The shape
is always the same: split into balanced chunks, process in parallel, combine,
with overflow-safe arithmetic when combining.

## Bug defended against

- Unbalanced chunks (`len / workers` with the remainder dumped on the last
  thread) make one thread the bottleneck.
- Summing each chunk with checked math but combining partials with `+`
  overflows silently in release builds.
- Spawning more threads than items creates empty work.

## Wrong code

```rust,ignore
let chunk = values.len() / workers;           // remainder handled badly, panics if 0
let handles: Vec<_> = (0..workers).map(|i| {
    let v = values.to_vec();                   // needless copy to satisfy 'static
    std::thread::spawn(move || v[i*chunk..(i+1)*chunk].iter().sum::<u64>())
}).collect();
let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum(); // unchecked
```

## Coding approach

1. Write `chunk_ranges` first; it has no threads and is easy to test.
2. Use `std::thread::scope(|s| { ... })` and `s.spawn`.
3. Each thread returns `Option<u64>` from a `try_fold` with `checked_add`.
4. Combine with `checked_add` again.

## Testing

Ranges for uneven splits, more workers than items, zero length, overflow
inside a chunk, and overflow only when combining.

## Explain before continuing

Why can a scoped thread borrow `values` but a `thread::spawn` thread can't?
What exactly does the scope promise the compiler?

## Required learner test

Add a case where the overflow happens only when partial sums are combined.
