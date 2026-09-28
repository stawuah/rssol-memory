# Module 8 — Async with Tokio

Level: **Senior**. The same designs from Modules 3–5 (actors, bounded
queues, timeouts, shutdown, bounded fan-out), now in tokio, where most
Solana off-chain infrastructure lives.

## Map

| Exercise | Rust skill | Production application |
|---|---|---|
| ex23 | `mpsc` + `oneshot` + `select!` + `timeout`, owned-state actor | RPC services, relayers, Geyser consumers |
| ex24 | `JoinSet` + `Semaphore` + per-call timeouts, panic isolation | fetchers, backfills, keeper scans, health checks |

## Ordered exercises

1. [Tokio actor](./exercises/ex23-tokio-actor/README.md)
2. [Bounded fan-out](./exercises/ex24-bounded-fanout/README.md)

## Coverage checklist

- [x] every request has a deadline
- [x] shutdown vs all-handles-dropped exits
- [x] permits acquired before spawning
- [x] per-task failure isolation and deterministic output order

**Common interview question:** When do you use `tokio::spawn`, when
`spawn_blocking`, and when a rayon pool? What goes wrong if you verify
signatures directly inside an async task?
