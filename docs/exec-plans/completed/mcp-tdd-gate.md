# Execution Plan: MCP TDD Gate Tool

## Status

- State: active
- Owner: coding agent
- Created: 2026-05-07
- Last updated: 2026-05-07

## Summary

Add a read-only local MCP tool, `vibe_sentinel_tdd_gate`, that lets coding
agents ask whether a proposed next modified-TDD workflow transition is currently
allowed for the active execution plans. The tool must preserve existing MCP
transport behavior, keep protocol handling thin, and reuse application-core plan
validation semantics.

## Scope

### In scope

- Add the `vibe_sentinel_tdd_gate` MCP tool to the existing local stdio server.
- Add strict object argument validation for a required `next_action` enum.
- Add domain and core types for TDD gate action, phase, and report data.
- Reuse active-plan validation semantics to make deterministic gate decisions.
- Add core tests with fake workspace fixtures.
- Add MCP request/response fixture coverage for tool metadata, success, invalid
  arguments, workspace errors, and session survival.
- Update app specs and execution-plan documentation for the new public MCP
  contract.

### Out of scope

- Remote MCP transport.
- Write-capable MCP tools or plan-file mutation.
- MCP resources, prompts, or multi-tool workflows.
- Dependency changes.
- Replacing `python3 scripts/validate_tdd_workflow.py`.
- Deep chronology reconstruction, approver inference, or pseudocode-to-checklist
  one-to-one coverage.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/exec-plans/plan-template.md`
- `docs/app-specs/index.md`
- `docs/app-specs/mcp-active-plan-validation-slice.md`
- `docs/app-specs/mcp-tdd-gate-slice.md`
- `docs/architecture.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/quality.md`
- `docs/tooling.md`

## Acceptance criteria

- `tools/list` advertises `vibe_sentinel_tdd_gate` as read-only, idempotent,
  local-only, non-destructive, and closed-world.
- `vibe_sentinel_tdd_gate` requires object arguments with a valid `next_action`.
- Invalid tool arguments return deterministic JSON-RPC `-32602` responses and do
  not abort the stdio session.
- The tool delegates gate decisions to application-core logic instead of parsing
  plan semantics in `src/mcp.rs`.
- The tool returns structured gate data with `project_name`, `allowed`,
  `current_phase`, `blocking_issues`, `warnings`, and `next_allowed_actions`.
- Idle active-plan state is reported as idle and blocks feature-work transitions.
- Existing MCP tools and status behavior remain compatible.
- Validation commands listed in `docs/tooling.md` are run or explicitly reported
  if unavailable.

## Modified TDD artifacts

### Feature Info

- Goal: Add a read-only MCP TDD gate tool that helps agents enforce the modified
  TDD sequence before moving to the next workflow phase.
- Acceptance criteria: Match the acceptance criteria in this plan and
  `docs/app-specs/mcp-tdd-gate-slice.md`.
- Constraints: Keep MCP protocol handling thin; preserve architecture layer
  direction; do not add dependencies; do not mutate workspace files; validate MCP
  payloads at the boundary; preserve idle-state semantics.
- Non-goals: Write-capable tools, remote transport, resources, prompts,
  approver inference, chronology reconstruction, and replacing the Python TDD
  validator.

### Research Notes

- Docs inspected: `AGENTS.md`, `docs/harness/scope.md`,
  `docs/harness/operating-model.md`, `docs/exec-plans/plan-template.md`,
  `docs/app-specs/index.md`, `docs/app-specs/mcp-active-plan-validation-slice.md`,
  `docs/architecture.md`, `docs/security.md`, `docs/reliability.md`,
  `docs/observability.md`, `docs/quality.md`, and `docs/tooling.md`.
- Code inspected: `src/domain.rs`, `src/core.rs`, `src/mcp.rs`, `src/ports.rs`,
  and `src/adapters/test_support.rs`.
- External references copied to `docs/references/`: none.
- Findings: Existing `PlanValidationService` already parses active plans and
  checks reviewed plan status, reviewed architecture status, skeleton evidence,
  mock-test evidence, validation notes, and TDD validator pass logs. Existing MCP
  tools are local read-only stdio tools with strict argument validation and
  structured content responses. The new tool can reuse these patterns without a
  new workspace port.

### Reviewed Plan

- Plan review status: reviewed
- Refinements made: Classified as `NEEDS_PLAN` with explicit public MCP contract
  approval from the 2026-05-07 user instruction to start implementation. Kept
  the first slice read-only and deferred write-capable workflow tools.

### Architecture Pseudocode

List every planned module, struct, enum, trait, function, and method before
scaffolding code.

```text
module src/domain.rs
  enum TddGateAction
    variant StartArchitecture
    variant StartSkeletons
    variant StartMockTests
    variant StartImplementation
    variant CompletePlan
  impl TddGateAction
    fn as_str(&self) -> &'static str
  enum TddWorkflowPhase
    variant Idle
    variant PlanCreated
    variant PlanReviewed
    variant ArchitectureReviewed
    variant ImplementationReady
    variant ImplementationUnderway
    variant CompleteReady
  struct TddGateReport
    field project_name: String
    field allowed: bool
    field current_phase: TddWorkflowPhase
    field blocking_issues: Vec<ValidationIssue>
    field warnings: Vec<ValidationIssue>
    field next_allowed_actions: Vec<TddGateAction>

