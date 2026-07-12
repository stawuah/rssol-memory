# Protocol Core Glossary

Terms are alphabetized. Each definition is followed by why it matters here.

## A

**Account constraints (`#[account(...)]`)** — Anchor rules that check properties
such as mutability, seeds, owner, address, and relationships before a handler runs.
They matter because caller-supplied accounts are untrusted.

**`#[derive(Accounts)]`** — Anchor's derive macro that generates account parsing and
constraint validation for a context struct. It defines much of an instruction's
security boundary before handler logic runs.

**Account owner vs authority** — The owner is the program allowed to modify account
data; an authority is a protocol-defined key allowed to request an action. Confusing
them creates authorization vulnerabilities.

**Accounts** — Solana's persistent records containing lamports, owner, and bytes.
Protocol state and token balances ultimately live in accounts.

**Address lookup table (ALT)** — An on-chain table whose addresses can be referenced
compactly by versioned transactions. ALTs help fit account-heavy workflows but their
authority and contents are another trust boundary.

**Adversarial test** — A test written from an attacker's perspective using hostile
accounts, ordering, values, or state. Every exercise requires one to prevent
happy-path-only learning.

**Arc / Mutex** — `Arc` shares ownership across threads; `Mutex` permits one guarded
mutator at a time. They matter mainly for off-chain services, indexers, and keepers,
not persistent on-chain state.

**Associated type** — A type selected by a trait implementation, such as its error
or output. Anchor and reusable protocol abstractions use associated types heavily.

**async / await** — Rust syntax for suspending asynchronous work without blocking a
thread. It matters in RPC clients, keepers, monitoring, and deployment tooling.

## B

**Borrowing** — Temporary access through `&T` or exclusive `&mut T` without taking
ownership. Account handlers rely on safe borrows while validation protects the
on-chain trust boundary.

**Bump** — A one-byte value used with seeds to find a valid PDA. Canonical bump and
seed validation bind state to the intended program-derived address.

## C

**Channels** — Rust send/receive queues for coordinating tasks or threads. They help
off-chain workers separate RPC ingestion, simulation, signing, and monitoring.

**Checked / wrapping / saturating arithmetic** — Checked math reports overflow,
wrapping math rolls modulo the type range, and saturating math clamps at a bound.
Financial invariants normally require checked math and explicit failure.

**Circuit breaker** — A rule that pauses risky behavior when prices, deviations, or
system conditions exceed limits. It prevents questionable inputs from cascading.

**Commit-reveal** — A two-phase pattern that first stores a hash, then later reveals
the bound payload and nonce. It hides information temporarily and reduces some
front-running, while adding timing and non-reveal problems.

**Completeness** — A proof system property that true statements with valid witnesses
can be proven successfully. It is one of three properties needed to reason about ZK.

**Compute budget / compute units** — Compute units meter execution; budget instructions
request limits and prices. Production transactions must fit measured worst cases.

**Confidence interval** — An oracle's uncertainty range around its reported price.
Wide confidence may require conservative valuation or rejection.

**Confidential transfer** — A Token-2022 flow that hides amounts with encrypted
balances and proofs while other metadata may remain visible. It teaches precise
privacy claims rather than “everything is private.”

**Context** — Anchor's handler wrapper containing validated accounts and bumps. Its
lifetimes and account types define the instruction boundary.

**CPI (cross-program invocation)** — One Solana program invoking another. Program
identity, forwarded accounts, privileges, return data, and signer seeds must all be
validated.

## D

**Devnet / mainnet** — Devnet is a public testing cluster; mainnet carries production
economic value. Completion requires devnet evidence but never implies mainnet safety.

**Discriminator** — Anchor's type-identifying prefix on account or instruction data.
Checking it prevents one serialized type from being interpreted as another.

## E

**ElGamal encryption** — A public-key encryption scheme with homomorphic properties
used conceptually in confidential token balances. Encryption alone does not prove a
hidden amount is valid, so range and equality proofs are required.

**Epoch** — A group of Solana slots used for schedules and protocol-level timing.
Operational and validator behavior can change at epoch boundaries.

**Error codes (`#[error_code]`)** — Anchor-defined domain failures surfaced to clients.
Stable, specific errors make monitoring, simulation, and regression tests reliable.

## F

**First-depositor attack** — A vault attacker manipulates the initial share price,
often through donation, so a victim receives zero shares. Virtual offsets and
minimum-share checks defend share accounting.

