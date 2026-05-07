# MCP TDD Gate Slice Spec

## Purpose

Add a local read-only MCP tool that tells coding agents whether a proposed next
modified-TDD workflow step is currently allowed for the active execution plans.

## Users

- Maintainers reviewing whether an agent can safely move to the next feature
  workflow phase.
- Coding agents checking the harness gate before architecture, skeleton, test,
  implementation, or closeout work.
- Future CLI, TUI, or MCP surfaces that need reusable TDD gate semantics.

## Public Contract

- Entry point: `vibe-sentinel mcp serve`.
- Transport: existing local stdio MCP server.
- Tool: `vibe_sentinel_tdd_gate`.
- Tool behavior: read-only, idempotent, local-only, and non-destructive.
- Tool input: object arguments with a required `next_action` string.
- Supported `next_action` values:
  - `start_architecture`
  - `start_skeletons`
  - `start_mock_tests`
  - `start_implementation`
  - `complete_plan`
- Tool output: structured gate data with `allowed`, `current_phase`,
  `blocking_issues`, `warnings`, and `next_allowed_actions`.
- Error behavior: invalid protocol payloads, invalid `next_action` values, and
  workspace read failures return deterministic MCP errors or tool error payloads
  without panics or hidden side effects.

The public MCP contract is approved for implementation by the 2026-05-07 user
instruction to start implementation after reviewing the proposed TDD guardrail
tool.

## Validation Shape

The TDD gate tool returns a structured report shaped for agents:

```json
{
  "project_name": "vibe-sentinel",
  "allowed": false,
  "current_phase": "plan_review_needed",
  "blocking_issues": [
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
  ],
  "warnings": [],
  "next_allowed_actions": ["start_architecture"]
}
```

When no active plans exist, the tool treats the workspace as idle. It blocks
feature-work transitions and reports `current_phase: idle` instead of treating
the repository as corrupt.

## Rule Scope

The first gate slice checks coarse workflow transitions using the existing
active-plan validation rules plus action-specific readiness:

- No active plans means feature workflow actions are blocked with idle-state
  guidance.
- `start_architecture` requires an active plan to exist.
- `start_skeletons` requires Reviewed Plan status to be present and not pending.
- `start_mock_tests` requires Reviewed Architecture status to be present and not
  pending.
- `start_implementation` requires the active-plan validation report to be ready.
- `complete_plan` requires the active-plan validation report to be ready and at
  least one checked implementation item with validation evidence.

This slice intentionally does not infer human approvers, reconstruct exact
chronology from prose logs, or mutate execution-plan files.

## Acceptance Criteria

- `tools/list` advertises `vibe_sentinel_tdd_gate` as a read-only,
  idempotent, local tool with a strict input schema.
- The tool reuses application-core validation logic and does not evaluate plan
  semantics directly in the MCP protocol adapter.
- The tool validates all active non-README markdown plans in deterministic order.
- The tool returns structured `allowed`, `current_phase`, blocking issue,
  warning, and next-action data.
- Invalid `next_action` values are rejected deterministically with JSON-RPC
  `-32602` invalid params errors.
- Missing active plans, semantic validation issues, and unreadable workspace
  inputs are reported deterministically.
- Existing `status`, `status --json`, `status --tui`, `vibe_sentinel_status`,
  and `vibe_sentinel_validate_active_plans` behavior remains compatible.
- No network calls, credential reads, deployment actions, destructive file
  operations, or hidden long-running work are added.

## Non-goals

- Add remote MCP transport.
- Add write-capable MCP tools.
- Add MCP resources, prompts, or multi-tool workflows.
- Parse architecture pseudocode symbols for one-to-one checklist coverage.
- Reconstruct exact chronological sequencing from prose validation logs.
- Infer human approvers or review actors from freeform prose.
- Replace `python3 scripts/validate_tdd_workflow.py`.