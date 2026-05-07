# Execution Plan: MCP Status Tool

## Status

- State: active
- Owner: coding agent
- Created: 2026-05-07
- Last updated: 2026-05-07

## Summary

Implement the first MCP surface for `vibe-sentinel`: a local, read-only status
tool launched with `vibe-sentinel mcp serve` and backed by the existing status
evaluation path. This plan starts with the public contract and architecture
pseudocode required by the modified TDD workflow, then proceeds through
skeletons, fixture tests, and one implementation unit at a time.

## Scope

### In scope

- Add a local MCP server entry point for `vibe-sentinel mcp serve`.
- Add a read-only MCP status tool named `vibe_sentinel_status`.
- Reuse existing `StatusService`, `StatusReport`, `WorkspaceProbe`,
  `FsWorkspaceProbe`, and `FakeWorkspaceProbe` patterns.
- Add MCP request/response fixture tests for protocol behavior.
- Preserve existing CLI text, JSON, and TUI status behavior.
- Update docs and validation guidance for the shipped MCP slice.

### Out of scope

- Remote MCP transport.
- Write-capable MCP tools, resources, prompts, or multi-tool workflows.
- Deep semantic TDD workflow enforcement beyond the current status report.
- Changes to idle-state active-plan readiness semantics.
- Deployment, credentials, persistent storage, or destructive operations.
- Dependency swaps or runtime changes beyond the explicitly approved MCP server
  dependency set selected during this plan.

## Harness docs consulted

- `AGENTS.md`
- `docs/harness/scope.md`
- `docs/harness/operating-model.md`
- `docs/app-specs/app-spec.md`
- `docs/app-specs/status-json-slice.md`
- `docs/app-specs/mcp-status-tool-slice.md`
- `docs/architecture.md`
- `docs/tooling.md`
- `docs/quality.md`
- `docs/security.md`
- `docs/reliability.md`
- `docs/observability.md`
- `docs/references/symdex-mcp-tools-usage.md`

## Acceptance criteria

- `vibe-sentinel mcp serve` is parsed and routed without breaking existing
  `status`, `status --json`, or `status --tui` behavior.
- The MCP status tool returns structured readiness data equivalent to
  `vibe-sentinel status --json`.
- MCP protocol handling remains a thin surface over application-core status
  evaluation.
- Request/response fixture tests cover successful status calls and deterministic
  protocol or workspace error mapping.
- Tool metadata marks the status tool as read-only, idempotent, and local-only.
- Validation commands in this plan pass or skipped commands are recorded with
  risk.

## Modified TDD artifacts

### Feature Info

- Goal: Add the first local MCP status tool so coding agents can read
  `vibe-sentinel` readiness through MCP.
- Acceptance criteria: The public contract in
  `docs/app-specs/mcp-status-tool-slice.md` is implemented, existing status
  surfaces remain compatible, and fixture-backed MCP tests pass.
- Constraints: Preserve architecture layering; validate MCP payloads at the
  boundary; keep the tool read-only and local-only; do not add deeper workflow
  enforcement in this slice.
- Non-goals: Remote transport, write tools, resources, prompts, deployment,
  credentials, storage, or semantic TDD plan-quality validation.

### Research Notes

- Docs inspected: `AGENTS.md`, `docs/harness/scope.md`,
  `docs/harness/operating-model.md`, `docs/app-specs/app-spec.md`,
  `docs/app-specs/status-json-slice.md`,
  `docs/app-specs/mcp-status-tool-slice.md`, `docs/architecture.md`,
  `docs/tooling.md`, `docs/quality.md`, `docs/security.md`,
  `docs/reliability.md`, `docs/observability.md`, and
  `docs/references/symdex-mcp-tools-usage.md`.
- Code inspected: `src/core.rs`, `src/domain.rs`, `src/ports.rs`,
  `src/adapters/fs.rs`, `src/adapters/test_support.rs`, `src/cli.rs`,
  `src/main.rs`, `src/lib.rs`, `tests/cli_status.rs`, and `tests/tui_status.rs`.
