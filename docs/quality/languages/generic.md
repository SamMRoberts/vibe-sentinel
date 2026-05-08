# Generic Language Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard in the language or domain.
- Name functions, methods, classes, and modules by intent, not implementation detail.
- Keep public APIs small, explicit, and hard to misuse.
- Prefer expressive types, parameters, and return values over hidden flags or implicit conventions.
- Favor readability and consistency over clever or overly compact code.

## Structure and modularity

- Keep functions and methods focused on one task; extract helpers when branching or setup obscures intent.
- Keep classes, modules, and files cohesive; split units with multiple unrelated responsibilities.
- Prefer composition of small units over large all-knowing objects or utility files.
- Treat long functions, large classes, and sprawling files as refactor signals.
- Design core logic so it can run without direct filesystem, network, clock, process, or UI dependencies.

## Data and state

- Make ownership, mutability, and lifetime expectations obvious in the code shape available to the language.
- Prefer immutable data and narrow mutation scope unless shared mutable state is clearly required.
- Validate external input at boundaries before it reaches core logic.
- Return consistent result shapes and make optionality or failure explicit.
- Keep hidden global state to a minimum and document any unavoidable shared state clearly.

## Errors and diagnostics

- Fail early at boundaries with precise, actionable error messages.
- Use the language's normal error model consistently instead of mixing patterns casually.
- Preserve the original failure context when adding logging or wrapping errors.
- Do not swallow failures silently; translate or report them where callers can act.
- Keep logs and diagnostics specific enough to identify the failed operation and key inputs.

## Testing and maintainability

- Test observable behavior, edge cases, and failure paths rather than private implementation details.
- Create test seams so business logic can run without external side effects.
- Isolate time, randomness, file I/O, network, and process boundaries so tests stay deterministic.
- Remove dead code, commented-out code, and abandoned abstractions quickly.
- When code becomes hard to explain briefly, treat that as a refactor signal.
