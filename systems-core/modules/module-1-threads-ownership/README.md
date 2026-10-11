# Module 1 — Threads and Ownership

Level: **Junior**. Rust's thread APIs encode the rules of safe concurrency in
their signatures (`Send`, `'static`, `FnOnce`). This module teaches you to read
those signatures and work with them instead of fighting them with `Arc` and
`clone` everywhere.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex01 | `thread::scope`, borrowing across threads, checked combine | parallel hashing and verification over a slice |
| ex02 | `thread::spawn`, `'static`, `Send`, `join`, panic payloads | long-lived named service threads that must not die silently |

## Ordered exercises

1. [Scoped parallel sum](./exercises/ex01-scoped-parallel-sum/README.md)
2. [Join and panics](./exercises/ex02-join-and-panics/README.md)

## Coverage checklist

- [x] balanced chunking, scoped threads, borrow without `Arc`
- [x] overflow-safe combination of partial results
- [x] spawn-all-then-join, thread naming, panic capture

**Common interview question:** Why does `thread::spawn` require `'static` and
`Send`, and how does `thread::scope` get away without `'static`?
