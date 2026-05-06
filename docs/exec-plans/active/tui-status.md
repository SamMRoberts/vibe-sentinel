# Execution Plan: TUI Status View

## Status

- State: active
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Add a proposed read-only ratatui status view over the existing readiness report while preserving the completed text and JSON status behavior. Rust behavior changes are blocked until the public TUI contract and any dependency additions receive explicit human approval.

## Scope

### In scope

- Propose `vibe-sentinel status --tui` as the next public TUI entrypoint.
- Add a minimal TUI surface that renders the existing `StatusReport`.
- Keep status evaluation in `StatusService` and reuse existing domain types.
- Add render/state tests that do not require an interactive terminal after approval.
- Preserve existing `status` and `status --json` behavior unchanged.

### Out of scope

- Multiple TUI screens or broad navigation.
- New readiness checks.
- MCP tools or resources.
- Network, credential, deployment, data migration, or destructive behavior.
- Hand-rolled terminal rendering if `ratatui` and runtime backend dependencies are approved.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/app-specs/app-spec.md`
- `docs/app-specs/cli-status-slice.md`
- `docs/app-specs/status-json-slice.md`
- `docs/app-specs/tui-status-slice.md`
- `docs/architecture.md`
- `docs/tooling.md`
- `docs/quality.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/review.md`
- `docs/exec-plans/plan-template.md`

## Acceptance criteria

- Explicit human approval is recorded for the public TUI contract `vibe-sentinel status --tui` before Rust behavior changes.
- Explicit human approval is recorded before adding `ratatui` or any runtime terminal backend dependency.
- Existing `vibe-sentinel status` text output remains unchanged.
- Existing `vibe-sentinel status --json` output remains unchanged.
- TUI rendering uses the existing `StatusReport` and ordered checks from application core.
- TUI render/state behavior is covered by non-interactive tests.
- Full validation passes before completion.

## Modified TDD artifacts

### Feature Info

- Goal: add a minimal read-only terminal UI for the existing status report without changing status evaluation or existing CLI outputs.
- Acceptance criteria:
  - `vibe-sentinel status` remains unchanged.
  - `vibe-sentinel status --json` remains unchanged.
  - `vibe-sentinel status --tui` renders the existing readiness report after approval.
  - TUI state and rendering are testable without a real terminal.
  - CLI parsing remains separate from status evaluation behavior.
- Constraints:
  - Public TUI contract change requires explicit human approval.
  - Adding `ratatui` and any terminal backend dependency requires explicit human approval.
  - No Rust implementation bodies before reviewed plan, reviewed architecture, skeletons, skeleton-level tests, and passing skeleton-level validation.
- Non-goals:
  - Broad TUI navigation, MCP, deployment, credential, network, or new readiness-check work.

### Research Notes

- Docs inspected:
  - `AGENTS.md`
  - `docs/harness/scope.md`
  - `docs/harness/operating-model.md`
  - `docs/app-specs/app-spec.md`
  - `docs/app-specs/cli-status-slice.md`
  - `docs/app-specs/status-json-slice.md`
  - `docs/app-specs/tui-status-slice.md`
  - `docs/architecture.md`
  - `docs/tooling.md`
  - `docs/quality.md`
  - `docs/security.md`
  - `docs/reliability.md`
  - `docs/observability.md`
- Code inspected:
  - `Cargo.toml`
  - `src/lib.rs`
  - `src/core.rs`
  - `src/domain.rs`
  - `src/ports.rs`
  - `src/cli.rs`
  - `tests/cli_status.rs`
- External references copied to `docs/references/`:
  - None.
- Findings:
  - `StatusService` already produces a surface-neutral `StatusReport`.
  - `StatusReport`, `StatusCheck`, and `ReadinessState` are the correct domain model for a TUI view.
  - Existing text and JSON status behavior is already covered by unit and integration tests.
  - `Cargo.toml` does not yet include `ratatui` or a terminal backend dependency.
  - The JSON status slice explicitly anticipated future TUI and MCP consumers.

### Reviewed Plan

- Plan review status: pending
- Refinements made:
  - Drafted plan only; Rust behavior is blocked until public contract and dependency approvals are recorded.

### Architecture Pseudocode

```text
module domain
  enum ReadinessState
    variant Ready
    variant Missing
  struct StatusCheck
    field name: String
    field state: ReadinessState
    field detail: String
  struct StatusReport
    field project_name: String
    field checks: Vec<StatusCheck>
    fn is_ready(&self) -> bool

module core
  struct StatusService<P: WorkspaceProbe>
    fn evaluate(&self) -> Result<StatusReport, VibeError>

