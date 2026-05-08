# MCP Active Plan Resources Slice

## Status

- State: shipped
- Created: 2026-05-07
- Shipped: 2026-05-07

## Summary

Expose active execution plans as local read-only MCP resources so MCP clients can discover and read the current plan artifact before taking workflow actions.

## Users

- Maintainers using MCP clients to inspect local repository workflow state.
- Coding agents that need the active execution plan as local context before moving through TDD gates.

## User-visible behavior

- `initialize` advertises MCP resources support without changing existing tool capabilities.
- `resources/list` returns one resource for each non-README markdown file in `docs/exec-plans/active/`, sorted deterministically.
- `resources/list` returns an empty resource list when no active execution plans exist.
- `resources/read` returns markdown text for a known active-plan resource URI.
- `resources/read` rejects malformed, non-string, non-active-plan, and unknown resource URIs with deterministic JSON-RPC errors.
- Existing MCP tools and their structured payloads remain unchanged.

## Resource contract

- Resource URIs use the server-owned form `vibe-sentinel://active-plans/<file-name>`.
- Resource names are derived from active plan file names.
- Resource MIME type is `text/markdown`.
- Resource contents are read only through the workspace probe allowlist created from discovered active plan paths.
- Resources do not expose completed plans, references, arbitrary workspace files, prompts, or write operations.

## Acceptance Criteria

- MCP initialization advertises `resources.listChanged` as `false` alongside existing `tools.listChanged` behavior.
- Resource listing is deterministic and excludes `README.md`, non-markdown files, directories, and files outside `docs/exec-plans/active/`.
- Resource reads return only allowlisted active-plan markdown content.
- Invalid resource requests map to protocol errors without aborting the stdio session.
- Existing MCP status, active-plan validation, and TDD gate tools keep their current names, input schemas, structured content, and error behavior.

## Non-goals

- Generic workspace file browsing.
- Completed-plan, reference, app-spec, or harness-doc resources.
- MCP prompts.
- Remote or network resource access.
- Any write-capable MCP resource behavior.

## Validation expectations

- Core tests cover deterministic resource discovery, URI allowlisting, successful reads, and unknown-resource rejection.
- MCP fixture tests cover `initialize`, `resources/list`, `resources/read`, invalid params, unknown URIs, empty active-plan state, workspace read failures, and existing tool regressions.
- Full validation follows the commands in `docs/tooling.md`.