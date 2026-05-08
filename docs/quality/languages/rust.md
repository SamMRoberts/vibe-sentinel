# Rust Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard Rust terms.
- Keep public APIs explicit and small; prefer types that make invalid states hard to represent.
- Separate parsing, validation, domain logic, and transport concerns cleanly.
- Prefer enums and newtypes over booleans, strings, or loosely typed flags.
- Make ownership, borrowing, and mutability choices obvious from the signature.

## Types and ownership

- Default to immutable data and narrow mutation scope to the smallest useful block.
- Model invariants in the type system before relying on comments or runtime convention.
- Use borrowing when it simplifies ownership without making call sites harder to reason about.
- Prefer owned return values when borrowed lifetimes would complicate the API unnecessarily.
- Reach for `Arc`, `Rc`, `Mutex`, or `RefCell` only when simpler ownership models fail clearly.

## Structure and modularity

- Keep functions focused on one task; extract helpers when control flow or setup hides intent.
- Keep modules cohesive; if a file mixes unrelated responsibilities, split it.
- Prefer composition through traits and small types over large all-knowing structs.
- Design core logic so it can run without direct terminal, filesystem, network, clock, or process access.
- Treat code that is hard to explain briefly as a refactor signal.

## Errors and async work

- Use `Result` and `Option` deliberately; choose the type that matches the contract.
- Add context to errors at subsystem boundaries so failures remain actionable.
- Reserve `panic!` for impossible states, test scaffolding, or explicit crash-on-bug policy.
- Keep async boundaries explicit and pass cancellation or shutdown signals through the stack.
- Do not block inside async code unless the runtime and call path are designed for it.

## Collections and performance

- Choose the simplest collection that matches access and ownership needs.
- Avoid unnecessary cloning; clone where it clarifies ownership, not as a reflex.
- Prefer iterators when they stay readable; use loops when they make state transitions clearer.
- Keep allocation and copy behavior visible in hot paths.
- Optimize after measurement, but remove obvious waste and redundant conversions early.

## Testing and maintainability

- Test observable behavior, invariants, and failure modes instead of private implementation details.
- Keep tests deterministic by isolating time, randomness, terminal state, filesystem, and network access.
- Build seams around external processes, clocks, storage, and protocol boundaries for testability.
- Remove dead code, stale feature flags, and speculative abstractions quickly.
- Refactor repeated logic into shared helpers only after the common contract is clear.
