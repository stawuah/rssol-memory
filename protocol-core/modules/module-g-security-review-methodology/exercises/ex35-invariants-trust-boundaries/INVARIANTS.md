# Capstone Invariant and Trust-Boundary Specification

This document must be completed before capstone implementation counts as complete.

## Assets and accounting identities

For each token, lamport balance, share, debt, position, proof/nullifier, governance
power, and upgrade key: where is it stored, who can move/change it, and which totals
must reconcile?

## Roles and capabilities

List users, admins, multisig members, keepers, liquidators, oracle publishers, upgrade
authority, external programs, RPC/operators, and attackers. Distinguish signer from
stored authority and account owner.

## Entrypoint table

For every instruction record inputs, accounts, signer/writable/owner/address/PDA/type
checks, preconditions, transition, CPI, emitted evidence, postconditions, replay rule,
and failure atomicity.

## Invariants

Number each invariant (`INV-001`). Make it falsifiable and map it to unit,
integration, adversarial, fuzz, monitoring, and audit evidence.

## Trust boundaries and emergency behavior

For every oracle, CPI, return value, proof verifier, token program, governance action,
client, keeper, and deployment step, document what is trusted, validation performed,
failure behavior, timeout/staleness, and blast radius. Define pause/unpause and which
actions remain available during failure.
