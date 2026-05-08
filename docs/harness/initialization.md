# Harness Initialization

Initialized: 2026-05-06
Project slug: `vibe-sentinel`

## Captured answers

| Question | Answer |
| --- | --- |
| Project or product name | vibe-sentinel |
| One-sentence purpose | Rust CLI and ratatui TUI feature development under a strict TDD workflow. |
| Primary users/operators | Maintainers and coding agents building, testing, and reviewing Rust features. |
| Agent in-scope work | Rust CLI, ratatui TUI, tests, mocks/fakes, validation scripts, docs, and harness maintenance. |
| Agent out-of-scope work | Non-Rust rewrites, production credential changes, destructive data operations, unapproved deployment changes, unapproved architecture rewrites, and implementation work that bypasses the TDD workflow. |
| Approval-required changes | Security model changes, public CLI/TUI/storage contracts, data migrations, dependency swaps, deployment changes, scope expansion, and bypassing TDD gates. |
| Product domains/app areas | CLI, ratatui TUI, application core, developer tooling, documentation, validation harness. |
| Architecture boundaries | Domain types -> service traits -> application core -> adapters -> CLI/TUI surfaces, with cross-cutting providers entering through explicit interfaces. |
| Tech stack conventions | Rust, Cargo, ratatui for terminal UI, and Python only for lightweight repository validation scripts. |
| Validation commands | `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`, `cargo build --all-targets`, and `python3 scripts/validate_tdd_workflow.py`. |
| Observability workflows | CLI output snapshots, ratatui render/state tests, structured logs, and explicit validation logs in execution plans. |
| Security/reliability constraints | Preserve testable boundaries, do not hide long-running tasks, validate all external input at boundaries, and do not weaken security or reliability behavior without approval. |
| Documentation locations | Design docs: docs/design-docs/. Execution plans: docs/exec-plans/. App specs: docs/app-specs/. References: docs/references/. |
| Plan process | Active plans live in docs/exec-plans/active/. Completed plans move to docs/exec-plans/completed/. Technical debt is tracked in docs/exec-plans/tech-debt-tracker.md. |
| Quality invariants | TDD artifacts are required, implementation proceeds one skeleton unit at a time, boundaries stay mockable, and durable lessons become docs or validator checks. |

## Reinitialization rule

Do not overwrite these answers silently. When project direction changes, update the relevant docs and add a dated note explaining the change.

## Direction changes

- 2026-05-08: Removed the local Vibe Sentinel MCP server from product scope.
  The product now ships with the CLI and ratatui TUI only. Symdex MCP evidence
  references remain because they describe an external agent workflow, not a
  Vibe Sentinel runtime surface.
