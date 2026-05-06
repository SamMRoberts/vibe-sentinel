# Agent Operating Model

## Human and agent roles

Humans steer goals, acceptance criteria, scope changes, and final judgment.
Agents execute inside the harness, keep work inspectable, and preserve the
modified TDD workflow.

## Scope gate

Before any code edit, read `AGENTS.md`, `docs/harness/scope.md`, and the
task-specific docs routed by the table of contents. Classify the request with
the scope gate. Multi-file, architectural, user-visible, security, reliability,
or unclear work requires an active execution plan.

## Modified TDD workflow

All feature development must follow this sequence in order:

1. Gather feature info, acceptance criteria, constraints, and non-goals.
2. Research and explore the current code, docs, crates, and examples.
3. Create an execution plan.
4. Review and refine the plan before architecture work starts.
5. Architect with pseudocode covering every planned module, struct, enum, trait, function, and method.
6. Review and refine the architecture before code scaffolding starts.
7. Scaffold skeleton structs, traits, functions, and methods with the bare minimum needed for tests and mocks.
8. Create tests around skeleton behavior using mocks, fakes, fixtures, or test doubles where needed.
9. Run tests, fix skeleton seams, and repeat until skeleton-level tests pass.
10. Fill in exactly one skeleton unit at a time.
11. Run the relevant tests after each filled unit.
12. Review, refine, and repeat until the feature is complete and validated.

## Required TDD artifacts

Feature execution plans must include these sections:

- Feature Info
- Research Notes
- Reviewed Plan
- Architecture Pseudocode
- Reviewed Architecture
- Skeleton Checklist
- Mock Test Checklist
- Implementation Checklist
- Validation Log
- Review Notes

The architecture pseudocode must name every planned module, struct, enum, trait,
function, and method before skeletons are added. The skeleton checklist must
track each planned unit independently. The implementation checklist must be
completed one unit at a time with validation evidence after each unit.

## Enforcement rules

- Do not add feature implementation bodies before skeleton-level tests exist and pass.
- Do not combine multiple skeleton-unit implementations without running the relevant tests between them.
- Do not skip plan or architecture review by folding those steps into the final response.
- Run `python3 scripts/validate_tdd_workflow.py` for feature plans before implementation.
- If the workflow cannot be followed, stop and update the harness or plan before editing code.

## Missing capability rule

When an agent cannot complete a task reliably, do not retry blindly. Identify
whether the missing capability is documentation, tooling, architecture, tests,
observability, or scope. Propose a harness update or execution plan.
