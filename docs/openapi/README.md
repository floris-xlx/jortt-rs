# OpenAPI Inventory

This folder pins the source-of-truth API snapshot used by this SDK release.

- `jortt-swagger-2026-05-05.json`: raw OpenAPI 2.0 document fetched on 2026-05-05
- `operations-2026-05-05.json`: normalized operation inventory derived from that snapshot

Current inventory count: **126 operations**.

The inventory drives:

- `src/api/operations.rs` typed enums and operation specs
- `docs/endpoint-coverage.md` coverage matrix
