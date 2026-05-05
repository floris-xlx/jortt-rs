//! Shared model types.

use serde::{Deserialize, Serialize};

/// Generic API response wrapper for successful responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataEnvelope<T> {
    /// Wrapped payload.
    pub data: T,
}

/// Hypermedia links object used by list/detail responses.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Links {
    /// Link map for pagination or navigation.
    #[serde(flatten)]
    pub entries: std::collections::BTreeMap<String, String>,
}

/// Response containing collection data and optional links.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListEnvelope<T> {
    /// Collection payload.
    pub data: Vec<T>,
    /// Pagination/navigation links.
    #[serde(default)]
    pub _links: Option<Links>,
}

/// Money value with currency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Money {
    /// Decimal value represented as a string by the API.
    pub value: String,
    /// ISO 4217 currency code.
    pub currency: String,
}
