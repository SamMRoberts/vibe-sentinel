# Python Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard in the problem space.
- Name modules, classes, and functions by responsibility, not by implementation detail.
- Keep public APIs small and explicit; prefer keyword arguments when they improve call-site clarity.
- Follow PEP 8 naming: `snake_case` for functions and variables, `PascalCase` for classes, `UPPER_SNAKE_CASE` for constants.
- Prefer verbs for functions and nouns for data-bearing types.

## Types and validation

- Add type hints to public functions, shared utilities, and complex internal code paths.
- Treat type hints as part of the contract; keep them accurate when behavior changes.
- Validate external inputs at boundaries and raise clear exceptions early.
- Prefer explicit optional types over sentinel `None` values with undocumented meaning.
- Return empty collections instead of `None` unless the absence itself is meaningful.

## Structure and state

- Keep modules focused; split files that mix unrelated domains or responsibilities.
- Keep classes small and cohesive; if a class has multiple reasons to change, split it.
- Keep functions focused on one task; extract helpers when control flow obscures intent.
- Prefer composition of small objects and functions over large utility modules.
- Avoid hidden global state; pass dependencies explicitly when behavior needs configuration.
- Use `dataclass` or small value objects when structured data improves clarity.

## Collections and data flow

- Prefer straightforward loops when they are easier to read than nested comprehensions.
- Avoid clever one-liners that hide branching, mutation, or expensive work.
- Be explicit about mutability; copy shared collections when callers must not observe changes.
- Use generators for streaming pipelines, but materialize results when stable reuse is required.
- Do not hide expensive work behind properties or default argument evaluation.

## Errors and diagnostics

- Raise the most specific exception type that matches the failure mode.
- Write exception messages that explain the failed operation and relevant identifiers.
- Use exceptions for exceptional failures, not normal control flow.
- Preserve the original cause when translating errors with `raise ... from ...`.
- Log actionable context at boundaries, but avoid duplicate noise at every layer.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths, not private implementation details.
- Keep tests deterministic by isolating time, randomness, file I/O, environment, and network access.
- Design units so core logic can run without real files, shells, services, or global process state.
- Prefer dependency injection for collaborators that need substitution in tests.
- When a function or class becomes hard to explain briefly, treat that as a refactor signal.
- Remove dead code, commented-out code, and unused abstractions quickly.
- Refactor duplication into shared helpers only after the common shape is clear.
