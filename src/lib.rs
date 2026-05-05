#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

//! # Jortt Rust SDK
//!
//! Async-first Rust SDK for the Jortt API.
//! The crate provides:
//! - typed operation modules for all OpenAPI tags,
//! - a raw operation escape hatch,
//! - hybrid OAuth helpers, and
//! - robust API error parsing.

/// Typed operation APIs and request builders.
pub mod api;
/// Authentication abstractions and OAuth helpers.
pub mod auth;
/// High-level SDK client facade.
pub mod client;
/// Structured SDK and API errors.
pub mod error;
/// Strongly typed resource identifiers.
pub mod ids;
/// Request/response domain models.
pub mod models;
/// Raw escape hatch for ad-hoc endpoints.
pub mod raw;

pub use api::endpoints::ApiMethods;
pub use api::operations::*;
pub use api::{DomainApi, OperationRequest, PathParam, QueryParam, RequestBuilder};
pub use auth::{AccessTokenSource, OAuthClient, OAuthConfig, Scope, StaticAccessToken, TokenSet};
pub use client::{JorttClient, JorttClientBuilder};
pub use error::{ApiError, ApiErrorDetail, ErrorBuilder, JorttError};
pub use models::{
    CreateInvoiceRequest, Customer, Invoice, InvoiceDownload, InvoiceLineItemRequest,
    LedgerAccount, ListCustomersQuery, ListCustomersResponse, ListInvoicesQuery,
    ListInvoicesResponse, ListLedgerAccountsResponse, Money, UpsertCustomerRequest,
};
