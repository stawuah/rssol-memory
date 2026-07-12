# Week 04 Exercises

Theme: traits, generics, iterators, and testing.

## Interactive exercise set

### ex01-storage-trait

Define a trait for storing commands and implement an in-memory backend.

### ex02-generic-command-apply

Write a generic function that applies a sequence of commands to any store implementing your trait.

### ex03-event-filter-iterator

Use iterators to filter leader-election events or failed requests.

### ex04-state-machine-tests

Write tests proving invalid state transitions are rejected or ignored.

## Questions to answer

- What does the trait abstract over?
- What stays concrete even with generics?
- Why are mocks useful in system tests?
- When is iterator style clearer than manual loops?
