# Execution Plan: Status JSON Output

## Status

- State: pending approval
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Add an approved structured JSON output mode to the existing `vibe-sentinel status` command while preserving the current deterministic text output. This plan is intentionally stopped before Rust behavior changes until the public CLI contract and dependency decision receive explicit human approval.

## Scope

### In scope

- Propose `vibe-sentinel status --json` as the next public CLI contract.
- Preserve `vibe-sentinel status` text output unchanged.
- Add JSON serialization for `StatusReport` and readiness checks after approval.
- Add unit and integration tests for JSON output after approval.
- Update README and validation logs after implementation.

### Out of scope

- TUI screens.
- MCP tools or resources.
- New readiness checks.
- Network, credential, deployment, data migration, or destructive behavior.
- Hand-rolled JSON if `serde` and `serde_json` are approved.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/app-specs/cli-status-slice.md`
- `docs/app-specs/status-json-slice.md`
- `docs/architecture.md`
- `docs/tooling.md`
- `docs/quality.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/review.md`
- `docs/exec-plans/plan-template.md`

## Acceptance criteria

- Explicit human approval is recorded for the public CLI contract `vibe-sentinel status --json` before Rust behavior changes.
- Explicit human approval is recorded before adding `serde` and `serde_json`, or the plan is revised to avoid dependencies.
- Existing text status output remains unchanged.
- JSON output includes `project_name`, aggregate `ready`, and ordered `checks` with `name`, `state`, and `detail`.
- JSON unit and integration tests pass.
- Full validation passes before completion.

## Modified TDD artifacts

### Feature Info

- Goal: add structured JSON output to the existing status command without changing current text output.
- Acceptance criteria:
  - `vibe-sentinel status` remains unchanged.
  - `vibe-sentinel status --json` emits deterministic JSON after approval.
  - CLI parsing remains separate from status evaluation behavior.
  - JSON formatting is covered by unit and integration tests.
- Constraints:
  - Public CLI contract change requires explicit human approval.
  - Adding `serde` and `serde_json` requires explicit human approval.
  - No Rust implementation bodies before reviewed plan, reviewed architecture, skeleton/test updates, and passing skeleton-level tests.
- Non-goals:
  - TUI, MCP, deployment, credential, or network work.

### Research Notes

- Docs inspected:
  - `AGENTS.md`
  - `docs/harness/scope.md`
  - `docs/harness/operating-model.md`
  - `docs/app-specs/cli-status-slice.md`
  - `docs/app-specs/status-json-slice.md`
  - `docs/architecture.md`
  - `docs/tooling.md`
- Code inspected:
  - `Cargo.toml`
  - `src/main.rs`
  - `src/cli.rs`
  - `src/core.rs`
  - `src/domain.rs`
  - `tests/cli_status.rs`
- External references copied to `docs/references/`:
  - None.
- Findings:
  - The existing status slice cleanly separates parsing, evaluation, filesystem probing, and formatting.
  - `Cargo.toml` currently has no runtime dependencies.
  - JSON output is currently listed as a non-goal in `docs/app-specs/cli-status-slice.md`, so this must be a separate approved slice.

### Reviewed Plan

- Plan review status: pending
- Refinements made:
  - Drafted plan only; Rust behavior is blocked until approval.

### Architecture Pseudocode

```text
module domain
  enum ReadinessState
    derive Serialize after dependency approval
  struct StatusCheck
    derive Serialize after dependency approval
  struct StatusReport
    derive Serialize after dependency approval

module cli
  struct CliArgs
    field command: CliCommand
    field output_format: OutputFormat
  enum CliCommand
    variant Status
  enum OutputFormat
    variant Text
    variant Json
  fn parse_args<I, S>(args: I) -> Result<CliArgs, VibeError>
  fn execute_with_probe<P: WorkspaceProbe>(args: CliArgs, probe: P) -> Result<StatusReport, VibeError>
  fn format_status(report: &StatusReport) -> String
  fn format_status_json(report: &StatusReport) -> Result<String, VibeError>

file src/main.rs
  fn main() -> ExitCode
  fn run<I, S>(args: I) -> Result<String, VibeError>
```

### Reviewed Architecture

- Architecture review status: pending
- Refinements made:
  - Draft keeps status evaluation unchanged and adds output selection at the CLI surface.

### Skeleton Checklist

- [ ] Public CLI contract approved.
- [ ] Dependency decision approved or revised.
- [ ] `OutputFormat` skeleton added.
- [ ] `CliArgs` skeleton updated with output format.
- [ ] `format_status_json` skeleton added.
- [ ] `main::run` skeleton updated to choose text or JSON output.

### Mock Test Checklist

- [ ] `parse_args_accepts_status_json` covers JSON flag parsing.
- [ ] `parse_args_rejects_unknown_status_flag` covers invalid flags.
- [ ] `format_status_json_is_deterministic` covers structured output.
- [ ] `status_json_command_prints_workspace_status_json` covers binary JSON output.
- [ ] Existing text-output tests continue to pass unchanged.

### Implementation Checklist

- [ ] Record public CLI contract approval.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md`.
- [ ] Record dependency approval or revise no-dependency approach.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md`.
- [ ] Add skeletons and tests for JSON output.
- Validation after this unit: run focused parser/formatter tests.
- [ ] Fill one JSON implementation unit at a time.
- Validation after each unit: run focused tests before moving to the next unit.

### Validation Log

- 2026-05-06: Created pending active plan; Rust behavior not changed because approval is pending.

### Review Notes

- Diff review: pending
- Risks:
  - Public CLI contract and dependency changes are approval-required.
  - Manual JSON would be more error-prone than `serde_json` if dependencies are not approved.
- Follow-ups:
  - Ask human reviewer to approve or revise the proposed contract and dependency decision.

## Intended changes

- `Cargo.toml`: add `serde` and `serde_json` after approval.
- `src/domain.rs`: derive/implement serialization after approval.
- `src/cli.rs`: parse `--json` and format JSON after approval.
- `src/main.rs`: select output format after approval.
- `tests/cli_status.rs`: add JSON integration test after approval.
- `README.md`: document JSON mode after implementation.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md`: required before Rust implementation.
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- `cargo build --all-targets`
- `cargo run -- status`
- `cargo run -- status --json`

## Risks and rollback

- Risk: public contract proceeds without approval.
- Mitigation: block Rust behavior changes until approval is recorded.
- Rollback: keep this plan pending or delete it before implementation.
- Risk: dependency addition is not approved.
- Mitigation: revise plan before code changes.
- Rollback: leave `Cargo.toml` unchanged.

## Progress log

- 2026-05-06: Created pending active plan and status JSON spec.

## Decisions

- `status --json` is planned as a separate slice because JSON output was explicitly out of scope for the completed CLI status slice.
- Rust behavior changes are blocked until approval is explicit.