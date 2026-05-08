# Security Engineering Guidelines

## Boundary discipline

- Validate untrusted input at CLI, TUI, MCP, storage, and process boundaries before it reaches application behavior.
- Keep security-sensitive checks close to the adapter or service that owns the boundary.
- Prefer explicit allowlists, typed parsing, and narrow contracts over permissive catch-all handlers.
- Treat hidden side effects as security debt, especially in MCP tools and automation-facing paths.

## Data handling

- Minimize the sensitive data each layer sees; do not pass secrets through layers that do not need them.
- Keep logs, errors, fixtures, and test output free of tokens, credentials, and private user content.
- Use domain types that make redaction and validation obvious instead of relying on stringly typed data.
- Return only the data a caller needs; avoid broad debug dumps in normal flows.

## Dependency and process safety

- Wrap external-process, filesystem, and network interactions behind explicit interfaces.
- Sanitize paths, arguments, and serialized payloads before handing them to external tools.
- Do not trust dependency defaults for auth, TLS, retries, or serialization safety without verifying them.
- Prefer explicit read-only and write-capable paths when defining MCP or automation behavior.

## Failure behavior

- Fail closed when validation is missing, parsing is ambiguous, or authorization context is unclear.
- Make security-relevant errors actionable for maintainers without leaking sensitive details to callers.
- Preserve auditability around writes, destructive actions, and privileged flows.
- Keep timeout, cancellation, and retry behavior aligned with `docs/reliability.md` so failures do not degrade into unsafe hangs.

## Review expectations

- Anchor implementation decisions to the broader policy in `docs/security.md`; this file complements it and does not replace it.
- Flag security-sensitive diffs early when review touches boundary parsing, secret handling, storage shape, or external process control.
- Add focused tests or fixtures for security-relevant edge cases instead of relying on happy-path coverage.
- When a design weakens validation, privacy, or observability, require explicit approval before landing it.
