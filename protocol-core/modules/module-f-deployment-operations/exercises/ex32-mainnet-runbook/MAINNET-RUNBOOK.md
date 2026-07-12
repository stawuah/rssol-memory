# Mainnet Readiness Runbook

This is a required operational artifact, not an essay. Replace every placeholder.

## Ownership

- Release lead and backup:
- Security approver:
- Upgrade-authority custody/quorum:
- Incident commander and communication channel:

## Go/no-go prerequisites

- [ ] Commit, toolchain, dependencies, binary hash, IDL hash, and program ID pinned
- [ ] Unit, integration, adversarial, fuzz, and regression suites pass
- [ ] Independent review findings resolved or explicitly accepted
- [ ] Invariants and trust boundaries mapped to tests and monitors
- [ ] Devnet deployment rehearsal completed from a clean environment
- [ ] Authority, treasury, RPC, oracle, and external-program addresses verified
- [ ] Compute/fee measurements include maximum state/input
- [ ] Monitoring, paging, dashboards, and incident roles tested
- [ ] Migration and rollback/forward-fix decision approved

## Deployment procedure

For each command record operator, UTC time, expected output, actual output, and
transaction signature. Stop immediately when output differs; do not improvise.

## Post-deployment verification

- Confirm program-data address, binary hash, upgrade authority, executable state,
  IDL/client version, configuration PDAs, token/oracle program identities, and all
  privileged roles.
- Simulate first, then execute a minimal canary transaction.
- Confirm events, balances, account diffs, compute, and alerts.

## Incident actions

Define observable trigger, severity, pause authority, exact pause procedure, user
communication, evidence preservation, and recovery criteria for: oracle failure,
stuck transactions, compute regression, accounting mismatch, authority compromise,
external CPI outage, and active exploit.

## Rollback / forward fix

Solana state migrations may not be reversible. Document which code/config changes
can be rolled back, how account migrations are resumed, and the invariant required
before unpausing. Never claim rollback exists without a rehearsed procedure.
