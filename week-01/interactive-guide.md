# Week 01 Interactive Guide

## Big idea

This week is about learning to tell the computer exactly what you mean.

In beginner-friendly languages, the computer lets you be vague and then fails later.
Rust is stricter earlier so your programs fail less mysteriously later.

## Explain like I'm five

Programming is like giving instructions to a very literal robot.

If you say:

- "add these numbers"

the robot can do it.

If you say:

- "use that thing from over there"

the robot asks:

- "which thing?"
- "who owns it?"
- "can I change it?"

Rust makes you answer those questions early.

## Distributed systems from scratch

### What is a distributed system?

A distributed system is a bunch of computers trying to act like one system.

### Why is that hard?

Because they do not share one brain.

They have to:

- send messages
- wait
- guess whether another computer is slow or dead
- handle duplicates and delays

### ELI5 version

Imagine four kids trying to keep the same score in a game while passing notes across a noisy room.
Sometimes notes arrive late. Sometimes the same note arrives twice. Sometimes one kid leaves.
That is distributed systems.

### What we care about this week

We start with tiny local versions of these ideas:

- state
- updates
- retries
- health checks

## Checkpoints before coding

Answer these in `notes/`:

1. What is the difference between a variable and a value?
2. What is state?
3. Why is retrying useful?
4. Why can retrying be dangerous if an action is not idempotent?
5. What does it mean for a node heartbeat to go stale?

## Exercise-by-exercise prompts

### ex01-temperature-converter

Predict:

- what is the common base unit you can convert through?
- how do you avoid writing every pairwise conversion separately?

### ex02-fibonacci

Predict:

- what values should `F(0)` and `F(1)` return?
- is recursion or iteration simpler here?

### ex03-cli-calculator

Predict:

- what are the three parts of `12 * 3`?
- what input shapes should be rejected?

### ex04-retry-loop

Predict:

- when should the loop stop?
- what should happen if the operation never succeeds?

## Mini-project prompts

For `node-heartbeat-cli`, answer before coding:

1. What is the system state?
2. Which heartbeat matters if the same node sends many?
3. When is a node considered unhealthy?
4. How would duplicate heartbeats affect the result?
5. How would a real network make this harder?

## Improvement questions

After you finish the week:

- How would you persist heartbeat history?
- How would you mark nodes as `Unknown` instead of only healthy/unhealthy?
- How would you test time-based logic more cleanly?
