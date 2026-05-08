# PHP Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard in the codebase.
- Name classes, methods, and functions by intent rather than implementation detail.
- Keep public APIs small and predictable; prefer explicit parameter objects or named arguments over flag piles.
- Use `PascalCase` for classes, `camelCase` for methods and variables, and consistent file-to-type mapping.
- Prefer verbs for operations and nouns for data-carrying types and services.

## Types and validation

- Declare parameter, return, and property types wherever the language and runtime support them.
- Use strict comparisons and enable strict typing where the project allows it.
- Validate external input at boundaries before it reaches domain logic.
- Model optionality explicitly instead of relying on loose truthiness.
- Return consistent result shapes rather than mixing arrays, `false`, and exceptions casually.

## Structure and modularity

- Keep classes focused on one responsibility; split types with multiple reasons to change.
- Keep methods small and single-purpose; extract helpers when control flow obscures intent.
- Prefer composition of focused services over large utility or manager classes.
- Separate domain logic from framework, transport, storage, and view concerns.
- When a method or class becomes hard to explain briefly, treat that as a refactor signal.

## State, collections, and control flow

- Keep mutable state local and avoid hidden writes through globals or shared service state.
- Prefer simple loops and explicit conditionals when they are clearer than dense array pipelines.
- Be deliberate about array shape; document or model associative structures that act like records.
- Avoid hidden work in magic methods unless the behavior is conventional and obvious.
- Keep side effects explicit so callers can reason about persistence, I/O, and cache updates.

## Errors and observability

- Use exceptions for exceptional failures and expected result values for normal control flow.
- Throw specific exception types where callers need distinct handling.
- Write error messages that explain the failed operation and relevant identifiers.
- Do not swallow exceptions; translate, log, or rethrow where a caller can respond.
- Keep logs structured and focused on actions that matter operationally.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths instead of private implementation details.
- Design business logic so it can run without framework bootstrapping, network, or database coupling.
- Inject dependencies where it improves substitution and direct unit testing.
- Remove dead code, commented-out code, and stale compatibility layers quickly.
- Refactor duplication into shared helpers only after the common behavior is clear.
