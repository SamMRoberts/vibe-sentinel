# Observability Guide

## What must be visible

- Make CLI failures expose the failed operation, key inputs, and deterministic exit behavior.
- Make TUI stateful or long-running work visible in the UI so background activity does not disappear silently.
- Make MCP failures observable through stable error mapping and fixture-backed protocol responses.
- Keep enough local status and debug signal that a future agent can inspect runtime behavior without guessing.
- Prefer observability that survives partial failure over logs that only help on the happy path.

## Surface-specific guidance

- For CLI changes, record representative commands and expected output when behavior changes.
- For ratatui features, expose loading, empty, error, and background-work states explicitly instead of hiding them behind stale screens.
- For MCP tools and resources, keep request and response fixtures current for success, invalid input, and workspace read failures.
- Use consistent terms across CLI, TUI, and MCP when they report the same underlying state.
- Keep machine-readable output stable when it is intended for automation.

## Logging and diagnostics

- Log at adapter and boundary points where external state can fail, not in every internal helper.
- Add context that identifies the subsystem and operation without dumping unrelated noise.
- Do not swallow the original failure when wrapping or translating errors.
- Keep debug output useful for local diagnosis but avoid making normal success paths noisy.
- Remove temporary diagnostics once a durable contract or test covers the behavior.

## Validation evidence

- Keep execution-plan and final-response validation evidence aligned with the actual commands and fixtures you ran.
- Distinguish successful validation, failed validation, skipped validation, and residual risk clearly.
- Update render assertions, snapshots, and protocol fixtures when observable behavior changes.
- Prefer artifacts that another agent can rerun locally over one-off narrative descriptions.
- Treat missing observability for long-running work as a quality bug, not a documentation issue.
