//! Ledger account models.

use serde::{Deserialize, Serialize};

use super::common::ListEnvelope;

/// Ledger account that can be associated with invoice line items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerAccount {
    /// Ledger account id.
    pub id: String,
    /// Display name.
    #[serde(default)]
    pub name: Option<String>,
}

/// List ledger accounts response.
pub type ListLedgerAccountsResponse = ListEnvelope<LedgerAccount>;
