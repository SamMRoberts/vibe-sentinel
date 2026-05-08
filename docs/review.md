# Review Expectations

## Scope

- This file owns the repo review loop and handoff expectations.
- Use `docs/quality/testing.md`, `docs/quality/fixtures.md`, and `docs/quality/docs.md` for implementation-level guidance that supports the review loop.

## Local review loop

1. Review the diff before final response.
2. Check that changes stay inside harness scope.
3. Check feature work followed the TDD artifacts in the active plan.
4. Run relevant validation from `docs/tooling.md`.
5. Update docs when durable knowledge changes.
6. Report unresolved risks clearly.

## Feedback loop

When review feedback exposes a reusable rule, update the harness docs or propose a mechanical enforcement check.

## Harness status handoff

When reviewing or refining the coding harness, report:

- Validation directory used.
- Counts for `complete`, `needs_update`, and `failed` section-state files.
- Next action from the harness section-status script.
- Global issues affecting multiple sections.
- Validation commands run, failed, or skipped.
- Whether the harness is ready for refinement, complete, or blocked.
