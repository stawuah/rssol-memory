# ex22 — Proof of History verification

## Why this exists

PoH is a hash chain: `h1 = hash(h0)`, `h2 = hash(h1)`, … Each step needs the
previous one, so **generating** N hashes takes N sequential steps: it proves
time passed. Data (transactions) can be mixed into a step, which proves the
data existed at that point in the sequence.

The trick: the leader publishes **entries** containing intermediate hashes. A
verifier knows every entry's starting point, so it can check **all entries in
parallel**, on many cores or a GPU. Generation is sequential and verification
is parallel. That asymmetry is the point.

## Production Solana application

Every validator replaying a block verifies its PoH entries in parallel (the
`entry` module, SIMD/GPU paths). PoH gives the cluster a shared clock
("slot ticks") without trusting wall clocks (compare ex12). Leaders must
produce the right number of ticks per slot. Too few means an invalid block.

## Bug defended against

- Verifying sequentially and wasting 15 of 16 cores during replay.
- Accepting `num_hashes == 0` entries (free "time").
- Checking hashes but not what was mixed in, so transactions can be swapped.

## Wrong code

```rust,ignore
// Parallel, but each thread recomputes from `start`: no speedup, and the wrong idea
entries.par_iter().all(|e| hash_from_start_up_to(e) == e.hash)
```

## Coding approach

`hash_n`, then `next_entry`, then `verify_entry`, then the two verifiers. For
parallel, build the `prevs` vector first; the rest is ex01's chunking again.

## Testing

Valid chains for many thread counts, tampering with data or hash count at
several positions, wrong start, and zero-hash entries.

## Explain before continuing

Why can't PoH *generation* be parallelized? What exactly does a PoH chain
prove, and what does it NOT prove (hint: it isn't consensus)?

## Required learner test

Swap two entries in a valid chain and assert verification fails. Explain why.
