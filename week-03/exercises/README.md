# Week 03 Exercises

Theme: collections, modules, and error handling.

## Interactive exercise set

### ex01-message-deduplicator

Build a function that accepts message IDs and returns only the first occurrence of each ID while preserving order.

Checkpoints:

- what state is required to know whether a message is new?
- why is order preservation important?
- which collection is best here, and why?

### ex02-command-parser

Parse commands like `set user:1 online` and return typed results or typed errors.

Checkpoints:

- when should parsing return `Option` versus `Result`?
- what information belongs inside a good parse error?

### ex03-replicated-log

Append operations to a local log and reject duplicates by request ID.

Checkpoints:

- what makes a request duplicate?
- why is duplicate rejection connected to idempotency?

### ex04-error-classifier

Create a custom error enum for transport, storage, and validation failures.

Checkpoints:

- which failures should be retried?
- which failures should immediately stop processing?

## Completion rule

Do not move on until you can explain idempotency in plain English and with a code example.
