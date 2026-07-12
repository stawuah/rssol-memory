# Curriculum Track Summary

This document summarizes what the curriculum covers in:

- Rust
- Distributed systems
- Solana

It is a track-level view of the full roadmap in
[../solana-rust-90-day-curriculum.md](/home/stephen/rustworkspace/learn-rs/solana-rust-90-day-curriculum.md:1).

## Rust Track

The Rust track is designed to take you from beginner syntax to systems-level engineering habits.

### Beginner Rust

Covered in the early weeks:

- variables
- mutability
- scalar types
- strings
- functions
- control flow
- loops
- `match`
- basic CLI programs

### Core Rust

This is the center of the language and the part that matters most for Solana and systems work:

- ownership
- borrowing
- shared references
- mutable references
- slices
- structs
- enums
- pattern matching
- `Option`
- `Result`
- custom error types

### Intermediate Rust

You then move into the features that make Rust scale across larger codebases:

- collections like `Vec`, `HashMap`, and `BTreeMap`
- modules and visibility
- traits
- generics
- iterators
- unit tests
- doc tests
- reusable library design

### Advanced Rust Foundations

The later Rust foundation weeks cover the parts that usually separate surface familiarity from real fluency:

- lifetimes
- stack vs heap reasoning
- memory layout awareness
- `Box`
- `Rc`
- `Arc`
- `RefCell`
- `Mutex`
- interior mutability
- avoiding unnecessary clones

### Concurrent and Systems Rust

The systems-oriented Rust part includes:

- threads
- channels
- `Send`
- `Sync`
- concurrency vs parallelism
- async foundations
- queue thinking
- backpressure thinking
- state coordination

### Data and Durability Rust

You also cover:

- serialization
- manual binary encoding
- file I/O
- append-only logs
- replay
- recovery
- workspace structure
- API cleanup
- profiling mindset

### What the Rust track builds

Projects and exercises that make the Rust track concrete:

- `node-heartbeat-cli`
- `raft-lite-state-machine`
- `replicated-log-sim`
- `distributed-kv-core`
- `membership-registry`
- `replica-coordinator`
- `durable-event-log`

## Distributed Systems Track

The distributed-systems track starts from scratch and is explained gradually, with real-life analogies, ELI5 versions, system models, failure models, and engineering responses.

### Foundations from zero

You start by learning:

- what a distributed system is
- how it differs from a single-process program
- nodes
- messages
- state
- waiting
- latency
- partial failure

### Early core concepts

The first important concepts introduced are:

- retries
- timeouts
- stale state
- heartbeats
- message duplication
- state transitions
- finite-state machines

These are deliberately introduced before advanced theory so the mental model is built from behavior first.

### Reliability and correctness concepts

The curriculum then covers:

- idempotency
- duplicate request handling
- explicit invariants
- error classification
- deterministic transitions
- replay and recovery
- durable state

### Coordination and shared state

As the Rust track gets stronger, the distributed track deepens into:

- leader / follower / candidate roles
- local state vs shared state
- safe mutation
- shared ownership
- queues
- event loops
- backpressure

### Storage and log-based thinking

You also learn:

- append-only logs
- ordered replay
- rebuilding derived state
- persistence vs in-memory state
- why logs are so common in distributed systems

### Eventual consistency and product architecture

In the later weeks the track moves into product-facing distributed systems ideas:

- read models
- event consumers
- indexers
- cache recovery
- lagging replicas
- eventual consistency
- bottlenecks
- failure propagation across layers

### Teaching method for distributed systems

Every important distributed concept is meant to be explained through this ladder:

1. Real-life analogy
2. Explain like I’m five
3. System model
4. Failure model
5. Design response
6. Solana mapping when relevant

This structure is defined in
[shared-templates/distributed-concept-ladder.md](/home/stephen/rustworkspace/learn-rs/roadmap/shared-templates/distributed-concept-ladder.md:1).

### What the distributed systems track builds

Projects and exercises connected to distributed systems:

- heartbeat monitoring
- simple election/state-machine logic
- replicated-log simulation
- key-value command application
- membership tracking
- task dispatch and coordination
- durable event logging
- lagging indexer and cache recovery exercises

## Solana Track

The Solana track begins only after enough Rust and systems grounding has been built. That is intentional.

The goal is not to learn Solana as isolated framework trivia, but as a deterministic state-machine platform built on strict state, authority, and execution rules.

### Solana foundations

The first Solana concepts introduced are:

- accounts
- programs
- instructions
- transactions
- rent
- signers
- writable accounts
- ownership checks
- account data layout

These are taught before heavy framework use.

### Solana execution model

The curriculum then explains:

- deterministic program execution
- explicit account declaration
- why Solana declares read/write sets up front
- how this relates to parallel execution
- why programs mutate account state instead of holding internal storage

### PDAs and authority

One of the most important Solana sections covers:

- PDAs
- seeds
- bumps
- derived authority
- address determinism
- safe program-controlled state

This is where the Rust ownership mindset and distributed authority mindset connect directly to Solana.

### Native Rust Solana development

Before relying on Anchor, the curriculum introduces native Rust Solana thinking:

- manual instruction decoding
- byte layouts
- fixed-size account state
- explicit validation
- deterministic updates

### Anchor development

After the foundations are strong, the curriculum moves into Anchor:

- Anchor project structure
- handlers
- account constraints
- seeds and bumps in Anchor
- custom errors
- event patterns
- bounded account sizing

The point is to use Anchor productively without confusing the framework with the protocol model.

### Solana security and correctness

A dedicated section covers the common mistake patterns that matter in real Solana work:

- missing signer checks
- missing owner checks
- stale or mismatched state assumptions
- authority mistakes
- unsafe update rules
- bad account sizing assumptions
- negative testing mindset

### CPI and token interaction

The curriculum also includes:

- CPI thinking
- validation before calling another program
- SPL Token basics
- Token-2022 awareness
- safe boundary handling

### Off-chain Solana architecture

The later Solana weeks expand from program code to full protocol/product architecture:

- client-side instruction building
- simulation
- event consumers
- indexers
- read models
- lag and recovery
- eventual consistency between chain state and product views

### Capstone Solana outcomes

By the end of the Solana track, the learner should be able to reason about and build:

- native Rust Solana programs
- Anchor-based Solana programs
- authority-controlled state transitions
- simple token or vault logic
- indexed protocol stacks
- a product-grade capstone with documentation and testing

### What the Solana track builds

Projects and exercises on the Solana side:

- `solana-native-hello-state`
- `anchor-task-board`
- `vault-program`
- `protocol-stack`
- final capstone

## How the tracks fit together

The curriculum is not three unrelated courses.

It is one sequence:

1. Rust teaches safe local reasoning
2. Distributed systems teaches reasoning about messages, failure, and state across machines
3. Solana applies those lessons to deterministic blockchain programs and the off-chain systems around them

That sequencing is the main strength of the curriculum.

## Where to read more in this roadmap

- Full roadmap: [README.md](/home/stephen/rustworkspace/learn-rs/roadmap/README.md:1)
- Study method: [study-workflow.md](/home/stephen/rustworkspace/learn-rs/roadmap/study-workflow.md:1)
- Concept progression: [concept-map.md](/home/stephen/rustworkspace/learn-rs/roadmap/concept-map.md:1)
- Mastery levels: [mastery-rubric.md](/home/stephen/rustworkspace/learn-rs/roadmap/mastery-rubric.md:1)
- Shared terminology: [glossary.md](/home/stephen/rustworkspace/learn-rs/roadmap/glossary.md:1)
