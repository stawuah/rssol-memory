# Rust to Anchor: The Practical Field Guide

**A visual reference for `rustlings-core`, `anchor-core`, and your first Solana protocols**

Version 2 | Designed for learning, revision, and use beside your editor

> The central model: a Solana program is a deterministic Rust state machine. A caller supplies
> accounts and instruction data. Your program must distrust them, validate them, apply one legal
> transition, and preserve every protocol invariant.

---

## How to use this guide

Do not memorize every page. Use three passes:

1. **Learn:** read the Rust concept and type the example yourself.
2. **Translate:** explain the matching Anchor or Solana concept aloud.
3. **Defend:** identify what could go wrong with malicious inputs.

For every instruction, ask:

- What state may change?
- Who is authorized to request the change?
- Are all addresses, owners, types, signers, and writable flags correct?
- Which invariant must still be true afterward?
- Can arithmetic, allocation, CPI, or compute usage fail?

---

## Table of contents

1. The Rust-to-Solana map
2. Values, variables, types, and control flow
3. Ownership, borrowing, slices, and lifetimes
4. Structs, enums, pattern matching, and state machines
5. `Option`, `Result`, errors, and checked arithmetic
6. Collections, iterators, traits, generics, and modules
7. Serialization, memory layout, and account sizing
8. Solana accounts, programs, instructions, and transactions
9. Anchor program anatomy and account constraints
10. PDAs, CPIs, tokens, and closing accounts
11. Protocol invariants and security review
12. Testing, debugging, and compute optimization
13. Worked vault example
14. Common compiler errors and translations
15. Recommended Rustlings expansion and study plan

---

# 1. The Rust-to-Solana map

| Rust idea | Plain-English meaning | Solana / Anchor mapping |
|---|---|---|
| `let mut` | This binding may change | Local state may change |
| `&T` | Temporary read access | Read-only account/reference |
| `&mut T` | One temporary writer | Mutable account data access |
| `struct` | Named fields forming one value | Account state or instruction data |
| `enum` | Exactly one of several cases | Protocol phase, command, or error |
| `match` | Handle every possible case | Explicit state-transition logic |
| `Result<T, E>` | Success or known failure | Instruction result / program error |
| `checked_add` | Addition that detects overflow | Safe balance and accounting math |
| lifetime | How borrowed references relate | Account references inside `Context` |
| serialization | Value encoded as bytes | Account data and instruction data |
| trait | Shared behavior contract | Anchor serialization/account traits |

Important distinctions:

| Often confused | Correct distinction |
|---|---|
| Rust owner vs account owner | Rust owner manages memory; Solana owner is the program allowed to modify account data. |
| Authority vs signer | Authority is a protocol role; signer proves control of a private key. Usually you must check both. |
| Rust `mut` vs Anchor `mut` | Rust permits mutation through a binding; Anchor declares an account writable in the transaction. |
| Account address vs account data | Address identifies the account; serialized bytes hold its state. |
| PDA vs wallet | A PDA has no private key; its program can sign using verified seeds during a CPI. |

---

# 2. Values, variables, types, and control flow

## Bindings and mutability

```rust
let balance = 10_u64;       // immutable
let mut attempts = 0_u8;    // mutable
attempts += 1;

const MAX_POSITIONS: usize = 32;
```

Prefer immutability until mutation is required. It narrows the places where state can change and
makes audits easier. Use `const` for compile-time protocol limits.

## Integer types

| Type | Bytes | Typical protocol use |
|---|---:|---|
| `u8` | 1 | bump, status, small counter |
| `u16` | 2 | bounded count |
| `u32` | 4 | timestamp offset, index |
| `u64` | 8 | lamports, token amount, slot |
| `u128` | 16 | intermediate multiplication, fixed-point math |
| `i64` | 8 | Unix timestamp, signed change |
| `usize` | platform | in-memory indexing; avoid as persistent portable state |

Never use floating point for financial protocol state. Define a fixed-point scale, units, precision,
and rounding direction. Rounding determines who gains or loses value.

