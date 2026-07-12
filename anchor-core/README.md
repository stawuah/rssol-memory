# Anchor Core

This folder teaches Solana and Anchor by analogy first, then code.

## Mental model

- A Solana account is a locker with a public address.
- A program is the machine that can change lockers.
- An instruction is a request to run one machine action.
- A transaction is a bundle of instructions plus the account list they may touch.
- A PDA is a locker with no private key. The program can act for it through runtime rules.
- CPI is one program asking another program to do work.
- Anchor is the safety harness that reduces boilerplate and forces account checks early.

## How to study

1. Read the analogy for the exercise.
2. Predict what the state should do.
3. Fill in the `todo!()`s.
4. Run `cargo test --workspace`.
5. Write one sentence explaining what the public observer can still see.

## Read this next

- [lessons/README.md](./lessons/README.md)

## Study loop

- Read one exercise in full before editing.
- Say the analogy out loud.
- Predict the state transition.
- Implement the smallest correct answer.
- Use the test failure to locate your misunderstanding.

## Exercise order

- `ex01-account-as-state`: model Solana account state like a struct
- `ex02-program-vs-account`: separate code from state
- `ex03-pda-determinism`: derive deterministic addresses
- `ex04-cpi-router`: forward work to another program
- `ex05-anchor-checks`: validate signer, owner, and mutability
- `ex06-confidential-flow`: model what confidential transfers hide and leak

## Capstone warning

Confidential transfers can hide amounts.
They do not automatically hide:

- transaction timing
- transaction intent
- routing structure
- failed attempt patterns
