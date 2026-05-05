# Live Examples

This folder contains runnable examples for real Jortt integrations.

Detailed guide: [docs/live-testing.md](../docs/live-testing.md)

## Credential Policy

Real client IDs/secrets are never stored in this repository.
Set your own credentials via environment variables.

## Required Environment

- `JORTT_CLIENT_ID`
- `JORTT_CLIENT_SECRET`

Optional:

- `JORTT_ACCESS_TOKEN` (when present, skips OAuth exchange)
- `JORTT_BASE_URL` (defaults to `https://api.jortt.nl/`)

Templates:

- PowerShell: [`examples/env.template.ps1`](env.template.ps1)
- POSIX shell: [`examples/env.template.sh`](env.template.sh)

## Examples

## `oauth_client_credentials`

Fetches a token set from OAuth client-credentials flow.

```bash
cargo run --example oauth_client_credentials
```

## `typed_live_workflow`

Exercises typed high-level methods (customers/invoices).

```bash
cargo run --example typed_live_workflow
```

Optional vars:

- `JORTT_PARAM_CUSTOMER_ID` to execute `get_customer`
- `JORTT_PARAM_INVOICE_ID` to execute `get_invoice` and PDF URL lookup
- `JORTT_ALLOW_WRITES=true` to run create operations

## `full_openapi_smoke`

Runs against the generated operation catalog and attempts all operations that can be satisfied by your fixtures.

```bash
cargo run --example full_openapi_smoke
```

Read-only by default. Enable mutations with:

```bash
set JORTT_RUN_MUTATIONS=true
cargo run --example full_openapi_smoke
```

Per-operation fixtures:

- `JORTT_PARAM_<NAME>` for path placeholders
- `JORTT_QUERY_<METHOD>_<PATH_KEY>` for query strings (`k=v&k2=v2`)
- `JORTT_BODY_<METHOD>_<PATH_KEY>` for JSON bodies
- `JORTT_ACCEPT_<METHOD>_<PATH_KEY>` for `Accept` overrides

Refer to [docs/live-testing.md](../docs/live-testing.md#path-key-derivation) for `<PATH_KEY>` rules and examples.
