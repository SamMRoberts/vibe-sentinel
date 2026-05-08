# Concurrency Guide

## Ownership and lifecycle

- Give every background task a clear owner, start condition, and stop condition.
- Do not introduce hidden forever-work in the CLI or ratatui TUI paths.
- Pass shutdown and cancellation through the stack instead of relying on process exit to clean up.
- Keep task boundaries explicit so the next agent can tell which layer owns retries, waiting, and cleanup.
- Treat leaked work, orphaned watchers, and stale loops as correctness bugs.

## Surface expectations

- Keep CLI commands single-shot by default; long-running behavior must expose status and exit conditions clearly.
- Keep TUI background work visible in state and easy to stop when the screen or session ends.
- Isolate concurrency in adapters or orchestrators rather than scattering task spawning across feature code.
- Prefer message passing or explicit state transitions over shared mutable state when coordination is required.

## Reliability controls

- Preserve existing timeouts, retries, and idempotent behavior unless an approved plan changes them.
- Make retry policy explicit at the boundary that owns the failure mode.
- Avoid blocking async paths with terminal, filesystem, or process work that should be awaited or delegated.
- Keep lock scope narrow and avoid holding shared state across awaits, renders, or external calls.
- When concurrency makes a contract harder to test, simplify the design before adding more coordination code.

## Testing signals

- Add deterministic tests for shutdown, cancellation, and repeated start-stop behavior when concurrency is introduced.
- Use fakes or scripted channels to prove ordering without depending on wall-clock sleeps.
- Cover error and timeout paths, not just successful background work.
- Make race-prone behavior inspectable through state transitions or emitted events.
- If the only proof is manual observation, the concurrency design is not finished.
