# Execution Plan: Post-TUI Cleanup

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Clean up post-TUI drift after the completed text, JSON, and TUI status slices. This slice fixes stale CLI guidance, tightens a misleading TUI string-rendering path, improves TUI startup cleanup reliability, and aligns the shipped status documentation and technical-debt tracker without adding new product behavior.

## Scope

### In scope

- Correct stale `status` too-many-arguments guidance so it mentions both `--json` and `--tui`.
- Tighten `render_status` so `OutputFormat::Tui` does not silently render an empty string.
- Fix TUI startup cleanup when setup fails after entering the alternate screen.
- Add focused parser and TUI lifecycle regression tests before implementation.
- Update status slice specs from proposal wording to shipped/current wording.
- Clarify that missing active plans are expected when no implementation slice is underway.
- Record known validator and TUI lifecycle follow-ups in the technical debt tracker.

### Out of scope

- New CLI, JSON, TUI, or MCP features.
- New readiness checks or changed readiness semantics.
- New dependencies, dependency swaps, or runtime changes.
- TUI navigation, layout redesign, or changed quit behavior.
- Network, credential, deployment, data migration, or destructive behavior.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
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
- `docs/exec-plans/tech-debt-tracker.md`

## Acceptance criteria

- Explicit human approval is recorded before changing public CLI error text.
- Explicit human approval is recorded before tightening the public `render_status` behavior for TUI output format.
- Too-many-arguments parser errors mention both optional output flags.
- `render_status` returns an actionable error for TUI output format misuse.
- TUI setup cleanup attempts to leave the alternate screen if setup fails after entering it.
- App specs describe shipped status slices rather than pending proposals.
- Technical-debt tracker records known TDD-validator and TUI lifecycle follow-ups.
- Full validation passes before completion.

## Modified TDD artifacts

### Feature Info

- Goal: close post-TUI cleanup drift without adding new status behavior.
- Acceptance criteria:
  - Parser guidance for too many status arguments mentions `--json` and `--tui`.
  - `render_status` does not silently produce empty TUI output.
  - TUI startup cleanup covers partial setup after alternate-screen entry.
  - Shipped status specs and the debt tracker reflect current state.
  - Existing text, JSON, and TUI behavior remains unchanged.
- Constraints:
  - Public CLI error text and public library behavior tightening require explicit approval.
  - Reliability-sensitive TUI changes require a plan and focused regression tests.
  - No new dependencies or product-surface changes.
- Non-goals:
  - MCP, new readiness checks, TUI redesign, deployment, credential, network, or destructive work.

### Research Notes

- Docs inspected:
  - `AGENTS.md`
  - `docs/harness/scope.md`
  - `docs/harness/operating-model.md`
  - `docs/app-specs/cli-status-slice.md`
  - `docs/app-specs/status-json-slice.md`
  - `docs/app-specs/tui-status-slice.md`
  - `docs/exec-plans/tech-debt-tracker.md`
  - `docs/exec-plans/active/README.md`
  - `README.md`
- Code inspected:
  - `src/cli.rs`
  - `src/main.rs`
  - `src/tui.rs`
  - `tests/tui_status.rs`
- External references copied to `docs/references/`:
  - None.
- Findings:
  - `parse_args` accepts `--tui`, but the too-many-arguments branch still mentions only `--json`.
  - `render_status` returns an empty string for `OutputFormat::Tui`, even though `main` handles TUI mode separately.
  - `TerminalGuard::enter` disables raw mode after `Terminal::new` failure but does not leave the alternate screen if it was already entered.
  - Status app specs still use proposal wording even though the slices are shipped.
  - The technical debt tracker has no concrete rows for known validator and TUI lifecycle follow-ups.

### Reviewed Plan

- Plan review status: approved on 2026-05-06
- Refinements made:
  - User requested implementation start after reviewing the cleanup recommendations.
  - This approval covers correcting public CLI error text and tightening the `render_status` TUI behavior as described by this plan.

### Architecture Pseudocode

```text
module cli
  const STATUS_OUTPUT_FLAG_USAGE: &str
  struct CliArgs
    field command: CliCommand
    field output_format: OutputFormat
  enum OutputFormat
    variant Text
    variant Json
    variant Tui
  fn parse_args<I, S>(args: I) -> Result<CliArgs, VibeError>
  fn render_status(args: &CliArgs, report: &StatusReport) -> Result<String, VibeError>
  fn format_status(report: &StatusReport) -> String
  fn format_status_json(report: &StatusReport) -> Result<String, VibeError>

module tui
  struct StatusTuiModel
    fn handle_key(&mut self, key: StatusTuiKey)
    fn should_quit(&self) -> bool
  enum StatusTuiKey
    variant Quit
    variant Escape
    variant Other
  struct StatusTuiView
    fn render(model: &StatusTuiModel, frame: &mut Frame)
  struct TerminalGuard
    field terminal: Terminal<CrosstermBackend<io::Stdout>>
    fn enter() -> Result<Self, VibeError>
  struct TerminalSetupState
    field raw_mode_enabled: bool
    field alternate_screen_entered: bool
    fn mark_raw_mode_enabled(&mut self)
    fn mark_alternate_screen_entered(&mut self)
    fn cleanup_partial<W, D>(&self, writer: &mut W, disable_raw_mode: D)
  fn run_status_tui(report: StatusReport) -> Result<(), VibeError>
```

### Reviewed Architecture

- Architecture review status: approved on 2026-05-06
- Refinements made:
  - Keep changes localized to CLI parsing/rendering and TUI terminal lifecycle boundaries.
  - Preserve existing application core, domain model, and output surfaces.

### Skeleton Checklist

