# AGENTS.md

## Purpose

This file is the routing map for coding agents working on vibe-sentinel.
Project purpose: Rust CLI, ratatui TUI, and MCP feature development under a strict modified TDD workflow.
Keep this file short; put durable project knowledge in `docs/`.

## Harness rule

Before code edits, read this file, then `docs/harness/scope.md`.
Do not edit code that conflicts with the harness, is out of scope,
or is not included in the harness scope.

## Table of contents

| Context | Read first |
| --- | --- |
| Harness scope, exclusions, approvals | `docs/harness/scope.md` |
| Initialization answers and setup history | `docs/harness/initialization.md` |
| Modified TDD operating model | `docs/harness/operating-model.md` |
| Documentation map | `docs/README.md` |
| Product intent and users | `docs/app-specs/app-spec.md` |
| Product specs index | `docs/app-specs/index.md` |
| Architecture and boundaries | `docs/architecture.md` |
| Design docs and decisions | `docs/design-docs/index.md` |
| Core engineering beliefs | `docs/design-docs/core-beliefs.md` |
| Active plans | `docs/exec-plans/active/` |
| Plan template | `docs/exec-plans/plan-template.md` |
| Completed plans | `docs/exec-plans/completed/` |
| Technical debt | `docs/exec-plans/tech-debt-tracker.md` |
| Commands, tools, package manager | `docs/tooling.md` |
| Quality bar and invariants | `docs/quality.md` |
| Security constraints | `docs/security.md` |
| Reliability constraints | `docs/reliability.md` |
| Logs, metrics, traces, UI validation | `docs/observability.md` |
| Review expectations | `docs/review.md` |
| External or copied references | `docs/references/index.md` |
| Symdex MCP evidence workflow (any repository) | `docs/references/symdex-mcp-tools-usage.md` |

## Scope gate

Classify every requested code change before editing.

- `IN_SCOPE`: allowed by `docs/harness/scope.md`.
- `NEEDS_PLAN`: allowed, but requires an execution plan.
- `HARNESS_UPDATE_REQUIRED`: useful, but not currently included.
- `OUT_OF_SCOPE`: explicitly excluded.
- `CONFLICTS_WITH_HARNESS`: violates a harness rule.

Proceed only for `IN_SCOPE`, or `NEEDS_PLAN` after creating/reading a plan.

## Blocked-change response

When blocked, do not patch code. Respond with:

```text
Harness gate blocked this change.
Reason: <specific doc/rule that blocks it>
Resolution options:
1. Update the harness scope/docs, then create a new plan.
2. Stop this task with no changes.
3. Create a new in-scope plan that satisfies the current harness.
```

## Planning requirements

Create an active plan for multi-file, architectural, security, reliability,
or user-visible behavior changes.

Active plans live in `docs/exec-plans/active/`.
Completed plans move to `docs/exec-plans/completed/`.
Use `docs/exec-plans/plan-template.md`.
Feature plans must include the modified TDD artifacts required by
`docs/harness/operating-model.md`.

## Documentation requirements

Update docs when a change affects product behavior, architecture,
commands, validation, scope, security, reliability, or quality rules.

Add design docs under `docs/design-docs/`.
Add execution plans under `docs/exec-plans/`.
Add app specs under `docs/app-specs/`.
Add reference docs under `docs/references/`.

## Architecture requirements

Preserve these boundaries: domain types -> service traits -> application core -> adapters -> CLI/TUI/MCP surfaces.
Do not add cross-layer imports or hidden coupling that bypasses documented boundaries.
When architecture needs to change, update docs first and create an execution plan.

## Validation requirements

Run the commands listed in `docs/tooling.md` for the files you changed.
If commands cannot run, explain why and record the risk in the plan or final response.
Do not invent validation results.

## Quality requirements

Follow `docs/quality.md` for naming, file size, testing, logging,
schema validation, and maintainability expectations.
Prefer shared utilities and documented patterns over one-off helpers.
Do not fill implementation bodies for feature work before skeleton-level
tests exist and pass.

## Security and reliability

Follow `docs/security.md` and `docs/reliability.md`.
Do not weaken auth, validation, privacy, rate limits, retries, timeouts,
observability, or error handling without explicit harness approval.

## References

Use `docs/references/` for external docs copied into the repo.
Prefer repo-local references over relying on inaccessible chat history or memory.

## Drift control

If code and docs disagree, stop and identify the discrepancy.
Ask whether to update docs, update code, or create a new execution plan.
Record recurring issues in `docs/exec-plans/tech-debt-tracker.md`.
