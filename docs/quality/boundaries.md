# Boundary Guide

## Layer ownership

- Keep domain types focused on validated data, invariants, and error shapes without terminal, filesystem, or protocol details.
- Keep service traits as the only route from the application core to storage, clocks, processes, and other external systems.
- Keep adapters responsible for concrete I/O, serialization, and integration details, not feature policy.
- Keep CLI and ratatui TUI surfaces as thin entry points that call the application core.
- Do not bypass the documented domain -> traits -> core -> adapters -> surfaces direction.

## Parsing and validation

- Validate user, file, process, and protocol input at the first boundary that understands the raw shape.
- Translate raw CLI flags and TUI events into explicit domain inputs before invoking core behavior.
- Reject malformed or incomplete input early with actionable errors instead of letting loose maps or strings leak inward.
- Normalize optionality, defaults, and naming once at the edge instead of repeating conversions in the core.
- Treat repeated boundary translation logic as a signal for a shared adapter helper.

## Coupling rules

- Do not let UI state structs become domain models or storage DTOs.
- Keep filesystem paths, environment access, and subprocess handles out of pure business logic.
- Keep adapter-specific logging or retry policy out of domain code unless exposed through an explicit interface.
- Prefer a small number of clear boundary types over hidden conversions spread across modules.

## Review signals

- If a feature needs terminal access to compute core decisions, the boundary is likely wrong.
- If a trait exists only to hide one local helper and has no test value, it is likely too abstract.
- If the same validation rule appears in CLI and TUI code, move it into the core or a shared boundary utility.
- If tests require a live terminal or protocol session to prove domain behavior, split the layers further.
- When code crosses layers casually, update the design before the pattern spreads.
