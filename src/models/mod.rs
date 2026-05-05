//! Model namespace.

pub mod common;
pub mod customers;
pub mod invoices;
pub mod ledger_accounts;

pub use common::{DataEnvelope, Links, ListEnvelope, Money};
pub use customers::{Customer, ListCustomersQuery, ListCustomersResponse, UpsertCustomerRequest};
pub use invoices::{
    CreateInvoiceRequest, Invoice, InvoiceDownload, InvoiceLineItemRequest, ListInvoicesQuery,
    ListInvoicesResponse,
};
pub use ledger_accounts::{LedgerAccount, ListLedgerAccountsResponse};