```rust
let product = (amount as u128)
    .checked_mul(price as u128)
    .ok_or(Error::Overflow)?;
let quote = product
    .checked_div(SCALE)
    .ok_or(Error::DivisionByZero)?;
```

## Control flow

`if` handles conditions. `match` handles variants and is exhaustive. Loops must be bounded on-chain.

```rust
if amount == 0 {
    return Err(Error::ZeroAmount);
}

for position in positions.iter().take(MAX_POSITIONS) {
    // bounded work
}
```

**Protocol rule:** validate cheap conditions before hashing, allocation, loops, or CPIs.

---

# 3. Ownership, borrowing, slices, and lifetimes

## Ownership

Every Rust value has one owner. Moving a non-`Copy` value transfers ownership. When the owner leaves
scope, Rust drops the value.

```rust
let label = String::from("vault");
let moved = label;           // label is no longer usable

let amount = 10_u64;
let copied = amount;         // u64 is Copy; both are usable
```

Use `.clone()` only when you truly need another owned value. Cloning to silence the compiler often
hides a confused ownership design and can waste compute.

## Borrowing

```text
many readers:  &T  &T  &T       OR       one writer:  &mut T
```

```rust
fn read_balance(vault: &Vault) -> u64 {
    vault.balance
}

fn freeze(vault: &mut Vault) {
    vault.frozen = true;
}
```

A borrow temporarily accesses a value without owning it. Shared references read. A mutable reference
is exclusive so nobody else can read or write through another reference at the same time.

## Strings and slices

| Type | Owns data? | Growable? | Meaning |
|---|---|---|---|
| `String` | yes | yes | owned UTF-8 text |
| `&str` | no | no | borrowed UTF-8 view |
| `Vec<T>` | yes | yes | owned sequence |
| `&[T]` | no | no | borrowed sequence view |
| `[T; N]` | yes | no | fixed-size array |

```rust
fn prefix_until(text: &str, delimiter: char) -> &str {
    &text[..text.find(delimiter).unwrap_or(text.len())]
}
```

String indexes are byte offsets. Slicing panics if an index splits a UTF-8 character. For untrusted
text, use safe methods such as `find`, `char_indices`, or validated ASCII inputs.

## Lifetimes

A lifetime describes how references relate; it does not keep data alive.

```rust
fn choose<'a>(left: &'a str, right: &'a str, first: bool) -> &'a str {
    if first { left } else { right }
}
```

Read `'a` as: “the returned reference cannot outlive the input references represented by `'a`.”

Anchor types often contain `'info`:

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.balance = vault.balance.checked_add(amount).unwrap();
    Ok(())
}
```

`'info` connects the references stored in the account context. Most handlers need no manual lifetime
annotations; helper functions involving several account references sometimes do.

### Borrow-checker repair checklist

1. Is a value moved when the function only needs `&T`?
2. Is a mutable borrow held longer than necessary?
3. Can you copy a small field before creating another borrow?
4. Can a large expression be split into smaller scopes?
5. Are you cloning merely to avoid understanding the lifetime?

---

# 4. Structs, enums, pattern matching, and state machines

## Structs and methods

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vault {
    pub authority: [u8; 32],
    pub balance: u64,
    pub frozen: bool,
}

impl Vault {
    pub fn can_withdraw(&self, signer: &[u8; 32]) -> bool {
        !self.frozen && &self.authority == signer
    }
}
```

Use `&self` to read, `&mut self` to change, and `Self` for constructors. Derived traits support
debugging, comparisons, copying, defaults, or serialization depending on the trait.

## Enums and `match`

```rust
enum Phase {
    Uninitialized,
    Active,
    Frozen,
    Closed,
}

fn withdraw_allowed(phase: Phase) -> bool {
    match phase {
        Phase::Active => true,
        Phase::Uninitialized | Phase::Frozen | Phase::Closed => false,
    }
}
```

Enums make illegal states harder to represent. A state machine should define:

- valid states;
- allowed transitions;
- authority required for each transition;
- invariant before and after each transition;
- behavior when an instruction is repeated.

```text
Uninitialized --initialize--> Active --freeze--> Frozen --close--> Closed
                                 |                   |
                                 +--- deposit -------+  (reject when frozen)
