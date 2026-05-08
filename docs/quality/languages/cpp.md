# C++ Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard terms.
- Keep public APIs small, explicit, and hard to misuse.
- Prefer strong types and enums over booleans, sentinel values, or overloaded meaning.
- Make ownership and lifetime expectations obvious in function signatures.
- Favor readability and consistency over clever template or operator tricks.

## Types and ownership

- Prefer RAII for every owned resource; do not separate acquisition from cleanup without cause.
- Use stack allocation by default and reach for heap allocation only when lifetime requires it.
- Prefer `std::unique_ptr` for single ownership and `std::shared_ptr` only when shared lifetime is real.
- Use `const` aggressively to document invariants and reduce accidental mutation.
- Prefer value semantics for small, coherent types unless reference semantics are required.

## Structure and modularity

- Keep classes small and cohesive; split types with multiple reasons to change.
- Keep methods focused on one task; extract helpers when branching or setup obscures intent.
- Prefer composition over deep inheritance hierarchies and god objects.
- Isolate domain logic from filesystem, network, clock, threading, and UI dependencies.
- Treat classes or functions that are hard to summarize briefly as refactor candidates.

## Containers and algorithms

- Choose standard library containers and algorithms before building custom infrastructure.
- Prefer range-based loops and standard algorithms when they improve clarity.
- Avoid hidden copies in hot paths; be intentional with move semantics, views, and temporaries.
- Use `std::span`, references, and views to express non-owning access where appropriate.
- Do not hide expensive work behind cheap-looking accessors.

## Errors and concurrency

- Use exceptions only when the codebase and API contract are built around them consistently.
- If exceptions are used, throw specific types and preserve actionable context.
- If status-returning APIs are used, keep the convention uniform and check every result.
- Prefer clear ownership and message passing over ad hoc shared mutable state.
- Make locking scope small, explicit, and easy to audit.

## Testing and maintainability

- Test observable behavior, invariants, and failure paths instead of private implementation details.
- Design seams so business logic can run without real files, sockets, clocks, or threads.
- Keep templates and metaprogramming proportionate to the value they provide.
- Remove dead abstractions, stale overloads, and commented-out code quickly.
- Refactor duplication into shared utilities only after the common behavior is stable.
