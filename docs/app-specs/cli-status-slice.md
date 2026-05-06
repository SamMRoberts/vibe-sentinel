# CLI Status Slice Spec

## Purpose

Provide the first narrow `vibe-sentinel` product slice: a local CLI status path
that reports whether the current workspace has the expected harness and Rust
project readiness signals.

## Users

- Maintainers checking whether a workspace is ready for harness-governed Rust work.
- Coding agents checking local readiness before making product changes.

## Proposed Public Contract

- Command: `vibe-sentinel status`
- Output format: deterministic human-readable text.
- Exit behavior:
  - Exit success when the command runs and can report readiness.
  - Exit failure for invalid arguments or unreadable workspace inputs.

This public CLI contract requires human approval before Rust skeletons are
created.

## Behavior

The status slice inspects local repository signals and returns a report with
named checks. The initial checks are:

- Harness docs: required harness docs are present.
- Active plan: at least one active execution plan is present when feature work
  is underway.
- Rust workspace: Cargo workspace files are present after bootstrap scaffolding.

The command must not perform network calls, credential reads, deployment
operations, destructive file operations, or hidden long-running work.

## Acceptance Criteria

- CLI parsing remains separate from status evaluation behavior.
- Status evaluation is expressed in the application core against a mockable
  workspace-probe trait.
- Filesystem access is isolated in an adapter.
- CLI output is deterministic and covered by tests.
- Invalid arguments produce actionable errors and deterministic exit behavior.
- Skeleton-level tests pass before status behavior is implemented.

## Non-goals

- Full CLI command set.
- TUI screens.
- MCP tools or resources.
- JSON output.
- Persistent configuration or storage.
- Network, credential, deployment, or destructive operations.