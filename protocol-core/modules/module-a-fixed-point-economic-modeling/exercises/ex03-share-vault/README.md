# ex03 — Share-based vault accounting

## Why this exists

A yield vault pools assets and gives depositors **shares**, which represent a
fraction of the pool rather than a fixed asset amount. If yield increases vault
assets, each share becomes redeemable for more assets.

The **first-depositor donation attack** occurs when an attacker deposits a tiny
amount, receives the initial shares, then donates assets directly. A victim's
deposit may round to zero shares, effectively gifting value to the attacker.

This exercise uses virtual assets and virtual shares. These constants participate
in conversion math from genesis, reducing the attacker's ability to manipulate the
initial exchange rate. Real vault design must select and analyze these values.

## Production Solana application

Tokenized vaults, staking pools, lending deposit receipts, and liquidity positions
all use ratio-based share accounting.

## What this defends against

It defends against zero-share deposits, donation manipulation, inconsistent total
accounting, and over-redemption caused by wrong rounding.

## What wrong code looks like

```rust,ignore
let shares = if total_shares == 0 { assets } else {
    assets * total_shares / total_assets
};
// No minimum-share check; a manipulated rate can produce zero.
```

## How to code it

1. Include virtual assets and shares in every conversion.
2. Round deposit-to-shares down, then reject a zero result.
3. Update totals only after conversion succeeds.
4. Treat direct donations as assets without newly minted shares.
5. Convert shares to assets with withdrawal-favoring solvency rounding down.
6. Burn shares and subtract assets atomically.

## How to test it

Test multiple proportional depositors, yield donation, full redemption, zero input,
zero-share prevention, and a simulated donation attack.

## Explain before continuing

Explain how a donation changes assets-per-share without minting shares, and why a
victim receiving zero shares transfers economic value to existing shareholders.

## Required learner test

Add an attack test with values different from the provided scenario and assert that
a non-zero victim deposit either receives shares or is rejected without mutation.
