# ex11 — Replay protection with bounded memory

## Why this exists

A signed message can be copied and resent (**replayed**). "Remember every
signature forever" works, but memory grows forever. The trick is to make every
message **expire**: it must reference something recent. Anything that references
something old is rejected by age alone, so you only need to remember
signatures from the recent window.

**Idempotency** is the same idea from the client side: give each request a
unique ID so retrying it is harmless.

## Production Solana application

Every transaction carries a `recent_blockhash`. It's valid for about 150
blocks (~60–90 s). The bank's **status cache** remembers signatures only for
blockhashes still in that window. That's why a transaction can be retried
safely until it lands or expires, and why expired transactions must be
re-signed with a new blockhash. Durable nonces are the escape hatch for
long-lived transactions.

## Bug defended against

- Checking the signature set before the age: after a purge, an old transaction
  looks "new" and is accepted twice.
- Unbounded memory from never purging.
- Treating a future blockhash as age 0.

## Wrong code

```rust,ignore
if self.seen.contains(&sig) { return Err(AlreadyProcessed) }
self.seen.insert(sig);                  // no expiry: grows forever, and no age check at all
```

## Coding approach

Age first, then duplicates. Two indexes: `seen` for O(1) lookup and `by_slot`
(a `BTreeMap`) for cheap range removal with `split_off`.

## Testing

Duplicates, exact age boundaries, a replay after purge still rejected, and
memory bounded over 20,000 slots.

## Explain before continuing

Why is it safe to forget a signature? What breaks if two validators disagree
on `current_slot`?

## Required learner test

`purge` with `current_slot < max_age` must not underflow or drop valid entries.
