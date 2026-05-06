# Status JSON Slice Spec

## Purpose

Add a structured output mode to the existing `vibe-sentinel status` command so
future agents, tests, TUI views, and MCP surfaces can consume readiness results
without parsing human-readable text.

## Users

- Maintainers integrating status checks into scripts.
- Coding agents reading local readiness signals programmatically.
- Future TUI and MCP surfaces that need structured status data.

## Proposed Public Contract

- Command: `vibe-sentinel status --json`
- Existing command remains unchanged: `vibe-sentinel status`
- Output format: deterministic JSON object written to stdout.
- Error behavior: invalid arguments still write actionable errors to stderr and
  exit failure.

This public CLI contract requires explicit human approval before Rust behavior
is changed.

## Proposed JSON Shape

```json
{
  "project_name": "vibe-sentinel",
  "ready": false,
  "checks": [
    {
      "name": "harness docs",
      "state": "ready",
      "detail": "required harness docs present"
    }
  ]
}
```

## Dependency Decision

Preferred implementation uses `serde` and `serde_json` to avoid hand-rolled JSON.
Adding these dependencies requires explicit human approval before Rust changes.

## Acceptance Criteria

- Text status output remains byte-for-byte compatible with the completed CLI
  status slice.
- `status --json` emits deterministic JSON with project name, aggregate readiness,
  and all status checks.
- CLI parsing remains separate from status evaluation behavior.
- JSON formatting is covered by unit tests.
- Binary integration tests cover `vibe-sentinel status --json`.

## Non-goals

- Add broad CLI command structure.
- Add TUI behavior.
- Add MCP tools or resources.
- Change status evaluation checks.
- Add network, credential, deployment, or destructive behavior.