module src/core.rs
  struct TddGateService<P: WorkspaceProbe>
    field probe: P
  impl TddGateService<P>
    fn new(probe: P) -> Self
    fn evaluate(&self, next_action: TddGateAction) -> Result<TddGateReport, VibeError>
    fn report_from_validation(report: ActivePlansValidationReport, next_action: TddGateAction) -> TddGateReport
    fn current_phase(report: &ActivePlansValidationReport) -> TddWorkflowPhase
    fn next_allowed_actions(phase: TddWorkflowPhase) -> Vec<TddGateAction>
    fn blocking_issues_for_action(report: &ActivePlansValidationReport, next_action: TddGateAction, phase: TddWorkflowPhase) -> Vec<ValidationIssue>
    fn implementation_evidence_present(report: &ActivePlansValidationReport) -> bool
    fn issue(rule_id: &str, message: &str) -> ValidationIssue

module src/mcp.rs
  const TDD_GATE_TOOL_NAME: &str
  enum McpTool
    variant TddGate
  enum McpToolInputSchema
    variant TddGate
  struct TddGateArguments
    field next_action: TddGateAction
  struct McpTddGateResponse
    field project_name: String
    field allowed: bool
    field current_phase: TddWorkflowPhase
    field blocking_issues: Vec<ValidationIssue>
    field warnings: Vec<ValidationIssue>
    field next_allowed_actions: Vec<TddGateAction>
  fn tdd_gate_tool_descriptor() -> McpToolDescriptor
  fn evaluate_tdd_gate_tool<P: WorkspaceProbe>(probe: P, next_action: TddGateAction) -> Result<McpTddGateResponse, VibeError>
  fn tdd_gate_response_from_report(report: TddGateReport) -> McpTddGateResponse
  fn parse_tdd_gate_arguments(id: Option<Value>, tool_name: &str, arguments: Option<Value>) -> ToolCallValidationResult<TddGateArguments>
  fn handle_tdd_gate_tool_call(config: &McpServerConfig, next_action: TddGateAction) -> Result<Value, VibeError>
```

### Reviewed Architecture

- Architecture review status: reviewed
- Refinements made: No new workspace port is needed for this slice because the
  gate can call `PlanValidationService::evaluate_active_plans`. Kept enum input
  parsing at the MCP boundary and phase/gate semantics in the core service.

### Skeleton Checklist

- [x] `domain::TddGateAction` skeleton added with serialization shape only
- [x] `domain::TddWorkflowPhase` skeleton added with serialization shape only
- [x] `domain::TddGateReport` skeleton added as a data container only
- [x] `core::TddGateService::new` skeleton added
- [x] `core::TddGateService::evaluate` skeleton added with minimal mockable behavior
- [x] `mcp::tdd_gate_tool_descriptor` skeleton added
- [x] `mcp::evaluate_tdd_gate_tool` skeleton added
- [x] `mcp::parse_tdd_gate_arguments` skeleton added
- [x] `mcp::handle_tdd_gate_tool_call` skeleton added

### Mock Test Checklist

- [x] `domain::tests::tdd_gate_action_serializes_as_mcp_argument_value` covers action serialization shape
- [x] `domain::tests::tdd_workflow_phase_serializes_as_structured_content_value` covers phase serialization shape
- [x] `tdd_gate_blocks_idle_workspace_actions` covers idle gate behavior with a fake workspace
- [x] `tdd_gate_allows_architecture_when_plan_exists` covers active-plan gate behavior with a fake workspace
- [x] `tdd_gate_blocks_implementation_until_validation_ready` covers semantic readiness issues with fixtures
- [x] `tdd_gate_allows_implementation_when_active_plan_validation_is_ready` covers ready implementation gate fixtures
- [x] `tdd_gate_tool_descriptor_is_read_only_idempotent_and_local` covers descriptor metadata
- [x] `tools_list_uses_descriptor_input_schema_for_current_tools` covers MCP tool discovery and strict input schema
- [x] `session_handles_tdd_gate_tool_call_request` covers successful MCP tool call fixture
- [x] `session_rejects_tdd_gate_tool_call_with_invalid_next_action_without_aborting` covers invalid enum args without aborting
- [x] `session_maps_tdd_gate_workspace_errors_to_tool_error_payload` covers workspace read errors

### Implementation Checklist

- [x] Add domain TDD gate data types.
- Validation after this unit: `cargo test domain::tests` passed.
- [x] Add core TDD gate service semantics.
- Validation after this unit: `cargo test core::tests::tdd_gate` passed after tightening implementation evidence detection.
- [x] Add MCP TDD gate argument parsing, descriptor, handler, and protocol tests.
- Validation after this unit: `cargo test mcp::tests` passed.
- [x] Update execution-plan validation evidence and review notes.
- Validation after this unit: full validation suite passed.

### Validation Log

- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-tdd-gate.md` -> passed.
- 2026-05-07: `vibe_sentinel_validate_active_plans` -> passed; active plan ready.
- 2026-05-07: `cargo test domain::tests` -> passed.
- 2026-05-07: `cargo test core::tests::tdd_gate` -> failed; validator-log evidence alone was incorrectly treated as checked implementation evidence.
- 2026-05-07: `cargo test core::tests::tdd_gate` -> passed after using checked implementation evidence only.
- 2026-05-07: `cargo test mcp::tests` -> passed.
- 2026-05-07: `cargo fmt --check` -> passed.
- 2026-05-07: `cargo clippy --all-targets --all-features -- -D warnings` -> passed.
- 2026-05-07: `cargo test --all` -> passed.
- 2026-05-07: `cargo build --all-targets` -> passed.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-tdd-gate.md` -> passed after implementation.
- 2026-05-07: `vibe_sentinel_validate_active_plans` -> passed after implementation.
- 2026-05-07: Moved the `WorkspaceProbe` reference implementation from core to ports during final diff review.
- 2026-05-07: `cargo fmt --check` -> passed after final diff review.
- 2026-05-07: `cargo clippy --all-targets --all-features -- -D warnings` -> passed after final diff review.
- 2026-05-07: `cargo test --all` -> passed after final diff review.
- 2026-05-07: `cargo build --all-targets` -> passed after final diff review.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py` -> passed after archiving completed plan.
- 2026-05-07: `vibe_sentinel_validate_active_plans` -> passed with no active plans after archiving.