- External references copied to `docs/references/`: none yet.
- Findings: The existing status report is already protocol-neutral and serde
  serializable. Symdex freshness is current for indexed Rust and Cargo files.
  Symdex impact evidence flags `cli::parse_args`, `cli::execute_with_probe`,
  `cli::render_status`, `main::run`, `domain::StatusReport` methods, and
  `FsWorkspaceProbe` methods as likely affected by CLI routing, response shape,
  or filesystem-probe changes. MCP implementation must keep protocol handling
  thin and avoid changing the status semantics shipped by the CLI, JSON, and TUI
  slices.

### Reviewed Plan

- Plan review status: reviewed
- Refinements made: Start with the narrow read-only status tool and CLI startup
  path selected on 2026-05-07. Defer deeper TDD workflow enforcement to a later
  execution plan.

### Architecture Pseudocode

List every planned module, struct, enum, trait, function, and method before
scaffolding code.

```text
module cli
  enum CliCommand
    variant Status
    variant McpServe
  fn parse_args(args) -> Result<CliArgs, VibeError>
  fn execute_with_probe(args, probe) -> Result<StatusReport, VibeError>
  fn render_status(args, report) -> Result<String, VibeError>

module main
  fn main() -> ExitCode
  fn run(args) -> Result<Option<String>, VibeError>
    route Status text/json/tui exactly as today
    route McpServe to mcp::run_stdio_server(root)

module mcp
  const STATUS_TOOL_NAME: &str

  struct McpServerConfig
    field root: PathBuf

  struct McpToolDescriptor
    field name: String
    field description: String
    field read_only: bool
    field idempotent: bool
    field local_only: bool

  enum McpStatusRequest
    variant Status

  struct McpStatusResponse
    field project_name: String
    field ready: bool
    field checks: Vec<StatusCheck>

  enum McpErrorCode
    variant InvalidRequest
    variant WorkspaceUnreadable
    variant InternalError

  struct McpErrorResponse
    field code: McpErrorCode
    field message: String

  struct JsonRpcRequest
    field jsonrpc: String
    field id: Option<Value>
    field method: String
    field params: Option<Value>

  struct JsonRpcResponse
    field jsonrpc: String
    field id: Option<Value>
    field result: Option<Value>
    field error: Option<JsonRpcError>

  struct JsonRpcError
    field code: i64
    field message: String
    field data: Option<Value>

  struct StdioServer<R: BufRead, W: Write>
    field config: McpServerConfig
    field reader: R
    field writer: W

  impl StdioServer<R, W>
    fn new(config, reader, writer) -> Self
    fn run(&mut self) -> Result<(), VibeError>

  fn status_tool_descriptor() -> McpToolDescriptor
  fn evaluate_status_tool<P: WorkspaceProbe>(probe: P) -> Result<McpStatusResponse, VibeError>
  fn response_from_report(report: StatusReport) -> McpStatusResponse
  fn map_error(error: VibeError) -> McpErrorResponse
  fn run_stdio_server(config: McpServerConfig) -> Result<(), VibeError>
  fn run_stdio_session<R: BufRead, W: Write>(config, reader, writer) -> Result<(), VibeError>
  fn read_content_length_message<R: BufRead>(reader) -> Result<Option<String>, VibeError>
  fn write_content_length_message<W: Write>(writer, payload) -> Result<(), VibeError>
  fn handle_json_rpc_request(config, request) -> Result<Option<JsonRpcResponse>, VibeError>
  fn handle_initialize(id, params) -> Result<JsonRpcResponse, VibeError>
  fn initialize_protocol_version(params) -> String
  fn handle_tools_list(id) -> Result<JsonRpcResponse, VibeError>
  fn handle_tools_call(config, id, params) -> Result<JsonRpcResponse, VibeError>
  fn json_rpc_success(id, result) -> JsonRpcResponse
  fn json_rpc_error(id, code, message, data) -> JsonRpcResponse
  fn serialize_protocol_result(value) -> Result<Value, VibeError>

module tests/mcp_status fixtures or src/mcp.rs tests
  fn status_tool_descriptor_is_read_only_idempotent_and_local()
  fn status_tool_response_matches_status_report_shape()
  fn status_tool_maps_workspace_errors_to_mcp_errors()
  fn mcp_serve_command_is_parsed_without_changing_status_commands()
  fn content_length_round_trips_json_payload()
  fn session_handles_initialize_and_tools_list_requests()
  fn initialize_echoes_client_protocol_version()
  fn session_handles_status_tool_call_request()
  fn session_maps_unknown_tool_to_invalid_request_error()
  fn session_maps_workspace_probe_errors_to_tool_error_payload()
```