**Fixed-point arithmetic** — Integers represent fractions through a fixed scale such
as Q32.32. It provides deterministic on-chain financial math with explicit rounding.

**Front-running** — Observing a pending or predictable action and executing first to
profit or change its outcome. Commit-reveal and strict execution bounds address some
forms.

**Fuzzing** — Automatically generating instruction sequences and inputs to discover
violations. Trident exercises require model-based invariants rather than random-byte
deserialization failures.

## G

**Generics / trait bounds** — Generics reuse code across types; bounds state required
behavior. They explain many Anchor APIs but should not obscure security boundaries.

**Griefing** — Causing cost, delay, or denial of service without direct profit.
Permissionless keepers, queues, and commitments need spam and non-reveal defenses.

**Groth16** — A succinct proof system commonly offering small proofs and fast
verification with circuit-specific setup. Its setup and circuit lifecycle matter as
much as verifier speed.

## H

**Health factor** — Risk-adjusted collateral divided by debt, commonly liquidatable
below one. Correct scaling, prices, and thresholds protect protocol solvency.

## I

**IDL / IDL versioning** — Anchor's interface description of instructions, accounts,
types, and errors; versioning connects deployed binaries to compatible clients.

**Instruction** — A program ID, account metadata list, and input bytes. It is the
smallest program invocation unit inside a transaction.

**Invariant** — A fact that must remain true through every public transition. Tests,
fuzzing, and audits in this curriculum are organized around invariants.

## K

**Keeper network** — Permissionless or selected workers that trigger maintenance
such as liquidation or order execution. Idempotency, rewards, and grief resistance
determine whether it remains live and affordable.

**Kinked interest curve** — A utilization-based rate with a steeper slope above a
target. It protects lending liquidity as utilization approaches 100%.

## L

**Leader schedule** — The planned assignment of validators to produce blocks/slots.
Ordering and liveness analysis must account for leader behavior.

**Lifetimes** — Compile-time relationships describing how long references remain
valid. Anchor's `'info` connects borrowed accounts during instruction execution.

**Light-style ZK compression** — Committed state stored in Merkle structures with
validity proofs rather than one ordinary account per item. It reduces state cost but
adds proof, root, and nullifier trust boundaries.

**Liquidation** — Repaying unhealthy debt in exchange for collateral, often with a
bonus. Correct close factors, prices, rounding, and caps prevent unfair seizure.

## M

**Mainnet-readiness runbook** — A written procedure for deployment, verification,
monitoring, incidents, and rollback. Production safety requires repeatable operations.

**Merkle tree** — A hash tree whose root commits to many leaves; a path proves leaf
membership. Compression and ZK exercises bind proofs to current accepted roots.

**MEV (maximal extractable value)** — Value gained by controlling or exploiting
transaction inclusion and ordering. Sandwich attacks and trigger leakage are examples.

**Move semantics** — Passing a non-`Copy` Rust value transfers ownership. Understanding
moves prevents accidental clones and confused state APIs.

**Multisig** — Governance requiring a threshold of distinct member approvals.
Proposal binding, member identity, replay prevention, and execution-once rules matter.

**`mut` vs shadowing** — `mut` changes one binding's value; shadowing creates a new
binding with the same name. Explicit mutation keeps state transitions reviewable.

## O

**Overcollateralization** — Requiring collateral value above issued debt. It provides
a buffer for volatility and liquidation delay but depends on oracle and market depth.

**Ownership** — Rust's rule assigning responsibility for a value's lifetime. It is
distinct from Solana account ownership.

## P

**Panic vs error** — A panic aborts unexpectedly; a returned error represents a
known rejected condition. Untrusted program input should produce explicit errors.

**PDA (program-derived address)** — An address derived from seeds and program ID with
no private key. Programs sign for PDAs during CPI using runtime-verified seeds.

**Peg** — A stablecoin's target market value. Collateral rules support confidence but
do not alone guarantee market price equals the peg.

**PLONK** — A family of proof systems with universal/updatable setup and flexible
circuit workflows, with different proof and verifier tradeoffs from Groth16.

**Price feed / Pyth / Switchboard** — A feed publishes external market data; Pyth and
Switchboard are oracle systems. Consumers must validate feed identity, freshness,
confidence, scale, and independence assumptions.

**Program / `#[program]`** — A Solana program is executable code; Anchor's macro marks
instruction handlers. Programs store mutable state in separate accounts.