module cli
  struct CliArgs
    field command: CliCommand
    field output_format: OutputFormat
  enum OutputFormat
    variant Text
    variant Json
    variant Tui
  fn parse_args<I, S>(args: I) -> Result<CliArgs, VibeError>
  fn render_status(args: &CliArgs, report: &StatusReport) -> Result<String, VibeError>

module tui
  struct StatusTuiModel
    field report: StatusReport
    field should_quit: bool
    fn new(report: StatusReport) -> Self
    fn handle_key(&mut self, key: StatusTuiKey)
    fn should_quit(&self) -> bool
  enum StatusTuiKey
    variant Quit
    variant Other
  struct StatusTuiView
    fn render(model: &StatusTuiModel, frame: &mut Frame)
  fn render_status_to_buffer(report: &StatusReport, area: Rect) -> Buffer
  fn run_status_tui(report: StatusReport) -> Result<(), VibeError>

file src/main.rs
  fn main() -> ExitCode
  fn run<I, S>(args: I) -> Result<Option<String>, VibeError>
```

### Reviewed Architecture

- Architecture review status: pending
- Refinements made:
  - Draft keeps readiness evaluation in application core and adds TUI as a surface over `StatusReport`.

### Skeleton Checklist

- [ ] `OutputFormat::Tui` skeleton added with no behavior beyond parsed selection.
- [ ] `StatusTuiModel` skeleton added with minimal state only.
- [ ] `StatusTuiKey` skeleton added for normalized key events.
- [ ] `StatusTuiView::render` skeleton added with placeholder render behavior only.
- [ ] `render_status_to_buffer` skeleton added for non-interactive render tests.
- [ ] `run_status_tui` skeleton added as the runtime boundary.
- [ ] `main::run` skeleton updated to support output-producing and TUI-running paths.

### Mock Test Checklist

- [ ] `parse_args_accepts_status_tui` covers the proposed TUI flag.
- [ ] `parse_args_rejects_conflicting_status_output_flags` covers invalid output combinations.
- [ ] `status_tui_model_quits_on_q_or_escape` covers state transitions without a terminal.
- [ ] `status_tui_render_includes_project_ready_state_and_checks` covers render output using a test buffer.
- [ ] Existing text-output and JSON-output tests continue to pass unchanged.

### Implementation Checklist

- [ ] Record public TUI contract approval.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md`.
- [ ] Record dependency approval or revise no-dependency approach.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md`.
- [ ] Add skeletons and tests for TUI output.
- Validation after this unit: run focused parser and TUI skeleton tests.
- [ ] Fill one TUI implementation unit at a time.
- Validation after each unit: run focused tests before moving to the next unit.

### Validation Log

- 2026-05-06: Created active plan and TUI status slice spec; Rust behavior not changed because approval is pending.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md` passed.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py` passed.
- 2026-05-06: Requested explicit approval for `vibe-sentinel status --tui` and TUI dependencies; user was unavailable, so Rust behavior changes remain blocked.

### Review Notes

- Diff review: pending
- Risks:
  - Public TUI contract and dependency changes are approval-required.
  - Introducing a runtime terminal backend can complicate deterministic tests if render/state boundaries are not kept isolated.
- Follow-ups:
  - Ask human reviewer to approve or revise the proposed contract and dependency decision.

## Intended changes

- `Cargo.toml`: add `ratatui` and any approved terminal backend dependency after approval.
- `src/lib.rs`: expose a new TUI module after approval.
- `src/cli.rs`: parse `--tui` after approval while preserving text and JSON behavior.
- `src/tui.rs` or `src/tui/`: add status TUI model, render, and runtime boundary after approval.
- `src/main.rs`: launch TUI mode after approval without printing normal status output.
- `tests/`: add TUI render/state tests after approval.
- `README.md`: document TUI mode after implementation.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md`: required before Rust implementation.
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- `cargo build --all-targets`
- `cargo run -- status`
- `cargo run -- status --json`
- Manual or automated TUI smoke check after runtime implementation.

## Risks and rollback

- Risk: TUI work changes existing text or JSON status behavior.
- Mitigation: preserve existing tests and run focused integration tests for both existing modes.
- Rollback: revert TUI parsing, module, dependency, and runtime changes.
- Risk: TUI render tests depend on an interactive terminal.
- Mitigation: keep render/state logic isolated behind testable model and buffer helpers.
- Rollback: revise architecture before filling implementation bodies.

## Progress log

- 2026-05-06: Created active plan and slice spec after harness status review recommended TUI as the next lowest-risk product slice.
- 2026-05-06: Validated active plan shape; Rust behavior remains blocked pending explicit approval of the public TUI contract and dependency decision.
- 2026-05-06: Paused before Rust changes because approval was not available.

## Decisions

- `status --tui` is proposed as a narrow extension of the existing status command rather than a broad TUI application shell.
- TUI implementation is blocked until the public contract and dependency approvals are explicit.