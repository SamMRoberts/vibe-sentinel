# Execution Plan: Product Bootstrap

## Status

- State: active
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Bootstrap `vibe-sentinel` from a harness-only repository into a Rust product workspace under the modified TDD workflow. The first implementation slice is a narrow CLI status path that reports local harness and workspace readiness signals. The work proceeds through reviewed architecture pseudocode, mockable skeletons, skeleton-level tests, and one-unit-at-a-time implementation before product behavior is filled in.

## Scope

### In scope

- Define the first concrete Rust product vertical slice and acceptance criteria.
- Update app-spec or feature-spec documentation for the chosen slice.
- Design a Cargo workspace and module topology that preserves the documented layer order.
- Scaffold minimal Rust skeletons for domain types, service traits, application core, filesystem adapter, and CLI surface required by the first slice.
- Add skeleton-level tests, mocks, fakes, and fixtures before filling feature implementation bodies.
- Implement the chosen slice one planned skeleton unit at a time with validation after each unit.
- Update README, tooling notes, observability evidence, and plan logs once runnable workflows exist.

### Out of scope

- Defining or implementing broad CLI, TUI, or MCP product contracts beyond the first approved CLI status slice.
- Production credential changes, deployment changes, data migrations, or destructive operations.
- Dependency swaps or runtime/package-manager changes without explicit human approval.
- Filling feature implementation bodies before reviewed plan, reviewed architecture pseudocode, mockable skeletons, and passing skeleton-level tests exist.
- Non-Rust rewrites or replacing the CLI/TUI/MCP product shape.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/harness/initialization.md`
- `docs/README.md`
- `docs/app-specs/app-spec.md`
- `docs/app-specs/cli-status-slice.md`
- `docs/app-specs/index.md`
- `docs/architecture.md`
- `docs/tooling.md`
- `docs/quality.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/review.md`
- `docs/exec-plans/index.md`
- `docs/exec-plans/plan-template.md`

## Acceptance criteria

- A concrete first CLI status slice is documented with goal, acceptance criteria, constraints, non-goals, surfaces involved, and approval-required public contract decisions.
- The active plan passes `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` before product implementation begins.
- Architecture pseudocode names every planned module, struct, enum, trait, function, and method before Rust skeletons are added.
- Skeleton checklist and mock test checklist map one-to-one to planned units before implementation bodies are filled.
- Skeleton-level tests pass before any feature behavior is implemented.
- Each implementation unit is completed separately, with relevant validation recorded after that unit.
- Final validation results distinguish successful, failed, skipped, and unavailable commands.

## Modified TDD artifacts

### Feature Info

- Goal: bootstrap the first approved `vibe-sentinel` Rust product slice as a local CLI status command without bypassing the modified TDD workflow.
- Acceptance criteria:
  - Document the CLI status vertical slice before creating Rust skeletons.
  - Proposed public command is `vibe-sentinel status`, pending explicit human approval before skeleton creation.
  - Preserve the layer order: domain types -> service traits -> application core -> adapters -> CLI/TUI/MCP surfaces.
  - Keep command parsing separate from command execution.
  - Keep filesystem access isolated behind a mockable workspace-probe trait.
  - Keep CLI output deterministic and testable.
  - Validate untrusted input at boundaries and avoid hidden side effects.
- Constraints:
  - Public CLI, TUI, MCP, storage, or wire-contract decisions require explicit human approval before implementation.
  - Security model, dependency/runtime/package-manager, deployment, data migration, and scope/process changes require explicit human approval.
  - No feature implementation bodies before reviewed architecture, skeletons, and passing skeleton-level tests.
- Non-goals:
  - Build a full product surface in the first slice.
  - Add production deployment workflows.
  - Add credentials, data migrations, or destructive operations.
  - Choose broad dependency policy beyond what the first slice requires.

### Research Notes

- Docs inspected:
  - `AGENTS.md`
  - `docs/harness/scope.md`
  - `docs/harness/operating-model.md`
  - `docs/app-specs/app-spec.md`
  - `docs/app-specs/cli-status-slice.md`
  - `docs/app-specs/index.md`
  - `docs/architecture.md`
  - `docs/tooling.md`
  - `docs/quality.md`
  - `docs/security.md`
  - `docs/reliability.md`
  - `docs/observability.md`
  - `docs/review.md`
  - `docs/exec-plans/plan-template.md`
- Code inspected:
  - `scripts/validate_tdd_workflow.py`
  - Repository file search found no `Cargo.toml` files at plan creation time.
- External references copied to `docs/references/`:
  - None yet. Copy durable crate or protocol references before depending on them for architecture decisions.
- Findings:
  - The repository currently appears to contain the harness and validation script, but no Rust workspace.
  - The high-level product spec does not yet define broad commands, TUI workflows, MCP tools/resources, storage behavior, or data model.
  - The first proposed slice is documented in `docs/app-specs/cli-status-slice.md`.
  - `docs/tooling.md` lists Cargo validation commands that become runnable after workspace scaffolding exists.
  - `scripts/validate_tdd_workflow.py` verifies required plan headings but not review quality, pseudocode completeness, or validation evidence quality.

### Reviewed Plan

- Plan review status: approved on 2026-05-06
- Refinements made:
  - User selected product bootstrap as the target.
  - User selected defining the first slice during the plan rather than assuming CLI-first, TUI-first, or MCP-first implementation.
  - User approved this active plan and selected CLI status as the first vertical slice.
  - User approved the proposed public CLI contract `vibe-sentinel status` with deterministic text output.

### Architecture Pseudocode

This architecture pseudocode targets the CLI status slice. It must be reviewed before Rust skeleton files are created.

```text
workspace vibe-sentinel
  package vibe-sentinel
    binary vibe-sentinel
    library vibe_sentinel