**Program immutability** — Absence of an upgrade authority capable of replacing code.
It removes upgrade trust but also emergency repair capability.

**Proof / prover / verifier** — The prover creates evidence for a statement using a
witness; the verifier checks it against public inputs. Protocol code must bind proof,
verifier identity, and exact statement.

**Property-based testing** — Generating many structured cases and checking general
properties rather than a few examples. It bridges unit tests and stateful fuzzing.

## R

**Range proof** — A proof that a hidden number falls within an allowed interval.
Confidential transfers need it to prevent negative or overflowing hidden balances.

**Reentrancy** — Re-entering code before an earlier call finishes; Solana locking
differs from EVM callbacks, but CPI-induced stale state and inconsistent effects are
analogous risks.

**Regression test** — A deterministic test permanently reproducing a previously found
bug. Every minimized fuzz failure should become one.

**Rent / rent exemption** — Accounts need lamports associated with allocated storage;
closing can refund them. Correct sizing and recipients are lifecycle invariants.

**Result / Option / `?`** — `Result` models success/failure, `Option` presence/absence,
and `?` propagates failure. They keep rejected protocol transitions explicit.

**Runbook** — An operational document another engineer can execute during deployment
or incident response. Module F treats it as a required artifact, not code comments.

## S

**Sandwich attack** — An attacker trades before a victim to worsen price and trades
afterward to capture the movement. Minimum output and ordering-aware design limit it.

**Seeds** — Ordered byte strings used with a program ID and bump to derive a PDA.
Seed design is part of account identity and authorization.

**Share-based accounting** — Depositors hold proportional claims rather than fixed
assets. Conversion rounding and donation behavior are core vault invariants.

**Signer / writable account** — A signer proves key authorization; writable metadata
permits mutation and locking. Neither alone proves protocol authority.

**Slices / `&str` vs `String`** — Slices borrow sequences; `&str` borrows UTF-8 while
`String` owns it. Borrowed validated views can reduce on-chain allocation.

**Slippage** — Difference between expected and executed trade price/output. Minimum-out
limits make unacceptable execution revert.

**Slot** — A Solana leader opportunity and common freshness unit. Slot gaps mean slot
age is a protocol policy rather than exact wall-clock time.

**Soundness** — A proof property that false statements cannot feasibly convince the
verifier. Accepting unbound public inputs destroys useful soundness at the application layer.

**Space calculation** — Computing maximum serialized account bytes including tags,
length prefixes, and discriminator. Under-allocation can lock state updates.

**Staleness** — Oracle data older than allowed policy. Risky operations should fail
closed when fresh data is unavailable.

## T

**Timelock** — A required delay between queuing and executing a sensitive action.
Payload binding, cancellation, expiry, and execute-once behavior are essential.

**Traits** — Rust behavior contracts implemented by multiple types. Anchor uses them
for serialization, account loading, identity, and instruction interfaces.

**Transaction** — An atomic signed bundle of instructions with recent blockhash and
account list. All succeed or state changes roll back.

**Trust boundary** — A point where data, authority, or behavior not controlled by the
protocol enters. Reviews must name every boundary and its validation.

**TWAP** — A time-weighted average price giving observations weight by duration.
It reduces sensitivity to brief spot manipulation but requires good history policy.

## U

**unsafe** — Rust code where the programmer manually upholds memory-safety contracts.
Zero-copy and performance code require narrow, documented, reviewed unsafe boundaries.

**Upgrade authority** — The key or governance allowed to replace program code.
Its custody is a central production trust assumption.

**Utilization rate** — Borrowed liquidity divided by supplied liquidity. Lending
curves use it to price scarcity and encourage repayment/supply.

## V

**Validator** — A Solana node participating in consensus and transaction processing.
Deterministic programs must produce the same result across validators.

**Versioned transaction** — A transaction format supporting features such as ALTs.
It helps account-heavy workflows but still obeys compute, locks, and packet limits.

## Z

**Zero-copy / `#[account(zero_copy)]`** — Reading typed account memory without ordinary
full deserialization. It improves compute at the cost of strict stable layout rules.

**Zero-knowledge proof** — A proof revealing no information beyond a statement's truth
under its defined model. It does not hide public inputs, timing, or surrounding metadata.

**ZK compression** — Representing many state items through commitments and validity
proofs rather than ordinary accounts. It trades storage cost for proof infrastructure.
