# Module C — MEV and Ordering

This module progresses from a small honest model to production review questions. Every exercise requires a learner-owned adversarial test and a written explanation of its invariant.

## Map

| Exercise | Rust / Anchor skill | Solana / production application |
|---|---|---|
| ex14 | AMM reserve math and minimum-out enforcement | Sandwich attack simulation |
| ex15 | Hash preimages and replay-safe state | Commit-reveal ordering |
| ex16 | Ciphertext-state validation | Token-2022 confidential transfers |
| ex17 | Commitment-bound encrypted order state | Encrypted trigger orders |
| ex18 | Idempotency and bounded rewards | Permissionless keeper network |

## Ordered exercises

1. [ex14 — Sandwich attack simulation](./exercises/ex14-sandwich-attack/README.md)
2. [ex15 — Commit-reveal ordering](./exercises/ex15-commit-reveal/README.md)
3. [ex16 — Token-2022 confidential transfers](./exercises/ex16-confidential-transfers/README.md)
4. [ex17 — Encrypted trigger orders](./exercises/ex17-encrypted-trigger-orders/README.md)
5. [ex18 — Permissionless keeper network](./exercises/ex18-keeper-network/README.md)

## Coverage checklist

- [x] Sandwich attack simulation
- [x] Commit-reveal ordering
- [x] Token-2022 confidential transfers
- [x] Encrypted trigger orders
- [x] Permissionless keeper network

**Common interview question:** How can transaction ordering extract value, and which designs reduce information leakage or make execution order less exploitable?
