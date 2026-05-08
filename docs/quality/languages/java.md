# Java Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard JVM terms.
- Name classes, methods, and fields by intent, not implementation detail.
- Keep public APIs small and explicit; prefer readable parameters over boolean flags.
- Use `PascalCase` for types and `camelCase` for methods, fields, and locals.
- Prefer verbs for methods and nouns for types that represent data or behavior.

## Nullability and validation

- Make null-handling explicit at boundaries and document when `null` is accepted or returned.
- Prefer empty collections and optional wrappers over sentinel `null` values.
- Validate external input early and fail with precise exception messages.
- Avoid deep chains of null checks by normalizing inputs and extracting small helpers.
- Use annotations and static analysis consistently when the codebase supports them.

## Types and state

- Prefer immutable data objects for values that do not need mutation after construction.
- Keep classes small and cohesive; if a class has multiple reasons to change, split it.
- Keep methods focused on one task; extract helpers when branching or setup obscures intent.
- Prefer composition over inheritance unless a true subtype relationship is stable and obvious.
- Avoid god objects, generic `Util` classes, and shared mutable state.

## Collections and streams

- Choose the narrowest collection type that matches the contract.
- Use streams for clear transformations, not to compress complicated control flow.
- Prefer a short loop when it is easier to read, debug, or profile than a long stream chain.
- Be conscious of allocations, boxing, and repeated traversal in hot paths.
- Do not hide expensive work behind getters or lazy pipelines.

## Concurrency and async work

- Keep ownership of mutable state clear before adding threads, pools, or reactive flows.
- Propagate interruption and cancellation correctly; do not swallow `InterruptedException`.
- Prefer structured concurrency patterns provided by the platform or framework in use.
- Make timeout, retry, and partial-failure behavior explicit at service boundaries.
- Avoid blocking inside code paths that are expected to stay responsive or scalable.

## Errors and diagnostics

- Use exceptions for exceptional failures, not routine branching.
- Throw the most specific exception type that matches the contract violation.
- Write error messages that explain the failed operation and key identifiers.
- Log with enough context to diagnose production failures without dumping sensitive data.
- Preserve root causes when translating exceptions across layers.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths, not private implementation details.
- Design units around test seams so business logic can run without filesystem, network, clock, or process dependencies.
- Prefer constructor injection for collaborators when it improves substitution and test control.
- Keep tests deterministic by isolating time, randomness, and concurrency.
- When a method or class becomes hard to explain briefly, treat that as a refactor signal.
- Remove dead code, commented-out code, and unused abstractions quickly.

