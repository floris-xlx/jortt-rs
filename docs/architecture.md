# Architecture

## Layering

The SDK follows a strict inward dependency rule:

1. **Facade Layer** (`JorttClient`, `DomainApi`, convenience methods)
2. **Operation Layer** (`src/api/operations.rs`, typed operation enums)
3. **Transport Layer** (HTTP execution, auth injection, retries, error mapping)
4. **Model Layer** (request/response structs and envelopes)

The operation layer is generated from the pinned OpenAPI inventory file:

- `docs/openapi/jortt-swagger-2026-05-05.json`
- `docs/openapi/operations-2026-05-05.json`

## Core Interfaces

- `AccessTokenSource`: async token provider seam
- `OAuthClient`: explicit token exchange/refresh workflows
- `JorttClient`: high-level entrypoint and typed domain handles
- `ApiMethods`: generated per-tag/per-operation method surface (full 126 endpoint coverage)
- `DomainApi<O: TypedOperation>`: generic operation executor
- `RequestBuilder`: one canonical request construction path
- `ErrorBuilder`: one canonical error construction path
- `RawClient`: low-level escape hatch

## Request Pipeline

1. caller chooses operation enum variant
2. SDK resolves `OperationSpec` (`method`, `path`, `operation_id`)
3. path/query/body are rendered into HTTP request
4. bearer token is pulled from `AccessTokenSource` (if set)
5. retry policy applies on transport errors and `429/5xx`
6. JSON success payload is returned, or structured API error is parsed

## Coverage Strategy

- All `126` operations in the pinned OpenAPI snapshot are represented as typed enum variants.
- Generated method groups map 1:1 to OpenAPI tags and contain one method per operation.
- High-frequency workflows (customers/invoices/ledger-accounts) also have ergonomic typed methods on `JorttClient`.
- Drift protection is enforced via tests that compare `docs/openapi/operations-2026-05-05.json` to the generated Rust operation catalog exactly.
