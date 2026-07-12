# Week 05 Interactive Guide

## Big idea

This week is about memory layout and shared ownership.

## Explain like I'm five

Some toys are passed around by hand. Some are locked in a cabinet. Some are copied. Some are shared with rules.

## Distributed systems from scratch

Shared mutable state is dangerous both inside one process and across many machines.
Rust's local rules train you to think carefully before you mutate shared data.

## Checkpoints

- When do you need `Box`, `Rc`, `Arc`, `RefCell`, or `Mutex`?
- Why is shared mutable state risky?
- What does a lifetime represent in plain language?
- Where would zero-copy matter in systems or Solana?