### Reviewed Architecture

- Architecture review status: reviewed
- Refinements made: Keep MCP as a thin surface module. Reuse the existing
  application-core status path and defer any new workflow-inspection ports until
  a later TDD enforcement plan.

### Skeleton Checklist

- [x] `cli::CliCommand::McpServe` skeleton added.
- [x] `cli::parse_args` skeleton accepts `mcp serve` without implementing server behavior.
- [x] `main::run` skeleton routes `McpServe` to an MCP runtime seam.
- [x] `mcp` module skeleton exported from `src/lib.rs`.
- [x] `mcp::McpServerConfig` skeleton added.
- [x] `mcp::McpToolDescriptor` skeleton added.
- [x] `mcp::McpStatusRequest` skeleton added.
- [x] `mcp::McpStatusResponse` skeleton added.
- [x] `mcp::McpErrorCode` and `mcp::McpErrorResponse` skeletons added.
- [x] `mcp::status_tool_descriptor` skeleton added.
- [x] `mcp::evaluate_status_tool` skeleton added.
- [x] `mcp::response_from_report` skeleton added.
- [x] `mcp::map_error` skeleton added.
- [x] `mcp::run_stdio_server` skeleton added.
- [x] `mcp::JsonRpcRequest`, `JsonRpcResponse`, and `JsonRpcError` skeletons added.
- [x] `mcp::StdioServer` and `StdioServer::run` skeletons added.
- [x] `mcp::run_stdio_session` skeleton added.
- [x] `mcp::read_content_length_message` skeleton added.
- [x] `mcp::write_content_length_message` skeleton added.
- [x] `mcp::handle_json_rpc_request` skeleton added.
- [x] `mcp::handle_initialize` skeleton added.
- [x] `mcp::handle_tools_list` skeleton added.
- [x] `mcp::handle_tools_call` skeleton added.
- [x] `mcp::json_rpc_success` and `mcp::json_rpc_error` skeletons added.
- [x] `mcp::serialize_protocol_result` skeleton added.

### Mock Test Checklist

- [x] `status_tool_descriptor_is_read_only_idempotent_and_local` covers tool metadata.
- [x] `status_tool_response_matches_status_report_shape` covers report-to-MCP response shape using `FakeWorkspaceProbe`.
- [x] `status_tool_maps_workspace_errors_to_mcp_errors` covers deterministic error mapping.
- [x] `mcp_serve_command_is_parsed_without_changing_status_commands` covers CLI parsing and existing status compatibility.
- [x] Binary or fixture-level MCP request/response test covers a successful status call.
- [x] `content_length_round_trips_json_payload` covers stdio framing.
- [x] `session_handles_initialize_and_tools_list_requests` covers basic MCP capability negotiation and tool discovery.
- [x] `session_handles_status_tool_call_request` covers successful fixture-level status response.
- [x] `session_maps_unknown_tool_to_invalid_request_error` covers deterministic protocol error mapping.
- [x] `session_maps_workspace_probe_errors_to_tool_error_payload` covers deterministic workspace error payloads.

### Implementation Checklist