```

Avoid magic status numbers and scattered boolean combinations when an enum expresses the model.

---

# 5. Option, Result, errors, and checked arithmetic

## `Option<T>`

`Some(value)` means present; `None` means absent.

```rust
let authority = optional_authority.ok_or(Error::MissingAuthority)?;
```

## `Result<T, E>` and `?`

```rust
fn parse_amount(raw: &str) -> Result<u64, AmountError> {
    let amount = raw.parse::<u64>().map_err(|_| AmountError::Invalid)?;
    if amount == 0 { return Err(AmountError::Zero); }
    Ok(amount)
}
```

The `?` operator unwraps success and returns failure to the caller. Do not use `unwrap()` or
`expect()` on untrusted instruction/account data. Return a meaningful program error.

## Checked arithmetic

| Risk | Safe operation |
|---|---|
| addition overflow | `checked_add` |
| subtraction underflow | `checked_sub` |
| multiplication overflow | `checked_mul` |
| divide by zero | `checked_div` |
| lossy conversion | `TryFrom` / `try_into` |

```rust
fn debit(balance: u64, amount: u64) -> Result<u64, Error> {
    balance.checked_sub(amount).ok_or(Error::InsufficientFunds)
}
```

Use a wider intermediate type for multiplication, but verify the final downcast:

```rust
let wide = (amount as u128).checked_mul(rate as u128).ok_or(Error::Overflow)?;
let result: u64 = wide.try_into().map_err(|_| Error::Overflow)?;
```

---

# 6. Collections, iterators, traits, generics, and modules

## Collections and bounded input

`Vec<T>` is growable. Arrays have fixed length. Maps associate keys and values. Every collection
stored or processed on-chain needs a maximum size or an account-growth strategy.

Unbounded collections risk:

- insufficient account space;
- compute exhaustion;
- excessive allocation;
- denial of service;
- transactions that can no longer update oversized state.

```rust
if members.len() >= MAX_MEMBERS {
    return Err(Error::CapacityReached);
}
```

For large sets, consider one PDA per item or paged accounts instead of one ever-growing vector.

## Iterators

```rust
let total = positions
    .iter()
    .try_fold(0_u64, |sum, p| sum.checked_add(p.amount))
    .ok_or(Error::Overflow)?;
```

Useful methods: `iter`, `iter_mut`, `map`, `filter`, `find`, `any`, `all`, `fold`, `try_fold`,
`enumerate`, and `collect`. `collect` allocates; process items directly when a new collection is
unnecessary. Elegant iterator code is not automatically cheaper—measure hot paths.

## Traits and generics

```rust
trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

fn checked<T: Validate>(value: &T) -> Result<(), Error> {
    value.validate()
}
```

A trait defines behavior. Generics reuse logic across types that satisfy trait bounds. Anchor uses
traits extensively for account loading, serialization, discriminators, owners, and instruction data.

## Modules and visibility

```text
src/
  lib.rs          program entrypoints
  instructions/   one module per operation
  state/          account state and invariants
  error.rs        protocol errors
  constants.rs    seeds, limits, precision
```

Items are private unless declared `pub`. Keep handlers small. Pure calculation and state-transition
functions are easier to test than logic tangled with account loading.

---

# 7. Serialization, memory layout, and account sizing

Persistent state is bytes. Serialization defines how typed values become bytes and return to types.
Anchor commonly uses an 8-byte account discriminator followed by serialized fields.

| Value | Typical serialized size |
|---|---:|
| discriminator | 8 bytes |
| `bool` / `u8` | 1 byte |
| `u16` | 2 bytes |
| `u32` | 4 bytes |
| `u64` / `i64` | 8 bytes |
| `u128` | 16 bytes |
| `Pubkey` | 32 bytes |
| `String` | 4-byte length + UTF-8 bytes |
| `Vec<T>` | 4-byte length + encoded elements |
| `Option<T>` | tag + possible encoded `T` |
| enum | variant tag + largest possible variant data |

Example account:

```rust
#[account]
pub struct Profile {
    pub authority: Pubkey, // 32
    pub score: u64,        // 8
    pub active: bool,      // 1
    pub name: String,      // 4 + MAX_NAME_BYTES
}
```

```text
space = 8 + 32 + 8 + 1 + 4 + MAX_NAME_BYTES
        ^ discriminator
