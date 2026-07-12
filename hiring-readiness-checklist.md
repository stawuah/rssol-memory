# Hiring Readiness Checklist

Use this before applying.

This is not a motivation sheet. It is a reality check.

## General rule

You are ready to apply when:

- your best projects are finished enough to demo
- you can explain what you built without bluffing
- you can answer technical questions with structure
- your GitHub and resume show real engineering, not just exposure

## Core evidence checklist

You should have:

- 2 to 4 serious projects
- tests in your important repos
- READMEs with setup, usage, and architecture
- one capstone you know deeply
- one project where you can explain a bug you hit and fixed
- one project where you can explain a design tradeoff

## Role-by-role readiness

### Junior Rust engineer

You should be able to:

- read and write small to medium Rust programs confidently
- explain ownership, borrowing, `Option`, `Result`, enums, traits, and iterators
- write tests without being told exactly what to test
- debug compiler errors without freezing
- explain at least one small systems-oriented project

Good evidence:

- `node-heartbeat-cli`
- `raft-lite-state-machine`
- `replicated-log-sim`
- `distributed-kv-core`

You are probably ready if:

- you can solve most week 1 through week 6 exercises independently
- you can implement and modify the mini-projects without copying
- you can explain why your code is safe, not just that it compiles

### Early-to-mid Solana engineer

You should be able to:

- explain accounts, instructions, transactions, signers, owners, and PDAs clearly
- write small Solana programs in Rust
- use Anchor productively without treating it like magic
- explain authority rules and negative test cases
- discuss at least one security mistake you intentionally guarded against

Good evidence:

- `solana-native-hello-state`
- `anchor-task-board`
- `vault-program`

You are probably ready if:

- you understand the difference between program code and account state
- you can explain when to use native Rust vs Anchor
- you can design account schemas and authority constraints
- you can write failure tests for unauthorized or invalid operations

### Protocol engineer: junior

You should be able to:

- model state transitions explicitly
- reason about invariants
- discuss Solana account design and protocol flows
- explain off-chain components like indexers and clients at a high level
- write tests for happy path and failure path behavior

Good evidence:

- `anchor-task-board`
- `vault-program`
- `protocol-stack`

You are probably ready if:

- your capstone has clear authority, state, and failure rules
- you can explain why a protocol invariant matters
- you can describe a full end-to-end protocol flow

### Protocol engineer: mid

You should be able to:

- design cleaner account schemas with explicit tradeoffs
- explain protocol state machines and failure cases precisely
- reason about CPIs, token flows, authority boundaries, and stale state
- discuss indexers, read models, and eventual consistency
- identify common attack surfaces and missing validations

You are probably ready if:

- your capstone feels like a product or protocol, not a tutorial clone
- your tests cover unauthorized actions, stale writes, and invalid inputs
- you can explain tradeoffs in account size, UX, and safety

### Protocol engineer: senior

You should **not** assume this curriculum alone makes you senior.

Senior protocol roles usually require:

- real production experience
- operating systems through failures
- reviewing other engineers’ designs
- security maturity under ambiguity
- shipping and maintaining protocol upgrades

What this curriculum can do:

- make you strong enough to talk to those teams
- make you competitive for some mid roles if your project depth is exceptional
- give you the raw material to ramp into seniority faster later

### Backend / systems engineer with a strong self-taught project story

You should be able to:

- explain concurrency and state machines
- reason about retries, timeouts, idempotency, and logs
- design and test a small service
- discuss persistence and recovery at a practical level
- explain tradeoffs between clarity, performance, and safety

Good evidence:

- `replica-coordinator`
- `durable-event-log`
- `membership-registry`
- `protocol-stack`

You are probably ready if:

- you can speak clearly about message flow and failure cases
- your code is modular and tested
- you can talk through at least one system bottleneck

## Resume checklist

Before applying, confirm:

- your resume bullets describe what you built
- your bullets mention constraints, validation, testing, or recovery
- you do not overclaim seniority
- you do not list tools you barely used
- your capstone appears clearly and early

## GitHub checklist

Before applying, confirm:

- pinned repos are your strongest work
- READMEs explain architecture and run steps
- tests exist and are easy to discover
- repo names are understandable
- no important repo looks abandoned or broken

## Interview checklist

Before applying, confirm:

- you can explain one bug, one tradeoff, and one limitation per project
- you can answer Rust basics without hand-waving
- you can explain PDAs, signer checks, owner checks, and CPIs
- you can explain eventual consistency in plain English
- you can talk about failure cases honestly

## Honest target guidance

Apply for these confidently after strong completion:

- junior Rust engineer
- junior backend engineer
- junior protocol engineer
- early-to-mid Solana engineer
- systems-leaning backend roles

Apply selectively for these if your capstone quality is unusually high:

- mid Rust engineer
- mid Solana engineer
- mid protocol engineer

Do not center your application strategy around senior titles unless you already have strong production experience outside this curriculum.
