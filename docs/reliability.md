# Reliability

## Reliability expectations

- CLI commands should fail with actionable errors and deterministic exit behavior.
- ratatui workflows should avoid hidden background work that can run forever unnoticed.
- MCP handlers should have explicit error mapping, timeouts where relevant, and fixture-backed behavior.
- Long-running or stateful behavior needs observability that a future agent can inspect locally.

## Operational rules

- Preserve timeouts, retries, idempotency, rate limits, and error handling unless an approved plan says otherwise.
- Do not remove observability needed to diagnose runtime behavior.
- Prefer deterministic validation over guesswork.
- Record skipped or unavailable validation in the execution plan and final response.
