# Module H — Fuzz Testing

This module progresses from a small honest model to production review questions. Every exercise requires a learner-owned adversarial test and a written explanation of its invariant.

## Map

| Exercise | Rust / Anchor skill | Solana / production application |
|---|---|---|
| ex36 | Fuzz environment and account snapshots | Trident vault setup |
| ex37 | Stateful model-based testing | Meaningful fuzz harnesses |
| ex38 | Failure reduction and deterministic tests | Minimize and regress a fuzz bug |

## Ordered exercises

1. [ex36 — Trident vault setup](./exercises/ex36-trident-setup/README.md)
2. [ex37 — Meaningful fuzz harnesses](./exercises/ex37-meaningful-fuzzing/README.md)
3. [ex38 — Minimize and regress a fuzz bug](./exercises/ex38-minimize-regression/README.md)

## Coverage checklist

- [x] Trident vault setup
- [x] Meaningful fuzz harnesses
- [x] Minimize and regress a fuzz bug

**Common interview question:** What makes a stateful Solana fuzz harness meaningful, and how do you convert a random failure into a permanent regression test?
