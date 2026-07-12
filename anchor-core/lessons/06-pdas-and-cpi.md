# 06 PDAs and CPI

## Analogy

- PDA = locker derived from a recipe
- CPI = one machine calling another machine

## Official facts

PDAs are deterministic addresses derived from a program ID and seeds, and they have no private key.
CPI is one program invoking an instruction on another program.

- [Solana PDAs](https://solana.com/docs/core/pda)
- [Solana CPI](https://solana.com/docs/core/cpi)
- [Anchor PDA Constraints](https://www.anchor-lang.com/docs/basics/pda)
- [Anchor CPI](https://www.anchor-lang.com/docs/basics/cpi)

## Exercise

Explain why each of these is true:

1. same seeds + same program ID = same PDA
2. a PDA has no private key
3. one program can call another program through CPI

## Hint

The runtime is the one doing the signing for a PDA, not a user wallet.

