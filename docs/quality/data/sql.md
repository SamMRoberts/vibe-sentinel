# SQL Guidelines

## Schema design

- Model tables around clear entities and relationships rather than around one report shape.
- Use explicit primary keys, foreign keys, and uniqueness constraints where the database can enforce them.
- Keep columns typed narrowly enough to protect invariants and avoid overloaded meanings.
- Normalize until duplication becomes a measured performance problem, then denormalize intentionally.
- Prefer additive schema evolution over destructive rewrites.

## Query design

- Write queries for clarity first; make joins, filters, and aggregation intent obvious.
- Select only the columns you need instead of relying on `SELECT *`.
- Keep transactional boundaries explicit and as small as correctness allows.
- Treat repeated hard-to-read query logic as a signal to extract views or shared query helpers.
- Make ordering explicit whenever consumers depend on result order.

## Performance and indexing

- Add indexes to support real filter, join, and ordering patterns rather than guessing.
- Revisit indexes when query patterns change; unused indexes still cost writes and storage.
- Avoid row-by-row loops in application code when set-based operations are clearer and safer.
- Check query plans before assuming a slow query needs caching or denormalization.
- Keep large migrations and backfills chunked, observable, and restartable.

## Integrity and operations

- Validate data at the application boundary and enforce critical invariants in the database too.
- Use clear nullability rules and avoid encoding multiple states through `NULL` alone.
- Keep destructive operations gated, reviewable, and easy to rehearse safely.
- Preserve enough audit context to explain how critical rows changed.
- Test schema changes against realistic data volume and edge cases before rollout.
