# Versioning Policy

## Current Line

This crate is published as `0.x`.

## Compatibility Contract (Pre-1.0)

- Minor releases (`0.x+1.0`) may include API breaking changes.
- Patch releases (`0.x.y+1`) must remain backwards compatible.
- Breaking changes are documented in `CHANGELOG.md` with migration notes.

## Endpoint Versioning Strategy

Jortt exposes unversioned and versioned routes (`/`, `/v1`, `/v2`, `/v3`).

SDK policy:

- Preserve all exposed operations from the pinned snapshot as typed variants.
- Do not silently rewrite versioned routes.
- Add new endpoints in additive releases when possible.
- Mark removed upstream endpoints in docs/changelog before removal from public enums.
