# 01 Account vs Program

## Analogy

- An account is a locker
- A program is the machine that can change lockers
- An instruction is the action request
- A transaction is the bundle of requests plus the lockers involved

## Official facts

Solana says an account is its fundamental data unit and that a program is stateless, with mutable state living in separate data accounts.

- [Solana Accounts](https://solana.com/docs/core/accounts)
- [Solana Programs](https://solana.com/docs/core/programs)

## Why it matters

If you mix code and state in your head, Anchor feels confusing.
If you separate them, almost everything becomes a state transition.

## Exercise

Answer these in one sentence each:

1. What is the state?
2. What is the code?
3. What does the transaction declare up front?

## Hint

Think: public notebook plus strict machine.

