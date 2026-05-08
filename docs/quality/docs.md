# Documentation Quality Guidelines

## When to update docs

- Update docs in the same change when behavior, commands, validation, scope, or durable review guidance changes.
- Treat architecture, security, reliability, and quality rule changes as documentation work, not optional follow-up.
- Promote repeated review feedback into docs or checks, consistent with `docs/review.md`.
- Stop and resolve drift when code and docs disagree instead of leaving both half-correct.

## Placement

- Put durable engineering rules under `docs/quality/`, not in `AGENTS.md`.
- Put architecture decisions in `docs/architecture.md` or a design doc when they need more than a brief rule.
- Put security policy and security implementation guidance in `docs/security.md`; keep other local implementation advice in supporting guides.
- Prefer repo-local references over relying on chat history or undocumented conventions.

## Writing style

- Keep guides concise, scannable, and specific to the repo's CLI, TUI, MCP, and harness workflow.
- Use short sections and practical bullets that tell a future agent what to do or avoid.
- Prefer explicit examples of ownership and boundaries over generic best-practice slogans.
- Write so another agent can act without inferring hidden rules.

## Evidence and commands

- Document relevant validation commands from `docs/tooling.md` when behavior or workflows depend on them.
- When a command is skipped or unavailable, record that clearly in the plan or final response.
- Keep command names, file paths, and ownership points accurate so a future agent can rerun them locally.
- Do not claim validation or behavior that the repo does not actually implement.

## Maintenance

- Remove stale examples, outdated caveats, and empty placeholders quickly.
- Keep indexes and cross-references current when adding new guide categories or subtrees.
- Split oversized docs when multiple audiences or concerns make one file hard to navigate.
- Treat documentation that becomes hard to summarize briefly as a signal to refactor its structure.
