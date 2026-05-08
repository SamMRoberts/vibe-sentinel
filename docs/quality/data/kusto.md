# Kusto Guidelines

## Table and event design

- Model tables and events around stable analytical questions, not around one dashboard widget.
- Keep column names, types, and semantic units consistent across related tables.
- Separate raw ingestion fields from curated derived fields when both are needed.
- Prefer explicit timestamps, identifiers, and correlation fields over parsing meaning from free text.
- Treat very wide records and semi-structured blobs as a sign to refine the ingestion contract.

## Query design

- Write KQL for readability first; keep each transformation stage easy to explain.
- Filter early, project only needed columns, and summarize with explicit grouping intent.
- Prefer named `let` bindings for reusable subqueries instead of repeating large expressions.
- Keep joins intentional and verify the key cardinality before assuming correctness.
- Make time windows explicit so results remain stable and reviewable.

## Performance and cost

- Reduce scanned data before adding complex parsing, joins, or aggregation.
- Materialize or precompute only when repeated query cost justifies the added maintenance burden.
- Revisit retention, update policies, and materialized views when usage changes.
- Avoid parsing the same dynamic payload repeatedly across many queries when ingestion can normalize it once.
- Treat slow or costly queries as a signal to revisit schema, retention, or ingestion strategy.

## Reliability and operations

- Validate event shape and critical fields before ingestion where the pipeline allows it.
- Keep source provenance, ingestion time, and correlation context available for investigations.
- Design dashboards and alerts around late, missing, and duplicated events.
- Document derived metrics so operators can trace them back to source events and filters.
- Test query changes against representative time ranges and noisy edge cases before relying on them operationally.
