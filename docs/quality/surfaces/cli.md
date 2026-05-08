# CLI Quality Guide

## Purpose

- Keep CLI behavior legible, deterministic, and easy for maintainers and coding agents to validate.
- Treat the CLI as a thin surface over the application core, consistent with `docs/architecture.md`.

## Command shape

- Keep command parsing separate from command execution.
- Make each command do one clear job and name subcommands by domain intent.
- Prefer explicit required arguments over implicit defaults that hide behavior.
- Do not duplicate business rules in argument parsing when the application core should own them.

## Flags and inputs

- Use predictable long flag names and add short flags only when they are conventional and unambiguous.
- Keep flag meanings stable; avoid reusing one flag for unrelated behaviors.
- Validate user input at the CLI boundary and report what is wrong and how to fix it.
- Accept machine-readable selectors, paths, and ids in a form that scripts can pass without guesswork.

## Output discipline

- Reserve `stdout` for command results and intentional machine-readable output.
- Send diagnostics, progress notes, and actionable failures to `stderr`.
- Keep success output concise and structured so future commands or agents can consume it reliably.
- Do not mix human commentary into machine-readable output modes.

## Exit behavior

- Return deterministic exit codes for success, user error, validation failure, and runtime failure.
- Keep identical failure conditions mapped to the same exit behavior across commands.
- Fail fast when preconditions are not met instead of continuing with partial or surprising results.
- Do not hide partial failure behind a success exit code.

## Deterministic failures

- Error messages should name the failed operation, the relevant identifier, and the next useful action.
- Avoid random ordering, ambiguous summaries, or environment-dependent wording in normal command output.
- Make timeout, retry, and cancellation behavior explicit when a command can block or wait.
- Preserve observability needed by `docs/reliability.md` so a future agent can inspect what happened locally.

## Testability

- Keep formatting, parsing, and exit-code decisions testable without spawning a real terminal.
- Cover command success, validation failure, and runtime failure with fixture-backed tests where practical.
- Isolate filesystem, process, and network access behind traits so command behavior stays mockable.
- When a command grows multiple output modes or branches that are hard to explain briefly, split it.
