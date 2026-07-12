# Week 03 Interactive Guide

## Big idea

This week is about organizing data and failures clearly.

## Explain like I'm five

A messy toolbox makes even simple repairs hard.
Modules organize the tools. Error types explain what went wrong instead of yelling "something broke".

## Distributed systems from scratch

This week introduces idempotency.

ELI5:

If you ask someone to close a door twice, the second request should not break the door.

System version:

If a message is delivered twice, the second processing should not corrupt state.

## Checkpoints

- What is the difference between `Option` and `Result`?
- Why are duplicate messages common in distributed systems?
- What state do you need in order to deduplicate safely?
- Why is "unknown error" a bad API?
