# Week 02 Interactive Guide

## Big idea

This week is about who owns data, who may read it, and who may change it.

That is a Rust topic, but it is also a systems topic.
Bad ownership rules create confusing programs.
Bad authority rules create broken distributed systems and insecure Solana programs.

## Explain like I'm five

Imagine toys in a classroom.

- If one kid owns a toy, they decide what happens to it.
- Other kids may look at it.
- Maybe one kid is allowed to hold it for a moment.
- But two kids should not both think they are changing the exact same toy at the exact same time.

Rust turns that toy rule into compiler rules for data.

## Distributed systems from scratch

### What is a state machine?

A state machine is a system that:

- has a current state
- receives an event
- moves to the next valid state

### ELI5 version

Think of a traffic light:

- green
- yellow
- red

It does not randomly jump wherever it wants.
Each event moves it in a controlled way.

### Why this matters

Distributed systems are full of state machines:

- follower to candidate
- candidate to leader
- task proposed to acknowledged to completed

Solana programs are also state machines:

- instruction comes in
- account state changes deterministically

## Checkpoints before coding

Answer these in `notes/`:

1. What is the difference between owning a `String` and borrowing `&str`?
2. Why would returning `&Node` be better than cloning a `Node` in some cases?
3. What is a state transition?
4. Why should state transitions be explicit?
5. Why is "who can mutate state" a security question on Solana?

## Exercise-by-exercise prompts

### ex01-moves-and-borrows

Predict:

- do you need to return a new `Node`, or a reference to one already in the slice?
- when do you need `&mut` instead of `&`?

### ex02-cluster-message-parser

Predict:

- which parts of the input can be borrowed directly?
- what input shapes should be rejected immediately?

### ex03-node-role-model

Predict:

- which roles can initiate elections?
- which roles can accept writes?

### ex04-state-transitions

Predict:

- what should happen when a follower times out?
- what should happen when a leader sees a higher-term leader?

## Mini-project prompts

For `raft-lite-state-machine`, answer before coding:

1. What fields define the node state?
2. Which event should increment the term?
3. When should votes reset?
4. What invariant must hold before becoming leader?
5. What would go wrong if higher-term messages were ignored?

## Improvement questions

After you finish the week:

- How would you model rejected transitions?
- How would you store a history of transitions?
- How would you test that impossible states never occur?
- How does this map to Solana account state updates?
