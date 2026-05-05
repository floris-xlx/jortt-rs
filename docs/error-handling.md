# Error Handling Guide

## Error Surfaces

`JorttError` variants:

- `Config`
- `MissingPathParam`
- `Transport`
- `Serialize`
- `Deserialize`
- `Api`
- `Http`

`ErrorBuilder` is the canonical way to assemble SDK errors. It is used internally by transport code and can also be used by extensions/adapters that need to map custom failures into `JorttError`.

## Structured API Error Parsing

When Jortt returns non-2xx JSON with an `error` object, the SDK maps to `JorttError::Api` and preserves:

- `error.code`
- `error.key`
- `error.message`
- `error.details[]`

Use this for retry policy, scope diagnostics, and user messaging.

## Retry Behavior

The transport retries on:

- request transport failures
- HTTP `429`
- HTTP `5xx`

Backoff schedule: `100ms * 2^attempt`, bounded by `max_retries`.

## Best Practices

- Match on `JorttError::Api` for domain-aware handling.
- Treat `error.key` as stable machine key.
- Log full `ApiError` payload for troubleshooting.
