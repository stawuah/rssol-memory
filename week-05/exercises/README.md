# Week 05 Exercises

Theme: lifetimes, memory layout, and smart pointers.

## Interactive exercise set

### ex01-borrowed-parser

Parse borrowed string slices without cloning the whole input.

### ex02-shared-registry

Use `Arc<Mutex<_>>` to manage a registry shared by worker threads.

### ex03-interior-mutability

Compare a `RefCell`-based design with a normal mutable-reference design.

### ex04-layout-awareness

Measure or reason about stack vs heap placement for your data model.

## Questions to answer

- What does a lifetime express in human language?
- When is cloning the wrong fix?
- Why is shared mutable state dangerous?
- Where will these ideas matter later in Solana?
