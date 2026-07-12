# Interview Bootcamp: 14 Days

This is a two-week interview curriculum for turning the roadmap into hiring outcomes.

It is designed for these role tracks:

- junior Rust engineer
- early-to-mid Solana engineer
- protocol engineering junior to mid
- backend/systems engineer with a strong self-taught project story

It also includes a reality check for senior-level expectations.

## Ground rules

Every day, do all four:

1. `Explain`
   One concept out loud without notes.
2. `Solve`
   One coding problem or one curriculum exercise from memory.
3. `Review`
   One of your own projects and identify one weakness.
4. `Write`
   One short answer to an interview-style question.

Daily time budget:

- 60 minutes concept review
- 60 minutes coding
- 45 minutes project review
- 30 minutes spoken practice

## Day 1: Rust foundations under pressure

Focus:

- ownership
- borrowing
- `Option`
- `Result`
- enums
- pattern matching

Tasks:

- explain ownership like you are five
- explain ownership technically
- solve one borrowing-focused week 2 exercise from scratch
- write one answer about why Rust rejects some programs that other languages allow

Interview questions:

- What problem does ownership solve?
- What is the difference between moving and borrowing?
- When do you use `Option` versus `Result`?
- Why are enums important in Rust design?

## Day 2: Rust data modeling and errors

Focus:

- structs
- enums
- error enums
- state modeling

Tasks:

- rebuild a small parser from memory
- explain why explicit states beat boolean flags
- review one project and identify where the data model could be tighter

Interview questions:

- How do you design a good error type?
- Why is an enum often better than multiple booleans?
- What makes a data model easy to evolve?

## Day 3: Traits, generics, iterators

Focus:

- reusable abstractions
- trait bounds
- iterator reasoning

Tasks:

- implement a tiny trait-based store
- explain what a trait is to a beginner
- explain one example where generics helped avoid duplication

Interview questions:

- What is a trait?
- When should you use generics?
- When does abstraction become overengineering?
- What is an iterator buying you besides shorter syntax?

## Day 4: Lifetimes, pointers, and memory

Focus:

- references
- lifetimes
- `Box`
- `Rc`
- `Arc`
- `RefCell`
- `Mutex`

Tasks:

- explain lifetimes in plain English
- compare `Rc` and `Arc`
- compare `RefCell` and `Mutex`
- review your own code and find one unnecessary clone

Interview questions:

- What does a lifetime represent?
- When would you use `Arc<Mutex<T>>`?
- Why is interior mutability useful?
- Why can cloning be a code smell?

## Day 5: Concurrency and systems basics

Focus:

- threads
- channels
- `Send`
- `Sync`
- concurrency vs parallelism
- backpressure

Tasks:

- explain channels with a real-world analogy
- solve one task-distribution exercise
- review your coordinator or queue-like code

Interview questions:

- What is the difference between concurrency and parallelism?
- Why choose channels over shared mutable state?
- What is backpressure?
- What does `Send` mean? What does `Sync` mean?

## Day 6: Distributed systems from scratch

Focus:

- retries
- timeouts
- idempotency
- message duplication
- stale state

Tasks:

- explain idempotency like you are five
- explain why retries are both useful and dangerous
- solve one deduplication or retry exercise again from memory

Interview questions:

- What is idempotency?
- Why are retries dangerous without careful design?
- What is stale state?
- Why are timeouts imperfect?

## Day 7: Logs, replay, recovery, eventual consistency

Focus:

- append-only logs
- replay
- durability
- read models
- eventual consistency

Tasks:

- explain why logs are everywhere in distributed systems
- explain eventual consistency to a non-engineer
- review your durable-event-log or indexing-related work

Interview questions:

- Why are logs useful?
- What does replay mean?
- What is eventual consistency?
- What happens when an indexer misses events?

## Day 8: Solana core concepts

Focus:

- accounts
- programs
- instructions
- transactions
- signer rules
- owner rules

Tasks:

- explain Solana accounts in plain English
- draw or describe a transaction lifecycle
- review week 8 exercises and explain every validation check

Interview questions:

- What is a Solana account?
- Why does Solana declare accounts up front?
- What is the difference between account owner and transaction signer?
- Why is Solana’s model good for parallelism?

## Day 9: PDAs and authority design

Focus:

- seeds
- bumps
- deterministic addresses
- authority models

Tasks:

- explain PDAs like you are five
- explain how PDAs differ from normal keypairs
- review one project and explain its authority model

