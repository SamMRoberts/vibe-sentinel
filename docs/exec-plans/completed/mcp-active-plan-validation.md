# Execution Plan: MCP Active Plan Validation

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-07
- Last updated: 2026-05-07

## Summary

Implement the next MCP execution slice for `vibe-sentinel`: a local, read-only
active-plan validation tool named `vibe_sentinel_validate_active_plans`. The
tool validates active execution plans for implementation readiness using narrow
semantic modified-TDD checks in the application core, then exposes structured
results through the existing local stdio MCP server.

## Scope

### In scope

- Add a local MCP tool named `vibe_sentinel_validate_active_plans`.
- Validate all non-README markdown plans in `docs/exec-plans/active/`.
- Reuse and extend existing `WorkspaceProbe`, `FsWorkspaceProbe`, and
  `FakeWorkspaceProbe` patterns for active-plan listing and file reads.
- Add application-core plan parsing and implementation-readiness rule checks.
- Add MCP request/response fixture tests for discovery, successful validation,
  empty active-plan state, invalid call arguments, and workspace read failures.
- Preserve existing CLI text, JSON, TUI, and MCP status behavior.
- Update docs and validation guidance for the shipped MCP validation slice.
- Clean up stale MCP status wording and completed-plan index drift discovered
  before this slice.

### Out of scope

- Remote MCP transport.
- Write-capable MCP tools, resources, prompts, or multi-tool workflows.
- Parsing architecture pseudocode symbols for one-to-one checklist coverage.
- Reconstructing exact chronological sequencing from prose validation logs.
- Inferring human approvers or review actors from freeform prose.
- Replacing the Python validation command as the repository's local
  plan-checking helper.
- Deployment, credentials, persistent storage, or destructive operations.
- Dependency swaps or runtime changes.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/app-specs/app-spec.md`
- `docs/app-specs/mcp-status-tool-slice.md`
- `docs/app-specs/mcp-active-plan-validation-slice.md`
- `docs/architecture.md`
- `docs/tooling.md`
- `docs/quality.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/review.md`
- `docs/exec-plans/tech-debt-tracker.md`
- `docs/exec-plans/completed/mcp-status-tool.md`

## Acceptance criteria

- `tools/list` advertises both `vibe_sentinel_status` and
  `vibe_sentinel_validate_active_plans` with read-only, idempotent, local
  metadata.
- `tools/call` for `vibe_sentinel_validate_active_plans` returns structured
  implementation-readiness data for all active non-README markdown plans.
- Plan validation logic lives in the application core and is reusable outside
  MCP.
- Workspace access stays behind `WorkspaceProbe` and concrete/fake adapters.
- Missing active plans, pending review statuses, missing skeleton/mock-test
  evidence, missing validation notes, and workspace read failures are reported
  deterministically.
- Existing `vibe_sentinel_status` and CLI/TUI status behavior remains
  compatible.
- Validation commands in this plan pass or skipped commands are recorded with
  risk.

## Modified TDD artifacts

### Feature Info

- Goal: Add a second local MCP tool so coding agents can validate active
  execution plans before implementation work proceeds.
- Acceptance criteria: The public contract in
  `docs/app-specs/mcp-active-plan-validation-slice.md` is implemented, existing
  status surfaces remain compatible, and fixture-backed MCP tests pass.
- Constraints: Preserve architecture layering; validate MCP payloads and file
  reads at boundaries; keep the tool read-only and local-only; do not replace
  the Python validation helper in this slice.
- Non-goals: Remote transport, write tools, resources, prompts, deployment,
  credentials, storage, pseudocode symbol coverage, exact chronology inference,
  or human-approver inference.

### Research Notes

- Docs inspected: `AGENTS.md`, `docs/harness/scope.md`,
  `docs/harness/operating-model.md`, `docs/app-specs/app-spec.md`,
  `docs/app-specs/index.md`, `docs/app-specs/mcp-status-tool-slice.md`,
  `docs/exec-plans/tech-debt-tracker.md`,
  `docs/exec-plans/completed/mcp-status-tool.md`, `docs/architecture.md`,
  `docs/tooling.md`, `docs/quality.md`, `docs/observability.md`, and
  `docs/exec-plans/plan-template.md`.
- Code inspected: `src/domain.rs`, `src/core.rs`, `src/ports.rs`,
  `src/adapters/fs.rs`, `src/adapters/test_support.rs`, `src/mcp.rs`,
  `src/lib.rs`, and `scripts/validate_tdd_workflow.py`.
- External references copied to `docs/references/`: none.
- Findings: The repository has no active execution plan before this slice. The
  first MCP status tool is archived and fixture-tested. The technical debt
  tracker names shallow TDD workflow validation as open debt. `ValidationIssue`
  exists but is too thin for MCP-facing evidence. `WorkspaceProbe` needs narrow
  active-plan listing and file-reading seams. The existing MCP protocol adapter
  can be extended with a small multi-tool dispatcher while keeping validation
  logic in the application core. Stale docs were found in the MCP app-spec
  index, MCP status spec wording, completed-plan README, and modified-TDD
  terminology in harness docs.

### Reviewed Plan

- Plan review status: reviewed
- Refinements made: Use the active-plan validation tool as the next MCP slice,
  not MCP resources/prompts. Keep rule scope to implementation-readiness checks
  and defer pseudocode symbol coverage, exact chronology inference, and Python
  validator replacement. Public MCP contract approval is recorded from the
  2026-05-07 instruction to start implementation.

### Architecture Pseudocode

List every planned module, struct, enum, trait, function, and method before
scaffolding code.

```text
module domain
  enum ValidationSeverity
    variant Error
    variant Warning
  enum ValidationState
    variant Ready
    variant Missing
  struct ValidationEvidence
    field section: String
    field line: Option<usize>
    field excerpt: String
  struct ValidationIssue
    field rule_id: String
    field severity: ValidationSeverity
    field message: String
    field evidence: Option<ValidationEvidence>
  struct ValidationCheck
    field rule_id: String
    field state: ValidationState
    field severity: ValidationSeverity
    field message: String
    field evidence: Option<ValidationEvidence>
  struct PlanValidationReport
    field path: String
    field ready: bool
    field checks: Vec<ValidationCheck>
    field issues: Vec<ValidationIssue>
  struct ActivePlansValidationReport
    field project_name: String
    field ready: bool
    field plans: Vec<PlanValidationReport>

