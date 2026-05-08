# Execution Plan: MCP Active Plan Resources

## Status

- State: completed
- Owner: coding agent
- Created: 2026-05-07
- Last updated: 2026-05-07

## Summary

Add a narrow read-only MCP resource surface for active execution plans so local MCP clients can discover and read the current TDD plan artifact without exposing arbitrary workspace files or changing existing MCP tools.

## Scope

### In scope

- Add the public MCP `resources/list` and `resources/read` behavior for active execution plans.
- Advertise MCP resource capability during initialization.
- Represent active-plan resources through core-owned, workspace-probe-backed behavior.
- Add fixture and core tests for resource listing, reading, invalid requests, and existing tool regressions.
- Update app spec, tooling, observability, and README docs as needed for the new MCP resource surface.

### Out of scope

- Generic workspace file browsing.
- MCP prompts.
- Completed plan, app spec, reference, or harness document resources.
- Write-capable MCP behavior.
- Dependency changes or transport changes outside existing stdio JSON-RPC handling.
- Changes to existing MCP tool names, arguments, structured payloads, or semantics.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/app-specs/app-spec.md`
- `docs/app-specs/index.md`
- `docs/app-specs/mcp-active-plan-resources-slice.md`
- `docs/architecture.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/tooling.md`
- `docs/exec-plans/completed/mcp-tools-call-arguments.md`

## Acceptance criteria

- `initialize` advertises `resources.listChanged` as `false` while preserving existing tools capability output.
- `resources/list` returns sorted resources for active non-README markdown plans and returns an empty list when no active plan exists.
- `resources/read` accepts only server-owned active-plan resource URIs and returns markdown text for known active plans.
- Malformed resource params, non-string URIs, non-active-plan URIs, and unknown active-plan URIs produce deterministic JSON-RPC errors without aborting the session.
- Existing `vibe_sentinel_status`, `vibe_sentinel_validate_active_plans`, and `vibe_sentinel_tdd_gate` behavior remains compatible.

## TDD artifacts

### Feature Info

- Goal: expose active execution plan markdown through read-only local MCP resources.
- Acceptance criteria: initialization advertises resources; listing and reading work for active plans only; invalid resource requests are deterministic; existing MCP tools are unchanged.
- Constraints: public MCP contract change has explicit human approval from the 2026-05-07 user request to start implementation; implementation must follow the TDD sequence; resource reads must be allowlisted through active plan discovery.
- Non-goals: generic file browsing, prompts, completed plan resources, write behavior, dependency changes, and existing tool contract changes.

### Research Notes

- Docs inspected: `AGENTS.md`, `docs/harness/scope.md`, `docs/harness/operating-model.md`, `docs/app-specs/app-spec.md`, `docs/app-specs/index.md`, `docs/architecture.md`, `docs/security.md`, `docs/reliability.md`, `docs/observability.md`, `docs/tooling.md`, `docs/exec-plans/completed/mcp-tools-call-arguments.md`.
- Code inspected: `src/mcp.rs`, `src/core.rs`, `src/ports.rs`, `src/adapters/fs.rs`, `src/adapters/test_support.rs`.
- External references copied to `docs/references/`: none.
- Findings: existing `WorkspaceProbe` seams already provide sorted active plan discovery and text reads; `src/mcp.rs` already owns JSON-RPC method dispatch, initialize capability serialization, fixture-style tests, and tool error mapping; the slice should add a core resource service and keep MCP protocol handling thin.

### Reviewed Plan

- Plan review status: approved
- Refinements made: selected active-plan-only resources over generic file resources; recorded the public MCP contract approval from the user's 2026-05-07 implementation request; kept validator hardening and TUI lifecycle smoke testing outside this slice.

### Architecture Pseudocode

List every planned module, struct, enum, trait, function, and method before
scaffolding code.

```text
module domain
  struct ActivePlanResource
    field uri: String
    field name: String
    field path: String
    field mime_type: String
  struct ActivePlanResourceRead
    field uri: String
    field mime_type: String
    field text: String

