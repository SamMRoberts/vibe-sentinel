# Execution Plan: Status JSON Output

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Added an approved structured JSON output mode to the existing `vibe-sentinel status` command while preserving the current deterministic text output. The user approved starting implementation, which covered the public CLI contract and the `serde`/`serde_json` dependency decision described in this plan.

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

- [x] Explicit human approval is recorded for the public CLI contract `vibe-sentinel status --json` before Rust behavior changes.
- [x] Explicit human approval is recorded before adding `serde` and `serde_json`.
- [x] Existing text status output remains unchanged.
- [x] JSON output includes `project_name`, aggregate `ready`, and ordered `checks` with `name`, `state`, and `detail`.
- [x] JSON unit and integration tests pass.
- [x] Full validation passes before completion.

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
  - `Cargo.toml` previously had no runtime dependencies.
  - JSON output was listed as a non-goal in `docs/app-specs/cli-status-slice.md`, so this was handled as a separate approved slice.

### Reviewed Plan

- Plan review status: approved on 2026-05-06
- Refinements made:
  - User requested implementation start, approving the proposed public CLI contract `vibe-sentinel status --json`.
  - User requested implementation start, approving the `serde` and `serde_json` dependency addition described by this plan.

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

- Architecture review status: approved on 2026-05-06
- Refinements made:
  - Draft kept status evaluation unchanged and added output selection at the CLI surface.
  - User requested implementation start after reviewing harness status, approving this architecture for skeleton and test updates.

### Skeleton Checklist

- [x] Public CLI contract approved.
- [x] Dependency decision approved.
- [x] `OutputFormat` skeleton added.
- [x] `CliArgs` skeleton updated with output format.
- [x] `format_status_json` skeleton added.
- [x] `main::run` skeleton updated to choose text or JSON output.

### Mock Test Checklist

- [x] `parse_args_accepts_status_json` covers JSON flag parsing.
- [x] `parse_args_rejects_unknown_status_flag` covers invalid flags.
- [x] `format_status_json_is_deterministic` covers structured output.
- [x] `status_json_command_prints_workspace_status_json` covers binary JSON output.
- [x] Existing text-output tests continue to pass unchanged in focused and full validation.

### Implementation Checklist

- [x] Record public CLI contract approval.
- Validation after this unit: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md` passed.
- [x] Record dependency approval.
- Validation after this unit: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md` passed.
- [x] Add skeletons and tests for JSON output.
- Validation after this unit: `cargo test status_json` first failed on missing `OutputFormat`, `CliArgs.output_format`, and `format_status_json`.
- [x] Fill one JSON implementation unit at a time.
- Validation after each unit: `cargo test status_json` and `cargo test parse_args_rejects_unknown_status_flag` passed.

### Validation Log

- 2026-05-06: Created pending active plan; Rust behavior not changed because approval was pending.
- 2026-05-06: User requested implementation start, approving the `status --json` public CLI contract and `serde`/`serde_json` dependency addition.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md` passed after approval was recorded.
- 2026-05-06: `cargo test status_json` first failed on missing JSON skeleton/implementation symbols.
- 2026-05-06: `cargo test status_json` passed after adding JSON parser, formatter, and binary output selection.
- 2026-05-06: `cargo test parse_args_rejects_unknown_status_flag` passed.
- 2026-05-06: `cargo fmt --check` initially reported formatting drift; `cargo fmt` applied the required formatting.
- 2026-05-06: Full validation passed: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all && cargo build --all-targets && python3 scripts/validate_tdd_workflow.py && cargo run -- status && cargo run -- status --json`.
- 2026-05-06: Post-archive validation passed: `python3 scripts/validate_tdd_workflow.py && cargo run -- status --json`. The final smoke check reported `ready:false` because no active execution plan remains after archiving this completed slice.

### Review Notes

- Diff review: completed after full validation.
- Risks:
  - JSON output could accidentally drift from the documented stable field order.
  - Text output must remain byte-for-byte unchanged.
- Follow-ups:
  - None for this slice.

## Intended changes

- `Cargo.toml`: added `serde` and `serde_json` after approval.
- `src/domain.rs`: derived serialization for status domain output types.
- `src/cli.rs`: parses `--json` and formats deterministic JSON.
- `src/main.rs`: selects text or JSON output by parsed output format.
- `tests/cli_status.rs`: added JSON integration test.
- `README.md`: documents JSON mode.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/status-json.md`: passed before Rust implementation.
- `cargo fmt --check`: passed after `cargo fmt`.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --all`: passed.
- `cargo build --all-targets`: passed.
- `python3 scripts/validate_tdd_workflow.py`: passed.
- `cargo run -- status`: passed.
- `cargo run -- status --json`: passed.
- Post-archive `python3 scripts/validate_tdd_workflow.py && cargo run -- status --json`: passed; JSON output correctly reflected that no active plan remains.

## Risks and rollback

- Risk: JSON output changes existing text status behavior.
- Mitigation: preserved existing text tests and ran focused integration tests for both modes.
- Rollback: revert JSON-specific parsing, serialization, and dependency changes.

## Progress log

- 2026-05-06: Created pending active plan and status JSON spec.
- 2026-05-06: User approved implementation; plan and architecture marked active/approved.
- 2026-05-06: Added skeleton tests, observed expected missing-symbol failure, then implemented JSON mode and focused tests passed.
- 2026-05-06: Full validation passed and plan was archived to completed.

## Decisions

- `status --json` was implemented as a separate slice because JSON output was explicitly out of scope for the completed CLI status slice.
- Rust behavior changes proceeded under the active plan after tests were added first.