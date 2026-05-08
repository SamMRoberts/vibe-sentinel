# TUI Quality Guide

## Purpose

- Keep ratatui workflows observable, testable, and safe for long-running interactive use.
- Treat the TUI as a surface over the application core, not as a second home for business logic.

## Render and state separation

- Keep rendering pure from the perspective of application behavior; rendering should read state, not mutate it.
- Keep state transitions explicit and testable without a real terminal, consistent with `docs/architecture.md`.
- Put domain decisions in the application core and keep screen code focused on presentation and interaction flow.
- Avoid hidden coupling between widget layout and side-effectful operations.

## Event handling

- Route keyboard, timer, and background events through clear handlers with narrow responsibilities.
- Keep event handling deterministic; identical input sequences should produce identical state transitions.
- Debounce or coalesce noisy events only when the behavior stays visible and testable.
- Do not bury cross-screen navigation rules inside rendering code.

## Background work visibility

- Surface long-running work, pending tasks, and failures in a way an operator can see without guessing.
- Do not let background tasks run forever unnoticed; make ownership, shutdown, and retry behavior explicit.
- Keep cancellation and completion paths visible in state so the TUI reflects real runtime status.
- Preserve the local observability required by `docs/reliability.md`.

## Keyboard flows

- Keep key bindings consistent across screens for navigation, confirmation, cancel, and exit behavior.
- Prefer explicit focus movement over hidden mode switches.
- Make destructive or irreversible actions require clear confirmation.
- Keep empty states and first-run flows keyboard-accessible without relying on mouse behavior.

## Loading, empty, and error states

- Design loading, empty, success, and error states intentionally for each screen.
- Show actionable messages when data cannot load, validation fails, or background work stops unexpectedly.
- Avoid blank panels or frozen-looking states that hide whether work is pending or failed.
- Keep transient status messages short and tie them to real state changes.

## Testability

- Test reducers, state transitions, and event handling without a live terminal whenever possible.
- Keep rendering assertions focused on stable behavior, not fragile pixel-perfect details.
- Isolate clocks, async work, filesystem access, and process spawning behind traits or adapters.
- When a screen or state object becomes hard to describe briefly, split it into smaller components.
