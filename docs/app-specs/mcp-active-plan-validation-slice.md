# MCP Active Plan Validation Slice Spec

## Purpose

Add a second local MCP surface for `vibe-sentinel`: a read-only active-plan
validation tool that helps coding agents verify whether current execution plans
are ready for implementation under the TDD workflow.

## Users

- Maintainers reviewing whether active plans contain enough implementation
  evidence to proceed.
- Coding agents checking plan readiness before skeleton or implementation work.
- Future CLI, TUI, or MCP surfaces that need reusable semantic plan validation.

## Public Contract

- Entry point: `vibe-sentinel mcp serve`.
- Transport: existing local stdio MCP server.
- Tool: `vibe_sentinel_validate_active_plans`.
- Tool behavior: read-only, idempotent, local-only, and non-destructive.
- Tool input: no required arguments for this slice; omitted `arguments` and
  empty-object `arguments: {}` are accepted, while non-object or non-empty
  argument objects are rejected deterministically.
- Tool scope: validate all non-README markdown plans in
  `docs/exec-plans/active/`.
- Tool output: structured implementation-readiness data with per-plan checks,
  issues, severity, and evidence snippets.
- Error behavior: invalid protocol payloads and workspace read failures return
  deterministic MCP errors without panics or hidden side effects.

The public MCP contract is approved for implementation by the 2026-05-07
instruction to start implementation of the next MCP execution slice.

## Validation Shape

The active-plan validation tool returns a structured report shaped for agents:

```json
{
  "project_name": "vibe-sentinel",
  "ready": false,
  "plans": [
    {
      "path": "docs/exec-plans/active/example.md",
      "ready": false,
      "checks": [
        {
          "rule_id": "reviewed_plan_not_pending",
          "state": "missing",
          "severity": "error",
          "message": "Reviewed Plan must be recorded before implementation."
        }
      ],
      "issues": [
        {
          "rule_id": "reviewed_plan_not_pending",
          "severity": "error",
          "message": "Reviewed Plan is pending or missing.",
          "evidence": {
            "section": "Reviewed Plan",
            "line": 42,
            "excerpt": "Plan review status: pending"
          }
        }
      ]
    }
  ]
}
```

An empty active-plan directory is valid idle state for `vibe-sentinel status`,
but the validation tool reports no active plans instead of inventing a plan.

## Rule Scope

The first validation slice checks implementation readiness only:

- Reviewed Plan status is present and not pending.
- Reviewed Architecture status is present and not pending.
- Checked implementation items require at least one checked skeleton item.
- Checked implementation items require at least one checked mock-test item.
- Each checked implementation item has a non-empty validation note that is not
  pending.
- Checked implementation work requires validation-log evidence that
  `python3 scripts/validate_tdd_workflow.py` passed.

## Acceptance Criteria

- `tools/list` advertises both `vibe_sentinel_status` and
  `vibe_sentinel_validate_active_plans` as read-only, idempotent, local tools.
- The new tool reuses application-core validation logic rather than evaluating
  plan semantics directly in the MCP protocol adapter.
- The new tool validates all active non-README markdown plans in deterministic
  order.
- The new tool returns structured readiness, check, issue, severity, and
  evidence data for each active plan.
- Missing active plans, invalid plan content, and unreadable workspace inputs are
  reported deterministically.
- Existing `status`, `status --json`, `status --tui`, and
  `vibe_sentinel_status` behavior remains compatible.
- No network calls, credential reads, deployment actions, destructive file
  operations, or hidden long-running work are added.

## Non-goals

- Add remote MCP transport.
- Add write-capable MCP tools.
- Add MCP resources, prompts, or multi-tool workflows.
- Parse architecture pseudocode symbols for one-to-one checklist coverage.
- Reconstruct exact chronological sequencing from prose validation logs.
- Infer human approvers or review actors from freeform prose.
- Replace the Python validation command as the repository's local plan-checking
  helper in this slice.