```

Allocate for the maximum future encoded value, not only the initial value. Reallocation requires
explicit design, rent funding, size limits, and compute consideration. Validate data owner, length,
and discriminator before trusting deserialized content.

Zero-copy layouts trade convenience for stricter alignment, padding, and compatibility concerns.
Use them only when profiling shows they are needed and you understand the layout rules.

---

# 8. Solana accounts, programs, instructions, and transactions

## Account model

A Solana account contains:

- **address:** public identity;
- **lamports:** native-token balance;
- **owner:** program allowed to modify its data;
- **data:** persistent byte array;
- **executable flag:** whether the account contains a program;
- **rent-related status:** sufficient balance for its allocation rules.

Programs are executable code. They do not keep mutable global state; application state lives in
separate accounts.

## Execution flow

```text
client builds instruction
        |
        v
transaction lists accounts + signatures + recent blockhash
        |
        v
runtime locks writable accounts and invokes program
        |
        v
Anchor validates account context
        |
        v
handler validates business rules and changes state
        |
        v
all instructions commit, or the transaction rolls back
```

An instruction includes program ID, account metadata, and encoded instruction data. A transaction
contains one or more instructions. Transactions are atomic: all succeed or all state changes roll
back. Atomicity does not make incorrect logic safe.

## Account properties answer different questions

- **Address:** Is this the exact account expected?
- **Owner:** Is this account data controlled by the expected program?
- **Signer:** Did a private key authorize this transaction?
- **Writable:** May this transaction modify this account?
- **Authority field:** Does protocol state grant this signer the requested role?
- **Discriminator/type:** Is this the expected kind of serialized account?

Checking only “is signer” is not enough if any signer can pass. Checking only an authority field is
not enough if that authority did not sign.

---

# 9. Anchor program anatomy and account constraints

```rust
use anchor_lang::prelude::*;

declare_id!("...");

#[program]
pub mod example {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, ProtocolError::ZeroAmount);
        let vault = &mut ctx.accounts.vault;
        vault.balance = vault.balance
            .checked_add(amount)
            .ok_or(ProtocolError::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
}
```

## Constraint reference

| Constraint/type | What it establishes |
|---|---|
| `Signer<'info>` | account signed the transaction |
| `mut` | account may be written |
| `owner = id` | account owner matches program ID |
| `address = key` | exact address matches |
| `has_one = authority` | stored key equals supplied account key |
| `seeds`, `bump` | account is the expected PDA |
| `init` | create and initialize account |
| `init_if_needed` | initialize conditionally; requires careful replay semantics |
| `close = recipient` | drain lamports and close to chosen recipient |
| `constraint = expr` | custom boolean relationship |
| `Account<T>` | owned, typed Anchor account |
| `UncheckedAccount` | little automatic validation; burden is yours |
| `Program<T>` | supplied executable program has expected ID |

Constraints are the first validation wall, not the full business logic. Still validate amount,
phase, capacity, time, price freshness, limits, and cross-account invariants.

## Validation order

1. identity and program IDs;
2. signer and authority;
3. owner and account type;
4. PDA/address relationships;
5. state phase and replay protection;
6. numeric bounds and arithmetic;
7. mutate state;
8. perform carefully reviewed CPI if needed;
9. verify final invariant when appropriate.

---

# 10. PDAs, CPIs, tokens, and closing accounts

## PDAs

A PDA is derived from seeds, a program ID, and a bump. Same inputs produce the same address. It has
no private key. The owning program can sign for the PDA during CPI with verified signer seeds.

```text
PDA = derive(["position", market_pubkey, user_pubkey], program_id, bump)
```

Seed design defines a namespace and often part of authorization. Use stable domain prefixes and bind
all relevant identities. Avoid ambiguous concatenation. Enforce seed length/count restrictions.

