# Mock Audit Report Template

## Executive summary

Repository, exact commit, dates, reviewer, architecture, assets at risk, and overall
assessment. State limitations; this exercise is not a professional audit warranty.

## Scope and methodology

List included/excluded files, build/test commands, threat model, roles, external
programs, oracle assumptions, manual review, static tools, tests, and fuzzing.

## Architecture and trust boundaries

Map entrypoints, state accounts, authorities, PDAs, token custody, CPI edges,
oracles, upgrade governance, and off-chain actors.

## Findings

For every finding include: stable ID; title; severity and likelihood/impact rationale;
affected file/line/commit; violated invariant; prerequisites; step-by-step reproduction;
proof test or trace; impact; remediation; patch review; regression test; status.

Do not report style preferences as security findings. If no exploit path or invariant
violation exists, record it as hardening/informational with honest severity.

## Unresolved assumptions and appendix

List accepted risks, untested dependencies, commands/output, and all artifacts needed
for another reviewer to reproduce the work.
