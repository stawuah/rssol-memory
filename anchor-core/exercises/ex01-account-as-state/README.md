# ex01 — Account as persistent state

## Rust idea

The struct groups related fields. The functions borrow it because they only read.

## Solana model

An account has an address, lamports, owner program, and byte data. The owner
program—not a user authority—is allowed to modify its data. The rent-like rule
here is simplified for learning.

## How to solve it

1. Compare the stored owner with `program_id`.
2. Return that boolean directly.
3. Treat `data_len > 0` as storing state.
4. Test matching and wrong owners plus zero and non-zero data lengths.

Real programs must validate owner, discriminator/type, and data length before
trusting deserialized account data.
