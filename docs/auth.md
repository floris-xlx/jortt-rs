# Auth Guide

## Hybrid Auth Model

The SDK intentionally separates token lifecycle from request execution.

- Request execution pulls bearer tokens from `AccessTokenSource`.
- OAuth grant/refresh flows are explicit helper calls through `OAuthClient`.
- No automatic refresh middleware is performed by `JorttClient`.

## `AccessTokenSource`

Implement this trait to supply an access token per request:

```rust
#[async_trait::async_trait]
trait AccessTokenSource {
    async fn access_token(&self) -> Result<String, jortt::JorttError>;
}
```

Use `StaticAccessToken` for simple integrations.

## OAuth Flows

`OAuthClient` supports:

- `exchange_authorization_code`
- `exchange_client_credentials`
- `refresh_access_token`

All methods return `TokenSet` with `access_token`, `token_type`, optional `refresh_token`, optional `expires_in`, and optional `scope`.

## Scope Values

The `Scope` enum includes the known Jortt scope set and a `Scope::Custom(String)` fallback for forward compatibility.
