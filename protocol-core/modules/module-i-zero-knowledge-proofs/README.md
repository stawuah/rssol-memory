# Module I — Zero-Knowledge Proofs

This module progresses from a small honest model to production review questions. Every exercise requires a learner-owned adversarial test and a written explanation of its invariant.

## Map

| Exercise | Rust / Anchor skill | Solana / production application |
|---|---|---|
| ex39 | Statement/witness separation | Proof-system intuition |
| ex40 | Engineering tradeoff evaluation | Groth16 versus PLONK |
| ex41 | Merkle root and nullifier modeling | Light-style ZK compression |
| ex42 | CPI validation and compute budgeting | On-chain verifier compute |
| ex43 | Hidden-value invariant reasoning | Range proofs and confidential tokens |
| ex44 | Architecture decision writing | When ZK is the wrong tool |

## Ordered exercises

1. [ex39 — Proof-system intuition](./exercises/ex39-proof-system-intuition/README.md)
2. [ex40 — Groth16 versus PLONK](./exercises/ex40-groth16-vs-plonk/README.md)
3. [ex41 — Light-style ZK compression](./exercises/ex41-zk-compression/README.md)
4. [ex42 — On-chain verifier compute](./exercises/ex42-verifier-compute/README.md)
5. [ex43 — Range proofs and confidential tokens](./exercises/ex43-range-proofs/README.md)
6. [ex44 — When ZK is the wrong tool](./exercises/ex44-when-not-zk/README.md)

## Coverage checklist

- [x] Proof-system intuition
- [x] Groth16 versus PLONK
- [x] Light-style ZK compression
- [x] On-chain verifier compute
- [x] Range proofs and confidential tokens
- [x] When ZK is the wrong tool

**Common interview question:** What does a proof establish, what remains public, and when is ZK worth its proving, verification, and maintenance cost?
