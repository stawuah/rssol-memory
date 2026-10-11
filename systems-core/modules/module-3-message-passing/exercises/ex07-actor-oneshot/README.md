# ex07 — Actors: one owner, many askers

## Why this exists

Instead of sharing state behind locks, give it to **one thread** (the actor)
and let everyone else send it **messages**. Replies come back on a **one-shot**
channel created per request. No locks, no lock ordering, no torn updates: the
actor processes one command at a time, so every command is atomic by
construction.

## Production Solana application

This is the dominant shape of Rust infrastructure: RPC services, the Jito
block engine, Geyser plugins, indexers, and most tokio services are "a handle
(clone-able Sender) + a task that owns the state". Many validator services
follow it as well.

## Bug defended against

- Check-then-act races (read the balance, then withdraw) are impossible: the
  actor does both in one step.
- Hanging forever on a dead actor: every channel failure maps to `ActorGone`.
- An unbounded mailbox growing without limit: the mailbox is a `sync_channel`.

## Wrong code

```rust,ignore
if bank.balance(7)? >= 10 { bank.withdraw_unchecked(7, 10)?; } // two messages = race again
```

## Coding approach

1. Pure `handle_command` first; test it without threads.
2. `spawn_bank`: an owning loop.
3. `ask`: create the one-shot, send, wait, and map errors to `ActorGone`.

## Testing

Basic ops, 25 concurrent withdrawals of 10 from 100 (exactly 10 succeed),
`Stop` → final state returned → other handles see `ActorGone`.

## Explain before continuing

Why does the actor need no lock? What happens to throughput when one command
is slow, and how would you shard actors to fix it?

## Required learner test

An overflowing `Deposit` passed directly to `handle_command` leaves state unchanged.
