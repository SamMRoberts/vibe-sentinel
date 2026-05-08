# Security and Privacy

## Scope

- This file owns the repo-wide security and privacy policy.
- This file also owns implementation-level security guidance for boundary parsing, data handling, dependency use, process control, and review expectations.

## Constraints

- Validate all CLI arguments, config input, MCP payloads, storage records, and external-process data before they enter application behavior.
- Treat credentials, tokens, local memory content, and repository paths as sensitive unless a design doc says otherwise.
- Keep MCP tools explicit about read/write behavior and avoid hidden side effects.

## Non-negotiable rules

- Do not weaken authentication, authorization, validation, privacy, or secret handling without explicit harness approval.
- Do not log secrets or sensitive user data.
- Validate untrusted input at boundaries.
- Treat dependency, deployment, public contract, and credential changes as approval-required unless scope says otherwise.

## Boundary discipline

- Keep security-sensitive checks close to the adapter or service that owns the boundary.
- Prefer explicit allowlists, typed parsing, and narrow contracts over permissive catch-all handlers.
- Treat hidden side effects as security debt, especially in MCP tools and automation-facing paths.
- Fail closed when validation is missing, parsing is ambiguous, or authorization context is unclear.

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

## Review expectations

- Flag security-sensitive diffs early when review touches boundary parsing, secret handling, storage shape, or external process control.
- Add focused tests or fixtures for security-relevant edge cases instead of relying on happy-path coverage.
- Preserve auditability around writes, destructive actions, and privileged flows.
- When a design weakens validation, privacy, or observability, require explicit approval before landing it.