Knowing a PDA address is not authorization. A user action still requires the correct signer or other
explicit authority rule.

## Cross-program invocations

A CPI asks another program to perform work. Treat it as a trust boundary:

- verify the target program ID;
- validate every forwarded account;
- forward only required writable/signer privileges;
- understand which accounts the callee can change;
- use correct PDA signer seeds;
- propagate errors unless intentionally handled;
- account for additional compute and call depth.

## Tokens

Token balances live in token accounts governed by a token program. Validate the token program variant,
mint, token-account authority, decimals assumptions, and any Token-2022 extensions. Do not accept an
arbitrary token program or mint when protocol economics require a specific one.

Transfer logic commonly needs:

```text
source token account -> expected mint and authority
destination account  -> expected mint and recipient
token program        -> expected SPL Token or Token-2022 program ID
amount               -> correct base units, checked bounds
```

## Closing accounts

Closing normally sends remaining lamports to a recipient and invalidates the data account. Verify the
recipient, authorization, and terminal state. Prevent revival/reinitialization paths that violate replay
or uniqueness assumptions.

---

# 11. Protocol invariants and security review

An invariant is a fact that must remain true before and after every public instruction.

Example vault invariants:

- stored authority never changes without authorized rotation;
- balance equals the protocol's internal accounting definition;
- withdrawal never exceeds available balance;
- frozen vaults cannot deposit or withdraw;
- vault PDA is uniquely bound to its authority;
- only the expected token mint is accepted;
- closing is allowed only when balance is zero.

## Security review matrix

| Area | Questions to answer |
|---|---|
| Authorization | Which role may act? Did the correct key sign? Can authority be substituted? |
| Identity | Are exact addresses or PDA seeds verified? Is the bump canonical/expected? |
| Ownership | Is each data account owned by the expected program? |
| Type | Are discriminator, data length, and serialization trusted safely? |
| Aliasing | Can the same account be passed into two roles and break assumptions? |
| Arithmetic | Overflow, underflow, division by zero, precision loss, bad rounding? |
| State | Is transition legal? What happens on replay or duplicate requests? |
| CPI | Correct target? Minimal privileges? Reentrancy-like state inconsistency? |
| Tokens | Correct mint, token program, authority, decimals, extensions? |
| Time/oracles | Is data fresh? Is clock manipulation tolerance defined? |
| Resources | Are loops, inputs, allocations, account growth, and compute bounded? |
| Closing | Correct recipient and terminal conditions? Can state be revived? |

## Checks-effects-interactions

A useful mental order is:

```text
CHECKS: validate all inputs and accounts
EFFECTS: place local protocol state in a consistent form
INTERACTIONS: call other programs with minimum privileges
```

Solana's runtime differs from EVM execution, but the discipline still helps prevent inconsistent state
around CPIs. Understand the exact account reload/serialization behavior of your framework version.

## Never trust

- account order supplied by a client;
- arbitrary program IDs;
- a signer merely because it signed;
- stored data without owner/type validation;
- user-provided lengths, indices, bumps, decimals, or timestamps;
- a token account without checking mint and authority;
- arithmetic because “the number will never get that large”;
- client-side validation as a security control.

---

# 12. Testing, debugging, and compute optimization

## Test pyramid

```text
              cluster / end-to-end tests
           program integration tests (PDAs, CPI)
        pure Rust unit tests (math, transitions, errors)
```

Pure tests are fastest and should cover most state logic. Program tests prove framework constraints,
serialization, account creation, PDA signing, CPI, and transaction behavior.

## Minimum test matrix for each instruction

- happy path and exact final state;
- zero, one, maximum, maximum plus one;
- wrong signer, missing signer, wrong stored authority;
- wrong owner, wrong PDA, wrong program ID, wrong mint;
- duplicate account passed in multiple roles;
- invalid current state;
- repeated instruction and replay behavior;
- arithmetic overflow and underflow;
- maximum permitted collection/input size;
- CPI failure and state consistency;
- expected custom error code;
- compute use on realistic worst-case data.

## Debugging flow

