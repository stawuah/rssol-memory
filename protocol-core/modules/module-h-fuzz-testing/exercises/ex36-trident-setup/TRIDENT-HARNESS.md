# Trident Harness Requirements

Integrate the current Trident version compatible with the learner's Anchor/Solana
toolchain; pin versions and record installation/build commands.

The harness must initialize valid vault/mint/token/PDA state, generate weighted
deposit/donate/redeem/fee/close actions, maintain an independent accounting model,
sometimes generate deliberately invalid authority/owner/PDA/mint/alias cases, and
check after every step: asset conservation, share conservation, non-negative balances,
authority stability, no zero-share accepted deposits, and failure atomicity.

Record corpus/seed, action distribution, maximum sequence length, coverage evidence,
false-positive policy, compute/time budget, and exact reproduction command.