- [x] `STATUS_OUTPUT_FLAG_USAGE` skeleton added for shared parser guidance.
- [x] `render_status` TUI misuse error path skeleton added.
- [x] `TerminalSetupState` skeleton added for partial setup cleanup tracking.
- [x] `TerminalSetupState::cleanup_partial` skeleton added with testable writer/raw-mode seams.

### Mock Test Checklist

- [x] `parse_args_rejects_status_with_too_many_arguments_listing_json_and_tui` covers stale parser branch.
- [x] `render_status_rejects_tui_output_format` covers silent empty string drift.
- [x] `terminal_setup_state_leaves_alternate_screen_after_partial_setup` covers TUI cleanup sequencing.
- [x] Existing text-output, JSON-output, and TUI render/state tests continue to pass.

### Implementation Checklist

- [x] Record CLI error text approval.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/post-tui-cleanup.md`.
- [x] Record `render_status` TUI behavior approval.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/post-tui-cleanup.md`.
- [x] Add focused failing regression tests.
- Validation after this unit: run focused parser/TUI lifecycle tests.
- [x] Fill CLI guidance and `render_status` cleanup.
- Validation after this unit: run focused CLI tests.
- [x] Fill TUI setup cleanup.
- Validation after this unit: run focused TUI lifecycle tests.
- [x] Update shipped app-spec wording and tech-debt tracker.
- Validation after this unit: run plan validator and relevant searches.

### Validation Log

- 2026-05-06: Created active post-TUI cleanup plan; user approval recorded for the public CLI stderr wording and public `render_status` behavior tightening.
- 2026-05-06: `cargo test parse_args_rejects_status_with_too_many_arguments_listing_json_and_tui render_status_rejects_tui_output_format terminal_setup_state_leaves_alternate_screen_after_partial_setup` was invalid because `cargo test` accepts one name filter.
- 2026-05-06: `cargo test tui_output_format` first failed because `TerminalSetupState` and the `render_status` test import were missing.
- 2026-05-06: `cargo test tui_output_format` passed after implementation.
- 2026-05-06: `cargo test too_many_arguments` passed after implementation.
- 2026-05-06: `cargo test partial_setup` passed after implementation.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/post-tui-cleanup.md` passed after docs updates.
- 2026-05-06: `cargo fmt --check` first found one formatting diff in `src/cli.rs`; `cargo fmt` applied the mechanical fix.
- 2026-05-06: `cargo fmt --check` passed after formatting.
- 2026-05-06: `cargo clippy --all-targets --all-features -- -D warnings` passed.
- 2026-05-06: `cargo test --all` passed with 21 total tests across unit, integration, and doc-test targets.
- 2026-05-06: `cargo build --all-targets` passed.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py` passed for 2 plan files.
- 2026-05-06: `cargo run -- status` passed and reported all checks ready while this active plan exists.
- 2026-05-06: `cargo run -- status --json` passed and reported all checks ready while this active plan exists.
- 2026-05-06: Interactive `cargo run -- status --tui` was not run in this slice; TUI coverage is deterministic model/render tests plus setup cleanup unit coverage.
- 2026-05-06: Final TUI refinement made partial cleanup reachable for terminal-clear failure after alternate-screen entry.
- 2026-05-06: Final post-archive validation passed: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, `cargo build --all-targets`, and `python3 scripts/validate_tdd_workflow.py`.
- 2026-05-06: Final post-archive smoke passed: `cargo run -- status` and `cargo run -- status --json` reported the expected idle state with no active plan.

### Review Notes

- Diff review: complete after full non-interactive validation
- Risks:
  - TUI lifecycle cleanup should not change normal TUI startup or quit behavior.
  - Parser copy should remain consistent across unknown, conflicting, and too-many-argument branches.
- Follow-ups:
  - MCP status should remain the next product slice after cleanup.

## Intended changes

- `src/cli.rs`: update parser guidance and make TUI string rendering fail explicitly.
- `src/tui.rs`: add partial setup cleanup tracking and test seam.
- `tests/tui_status.rs` or `src/tui.rs`: add TUI lifecycle regression coverage.
- `docs/app-specs/index.md`: describe shipped status slices.
- `docs/app-specs/cli-status-slice.md`: update proposal wording and clarify active-plan idle state.
- `docs/app-specs/status-json-slice.md`: update proposal/dependency wording to shipped/current state.
- `docs/app-specs/tui-status-slice.md`: update proposal/dependency wording to shipped/current state.
- `docs/exec-plans/tech-debt-tracker.md`: record known follow-ups.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/post-tui-cleanup.md`: required before implementation.
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- `cargo build --all-targets`
- `python3 scripts/validate_tdd_workflow.py`
- `cargo run -- status`
- `cargo run -- status --json`

## Risks and rollback

- Risk: parser changes alter existing successful status modes.
- Mitigation: preserve existing focused and integration tests for text, JSON, and TUI parsing.
- Rollback: revert parser copy and `render_status` TUI error branch.
- Risk: TUI cleanup seam complicates runtime setup.
- Mitigation: keep setup state private and limited to partial cleanup decisions.
- Rollback: revert TUI lifecycle helper changes.

## Progress log

- 2026-05-06: Created active plan after harness review identified post-TUI cleanup as the next step.
- 2026-05-06: Added focused regression tests, implemented CLI/TUI cleanup, and aligned shipped app specs plus the technical-debt tracker.
- 2026-05-06: Archived plan after full non-interactive validation passed.

## Decisions

- Treat user request to start implementation as approval for the public CLI error text correction and public `render_status` behavior tightening described in this plan.
- Keep this slice as cleanup only; MCP remains the next product feature after cleanup.