module core
  struct ActivePlanResourceService<P: WorkspaceProbe>
    field probe: P
    fn new(probe: P) -> Self
    fn list_resources(&self) -> Result<Vec<ActivePlanResource>, VibeError>
    fn read_resource(&self, uri: &str) -> Result<ActivePlanResourceRead, VibeError>
    fn resource_from_path(path: &str) -> ActivePlanResource
    fn path_from_resource_uri(uri: &str, allowed_paths: &[String]) -> Result<String, VibeError>
  fn active_plan_resource_uri(path: &str) -> String
  fn active_plan_resource_name(path: &str) -> String

module mcp
  const ACTIVE_PLAN_RESOURCE_URI_PREFIX: &str
  struct ResourceReadParams
    field uri: String
  fn evaluate_active_plan_resources_list<P: WorkspaceProbe>(probe: P) -> Result<Vec<ActivePlanResource>, VibeError>
  fn evaluate_active_plan_resource_read<P: WorkspaceProbe>(probe: P, uri: &str) -> Result<ActivePlanResourceRead, VibeError>
  fn handle_resources_list(config: &McpServerConfig, id: Option<Value>) -> Result<JsonRpcResponse, VibeError>
  fn handle_resources_read(config: &McpServerConfig, id: Option<Value>, params: Option<Value>) -> Result<JsonRpcResponse, VibeError>
  fn parse_resource_read_params(id: Option<Value>, params: Option<Value>) -> ToolCallValidationResult<ResourceReadParams>
  fn resource_descriptor_json(resource: ActivePlanResource) -> Value
  fn resource_content_json(read: ActivePlanResourceRead) -> Value
