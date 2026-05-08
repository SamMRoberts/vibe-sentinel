# NoSQL Guidelines

## Data modeling

- Choose document, key-value, column-family, or graph shapes based on access patterns, not habit.
- Model records around stable read and write paths while keeping ownership boundaries clear.
- Keep documents and items cohesive; split structures that mix unrelated responsibilities.
- Duplicate data only when it makes a specific read path simpler or faster and the update strategy is explicit.
- Treat unbounded nesting or unbounded collection growth as a schema smell.

## Keys and access patterns

- Design partition, shard, and sort keys around the queries the system must serve reliably.
- Make hot-key and hot-partition risks part of the initial design, not an afterthought.
- Keep secondary indexes intentional and limited to proven access paths.
- Prefer explicit query patterns over broad scans in latency-sensitive paths.
- Document which fields are authoritative when multiple denormalized copies exist.

## Consistency and correctness

- Be explicit about consistency expectations, conflict handling, and duplicate-write behavior.
- Validate document shape and required fields before writes reach the store.
- Keep idempotency and retry behavior clear for write operations.
- Design for partial records, delayed propagation, and backfill repair paths.
- Preserve enough metadata to reconcile divergent copies or replay writes safely.

## Operations and evolution

- Evolve schemas additively and keep old readers in mind during transitions.
- Keep migrations and reindexing jobs chunked, observable, and restartable.
- Avoid silent coercion between numbers, strings, timestamps, and booleans.
- Measure storage and query costs for large nested objects and wide fan-out patterns.
- When queries become hard to explain, revisit the model before adding more application-side fixes.
