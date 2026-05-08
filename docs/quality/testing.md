# Testing Guide

## Test layers

- Keep application-core behavior covered by fast unit tests that run without a terminal, MCP transport, filesystem, clock, or process access.
- Use render and state-transition tests for ratatui behavior instead of relying on manual terminal checks.
- Use request and response fixtures for MCP protocol behavior, especially around `initialize`, `tools/list`, `tools/call`, resources, and malformed envelopes.
- Keep CLI tests focused on parsing, exit behavior, and user-visible output contracts.
- Add integration coverage only where multiple layers must prove they compose correctly.

## Test seams

- Express core behavior behind traits so storage, clocks, process I/O, and transport can be replaced with fakes in tests.
- Keep command parsing separate from command execution so CLI behavior stays testable without full environment setup.
- Keep TUI rendering separate from event handling and state mutation so each concern can be asserted directly.
- Keep MCP handlers thin so protocol fixtures validate adapter behavior while core tests validate feature logic.
- Treat code that cannot be tested without global state or live side effects as a refactor target.

## Determinism

- Isolate time, randomness, filesystem paths, environment variables, and subprocess behavior behind explicit interfaces.
- Avoid tests that depend on terminal size, clock timing, background races, or unordered output.
- Make snapshots and golden outputs intentional, small, and easy to update after reviewed behavior changes.
- Keep fixture names specific to the scenario they prove, not just the command or tool name.
- Fail tests with messages that make the broken contract obvious to the next agent.

## TDD fit

- Follow the repo workflow from reviewed plan to skeleton tests before filling feature bodies.
- Land one planned behavior slice at a time with matching validation evidence.
- Add regression tests when a bug crosses a CLI, TUI, MCP, or boundary contract.
- Remove stale fixtures and dead test helpers when the contract they modeled no longer exists.
- Record skipped validation clearly when a test cannot run locally.
