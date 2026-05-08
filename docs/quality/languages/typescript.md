# TypeScript Guidelines

## Naming and API design

- Use clear, domain-specific names and avoid abbreviations unless they are standard for the domain.
- Name exported functions and types by behavior and contract, not by implementation detail.
- Keep public APIs narrow and predictable; prefer explicit parameter objects over boolean flags.
- Use `camelCase` for values and functions and `PascalCase` for types, interfaces, and classes.
- Prefer verbs for operations and nouns for data models and result types.

## Types and contracts

- Model invariants in the type system before adding runtime comments or conventions.
- Prefer precise types over `any`; use `unknown` at trust boundaries and narrow it explicitly.
- Keep unions small and purposeful so callers can reason about each case clearly.
- Use readonly types when mutation is not required.
- Do not expose internal implementation types from module boundaries unless they are intentional API.

## Structure and modularity

- Keep files and modules cohesive; split code when one file carries unrelated responsibilities.
- Keep functions focused on one task; extract helpers when control flow obscures intent.
- Keep classes small and cohesive; prefer composition over oversized service objects.
- Separate domain logic from transport, persistence, UI, and framework glue.
- When a function or type becomes hard to explain briefly, treat that as a refactor signal.

## State, collections, and async behavior

- Prefer explicit data flow over hidden mutation of outer scope or shared module state.
- Validate external input at boundaries before it enters core logic.
- Use simple loops when they are clearer than dense chaining across arrays or iterables.
- Use `async` and `await` end to end; avoid mixing promise chains and callback-style flow.
- Accept and forward abort or cancellation signals where work can be interrupted.

## Errors and diagnostics

- Make failure shapes explicit; use thrown errors or typed results consistently within a boundary.
- Write error messages that describe the failed operation and include useful identifiers.
- Preserve original causes when translating errors into higher-level failures.
- Do not swallow exceptions or rejection paths.
- Keep logs structured and tied to actions a caller or operator can understand.

## Testing and maintainability

- Test observable behavior, type-driven edge cases, and failure paths.
- Design core logic so it can run without network, filesystem, clock, or process globals.
- Keep test seams explicit by injecting dependencies instead of reaching through global singletons.
- Remove dead types, stale overloads, and commented-out code quickly.
- Refactor duplication into shared helpers only after the common contract is stable.
