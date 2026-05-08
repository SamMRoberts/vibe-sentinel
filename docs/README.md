# Documentation Map

This folder is the system of record for coding agents working on `vibe-sentinel`.

## Required folders

- `harness/`: scope, initialization, open questions, and the operating model.
- `design-docs/`: decisions, design history, and core beliefs.
- `exec-plans/`: active plans, completed plans, technical debt, and the plan template.
- `app-specs/`: product requirements and user-facing behavior.
- `quality/`: implementation-level language, data, surface, and engineering guides that extend but do not replace the top-level policy docs.
- `references/`: external or copied reference material made repo-local.

## Required top-level docs

- `architecture.md`: Rust CLI, ratatui TUI, and application-core boundaries.
- `tooling.md`: Cargo commands and harness validation workflows.
- `quality.md`: Quality bar and implementation invariants.
- `security.md`: security and privacy constraints.
- `reliability.md`: reliability and operational expectations.
- `observability.md`: CLI, TUI, logs, and validation evidence workflows.
- `review.md`: review loop and TDD compliance checks.

Update this map when docs are added, moved, retired, or replaced.
