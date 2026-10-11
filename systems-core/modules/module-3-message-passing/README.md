# Module 3 — Message Passing

Level: **Mid**. Most production Rust services are built from channels:
pipelines of stages, actors that own state, and service loops that tick and
shut down cleanly. Backpressure and shutdown are the senior-level details.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex06 | bounded `sync_channel`, fan-out/fan-in, drain-before-join | TPU-style fetch → sigverify → banking pipeline |
| ex07 | actor + one-shot replies, pure command handler | RPC/indexer/relayer services, "handle + task" design |
| ex08 | `recv_timeout` loops, stop flags, draining shutdown | every long-running validator service thread |

## Ordered exercises

1. [Bounded pipeline](./exercises/ex06-bounded-pipeline/README.md)
2. [Actor with one-shot replies](./exercises/ex07-actor-oneshot/README.md)
3. [Shutdown loop](./exercises/ex08-shutdown-loop/README.md)

## Coverage checklist

- [x] bounded vs rendezvous channels and backpressure
- [x] join-before-drain deadlock
- [x] request/response with failure → `ActorGone`
- [x] cooperative cancellation and graceful drain

**Common interview question:** Your service's memory grows under load and it
hangs on shutdown. What are the likely causes in its channel design, and how
do you fix each?
