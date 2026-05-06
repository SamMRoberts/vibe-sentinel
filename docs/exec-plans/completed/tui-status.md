# Execution Plan: TUI Status View

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Added an approved read-only ratatui status view over the existing readiness report while preserving the completed text and JSON status behavior. The user approved continuing implementation, which covered the public TUI contract and the TUI dependency additions described in this plan.

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

- [x] Explicit human approval is recorded for the public TUI contract `vibe-sentinel status --tui` before Rust behavior changes.
- [x] Explicit human approval is recorded before adding `ratatui` or any runtime terminal backend dependency.
- [x] Existing `vibe-sentinel status` text output remains unchanged.
- [x] Existing `vibe-sentinel status --json` output remains unchanged.
- [x] TUI rendering uses the existing `StatusReport` and ordered checks from application core.
- [x] TUI render/state behavior is covered by non-interactive tests.
- [x] Full validation passes before completion.

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
  - `Cargo.toml` did not include `ratatui` or a terminal backend dependency before this slice.
  - The JSON status slice explicitly anticipated future TUI and MCP consumers.

### Reviewed Plan

- Plan review status: approved on 2026-05-06
- Refinements made:
  - User approved continuing implementation, approving the proposed public TUI contract `vibe-sentinel status --tui`.
  - User approved continuing implementation, approving the TUI dependency additions described by this plan.

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
    variant Escape
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

- Architecture review status: approved on 2026-05-06
- Refinements made:
  - Draft kept readiness evaluation in application core and added TUI as a surface over `StatusReport`.
  - User approved proceeding with this architecture after reviewing the plan gate.

### Skeleton Checklist

- [x] `OutputFormat::Tui` skeleton added with no behavior beyond parsed selection.
- [x] `StatusTuiModel` skeleton added with minimal state only.
- [x] `StatusTuiKey` skeleton added for normalized key events.
- [x] `StatusTuiView::render` skeleton added with render behavior isolated from the runtime terminal.
- [x] `render_status_to_buffer` skeleton added for non-interactive render tests.
- [x] `run_status_tui` skeleton added as the runtime boundary.
- [x] `main::run` skeleton updated to support output-producing and TUI-running paths.

### Mock Test Checklist

- [x] `parse_args_accepts_status_tui` covers the proposed TUI flag.
- [x] `parse_args_rejects_conflicting_status_output_flags` covers invalid output combinations.
- [x] `status_tui_model_quits_on_q_or_escape` covers state transitions without a terminal.
- [x] `status_tui_render_includes_project_ready_state_and_checks` covers render output using a test buffer.
- [x] Existing text-output and JSON-output tests continue to pass unchanged.

### Implementation Checklist

- [x] Record public TUI contract approval.
- Validation after this unit: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md` passed.
- [x] Record dependency approval.
- Validation after this unit: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md` passed.
- [x] Add skeletons and tests for TUI output.
- Validation after this unit: `cargo test status_tui` first failed on missing `tui` module and `OutputFormat::Tui`.
- [x] Fill one TUI implementation unit at a time.
- Validation after each unit: `cargo test status_tui` and `cargo test parse_args_rejects_conflicting_status_output_flags` passed.

### Validation Log

- 2026-05-06: Created active plan and TUI status slice spec; Rust behavior not changed because approval was pending.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md` passed.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py` passed.
- 2026-05-06: Requested explicit approval for `vibe-sentinel status --tui` and TUI dependencies; user was unavailable, so Rust behavior changes remained blocked.
- 2026-05-06: User approved continuing implementation, covering the `status --tui` public TUI contract and TUI dependency additions.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md` passed after approval was recorded.
- 2026-05-06: `cargo test status_tui` first failed on missing TUI skeleton/implementation symbols.
- 2026-05-06: `cargo test status_tui` passed after adding parser, TUI model, render buffer, and runtime boundary.
- 2026-05-06: `cargo test parse_args_rejects_conflicting_status_output_flags` passed.
- 2026-05-06: `cargo fmt --check` initially reported rustfmt layout changes; `cargo fmt` applied the required formatting.
- 2026-05-06: Full validation passed: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all && cargo build --all-targets && python3 scripts/validate_tdd_workflow.py && cargo run -- status && cargo run -- status --json`.
- 2026-05-06: TUI smoke launched into the alternate buffer with `cargo run -- status --tui`, but automated quit delivery through the terminal tool was unreliable; the run was interrupted and treated as tool-limited rather than pass/fail evidence.
- 2026-05-06: Post-archive validation passed: `python3 scripts/validate_tdd_workflow.py && cargo test --all && cargo run -- status --json`. The final smoke check reported `ready:false` because no active execution plan remains after archiving this completed slice.

### Review Notes

- Diff review: completed after full validation.
- Risks:
  - Automated interactive TUI smoke remains limited by the terminal tool; non-interactive render/state tests cover the core TUI behavior.
- Follow-ups:
  - Add a stronger automated TUI lifecycle smoke if future tooling supports sending raw terminal keys reliably.

## Intended changes

- `Cargo.toml`: added `ratatui` and `crossterm` after approval.
- `src/lib.rs`: exposed the new TUI module.
- `src/cli.rs`: parses `--tui` while preserving text and JSON behavior.
- `src/tui.rs`: added status TUI model, render helper, and runtime boundary.
- `src/main.rs`: launches TUI mode without printing normal status output.
- `tests/tui_status.rs`: added TUI render/state tests.
- `README.md`: documents TUI mode.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/tui-status.md`: passed before Rust implementation and after approval was recorded.
- `cargo fmt --check`: passed after `cargo fmt`.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --all`: passed.
- `cargo build --all-targets`: passed.
- `python3 scripts/validate_tdd_workflow.py`: passed.
- `cargo run -- status`: passed.
- `cargo run -- status --json`: passed.
- `cargo run -- status --tui`: launched into alternate buffer; automated quit delivery was tool-limited and the run was interrupted.
- Post-archive `python3 scripts/validate_tdd_workflow.py && cargo test --all && cargo run -- status --json`: passed; JSON output correctly reflected that no active plan remains.

## Risks and rollback

- Risk: TUI work changes existing text or JSON status behavior.
- Mitigation: preserved existing tests and ran focused integration tests for both existing modes.
- Rollback: revert TUI parsing, module, dependency, and runtime changes.
- Risk: TUI render tests depend on an interactive terminal.
- Mitigation: kept render/state logic isolated behind testable model and buffer helpers.
- Rollback: revise architecture before expanding TUI behavior.

## Progress log

- 2026-05-06: Created active plan and slice spec after harness status review recommended TUI as the next lowest-risk product slice.
- 2026-05-06: Validated active plan shape; Rust behavior remained blocked pending explicit approval of the public TUI contract and dependency decision.
- 2026-05-06: Paused before Rust changes because approval was not available.
- 2026-05-06: User approved the public TUI contract and TUI dependency additions.
- 2026-05-06: Implemented the TUI status slice and archived this plan after validation.

## Decisions

- `status --tui` was implemented as a narrow extension of the existing status command rather than a broad TUI application shell.
- `ratatui` and `crossterm` were added after explicit approval.
- TUI runtime lifecycle testing is currently covered by non-interactive state/render tests plus build validation; interactive quit smoke is noted as tool-limited.