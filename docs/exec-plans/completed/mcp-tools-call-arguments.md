# Execution Plan: MCP Tools/Call Argument Validation

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-07
- Last updated: 2026-05-07

## Summary

Harden the existing local MCP `tools/call` boundary so call-time validation
matches the empty input schemas advertised for the shipped no-argument tools.
This slice preserves omitted-argument compatibility while rejecting malformed or
unexpected arguments deterministically, without changing tool output shapes or
application-core behavior.

## Scope

### In scope

- Validate `tools/call` arguments for `vibe_sentinel_status` and
  `vibe_sentinel_validate_active_plans`.
- Accept omitted `arguments` and explicit empty-object `arguments: {}`.
- Reject non-object `arguments` and argument objects with unexpected properties.
- Reject malformed `tools/call` envelopes, including non-object `params` and
  missing or non-string `name`, with JSON-RPC invalid-params errors.
- Ensure invalid `tools/call` requests do not abort the stdio session.
- Generate `tools/list` input schemas from descriptor-owned schema metadata.
- Preserve current successful tool outputs, workspace-error payloads, tool names,
  CLI/TUI behavior, and MCP framing behavior.
- Clarify existing no-argument MCP slice specs and fixture expectations.

### Out of scope

- Adding new required or optional arguments to either MCP tool.
- Adding new MCP tools, resources, prompts, transports, or workflows.
- Changing status or active-plan validation response shapes.
- Changing application-core status or active-plan validation semantics.
- Broad JSON-RPC refactors outside the current `tools/call` validation path.
- CLI, TUI, deployment, dependency, credential, or storage changes.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/app-specs/mcp-status-tool-slice.md`
- `docs/app-specs/mcp-active-plan-validation-slice.md`
- `docs/architecture.md`
- `docs/tooling.md`
- `docs/quality.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/review.md`
- `docs/exec-plans/completed/mcp-status-tool.md`
- `docs/exec-plans/completed/mcp-active-plan-validation.md`

## Acceptance criteria

- `tools/list` still advertises both current tools with `type: object`, empty
  `properties`, and `additionalProperties: false`, generated from descriptor
  schema metadata.
- `tools/call` accepts `vibe_sentinel_status` and
  `vibe_sentinel_validate_active_plans` with omitted `arguments` or `{}`.
- `tools/call` rejects non-object `arguments` for both current tools with a
  deterministic JSON-RPC `-32602` invalid-params error.
- `tools/call` rejects non-empty argument objects for both current tools with a
  deterministic JSON-RPC `-32602` invalid-params error.
- Malformed `tools/call` envelopes return deterministic JSON-RPC `-32602`
  errors and do not abort the stdio session.
- Existing successful tool responses and workspace-error `isError: true`
  payloads remain unchanged.
- Existing `initialize`, `tools/list`, unknown-tool handling, CLI status
  surfaces, and active-plan validation semantics remain compatible.

## TDD artifacts

### Feature Info

- Goal: Make MCP no-argument `tools/call` validation match the schemas already
  advertised for shipped local tools.
- Acceptance criteria: The existing no-argument MCP tools accept omitted or empty
  arguments, reject malformed or unexpected arguments deterministically, keep the
  stdio session alive after invalid calls, and preserve existing tool results.
- Constraints: Keep the change inside the MCP boundary layer; do not alter core
  status or plan-validation behavior; preserve backward compatibility for
  omitted arguments.
- Non-goals: New tools, resources, prompts, transports, argument-taking tools,
  core semantic changes, CLI/TUI changes, dependencies, deployment, storage, or
  credentials.

### Research Notes

- Docs inspected: `AGENTS.md`, `docs/harness/scope.md`,
  `docs/harness/operating-model.md`, `docs/app-specs/mcp-status-tool-slice.md`,
  `docs/app-specs/mcp-active-plan-validation-slice.md`, `docs/architecture.md`,
  `docs/tooling.md`, `docs/security.md`, `docs/reliability.md`,
  `docs/observability.md`, and recent completed MCP plans.
- Code inspected: `src/mcp.rs` and current MCP fixture helpers/tests.
- External references copied to `docs/references/`: none.
- Findings: Current `tools/list` advertises empty object schemas with
  `additionalProperties: false`, but `handle_tools_call` dispatches by name and
  ignores `arguments`. Existing tests cover successful calls with `{}`, unknown
  tools, and workspace errors, but not omitted arguments, non-object arguments,
  unexpected argument properties, or malformed envelopes followed by valid calls.

### Reviewed Plan

- Plan review status: reviewed
- Refinements made: Treat this as compatibility hardening for shipped tools, not
  a new MCP feature. Preserve omitted `arguments` compatibility and reject only
  schema-invalid argument payloads for the current no-argument tools. Explicit
  approval is recorded from the 2026-05-07 instruction to start implementation
  after this plan was proposed.

### Architecture Pseudocode

List every planned module, struct, enum, trait, function, and method before
scaffolding code.

```text
module mcp
  enum McpTool
    variant Status
    variant ActivePlanValidation

  enum McpToolInputSchema
    variant NoArguments

  struct McpToolDescriptor
    field name: String
    field description: String
    field read_only: bool
    field idempotent: bool
    field local_only: bool
    field input_schema: McpToolInputSchema

  struct ToolCallParams
    field name: String
    field arguments: Option<Value>

  struct NoArguments

  fn status_tool_descriptor() -> McpToolDescriptor
  fn active_plan_validation_tool_descriptor() -> McpToolDescriptor
  fn tool_descriptors() -> Vec<McpToolDescriptor>
  fn tool_from_name(name) -> Option<McpTool>
  fn input_schema_json(schema) -> Value
  fn invalid_params(id, message) -> JsonRpcResponse
  fn parse_tool_call_params(id, params) -> Result<ToolCallParams, JsonRpcResponse>
  fn parse_no_arguments(id, tool_name, arguments) -> Result<NoArguments, JsonRpcResponse>
  fn validate_tool_call_arguments(id, tool, tool_name, arguments) -> Result<(), JsonRpcResponse>
  fn handle_tools_list(id) -> Result<JsonRpcResponse, VibeError>
  fn handle_tools_call(config, id, params) -> Result<JsonRpcResponse, VibeError>

