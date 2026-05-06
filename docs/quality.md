# Quality Bar

## Quality invariants

- Follow the modified TDD workflow for all feature development.
- Prefer legible, tested, documented patterns.
- Validate data at CLI, TUI, MCP, storage, and external-process boundaries.
- Avoid hidden coupling between application core and UI/protocol adapters.
- Keep future agent runs easy to reason about.

## General expectations

- Keep behavior legible to future agent runs.
- Prefer explicit names and small, testable units.
- Avoid speculative abstractions.
- Do not spread inconsistent patterns.
- Promote repeated review feedback into docs or mechanical checks.
- Skeleton code may be minimal, but it must be intentionally mockable.
- Implementation should land one planned unit at a time with validation evidence.
