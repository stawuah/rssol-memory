# Week 03 Mini-Project

Project: `replicated-log-sim`

## Build target

Create a small Rust library that:

- accepts client operations
- assigns sequence numbers
- stores them in order
- rejects duplicate request IDs
- exposes a replay function

## Interactive prompts

- What is the log state?
- Which invariant must always hold?
- How will you prove replay preserves order?
- What bug appears if duplicates are appended twice?

## Stretch goals

- persist the log to a file
- add a snapshot function
- add a replay test from serialized data
