# Week 07 Mini-Project

Project: `durable-event-log`

## Build target

Build a small event log that:

- appends events
- flushes or persists them safely enough for the exercise
- replays them on startup
- rebuilds derived state
- exposes tests for crash-recovery behavior

## Interactive prompts

- What is the source of truth?
- What state is derived?
- What happens if the last write is partial?
- How would a real database improve this design?
