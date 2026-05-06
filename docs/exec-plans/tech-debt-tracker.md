
# Technical Debt Tracker

Track recurring drift, stale docs, missing checks, cleanup tasks, and future mechanical enforcement.

| Date | Area | Issue | Proposed cleanup | Status |
| --- | --- | --- | --- | --- |
| 2026-05-06 | TDD workflow validator | Plan validation currently checks required headings but does not deeply verify checklist quality, approval evidence, or test-before-implementation sequencing. | Strengthen `scripts/validate_tdd_workflow.py` with semantic checks once the harness has more completed-plan examples to validate against. | open |
| 2026-05-06 | TUI validation | Interactive TUI lifecycle smoke testing is tool-limited; current coverage is deterministic render/state tests plus private setup cleanup tests. | Add a deterministic terminal lifecycle smoke harness or documented pseudo-terminal test path for `status --tui`. | open |
