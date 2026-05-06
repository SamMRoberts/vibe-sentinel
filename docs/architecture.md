# Architecture

## Required boundaries

- Domain types: pure data, value semantics, parsing targets, and error types.
- Service traits: mockable ports for storage, clocks, process I/O, MCP transport, and other boundaries.
- Application core: feature behavior expressed against traits, not concrete UI or protocol adapters.
- Adapters: concrete storage, filesystem, process, network, or MCP integration code.
- Surfaces: CLI commands, ratatui TUI screens, and MCP tools/resources call into the application core.

## Dependency rules

- Preserve documented layer direction.
- Do not introduce hidden cross-layer coupling.
- Cross-cutting concerns must enter through explicit interfaces.
- Validate data shapes at system boundaries.
- Prefer shared utilities over repeated local helpers when behavior should be consistent.
- Keep command parsing separate from command execution.
- Keep ratatui rendering and state transitions testable without a real terminal.
- Keep MCP protocol handling thin and covered by request/response fixtures.

## Architecture change rule

Architecture changes require an active execution plan and, when listed in
`docs/harness/scope.md`, explicit human approval.
