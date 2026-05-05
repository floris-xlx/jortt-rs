# jortt-rs

`jortt` is an async-first Rust SDK for the Jortt API.

## Highlights

- Typed operation enums for every operation in the pinned Jortt OpenAPI snapshot (`126` operations)
- Generated method surface for every operation (`client.methods().<tag>().<operation>()`)
- High-level `JorttClient` with domain handles (`customers`, `invoices`, `projects`, `reports`, ...)
- One canonical request builder: `RequestBuilder`
- One canonical error builder: `ErrorBuilder`
- Hybrid auth design:
  - user-supplied `AccessTokenSource` for request-time bearer tokens
  - explicit OAuth helper flows for auth-code, refresh-token, and client-credentials
- `RawClient` escape hatch for ad-hoc endpoints while keeping typed API stable
- Structured Jortt API error parsing (`error.code`, `error.key`, `error.message`, `error.details`)

## Install

```toml
[dependencies]
jortt = "0.1.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start (Bearer Token Source)

```rust,no_run
use std::sync::Arc;

use jortt::{JorttClient, ListCustomersQuery, StaticAccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token_source = Arc::new(StaticAccessToken::new("YOUR_ACCESS_TOKEN"));

    let client = JorttClient::builder()
        .with_token_source(token_source)
        .build()?;

    let customers = client
        .list_customers(&ListCustomersQuery {
            query: Some("janssen".to_string()),
            page: None,
        })
        .await?;

    println!("customers found: {}", customers.data.len());
    Ok(())
}
```

## Full Coverage Method Surface

```rust,no_run
use std::sync::Arc;

use jortt::{JorttClient, RequestBuilder, StaticAccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JorttClient::builder()
        .with_token_source(Arc::new(StaticAccessToken::new("YOUR_ACCESS_TOKEN")))
        .build()?;

    let customers = client
        .methods()
        .customers()
        .get_customers(RequestBuilder::new().query_param("query", "acme").build())
        .await?;

    println!("{customers}");
    Ok(())
}
```

## Quick Start (Hybrid OAuth Helpers)

```rust,no_run
use jortt::{OAuthClient, OAuthConfig, Scope};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = OAuthClient::new(OAuthConfig::default())?;

    let tokens = oauth
        .exchange_client_credentials(
            "CLIENT_ID",
            "CLIENT_SECRET",
            &[Scope::CustomersRead, Scope::InvoicesRead],
        )
        .await?;

    println!("access token length: {}", tokens.access_token.len());
    Ok(())
}
```

## Raw Escape Hatch

```rust,no_run
use std::sync::Arc;

use jortt::{HttpMethod, JorttClient, OperationRequest, StaticAccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JorttClient::builder()
        .with_token_source(Arc::new(StaticAccessToken::new("YOUR_ACCESS_TOKEN")))
        .build()?;

    let response = client
        .raw()
        .execute(
            HttpMethod::Get,
            "/customers/{customer_id}",
            OperationRequest::new().with_path_param("customer_id", "408d4652-b07a-4195-817e-0390bb0c9428"),
        )
        .await?;

    println!("response: {response}");
    Ok(())
}
```

## Documentation Index

- [Architecture](docs/architecture.md)
- [UML](docs/uml/README.md)
- [Auth Guide](docs/auth.md)
- [Error Handling](docs/error-handling.md)
- [Versioning Policy](docs/versioning.md)
- [Endpoint Coverage Matrix](docs/endpoint-coverage.md)
- [Pinned OpenAPI Snapshot](docs/openapi/jortt-swagger-2026-05-05.json)

## Stability

This crate is currently `0.x` and follows a pre-1.0 compatibility policy (see [Versioning Policy](docs/versioning.md)).

