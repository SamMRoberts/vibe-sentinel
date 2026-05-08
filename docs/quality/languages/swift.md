# Swift Guidelines

## Naming and API design

- Use clear, domain-specific names that read naturally at the call site.
- Name types, methods, and properties by intent, not implementation detail.
- Keep public APIs small and explicit; prefer expressive parameter labels over overloaded shortcuts.
- Use `UpperCamelCase` for types and `lowerCamelCase` for methods, properties, and locals.
- Prefer verbs for functions and nouns for types that model data or capabilities.

## Optionals and validation

- Make optionality meaningful; use `nil` only when absence is a real part of the model.
- Validate external input at boundaries and fail early with precise errors.
- Avoid force unwraps unless the invariant is local, obvious, and impossible to express better.
- Normalize invalid or incomplete input close to the source instead of scattering checks.
- Return empty collections instead of optional collections.

## Types and state

- Prefer value types for plain data and local state when reference semantics are unnecessary.
- Keep types small and cohesive; if a type has multiple reasons to change, split it.
- Keep functions focused on one task; extract helpers when branching or setup obscures intent.
- Prefer composition and protocol-oriented design over deep inheritance trees.
- Limit mutable shared state and make ownership explicit when mutation is required.

## Collections and performance

- Choose the narrowest collection abstraction that matches the contract.
- Prefer straightforward loops when they are clearer than long chains of higher-order functions.
- Be mindful of copying, bridging, and allocation costs in hot paths.
- Do not hide expensive work behind computed properties with surprising behavior.
- Measure before introducing low-level micro-optimizations.

## Concurrency and async work

- Use `async` and `await` end to end; avoid blocking async code with semaphores or sleeps.
- Make cancellation behavior explicit and check for cancellation in long-running work.
- Isolate mutable state behind clear ownership or actors when concurrency is involved.
- Keep task creation intentional; avoid spawning detached work without a strong lifecycle reason.
- Define timeout, retry, and partial-failure behavior at boundaries that call unreliable dependencies.

## Errors and diagnostics

- Use `throw` for recoverable failures and `precondition` or `assertionFailure` only for programmer errors.
- Surface errors with messages that explain the failed operation and important identifiers.
- Preserve underlying errors when translating failures across layers.
- Log enough context to diagnose issues without leaking sensitive data.
- Do not silently ignore errors; handle, propagate, or explicitly downgrade them.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths, not private implementation details.
- Design units around test seams so core logic can run without filesystem, network, clock, or UI dependencies.
- Prefer small protocols or concrete collaborators only where they improve substitution and testing.
- Keep tests deterministic by isolating time, randomness, and task scheduling assumptions.
- When a function or type becomes hard to explain briefly, treat that as a refactor signal.
- Remove dead code, commented-out code, and unused abstractions quickly.
