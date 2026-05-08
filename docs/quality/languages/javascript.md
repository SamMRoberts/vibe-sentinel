# JavaScript Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are widely understood.
- Name functions by intent, not by incidental implementation detail.
- Keep public module APIs small and predictable; prefer explicit options objects over positional flags.
- Use `camelCase` for variables and functions and `PascalCase` for classes and constructor-like types.
- Prefer verbs for operations and nouns for values, models, and state containers.

## Structure and modularity

- Keep modules focused on one area of behavior; split files that mix unrelated responsibilities.
- Keep functions small and single-purpose; extract helpers when branching or setup hides intent.
- Prefer composition of small utilities over large shared helper files.
- Keep classes small and cohesive; if a class has multiple reasons to change, split it.
- When code becomes hard to explain briefly, treat that as a refactor signal.

## State and data flow

- Prefer explicit inputs and return values over hidden mutation of outer scope.
- Keep mutable state local and minimize shared writable objects.
- Validate external data at boundaries before it reaches core logic.
- Return consistent shapes from a function; avoid mixing sentinel values with normal results.
- Use plain objects for simple data and richer abstractions only when behavior justifies them.

## Collections and async behavior

- Prefer readable loops when they are clearer than dense chaining with `map`, `filter`, and `reduce`.
- Avoid repeated array passes in hot paths when one pass is simpler and cheaper.
- Use `async` and `await` instead of raw promise chains for multi-step control flow.
- Propagate cancellation or abort signals where operations can be interrupted.
- Do not block on asynchronous work through polling, sleeps, or implicit ordering assumptions.

## Errors and observability

- Throw or return errors in a way that matches the module contract; do not mix styles casually.
- Write error messages that explain the failed operation and include key identifiers.
- Preserve original failure context when wrapping errors.
- Do not swallow exceptions; log, translate, or rethrow where the caller can act.
- Keep logging structured and purposeful instead of noisy debug output.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths instead of private implementation details.
- Design code so business logic can run without network, filesystem, clock, or process globals.
- Isolate framework glue from core logic so units stay easy to exercise directly.
- Remove dead code, commented-out code, and stale feature toggles quickly.
- Refactor duplication after the common shape is clear, not before.
