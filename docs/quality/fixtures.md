# Fixture Guidelines

## Purpose

- Use fixtures to make CLI, TUI, storage, and adapter behavior repeatable for future agent runs.
- Keep fixtures focused on one behavior or failure mode so test intent stays obvious.
- Prefer realistic boundary shapes over oversized sample projects with unrelated noise.

## Layout and naming

- Name fixtures by behavior and scenario, not by who created them or when.
- Keep fixture paths stable once referenced by tests unless the owning tests move too.
- Group fixtures by surface or adapter so ownership is easy to trace during review.
- Keep fixture contents small enough to inspect quickly in diffs.

## Content rules

- Include only the fields, rows, events, or files needed to prove the behavior under test.
- Preserve invalid, partial, and failure fixtures when they document important boundary rules.
- Redact or synthesize sensitive values; never store secrets, tokens, or private user data.
- Prefer deterministic timestamps, ordering, and identifiers unless the test explicitly exercises variance.

## Maintenance

- Update fixtures with the behavior change in the same review rather than leaving drift for later.
- Delete orphaned fixtures once no test or validation step depends on them.
- When a fixture starts covering multiple unrelated concerns, split it before adding more cases.
- Treat golden-file churn as a review signal; explain why output changed and whether the contract changed.

## Review and validation

- Pair fixture changes with the narrowest relevant tests or validation commands from `docs/tooling.md`.
- Prefer explicit assertions over snapshot sprawl when a few fields prove the behavior.
- Report skipped fixture validation clearly in the final response, consistent with `docs/review.md`.