module ports
  trait WorkspaceProbe
    fn exists(relative_path) -> Result<bool, VibeError>
    fn has_any_active_plan() -> Result<bool, VibeError>
    fn active_plan_paths() -> Result<Vec<String>, VibeError>
    fn read_text_file(relative_path) -> Result<String, VibeError>

module adapters::fs
  struct FsWorkspaceProbe
    field root: PathBuf
  impl WorkspaceProbe for FsWorkspaceProbe
    fn active_plan_paths() -> Result<Vec<String>, VibeError>
    fn read_text_file(relative_path) -> Result<String, VibeError>

module adapters::test_support
  struct FakeWorkspaceProbe
    field existing_paths: Vec<String>
    field has_active_plan: bool
    field text_files: Vec<(String, String)>
  impl FakeWorkspaceProbe
    fn with_text_file(relative_path, contents) -> Self
    fn with_active_plan_file(relative_path, contents) -> Self
  impl WorkspaceProbe for FakeWorkspaceProbe
    fn active_plan_paths() -> Result<Vec<String>, VibeError>
    fn read_text_file(relative_path) -> Result<String, VibeError>

module core
  struct PlanValidationService<P: WorkspaceProbe>
    field probe: P
  impl PlanValidationService<P>
    fn new(probe) -> Self
    fn evaluate_active_plans() -> Result<ActivePlansValidationReport, VibeError>
    fn evaluate_plan(path, text) -> PlanValidationReport
    fn parse_plan_document(path, text) -> ActivePlanDocument
    fn rule_reviewed_plan_not_pending(document) -> ValidationCheck
    fn rule_reviewed_architecture_not_pending(document) -> ValidationCheck
    fn rule_implementation_requires_skeletons(document) -> ValidationCheck
    fn rule_implementation_requires_mock_tests(document) -> ValidationCheck
    fn rule_checked_implementation_items_require_validation_notes(document) -> ValidationCheck
    fn rule_implementation_requires_validator_pass_log(document) -> ValidationCheck
    fn missing_check(rule_id, severity, message, evidence) -> ValidationCheck
    fn ready_check(rule_id, severity, message, evidence) -> ValidationCheck
  struct ActivePlanDocument
    field path: String
    field title: String
    field reviewed_plan_status: Option<SectionStatus>
    field reviewed_architecture_status: Option<SectionStatus>
    field skeleton_items: Vec<ChecklistItem>
    field mock_test_items: Vec<ChecklistItem>
    field implementation_items: Vec<ImplementationItem>
    field validation_log_entries: Vec<ValidationLogEntry>
  struct SectionStatus
    field value: String
    field line: usize
    field excerpt: String
  struct ChecklistItem
    field text: String
    field checked: bool
    field line: usize
  struct ImplementationItem
    field text: String
    field checked: bool
    field line: usize
    field validation_after_text: Option<String>
    field validation_after_line: Option<usize>
  struct ValidationLogEntry
    field text: String
    field line: usize
  enum PlanSection
    variant ReviewedPlan
    variant ReviewedArchitecture
    variant SkeletonChecklist
    variant MockTestChecklist
    variant ImplementationChecklist
    variant ValidationLog
    variant Other

