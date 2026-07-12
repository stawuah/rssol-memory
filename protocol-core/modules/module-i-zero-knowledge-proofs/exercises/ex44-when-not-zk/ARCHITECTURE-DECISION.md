# Architecture Decision: Is ZK Necessary?

Define the exact secret witness, public statement/inputs, verifier, adversary, and
metadata that remains visible. Then compare ZK with direct validation, oracle,
signature/attestation, commit-reveal, access control, encryption without proof, and
ordinary Merkle membership.

Score each option on whether it solves the threat, latency, on-chain compute, proof
infrastructure, setup/key risk, circuit upgrades, auditing, client compatibility,
availability, and operator burden. Choose ZK only when proving a relation about a
hidden witness is essential and cheaper mechanisms do not meet the security goal.
