
# Execution Plans Index

Execution plans are first-class repo artifacts.

## Folders

- `active/`: plans currently being implemented.
- `completed/`: plans moved here after validation.
- `plan-template.md`: required plan shape with modified TDD artifacts.
- `tech-debt-tracker.md`: known drift, stale docs, missing checks, and cleanup work.

Create a plan before multi-file, architectural, security, reliability, or user-visible behavior changes.
Run `python3 scripts/validate_tdd_workflow.py <plan>` before feature implementation begins.