module tests
  fn tools_list_uses_descriptor_input_schema_for_current_tools()
  fn session_handles_status_tool_call_request_without_arguments_field()
  fn session_handles_active_plan_validation_tool_call_request_without_arguments_field()
  fn session_rejects_status_tool_call_with_unexpected_arguments()
  fn session_rejects_active_plan_validation_tool_call_with_unexpected_arguments()
  fn session_rejects_status_tool_call_with_non_object_arguments()
  fn session_rejects_active_plan_validation_tool_call_with_non_object_arguments()
  fn session_rejects_tools_call_with_non_object_params_without_aborting()
  fn session_rejects_tools_call_without_name_without_aborting()
```

### Reviewed Architecture

- Architecture review status: reviewed
- Refinements made: Keep validation in the MCP adapter because this slice checks
  protocol boundary payloads, not application data. Use descriptor-owned schema
  metadata so `tools/list` and call-time validation stay aligned.

### Skeleton Checklist

- [x] `mcp::McpToolInputSchema` skeleton added.
- [x] `mcp::McpToolDescriptor` input-schema field skeleton added.
- [x] `mcp::ToolCallParams` skeleton added.
- [x] `mcp::NoArguments` skeleton added.
- [x] `mcp::input_schema_json` skeleton added.
- [x] `mcp::invalid_params` skeleton added.
- [x] `mcp::parse_tool_call_params` skeleton added.
- [x] `mcp::parse_no_arguments` skeleton added.
- [x] `mcp::validate_tool_call_arguments` skeleton added.
- [x] `mcp::handle_tools_list` skeleton routes descriptor schema metadata.
- [x] `mcp::handle_tools_call` skeleton routes through argument validation.

### Mock Test Checklist

- [x] `tools_list_uses_descriptor_input_schema_for_current_tools` covers schema metadata.
- [x] `session_handles_status_tool_call_request_without_arguments_field` covers omitted-arguments compatibility.
- [x] `session_handles_active_plan_validation_tool_call_request_without_arguments_field` covers omitted-arguments compatibility.
- [x] `session_rejects_status_tool_call_with_unexpected_arguments` covers extra properties.
- [x] `session_rejects_active_plan_validation_tool_call_with_unexpected_arguments` covers extra properties.
- [x] `session_rejects_status_tool_call_with_non_object_arguments` covers non-object arguments.
- [x] `session_rejects_active_plan_validation_tool_call_with_non_object_arguments` covers non-object arguments.
- [x] `session_rejects_tools_call_with_non_object_params_without_aborting` covers malformed params and session continuity.
- [x] `session_rejects_tools_call_without_name_without_aborting` covers missing tool name and session continuity.

### Implementation Checklist

- [x] Fill descriptor-owned input schema rendering.
- Validation after this unit: focused MCP tools/list schema test.
- [x] Fill `ToolCallParams` parsing and invalid-params response helper.
- Validation after this unit: malformed envelope fixture tests.
- [x] Fill no-argument validation for omitted and empty object arguments.
- Validation after this unit: omitted-argument compatibility fixture tests.
- [x] Fill rejection for non-object and non-empty argument payloads.
- Validation after this unit: invalid argument fixture tests.
- [x] Update MCP docs and execution-plan evidence.
- Validation after this unit: docs review and full required validation commands.

### Validation Log

- 2026-05-07: Plan created after scope-gating the request as `NEEDS_PLAN` and
  confirming the existing MCP no-argument contract gap.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-tools-call-arguments.md` passed.
