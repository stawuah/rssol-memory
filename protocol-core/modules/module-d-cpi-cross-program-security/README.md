# Module D — CPI and Cross-Program Security

This module progresses from a small honest model to production review questions. Every exercise requires a learner-owned adversarial test and a written explanation of its invariant.

## Map

| Exercise | Rust / Anchor skill | Solana / production application |
|---|---|---|
| ex19 | Account metadata and signer-seed construction | invoke and invoke_signed |
| ex20 | Executable identity validation | Missing program-ID exploit |
| ex21 | Privilege tracking across call graphs | Multi-hop CPI privileges |
| ex22 | Checks-effects-interactions around CPI | Account locking and reentrancy analogues |
| ex23 | Length, owner, discriminator, and value checks | CPI return-data validation |
| ex24 | Account identity and ownership constraints | Fake account substitution exploit |

## Ordered exercises

1. [ex19 — invoke and invoke_signed](./exercises/ex19-invoke-signed/README.md)
2. [ex20 — Missing program-ID exploit](./exercises/ex20-program-id-substitution/README.md)
3. [ex21 — Multi-hop CPI privileges](./exercises/ex21-multihop-privileges/README.md)
4. [ex22 — Account locking and reentrancy analogues](./exercises/ex22-account-locking/README.md)
5. [ex23 — CPI return-data validation](./exercises/ex23-cpi-return-data/README.md)
6. [ex24 — Fake account substitution exploit](./exercises/ex24-fake-account-substitution/README.md)

## Coverage checklist

- [x] invoke and invoke_signed
- [x] Missing program-ID exploit
- [x] Multi-hop CPI privileges
- [x] Account locking and reentrancy analogues
- [x] CPI return-data validation
- [x] Fake account substitution exploit

**Common interview question:** What privileges cross a CPI boundary, and how do you prevent program, account, signer, and return-data substitution?