### Review Notes

- Diff review: Implementation is scoped to the new app spec, active plan, domain
  gate data, core gate semantics, and MCP adapter wiring/tests. No dependency,
  transport, credential, deployment, or write-capable MCP changes were added.
- Risks: The gate can only enforce coarse workflow state from existing plan
  markers; deeper chronology and approver checks remain deferred.
- Follow-ups: Consider a separate read-only evidence-map tool after this slice if
  maintainers want richer debugging of TDD artifacts.

## Intended changes

- `docs/app-specs/mcp-tdd-gate-slice.md`: define the public contract and
  acceptance criteria.
- `docs/app-specs/index.md`: add the new app spec entry.
- `src/domain.rs`: add TDD gate action, phase, and report types.
- `src/core.rs`: add a core service that evaluates gate decisions from active
  plan validation reports.
- `src/mcp.rs`: add the MCP descriptor, input schema, argument parser, handler,
  response conversion, and fixture tests.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-tdd-gate.md`: required before feature implementation.
- `cargo test mcp::tests`: expected to pass after MCP wiring.
- Targeted core tests for `TddGateService`: expected to pass after core unit.
- `cargo fmt --check`: expected to pass.
- `cargo clippy --all-targets --all-features -- -D warnings`: expected to pass.
- `cargo test --all`: expected to pass.
- `cargo build --all-targets`: expected to pass.

## Risks and rollback

- Risk: The new gate duplicates or drifts from active-plan validation semantics.
- Mitigation: Reuse `PlanValidationService::evaluate_active_plans` and keep new
  gate rules coarse.
- Rollback: Remove the new MCP descriptor/handler, core gate service, domain
  gate types, app spec, and active plan.
- Risk: Strict enum argument parsing may reject client payloads with casing or
  spelling differences.
- Mitigation: Document snake_case values and reject invalid values with clear
  `-32602` errors.

## Progress log

- 2026-05-07: User approved starting implementation after reviewing the MCP TDD
  guardrail recommendation.
- 2026-05-07: Scope gate classified the work as `NEEDS_PLAN`; active plan and
  app spec created before Rust implementation.
- 2026-05-07: Required plan validation passed before Rust implementation.
- 2026-05-07: Domain TDD gate data shapes implemented and validated.
- 2026-05-07: Core TDD gate semantics implemented and validated.
- 2026-05-07: MCP TDD gate protocol wiring implemented and validated.
- 2026-05-07: Full validation suite passed; plan ready to archive as completed.
- 2026-05-07: Final diff review moved port-related glue into `src/ports.rs`; full validation passed again.

## Decisions

- First slice is read-only: avoids creating plan mutation paths that could bypass
  modified TDD review steps.
- Gate semantics live in core: preserves the architecture boundary requiring MCP
  adapters to stay thin.
- Existing validation service is reused: prevents two independent parsers for the
  same active-plan semantics.