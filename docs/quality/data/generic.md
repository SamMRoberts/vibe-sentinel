# Generic Data Guidelines

## Modeling and structure

- Model data around stable domain concepts, not around a single query or screen.
- Keep schemas and document shapes cohesive; split unrelated concerns into separate structures.
- Make identifiers, relationships, and ownership rules explicit.
- Treat wide, sparse, or deeply nested structures as refactor signals.
- Prefer additive evolution over disruptive redesign when data already has consumers.

## Contracts and validation

- Validate external data at ingestion boundaries before storing or indexing it.
- Make required fields, optional fields, and defaulting rules explicit.
- Use consistent names, units, and enum values across producers and consumers.
- Reject ambiguous sentinel values when a nullable or typed field is clearer.
- Version or document contract changes before multiple writers or readers drift apart.

## Queryability and maintainability

- Design structures so common queries remain obvious and efficient.
- Avoid encoding multiple unrelated meanings into one field.
- Keep derived data clearly separated from source-of-truth data.
- Document invariants that cannot be enforced directly by the storage engine.
- When query logic becomes hard to explain briefly, treat that as a modeling problem first.

## Reliability and operations

- Make retention, deduplication, and backfill expectations explicit.
- Preserve enough provenance to explain where critical data came from.
- Plan for partial failure, missing records, and delayed writes.
- Avoid silent lossy transforms during import, export, or migration paths.
- Keep migrations small, reversible when possible, and easy to validate.