module mcp
  const STATUS_TOOL_NAME: &str
  const ACTIVE_PLAN_VALIDATION_TOOL_NAME: &str
  struct McpToolDescriptor
    field name: String
    field description: String
    field read_only: bool
    field idempotent: bool
    field local_only: bool
  enum McpTool
    variant Status
    variant ActivePlanValidation
  struct McpActivePlanValidationResponse
    field project_name: String
    field ready: bool
    field plans: Vec<PlanValidationReport>
  fn active_plan_validation_tool_descriptor() -> McpToolDescriptor
  fn tool_descriptors() -> Vec<McpToolDescriptor>
  fn evaluate_active_plan_validation_tool<P: WorkspaceProbe>(probe) -> Result<McpActivePlanValidationResponse, VibeError>
  fn active_plan_validation_response_from_report(report) -> McpActivePlanValidationResponse
  fn tool_from_name(name) -> Option<McpTool>
  fn handle_tools_list(id) -> Result<JsonRpcResponse, VibeError>
  fn handle_tools_call(config, id, params) -> Result<JsonRpcResponse, VibeError>
  fn handle_status_tool_call(config) -> Result<Value, VibeError>
  fn handle_active_plan_validation_tool_call(config) -> Result<Value, VibeError>

module tests
  fn active_plan_validation_reports_no_active_plans()
  fn active_plan_validation_reports_ready_plan()
  fn active_plan_validation_reports_pending_review_statuses()
  fn active_plan_validation_requires_skeletons_before_implementation()
  fn active_plan_validation_requires_mock_tests_before_implementation()
  fn active_plan_validation_requires_validation_notes_for_checked_implementation_items()
  fn active_plan_validation_requires_tdd_validator_pass_log()
  fn fs_workspace_probe_lists_active_plan_files_in_deterministic_order()
  fn fs_workspace_probe_reads_text_files_under_workspace_root()
  fn fake_workspace_probe_reports_configured_plan_files()
  fn mcp_tools_list_includes_active_plan_validation_tool()
  fn session_handles_active_plan_validation_tool_call_request()
  fn session_maps_active_plan_validation_workspace_errors_to_tool_error_payload()