file Cargo.toml
  workspace/package metadata for vibe-sentinel

file src/main.rs
  fn main() -> ExitCode

file src/lib.rs
  pub mod domain
  pub mod ports
  pub mod core
  pub mod adapters
  pub mod cli

module domain
  enum VibeError
    variant InvalidArguments(String)
    variant WorkspaceUnreadable(String)
    variant StatusEvaluationFailed(String)
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
  impl StatusReport
    fn is_ready(&self) -> bool
    fn check_count(&self) -> usize
  struct ValidationIssue
    field message: String

module ports
  trait WorkspaceProbe
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError>
    fn has_any_active_plan(&self) -> Result<bool, VibeError>

module core
  struct StatusService<P: WorkspaceProbe>
    field probe: P
  impl StatusService<P>
    fn new(probe: P) -> Self
    fn evaluate(&self) -> Result<StatusReport, VibeError>

module adapters::fs
  struct FsWorkspaceProbe
    field root: PathBuf
  impl FsWorkspaceProbe
    fn new(root: PathBuf) -> Self
  impl WorkspaceProbe for FsWorkspaceProbe
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError>
    fn has_any_active_plan(&self) -> Result<bool, VibeError>

module adapters::test_support
  struct FakeWorkspaceProbe
    field existing_paths: Vec<String>
    field has_active_plan: bool
  impl FakeWorkspaceProbe
    fn new() -> Self
    fn with_path(self, relative_path: &str) -> Self
    fn with_active_plan(self, has_active_plan: bool) -> Self
  impl WorkspaceProbe for FakeWorkspaceProbe
    fn exists(&self, relative_path: &str) -> Result<bool, VibeError>
    fn has_any_active_plan(&self) -> Result<bool, VibeError>

module cli
  struct CliArgs
    field command: CliCommand
  enum CliCommand
    variant Status
  fn parse_args<I, S>(args: I) -> Result<CliArgs, VibeError>
  fn execute_with_probe<P: WorkspaceProbe>(args: CliArgs, probe: P) -> Result<StatusReport, VibeError>
  fn format_status(report: &StatusReport) -> String
