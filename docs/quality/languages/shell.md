# Shell Guidelines

## Script design

- Keep shell scripts small, task-focused, and easy to inspect from top to bottom.
- Use shell only for orchestration, glue, and simple text or process workflows.
- Move complex business logic into a better-suited language instead of forcing it into shell.
- Name scripts by outcome, not implementation detail.
- Prefer small reusable functions over long linear scripts with repeated blocks.

## Safety and correctness

- Start Bash scripts with `set -euo pipefail` unless a specific script contract requires different behavior.
- Quote variable expansions unless unquoted splitting is deliberate and documented.
- Treat all external input as untrusted, including environment variables, arguments, and command output.
- Avoid `eval` and indirect expansion patterns that make behavior hard to reason about.
- Check destructive operations carefully and fail fast when required inputs or paths are missing.

## Structure and modularity

- Keep functions focused on one task; extract helpers when branching or setup obscures intent.
- Keep scripts cohesive; if a script manages multiple unrelated workflows, split it.
- Prefer passing values as function arguments instead of relying on mutable global variables.
- Isolate filesystem, network, and subprocess boundaries so they can be stubbed in tests.
- Keep loops and conditionals shallow when possible; deeply nested shell logic is a refactor signal.

## Commands and data flow

- Prefer explicit command sequences over dense pipelines that are hard to debug.
- Use `printf` instead of `echo` when output must be predictable across environments.
- Check command availability and versions when a script depends on non-portable tools.
- Use arrays where supported instead of string-building command arguments by hand.
- Do not parse human-oriented output when a machine-readable flag or format is available.

## Errors and diagnostics

- Emit clear error messages to stderr with the failed operation and key inputs.
- Handle expected non-zero exits intentionally instead of masking failures with broad `|| true`.
- Preserve exit codes when wrapping commands so callers can react correctly.
- Add tracing or debug output in a way that can be enabled without editing the script.
- Avoid silent fallbacks that make failures look like success.

## Testing and maintainability

- Test observable outcomes such as files, exit codes, and emitted output.
- Keep scripts deterministic by controlling environment variables, working directories, and temp paths.
- Design functions so core behavior can be exercised with fixture files and stubbed commands.
- Prefer simple contracts over generic helpers that hide command behavior.
- When a function or script becomes hard to explain briefly, treat that as a refactor signal.
- Remove dead branches, commented-out commands, and duplicated command sequences quickly.
- Document non-obvious portability assumptions near the top of the script.
