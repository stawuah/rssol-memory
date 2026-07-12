# 00 Setup

## Analogy

Setting up Anchor is like setting up the workshop before building a machine.

- Rust is the tool kit
- Solana CLI is the local test bench
- Anchor CLI is the assembly jig
- `anchor init` gives you a standard room layout

## Official starting point

Anchor docs say to start with installation and quickstart, and that `anchor init <project-name>` creates a new modular Rust program structure. The installation page also notes that `anchor build` builds the program and `anchor test` builds, deploys, and runs tests. See:

- [Anchor Installation](https://www.anchor-lang.com/docs/installation)

## What to remember

- You need Rust
- You need the Solana CLI
- You need Anchor CLI
- A local validator is part of the normal loop

## Exercise

Write the exact command sequence you would use to:

1. initialize a new project
2. build it
3. test it locally

## Hint

Start from `anchor init`, then `anchor build`, then `anchor test`.

