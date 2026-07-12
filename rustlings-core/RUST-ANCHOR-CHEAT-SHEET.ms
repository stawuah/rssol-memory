.TL
Rust + Anchor Core Cheat Sheet
.AU
A practical bridge from beginner Rust to Solana programs
.AI
Keep this beside rustlings-core and anchor-core
.PP
This guide explains the concepts you will repeatedly meet while writing Anchor
programs. Read it as a map, not something to memorize. The central idea is that
a Solana program is a deterministic Rust state machine operating on accounts
supplied by a caller. Correctness comes from validating those accounts before
changing state.
.
.NH 1
The five questions to ask for every instruction
.IP 1. 4
What state may change?
.IP 2. 4
Who is authorized to request the change?
.IP 3. 4
Does every account have the expected address, owner, type, and permissions?
.IP 4. 4
Which invariant must remain true after the change?
.IP 5. 4
Can arithmetic, allocation, a CPI, or compute usage fail?
.
.NH 1
Rust values, variables, and mutability
.PP
"let" binds a name to a value. Values are immutable unless the binding uses
"mut". Immutability makes state changes visible and easier to audit.
.DS L
let balance = 10_u64;
let mut counter = 0_u64;
counter += 1;
.DE
.PP
Anchor mapping: an account must be marked mutable before its data or lamports
can change. Rust "mut" controls a local binding; Anchor's "mut" account
constraint tells the runtime and transaction that the account will be written.
They are related ideas but are not interchangeable.
.
.NH 1
Scalar types and arithmetic
.PP
Use explicit integer types for stored protocol state. Unsigned integers (u8,
u16, u32, u64, u128) cannot represent negative values. Their size controls both
range and account space. Never rely on release-mode wrapping for financial math.
.DS L
let next = balance.checked_add(deposit)
    .ok_or(ProtocolError::Overflow)?;
let remaining = balance.checked_sub(withdrawal)
    .ok_or(ProtocolError::InsufficientFunds)?;
.DE
.PP
For ratios and prices, define a fixed-point scale and rounding direction. Avoid
floating point in protocol state. Rounding is economic policy, not a cosmetic
implementation detail.
.
.NH 1
Ownership: who must clean up a value?
.PP
Every Rust value has one owner. Moving a non-Copy value transfers ownership.
When the owner leaves scope, Rust drops the value. Small scalar types such as
u64 usually implement Copy, so assignment copies their bits.
.DS L
let name = String::from("vault");
let moved = name;             // name can no longer be used
let amount = 10_u64;
let copied = amount;          // both remain usable
.DE
.PP
Rust ownership is different from Solana account ownership. Rust ownership is a
compile-time memory rule. A Solana account's owner field names the program
allowed to modify its data. Always check both concepts in their proper layer.
.
.NH 1
Borrowing and references
.PP
"&T" is a shared, read-only borrow. "&mut T" is an exclusive mutable borrow.
Rust permits many readers or one writer at a time. A borrow does not take
ownership and normally avoids a copy.
.DS L
fn balance(vault: &Vault) -> u64 { vault.balance }
fn freeze(vault: &mut Vault) { vault.frozen = true; }
.DE
.PP
Anchor's Context contains references to validated accounts. Instruction
handlers usually borrow account data, update it, then return. Exclusive borrows
help prevent two pieces of Rust code from mutating the same value at once, but
they do not replace on-chain account validation.
.
.NH 1
Strings and slices
.PP
String owns growable UTF-8 bytes. &str borrows UTF-8 text. [T] is a sequence of
items, and &[T] borrows such a sequence. Slices are views, so they avoid new
allocations.
.DS L
fn prefix(text: &str, delimiter: char) -> &str {
    &text[..text.find(delimiter).unwrap_or(text.len())]
}
.DE
.PP
String indexing uses byte offsets and may panic if an offset splits a UTF-8
character. For protocol inputs, limit maximum lengths before doing expensive
work. Anchor/Borsh stores a String as a 4-byte length followed by UTF-8 bytes;
allocate for the maximum encoded length, not today's value.
.
.NH 1
Structs, impl blocks, and state
.PP
A struct groups fields into one type. An impl block defines associated functions
and methods. A method with &mut self changes the value.
.DS L
struct Vault { authority: Pubkey, balance: u64, frozen: bool }
impl Vault {
    fn can_withdraw(&self, signer: Pubkey) -> bool {
        !self.frozen && self.authority == signer
    }
}
.DE
.PP
Anchor mapping: a struct marked as an account describes serialized on-chain
state. Keep stored state minimal, version it deliberately, and document each
field's invariant.
.
.NH 1
Enums and exhaustive match
.PP
An enum represents one of several valid alternatives. Variants may carry data.
"match" forces all cases to be handled, which is ideal for state machines.
.DS L
enum Command { Deposit(u64), Withdraw(u64), Freeze }
match command {
    Command::Deposit(n) => deposit(n),
    Command::Withdraw(n) => withdraw(n),
    Command::Freeze => freeze(),
}
.DE
.PP
Protocol states should make invalid transitions explicit. A frozen vault should
not silently behave like an active vault. Prefer a clear enum or explicit flag
plus validation over magic numbers.
.
.NH 1
Option, Result, and the question-mark operator
.PP
Option<T> means a value may be present or absent. Result<T, E> means an operation
either succeeds with T or fails with E. The ? operator returns an error early
while preserving the success value.
.DS L
fn debit(balance: u64, amount: u64) -> Result<u64, Error> {
    balance.checked_sub(amount).ok_or(Error::InsufficientFunds)
}
.DE
.PP
Do not use unwrap or expect on untrusted instruction or account data. A panic
produces a poor failure boundary. Return a specific program error that clients
and tests can understand.
.
.NH 1
Traits and generics
.PP
A trait defines behavior that multiple types can implement. A generic function
works with types satisfying stated trait bounds.
.DS L
trait Validate { fn validate(&self) -> Result<(), Error>; }
fn checked<T: Validate>(value: &T) -> Result<(), Error> {
    value.validate()
}
.DE
.PP
Anchor uses traits heavily for serialization, deserialization, account loading,
and instruction construction. You need to read trait bounds confidently; you do
not need advanced trait wizardry to begin writing programs.
.
.NH 1
Collections and bounded work
.PP
Vec<T> is a growable sequence. Arrays [T; N] have a compile-time length.
HashMap is useful off-chain but deterministic ordered representations are often
easier to reason about in protocol logic. Every on-chain collection needs an
explicit maximum size or a separate account-growth strategy.
.PP
Unbounded input creates three risks: insufficient account space, excessive
compute, and denial of service. Validate length before loops, allocation, hashing,
or CPIs.
.
.NH 1
Iterators and closures
.PP
Iterators process sequences through methods such as map, filter, find, any, and
fold. They are usually zero-cost abstractions, but collect allocates a new
collection.
.DS L
let total = positions.iter()
    .try_fold(0_u64, |sum, p| sum.checked_add(p.amount))
    .ok_or(Error::Overflow)?;
