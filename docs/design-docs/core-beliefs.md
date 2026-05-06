# Core Beliefs

## Agent legibility

Anything important for coding agents must be visible in the repository. Chat
threads, memory, and external docs do not count unless summarized or copied into
`docs/`.

## Repository as system of record

Project knowledge should live in versioned markdown, schemas, code, tests,
scripts, plans, and references.

## Modified TDD over implementation rush

Feature work starts with information, research, reviewed planning, and reviewed
architecture pseudocode. Code starts as mockable skeletons. Implementation lands
one skeleton unit at a time after skeleton-level tests pass.

## Feedback loops

When quality issues repeat, convert them into docs, tests, linters, scripts, or
execution-plan checks.

## Project-specific quality invariants

- Keep Rust CLI, ratatui TUI, and MCP boundaries thin and testable.
- Validate data at all external boundaries.
- Avoid hidden coupling between application core and adapters.
- Keep future agent runs easy to reason about.
