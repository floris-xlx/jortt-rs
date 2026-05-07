# End-to-End Testing Suite

This repository now includes two E2E layers:

1. deterministic E2E tests against a mock Jortt server (`tests/e2e_mock.rs`)
2. opt-in live E2E tests against real Jortt credentials (`tests/e2e_live.rs`)

## 1) Deterministic E2E (CI-safe)

Runs a full SDK flow in one test:

- customer list/create/get/update/delete
- invoice list/create/get/download
- ledger accounts read
- generated method group call (`client.methods()...`)
- raw client call (`client.raw()...`)

Command:

```bash
cargo test --test e2e_mock
```

This suite is fully local and does not require real credentials.

## 2) Live E2E (explicit opt-in)

The live suite is marked `#[ignore]` so it never runs accidentally in CI.

Command:

```bash
cargo test --test e2e_live -- --ignored --nocapture
```

Required opt-in flag:

- `JORTT_E2E_LIVE=true`

Credential inputs:

- `JORTT_ACCESS_TOKEN` (optional shortcut)
- or `JORTT_CLIENT_ID` + `JORTT_CLIENT_SECRET`
- optional `JORTT_BASE_URL` (defaults to `https://api.jortt.nl/`)

Optional lookup inputs:

- `JORTT_PARAM_CUSTOMER_ID` (enables targeted `get_customer`)
- `JORTT_PARAM_INVOICE_ID` (enables targeted `get_invoice` + PDF URL)

## Full OpenAPI live smoke (optional)

`tests/e2e_live.rs` also includes a full catalog smoke that iterates `ALL_OPERATION_SPECS`.

Enable it explicitly with:

- `JORTT_E2E_LIVE=true`
- `JORTT_E2E_FULL_SMOKE=true`
- optionally `JORTT_RUN_MUTATIONS=true` for POST/PUT/PATCH/DELETE calls

It reuses the same fixture variable naming as [`docs/live-testing.md`](./live-testing.md):

- `JORTT_PARAM_<NAME>`
- `JORTT_QUERY_<METHOD>_<PATH_KEY>`
- `JORTT_BODY_<METHOD>_<PATH_KEY>`
- `JORTT_ACCEPT_<METHOD>_<PATH_KEY>`

## Recommended local verification loop

```bash
cargo fmt
cargo test --test e2e_mock
cargo test --all-targets --all-features
```