- [x] Fill `cli::CliCommand::McpServe` and `cli::parse_args` behavior.
- Validation after this unit: `cargo test --all cli::tests` or the closest focused Cargo test available.
- [x] Fill `mcp::status_tool_descriptor` behavior.
- Validation after this unit: focused MCP descriptor test.
- [x] Fill `mcp::response_from_report` and `mcp::evaluate_status_tool` behavior.
- Validation after this unit: focused MCP status response tests.
- [x] Fill `mcp::map_error` behavior.
- Validation after this unit: focused MCP error mapping tests.
- [x] Fill `mcp` JSON-RPC envelope, framing, and response helper behavior.
- Validation after this unit: focused MCP framing and handler tests.
- [x] Fill `mcp::StdioServer`, `mcp::run_stdio_session`, `mcp::run_stdio_server`, and `main::run` routing behavior.
- Validation after this unit: MCP fixture tests plus existing CLI/TUI status tests.
- [x] Update README and tooling/observability docs for the shipped MCP command.
- Validation after this unit: docs review and full required validation commands.

### Validation Log

- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md` -> passed after creating the active plan.
- 2026-05-07: `python3 /Users/samroberts/plugins/agentic-coding-harness/scripts/harness_section_status.py --dir .harness-validation` -> sandboxed run failed with `Operation not permitted` opening the plugin script; unsandboxed rerun passed with 15 complete, 0 needs_update, 0 failed, next_action `skip`.
- 2026-05-07: `python3 /Users/samroberts/plugins/agentic-coding-harness/scripts/validate_harness.py` -> sandboxed run failed with `Operation not permitted` opening the plugin script; unsandboxed rerun passed with 0 failures and 0 warnings.
- 2026-05-07: `cargo test mcp_skeleton parse_args_accepts_mcp_serve execute_with_probe_rejects_mcp_serve` -> failed because Cargo accepts only one test filter before `--`.
- 2026-05-07: `cargo test --all` -> passed after skeletons were added.
- 2026-05-07: `cargo test status_tool_descriptor_is_read_only_idempotent_and_local` -> passed after filling MCP tool metadata.
- 2026-05-07: `cargo test status_tool_response_matches_status_report_shape` -> failed with Rust move error `E0382` in `response_from_report`; fixed by computing readiness before moving fields.
- 2026-05-07: `cargo test status_tool_response_matches_status_report_shape` -> passed after the move-order fix.
- 2026-05-07: `cargo test status_tool_maps_workspace_errors_to_mcp_errors` -> passed after filling MCP error mapping.
- 2026-05-07: `cargo fmt --check` -> failed with formatting diffs in `src/mcp.rs`.
- 2026-05-07: `cargo fmt` -> applied standard Rust formatting.
- 2026-05-07: `cargo fmt --check` -> passed after formatting.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md` -> passed after updating plan evidence.
- 2026-05-07: `cargo clippy --all-targets --all-features -- -D warnings` -> passed.
- 2026-05-07: `cargo test --all` -> passed with 24 unit tests, 2 CLI integration tests, 2 TUI integration tests, and 0 doctests.
- 2026-05-07: `cargo build --all-targets` -> passed.
- 2026-05-07: `git --no-pager diff --check` -> passed.
- 2026-05-07: VS Code diagnostics -> no errors found.
- 2026-05-07: Symdex watcher status -> `quality_ready` after indexing `src/mcp.rs`.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py` -> passed for the plan template and active MCP plan.
- 2026-05-07: Symdex watcher status -> `quality_ready`; explain-change report identified `src/mcp.rs`, `src/main.rs`, and the active MCP plan as affected by the stdio runtime work.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md` -> passed after expanding stdio runtime pseudocode.
- 2026-05-07: `cargo test content_length_round_trips_json_payload` -> failed first because the MCP test helper needed `std::io::Write`; fixed the test helper import.
- 2026-05-07: `cargo test content_length_round_trips_json_payload` -> failed against the intended content-length writer skeleton.
- 2026-05-07: `cargo test content_length_round_trips_json_payload` -> passed after filling content-length framing.
- 2026-05-07: `cargo test session_handles_initialize_and_tools_list_requests` -> failed against the intended stdio session skeleton.
- 2026-05-07: `cargo test session_handles_initialize_and_tools_list_requests` -> passed after filling JSON-RPC dispatch and session handling.
- 2026-05-07: `cargo test session_handles_status_tool_call_request` -> passed.
- 2026-05-07: `cargo test session_maps_unknown_tool_to_invalid_request_error` -> passed.
- 2026-05-07: `cargo test session_maps_workspace_probe_errors_to_tool_error_payload` -> passed.
- 2026-05-07: `cargo test mcp::tests` -> sandboxed run failed because Rust could not write `/Users/samroberts/.rustup/settings.toml` (`Operation not permitted`); unsandboxed async-capture rerun passed with 10 MCP tests.
- 2026-05-07: `cargo fmt` -> applied standard Rust formatting after stdio runtime implementation.
- 2026-05-07: `cargo fmt --check` -> passed.
- 2026-05-07: `cargo clippy --all-targets --all-features -- -D warnings` -> passed.
- 2026-05-07: `cargo test --all` -> passed with 29 unit tests, 2 CLI integration tests, 2 TUI integration tests, and 0 doctests.
- 2026-05-07: `cargo build --all-targets` -> passed.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md` -> passed after updating checklists and validation evidence.
- 2026-05-07: `python3 scripts/validate_tdd_workflow.py` -> passed for 2 files.
- 2026-05-07: `git --no-pager diff --check` -> passed.
- 2026-05-07: `python3 /Users/samroberts/plugins/agentic-coding-harness/scripts/harness_section_status.py --dir .harness-validation` -> sandboxed run failed opening the plugin script with `Operation not permitted`; unsandboxed rerun passed with 15 complete, 0 needs_update, 0 failed, next_action `skip`.
- 2026-05-07: `python3 /Users/samroberts/plugins/agentic-coding-harness/scripts/validate_harness.py` -> sandboxed run failed opening the plugin script with `Operation not permitted`; unsandboxed rerun passed with 0 failures and 0 warnings.
- 2026-05-07: VS Code diagnostics -> no errors found.
- 2026-05-07: Symdex watcher status -> `quality_ready` after final docs and runtime review.
- 2026-05-07: After JSON-RPC response serialization polish, `cargo fmt`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, and `cargo build --all-targets` all passed again.
- 2026-05-07: MCP stdio startup bugfix: `cargo test content_length_writer_flushes_response` failed before the fix because framed responses were not flushed after writing; passed after `write_content_length_message` began flushing the writer.
- 2026-05-07: `cargo test mcp::tests` passed with 11 MCP tests after the stdio flush regression test was added.
- 2026-05-07: Subprocess initialize smoke test against `target/debug/vibe-sentinel mcp serve` returned `{"id": 1, "server": "vibe-sentinel"}` within the timeout while stdin stayed open.
- 2026-05-07: Final stdio flush bugfix validation passed: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, `cargo build --all-targets`, `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md`, `python3 scripts/validate_tdd_workflow.py`, and `git --no-pager diff --check`.
- 2026-05-07: VS Code MCP initialize follow-up: exact `.vscode/mcp.json` launch path uses `cargo run -- mcp serve`; subprocess smoke reproduced a framed response but showed the server returned protocol version `2024-11-05` when the client requested `2025-06-18`.
- 2026-05-07: `cargo test initialize_echoes_client_protocol_version` failed before the protocol negotiation fix because initialize returned `2024-11-05`; passed after initialize began echoing the client-requested `protocolVersion` with a `2025-06-18` fallback.
- 2026-05-07: Exact configured subprocess smoke test against `cargo run -- mcp serve` with a VS Code-style initialize payload returned `{"id": 0, "protocolVersion": "2025-06-18", "server": "vibe-sentinel"}` within the timeout.
- 2026-05-07: Protocol negotiation follow-up validation passed: `cargo fmt`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, `cargo build --all-targets`, `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md`, `python3 scripts/validate_tdd_workflow.py`, and `git --no-pager diff --check`.

### Review Notes

- Diff review: completed after formatting, lint, tests, build, docs, and harness
  validation; final polish fixed markdown continuation whitespace and touched
  docs now end cleanly.
- Risks: Fixture coverage validates local Content-Length JSON-RPC behavior, but
  this slice has not been exercised with an external MCP inspector.
- Follow-ups: Create a separate deeper TDD enforcement plan after the MCP status
  surface is shipped and fixture-tested.

## Intended changes

- `docs/app-specs/mcp-status-tool-slice.md`: define first MCP public contract.
- `docs/app-specs/index.md`: link the MCP status slice spec.
- `docs/exec-plans/active/mcp-status-tool.md`: track the modified TDD work.
- `src/lib.rs`: export a future `mcp` module.
- `src/mcp.rs`: add the thin MCP status surface.
- `src/cli.rs`: parse `mcp serve` while preserving existing status commands.
- `src/main.rs`: route the MCP CLI command to the MCP runtime seam.
- `tests/cli_status.rs` or module tests: preserve existing CLI behavior and cover new parsing.
- MCP fixture tests: cover successful status and error mapping behavior.
- `README.md`, `docs/tooling.md`, `docs/observability.md`: document shipped MCP usage and validation after implementation.

## Validation

- `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/mcp-status-tool.md`: required before feature implementation.
- `cargo fmt --check`: required after Rust edits.
- `cargo clippy --all-targets --all-features -- -D warnings`: required after Rust edits.
- `cargo test --all`: required after Rust edits.
- `cargo build --all-targets`: required after Rust edits.
- `python3 scripts/validate_tdd_workflow.py`: required before final review.
- MCP fixture command: `cargo test mcp::tests`.

## Risks and rollback

- Risk: MCP SDK selection may force an approval-required dependency addition.
- Mitigation: Keep the first skeleton dependency-light and document the selected
  crate before adding it.
- Rollback: Revert `src/mcp.rs`, `src/lib.rs`, CLI/main routing changes, MCP
  fixture tests, and MCP docs; existing status CLI/TUI surfaces should remain
  untouched.
- Risk: Public contract changes could drift from the approved narrow status tool.
- Mitigation: Keep `docs/app-specs/mcp-status-tool-slice.md` as the contract and
  defer resources, prompts, write tools, and deeper enforcement.
- Rollback: Remove the MCP slice spec and active plan if the contract is rejected
  before runtime implementation begins.

## Progress log

- 2026-05-07: Created plan after scope-gating the request as `NEEDS_PLAN` and
  confirming the selected first MCP contract.
- 2026-05-07: Added MCP skeleton module, CLI parsing/routing skeletons, and
  skeleton-level tests.
- 2026-05-07: Filled descriptor, status response, and error-mapping units with
  focused validation after each unit.
- 2026-05-07: Completed required Rust formatting, lint, test, and build checks
  for the current implementation units.
- 2026-05-07: Filled dependency-light stdio JSON-RPC runtime and fixture-level
  MCP request/response coverage for initialize, tools/list, tools/call, unknown
  tools, and workspace read failures.

## Decisions

- First MCP surface: read-only status tool.
- MCP tool name: `vibe_sentinel_status`.
- MCP entry point: `vibe-sentinel mcp serve`.
- First slice reuses current status semantics and defers deeper workflow
  enforcement to a later plan.
- Public CLI/MCP contract approval was captured from the user on 2026-05-07.
- Runtime approach: dependency-light local stdio/JSON-RPC using `std::io`,
  `serde`, and `serde_json`; no MCP SDK or async runtime dependency is added in
  this slice.