.DE
.PP
On-chain, a concise iterator is not automatically cheaper than a loop. Measure
compute units for hot paths and avoid repeated scans when state can maintain a
safe aggregate.
.
.NH 1
Lifetimes
.PP
A lifetime describes how long a reference remains valid. It does not extend the
life of data; it records a relationship the compiler must enforce.
.DS L
fn choose<'a>(left: &'a str, right: &'a str, first: bool) -> &'a str {
    if first { left } else { right }
}
.DE
.PP
In Anchor, 'info connects account references inside Context and account wrapper
types. Read it as "these references are valid for the instruction's account
context." Start by following compiler guidance; explicit lifetime complexity is
usually needed only in helpers involving several account references.
.
.NH 1
Modules, visibility, and crates
.PP
A crate is a compilation unit. Modules organize code. Items are private unless
made pub. Keep instruction handlers small and place pure state-transition logic
in testable modules. Expose only the API other modules truly need.
.PP
Common program layout: instructions, state, errors, constants, events, and
utility validation. Structure is useful only when it makes security boundaries
easier to see.
.
.NH 1
Solana account mental model
.PP
An account has an address, lamports, an owner program, executable status, and a
byte array. Programs are stateless code; persistent application state lives in
accounts. A transaction names all accounts an instruction may read or write.
.IP Address 12
Identity of the account. Validate expected fixed addresses or derivations.
.IP Owner 12
Program allowed to modify account data. This is not the user authority.
.IP Authority 12
A protocol-defined key allowed to perform an action, often required to sign.
.IP Signer 12
Evidence that the transaction was authorized by a private key.
.IP Writable 12
Permission declared by the transaction for state mutation.
.PP
These properties answer different questions. Checking only one is rarely enough.
.
.NH 1
Anchor Context and constraints
.PP
Context groups the accounts for an instruction. The Accounts-derived struct and
its constraints form the first validation wall. Useful constraints include
signer, mut, owner, address, has_one, seeds, bump, constraint, init, close, and
token-specific checks.
.PP
Constraints reduce boilerplate but do not invent your business invariants. You
must still validate amounts, state transitions, timing, limits, relationships,
and any assumptions about accounts used by a CPI.
.
.NH 1
PDAs
.PP
A Program Derived Address is deterministically calculated from seeds, a program
ID, and a bump. It has no private key. The deriving program can sign for it
during a CPI by supplying the correct seeds.
.PP
Good seeds create one clear namespace and bind state to the correct domain, for
example ["position", market, user]. Validate seeds and bump on every instruction
that trusts the PDA. Seed design is part of the protocol's authorization model.
Never treat knowing a PDA address as proof of user authorization.
.
.NH 1
Cross-program invocations
.PP
A CPI asks another program to execute an instruction. Treat the target program
and every forwarded account as a trust boundary. Verify the program ID, provide
only required signer privileges, validate accounts before and after when needed,
and understand which state the callee may change.
.PP
A transaction is atomic: if a CPI fails and the error is propagated, the entire
transaction rolls back. Atomicity does not make a logically incorrect CPI safe.
.
.NH 1
Serialization and account space
.PP
Anchor account data normally starts with an 8-byte discriminator. Fixed-size
integers occupy their byte width. A bool is 1 byte. Pubkey is 32 bytes. String
and Vec include a 4-byte length prefix plus maximum contents. Option adds a tag
plus its possible value. Enums require a variant tag plus enough room for their
largest variant.
.DS L
space = 8                    // discriminator
      + 32                   // authority Pubkey
      + 8                    // balance u64
      + 1                    // frozen bool
      + 4 + MAX_NAME_BYTES;  // String
