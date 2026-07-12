# Anchor Core Interactive Guide

## Think in analogies first

- Account = locker
- Program = machine
- Instruction = request
- PDA = locker without a private key
- CPI = one machine calling another machine
- Anchor = safety harness and form validator

## How to work each exercise

1. Read the analogy.
2. Read the tests.
3. Predict the output.
4. Fill in the missing code.
5. Re-run the tests.
6. Explain what the chain observer can still see.

## What to notice

- Ownership tells you who can move or mutate data.
- Borrowing tells you who can look without taking.
- `Result` tells you where failure is expected.
- Enums tell you what states are allowed.
- Validation tells you which inputs the program trusts.

## If you get stuck

- Ask what is public.
- Ask what is derived.
- Ask what must be checked before mutation.
- Ask whether the runtime can enforce the rule or whether your code must enforce it.

