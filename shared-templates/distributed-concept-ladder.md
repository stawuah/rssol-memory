# Distributed Concept Ladder

Use this ladder whenever a distributed-systems concept appears.

## Level 0: Real life

Explain using people, messages, and waiting.

Examples:

- A timeout is waiting for your friend to reply and deciding they might be offline.
- A retry is calling again because the first call may not have gone through.
- Replication is multiple people keeping the same notebook.

## Level 1: Explain like I'm five

Use simple objects:

- computers are kids passing notes
- storage is a notebook
- network delay is the hallway being crowded
- duplication is the same note arriving twice
- failure is a kid disappearing mid-game

## Level 2: System model

State the actual engineering version:

- what are the nodes?
- what messages are sent?
- what state is stored?
- what assumptions are we making?

## Level 3: Failure model

Ask:

- can messages be delayed?
- can they be duplicated?
- can they be lost?
- can machines crash?
- can clocks disagree?

## Level 4: Design response

Ask:

- what invariant do we need?
- what data structure enforces it?
- what retry or dedup rule helps?
- what tests prove it?

## Level 5: Solana connection

Translate the concept to Solana when relevant:

- accounts are shared state
- instructions are deterministic state transitions
- transactions declare read/write sets
- PDAs express authority and derived ownership
- indexers create eventual-consistency problems off-chain
