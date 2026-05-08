# Dependency Guidelines

## Selection

- Add a crate or package only when it removes real maintenance cost or closes a capability gap.
- Prefer the standard library and existing repo patterns before introducing a new dependency.
- Choose dependencies with stable maintenance, clear ownership, and a narrow feature surface.
- Treat runtime, storage, protocol, and package-manager changes as approval-sensitive per `docs/harness/scope.md`.

## Integration boundaries

- Keep third-party APIs behind adapters or service traits from `docs/architecture.md`.
- Do not let vendor types leak through the application core when a local domain type is clearer.
- Keep CLI, TUI, and MCP surfaces thin even when a dependency offers its own workflow abstractions.
- Centralize repeated integration behavior instead of scattering one-off wrappers across features.

## Features and configuration

- Enable only the features the repo uses; avoid broad default-feature pulls when a narrower set exists.
- Keep dependency configuration explicit so future agents can trace why a feature is enabled.
- Prefer additive configuration changes over silent behavior changes hidden in dependency defaults.
- Recheck dependency choices when they force awkward layering or reduce testability.

## Safety and reliability

- Validate all data entering from dependency APIs at the adapter boundary as required by `docs/security.md`.
- Do not rely on undocumented retry, timeout, caching, or ordering behavior in third-party code.
- Keep long-running or background dependency behavior observable and stoppable per `docs/reliability.md`.
- Treat transitive file, process, and network access as part of the dependency review, not as an implementation detail.

## Testing and review

- Add or update adapter-focused tests when a dependency changes behavior at a boundary.
- Prefer fixtures and fakes for protocol behavior instead of brittle tests against live third-party systems.
- Note why the dependency was chosen and what surface owns it so review stays localized.
- Remove unused dependencies quickly; dead packages increase review and security cost.
