# Security and Privacy

## Constraints

- Validate all CLI arguments, config input, MCP payloads, storage records, and external-process data before they enter application behavior.
- Treat credentials, tokens, local memory content, and repository paths as sensitive unless a design doc says otherwise.
- Keep MCP tools explicit about read/write behavior and avoid hidden side effects.

## Non-negotiable rules

- Do not weaken authentication, authorization, validation, privacy, or secret handling without explicit harness approval.
- Do not log secrets or sensitive user data.
- Validate untrusted input at boundaries.
- Treat dependency, deployment, public contract, and credential changes as approval-required unless scope says otherwise.
