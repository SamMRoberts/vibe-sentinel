# R Guidelines

## Naming and API design

- Use clear, domain-specific names; avoid abbreviations unless they are standard in the field.
- Name functions by intent and keep argument names explicit at the call site.
- Keep public function interfaces small and predictable; prefer a few clear parameters over option bags.
- Use consistent naming across the file or package, such as `snake_case` for functions and variables.
- Prefer nouns for data objects and verbs for functions that transform or analyze data.

## Data and validation

- Validate external inputs early, especially column names, factor levels, dimensions, and missing-value assumptions.
- Make expected types and shapes explicit in code and documentation.
- Prefer explicit handling of `NA`, `NULL`, and empty inputs over relying on recycling or coercion side effects.
- Return stable object shapes so callers do not need branch-heavy post-processing.
- Avoid partial matching and other permissive defaults that hide mistakes.

## Structure and modularity

- Keep scripts and modules focused; split files that mix ingestion, modeling, plotting, and reporting concerns.
- Keep functions focused on one task; extract helpers when pipelines or branching obscure intent.
- Keep functions small enough to read without scrolling through unrelated setup.
- Prefer small composable transformations over large monolithic analysis scripts.
- Isolate side effects such as file I/O, plotting output, and external service calls from core data logic.
- Use small structured objects or lists when they clarify intermediate state and results.

## Data manipulation and performance

- Prefer readable transformations over dense chains that hide assumptions or reorder data silently.
- Be explicit about grouping, ordering, joins, and missing-data behavior.
- Avoid repeated copies of large data frames in hot paths when a simpler flow can reuse results.
- Use vectorized operations when they improve clarity and correctness, not just concision.
- Measure before optimizing and keep performance-sensitive code easy to profile.

## Errors and diagnostics

- Fail early with clear messages when required inputs, packages, or columns are missing.
- Use warnings sparingly; reserve them for recoverable situations that callers should notice.
- Include enough context in errors and logs for someone to reproduce the failed analysis step.
- Do not silently drop rows, columns, or factor levels without making that choice explicit.
- Keep package startup messages and incidental output from polluting scripted runs.

## Testing and maintainability

- Test observable behavior, edge cases, and statistical or tabular invariants.
- Keep tests deterministic by fixing seeds and isolating time, locale, file paths, and external data sources.
- Design analysis functions so they can run on small fixture data without requiring full reports or real datasets.
- Prefer dependency injection for paths, readers, and writers that need substitution in tests.
- When a function or script becomes hard to explain briefly, treat that as a refactor signal.
- Remove dead code, commented-out experiments, and unused helper layers quickly.
- Refactor duplication only after the shared pattern is clear and stable.
