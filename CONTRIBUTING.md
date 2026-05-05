# Contributing

## Setup

1. Install Rust stable and MSRV (`1.85`).
2. Clone repository.
3. Run:

```bash
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps
```

## Standards

- No `unsafe` code.
- Public APIs must be documented.
- New endpoints must be reflected in:
  - `docs/openapi/operations-*.json`
  - `src/api/operations.rs`
  - `docs/endpoint-coverage.md`

## Release Process

1. Update changelog.
2. Verify CI is green.
3. Run `cargo publish --dry-run`.
4. Tag and publish.
