# Required Devnet Deployment Record

Complete this with real output; never insert a private key or seed phrase.

## Reproducible inputs

- Git commit:
- Rust version:
- Solana/Agave CLI version:
- Anchor version:
- Cluster and RPC origin:
- Declared program ID:
- Upgrade authority public key or governance address:
- Program binary SHA-256:
- IDL SHA-256 and semantic version:

## Pipeline

1. Confirm CLI cluster is devnet and fee payer is intentionally selected.
2. Run tests and build with pinned versions.
3. Confirm `declare_id!`, `Anchor.toml`, keypair public key, and expected program ID agree.
4. Simulate representative initialization and maximum-compute instructions locally.
5. Deploy to devnet using the project's documented Anchor deployment command.
6. Verify the deployed program data and authority using official CLI inspection.
7. Publish/version the IDL using the Anchor version's supported IDL workflow.
8. Run smoke tests against the deployed address.
9. Record transaction signatures, hashes, UTC time, operator, and verification result.

## Evidence

- Deployment signature:
- IDL action signature:
- Explorer link:
- Smoke-test command/result:
- Binary/IDL equality verification:
- Rollback or corrective action if verification failed:
