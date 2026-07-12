# Module E — Upgrade Authority and Governance

This module progresses from a small honest model to production review questions. Every exercise requires a learner-owned adversarial test and a written explanation of its invariant.

## Map

| Exercise | Rust / Anchor skill | Solana / production application |
|---|---|---|
| ex25 | Authority-state modeling | Program upgrade authority |
| ex26 | Distinct signer quorum and proposal binding | Multisig-gated administration |
| ex27 | Slot windows and immutable queued payloads | Governance timelock |
| ex28 | Irreversible state transition analysis | Immutability tradeoff |

## Ordered exercises

1. [ex25 — Program upgrade authority](./exercises/ex25-upgrade-authority/README.md)
2. [ex26 — Multisig-gated administration](./exercises/ex26-multisig-admin/README.md)
3. [ex27 — Governance timelock](./exercises/ex27-timelock/README.md)
4. [ex28 — Immutability tradeoff](./exercises/ex28-immutability-tradeoff/README.md)

## Coverage checklist

- [x] Program upgrade authority
- [x] Multisig-gated administration
- [x] Governance timelock
- [x] Immutability tradeoff

**Common interview question:** What trust assumptions remain while a program is upgradeable, and how do multisigs and timelocks constrain that power?
