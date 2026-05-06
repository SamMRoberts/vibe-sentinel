# TUI Status Slice Spec

## Purpose

Add a read-only ratatui status view over the existing readiness report so
maintainers and coding agents can inspect local harness state in an interactive
terminal surface without changing the status evaluation checks.

## Users

- Maintainers who prefer an at-a-glance terminal UI for local readiness.
- Coding agents validating that TUI rendering consumes the same status core as
  CLI text and JSON output.
- Future TUI workflows that need a minimal, tested application shell.

## Proposed Public Contract

- Command: `vibe-sentinel status --tui`
- Existing commands remain unchanged: `vibe-sentinel status` and
  `vibe-sentinel status --json`.
- Behavior: launches a read-only terminal status view, renders the project name,
  aggregate readiness, and ordered checks, then exits when the user presses `q`
  or `Esc`.
- Error behavior: invalid arguments still write actionable errors to stderr and
  exit failure.

This public TUI contract requires explicit human approval before Rust behavior is
changed.

## Rendering Contract

- Render the existing `StatusReport` without adding new readiness checks.
- Show aggregate readiness using the existing `StatusReport::is_ready()` result.
- Show each check with its name, readiness state, and detail in the same order
  produced by the application core.
- Keep render and state behavior testable without a real terminal.

## Dependency Decision

Preferred implementation uses `ratatui` for rendering and a terminal backend
crate only if needed by the runtime surface. Adding dependencies requires
explicit human approval before Rust changes.

## Acceptance Criteria

- Text and JSON status output remain byte-for-byte compatible with completed
  status slices.
- `status --tui` renders project name, aggregate readiness, and ordered checks
  from the existing status service.
- TUI state transitions include deterministic exit handling for `q` and `Esc`.
- Render/state behavior is covered by tests that do not require an interactive
  terminal.
- Status evaluation remains in application core and is not duplicated in the TUI
  surface.

## Non-goals

- Add broad TUI navigation or multiple screens.
- Add new readiness checks.
- Add MCP tools or resources.
- Change text or JSON status output.
- Add network, credential, deployment, or destructive behavior.