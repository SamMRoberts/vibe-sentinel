# Error Guide

## Error shape

- Fail with messages that tell the operator or next agent what failed, where it failed, and what to inspect next.
- Keep domain errors focused on business meaning and boundary errors focused on I/O, protocol, or environment failures.
- Add context as errors cross CLI, TUI, MCP, storage, or process boundaries without hiding the root cause.
- Prefer explicit error types over loose strings once a subsystem has more than one meaningful failure mode.
- Reserve panics for impossible states, invariant violations, or intentional crash-on-bug behavior.

## Surface behavior

- Keep CLI failures deterministic in both exit code and stderr wording for the same class of problem.
- Keep TUI errors visible in the screen state; do not bury them in logs while leaving the UI ambiguous.
- Keep MCP error mapping stable so invalid params, unknown tools, and workspace failures produce clear protocol responses.
- Separate user-facing messages from debug detail when the same failure needs both.
- Do not leak adapter internals into surface messages unless that detail helps local diagnosis.

## Handling rules

- Validate early at boundaries so downstream code works with trusted shapes.
- Do not swallow errors and continue with partial or guessed state.
- When retrying, preserve the final failure with enough context to explain both the action and the retry path.
- Prefer one consistent error path per subsystem instead of mixing sentinel values, logs, and exceptions casually.
- Treat ambiguous fallback behavior as an error unless the contract explicitly allows it.

## Validation and maintenance

- Add tests for failure paths that affect exit behavior, visible UI state, or MCP protocol contracts.
- Keep fixture names and test assertions specific about the failure being proven.
- Update docs and examples when user-visible error wording or behavior changes materially.
- Remove dead error variants and stale translation code when contracts change.
- If an error cannot guide the next action, refine it before the pattern spreads.
