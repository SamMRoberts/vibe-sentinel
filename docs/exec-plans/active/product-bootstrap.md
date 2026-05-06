# Execution Plan: Product Bootstrap

## Status

- State: active
- Owner: coding agent
- Created: 2026-05-06
- Last updated: 2026-05-06

## Summary

Bootstrap `vibe-sentinel` from a harness-only repository into a Rust product workspace under the modified TDD workflow. The first implementation slice is not assumed up front; this plan starts by defining a concrete, approval-ready vertical slice, then uses reviewed architecture pseudocode, mockable skeletons, skeleton-level tests, and one-unit-at-a-time implementation before any product behavior is filled in.

## Scope

### In scope

- Define the first concrete Rust product vertical slice and acceptance criteria.
- Update app-spec or feature-spec documentation for the chosen slice.
- Design a Cargo workspace and module topology that preserves the documented layer order.
- Scaffold minimal Rust skeletons for domain types, service traits, application core, adapters, and only the CLI/TUI/MCP surfaces required by the first slice.
- Add skeleton-level tests, mocks, fakes, and fixtures before filling feature implementation bodies.
- Implement the chosen slice one planned skeleton unit at a time with validation after each unit.
- Update README, tooling notes, observability evidence, and plan logs once runnable workflows exist.

### Out of scope

- Defining or implementing broad CLI, TUI, or MCP product contracts beyond the first approved slice.
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

- A concrete first vertical slice is documented with goal, acceptance criteria, constraints, non-goals, surfaces involved, and approval-required public contract decisions.
- The active plan passes `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` before product implementation begins.
- Architecture pseudocode names every planned module, struct, enum, trait, function, and method before Rust skeletons are added.
- Skeleton checklist and mock test checklist map one-to-one to planned units before implementation bodies are filled.
- Skeleton-level tests pass before any feature behavior is implemented.
- Each implementation unit is completed separately, with relevant validation recorded after that unit.
- Final validation results distinguish successful, failed, skipped, and unavailable commands.

## Modified TDD artifacts

### Feature Info

- Goal: bootstrap the first approved `vibe-sentinel` Rust product slice without bypassing the modified TDD workflow.
- Acceptance criteria:
  - Define the first vertical slice before selecting public CLI/TUI/MCP contracts.
  - Preserve the layer order: domain types -> service traits -> application core -> adapters -> CLI/TUI/MCP surfaces.
  - Keep command parsing separate from command execution.
  - Keep ratatui state/render behavior testable without a real terminal if the slice includes TUI work.
  - Keep MCP protocol handling thin and fixture-backed if the slice includes MCP work.
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
  - The product spec is currently high level and does not yet define concrete commands, TUI workflows, MCP tools/resources, storage behavior, or data model.
  - `docs/tooling.md` lists Cargo validation commands that become runnable after workspace scaffolding exists.
  - `scripts/validate_tdd_workflow.py` verifies required plan headings but not review quality, pseudocode completeness, or validation evidence quality.

### Reviewed Plan

- Plan review status: pending
- Refinements made:
  - User selected product bootstrap as the target.
  - User selected defining the first slice during the plan rather than assuming CLI-first, TUI-first, or MCP-first implementation.

### Architecture Pseudocode

This section is intentionally not final. Architecture pseudocode must be completed and reviewed after the first vertical slice is defined and before any Rust skeleton files are created.

```text
workspace vibe-sentinel
  package vibe-sentinel
    binary vibe-sentinel
    library vibe_sentinel

module domain
  enum SentinelError
    variant InvalidInput
    variant OperationFailed
  struct ValidationIssue
    field message: String
  struct SentinelRequest
    field subject: String
  struct SentinelReport
    field summary: String

module ports
  trait Clock
    fn now(...) -> ...
  trait ReportSink
    fn write_report(...) -> ...
  trait SentinelService
    fn evaluate(...) -> ...

module core
  struct SentinelApp
    field service: SentinelService
  fn run_sentinel(...) -> ...

module adapters
  module memory
    struct InMemoryReportSink
      field reports: ...
    fn write_report(...) -> ...

module cli
  struct CliArgs
    field command: ...
  enum CliCommand
    variant Check
  fn parse_args(...) -> ...
  fn execute(...) -> ...

module tui
  struct TuiState
    field report: ...
  fn update(...) -> ...
  fn render(...) -> ...

module mcp
  struct McpToolRequest
    field payload: ...
  struct McpToolResponse
    field payload: ...
  fn handle_tool(...) -> ...
```

