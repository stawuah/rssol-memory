# ex05 — Anchor-style account validation

## Rust idea

A borrowed slice can be checked without taking ownership. A custom enum reports
which validation rule failed.

## Solana model

Anchor constraints validate signer, writable status, owner, exact address, PDA,
and relationships. Each check proves a different fact.

## How to solve it

1. Safely obtain the first account.
2. Require it to sign and be writable.
3. Iterate through every account and validate its owner.
4. Return the first precise error or success.
5. Add empty-list, read-only, and later-wrong-owner tests.

A signer is not automatically the protocol's authorized signer; relate its key
to trusted state.
