# Week 12 Mini-Project

Project: `confidential-route-sim`

## Build target

Build a small Rust simulator for a privacy-aware transfer flow.

## What it should model

- a request is created
- a proof is attached
- settlement happens only after validation
- the observer view reveals less than the internal state

## Why this matters

This is the bridge between “I understand the concept” and “I can design the protocol.”

## Constraints

- keep the flow deterministic
- do not use `unwrap` in the library
- make the privacy leak explicit in the types

## Hints

- Start by modeling the smallest unit of information you can safely reveal.
- Put every irreversible action behind a proof check.
- Ask what a chain observer can infer from the order of events, even when the amount is hidden.
- If your design only hides amount, say so directly.

