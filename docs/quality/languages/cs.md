# C# Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard .NET terms.
- Name types, methods, and properties by intent, not implementation detail.
- Keep public APIs small and predictable; prefer explicit parameters over magic flags.
- Use `PascalCase` for public members and `camelCase` for locals and parameters.
- Prefer verbs for methods and nouns for types that represent data or behavior.

## Nullability and validation

- Enable nullable reference types and treat warnings as design feedback, not noise.
- Express optional values in the type system instead of relying on `null` by convention.
- Validate external inputs at boundaries and fail early with clear argument messages.
- Avoid `!` unless you can prove the invariant locally and keep that proof obvious.
- Return empty collections instead of `null`.

## Types and state

- Prefer immutable types for shared data and request models when mutation is unnecessary.
- Use `record` or `record struct` when value semantics improve clarity.
- Keep classes focused on one responsibility and inject collaborators explicitly.
- Keep classes small and cohesive; if a class has multiple reasons to change, split it.
- Favor small methods and cohesive types over utility classes with unrelated behavior.
- Keep methods focused on one task; extract helpers when branching or setup obscures intent.
- Prefer composing small services over large `Manager`, `Processor`, or `Helper` types.
- Use `var` when the type is obvious from the right side; otherwise spell the type out.

## Collections and LINQ

- Choose the narrowest collection interface that matches the contract, such as `IReadOnlyList<T>`.
- Materialize LINQ queries when you need stable results or repeated enumeration.
- Avoid complex LINQ chains when a short loop is easier to read and debug.
- Be conscious of allocations in hot paths, especially with closures, boxing, and repeated projections.
- Do not hide expensive work behind properties or deferred query composition.

## Async and concurrency

- Use `async` and `await` end to end; avoid `.Result`, `.Wait()`, and sync-over-async code.
- Accept `CancellationToken` in cancellable async APIs and pass it through to dependencies.
- Keep async methods side-effect aware; make cancellation and partial completion behavior explicit.
- Use `ValueTask` only when measurement shows it matters and the calling contract stays simple.
- Protect shared mutable state with clear ownership instead of ad hoc locking.

## Errors and diagnostics

- Use exceptions for exceptional failures, not routine control flow.
- Throw the most specific exception type that matches the contract violation.
- Write exception and log messages that explain the failed operation and key identifiers.
- Preserve original stack traces by rethrowing with `throw;` when handling only to add context.
- Do not swallow exceptions; translate or log them where the caller can act on the failure.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths, not private implementation details.
- Keep tests deterministic by isolating time, randomness, file I/O, and network access.
- Design units around test seams so business logic can run without file I/O, network, time, or static global state.
- Prefer constructor injection and interfaces only where they improve substitution and testing.
- When a method or class becomes hard to explain briefly, treat that as a refactor signal.
- Remove dead code, commented-out code, and unused abstractions quickly.
- Refactor duplicated logic into shared helpers only after the common shape is clear.
