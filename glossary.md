# Glossary

## Rust

`ownership`
: The rule that each value has one primary owner responsible for its lifetime.

`borrow`
: A temporary reference to a value without taking ownership of it.

`mutable borrow`
: Temporary exclusive access that allows modification.

`trait`
: A shared interface describing behavior.

`generic`
: Code written to work over more than one type.

`lifetime`
: A compiler-checked relationship about how long references stay valid.

`Arc`
: Atomically reference-counted shared ownership for concurrent code.

`Mutex`
: A lock that allows one mutable accessor at a time.

## Distributed systems

`state machine`
: A system that moves from one valid state to another in response to events.

`idempotency`
: Repeating the same operation does not change the final result after the first success.

`retry`
: Trying an operation again because the first attempt amy have failed or been lost.

`timeout`
: A rule for stopping waiting after some duration.

`replication`
: Keeping the same logical data on multiple nodes.

`durability`
: State survives crashes and restarts.

`eventual consistency`
: Replicas or read models may be temporarily different, but should converge later.

`backpressure`
: Work arrives faster than the system can process it.

## Solana

`account`
: The core data unit on Solana, holding lamports, owner, data, and metadata.

`instruction`
: One operation executed by a program against a set of accounts.

`transaction`
: A signed bundle of one or more instructions.

`program`
: Solana smart contract code, usually written in Rust.

`PDA`
: Program Derived Address, a deterministic address derived from seeds and a program id.

`CPI`
: Cross-Program Invocation, when one Solana program calls another.

`Anchor`
: A framework that reduces Solana boilerplate with macros and account constraints.

`indexer`
: An off-chain service that reads chain data and builds query-friendly views.