1. Read the first compiler error, not the twentieth cascade error.
2. Identify whether it is ownership, type, trait, or lifetime related.
3. Reduce the expression and inspect inferred types.
4. For runtime failures, inspect program logs and the exact failed constraint/error.
5. Confirm cluster, program ID, account addresses, and deployed binary match.
6. Reproduce locally with the smallest deterministic test.

## Compute optimization

Optimize after correctness and measurement. Frequent costs include:

- unbounded/repeated account scans;
- unnecessary cloning and allocation;
- repeated serialization/deserialization;
- excessive logging;
- repeated PDA derivation;
- avoidable CPIs;
- large instruction/account data;
- algorithms whose cost grows badly at maximum input.

Prefer borrowed views, bounded inputs, cached safe aggregates, early cheap validation, and direct
iteration without allocation. Measure compute before and after each optimization.

---

# 13. Worked vault example

## State

```rust
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
    pub frozen: bool,
    pub bump: u8,
}
```

## Instruction: withdraw

Inputs: `amount: u64`

Accounts:

- `authority`: must sign;
- `vault`: mutable, correct PDA, stores the same authority;
- optional token/system accounts depending on what “balance” represents.

Preconditions:

```text
amount > 0
vault is not frozen
authority.key == vault.authority
vault PDA == derive(["vault", authority])
vault.balance >= amount
```

Transition:

```text
new_balance = old_balance - amount (checked)
```

Postconditions:

```text
new_balance <= old_balance
authority unchanged
PDA identity unchanged
all external transferred value matches internal accounting
```

## Pure Rust core

```rust
#[derive(Debug, PartialEq, Eq)]
enum VaultError { ZeroAmount, Frozen, InsufficientFunds }

fn withdraw(state: &mut VaultState, amount: u64) -> Result<(), VaultError> {
    if amount == 0 { return Err(VaultError::ZeroAmount); }
    if state.frozen { return Err(VaultError::Frozen); }
    state.balance = state.balance
        .checked_sub(amount)
        .ok_or(VaultError::InsufficientFunds)?;
    Ok(())
}
```

This pure transition is easy to exhaustively unit test. The Anchor handler should validate the account
boundary, call equivalent core logic, then perform external transfer/CPI consistently.

## Adversarial cases

- attacker supplies their own signer with someone else's vault;
- attacker supplies a look-alike account owned by another program;
- attacker supplies wrong PDA seeds or a different vault in a remaining account;
- amount is zero, `u64::MAX`, or one above balance;
- source and destination alias the same account;
- CPI program is replaced;
- CPI succeeds but internal balance update is inconsistent;
- withdraw is repeated from stale client state;
- frozen or closed vault is used.

---

# 14. Common compiler errors and translations

## “use of moved value”

Meaning: a non-`Copy` value was transferred to another owner.

Typical repair: pass `&T` if only reading, `&mut T` if editing, or deliberately return ownership.
Clone only when two independent owned values are required.

## “cannot borrow as mutable more than once”

Meaning: two overlapping exclusive borrows exist.

Typical repair: shorten the first borrow's scope, copy needed small fields, or split data safely.
For account collections, also consider whether duplicate account identities violate your logic.

## “cannot borrow as mutable because it is also borrowed as immutable”

Meaning: a reader remains alive while a writer is requested.

Typical repair: compute the read result first, let the reference go out of scope, then mutate.

## “mismatched types”

Meaning: actual and expected types differ (`String` vs `&str`, `u64` vs `usize`, `Option<T>` vs `T`).

Typical repair: read the full expected/found message and convert explicitly with borrowing, parsing,
`TryFrom`, `as_ref`, or pattern matching. Avoid unchecked `as` casts for financial values.

## “trait bound is not satisfied”

Meaning: a generic API requires behavior the type does not implement.

Typical repair: inspect the required trait, derive/implement it when semantically correct, or use the
correct wrapper type. In Anchor, this often points to account vs unchecked account or serialization traits.

## Lifetime error

Meaning: a returned/stored reference might outlive its source, or references need an explicit relationship.

Typical repair: return owned data when ownership is intended, add the correct lifetime relationship, or
avoid storing a short-lived borrow. Do not randomly add `'static`.