```

### Reviewed Architecture

- Architecture review status: approved on 2026-05-06
- Refinements made:
  - Topology narrowed to the CLI status slice only.
  - TUI and MCP surfaces are out of the first slice.
  - Filesystem access is isolated behind `WorkspaceProbe`.
  - User approved this architecture pseudocode for skeleton scaffolding.

### Skeleton Checklist

- [x] First vertical slice documented in `docs/app-specs/cli-status-slice.md`.
- [x] Proposed public CLI contract approved for skeleton creation.
- [x] Final architecture pseudocode reviewed.
- [x] `Cargo.toml` workspace skeleton added with no feature implementation bodies.
- [x] `src/main.rs` skeleton added.
- [x] `src/lib.rs` module skeleton added.
- [x] `src/domain.rs` skeleton added for `VibeError`, `ReadinessState`, `StatusCheck`, `StatusReport`, and `ValidationIssue`.
- [x] `src/ports.rs` skeleton added for `WorkspaceProbe`.
- [x] `src/core.rs` skeleton added for `StatusService`.
- [x] `src/adapters/mod.rs` skeleton added.
- [x] `src/adapters/fs.rs` skeleton added for `FsWorkspaceProbe`.
- [x] `src/adapters/test_support.rs` test skeleton added for `FakeWorkspaceProbe`.
- [x] `src/cli.rs` skeleton added for `CliArgs`, `CliCommand`, `parse_args`, `execute_with_probe`, and `format_status`.

### Mock Test Checklist

- [x] Plan validation test command passes for this active plan.
- [x] Cargo workspace builds with skeleton code only.
- [x] `status_report_readiness_reflects_checks` covers `StatusReport::is_ready` and `StatusReport::check_count`.
- [x] `fake_workspace_probe_reports_configured_paths` covers the test fake.
- [x] `status_service_uses_workspace_probe` covers application-core behavior through `WorkspaceProbe`.
- [x] `parse_args_skeleton_returns_status_command` covers the CLI parsing seam.
- [x] `parse_args_rejects_unknown_command` covers actionable invalid-argument errors.
- [x] `execute_with_probe_returns_status_report` covers CLI execution against a fake probe.
- [x] `format_status_is_deterministic` covers deterministic text output.
- [x] `status_command_prints_workspace_status` covers the binary entry point for the approved CLI status command.

### Implementation Checklist

- [x] Define and document the first vertical slice.
- Validation after this unit: pending rerun of `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` after this plan update.
- [ ] Approve proposed CLI contract and final architecture pseudocode.
- Validation after this unit: rerun `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` and manually verify every planned unit has checklist coverage.
- [x] Scaffold Cargo workspace and skeleton modules.
- Validation after this unit: `cargo fmt --check`, `cargo build --all-targets`, `cargo test --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` passed on 2026-05-06.
- [ ] Add skeleton-level tests and fixtures.
- Validation after this unit: run the smallest relevant `cargo test` command and record the result.
- [x] Fill first skeleton unit: `StatusService::evaluate`.
- Validation after this unit: `cargo test status_service` passed on 2026-05-06.
- [x] Fill second skeleton unit: `parse_args`.
- Validation after this unit: `cargo test parse_args` passed on 2026-05-06.
- [x] Fill third skeleton unit: `FsWorkspaceProbe::has_any_active_plan`.
- Validation after this unit: `cargo test fs_workspace_probe_detects_active_plan_files` passed on 2026-05-06.
- [x] Fill fourth skeleton unit: `format_status`.
- Validation after this unit: `cargo test format_status_is_deterministic` passed on 2026-05-06.
- [x] Fill fifth skeleton unit: `main` binary entry point.
- Validation after this unit: `cargo test status_command_prints_workspace_status` passed on 2026-05-06.
- [x] Continue one skeleton unit at a time until the approved slice is complete.
- Validation after each unit: record the focused command and result.

### Validation Log

- 2026-05-06: Created active plan; validation passed with `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md`.
- 2026-05-06: Updated plan for approved product-bootstrap target and selected CLI status slice; validation pending after this update.
- 2026-05-06: User approved CLI status public contract and architecture pseudocode for skeleton scaffolding.
- 2026-05-06: Added Cargo workspace and Rust skeleton modules; validation pending.
- 2026-05-06: `cargo fmt --check` failed before formatting; `cargo fmt` applied newline/formatting fixes.
- 2026-05-06: `cargo fmt --check` passed.
- 2026-05-06: `cargo build --all-targets` passed.
- 2026-05-06: `cargo test --all` passed: 6 tests passed.
- 2026-05-06: `cargo clippy --all-targets --all-features -- -D warnings` passed.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` passed.
- 2026-05-06: `cargo test status_service_uses_workspace_probe` failed before implementation as expected: skeleton report had 0 checks instead of 3.
- 2026-05-06: Implemented `StatusService::evaluate` against `WorkspaceProbe`; `cargo test status_service` passed with 2 tests.
- 2026-05-06: `cargo test parse_args_rejects_unknown_command` failed before implementation as expected: skeleton parser accepted `watch`.
- 2026-05-06: Implemented `parse_args`; `cargo test parse_args` passed with 2 tests.
- 2026-05-06: `cargo test fs_workspace_probe_detects_active_plan_files` failed before implementation as expected: active plan detection returned `false`.
- 2026-05-06: Implemented `FsWorkspaceProbe::has_any_active_plan`; `cargo test fs_workspace_probe_detects_active_plan_files` passed.
- 2026-05-06: `cargo test format_status_is_deterministic` failed before implementation as expected: formatter emitted only a check count.
- 2026-05-06: Implemented `format_status`; `cargo test format_status_is_deterministic` passed.
- 2026-05-06: `cargo test status_command_prints_workspace_status` failed before implementation as expected: binary emitted empty stdout.
- 2026-05-06: Implemented `main` entry point; `cargo test status_command_prints_workspace_status` passed.
- 2026-05-06: `cargo fmt --check` failed before final validation; `cargo fmt` applied formatting fixes.
- 2026-05-06: `cargo fmt --check` passed.
- 2026-05-06: `cargo clippy --all-targets --all-features -- -D warnings` passed.
- 2026-05-06: `cargo test --all` passed: 9 library tests and 1 integration test passed.
- 2026-05-06: `cargo build --all-targets` passed.
- 2026-05-06: `python3 scripts/validate_tdd_workflow.py` passed for 2 files.
- 2026-05-06: `cargo run -- status` passed and printed ready checks for harness docs, active plan, and Rust workspace.

