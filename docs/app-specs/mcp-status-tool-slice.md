# MCP Status Tool Slice Spec

## Purpose

Add the first MCP surface for `vibe-sentinel`: a local, read-only status tool
that exposes the existing readiness report to coding agents without requiring
them to parse CLI text output.

## Users

- Maintainers connecting local agent tooling to repository readiness checks.
- Coding agents checking harness readiness through MCP before changing code.
- Future workflow-enforcement slices that need a stable MCP status contract.

## Current Public Contract

- Entry point: `vibe-sentinel mcp serve`.
- Transport: local stdio MCP server unless the active execution plan approves a
  different local transport before implementation.
- Tool: `vibe_sentinel_status`.
- Tool behavior: read-only, idempotent, local-only, and non-destructive.
- Tool input: no required arguments for the first slice; omitted `arguments`
  and empty-object `arguments: {}` are accepted, while non-object or non-empty
  argument objects are rejected deterministically.
- Tool output: structured status data equivalent to `vibe-sentinel status --json`.
- Error behavior: invalid protocol payloads and workspace read failures return
  actionable MCP errors without panics or hidden side effects.

This public CLI and MCP contract was approved and shipped by the completed MCP
status tool slice.

## Status Shape

The MCP status tool returns the same readiness semantics as the shipped JSON
status slice:

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

The first MCP slice must not reinterpret the existing idle-state behavior: a
missing active plan may make aggregate readiness false when no implementation
slice is underway.

## Acceptance Criteria

- `vibe-sentinel status`, `vibe-sentinel status --json`, and
  `vibe-sentinel status --tui` remain compatible with their shipped contracts.
- `vibe-sentinel mcp serve` starts a local MCP server for the status tool.
- The MCP tool reuses the application core status evaluation path rather than
  duplicating readiness checks in the protocol adapter.
- MCP protocol handling stays thin and covered by request/response fixtures.
- Tool metadata identifies the status tool as read-only, idempotent, and local.
- MCP errors are deterministic and actionable for invalid payloads or unreadable
  workspace inputs.
- No network calls, credential reads, deployment actions, destructive file
  operations, or hidden long-running work are added.

## Non-goals

- Add remote MCP transport.
- Add write-capable MCP tools.
- Add resources, prompts, or multi-tool workflows.
- Deepen TDD semantic validation beyond the current status checks.
- Change the existing status report shape or idle-state readiness semantics.
- Add production deployment, credentials, persistent config, or storage.