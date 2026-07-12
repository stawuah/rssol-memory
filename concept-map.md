# Concept Map

## Phase 1: Rust foundations

- Week 01: variables, functions, control flow, simple state
- Week 02: ownership, borrowing, structs, enums, state transitions
- Week 03: collections, modules, error handling, idempotency
- Week 04: traits, generics, iterators, testing
- Week 05: lifetimes, memory layout, smart pointers
- Week 06: concurrency, channels, async basics
- Week 07: serialization, durability, recovery

## Phase 2: Solana foundations

- Week 08: accounts, instructions, transactions, PDAs, native Rust
- Week 09: Anchor, constraints, account design, authority
- Week 10: CPIs, tokens, security, negative testing

## Phase 3: Product and systems integration

- Week 11: clients, indexers, event consumers, eventual consistency
- Week 12: capstone, optimization, deployment thinking, interview packaging

## Cross-cutting distributed systems ladder

- state
- transitions
- messages
- retries
- deduplication
- shared ownership
- concurrency
- logs
- recovery
- deterministic execution
- authority
- eventual consistency
- scaling and failure modes

## Solana mapping

- state -> accounts
- transitions -> instructions
- authority -> signer checks + PDAs
- deterministic execution -> program logic
- message format -> instruction data
- durability -> chain state
- eventual consistency -> indexers and clients