### Reviewed Architecture

- Architecture review status: pending
- Refinements made:
  - Placeholder topology follows the documented layer order.
  - Final module/type/function list must be narrowed after the first slice is defined and reviewed.

### Skeleton Checklist

- [ ] First vertical slice documented and approved for implementation.
- [ ] Final architecture pseudocode reviewed.
- [ ] Cargo workspace skeleton added with no feature implementation bodies.
- [ ] Domain type skeletons added with minimal value semantics only.
- [ ] Service trait skeletons added for all application boundaries needed by the first slice.
- [ ] Application core skeletons added behind traits.
- [ ] Adapter skeletons added only where needed by the first slice.
- [ ] CLI surface skeleton added only if included in the first slice.
- [ ] TUI surface skeleton added only if included in the first slice.
- [ ] MCP surface skeleton added only if included in the first slice.

### Mock Test Checklist

- [ ] Plan validation test command passes for this active plan.
- [ ] Cargo workspace builds with skeleton code only.
- [ ] Domain skeleton tests cover basic construction or validation behavior.
- [ ] Service-trait fake or mock supports application-core skeleton tests.
- [ ] Application-core skeleton tests exercise behavior through traits.
- [ ] CLI parsing skeleton tests exist if the first slice includes CLI work.
- [ ] ratatui state/render skeleton tests exist if the first slice includes TUI work.
- [ ] MCP request/response fixture tests exist if the first slice includes MCP work.

### Implementation Checklist

- [ ] Define and document the first vertical slice.
- Validation after this unit: run `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md`.
- [ ] Finalize and review architecture pseudocode.
- Validation after this unit: rerun `python3 scripts/validate_tdd_workflow.py docs/exec-plans/active/product-bootstrap.md` and manually verify every planned unit has checklist coverage.
- [ ] Scaffold Cargo workspace and skeleton modules.
- Validation after this unit: run `cargo fmt --check` and `cargo build --all-targets` if Cargo workspace exists.
- [ ] Add skeleton-level tests and fixtures.
- Validation after this unit: run the smallest relevant `cargo test` command and record the result.
- [ ] Fill first skeleton unit.
- Validation after this unit: run the relevant focused tests before filling another unit.
- [ ] Continue one skeleton unit at a time until the approved slice is complete.
- Validation after each unit: record the focused command and result.

### Validation Log

- 2026-05-06: Created active plan; validation not yet run.

### Review Notes

- Diff review: pending
- Risks:
  - The product behavior is currently under-specified, so the first implementation unit must be requirements discovery and slice definition.
  - Cargo validation commands are unavailable until workspace scaffolding exists.
  - Public contract choices require human approval before implementation.
  - The TDD validator checks required headings only, so plan and architecture review must still be done manually.
- Follow-ups:
  - Consider adding stronger TDD validator checks after the first slice proves the workflow.
  - Update `README.md` after the first runnable workflow exists.

## Intended changes

- `docs/app-specs/app-spec.md` or a new file under `docs/app-specs/`: document the first vertical slice after requirements are defined.
- `docs/app-specs/index.md`: register any new feature spec.
- `docs/design-docs/`: add a design doc if workspace topology or public contracts need durable explanation.
- `docs/references/`: add durable external references if crate or protocol research is needed.
- `Cargo.toml`: add workspace metadata after plan and architecture review.
- Rust source tree: add skeleton modules after plan and architecture review.
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

## Decisions

- Product bootstrap is the first implementation track.
- The first concrete slice will be defined during this plan instead of being assumed.
- No Rust implementation bodies will be added until reviewed architecture, mockable skeletons, and passing skeleton-level tests exist.
- Public CLI, TUI, MCP, storage, or wire-contract choices must be treated as approval-required before implementation.