```

### Reviewed Architecture

- Architecture review status: reviewed
- Refinements made: Keep semantic plan parsing and rule evaluation in the core.
  Keep MCP limited to descriptor, dispatch, serialization, and error-payload
  mapping. Extend `WorkspaceProbe` narrowly for plan paths and text reads rather
  than introducing a generic filesystem abstraction.

### Skeleton Checklist

- [x] `domain::ValidationSeverity` skeleton added.
- [x] `domain::ValidationState` skeleton added.
- [x] `domain::ValidationEvidence` skeleton added.
- [x] `domain::ValidationIssue` skeleton expanded for rule, severity, and evidence.
- [x] `domain::ValidationCheck` skeleton added.
- [x] `domain::PlanValidationReport` skeleton added.
- [x] `domain::ActivePlansValidationReport` skeleton added.
- [x] `ports::WorkspaceProbe` active-plan path and text-read skeleton methods added.
- [x] `FsWorkspaceProbe` active-plan path and text-read skeleton methods added.
- [x] `FakeWorkspaceProbe` active-plan path and text-read skeleton methods added.
- [x] `core::PlanValidationService` skeleton added.
- [x] `core::ActivePlanDocument`, `SectionStatus`, `ChecklistItem`, `ImplementationItem`, `ValidationLogEntry`, and `PlanSection` skeletons added.
- [x] `PlanValidationService::evaluate_active_plans` skeleton added.
- [x] `PlanValidationService::evaluate_plan` skeleton added.
- [x] Plan document parsing and rule-function skeletons added.
- [x] `mcp::ACTIVE_PLAN_VALIDATION_TOOL_NAME` skeleton added.
- [x] `mcp::McpTool` skeleton added.
- [x] `mcp::McpActivePlanValidationResponse` skeleton added.
- [x] MCP descriptor, dispatcher, and active-plan tool-call skeletons added.

### Mock Test Checklist

- [x] `active_plan_validation_reports_no_active_plans` covers idle active-plan state.
- [x] `active_plan_validation_reports_ready_plan` covers a valid active plan fixture.
- [x] `active_plan_validation_reports_pending_review_statuses` covers review-gate failures.
- [x] `active_plan_validation_requires_skeletons_before_implementation` covers skeleton-before-implementation gating.
- [x] `active_plan_validation_requires_mock_tests_before_implementation` covers mock-test-before-implementation gating.
- [x] `active_plan_validation_requires_validation_notes_for_checked_implementation_items` covers implementation-unit validation evidence.
- [x] `active_plan_validation_requires_tdd_validator_pass_log` covers validation-log evidence.
- [x] `fs_workspace_probe_lists_active_plan_files_in_deterministic_order` covers active-plan discovery.
- [x] `fs_workspace_probe_reads_text_files_under_workspace_root` covers safe text reads.
- [x] `fake_workspace_probe_reports_configured_plan_files` covers fake-probe setup.
- [x] `mcp_tools_list_includes_active_plan_validation_tool` covers MCP discovery metadata.
- [x] `session_handles_active_plan_validation_tool_call_request` covers successful MCP response shape.
- [x] `session_maps_active_plan_validation_workspace_errors_to_tool_error_payload` covers deterministic error payloads.

### Implementation Checklist

- [x] Fill `WorkspaceProbe`, `FsWorkspaceProbe`, and `FakeWorkspaceProbe` active-plan listing and text-read behavior.
- Validation after this unit: `cargo test workspace_probe` passed.
- [x] Fill plan document parsing behavior in `PlanValidationService`.
- Validation after this unit: `cargo test active_plan_validation` passed after parser and rule implementation.
- [x] Fill implementation-readiness rule behavior in `PlanValidationService`.
- Validation after this unit: `cargo test active_plan_validation` passed.
- [x] Fill report aggregation behavior in `PlanValidationService::evaluate_active_plans`.
- Validation after this unit: `cargo test active_plan_validation` passed.
- [x] Fill MCP descriptor/list dispatcher behavior for multiple tools.
- Validation after this unit: `cargo test active_plan_validation` passed for descriptor and session tests.
- [x] Fill MCP active-plan validation tool-call behavior and error mapping.
- Validation after this unit: `cargo test active_plan_validation` passed; `cargo test mcp::tests` pending final validation.
- [x] Update README, tooling, observability, app-spec index, and tech-debt docs for the shipped MCP validation slice.
- Validation after this unit: docs review and full required validation commands pending final validation.

### Validation Log

- 2026-05-07: `cargo test mcp::tests` -> passed with 15 MCP tests before this slice began.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-active-plan-validation.md` -> passed after creating the active plan.
- 2026-05-07: `git --no-pager diff --check` -> passed after creating the active plan.
- 2026-05-07: `cargo test --all` -> passed after skeletons were added; warnings identified placeholder rule functions before implementation.
- 2026-05-07: `cargo test active_plan_validation` -> failed as intended against placeholder rule behavior with six failing core semantic tests.
- 2026-05-07: `cargo test active_plan_validation` -> passed after filling plan parsing and implementation-readiness rules.
- 2026-05-07: `cargo test workspace_probe` -> passed after active-plan listing and text-read behavior was filled.
- 2026-05-07: `cargo test mcp::tests` -> passed with 18 MCP tests after adding the active-plan validation tool.
- 2026-05-07: `cargo fmt`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, `cargo build --all-targets`, `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-active-plan-validation.md`, `python3 scripts/validate_tdd_workflow.py`, and `git --no-pager diff --check` -> passed.
- 2026-05-07: Subprocess MCP smoke against `target/debug/vibe-sentinel mcp serve` returned protocol version `2025-11-25`, tools `vibe_sentinel_status` and `vibe_sentinel_validate_active_plans`, one active plan, and `ready: true` for the active-plan validation tool.

