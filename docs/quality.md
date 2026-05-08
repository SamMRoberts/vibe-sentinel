# Quality Bar

## Quality invariants

- Follow the TDD workflow for all feature development.
- Prefer legible, tested, documented patterns.
- Validate data at CLI, TUI, MCP, storage, and external-process boundaries.
- Avoid hidden coupling between application core and UI/protocol adapters.
- Keep future agent runs easy to reason about.

## General expectations

- Keep behavior legible to future agent runs.
- Prefer explicit names and small, testable units.
- Avoid speculative abstractions.
- Do not spread inconsistent patterns.
- Promote repeated review feedback into docs or mechanical checks.
- Skeleton code may be minimal, but it must be intentionally mockable.
- Implementation should land one planned unit at a time with validation evidence.

## Language-specific guides

| Language | Guide |
| --- | --- |
| C | `docs/quality/languages/c.md` |
| C++ | `docs/quality/languages/cpp.md` |
| C# | `docs/quality/languages/cs.md` |
| Generic | `docs/quality/languages/generic.md` |
| Go | `docs/quality/languages/go.md` |
| HTML | `docs/quality/languages/html.md` |
| Java | `docs/quality/languages/java.md` |
| JavaScript | `docs/quality/languages/javascript.md` |
| PHP | `docs/quality/languages/php.md` |
| Python | `docs/quality/languages/python.md` |
| R | `docs/quality/languages/r.md` |
| Rust | `docs/quality/languages/rust.md` |
| Shell | `docs/quality/languages/shell.md` |
| Swift | `docs/quality/languages/swift.md` |
| TypeScript | `docs/quality/languages/typescript.md` |

## Data guides

| Data Type | Guide |
| --- | --- |
| Generic | `docs/quality/data/generic.md` |
| Kusto | `docs/quality/data/kusto.md` |
| NoSQL | `docs/quality/data/nosql.md` |
| SQL | `docs/quality/data/sql.md` |
