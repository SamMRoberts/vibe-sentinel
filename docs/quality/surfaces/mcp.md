# MCP Quality Guide

## Purpose

- Keep MCP tools and resources thin at the protocol boundary and consistent with the product areas in `docs/app-specs/app-spec.md`.
- Treat MCP handlers as adapters that validate inputs, call the application core, and map results back to protocol shapes.

## Handler boundaries

- Keep request parsing, validation, and response mapping in the MCP layer.
- Do not move business rules, storage policy, or domain orchestration into handlers.
- Call application-core operations through explicit traits and keep transport details out of domain code.
- Keep resource readers and tool handlers small enough to understand without tracing unrelated code paths.

## Tool and resource naming

- Name tools and resources by stable domain intent, not temporary implementation details.
- Keep names consistent with CLI and TUI terminology when they expose the same concept.
- Prefer explicit argument names and result fields over positional or overloaded meanings.
- Avoid near-duplicate tools that differ only by hidden defaults.

## Argument validation

- Validate required fields, enum-like values, identifiers, and shape constraints at the MCP boundary.
- Return clear protocol errors for malformed input before calling deeper layers.
- Normalize boundary data once, then pass typed values into the application core.
- Do not assume clients will send ideal shapes or omit optional fields consistently.

## Error mapping

- Map domain, validation, timeout, and internal failures into predictable MCP error responses.
- Keep error messages actionable without leaking irrelevant internal details.
- Preserve enough context for local debugging, logs, and fixture assertions.
- Do not swallow partial failures or convert every error into the same generic response.

## Timeout and cancellation behavior

- Make timeout behavior explicit for operations that call external processes, storage, or network boundaries.
- Honor cancellation signals where the transport and downstream code support them.
- Keep retries bounded and visible rather than silently looping in the handler.
- Preserve idempotent behavior for safe read-style operations where practical.

## Fixture-backed behavior

- Cover tool and resource behavior with request/response fixtures, as required by `docs/architecture.md`.
- Keep fixtures focused on stable wire behavior: success, validation failure, domain failure, and timeout-style cases.
- Update fixtures when protocol-visible behavior changes, not for irrelevant refactors.
- When a handler grows multiple branches that are hard to explain briefly, split the adapter logic.
