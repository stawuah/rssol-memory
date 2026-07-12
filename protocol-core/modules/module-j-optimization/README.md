# Module J — Optimization

This module progresses from a small honest model to production review questions. Every exercise requires a learner-owned adversarial test and a written explanation of its invariant.

## Map

| Exercise | Rust / Anchor skill | Solana / production application |
|---|---|---|
| ex45 | Layout-aware account design | Struct packing and zero-copy |
| ex46 | Benchmark interpretation | Compute-unit profiling |
| ex47 | Transaction planning | Batching, v0 transactions, and ALTs |
| ex48 | Account lifecycle and sizing | Rent, closing, and PDA reuse |

## Ordered exercises

1. [ex45 — Struct packing and zero-copy](./exercises/ex45-zero-copy-packing/README.md)
2. [ex46 — Compute-unit profiling](./exercises/ex46-compute-profiling/README.md)
3. [ex47 — Batching, v0 transactions, and ALTs](./exercises/ex47-batching-versioned-alt/README.md)
4. [ex48 — Rent, closing, and PDA reuse](./exercises/ex48-rent-close-pda-reuse/README.md)

## Coverage checklist

- [x] Struct packing and zero-copy
- [x] Compute-unit profiling
- [x] Batching, v0 transactions, and ALTs
- [x] Rent, closing, and PDA reuse

**Common interview question:** How do you lower compute, transaction size, and rent without weakening layout safety, lifecycle rules, or account validation?