```

### Reviewed Architecture

- Architecture review status: approved
- Refinements made: resource URI resolution stays in core so `resources/read` cannot bypass active-plan discovery; MCP adapter only serializes/deserializes protocol shapes and maps core results to JSON-RPC responses.

### Skeleton Checklist

- [x] `domain::ActivePlanResource` skeleton added with serializable resource metadata fields.
- [x] `domain::ActivePlanResourceRead` skeleton added with serializable markdown content fields.
- [x] `core::ActivePlanResourceService::new` skeleton added.
- [x] `core::ActivePlanResourceService::list_resources` skeleton added with minimal placeholder behavior.
- [x] `core::ActivePlanResourceService::read_resource` skeleton added with minimal placeholder behavior.
- [x] `core::ActivePlanResourceService::resource_from_path` skeleton added.
- [x] `core::ActivePlanResourceService::path_from_resource_uri` skeleton added.
- [x] `core::active_plan_resource_uri` skeleton added.
- [x] `core::active_plan_resource_name` skeleton added.
- [x] `mcp::evaluate_active_plan_resources_list` skeleton added.
- [x] `mcp::evaluate_active_plan_resource_read` skeleton added.
- [x] `mcp::ResourceReadParams` skeleton added.
- [x] `mcp::handle_resources_list` skeleton added.
- [x] `mcp::handle_resources_read` skeleton added.
- [x] `mcp::parse_resource_read_params` skeleton added.
- [x] `mcp::resource_descriptor_json` skeleton added.
- [x] `mcp::resource_content_json` skeleton added.
- [x] `mcp::handle_json_rpc_request` skeleton route entries added.
- [x] `mcp::handle_initialize` skeleton capability entry added.

### Mock Test Checklist

- [x] `active_plan_resources_list_sorted_active_markdown_files` covers core resource discovery using `FakeWorkspaceProbe`.
- [x] `active_plan_resources_read_known_resource` covers core markdown reading using `FakeWorkspaceProbe`.
- [x] `active_plan_resources_reject_unknown_uri` covers core URI allowlisting.
- [x] `mcp_initialize_advertises_resources_capability` covers initialize JSON-RPC fixture behavior.
- [x] `mcp_resources_list_returns_active_plan_resources` covers resource listing fixture behavior.
- [x] `mcp_resources_list_returns_empty_when_idle` covers idle resource listing.
- [x] `mcp_resources_read_returns_markdown_contents` covers successful resource reads.
- [x] `mcp_resources_read_rejects_invalid_params` covers malformed params and non-string URIs.
- [x] `mcp_resources_read_rejects_unknown_uri_without_aborting` covers deterministic error mapping and session continuity.
- [x] `mcp_existing_tools_still_list_and_call` covers existing tool regression behavior through existing status/tool list fixtures.

### Implementation Checklist

- [x] `domain::ActivePlanResource` and `domain::ActivePlanResourceRead` filled in.
- Validation after this unit: `cargo test active_plan_resources_list_sorted_active_markdown_files` passed after resource metadata was exercised by core listing.
- [x] `core::ActivePlanResourceService::list_resources` and helper formatting filled in.
- Validation after this unit: `cargo test active_plan_resources_list_sorted_active_markdown_files` passed.
- [x] `core::ActivePlanResourceService::read_resource` and URI allowlisting filled in.
- Validation after this unit: `cargo test active_plan_resources` passed after rerunning with Cargo's single-filter syntax; an earlier two-filter command failed with Cargo usage error.
- [x] MCP initialize resources capability filled in.
- Validation after this unit: `cargo test active_plan_resources` passed and existing initialize fixture checks resources capability.
- [x] MCP `resources/list` handler filled in.
- Validation after this unit: `cargo test active_plan_resources` passed.
- [x] MCP `resources/read` params, handler, and error mapping filled in.
- Validation after this unit: `cargo test mcp_resources` passed for 5 resource fixture tests.
- [x] Documentation updates filled in.
- Validation after this unit: documentation updates completed; full validation pending.

### Validation Log

- 2026-05-07: plan created; validation pending.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-active-plan-resources.md` passed.
- 2026-05-07: skeleton domain, core, and MCP resource units added; skeleton validation pending.
- 2026-05-07: `cargo test active_plan_resource` passed after skeletons compiled; 0 tests matched before mock tests were added.
- 2026-05-07: active-plan resource core and MCP fixture tests added; red/green validation pending.
- 2026-05-07: `cargo test active_plan_resources` failed as expected against placeholders: resource list returned 0 resources and known resource reads were rejected.
- 2026-05-07: `cargo test active_plan_resources_list_sorted_active_markdown_files` passed after filling core resource listing and metadata helpers.
- 2026-05-07: `cargo test active_plan_resources_read_known_resource active_plan_resources_reject_unknown_uri` failed with Cargo usage error because Cargo accepts a single test-name filter.
- 2026-05-07: `cargo test active_plan_resources` passed after filling core resource read behavior; 4 tests passed.
- 2026-05-07: `cargo test mcp_resources` failed as expected before filling `resources/read` params and JSON-RPC error mapping; 3 passed, 2 failed.
- 2026-05-07: `cargo test mcp_resources` passed after filling `resources/read` params and error mapping; 5 tests passed.
- 2026-05-07: `cargo test mcp_resources` passed after adding workspace-error fixture coverage; 6 tests passed.
- 2026-05-07: README, tooling, observability, app-spec status, and app-spec index docs updated for active-plan resources.
- 2026-05-07: `cargo fmt --check` failed with formatting diffs in `src/mcp.rs`.
- 2026-05-07: `cargo fmt` completed.
- 2026-05-07: `cargo fmt --check` passed after formatting.
- 2026-05-07: `cargo test mcp::tests` passed: 39 MCP tests.
- 2026-05-07: `cargo clippy --all-targets --all-features -- -D warnings` passed.
- 2026-05-07: `cargo test --all` passed: 77 unit tests, 2 CLI integration tests, 2 TUI integration tests, and doc tests.
- 2026-05-07: `cargo build --all-targets` passed.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-active-plan-resources.md` passed.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py` passed for 2 file(s).
- 2026-05-07: `git --no-pager diff --check` passed.
- 2026-05-07: `python3 /Users/samroberts/plugins/agentic-coding-harness/scripts/validate_harness.py` failed in sandbox with `Operation not permitted` when opening the plugin script.
- 2026-05-07: `python3 /Users/samroberts/plugins/agentic-coding-harness/scripts/validate_harness.py` passed unsandboxed: 0 failures, 0 warnings.
- 2026-05-07: subprocess MCP smoke against `target/debug/vibe-sentinel mcp serve` passed for `initialize`, `resources/list`, `resources/read`, and `vibe_sentinel_status`; resources and tools capabilities advertised `listChanged: false`, one active-plan resource was listed/read, and status returned `ready: true`.
- 2026-05-07: wording normalized to TDD per user direction; `cargo test tdd_gate_tool_descriptor_is_read_only_idempotent_and_local` and plan validation passed.
- 2026-05-07: plan archived to `docs/exec-plans/completed/mcp-active-plan-resources.md`; final `cargo fmt --check`, `cargo test mcp::tests`, `python3 scripts/validate_tdd_workflow.py`, explicit completed-plan validation, `git --no-pager diff --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, `cargo build --all-targets`, and unsandboxed harness validation passed.

### Review Notes

- Diff review: completed after formatting, lint, full tests, build, TDD validators, harness validation, diff whitespace check, and subprocess MCP smoke.
- Risks: MCP resource capability shape may need adjustment if a client expects a different resources payload; fixture coverage and subprocess smoke exercise the local stdio contract.
- Follow-ups: Python TDD validator semantic hardening and deterministic TUI lifecycle smoke remain separate tech-debt slices.

## Intended changes

- `docs/app-specs/mcp-active-plan-resources-slice.md`: define active-plan resource behavior and acceptance criteria.
- `docs/app-specs/index.md`: link the shipped slice.
- `docs/exec-plans/active/mcp-active-plan-resources.md`: track this TDD workflow.
- `src/domain.rs`: add active-plan resource domain data types.
- `src/core.rs`: add active-plan resource listing, URI allowlisting, and read behavior against `WorkspaceProbe`.
- `src/mcp.rs`: advertise resources capability, route `resources/list` and `resources/read`, validate resource read params, and map core results to protocol responses.
- `docs/tooling.md`: update MCP fixture coverage notes.
- `docs/observability.md`: document resource fixture/error expectations.
- `README.md`: update public MCP capability notes if the README enumerates MCP features.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-active-plan-resources.md`: required before feature implementation.
- `cargo test active_plan_resources`: required after core skeleton and implementation units.
- `cargo test mcp::tests`: required after MCP skeleton and implementation units.
- `cargo fmt --check`: required after Rust edits.
- `cargo clippy --all-targets --all-features -- -D warnings`: required after Rust edits.
- `cargo test --all`: required before final review.
- `cargo build --all-targets`: required before final review.
- `python3 scripts/validate_tdd_workflow.py`: required before final review.
- `git --no-pager diff --check`: required before final review.

## Risks and rollback

- Risk: resource URI design could accidentally imply generic file access.
- Mitigation: use `vibe-sentinel://active-plans/<file-name>` and resolve only through active-plan discovery.
- Rollback: remove resource capability, resource handlers, resource domain/core types, fixture tests, and docs for this slice while preserving existing MCP tools.

## Progress log

- 2026-05-07: selected MCP active-plan resources as the next slice and created spec/plan artifacts.

## Decisions

- Public MCP contract approval: granted by the user's 2026-05-07 request, "Start implementation".
- URI scope: expose only active execution plans through server-owned URIs, not workspace paths.
- Boundary placement: resource discovery and URI allowlisting belong in core over `WorkspaceProbe`; MCP remains a thin JSON-RPC adapter.