### Review Notes

- Diff review: pending final local diff inspection
- Risks:
  - The exact public CLI contract is proposed but still requires human approval before skeleton creation.
  - Cargo validation commands are unavailable until workspace scaffolding exists.
  - Public contract choices require human approval before implementation.
  - The TDD validator checks required headings only, so plan and architecture review must still be done manually.
- Follow-ups:
  - Consider adding stronger TDD validator checks after the first slice proves the workflow.
  - Update `README.md` after the first runnable workflow exists.

## Intended changes

- `docs/app-specs/cli-status-slice.md`: document the first CLI status vertical slice.
- `docs/app-specs/index.md`: register the new feature spec.
- `docs/design-docs/`: add a design doc if workspace topology or public contracts need durable explanation.
- `docs/references/`: add durable external references if crate or protocol research is needed.
- `Cargo.toml`: add workspace metadata after plan and architecture review.
- Rust source tree: add CLI status skeleton modules after CLI contract and architecture review.
- `README.md`: document setup and commands once runnable workflows exist.
- `docs/exec-plans/active/product-bootstrap.md`: keep checklist, validation log, progress log, and decisions current throughout implementation.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md`: required before feature implementation.
- `cargo fmt --check`: required after Rust files exist.
- `cargo clippy --all-targets --all-features -- -D warnings`: required before final review after Rust files exist.
- `cargo test --all`: required before final review after Rust files exist.
- `cargo build --all-targets`: required after workspace scaffolding and before final review.

## Risks and rollback

- Risk: implementing product code before the first slice and architecture are reviewed would violate the harness.
- Mitigation: keep this plan active and stop at review gates until required sections are complete.
- Rollback: remove unreviewed product code and return to this active plan.
- Risk: public contracts are selected without approval.
- Mitigation: explicitly record proposed CLI/TUI/MCP/storage/wire contracts and ask for approval before implementation.
- Rollback: revise the spec and architecture before skeletons are created.
- Risk: dependencies are added prematurely.
- Mitigation: document dependency rationale in research notes and require approval for dependency/runtime changes.
- Rollback: remove dependency entries before scaffold validation.

## Progress log

- 2026-05-06: Created active product-bootstrap plan from harness docs and repository inspection.
- 2026-05-06: User approved the active plan and selected CLI status as the first slice.
- 2026-05-06: Added CLI status slice spec and narrowed architecture pseudocode to CLI-only status reporting.
- 2026-05-06: User approved CLI status contract and architecture pseudocode.
- 2026-05-06: Scaffolded Rust skeleton modules for the CLI status slice.
- 2026-05-06: Implemented first unit, `StatusService::evaluate`, after focused failing test.
- 2026-05-06: Implemented second unit, `parse_args`, after focused failing test.
- 2026-05-06: Implemented third unit, `FsWorkspaceProbe::has_any_active_plan`, after focused failing test.
- 2026-05-06: Implemented fourth unit, `format_status`, after focused failing test.
- 2026-05-06: Implemented fifth unit, `main`, after focused failing integration test.
- 2026-05-06: Updated `README.md` with the runnable CLI workflow and validation commands.
- 2026-05-06: Final validation passed for formatting, clippy, tests, build, TDD plan validation, and CLI smoke test.

## Decisions

- Product bootstrap is the first implementation track.
- The first concrete slice is CLI status reporting.
- No Rust implementation bodies will be added until reviewed architecture, mockable skeletons, and passing skeleton-level tests exist.
- Public CLI, TUI, MCP, storage, or wire-contract choices must be treated as approval-required before implementation.