- 2026-05-07: `cargo test mcp::tests` red as expected after fixtures: 23 passed,
  4 failed on invalid argument payloads still being accepted.
- 2026-05-07: `cargo test mcp::tests` passed after no-argument validation: 27
  passed, 0 failed.
- 2026-05-07: `cargo fmt --check` passed.
- 2026-05-07: `cargo clippy --all-targets --all-features -- -D warnings` passed after boxing local validation response errors.
- 2026-05-07: `cargo test --all` passed: 56 unit tests, 2 CLI integration tests,
  2 TUI integration tests, and doc tests.
- 2026-05-07: `cargo build --all-targets` passed.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-tools-call-arguments.md` passed after implementation.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py` passed for 2 file(s).
- 2026-05-07: `git --no-pager diff --check` passed.

### Review Notes

- Diff review: completed; changes stay in the MCP adapter plus contract/tooling
  docs and do not alter application-core behavior.
- Risks: Tightening argument validation could break clients that sent ignored
  extra arguments; this intentionally matches the already-advertised empty
  schema while preserving omitted-argument compatibility.
- Follow-ups: Consider active-plan read resources after this contract hardening
  slice; keep semantic validator hardening and TUI lifecycle smoke testing as
  separate plans.

## Intended changes

- `src/mcp.rs`: add descriptor-owned no-argument schema metadata, argument
  parsing/validation helpers, and MCP fixture tests.
- `docs/app-specs/mcp-status-tool-slice.md`: clarify no-argument call semantics.
- `docs/app-specs/mcp-active-plan-validation-slice.md`: clarify no-argument call
  semantics.
- `docs/observability.md`: document invalid `tools/call` argument fixture
  expectations.
- `docs/tooling.md`: mention no-argument validation in MCP fixture coverage.
- `docs/exec-plans/completed/mcp-tools-call-arguments.md`: track the modified
  TDD workflow and archived evidence for this slice.
- `README.md` and `AGENTS.md`: clean up small stale wording discovered during
  planning, if touched by the final doc pass.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-tools-call-arguments.md`: required before feature implementation while the plan is active.
- `cargo test mcp::tests`: required after MCP edits.
- `cargo fmt --check`: required after Rust edits.
- `cargo clippy --all-targets --all-features -- -D warnings`: required after Rust edits.
- `cargo test --all`: required after Rust edits.
- `cargo build --all-targets`: required after Rust edits.
- `python3 scripts/validate_tdd_workflow.py`: required before final review.
- `git --no-pager diff --check`: required before final review.

## Risks and rollback

- Risk: Clients that sent ignored extra arguments will now receive invalid-params
  errors.
- Mitigation: This matches the existing advertised schema and keeps omitted
  `arguments` compatible.
- Rollback: Revert `src/mcp.rs` argument-validation helpers/tests and the MCP doc
  clarifications; existing no-argument tool execution should return to the prior
  dispatcher behavior.
- Risk: Validation helper complexity could grow into generic schema machinery.
- Mitigation: Support only `NoArguments` in this slice and defer future
  argument-taking tool schemas until needed.

## Progress log

- 2026-05-07: Created the active execution plan and clarified no-argument
  behavior in shipped MCP slice specs.
- 2026-05-07: Added descriptor-owned no-argument schema metadata, call-envelope
  parsing, no-argument validation, and fixture coverage for omitted, empty,
  malformed, and unexpected arguments.
- 2026-05-07: Completed validation and archived the slice.

## Decisions

- Scope classification: `NEEDS_PLAN`.
- Public contract approval: captured from the 2026-05-07 instruction to start
  implementation after this slice was recommended.
- Compatibility: omitted `arguments` and empty-object `arguments: {}` are both
  valid for existing tools.
- Error mapping: schema-invalid tool calls return JSON-RPC `-32602`; workspace
  evaluation failures remain tool-level `isError: true` payloads.
- Boundary: no core/domain/port changes are planned.