---

# 15. Recommended Rustlings expansion and study plan

The existing six exercises provide the minimum bridge:

1. mutable bindings;
2. ownership;
3. shared and mutable borrowing;
4. structs, enums, and `match`;
5. `Result`, custom errors, and `?`;
6. borrowed string slices.

Add these next:

| Exercise | Concepts | Anchor payoff |
|---|---|---|
| `ex07-collections` | arrays, `Vec`, maps, capacity | bounded account data and compute |
| `ex08-traits-generics` | traits, bounds, reusable validation | reading Anchor APIs and derives |
| `ex09-iterators` | `find`, `any`, `try_fold`, allocation | concise bounded account processing |
| `ex10-lifetimes` | returned borrows, multiple references | understanding `Context<'info>` |
| `ex11-checked-arithmetic` | overflow, fixed point, rounding | safe financial state transitions |
| `ex12-serialization-space` | byte encoding and max size | correct account allocation |
| `ex13-modules-tests` | crate design and adversarial tests | maintainable instruction modules |
| `ex14-security-invariants` | authority, owner, PDA, aliasing | protocol boundary defense |

## Four-week bridge plan

### Week 1: Rust memory model

- Finish exercises 1–6 without copying answers.
- Draw ownership moves and borrows for every function.
- Add one Unicode boundary test to the slices exercise.
- Write a pure Rust vault state machine.

### Week 2: Scalable Rust

- Practice collections, traits, iterators, lifetimes, and modules.
- Add checked fixed-point arithmetic with boundary tests.
- Manually encode/decode one small struct to bytes.
- Explain every compiler error before fixing it.

### Week 3: Solana and Anchor boundary

- Complete `anchor-core` account, PDA, CPI, constraint, lifetime, space, and error exercises.
- Calculate account space by hand.
- Build initialize/deposit/withdraw/freeze instructions on localnet.
- Test wrong owner, signer, PDA, mint, and program ID.

### Week 4: Protocol thinking

- Write invariants before code.
- Separate pure state logic from Anchor account plumbing.
- Add adversarial and replay tests.
- Measure worst-case compute.
- Document trust boundaries and remaining risks.

## Readiness checkpoints

You are ready for an educational Solana protocol when you can:

- explain ownership and borrowing without guessing;
- read an Anchor `Context` and explain every constraint;
- calculate account space correctly;
- derive and validate PDAs;
- perform and review a CPI;
- use checked arithmetic and explicit rounding;
- state protocol invariants in plain language;
- test hostile accounts and boundary values.

You are **not automatically ready to custody real funds** after completing a tutorial. Production financial
software requires deeper protocol design, threat modeling, extensive tests, operational controls, and
independent security review.

---

# One-page pre-commit checklist

## Rust

- [ ] No unnecessary `clone`, `unwrap`, `expect`, or unchecked numeric cast
- [ ] Borrow scopes are small and mutation is explicit
- [ ] Arithmetic is checked; units and rounding are documented
- [ ] Collections and loops have maximum bounds
- [ ] Errors identify the failed rule

## Accounts

- [ ] Correct address/PDA seeds and bump
- [ ] Correct account owner and discriminator/type
- [ ] Correct signer and stored authority relationship
- [ ] Only intended accounts are writable
- [ ] Duplicate/aliased accounts cannot violate assumptions

## Protocol

- [ ] Transition is legal from current state
- [ ] Replay and repeated invocation are safe or rejected
- [ ] CPI target and forwarded privileges are constrained
- [ ] Token mint, program, authority, and extensions are verified
- [ ] Final state preserves every documented invariant

## Tests

- [ ] Happy path and exact state
- [ ] Every authorization failure
- [ ] Wrong account/PDA/program/mint substitutions
- [ ] Zero, max, overflow, underflow, and maximum-size inputs
- [ ] CPI failure, replay, invalid phase, and compute ceiling

**Final habit:** Before writing a handler, write one sentence each for authority, inputs, state transition,
invariants, and failure cases. That small ritual prevents a surprising number of protocol bugs.
