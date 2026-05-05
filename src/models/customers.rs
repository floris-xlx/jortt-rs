//! Customer domain models.

use serde::{Deserialize, Serialize};

use super::common::ListEnvelope;

/// Customer resource.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Customer {
    /// Customer id.
    pub id: String,
    /// Indicates whether customer is a private individual.
    pub is_private: bool,
    /// Company or display name.
    pub customer_name: String,
    /// Street line.
    #[serde(default)]
    pub address_street: Option<String>,
    /// Postal code.
    #[serde(default)]
    pub address_postal_code: Option<String>,
    /// City name.
    #[serde(default)]
    pub address_city: Option<String>,
    /// Country code.
    #[serde(default)]
    pub address_country_code: Option<String>,
    /// Primary e-mail.
    #[serde(default)]
    pub email: Option<String>,
}

/// Request payload for creating or updating a customer.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UpsertCustomerRequest {
    /// Indicates whether customer is private.
    pub is_private: bool,
    /// Company or display name.
    pub customer_name: String,
    /// Street line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_street: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    /// Country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country_code: Option<String>,
    /// Attention line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attn: Option<String>,
    /// Primary e-mail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Extra e-mails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_emails: Option<Vec<String>>,
    /// Payment term in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<u32>,
}

/// Customer search/list query options.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ListCustomersQuery {
    /// Search query (minimum 3 characters according to docs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Page number for paged endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// List customers response.
pub type ListCustomersResponse = ListEnvelope<Customer>;
