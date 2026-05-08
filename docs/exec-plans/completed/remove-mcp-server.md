# Execution Plan: Remove MCP Server Surface

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-08
- Last updated: 2026-05-08

## Summary

Remove the local Vibe Sentinel MCP server from the shipped product so the crate
ships only the CLI and ratatui TUI surfaces. This includes deleting MCP runtime
code, CLI routing, MCP-only tests, and product documentation that describes the
local MCP server as a supported surface.

## Scope

### In scope

- Update harness and product docs from CLI/TUI/MCP scope to CLI/TUI scope.
- Remove `vibe-sentinel mcp serve` parsing and runtime routing.
- Delete the local MCP server module and MCP-only data/resource helpers.
- Remove MCP-only tests, specs, quality guidance, and validation commands.
- Remove dependencies that are only needed by the deleted MCP server.

### Out of scope

- Removing the Symdex MCP evidence workflow used by coding agents.
- Changing the CLI `status` or `status --json` behavior.
- Changing the ratatui TUI status view behavior.
- Rewriting historical non-MCP feature plans beyond removing obsolete MCP docs.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/tooling.md`
- `docs/architecture.md`
- `docs/app-specs/index.md`

## Acceptance criteria

- `cargo run -- mcp serve` is no longer a supported command.
- `src/mcp.rs` is removed and `src/lib.rs` no longer exports an MCP module.
- Local product docs describe Vibe Sentinel as CLI/TUI only.
- Local docs no longer route agents to local Vibe Sentinel MCP specs, commands,
  or quality guides.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all`, `cargo build --all-targets`, and plan validation pass.

## TDD artifacts

### Feature Info

- Goal: Remove the local MCP server and make CLI/TUI the only shipped product surfaces.
- Acceptance criteria: MCP command/runtime/tests/docs are removed while existing CLI/TUI status behavior remains validated.
- Constraints: Preserve harness workflow, keep Symdex MCP evidence docs because they describe an external agent tool workflow, and avoid unrelated rewrites.
- Non-goals: Adding replacement automation protocols, changing status semantics, or expanding CLI/TUI behavior.

### Research Notes

- Docs inspected: `AGENTS.md`, `docs/harness/scope.md`, `docs/harness/operating-model.md`, `docs/tooling.md`, `docs/app-specs/index.md`.
- Code inspected: `src/cli.rs`, `src/main.rs`, `src/lib.rs`, `src/core.rs`, `src/domain.rs`, `Cargo.toml`.
- External references copied to `docs/references/`: none.
- Findings: The local MCP server is exported from `src/lib.rs`, routed from `src/main.rs`, parsed as `mcp serve` in `src/cli.rs`, implemented in `src/mcp.rs`, and documented in app specs, quality docs, tooling, observability, README, and completed MCP execution plans.

### Reviewed Plan

- Plan review status: reviewed
- Refinements made: Treat the user's request as explicit approval to change product scope and dependencies; preserve Symdex MCP evidence workflow because it is not the removed server.

### Architecture Pseudocode

```text
module cli
  enum CliCommand
    variant Status
  fn parse_args(args) -> Result<CliArgs, VibeError>
    accept status, status --json, status --tui
    reject mcp and other unknown commands with CLI/TUI-only usage
  fn execute_with_probe(args, probe) -> Result<StatusReport, VibeError>
    evaluate status only

module main
  fn run(args) -> Result<Option<String>, VibeError>
    parse args
    evaluate status with FsWorkspaceProbe
    route OutputFormat::Tui to run_status_tui
    route text/json to render_status

module lib
  remove module mcp export

module core
  remove ActivePlanResourceService and active_plan_resource helpers

module domain
  remove ActivePlanResource and ActivePlanResourceRead
  rename MCP-specific serialization test to structured serialization wording

module docs
  update product scope, architecture, tooling, quality, observability, security,
  app-spec indexes, README, and harness docs to CLI/TUI-only shipping scope
  delete local MCP slice specs, completed MCP plans, and MCP surface quality guide
```

### Reviewed Architecture

- Architecture review status: reviewed
- Refinements made: Keep plan-validation and TDD-gate domain/core services intact because they remain harness capabilities even after the MCP adapter is removed.

### Skeleton Checklist

- [x] Existing CLI/TUI skeletons already exist and remain the tested boundary.
- [x] MCP removal is a deletion refactor, so no new skeleton units are needed.

### Mock Test Checklist

- [x] Update CLI parser tests to assert `mcp` is rejected as unsupported.
- [x] Keep existing CLI status tests and TUI status tests as regression coverage.
- [x] Keep core plan-validation and TDD-gate tests where they are not MCP-specific.

