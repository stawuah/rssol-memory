# Week 05

## Theme

Lifetimes, memory layout, and smart pointers.

## Goals

- Understand why Rust rejects certain code
- Reduce unnecessary cloning
- Reason about shared state and ownership boundaries

## Rust topics

- lifetimes
- stack vs heap
- `Box`, `Rc`, `Arc`
- `RefCell`, `Mutex`

## Distributed systems topics

- shared state tradeoffs
- in-memory indexes
- mutability control

## Exercises

- remove clones from a parser
- use `Arc<Mutex<_>>` correctly
- fix shared-state borrow problems
- compare borrowed vs copied data paths

## Mini-project

- `membership-registry`

## Deliverables

- notes in `notes/`
- solved exercises in `exercises/`
- shared-state registry in `mini-project/`
- completed `interactive-guide.md` checkpoints
- completed `retrospective.md`
