# Harness Scope

## Project

- Name: vibe-sentinel
- Purpose: Rust CLI, ratatui TUI, and MCP feature development under a strict modified TDD workflow.
- Primary users/operators: maintainers and coding agents building, testing, and reviewing Rust features.

## In scope for coding agents

- Rust CLI feature work with command parsing separated from business logic.
- ratatui TUI feature work with render/state behavior isolated for tests.
- MCP server, tool, resource, and protocol-adapter work that stays thin at the boundary.
- Tests, mocks, fakes, fixtures, and validation scripts that support the modified TDD workflow.
- Harness maintenance, docs updates, and execution plans that keep future agent work enforceable.

## Out of scope for coding agents

- Non-Rust application rewrites or replacing the CLI/TUI/MCP product shape.
- Production credential changes, destructive data operations, or unapproved deployment changes.
- Filling feature implementation bodies before reviewed plans, reviewed architecture pseudocode, skeletons, and skeleton-level tests exist.
- Unapproved architecture rewrites or work not represented in the harness scope.

## Requires explicit human approval

- Security model changes.
- Public CLI, TUI, MCP, storage, or wire-contract changes.
- Data migrations.
- Dependency swaps or runtime/package-manager changes.
- Deployment changes.
- Scope expansion or bypassing the modified TDD process.

## Scope gate

Before any code change, classify the request:

- `IN_SCOPE`: allowed by this document and not conflicting with any harness rule.
- `NEEDS_PLAN`: allowed, but requires an execution plan before implementation.
- `HARNESS_UPDATE_REQUIRED`: useful, but not currently included in scope.
- `OUT_OF_SCOPE`: explicitly excluded by this document.
- `CONFLICTS_WITH_HARNESS`: violates documented architecture, product, quality, security, reliability, validation, or process constraints.

Only `IN_SCOPE` work may proceed directly. `NEEDS_PLAN` work may proceed after an active plan is created or read.

## Blocked-change response

If a request is `HARNESS_UPDATE_REQUIRED`, `OUT_OF_SCOPE`, or `CONFLICTS_WITH_HARNESS`, do not edit code. Respond with:

```text
Harness gate blocked this change.
Reason: <specific doc/rule that blocks it>
Resolution options:
1. Update the harness scope/docs, then create a new plan.
2. Stop this task with no changes.
3. Create a new in-scope plan that satisfies the current harness.
```

If the user chooses to update the harness, update documentation first. Do not implement the originally blocked code change until the updated harness clearly permits it.
