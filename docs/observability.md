# Observability and Local Validation

## Local workflows

- Record CLI examples and expected output when command behavior changes.
- Use ratatui render/state assertions for UI behavior that can be tested without a terminal.
- Capture MCP request/response fixtures for protocol behavior.
- For the local MCP status tool, fixture coverage should include
  newline-delimited stdio messages, legacy Content-Length framing,
  `initialize`, `tools/list`, successful `tools/call`, unknown tools, and
  workspace read failures.
- Keep execution-plan validation logs current after each skeleton unit is implemented.

## Reporting

Final responses and execution plans must distinguish successful validation,
failed validation, skipped validation, and residual risk.
