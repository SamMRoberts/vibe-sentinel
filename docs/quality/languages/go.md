# Go Guidelines

## Naming and API design

- Use short, clear names that match the domain and Go conventions.
- Keep exported APIs minimal; export only what other packages truly need.
- Name interfaces by the behavior they provide, not by a generic `Impl` or `Manager` pattern.
- Prefer explicit parameters and return values over hidden package state.
- Keep package boundaries clear so responsibilities stay easy to find.

## Structure and modularity

- Keep functions focused on one job; split validation, parsing, I/O, and transformation work.
- Keep types and packages cohesive; if a package has unrelated reasons to change, split it.
- Prefer small structs with explicit dependencies over large service containers.
- Design core logic so it can run without direct filesystem, network, clock, or environment access.
- Treat long functions or sprawling methods as refactor signals, not normal style.

## Errors and control flow

- Return errors with context that identifies the failed operation and key inputs.
- Check errors immediately and handle them close to the failing boundary.
- Use sentinel errors sparingly; prefer wrapped errors and `errors.Is` or `errors.As`.
- Reserve `panic` for impossible states or unrecoverable startup failures, not normal control flow.
- Keep happy paths easy to read by extracting helper functions instead of nesting heavily.

## Concurrency and data handling

- Start with synchronous code and add goroutines only when concurrency is needed and measurable.
- Use context-aware APIs for cancelable work and pass `context.Context` through call chains.
- Make goroutine ownership explicit, including who starts, stops, and waits for work.
- Prefer clear channel or mutex ownership patterns over shared mutable state spread across packages.
- Avoid hiding expensive allocations or blocking calls behind simple-looking helpers.

## Collections and performance

- Choose concrete slice and map types for implementation details; expose narrower contracts when useful.
- Preallocate slices and maps when sizes are known or bounded.
- Keep loops straightforward; avoid abstraction that hides mutation or allocation behavior.
- Optimize only after measurement, but avoid obviously wasteful conversions and copies.
- Prefer standard library packages before adding custom helpers or external dependencies.

## Testing and maintainability

- Test observable behavior, edge cases, and error paths rather than private helpers.
- Keep tests deterministic by controlling time, randomness, environment, and external I/O.
- Build seams around filesystem, network, process execution, and clocks when tests need substitution.
- Remove dead code, stale comments, and one-off helpers quickly.
- Refactor duplicated logic only after the common shape and ownership are clear.
