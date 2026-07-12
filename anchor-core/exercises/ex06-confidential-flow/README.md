# ex06 — Confidentiality and metadata leakage

## Rust idea

Build a report by classifying boolean properties into vectors of visible, private,
and missing-control labels.

## Solana model

Hiding amounts does not hide timing, intent, routing, participants, or failures.
A proof must also be verified. Privacy properties must be named separately.

## How to solve it

1. Create three empty report vectors.
2. Classify amount, timing, and intent as private or visible.
3. Add batching when timing leaks.
4. Add an appropriate control when intent leaks.
5. Add proof when verification is missing.
6. Return the full report and test all-hidden/all-visible combinations.
