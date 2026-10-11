# ex19 — Deterministic leader schedule

## Why this exists

Solana knows **in advance** who produces each slot: the leader schedule is
computed per epoch from the stake distribution and a seed. Every validator
must compute the **identical** schedule independently, with no communication.
That makes determinism the whole game:

- a canonical input order (never rely on `HashMap` iteration order);
- a specified PRNG (never `rand::thread_rng()`);
- integer math only.

Stake-weighted sampling gives each validator slots in proportion to its stake.
Each chosen leader holds 4 consecutive slots, so it can stream blocks without
a handoff on every slot.

## Production Solana application

Transactions are forwarded to upcoming leaders (hence "Gulf Stream"). MEV
infrastructure (Jito) routes bundles by schedule. RPC exposes
`getLeaderSchedule`. If two validators computed different schedules, they'd
reject each other's blocks.

## Bug defended against

- Iterating a `HashMap` (random order per process), which makes different
  validators compute different schedules.
- `u64` overflow when summing big stakes.
- Zero-stake nodes being selected.
- Modulo bias: `r % total` is slightly biased when total doesn't divide 2⁶⁴.
  It's negligible here; know that it exists.

## Wrong code

```rust,ignore
let total: u64 = stakes.values().sum();                 // HashMap order + overflow
let leader = stakes.keys().nth(rand::random::<usize>() % stakes.len()); // not weighted, not deterministic
```

## Coding approach

`canonical_stakes`, then `pick_weighted` (cumulative walk), then the loop.

## Testing

Order independence, pick boundaries, determinism across seed and order,
4-slot runs, no zero-stake leaders, frequency ≈ stake, and errors.

## Explain before continuing

Why must the seed be something no single validator controls? What happens to
the schedule if stake changes mid-epoch?

## Required learner test

Stakes near `u64::MAX` for several nodes don't overflow.
