# Week 12

## Theme

Rust to Anchor to privacy-preserving Solana design.

## Goals

- write small Rust state machines without guessing
- read Anchor-style account validation with confidence
- understand what Token-2022 confidential transfers do and do not hide
- design the outline of a capstone protocol before coding it

## Topics

- Rust enums, `Result`, ownership, and deterministic state transitions
- Solana accounts, programs, instructions, transactions, PDAs, and CPI
- Anchor macros, account constraints, and program structure
- Token-2022 confidential transfer flow
- privacy boundaries for amount, timing, and intent
- proof-first design for MEV-resistant transaction flow

## How this week works

1. Read [interactive-guide.md](./interactive-guide.md).
2. Write your answers in `notes/`.
3. Complete exercises in order.
4. Do not jump to the mini-project before you can explain the failures in the exercises.
5. Use the mini-project as the bridge to your capstone.

## Exercises

- `ex01-state-machine`: model a program as a deterministic state machine
- `ex02-account-gatekeeper`: validate signer, owner, and writable assumptions
- `ex03-privacy-boundaries`: separate public data from private data

## Mini-project

- `confidential-route-sim`: a small protocol simulator for privacy-aware token routing

## Deliverables

- solved exercises
- completed mini-project
- one page of notes in your own words
- a retrospective explaining where your reasoning broke

## Capstone checkpoint

Before Monday, you should be able to answer:

- What is publicly visible in a confidential transfer?
- What still leaks through timing or transaction structure?
- Where does Anchor end and protocol design begin?
- What would a verifier need to check on chain?