### Review Notes

- Diff review: completed after formatting, lint, tests, build, docs validation,
  and local MCP subprocess smoke.
- Risks: External MCP inspector validation was not run; local fixture coverage
  and subprocess smoke exercised the stdio contract.
- Follow-ups: Consider hardening `scripts/validate_tdd_workflow.py` with the
  same semantic checks after this MCP core behavior ships; keep TUI lifecycle
  smoke testing as a separate non-MCP plan.

## Intended changes

- `docs/app-specs/mcp-active-plan-validation-slice.md`: define the new public
  MCP validation contract.
- `docs/app-specs/index.md`: link the new slice and mark MCP status as shipped.
- `docs/app-specs/mcp-status-tool-slice.md`: update shipped-status wording.
- `docs/harness/scope.md` and `docs/harness/operating-model.md`: align modified
  TDD terminology.
- `docs/exec-plans/completed/README.md`: add the missing harness
  operationalization plan entry.
- `docs/exec-plans/active/mcp-active-plan-validation.md`: track this modified
  TDD work.
- `src/domain.rs`: add serializable active-plan validation domain types.
- `src/core.rs`: add active-plan validation service, parser, and rules.
- `src/ports.rs`: add narrow active-plan listing and text-read seams.
- `src/adapters/fs.rs`: implement active-plan discovery and file reads.
- `src/adapters/test_support.rs`: extend fake probe for active-plan fixtures.
- `src/mcp.rs`: add the new tool descriptor, multi-tool dispatch, and fixture
  coverage.
- `docs/tooling.md`, `docs/observability.md`, `README.md`, and
  `docs/exec-plans/tech-debt-tracker.md`: update after behavior is implemented.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-active-plan-validation.md`: required before feature implementation.
- `cargo fmt --check`: required after Rust edits.
- `cargo clippy --all-targets --all-features -- -D warnings`: required after Rust edits.
- `cargo test mcp::tests`: required after MCP edits.
- `cargo test --all`: required after Rust edits.
- `cargo build --all-targets`: required after Rust edits.
- `python3 scripts/validate_tdd_workflow.py`: required before final review.
- `git --no-pager diff --check`: required before final review.
- MCP smoke or inspector validation: run if available; record skipped inspector
  validation as residual risk.

## Risks and rollback

- Risk: The new validation tool could freeze overly broad workflow semantics.
- Mitigation: Limit the first rule set to implementation-readiness checks named
  in this plan and defer more ambitious semantic validation.
- Rollback: Remove the new app spec, active plan, domain/core/port/adapter/MCP
  changes, and docs updates for this slice; the existing MCP status tool should
  remain untouched.
- Risk: Expanding `WorkspaceProbe` could become a generic filesystem API.
- Mitigation: Add only active-plan path listing and relative text reads required
  for this slice.
- Rollback: Revert `src/ports.rs`, `src/adapters/fs.rs`, and
  `src/adapters/test_support.rs` to the current status-only probe shape.

## Progress log

- 2026-05-07: Scope-gated the next MCP request as `NEEDS_PLAN`; public MCP
  contract approval captured from the user instruction to start implementation.
- 2026-05-07: Verified the shipped MCP status surface with `cargo test mcp::tests`.
- 2026-05-07: Created the active-plan validation slice spec and active execution
  plan; cleaned up stale MCP status/index wording discovered during exploration.
- 2026-05-07: Added active-plan validation domain types, core parser/rules,
  workspace probe extensions, fake/filesystem adapter support, MCP dispatch, and
  fixture coverage.
- 2026-05-07: Completed full validation and prepared the plan for archival.

## Decisions

- Tool name: `vibe_sentinel_validate_active_plans`.
- Tool input: no required arguments in this slice.
- Tool scope: all active non-README markdown plans in deterministic order.
- Rule scope: implementation readiness only.
- Python/Rust boundary: Rust owns MCP-facing semantic validation; Python remains
  a local validation command and is not invoked by the runtime.
- Public contract approval: captured from the 2026-05-07 instruction to start
  implementation after the next MCP slice plan was presented.