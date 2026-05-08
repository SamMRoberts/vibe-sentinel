# Storage Guidelines

## Boundary ownership

- Keep storage behind service traits so the application core depends on behavior, not storage details.
- Parse and validate storage records at the adapter boundary before they influence domain behavior.
- Do not let file layout, SQL shape, or cache structure leak into CLI or TUI surfaces.
- Keep persistence rules consistent with the layering in `docs/architecture.md`.

## Data design

- Model stored data around clear domain concepts rather than around one caller's convenience.
- Keep schemas and on-disk structures explicit, versionable, and easy to inspect locally.
- Use narrow types and explicit nullability rules to protect invariants.
- Prefer additive evolution over destructive rewrites when storage shape changes.

## Writes and updates

- Make writes idempotent when retries or repeated commands are plausible.
- Keep multi-step updates explicit so partial failure behavior is understandable and testable.
- Avoid hidden writes from read-oriented code paths, background refreshes, or status surfaces.
- Record enough context to explain what changed without exposing sensitive content.

## Reliability

- Keep storage operations observable, bounded, and cancellable where long-running work is possible.
- Preserve timeouts, retries, and backoff behavior intentionally, not by accident, per `docs/reliability.md`.
- Design repair and recovery paths that a future agent can run locally without guesswork.
- Chunk large backfills or rewrites so progress and failure points remain visible.

## Security and review

- Treat repository paths, local memory content, credentials, and user-derived records as sensitive per `docs/security.md`.
- Do not store secrets in plaintext unless an approved design doc explicitly allows it.
- Pair storage-shape changes with fixture or integration validation and clear review notes.
- Escalate migrations, public storage contract changes, or destructive cleanup for human approval before implementation.
