# Week 12 Interactive Guide

## Big idea

This week is not about memorizing APIs. It is about learning to think like the runtime.

## Study loop

For every exercise:

1. Read the objective and predict the answer before opening the code.
2. Write a one-line model of the state you think exists.
3. Write the failure you expect if the check is wrong.
4. Implement the smallest correct version.
5. Run the tests.
6. Compare the output with your prediction.
7. Write down the first invariant you broke.

## Hint ladder

Use hints only after you have a prediction.

- Hint 1: Identify what is owned and what is borrowed.
- Hint 2: Identify what can be modified and what must stay fixed.
- Hint 3: Ask which checks belong in the instruction handler and which checks belong in account validation.
- Hint 4: Ask what the public observer can see even when the token amount is hidden.

## What to say out loud

- This program is a state machine.
- This account is the source of truth.
- This check prevents invalid authority.
- This detail is public even if the balance is private.
- This design still leaks timing unless I add batching or delay.

## Capstone thinking

Your capstone is not only “hide the amount.”

You need to reason about:

- what the mempool can see
- what the chain can verify
- what the user reveals before execution
- what can be reconstructed from timestamps, ordering, and failed attempts

## Self-test

If you can explain these, you are on track:

- Why does a PDA not need a private key?
- Why is Anchor useful but not magic?
- Why is confidential transfer not the same as full transaction privacy?
- Why does a verifier need a proof and not just a promise?

