# Observability and Local Validation

## Local workflows

- Record CLI examples and expected output when command behavior changes.
- Use ratatui render/state assertions for UI behavior that can be tested without a terminal.
- Capture MCP request/response fixtures for protocol behavior.
- For the local MCP status tool, fixture coverage should include
  newline-delimited stdio messages, legacy Content-Length framing,
  `initialize`, `tools/list`, successful `tools/call`, unknown tools, and
  workspace read failures.
- For the local MCP active-plan validation tool, fixture coverage should include
  `tools/list` metadata, successful `tools/call`, empty active-plan state,
  semantic validation issues, and workspace read failures.
- For no-argument MCP tools, fixture coverage should include omitted arguments,
  empty-object arguments, non-object arguments, unexpected argument properties,
  and malformed `tools/call` envelopes that must not abort the stdio session.
- For local MCP active-plan resources, fixture coverage should include
  `initialize` capability metadata, `resources/list`, `resources/read`, empty
  active-plan state, malformed resource params, unknown URIs, workspace read
  failures, and session continuity after protocol errors.
- Keep execution-plan validation logs current after each skeleton unit is implemented.

## Reporting

Final responses and execution plans must distinguish successful validation,
failed validation, skipped validation, and residual risk.
