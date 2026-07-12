# Ten Protocol Core Capstones

Every project is complete only after: devnet deployment; a written invariant and
trust-boundary document; a Trident stateful fuzz suite; deterministic regression
tests for discovered bugs; deployment manifest and runbook; and a short security
review. Never treat these educational projects as audited mainnet software.

## 1. Overcollateralized stablecoin protocol

Build collateral deposits, oracle-valued mint/burn, withdrawal, liquidation, global
supply accounting, pause controls, and peg-monitoring signals. Primary modules: A,
B, D, E, F, G, H. Invariants: supply equals aggregate debt; debt never exceeds
allowed backing after successful user actions; only canonical collateral/mint/token
programs are accepted; liquidations require unhealthy fresh-priced positions.
Minimum adversarial tests: stale/wide oracle, price crash, exact ratio boundary,
wrong mint/program/PDA, overflow, duplicate accounts, liquidation cap, unauthorized
pause, CPI failure, replay, and global-supply divergence.

## 2. ERC4626-style yield vault

Build multi-depositor share minting/redemption, yield donation, fee accrual, strategy
CPI, and emergency withdrawal. Primary modules: A, D, E, G, H, J. Invariants:
shares are proportional claims; no non-zero accepted deposit mints zero shares;
assets equal idle plus verified strategy assets; fees cannot overdraw; redemption
never exceeds entitlement. Tests: first-depositor donation, rounding splits, tiny
deposit, huge donation, strategy substitution, CPI failure, fee overflow, full exit,
concurrent account aliases, close/reopen, and fuzzed conservation.

## 3. MEV-resistant encrypted trigger-order executor

Build committed/encrypted take-profit and stop-loss orders, authenticated reveal,
oracle eligibility, permissionless keepers, cancellation, expiry, and bounded swap
execution. Primary modules: B, C, D, F, G, H, I. Invariants: plaintext is not stored
before reveal; reveal matches owner/market/order commitment; execution happens once
inside its window; keeper reward follows useful execution; swap honors minimum out.
Tests: copied reveal, wrong nonce/domain, front-run attempt, stale oracle, invalid
proof, early/late execution, keeper spam, reward replay, fake swap program, and
sandwich-induced minimum-out failure.

## 4. Simplified perpetual futures market

Build collateralized long/short positions, oracle mark price, funding-rate accrual,
PnL, margin, health, and liquidation. Primary modules: A, B, C, E, G, H. Invariants:
position accounting is zero-sum except explicit fees; funding conserves value;
unhealthy positions are liquidatable while healthy positions are not; oracle inputs
are fresh and bounded. Tests: price/confidence boundaries, funding overflow/time
reversal, stale mark, manipulation spike, partial liquidation, rounding leakage,
close/reopen, keeper ordering, and fuzzed aggregate collateral conservation.

## 5. Undercollateralized lending pool

Build deposits, shares, utilization curve, borrower limits, interest index, repayment,
reserves, delinquency, and explicit bad-debt socialization policy. Primary modules:
A, B, E, G, H, J. Invariants: cash plus receivables equals liabilities plus reserves
under the documented loss model; utilization stays bounded; indexes never decrease;
borrow limits require authorized underwriting. Tests: 100% utilization, zero supply,
kink boundaries, missed payments, rate overflow, donation, bank-run withdrawals,
bad-debt allocation, unauthorized borrower, stale oracle, and long fuzz sequences.

## 6. Multisig treasury with timelocked governance

Build proposals, distinct member approvals, threshold changes, queued execution,
cancellation, treasury CPI, and upgrade-authority handoff. Primary modules: D, E, F,
G, H. Invariants: exact payload needs threshold distinct current members; execution
occurs once after delay; treasury CPI targets canonical programs; authority transfer
matches governance state. Tests: duplicate approvals, removed member, payload swap,
early/expired execution, replay, malicious CPI program, signer escalation, authority
renounce irreversibility, concurrent proposal IDs, and fuzzed membership transitions.

## 7. ZK-compressed NFT or token registry

Build Merkle-committed ownership, compressed mint/transfer/burn, root history,
validity-proof verification, and nullifier/replay protection. Primary modules: D,
F, G, H, I, J. Invariants: only accepted roots authorize transitions; each leaf is
consumed once; new root represents the exact authorized update; verifier/key/program
identity is fixed. Tests: bad path, old root, reused nullifier, wrong owner/public
input, fake verifier, malformed return data, proof compute exhaustion, concurrent
root update, close/reuse, and model-based state-root fuzzing.

## 8. Cross-program token escrow

Build deposit, acceptance, release, cancellation/timeout, PDA custody, and real token
program CPIs including multi-hop settlement. Primary modules: C, D, E, F, G, H.
Invariants: escrow terms and mint are immutable; only correct party and phase may
act; custody PDA signer seeds are canonical; assets release or refund exactly once.
Tests: fake token program/account/mint, wrong PDA/bump, signer forwarding, account
alias, timeout boundary, acceptance race, CPI return spoofing, partial failure,
Token vs Token-2022 mismatch, and fuzzed action ordering.

## 9. AMM with slippage and price-impact limits

Build liquidity shares, constant-product swaps, fees, minimum-out/deadline, TWAP,
price-impact breaker, and sandwich simulation. Primary modules: A, B, C, F, G, H,
J. Invariants: reserve product does not decrease except documented rounding; LP shares
track proportional reserves; failed swaps are atomic; output meets caller limits.
Tests: zero/tiny reserves, donation, first LP, fee/rounding splits, large price impact,
front/victim/back sandwich, stale TWAP, minimum-out failure, reserve aliasing, overflow,
and stateful conservation fuzzing.

## 10. Fuzzed and audited mystery vault

Receive an intentionally buggy or unfamiliar open-source vault without a bug list.
Map architecture, write invariants, build Trident model-based fuzzing, minimize real
failures, write audit findings, patch them, and produce regression tests/runbook.
Primary modules: all, especially G and H. Invariants must be discovered and defended
by the learner. Minimum tests are not pre-specified beyond authority/account/CPI/math/
ordering boundaries: completion requires at least three reproducible findings, one
fuzzer-discovered semantic bug, minimized traces, patches, permanent tests, devnet
evidence, and an audit report with severity, impact, reproduction, and remediation.