Interview questions:

- What is a PDA?
- Why are PDAs useful?
- What kinds of mistakes happen in authority design?
- How would you explain derived authority to a product manager?

## Day 10: Anchor and account design

Focus:

- Anchor
- account constraints
- account sizing
- versioned updates

Tasks:

- explain what Anchor automates
- explain what Anchor does not protect you from
- review your task board or counter design

Interview questions:

- Why use Anchor?
- When might you prefer native Rust over Anchor?
- What account-sizing mistakes are common?
- Why are version checks useful?

## Day 11: Solana security and CPI

Focus:

- missing signer checks
- missing owner checks
- stale state
- CPIs
- token movement validation

Tasks:

- explain one vulnerability class clearly
- explain a CPI boundary and what must be checked before it
- review your vault-style project and identify its most dangerous bug if implemented badly

Interview questions:

- What is a CPI?
- What is the difference between signer validation and owner validation?
- What security mistake do beginners make most often?
- Why do negative tests matter?

## Day 12: Your project story

Focus:

- resume narrative
- GitHub narrative
- architecture explanation

Tasks:

- write a 60-second summary of your capstone
- write three resume bullets from your best project
- explain one tradeoff and one limitation honestly

Interview questions:

- Tell me about a project you built.
- What was the hardest bug?
- What would you improve next?
- Why did you choose this design over another one?

## Day 13: Mock interview day

Focus:

- switching quickly between Rust, systems, and Solana questions

Tasks:

- answer 10 questions out loud in one sitting
- solve one small Rust problem on paper or in a blank file
- whiteboard your capstone architecture
- identify where you sounded vague

Mock question set:

- Explain ownership.
- Explain idempotency.
- Explain eventual consistency.
- Explain Solana accounts.
- Explain PDAs.
- Explain one failure mode in your capstone.
- Explain one security check you added and why.
- Explain one Rust tradeoff in your code.
- Explain how your indexer recovers from lag.
- Explain what would break first at 10x usage.

## Day 14: Application day

Focus:

- convert preparation into action

Tasks:

- finalize resume
- finalize GitHub pinned repos
- finalize README for top project
- apply to real jobs
- write a short role-target list

## Junior Rust engineer interview questions

- What is ownership?
- What is borrowing?
- What is the difference between `String` and `&str`?
- What is the difference between `Option` and `Result`?
- What is pattern matching?
- Why use enums?
- What is a trait?
- When would you use a `HashMap`?
- What is the difference between `Rc` and `Arc`?
- What does the borrow checker protect you from?

## Early-to-mid Solana engineer interview questions

- What is a Solana account?
- What is the difference between an account owner and a signer?
- What is a PDA?
- Why are PDAs useful?
- What is an instruction?
- What is a transaction?
- Why must accounts be declared up front?
- What common Solana program security mistakes exist?
- What is a CPI?
- When would you use Anchor versus native Rust?

## Protocol engineering interview questions: junior to mid

- What are the invariants in your protocol?
- What are the main account types in your design?
- How does authority work in your protocol?
- What happens if a user retries the same request?
- How do you prevent stale writes?
- Where do you use deterministic state transitions?
- What off-chain components exist in your protocol stack?
- What happens if the indexer lags or misses data?
- How do you test failure cases?
- What is your biggest attack surface?

## Backend / systems engineer interview questions

- What is a state machine?
- What is idempotency?
- Why do retries need caution?
- What is eventual consistency?
- What is backpressure?
- What is the difference between concurrency and parallelism?
- Why are logs so common in distributed systems?
- What happens during crash recovery?
- What is a read model?
- What is the bottleneck in one of your systems projects?

## Senior-level reality check

If you are aiming at senior roles, expect questions beyond implementation:

- How would you review a junior engineer’s protocol design?
- What would you change in this architecture for production?
- How would you reduce operational risk during rollout?
- Which failure mode are we underestimating?
- How would you stage migrations or upgrades?
- How would you defend this design to security reviewers?

This curriculum can help you answer some of those questions, but senior roles usually require real production depth and judgment formed by operating live systems.

## Application strategy after the bootcamp

Apply broadly for:

- junior Rust
- junior backend
- junior protocol
- early-to-mid Solana

Apply selectively for:

- mid Rust
- mid Solana
- mid protocol

Use senior applications as stretch targets only if:

- you already have strong professional experience elsewhere
- your project depth is exceptional
- you interview unusually well
