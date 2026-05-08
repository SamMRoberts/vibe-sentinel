# C Guidelines

## Naming and interfaces

- Use clear, domain-specific names; avoid abbreviations unless they are standard in the codebase.
- Name functions by behavior and nouns by the data or resource they represent.
- Keep public headers small and stable; expose only what callers need.
- Prefer explicit parameter lists and documented ownership over hidden global state.
- Use consistent naming and file structure so declarations and definitions are easy to find.

## Memory and resource management

- Make ownership explicit for every allocation, buffer, file handle, and OS resource.
- Pair allocation and cleanup paths clearly; use one cleanup path per function when practical.
- Initialize data before use and leave objects in a valid state after partial failure.
- Check sizes and bounds before copying, formatting, parsing, or indexing into buffers.
- Prefer passing buffer length with buffer pointers; do not rely on sentinel conventions alone.

## Structure and modularity

- Keep functions focused on one task; split parsing, validation, I/O, and transformation work.
- Keep modules cohesive; if a file mixes unrelated responsibilities, split it.
- Prefer small helper functions over deeply nested control flow.
- Design APIs so core logic can run without direct filesystem, network, clock, or terminal dependencies.
- Treat a function that is hard to explain briefly as a refactor signal.

## Errors and contracts

- Validate external input at module boundaries and return clear status codes or error objects.
- Use consistent error-handling patterns so callers know where cleanup and reporting happen.
- Preserve useful context in error messages, including the failed operation and key identifiers.
- Assert only for programmer mistakes or impossible states, not routine runtime failures.
- Do not ignore return values from allocation, I/O, parsing, or synchronization functions.

## Pointers, data, and performance

- Prefer `const` for data and pointers that should not be modified.
- Minimize mutable shared state; pass data explicitly instead of reaching through globals.
- Keep structs coherent and document invariants that callers must preserve.
- Optimize only after measurement, but avoid obviously wasteful copies and repeated scans.
- Favor straightforward loops and data flow over clever macros or opaque pointer arithmetic.

## Testing and maintainability

- Test observable behavior, edge cases, and failure cleanup paths, not private implementation details.
- Cover boundary sizes, invalid input, allocation failure handling, and partial I/O behavior.
- Build seams around allocators, files, sockets, time, and environment access when tests need control.
- Remove dead code, stale comments, and duplicated helpers before they spread.
- Refactor repeated patterns into shared helpers only after the common contract is clear.