.DE
.PP
Do not calculate dynamic space from only the initial input if future updates may
grow the value. Reallocation and rent funding need explicit design.
.
.NH 1
Transactions, instructions, and atomicity
.PP
An instruction contains a program ID, an account metadata list, and instruction
data. A transaction contains one or more instructions plus signatures and a
recent blockhash. All instructions succeed or the transaction fails atomically.
.PP
Transactions can be retried by clients. Design actions to reject duplicate or
stale operations when repetition would be harmful. Use sequence numbers,
receipts, unique PDAs, or explicit state transitions where appropriate.
.
.NH 1
Compute units and performance
.PP
Solana execution is metered. Expensive patterns include unbounded loops, repeated
serialization, large allocations, excessive logging, repeated PDA derivation,
and unnecessary CPIs. Optimize after correctness, then measure.
.PP
Fast Rust habits that also improve clarity: borrow instead of clone, return
slices where possible, validate cheap conditions before expensive work, avoid
collect when streaming is enough, and store safe aggregates rather than rescanning
large collections.
.
.NH 1
Testing strategy
.IP 1. 4
Test the happy path and exact state changes.
.IP 2. 4
Test each authorization failure: wrong signer, missing signer, wrong authority.
.IP 3. 4
Test account substitution: wrong owner, wrong PDA seeds, wrong program ID.
.IP 4. 4
Test boundaries: zero, one, maximum, maximum plus one, overflow, underflow.
.IP 5. 4
Test invalid state transitions and repeat the same instruction.
.IP 6. 4
Test CPI failure and confirm state remains consistent.
.IP 7. 4
Measure compute on realistic maximum-size inputs.
.PP
Prefer pure Rust unit tests for state math and validation. Add program-level tests
for account constraints, serialization, PDAs, CPIs, and transaction behavior.
.
.NH 1
Security checklist before every state change
.IP Authorization 16
Did the correct authority sign, and is that authority stored or derived correctly?
.IP Identity 16
Are fixed addresses, PDA seeds, bump, and account relationships verified?
.IP Ownership 16
Is each data account owned by the expected program?
.IP Type 16
Is data length and discriminator correct before deserialization?
.IP Mutability 16
Are only intended accounts writable, and can aliases/duplicates cause trouble?
.IP Arithmetic 16
Are checked operations, units, precision, and rounding direction explicit?
.IP State 16
Is this transition legal from the current state, including repeated calls?
.IP CPI 16
Is the target program trusted and are forwarded privileges minimal?
.IP Resources 16
Are input length, account growth, compute, and fees bounded?
.IP Closing 16
Does closing send lamports to the intended recipient and prevent account revival?
.
.NH 1
Common confusions
.IP "Rust owner" 16
The variable responsible for a value's memory.
.IP "Account owner" 16
The program ID allowed to modify an account's data.
.IP "Authority" 16
A key your protocol recognizes as permitted to perform an action.
.IP "Signer" 16
A transaction participant that proved control of a private key.
.IP "mut in Rust" 16
Allows mutation through a binding or reference.
.IP "mut in Anchor" 16
Declares an account writable for the transaction.
.IP "PDA signer" 16
A program signs through runtime-verified seeds during a CPI; no secret key exists.
.
.NH 1
Recommended Rustlings Core expansion
.PP
The existing six exercises cover mutability, ownership, borrowing, state models,
Result, and slices. Add these exercises next:
.IP ex07 8
Collections: Vec, arrays, maps, maximum lengths, and avoiding needless clones.
.IP ex08 8
Traits and generics: reusable validation with simple trait bounds.
.IP ex09 8
Iterators: find, any, try_fold, and allocation awareness.
.IP ex10 8
Lifetimes: returned borrows and Anchor-style Context relationships.
.IP ex11 8
Checked arithmetic: overflow, underflow, fixed-point scale, and rounding.
.IP ex12 8
Serialization and space: manual byte layout and maximum account size.
.IP ex13 8
Modules and tests: library boundaries, unit tests, and adversarial cases.
.IP ex14 8
Security invariants: signer, authority, owner, PDA, duplicates, and legal transitions.
.
.NH 1
Learning loop
.IP 1. 4
Read the test before the implementation.
.IP 2. 4
Predict ownership, state, and expected error on paper.
.IP 3. 4
Write the smallest correct implementation.
.IP 4. 4
Explain why each validation exists and what attack it blocks.
.IP 5. 4
Add one failure test the exercise did not provide.
.IP 6. 4
Translate the Rust idea into its Anchor/account equivalent.
.PP
You are ready to build educational protocols when you can implement a vault or
escrow locally, explain every account constraint, test all failure paths, and
state the protocol invariants in plain language. Real funds require substantially
more review, adversarial testing, and independent security assessment.
