# Interactive Week Template

## 1. Before you touch code

Answer in one or two sentences:

- What problem is this week trying to solve?
- Why is this hard?
- What would go wrong in Python or JavaScript that Rust is trying to prevent?
- What would go wrong in a distributed system if we modeled this badly?

## 2. Explain like I'm five

Write a child-level explanation:

- What is the concept?
- What is the thing being protected?
- What is the failure we are trying to avoid?

## 3. From scratch explanation

Now explain it more precisely:

- definition
- why it exists
- tradeoffs
- where it appears in Rust
- where it appears in Solana
- where it appears in distributed systems

## 4. Predict before running

Before each exercise:

- What do you think this function should return?
- What compiler error do you expect?
- Which values are owned?
- Which values are borrowed?
- Which state transition should happen?

## 5. Run, observe, and debug

- What actually happened?
- What error message appeared?
- What did the compiler or test output teach you?

## 6. Improvement questions

- How could this design be safer?
- How could it allocate less?
- How could it be easier to test?
- How could it fail in a networked environment?

## 7. Mini-project checkpoint

- What state exists?
- Who is allowed to mutate it?
- What invariants must always hold?
- What inputs are untrusted?
- What tests prove the system still works?
