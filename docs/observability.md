# Observability and Local Validation

## Scope

- This file owns the repo-wide observability and validation requirements.
- Use `docs/quality/testing.md`, `docs/quality/fixtures.md`, and `docs/quality/surfaces/` for surface-specific testing and fixture guidance that supports these requirements.

## What must be visible

- CLI failures must expose the failed operation, key inputs, and deterministic exit behavior.
- TUI stateful or long-running work must be visible in the UI so background activity does not disappear silently.
- Local status and debug output must be enough for a future agent to inspect runtime behavior without guessing.
- Observability must survive partial failure; do not rely only on happy-path logs.

## Local workflows

- Record CLI examples and expected output when command behavior changes.
- Use ratatui render/state assertions for UI behavior that can be tested without a terminal.
- Keep execution-plan validation logs current after each skeleton unit is implemented.

## Logs and diagnostics

- Log at adapter and boundary points where external state can fail, not in every internal helper.
- Add context that identifies the subsystem and operation without dumping unrelated noise.
- Do not swallow the original failure when wrapping or translating errors.
- Keep debug output useful for local diagnosis without making normal success paths noisy.
- Remove temporary diagnostics once durable tests, fixtures, or status surfaces cover the behavior.

## Reporting

- Final responses and execution plans must distinguish successful validation, failed validation, skipped validation, and residual risk.
- Validation evidence must match the commands, render checks, snapshots, and fixtures that actually ran.
- Missing observability for long-running work is a quality bug, not a documentation issue.
