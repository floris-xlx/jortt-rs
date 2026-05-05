//! Invoice domain models.

use serde::{Deserialize, Serialize};

use super::common::{ListEnvelope, Money};

/// Line item in an invoice request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceLineItemRequest {
    /// Description shown on the invoice.
    pub description: String,
    /// VAT percentage.
    pub vat: i32,
    /// Optional ledger account mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_account_id: Option<String>,
    /// Amount of units.
    pub units: i64,
    /// Unit price.
    pub amount_per_unit: Money,
}

/// Request payload for invoice creation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateInvoiceRequest {
    /// Customer UUID.
    pub customer_id: String,
    /// Invoice date in `YYYY-MM-DD`.
    pub invoice_date: String,
    /// Delivery period in `YYYY-MM-DD`.
    pub delivery_period: String,
    /// Reference identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Payment term in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<u32>,
    /// Whether line prices are net amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amounts: Option<bool>,
    /// Send method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_method: Option<String>,
    /// Intro text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<String>,
    /// Footer text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// Payment method enum string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Invoice line items.
    pub line_items: Vec<InvoiceLineItemRequest>,
}

/// Slim invoice model for list/get operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    /// Invoice id.
    pub id: String,
    /// Customer id.
    #[serde(default)]
    pub customer_id: Option<String>,
    /// Invoice number.
    #[serde(default)]
    pub invoice_number: Option<String>,
    /// Status string.
    #[serde(default)]
    pub invoice_status: Option<String>,
}

/// List invoices query.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ListInvoicesQuery {
    /// Free-text search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filter by invoice status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_status: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// Invoice list response.
pub type ListInvoicesResponse = ListEnvelope<Invoice>;

/// PDF download-location payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceDownload {
    /// Temporary URL to fetch the invoice PDF.
    pub download_location: String,
}