### Implementation Checklist

- [x] Update harness/product docs to CLI/TUI-only scope.
- Validation after this unit: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/remove-mcp-server.md` passed before implementation; docs updated before Rust removals.
- [x] Remove MCP CLI routing and runtime exports.
- Validation after this unit: Rust code review confirmed `src/main.rs` no longer imports or routes an MCP runtime and `src/lib.rs` no longer exports an MCP module.
- [x] Remove MCP module, MCP-only core/domain helpers, specs, and quality docs.
- Validation after this unit: `src/mcp.rs` and local MCP specs/plans/quality guide deleted; remaining MCP references reviewed with `rg`.
- [x] Remove obsolete dependencies if no longer needed.
- Validation after this unit: no MCP-only crate dependency remains; `serde_json` is retained for `status --json`.
- [x] Run full validation and review the diff for remaining local MCP references.
- Validation after this unit: formatting, clippy, tests, build, TDD validators, diff check, unsupported-command smoke, and reference sweep completed.

### Validation Log

- 2026-05-08: Initial repository inspection and Symdex metadata pass completed; Symdex staleness showed core files missing from the current index, so live file reads are authoritative.
- 2026-05-08: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/remove-mcp-server.md` -> passed before implementation.
- 2026-05-08: Updated harness/product docs to CLI/TUI-only scope, deleted the local MCP runtime module, removed local MCP specs/plans/quality guide, and retained only non-product Symdex MCP evidence references plus this removal plan.
- 2026-05-08: `cargo fmt --check` -> passed.
- 2026-05-08: `cargo clippy --all-targets --all-features -- -D warnings` -> passed.
- 2026-05-08: `cargo test --all` -> passed with 33 library tests, 2 CLI integration tests, 2 TUI integration tests, and doc tests.
- 2026-05-08: `cargo build --all-targets` -> passed.
- 2026-05-08: `python3 scripts/validate_tdd_workflow.py` -> passed for 2 files.
- 2026-05-08: `git --no-pager diff --check` -> passed.
- 2026-05-08: `cargo run -- mcp serve` -> failed as expected with `unknown command 'mcp': expected 'status'`.
- 2026-05-08: `rg -n "\bMCP\b|\bmcp\b|mcp serve|serve-mcp|tools/call|tools/list|resources/list|resources/read|Model Context" .` -> remaining hits are Symdex MCP evidence docs, the initialization change note, and this completed removal plan.
- 2026-05-08: After moving this plan to completed, `python3 scripts/validate_tdd_workflow.py docs/exec-plans/completed/remove-mcp-server.md`, `python3 scripts/validate_tdd_workflow.py`, and `git --no-pager diff --check` passed.

### Review Notes

- Diff review: complete; runtime MCP entrypoint and module are gone, product docs describe CLI/TUI-only shipping scope, and the remaining MCP references are not local Vibe Sentinel server support docs.
- Risks: Remaining references are limited to Symdex evidence guidance, the scope-change note, and this completed removal record.
- Follow-ups: none.

## Intended changes

- `src/cli.rs`: remove `McpServe`, `mcp serve` parsing, and MCP runtime error path.
- `src/main.rs`: remove MCP imports and routing.
- `src/lib.rs`: stop exporting `mcp`.
- `src/mcp.rs`: delete the local MCP server implementation.
- `src/core.rs` and `src/domain.rs`: delete MCP-resource-only helpers and types.
- Docs: update CLI/TUI-only product scope and remove local MCP specs/guides/plans.
- `Cargo.toml` and `Cargo.lock`: remove dependencies that become unused.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/remove-mcp-server.md`: required before implementation.
- `cargo fmt --check`: expected pass.
- `cargo clippy --all-targets --all-features -- -D warnings`: expected pass.
- `cargo test --all`: expected pass.
- `cargo build --all-targets`: expected pass.
- `python3 scripts/validate_tdd_workflow.py`: expected pass.
- `git --no-pager diff --check`: expected pass.
- `rg -n "\bMCP\b|\bmcp\b|mcp serve|serve-mcp" .`: expected to show only Symdex MCP evidence references or none for the local Vibe Sentinel server.

## Risks and rollback

- Risk: Removing `serde_json` may break `status --json`.
- Mitigation: Keep `serde_json` unless compiler and tests prove it is unused.
- Rollback: Restore `src/mcp.rs`, CLI/main routing, deleted docs, and Cargo dependency entries from git history.

## Progress log

- 2026-05-08: Created active plan after scope gate found current harness still listed MCP as a product surface.

## Decisions

- Preserve Symdex MCP references: They describe an external evidence workflow for agents, not the Vibe Sentinel server being removed.
