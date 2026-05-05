// @generated from docs/openapi/operations-2026-05-05.json
// Do not edit manually.

/// HTTP methods used in Jortt operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HttpMethod {
    /// `GET`
    Get,
    /// `POST`
    Post,
    /// `PUT`
    Put,
    /// `DELETE`
    Delete,
    /// `PATCH`
    Patch,
    /// `OPTIONS`
    Options,
    /// `HEAD`
    Head,
}

impl HttpMethod {
    /// Returns the uppercase wire format of the method.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Options => "OPTIONS",
            Self::Head => "HEAD",
        }
    }
}

/// Static description of a Jortt API operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OperationSpec {
    /// Domain tag from the OpenAPI spec.
    pub tag: &'static str,
    /// Operation id from the OpenAPI spec.
    pub operation_id: &'static str,
    /// HTTP method.
    pub method: HttpMethod,
    /// Relative path pattern including parameters.
    pub path: &'static str,
    /// Optional summary text.
    pub summary: &'static str,
}

/// Trait implemented by all typed operation enums.
pub trait TypedOperation: Copy {
    /// Returns the static OpenAPI metadata for this operation.
    fn spec(self) -> OperationSpec;
}

/// Typed operations for the 'bank-accounts' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BankAccountsOperation {
    /// GET /v3/bank-accounts
    GetV3BankAccounts,
    /// GET /v3/bank-accounts/{id}/bank-transactions
    GetV3BankAccountsByIdBankTransactions,
}

impl TypedOperation for BankAccountsOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetV3BankAccounts => OperationSpec {
                tag: "bank-accounts",
                operation_id: "List Bank Accounts V3",
                method: HttpMethod::Get,
                path: "/v3/bank-accounts",
                summary: "",
            },
            Self::GetV3BankAccountsByIdBankTransactions => OperationSpec {
                tag: "bank-accounts",
                operation_id: "List Bank Transactions V3",
                method: HttpMethod::Get,
                path: "/v3/bank-accounts/{id}/bank-transactions",
                summary: "",
            },
        }
    }
}

/// Typed operations for the 'customers' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CustomersOperation {
    /// GET /customers
    GetCustomers,
    /// POST /customers
    PostCustomers,
    /// DELETE /customers/{customer_id}
    DeleteCustomersByCustomerId,
    /// GET /customers/{customer_id}
    GetCustomersByCustomerId,
    /// PUT /customers/{customer_id}
    PutCustomersByCustomerId,
    /// POST /customers/{customer_id}/direct_debit_mandate
    PostCustomersByCustomerIdDirectDebitMandate,
    /// GET /customers/{customer_id}/extra_details
    GetCustomersByCustomerIdExtraDetails,
    /// PUT /customers/{customer_id}/set_archived
    PutCustomersByCustomerIdSetArchived,
    /// GET /customers/{customer_id}/vat-percentages
    GetCustomersByCustomerIdVatPercentages,
    /// GET /v1/customers
    GetV1Customers,
    /// POST /v1/customers
    PostV1Customers,
    /// DELETE /v1/customers/{customer_id}
    DeleteV1CustomersByCustomerId,
    /// GET /v1/customers/{customer_id}
    GetV1CustomersByCustomerId,
    /// PUT /v1/customers/{customer_id}
    PutV1CustomersByCustomerId,
    /// POST /v1/customers/{customer_id}/direct_debit_mandate
    PostV1CustomersByCustomerIdDirectDebitMandate,
    /// GET /v1/customers/{customer_id}/extra_details
    GetV1CustomersByCustomerIdExtraDetails,
    /// PUT /v1/customers/{customer_id}/set_archived
    PutV1CustomersByCustomerIdSetArchived,
    /// GET /v1/customers/{customer_id}/vat-percentages
    GetV1CustomersByCustomerIdVatPercentages,
    /// GET /v2/customers/{customer_id}/vat-percentages
    GetV2CustomersByCustomerIdVatPercentages,
}

impl TypedOperation for CustomersOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetCustomers => OperationSpec {
                tag: "customers",
                operation_id: "List Customers",
                method: HttpMethod::Get,
                path: "/customers",
                summary: "Returns a list of Customers",
            },
            Self::PostCustomers => OperationSpec {
                tag: "customers",
                operation_id: "Create Customer",
                method: HttpMethod::Post,
                path: "/customers",
                summary: "Creates a Customer",
            },
            Self::DeleteCustomersByCustomerId => OperationSpec {
                tag: "customers",
                operation_id: "Delete a Customer",
                method: HttpMethod::Delete,
                path: "/customers/{customer_id}",
                summary: "Delete a Customer",
            },
            Self::GetCustomersByCustomerId => OperationSpec {
                tag: "customers",
                operation_id: "Get Customer by ID",
                method: HttpMethod::Get,
                path: "/customers/{customer_id}",
                summary: "",
            },
            Self::PutCustomersByCustomerId => OperationSpec {
                tag: "customers",
                operation_id: "Update Customer",
                method: HttpMethod::Put,
                path: "/customers/{customer_id}",
                summary: "Updates a Customer",
            },
            Self::PostCustomersByCustomerIdDirectDebitMandate => OperationSpec {
                tag: "customers",
                operation_id: "Send direct debit authorization to a Customer",
                method: HttpMethod::Post,
                path: "/customers/{customer_id}/direct_debit_mandate",
                summary: "Send direct debit authorization to a Customer",
            },
            Self::GetCustomersByCustomerIdExtraDetails => OperationSpec {
                tag: "customers",
                operation_id: "Get Customer's details by ID",
                method: HttpMethod::Get,
                path: "/customers/{customer_id}/extra_details",
                summary: "",
            },
            Self::PutCustomersByCustomerIdSetArchived => OperationSpec {
                tag: "customers",
                operation_id: "Set Customer Archived",
                method: HttpMethod::Put,
                path: "/customers/{customer_id}/set_archived",
                summary: "Sets the archived status for a customer",
            },
            Self::GetCustomersByCustomerIdVatPercentages => OperationSpec {
                tag: "customers",
                operation_id: "Get vat percentages for a Customer by ID",
                method: HttpMethod::Get,
                path: "/customers/{customer_id}/vat-percentages",
                summary: "",
            },
            Self::GetV1Customers => OperationSpec {
                tag: "customers",
                operation_id: "List Customers",
                method: HttpMethod::Get,
                path: "/v1/customers",
                summary: "Returns a list of Customers",
            },
            Self::PostV1Customers => OperationSpec {
                tag: "customers",
                operation_id: "Create Customer",
                method: HttpMethod::Post,
                path: "/v1/customers",
                summary: "Creates a Customer",
            },
            Self::DeleteV1CustomersByCustomerId => OperationSpec {
                tag: "customers",
                operation_id: "Delete a Customer",
                method: HttpMethod::Delete,
                path: "/v1/customers/{customer_id}",
                summary: "Delete a Customer",
            },
            Self::GetV1CustomersByCustomerId => OperationSpec {
                tag: "customers",
                operation_id: "Get Customer by ID",
                method: HttpMethod::Get,
                path: "/v1/customers/{customer_id}",
                summary: "",
            },
            Self::PutV1CustomersByCustomerId => OperationSpec {
                tag: "customers",
                operation_id: "Update Customer",
                method: HttpMethod::Put,
                path: "/v1/customers/{customer_id}",
                summary: "Updates a Customer",
            },
            Self::PostV1CustomersByCustomerIdDirectDebitMandate => OperationSpec {
                tag: "customers",
                operation_id: "Send direct debit authorization to a Customer",
                method: HttpMethod::Post,
                path: "/v1/customers/{customer_id}/direct_debit_mandate",
                summary: "Send direct debit authorization to a Customer",
            },
            Self::GetV1CustomersByCustomerIdExtraDetails => OperationSpec {
                tag: "customers",
                operation_id: "Get Customer's details by ID",
                method: HttpMethod::Get,
                path: "/v1/customers/{customer_id}/extra_details",
                summary: "",
            },
            Self::PutV1CustomersByCustomerIdSetArchived => OperationSpec {
                tag: "customers",
                operation_id: "Set Customer Archived",
                method: HttpMethod::Put,
                path: "/v1/customers/{customer_id}/set_archived",
                summary: "Sets the archived status for a customer",
            },
            Self::GetV1CustomersByCustomerIdVatPercentages => OperationSpec {
                tag: "customers",
                operation_id: "Get vat percentages for a Customer by ID",
                method: HttpMethod::Get,
                path: "/v1/customers/{customer_id}/vat-percentages",
                summary: "",
            },
            Self::GetV2CustomersByCustomerIdVatPercentages => OperationSpec {
                tag: "customers",
                operation_id: "Get vat percentages for a Customer by ID V2",
                method: HttpMethod::Get,
                path: "/v2/customers/{customer_id}/vat-percentages",
                summary: "",
            },
        }
    }
}

/// Typed operations for the 'estimates' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EstimatesOperation {
    /// POST /v2/estimates
    PostV2Estimates,
    /// PUT /v2/estimates/{id}
    PutV2EstimatesById,
    /// POST /v2/estimates/{id}/send
    PostV2EstimatesByIdSend,
}

impl TypedOperation for EstimatesOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::PostV2Estimates => OperationSpec {
                tag: "estimates",
                operation_id: "Create (and optionally send) an Estimate V2",
                method: HttpMethod::Post,
                path: "/v2/estimates",
                summary: "Creates (and optionally sends) an Estimate",
            },
            Self::PutV2EstimatesById => OperationSpec {
                tag: "estimates",
                operation_id: "Edit Estimate V2",
                method: HttpMethod::Put,
                path: "/v2/estimates/{id}",
                summary: "Edits an Estimate",
            },
            Self::PostV2EstimatesByIdSend => OperationSpec {
                tag: "estimates",
                operation_id: "Send Estimate V2",
                method: HttpMethod::Post,
                path: "/v2/estimates/{id}/send",
                summary: "Sends an Estimate",
            },
        }
    }
}

/// Typed operations for the 'expenses' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpensesOperation {
    /// GET /v3/expenses
    GetV3Expenses,
    /// POST /v3/expenses
    PostV3Expenses,
    /// GET /v3/expenses/id/{id}
    GetV3ExpensesIdById,
    /// POST /v3/expenses/id/{id}
    PostV3ExpensesIdById,
    /// POST /v3/expenses/id/{id}/receipt
    PostV3ExpensesIdByIdReceipt,
}

impl TypedOperation for ExpensesOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetV3Expenses => OperationSpec {
                tag: "expenses",
                operation_id: "List Expenses",
                method: HttpMethod::Get,
                path: "/v3/expenses",
                summary: "List Expenses",
            },
            Self::PostV3Expenses => OperationSpec {
                tag: "expenses",
                operation_id: "Create an Expense",
                method: HttpMethod::Post,
                path: "/v3/expenses",
                summary: "Create an Expense",
            },
            Self::GetV3ExpensesIdById => OperationSpec {
                tag: "expenses",
                operation_id: "Get Expense by ID",
                method: HttpMethod::Get,
                path: "/v3/expenses/id/{id}",
                summary: "Get Expense by ID",
            },
            Self::PostV3ExpensesIdById => OperationSpec {
                tag: "expenses",
                operation_id: "Update an Expense",
                method: HttpMethod::Post,
                path: "/v3/expenses/id/{id}",
                summary: "Update an Expense",
            },
            Self::PostV3ExpensesIdByIdReceipt => OperationSpec {
                tag: "expenses",
                operation_id: "Attach Receipt to Expense",
                method: HttpMethod::Post,
                path: "/v3/expenses/id/{id}/receipt",
                summary: "Attach a Receipt to an Expense",
            },
        }
    }
}

/// Typed operations for the 'files' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilesOperation {
    /// GET /files/put_url
    GetFilesPutUrl,
    /// GET /v1/files/put_url
    GetV1FilesPutUrl,
}

impl TypedOperation for FilesOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetFilesPutUrl => OperationSpec {
                tag: "files",
                operation_id: "Upload URL for attachments",
                method: HttpMethod::Get,
                path: "/files/put_url",
                summary: "Request an upload URL for attachments",
            },
            Self::GetV1FilesPutUrl => OperationSpec {
                tag: "files",
                operation_id: "Upload URL for attachments",
                method: HttpMethod::Get,
                path: "/v1/files/put_url",
                summary: "Request an upload URL for attachments",
            },
        }
    }
}

/// Typed operations for the 'inbox' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InboxOperation {
    /// POST /inbox/images
    PostInboxImages,
    /// POST /v1/inbox/images
    PostV1InboxImages,
}

impl TypedOperation for InboxOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::PostInboxImages => OperationSpec {
                tag: "inbox",
                operation_id: "Upload receipt",
                method: HttpMethod::Post,
                path: "/inbox/images",
                summary: "Upload images to jortt",
            },
            Self::PostV1InboxImages => OperationSpec {
                tag: "inbox",
                operation_id: "Upload receipt",
                method: HttpMethod::Post,
                path: "/v1/inbox/images",
                summary: "Upload images to jortt",
            },
        }
    }
}

/// Typed operations for the 'invoices' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvoicesOperation {
    /// GET /invoices
    GetInvoices,
    /// POST /invoices
    PostInvoices,
    /// DELETE /invoices/{id}
    DeleteInvoicesById,
    /// GET /invoices/{id}
    GetInvoicesById,
    /// PUT /invoices/{id}
    PutInvoicesById,
    /// POST /invoices/{id}/copy
    PostInvoicesByIdCopy,
    /// POST /invoices/{id}/credit
    PostInvoicesByIdCredit,
    /// GET /invoices/{id}/download
    GetInvoicesByIdDownload,
    /// GET /invoices/{id}/line_item_suggestions
    GetInvoicesByIdLineItemSuggestions,
    /// GET /invoices/{id}/next_possible_invoice_number
    GetInvoicesByIdNextPossibleInvoiceNumber,
    /// POST /invoices/{id}/send
    PostInvoicesByIdSend,
    /// GET /invoices/{id}/send_settings
    GetInvoicesByIdSendSettings,
    /// PUT /invoices/{id}/set_labels
    PutInvoicesByIdSetLabels,
    /// GET /v1/invoices
    GetV1Invoices,
    /// POST /v1/invoices
    PostV1Invoices,
    /// DELETE /v1/invoices/{id}
    DeleteV1InvoicesById,
    /// GET /v1/invoices/{id}
    GetV1InvoicesById,
    /// PUT /v1/invoices/{id}
    PutV1InvoicesById,
    /// POST /v1/invoices/{id}/copy
    PostV1InvoicesByIdCopy,
    /// POST /v1/invoices/{id}/credit
    PostV1InvoicesByIdCredit,
    /// GET /v1/invoices/{id}/download
    GetV1InvoicesByIdDownload,
    /// GET /v1/invoices/{id}/line_item_suggestions
    GetV1InvoicesByIdLineItemSuggestions,
    /// GET /v1/invoices/{id}/next_possible_invoice_number
    GetV1InvoicesByIdNextPossibleInvoiceNumber,
    /// POST /v1/invoices/{id}/send
    PostV1InvoicesByIdSend,
    /// GET /v1/invoices/{id}/send_settings
    GetV1InvoicesByIdSendSettings,
    /// PUT /v1/invoices/{id}/set_labels
    PutV1InvoicesByIdSetLabels,
    /// POST /v2/invoices
    PostV2Invoices,
    /// PUT /v2/invoices/{id}
    PutV2InvoicesById,
    /// GET /v2/invoices/{id}/line_item_suggestions
    GetV2InvoicesByIdLineItemSuggestions,
    /// GET /v2/invoices/peppol-scheme-catalog
    GetV2InvoicesPeppolSchemeCatalog,
    /// POST /v3/invoices
    PostV3Invoices,
}

impl TypedOperation for InvoicesOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetInvoices => OperationSpec {
                tag: "invoices",
                operation_id: "List Invoices",
                method: HttpMethod::Get,
                path: "/invoices",
                summary: "Returns a list of Invoices",
            },
            Self::PostInvoices => OperationSpec {
                tag: "invoices",
                operation_id: "Create (and optionally send) an Invoice",
                method: HttpMethod::Post,
                path: "/invoices",
                summary: "Creates (and optionally sends) an Invoice",
            },
            Self::DeleteInvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Delete Invoice by ID",
                method: HttpMethod::Delete,
                path: "/invoices/{id}",
                summary: "",
            },
            Self::GetInvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Get Invoice by ID",
                method: HttpMethod::Get,
                path: "/invoices/{id}",
                summary: "",
            },
            Self::PutInvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Edit Invoice",
                method: HttpMethod::Put,
                path: "/invoices/{id}",
                summary: "Edits an Invoice",
            },
            Self::PostInvoicesByIdCopy => OperationSpec {
                tag: "invoices",
                operation_id: "Copy an Invoice",
                method: HttpMethod::Post,
                path: "/invoices/{id}/copy",
                summary: "Copies an Invoice",
            },
            Self::PostInvoicesByIdCredit => OperationSpec {
                tag: "invoices",
                operation_id: "Creates (and optionally sends) a credit Invoice",
                method: HttpMethod::Post,
                path: "/invoices/{id}/credit",
                summary: "Creates (and optionally sends) a credit Invoice",
            },
            Self::GetInvoicesByIdDownload => OperationSpec {
                tag: "invoices",
                operation_id: "Download Invoice PDF",
                method: HttpMethod::Get,
                path: "/invoices/{id}/download",
                summary: "Returns a URL from which the invoice PDF can be downloaded.",
            },
            Self::GetInvoicesByIdLineItemSuggestions => OperationSpec {
                tag: "invoices",
                operation_id: "Get line item suggestions",
                method: HttpMethod::Get,
                path: "/invoices/{id}/line_item_suggestions",
                summary: "Returns a list of suggested line items",
            },
            Self::GetInvoicesByIdNextPossibleInvoiceNumber => OperationSpec {
                tag: "invoices",
                operation_id: "Get next possible invoice number",
                method: HttpMethod::Get,
                path: "/invoices/{id}/next_possible_invoice_number",
                summary: "",
            },
            Self::PostInvoicesByIdSend => OperationSpec {
                tag: "invoices",
                operation_id: "Send Invoice",
                method: HttpMethod::Post,
                path: "/invoices/{id}/send",
                summary: "Sends an Invoice",
            },
            Self::GetInvoicesByIdSendSettings => OperationSpec {
                tag: "invoices",
                operation_id: "Get send settings",
                method: HttpMethod::Get,
                path: "/invoices/{id}/send_settings",
                summary: "",
            },
            Self::PutInvoicesByIdSetLabels => OperationSpec {
                tag: "invoices",
                operation_id: "Set invoice labels",
                method: HttpMethod::Put,
                path: "/invoices/{id}/set_labels",
                summary: "Sets the labels for a given invoice",
            },
            Self::GetV1Invoices => OperationSpec {
                tag: "invoices",
                operation_id: "List Invoices",
                method: HttpMethod::Get,
                path: "/v1/invoices",
                summary: "Returns a list of Invoices",
            },
            Self::PostV1Invoices => OperationSpec {
                tag: "invoices",
                operation_id: "Create (and optionally send) an Invoice",
                method: HttpMethod::Post,
                path: "/v1/invoices",
                summary: "Creates (and optionally sends) an Invoice",
            },
            Self::DeleteV1InvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Delete Invoice by ID",
                method: HttpMethod::Delete,
                path: "/v1/invoices/{id}",
                summary: "",
            },
            Self::GetV1InvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Get Invoice by ID",
                method: HttpMethod::Get,
                path: "/v1/invoices/{id}",
                summary: "",
            },
            Self::PutV1InvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Edit Invoice",
                method: HttpMethod::Put,
                path: "/v1/invoices/{id}",
                summary: "Edits an Invoice",
            },
            Self::PostV1InvoicesByIdCopy => OperationSpec {
                tag: "invoices",
                operation_id: "Copy an Invoice",
                method: HttpMethod::Post,
                path: "/v1/invoices/{id}/copy",
                summary: "Copies an Invoice",
            },
            Self::PostV1InvoicesByIdCredit => OperationSpec {
                tag: "invoices",
                operation_id: "Creates (and optionally sends) a credit Invoice",
                method: HttpMethod::Post,
                path: "/v1/invoices/{id}/credit",
                summary: "Creates (and optionally sends) a credit Invoice",
            },
            Self::GetV1InvoicesByIdDownload => OperationSpec {
                tag: "invoices",
                operation_id: "Download Invoice PDF",
                method: HttpMethod::Get,
                path: "/v1/invoices/{id}/download",
                summary: "Returns a URL from which the invoice PDF can be downloaded.",
            },
            Self::GetV1InvoicesByIdLineItemSuggestions => OperationSpec {
                tag: "invoices",
                operation_id: "Get line item suggestions",
                method: HttpMethod::Get,
                path: "/v1/invoices/{id}/line_item_suggestions",
                summary: "Returns a list of suggested line items",
            },
            Self::GetV1InvoicesByIdNextPossibleInvoiceNumber => OperationSpec {
                tag: "invoices",
                operation_id: "Get next possible invoice number",
                method: HttpMethod::Get,
                path: "/v1/invoices/{id}/next_possible_invoice_number",
                summary: "",
            },
            Self::PostV1InvoicesByIdSend => OperationSpec {
                tag: "invoices",
                operation_id: "Send Invoice",
                method: HttpMethod::Post,
                path: "/v1/invoices/{id}/send",
                summary: "Sends an Invoice",
            },
            Self::GetV1InvoicesByIdSendSettings => OperationSpec {
                tag: "invoices",
                operation_id: "Get send settings",
                method: HttpMethod::Get,
                path: "/v1/invoices/{id}/send_settings",
                summary: "",
            },
            Self::PutV1InvoicesByIdSetLabels => OperationSpec {
                tag: "invoices",
                operation_id: "Set invoice labels",
                method: HttpMethod::Put,
                path: "/v1/invoices/{id}/set_labels",
                summary: "Sets the labels for a given invoice",
            },
            Self::PostV2Invoices => OperationSpec {
                tag: "invoices",
                operation_id: "Create (and optionally send) an Invoice V2",
                method: HttpMethod::Post,
                path: "/v2/invoices",
                summary: "Creates (and optionally sends) an Invoice V2",
            },
            Self::PutV2InvoicesById => OperationSpec {
                tag: "invoices",
                operation_id: "Edit Invoice V2",
                method: HttpMethod::Put,
                path: "/v2/invoices/{id}",
                summary: "Edits an Invoice V2",
            },
            Self::GetV2InvoicesByIdLineItemSuggestions => OperationSpec {
                tag: "invoices",
                operation_id: "Get line item suggestions",
                method: HttpMethod::Get,
                path: "/v2/invoices/{id}/line_item_suggestions",
                summary: "Returns a list of suggested line items",
            },
            Self::GetV2InvoicesPeppolSchemeCatalog => OperationSpec {
                tag: "invoices",
                operation_id: "Get peppol scheme catalog",
                method: HttpMethod::Get,
                path: "/v2/invoices/peppol-scheme-catalog",
                summary: "",
            },
            Self::PostV3Invoices => OperationSpec {
                tag: "invoices",
                operation_id: "Create (and optionally send) an Invoice V3",
                method: HttpMethod::Post,
                path: "/v3/invoices",
                summary: "Creates (and optionally sends) an Invoice V3 with invoice-level VAT",
            },
        }
    }
}

/// Typed operations for the 'labels' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelsOperation {
    /// GET /labels
    GetLabels,
    /// POST /labels
    PostLabels,
    /// GET /v1/labels
    GetV1Labels,
    /// POST /v1/labels
    PostV1Labels,
}

impl TypedOperation for LabelsOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetLabels => OperationSpec {
                tag: "labels",
                operation_id: "List labels",
                method: HttpMethod::Get,
                path: "/labels",
                summary: "Returns a list of labels",
            },
            Self::PostLabels => OperationSpec {
                tag: "labels",
                operation_id: "Create a Label",
                method: HttpMethod::Post,
                path: "/labels",
                summary: "Create a label",
            },
            Self::GetV1Labels => OperationSpec {
                tag: "labels",
                operation_id: "List labels",
                method: HttpMethod::Get,
                path: "/v1/labels",
                summary: "Returns a list of labels",
            },
            Self::PostV1Labels => OperationSpec {
                tag: "labels",
                operation_id: "Create a Label",
                method: HttpMethod::Post,
                path: "/v1/labels",
                summary: "Create a label",
            },
        }
    }
}

/// Typed operations for the 'ledger_accounts' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LedgerAccountsOperation {
    /// GET /ledger_accounts/invoices
    GetLedgerAccountsInvoices,
    /// GET /ledger_accounts/invoices/default
    GetLedgerAccountsInvoicesDefault,
    /// GET /v1/ledger_accounts/invoices
    GetV1LedgerAccountsInvoices,
    /// GET /v1/ledger_accounts/invoices/default
    GetV1LedgerAccountsInvoicesDefault,
    /// GET /v3/ledger_accounts/expenses/balance
    GetV3LedgerAccountsExpensesBalance,
    /// GET /v3/ledger_accounts/expenses/cost
    GetV3LedgerAccountsExpensesCost,
    /// GET /v3/ledger_accounts/expenses/income
    GetV3LedgerAccountsExpensesIncome,
}

impl TypedOperation for LedgerAccountsOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetLedgerAccountsInvoices => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "List Invoice Ledger Accounts",
                method: HttpMethod::Get,
                path: "/ledger_accounts/invoices",
                summary: "",
            },
            Self::GetLedgerAccountsInvoicesDefault => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "The default Ledger Account for invoice line items",
                method: HttpMethod::Get,
                path: "/ledger_accounts/invoices/default",
                summary: "",
            },
            Self::GetV1LedgerAccountsInvoices => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "List Invoice Ledger Accounts",
                method: HttpMethod::Get,
                path: "/v1/ledger_accounts/invoices",
                summary: "",
            },
            Self::GetV1LedgerAccountsInvoicesDefault => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "The default Ledger Account for invoice line items",
                method: HttpMethod::Get,
                path: "/v1/ledger_accounts/invoices/default",
                summary: "",
            },
            Self::GetV3LedgerAccountsExpensesBalance => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "getV3LedgerAccountsExpensesBalance",
                method: HttpMethod::Get,
                path: "/v3/ledger_accounts/expenses/balance",
                summary: "",
            },
            Self::GetV3LedgerAccountsExpensesCost => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "getV3LedgerAccountsExpensesCost",
                method: HttpMethod::Get,
                path: "/v3/ledger_accounts/expenses/cost",
                summary: "",
            },
            Self::GetV3LedgerAccountsExpensesIncome => OperationSpec {
                tag: "ledger_accounts",
                operation_id: "getV3LedgerAccountsExpensesIncome",
                method: HttpMethod::Get,
                path: "/v3/ledger_accounts/expenses/income",
                summary: "",
            },
        }
    }
}

/// Typed operations for the 'loonjournaalposten' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoonjournaalpostenOperation {
    /// GET /loonjournaalposten
    GetLoonjournaalposten,
    /// POST /loonjournaalposten
    PostLoonjournaalposten,
    /// DELETE /loonjournaalposten/{loonjournaalpost_id}
    DeleteLoonjournaalpostenByLoonjournaalpostId,
    /// PUT /loonjournaalposten/{loonjournaalpost_id}
    PutLoonjournaalpostenByLoonjournaalpostId,
    /// GET /v1/loonjournaalposten
    GetV1Loonjournaalposten,
    /// POST /v1/loonjournaalposten
    PostV1Loonjournaalposten,
    /// DELETE /v1/loonjournaalposten/{loonjournaalpost_id}
    DeleteV1LoonjournaalpostenByLoonjournaalpostId,
    /// PUT /v1/loonjournaalposten/{loonjournaalpost_id}
    PutV1LoonjournaalpostenByLoonjournaalpostId,
}

impl TypedOperation for LoonjournaalpostenOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetLoonjournaalposten => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "List Loonjournaalposten",
                method: HttpMethod::Get,
                path: "/loonjournaalposten",
                summary: "Returns a list of Loonjournaalposten",
            },
            Self::PostLoonjournaalposten => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "Create a Loonjournaalpost",
                method: HttpMethod::Post,
                path: "/loonjournaalposten",
                summary: "Create a Loonjournaalpost",
            },
            Self::DeleteLoonjournaalpostenByLoonjournaalpostId => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "Delete a Loonjournaalpost",
                method: HttpMethod::Delete,
                path: "/loonjournaalposten/{loonjournaalpost_id}",
                summary: "Delete a Loonjournaalpost",
            },
            Self::PutLoonjournaalpostenByLoonjournaalpostId => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "Update a Loonjournaalpost",
                method: HttpMethod::Put,
                path: "/loonjournaalposten/{loonjournaalpost_id}",
                summary: "Update a Loonjournaalpost",
            },
            Self::GetV1Loonjournaalposten => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "List Loonjournaalposten",
                method: HttpMethod::Get,
                path: "/v1/loonjournaalposten",
                summary: "Returns a list of Loonjournaalposten",
            },
            Self::PostV1Loonjournaalposten => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "Create a Loonjournaalpost",
                method: HttpMethod::Post,
                path: "/v1/loonjournaalposten",
                summary: "Create a Loonjournaalpost",
            },
            Self::DeleteV1LoonjournaalpostenByLoonjournaalpostId => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "Delete a Loonjournaalpost",
                method: HttpMethod::Delete,
                path: "/v1/loonjournaalposten/{loonjournaalpost_id}",
                summary: "Delete a Loonjournaalpost",
            },
            Self::PutV1LoonjournaalpostenByLoonjournaalpostId => OperationSpec {
                tag: "loonjournaalposten",
                operation_id: "Update a Loonjournaalpost",
                method: HttpMethod::Put,
                path: "/v1/loonjournaalposten/{loonjournaalpost_id}",
                summary: "Update a Loonjournaalpost",
            },
        }
    }
}

/// Typed operations for the 'organizations' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganizationsOperation {
    /// GET /organizations/me
    GetOrganizationsMe,
    /// GET /v1/organizations/me
    GetV1OrganizationsMe,
}

impl TypedOperation for OrganizationsOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetOrganizationsMe => OperationSpec {
                tag: "organizations",
                operation_id: "Get the organization associated with the api credentials",
                method: HttpMethod::Get,
                path: "/organizations/me",
                summary: "",
            },
            Self::GetV1OrganizationsMe => OperationSpec {
                tag: "organizations",
                operation_id: "Get the organization associated with the api credentials",
                method: HttpMethod::Get,
                path: "/v1/organizations/me",
                summary: "",
            },
        }
    }
}

/// Typed operations for the 'projects' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectsOperation {
    /// GET /projects
    GetProjects,
    /// POST /projects
    PostProjects,
    /// DELETE /projects/{id}
    DeleteProjectsById,
    /// GET /projects/{id}
    GetProjectsById,
    /// PUT /projects/{id}
    PutProjectsById,
    /// POST /projects/{id}/invoice
    PostProjectsByIdInvoice,
    /// GET /projects/{id}/line_items
    GetProjectsByIdLineItems,
    /// POST /projects/{id}/line_items
    PostProjectsByIdLineItems,
    /// DELETE /projects/{id}/line_items/{line_item_id}
    DeleteProjectsByIdLineItemsByLineItemId,
    /// GET /projects/{id}/line_items/{line_item_id}
    GetProjectsByIdLineItemsByLineItemId,
    /// PUT /projects/{id}/line_items/{line_item_id}
    PutProjectsByIdLineItemsByLineItemId,
    /// GET /projects/{id}/line_items/summary
    GetProjectsByIdLineItemsSummary,
    /// GET /v1/projects
    GetV1Projects,
    /// POST /v1/projects
    PostV1Projects,
    /// DELETE /v1/projects/{id}
    DeleteV1ProjectsById,
    /// GET /v1/projects/{id}
    GetV1ProjectsById,
    /// PUT /v1/projects/{id}
    PutV1ProjectsById,
    /// POST /v1/projects/{id}/invoice
    PostV1ProjectsByIdInvoice,
    /// GET /v1/projects/{id}/line_items
    GetV1ProjectsByIdLineItems,
    /// POST /v1/projects/{id}/line_items
    PostV1ProjectsByIdLineItems,
    /// DELETE /v1/projects/{id}/line_items/{line_item_id}
    DeleteV1ProjectsByIdLineItemsByLineItemId,
    /// GET /v1/projects/{id}/line_items/{line_item_id}
    GetV1ProjectsByIdLineItemsByLineItemId,
    /// PUT /v1/projects/{id}/line_items/{line_item_id}
    PutV1ProjectsByIdLineItemsByLineItemId,
    /// GET /v1/projects/{id}/line_items/summary
    GetV1ProjectsByIdLineItemsSummary,
}

impl TypedOperation for ProjectsOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetProjects => OperationSpec {
                tag: "projects",
                operation_id: "List Projects",
                method: HttpMethod::Get,
                path: "/projects",
                summary: "Returns a list of Projects",
            },
            Self::PostProjects => OperationSpec {
                tag: "projects",
                operation_id: "Create Project",
                method: HttpMethod::Post,
                path: "/projects",
                summary: "Creates a Project",
            },
            Self::DeleteProjectsById => OperationSpec {
                tag: "projects",
                operation_id: "Delete Project",
                method: HttpMethod::Delete,
                path: "/projects/{id}",
                summary: "Deletes a Project",
            },
            Self::GetProjectsById => OperationSpec {
                tag: "projects",
                operation_id: "Get Project",
                method: HttpMethod::Get,
                path: "/projects/{id}",
                summary: "Returns a Project",
            },
            Self::PutProjectsById => OperationSpec {
                tag: "projects",
                operation_id: "Update Project",
                method: HttpMethod::Put,
                path: "/projects/{id}",
                summary: "Updates a Project",
            },
            Self::PostProjectsByIdInvoice => OperationSpec {
                tag: "projects",
                operation_id: "Invoice Project",
                method: HttpMethod::Post,
                path: "/projects/{id}/invoice",
                summary: "Creates an invoice based on project line items",
            },
            Self::GetProjectsByIdLineItems => OperationSpec {
                tag: "projects",
                operation_id: "List Project Line Items",
                method: HttpMethod::Get,
                path: "/projects/{id}/line_items",
                summary: "Returns a list of Project Line Items",
            },
            Self::PostProjectsByIdLineItems => OperationSpec {
                tag: "projects",
                operation_id: "Create Project Line item",
                method: HttpMethod::Post,
                path: "/projects/{id}/line_items",
                summary: "Creates a Project Line Item",
            },
            Self::DeleteProjectsByIdLineItemsByLineItemId => OperationSpec {
                tag: "projects",
                operation_id: "Delete Project Line item",
                method: HttpMethod::Delete,
                path: "/projects/{id}/line_items/{line_item_id}",
                summary: "Deletes a Project Line item",
            },
            Self::GetProjectsByIdLineItemsByLineItemId => OperationSpec {
                tag: "projects",
                operation_id: "Get Project Line Item",
                method: HttpMethod::Get,
                path: "/projects/{id}/line_items/{line_item_id}",
                summary: "Returns a Project Line Item",
            },
            Self::PutProjectsByIdLineItemsByLineItemId => OperationSpec {
                tag: "projects",
                operation_id: "Update Project Line item",
                method: HttpMethod::Put,
                path: "/projects/{id}/line_items/{line_item_id}",
                summary: "Updates a Project Line item",
            },
            Self::GetProjectsByIdLineItemsSummary => OperationSpec {
                tag: "projects",
                operation_id: "Get Project Line Item Summary",
                method: HttpMethod::Get,
                path: "/projects/{id}/line_items/summary",
                summary: "Returns a list of Project Line Item summaries",
            },
            Self::GetV1Projects => OperationSpec {
                tag: "projects",
                operation_id: "List Projects",
                method: HttpMethod::Get,
                path: "/v1/projects",
                summary: "Returns a list of Projects",
            },
            Self::PostV1Projects => OperationSpec {
                tag: "projects",
                operation_id: "Create Project",
                method: HttpMethod::Post,
                path: "/v1/projects",
                summary: "Creates a Project",
            },
            Self::DeleteV1ProjectsById => OperationSpec {
                tag: "projects",
                operation_id: "Delete Project",
                method: HttpMethod::Delete,
                path: "/v1/projects/{id}",
                summary: "Deletes a Project",
            },
            Self::GetV1ProjectsById => OperationSpec {
                tag: "projects",
                operation_id: "Get Project",
                method: HttpMethod::Get,
                path: "/v1/projects/{id}",
                summary: "Returns a Project",
            },
            Self::PutV1ProjectsById => OperationSpec {
                tag: "projects",
                operation_id: "Update Project",
                method: HttpMethod::Put,
                path: "/v1/projects/{id}",
                summary: "Updates a Project",
            },
            Self::PostV1ProjectsByIdInvoice => OperationSpec {
                tag: "projects",
                operation_id: "Invoice Project",
                method: HttpMethod::Post,
                path: "/v1/projects/{id}/invoice",
                summary: "Creates an invoice based on project line items",
            },
            Self::GetV1ProjectsByIdLineItems => OperationSpec {
                tag: "projects",
                operation_id: "List Project Line Items",
                method: HttpMethod::Get,
                path: "/v1/projects/{id}/line_items",
                summary: "Returns a list of Project Line Items",
            },
            Self::PostV1ProjectsByIdLineItems => OperationSpec {
                tag: "projects",
                operation_id: "Create Project Line item",
                method: HttpMethod::Post,
                path: "/v1/projects/{id}/line_items",
                summary: "Creates a Project Line Item",
            },
            Self::DeleteV1ProjectsByIdLineItemsByLineItemId => OperationSpec {
                tag: "projects",
                operation_id: "Delete Project Line item",
                method: HttpMethod::Delete,
                path: "/v1/projects/{id}/line_items/{line_item_id}",
                summary: "Deletes a Project Line item",
            },
            Self::GetV1ProjectsByIdLineItemsByLineItemId => OperationSpec {
                tag: "projects",
                operation_id: "Get Project Line Item",
                method: HttpMethod::Get,
                path: "/v1/projects/{id}/line_items/{line_item_id}",
                summary: "Returns a Project Line Item",
            },
            Self::PutV1ProjectsByIdLineItemsByLineItemId => OperationSpec {
                tag: "projects",
                operation_id: "Update Project Line item",
                method: HttpMethod::Put,
                path: "/v1/projects/{id}/line_items/{line_item_id}",
                summary: "Updates a Project Line item",
            },
            Self::GetV1ProjectsByIdLineItemsSummary => OperationSpec {
                tag: "projects",
                operation_id: "Get Project Line Item Summary",
                method: HttpMethod::Get,
                path: "/v1/projects/{id}/line_items/summary",
                summary: "Returns a list of Project Line Item summaries",
            },
        }
    }
}

/// Typed operations for the 'reports' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportsOperation {
    /// GET /reports/summaries/balance
    GetReportsSummariesBalance,
    /// GET /reports/summaries/btw
    GetReportsSummariesBtw,
    /// GET /reports/summaries/cash_and_bank
    GetReportsSummariesCashAndBank,
    /// GET /reports/summaries/invoices
    GetReportsSummariesInvoices,
    /// GET /reports/summaries/profit_and_loss
    GetReportsSummariesProfitAndLoss,
    /// GET /v1/reports/summaries/balance
    GetV1ReportsSummariesBalance,
    /// GET /v1/reports/summaries/btw
    GetV1ReportsSummariesBtw,
    /// GET /v1/reports/summaries/cash_and_bank
    GetV1ReportsSummariesCashAndBank,
    /// GET /v1/reports/summaries/invoices
    GetV1ReportsSummariesInvoices,
    /// GET /v1/reports/summaries/profit_and_loss
    GetV1ReportsSummariesProfitAndLoss,
}

impl TypedOperation for ReportsOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetReportsSummariesBalance => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard balance",
                method: HttpMethod::Get,
                path: "/reports/summaries/balance",
                summary: "Returns key organization balances for the current date",
            },
            Self::GetReportsSummariesBtw => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard btw",
                method: HttpMethod::Get,
                path: "/reports/summaries/btw",
                summary: "Returns a list of summarized btw periods",
            },
            Self::GetReportsSummariesCashAndBank => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard cash and bank",
                method: HttpMethod::Get,
                path: "/reports/summaries/cash_and_bank",
                summary: "Returns a summary of bank accounts, cash and liquid assets",
            },
            Self::GetReportsSummariesInvoices => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard invoices",
                method: HttpMethod::Get,
                path: "/reports/summaries/invoices",
                summary: "Returns a summary of invoices for the current year",
            },
            Self::GetReportsSummariesProfitAndLoss => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard profit and loss",
                method: HttpMethod::Get,
                path: "/reports/summaries/profit_and_loss",
                summary: "Returns a summary of profit and loss for the current year",
            },
            Self::GetV1ReportsSummariesBalance => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard balance",
                method: HttpMethod::Get,
                path: "/v1/reports/summaries/balance",
                summary: "Returns key organization balances for the current date",
            },
            Self::GetV1ReportsSummariesBtw => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard btw",
                method: HttpMethod::Get,
                path: "/v1/reports/summaries/btw",
                summary: "Returns a list of summarized btw periods",
            },
            Self::GetV1ReportsSummariesCashAndBank => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard cash and bank",
                method: HttpMethod::Get,
                path: "/v1/reports/summaries/cash_and_bank",
                summary: "Returns a summary of bank accounts, cash and liquid assets",
            },
            Self::GetV1ReportsSummariesInvoices => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard invoices",
                method: HttpMethod::Get,
                path: "/v1/reports/summaries/invoices",
                summary: "Returns a summary of invoices for the current year",
            },
            Self::GetV1ReportsSummariesProfitAndLoss => OperationSpec {
                tag: "reports",
                operation_id: "Dashboard profit and loss",
                method: HttpMethod::Get,
                path: "/v1/reports/summaries/profit_and_loss",
                summary: "Returns a summary of profit and loss for the current year",
            },
        }
    }
}

/// Typed operations for the 'tradenames' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradenamesOperation {
    /// GET /tradenames
    GetTradenames,
    /// GET /v1/tradenames
    GetV1Tradenames,
}

impl TypedOperation for TradenamesOperation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetTradenames => OperationSpec {
                tag: "tradenames",
                operation_id: "List tradenames",
                method: HttpMethod::Get,
                path: "/tradenames",
                summary: "Returns a list of tradenames",
            },
            Self::GetV1Tradenames => OperationSpec {
                tag: "tradenames",
                operation_id: "List tradenames",
                method: HttpMethod::Get,
                path: "/v1/tradenames",
                summary: "Returns a list of tradenames",
            },
        }
    }
}

/// Typed operations for the 'v2' domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum V2Operation {
    /// GET /v2/estimates
    GetV2Estimates,
    /// GET /v2/estimates/{id}
    GetV2EstimatesById,
    /// GET /v2/estimates/{id}/send_settings
    GetV2EstimatesByIdSendSettings,
    /// GET /v2/invoices
    GetV2Invoices,
    /// GET /v2/invoices/{id}
    GetV2InvoicesById,
}

impl TypedOperation for V2Operation {
    fn spec(self) -> OperationSpec {
        match self {
            Self::GetV2Estimates => OperationSpec {
                tag: "v2",
                operation_id: "List Estimates v2",
                method: HttpMethod::Get,
                path: "/v2/estimates",
                summary: "Returns a list of Estimates",
            },
            Self::GetV2EstimatesById => OperationSpec {
                tag: "v2",
                operation_id: "Get Estimate by ID v2",
                method: HttpMethod::Get,
                path: "/v2/estimates/{id}",
                summary: "",
            },
            Self::GetV2EstimatesByIdSendSettings => OperationSpec {
                tag: "v2",
                operation_id: "Get estimate send settings v2",
                method: HttpMethod::Get,
                path: "/v2/estimates/{id}/send_settings",
                summary: "",
            },
            Self::GetV2Invoices => OperationSpec {
                tag: "v2",
                operation_id: "List Invoices v2",
                method: HttpMethod::Get,
                path: "/v2/invoices",
                summary: "Returns a list of Invoices",
            },
            Self::GetV2InvoicesById => OperationSpec {
                tag: "v2",
                operation_id: "Get Invoice by ID v2",
                method: HttpMethod::Get,
                path: "/v2/invoices/{id}",
                summary: "",
            },
        }
    }
}

/// Canonical catalog of all operation specs in this SDK release.
pub const ALL_OPERATION_SPECS: [OperationSpec; OPERATION_COUNT] = [
    OperationSpec {
        tag: "bank-accounts",
        operation_id: "List Bank Accounts V3",
        method: HttpMethod::Get,
        path: "/v3/bank-accounts",
        summary: "",
    },
    OperationSpec {
        tag: "bank-accounts",
        operation_id: "List Bank Transactions V3",
        method: HttpMethod::Get,
        path: "/v3/bank-accounts/{id}/bank-transactions",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "List Customers",
        method: HttpMethod::Get,
        path: "/customers",
        summary: "Returns a list of Customers",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Create Customer",
        method: HttpMethod::Post,
        path: "/customers",
        summary: "Creates a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Delete a Customer",
        method: HttpMethod::Delete,
        path: "/customers/{customer_id}",
        summary: "Delete a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get Customer by ID",
        method: HttpMethod::Get,
        path: "/customers/{customer_id}",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Update Customer",
        method: HttpMethod::Put,
        path: "/customers/{customer_id}",
        summary: "Updates a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Send direct debit authorization to a Customer",
        method: HttpMethod::Post,
        path: "/customers/{customer_id}/direct_debit_mandate",
        summary: "Send direct debit authorization to a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get Customer's details by ID",
        method: HttpMethod::Get,
        path: "/customers/{customer_id}/extra_details",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Set Customer Archived",
        method: HttpMethod::Put,
        path: "/customers/{customer_id}/set_archived",
        summary: "Sets the archived status for a customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get vat percentages for a Customer by ID",
        method: HttpMethod::Get,
        path: "/customers/{customer_id}/vat-percentages",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "List Customers",
        method: HttpMethod::Get,
        path: "/v1/customers",
        summary: "Returns a list of Customers",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Create Customer",
        method: HttpMethod::Post,
        path: "/v1/customers",
        summary: "Creates a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Delete a Customer",
        method: HttpMethod::Delete,
        path: "/v1/customers/{customer_id}",
        summary: "Delete a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get Customer by ID",
        method: HttpMethod::Get,
        path: "/v1/customers/{customer_id}",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Update Customer",
        method: HttpMethod::Put,
        path: "/v1/customers/{customer_id}",
        summary: "Updates a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Send direct debit authorization to a Customer",
        method: HttpMethod::Post,
        path: "/v1/customers/{customer_id}/direct_debit_mandate",
        summary: "Send direct debit authorization to a Customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get Customer's details by ID",
        method: HttpMethod::Get,
        path: "/v1/customers/{customer_id}/extra_details",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Set Customer Archived",
        method: HttpMethod::Put,
        path: "/v1/customers/{customer_id}/set_archived",
        summary: "Sets the archived status for a customer",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get vat percentages for a Customer by ID",
        method: HttpMethod::Get,
        path: "/v1/customers/{customer_id}/vat-percentages",
        summary: "",
    },
    OperationSpec {
        tag: "customers",
        operation_id: "Get vat percentages for a Customer by ID V2",
        method: HttpMethod::Get,
        path: "/v2/customers/{customer_id}/vat-percentages",
        summary: "",
    },
    OperationSpec {
        tag: "estimates",
        operation_id: "Create (and optionally send) an Estimate V2",
        method: HttpMethod::Post,
        path: "/v2/estimates",
        summary: "Creates (and optionally sends) an Estimate",
    },
    OperationSpec {
        tag: "estimates",
        operation_id: "Edit Estimate V2",
        method: HttpMethod::Put,
        path: "/v2/estimates/{id}",
        summary: "Edits an Estimate",
    },
    OperationSpec {
        tag: "estimates",
        operation_id: "Send Estimate V2",
        method: HttpMethod::Post,
        path: "/v2/estimates/{id}/send",
        summary: "Sends an Estimate",
    },
    OperationSpec {
        tag: "expenses",
        operation_id: "List Expenses",
        method: HttpMethod::Get,
        path: "/v3/expenses",
        summary: "List Expenses",
    },
    OperationSpec {
        tag: "expenses",
        operation_id: "Create an Expense",
        method: HttpMethod::Post,
        path: "/v3/expenses",
        summary: "Create an Expense",
    },
    OperationSpec {
        tag: "expenses",
        operation_id: "Get Expense by ID",
        method: HttpMethod::Get,
        path: "/v3/expenses/id/{id}",
        summary: "Get Expense by ID",
    },
    OperationSpec {
        tag: "expenses",
        operation_id: "Update an Expense",
        method: HttpMethod::Post,
        path: "/v3/expenses/id/{id}",
        summary: "Update an Expense",
    },
    OperationSpec {
        tag: "expenses",
        operation_id: "Attach Receipt to Expense",
        method: HttpMethod::Post,
        path: "/v3/expenses/id/{id}/receipt",
        summary: "Attach a Receipt to an Expense",
    },
    OperationSpec {
        tag: "files",
        operation_id: "Upload URL for attachments",
        method: HttpMethod::Get,
        path: "/files/put_url",
        summary: "Request an upload URL for attachments",
    },
    OperationSpec {
        tag: "files",
        operation_id: "Upload URL for attachments",
        method: HttpMethod::Get,
        path: "/v1/files/put_url",
        summary: "Request an upload URL for attachments",
    },
    OperationSpec {
        tag: "inbox",
        operation_id: "Upload receipt",
        method: HttpMethod::Post,
        path: "/inbox/images",
        summary: "Upload images to jortt",
    },
    OperationSpec {
        tag: "inbox",
        operation_id: "Upload receipt",
        method: HttpMethod::Post,
        path: "/v1/inbox/images",
        summary: "Upload images to jortt",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "List Invoices",
        method: HttpMethod::Get,
        path: "/invoices",
        summary: "Returns a list of Invoices",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Create (and optionally send) an Invoice",
        method: HttpMethod::Post,
        path: "/invoices",
        summary: "Creates (and optionally sends) an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Delete Invoice by ID",
        method: HttpMethod::Delete,
        path: "/invoices/{id}",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get Invoice by ID",
        method: HttpMethod::Get,
        path: "/invoices/{id}",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Edit Invoice",
        method: HttpMethod::Put,
        path: "/invoices/{id}",
        summary: "Edits an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Copy an Invoice",
        method: HttpMethod::Post,
        path: "/invoices/{id}/copy",
        summary: "Copies an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Creates (and optionally sends) a credit Invoice",
        method: HttpMethod::Post,
        path: "/invoices/{id}/credit",
        summary: "Creates (and optionally sends) a credit Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Download Invoice PDF",
        method: HttpMethod::Get,
        path: "/invoices/{id}/download",
        summary: "Returns a URL from which the invoice PDF can be downloaded.",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get line item suggestions",
        method: HttpMethod::Get,
        path: "/invoices/{id}/line_item_suggestions",
        summary: "Returns a list of suggested line items",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get next possible invoice number",
        method: HttpMethod::Get,
        path: "/invoices/{id}/next_possible_invoice_number",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Send Invoice",
        method: HttpMethod::Post,
        path: "/invoices/{id}/send",
        summary: "Sends an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get send settings",
        method: HttpMethod::Get,
        path: "/invoices/{id}/send_settings",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Set invoice labels",
        method: HttpMethod::Put,
        path: "/invoices/{id}/set_labels",
        summary: "Sets the labels for a given invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "List Invoices",
        method: HttpMethod::Get,
        path: "/v1/invoices",
        summary: "Returns a list of Invoices",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Create (and optionally send) an Invoice",
        method: HttpMethod::Post,
        path: "/v1/invoices",
        summary: "Creates (and optionally sends) an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Delete Invoice by ID",
        method: HttpMethod::Delete,
        path: "/v1/invoices/{id}",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get Invoice by ID",
        method: HttpMethod::Get,
        path: "/v1/invoices/{id}",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Edit Invoice",
        method: HttpMethod::Put,
        path: "/v1/invoices/{id}",
        summary: "Edits an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Copy an Invoice",
        method: HttpMethod::Post,
        path: "/v1/invoices/{id}/copy",
        summary: "Copies an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Creates (and optionally sends) a credit Invoice",
        method: HttpMethod::Post,
        path: "/v1/invoices/{id}/credit",
        summary: "Creates (and optionally sends) a credit Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Download Invoice PDF",
        method: HttpMethod::Get,
        path: "/v1/invoices/{id}/download",
        summary: "Returns a URL from which the invoice PDF can be downloaded.",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get line item suggestions",
        method: HttpMethod::Get,
        path: "/v1/invoices/{id}/line_item_suggestions",
        summary: "Returns a list of suggested line items",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get next possible invoice number",
        method: HttpMethod::Get,
        path: "/v1/invoices/{id}/next_possible_invoice_number",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Send Invoice",
        method: HttpMethod::Post,
        path: "/v1/invoices/{id}/send",
        summary: "Sends an Invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get send settings",
        method: HttpMethod::Get,
        path: "/v1/invoices/{id}/send_settings",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Set invoice labels",
        method: HttpMethod::Put,
        path: "/v1/invoices/{id}/set_labels",
        summary: "Sets the labels for a given invoice",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Create (and optionally send) an Invoice V2",
        method: HttpMethod::Post,
        path: "/v2/invoices",
        summary: "Creates (and optionally sends) an Invoice V2",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Edit Invoice V2",
        method: HttpMethod::Put,
        path: "/v2/invoices/{id}",
        summary: "Edits an Invoice V2",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get line item suggestions",
        method: HttpMethod::Get,
        path: "/v2/invoices/{id}/line_item_suggestions",
        summary: "Returns a list of suggested line items",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Get peppol scheme catalog",
        method: HttpMethod::Get,
        path: "/v2/invoices/peppol-scheme-catalog",
        summary: "",
    },
    OperationSpec {
        tag: "invoices",
        operation_id: "Create (and optionally send) an Invoice V3",
        method: HttpMethod::Post,
        path: "/v3/invoices",
        summary: "Creates (and optionally sends) an Invoice V3 with invoice-level VAT",
    },
    OperationSpec {
        tag: "labels",
        operation_id: "List labels",
        method: HttpMethod::Get,
        path: "/labels",
        summary: "Returns a list of labels",
    },
    OperationSpec {
        tag: "labels",
        operation_id: "Create a Label",
        method: HttpMethod::Post,
        path: "/labels",
        summary: "Create a label",
    },
    OperationSpec {
        tag: "labels",
        operation_id: "List labels",
        method: HttpMethod::Get,
        path: "/v1/labels",
        summary: "Returns a list of labels",
    },
    OperationSpec {
        tag: "labels",
        operation_id: "Create a Label",
        method: HttpMethod::Post,
        path: "/v1/labels",
        summary: "Create a label",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "List Invoice Ledger Accounts",
        method: HttpMethod::Get,
        path: "/ledger_accounts/invoices",
        summary: "",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "The default Ledger Account for invoice line items",
        method: HttpMethod::Get,
        path: "/ledger_accounts/invoices/default",
        summary: "",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "List Invoice Ledger Accounts",
        method: HttpMethod::Get,
        path: "/v1/ledger_accounts/invoices",
        summary: "",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "The default Ledger Account for invoice line items",
        method: HttpMethod::Get,
        path: "/v1/ledger_accounts/invoices/default",
        summary: "",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "getV3LedgerAccountsExpensesBalance",
        method: HttpMethod::Get,
        path: "/v3/ledger_accounts/expenses/balance",
        summary: "",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "getV3LedgerAccountsExpensesCost",
        method: HttpMethod::Get,
        path: "/v3/ledger_accounts/expenses/cost",
        summary: "",
    },
    OperationSpec {
        tag: "ledger_accounts",
        operation_id: "getV3LedgerAccountsExpensesIncome",
        method: HttpMethod::Get,
        path: "/v3/ledger_accounts/expenses/income",
        summary: "",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "List Loonjournaalposten",
        method: HttpMethod::Get,
        path: "/loonjournaalposten",
        summary: "Returns a list of Loonjournaalposten",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "Create a Loonjournaalpost",
        method: HttpMethod::Post,
        path: "/loonjournaalposten",
        summary: "Create a Loonjournaalpost",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "Delete a Loonjournaalpost",
        method: HttpMethod::Delete,
        path: "/loonjournaalposten/{loonjournaalpost_id}",
        summary: "Delete a Loonjournaalpost",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "Update a Loonjournaalpost",
        method: HttpMethod::Put,
        path: "/loonjournaalposten/{loonjournaalpost_id}",
        summary: "Update a Loonjournaalpost",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "List Loonjournaalposten",
        method: HttpMethod::Get,
        path: "/v1/loonjournaalposten",
        summary: "Returns a list of Loonjournaalposten",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "Create a Loonjournaalpost",
        method: HttpMethod::Post,
        path: "/v1/loonjournaalposten",
        summary: "Create a Loonjournaalpost",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "Delete a Loonjournaalpost",
        method: HttpMethod::Delete,
        path: "/v1/loonjournaalposten/{loonjournaalpost_id}",
        summary: "Delete a Loonjournaalpost",
    },
    OperationSpec {
        tag: "loonjournaalposten",
        operation_id: "Update a Loonjournaalpost",
        method: HttpMethod::Put,
        path: "/v1/loonjournaalposten/{loonjournaalpost_id}",
        summary: "Update a Loonjournaalpost",
    },
    OperationSpec {
        tag: "organizations",
        operation_id: "Get the organization associated with the api credentials",
        method: HttpMethod::Get,
        path: "/organizations/me",
        summary: "",
    },
    OperationSpec {
        tag: "organizations",
        operation_id: "Get the organization associated with the api credentials",
        method: HttpMethod::Get,
        path: "/v1/organizations/me",
        summary: "",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "List Projects",
        method: HttpMethod::Get,
        path: "/projects",
        summary: "Returns a list of Projects",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Create Project",
        method: HttpMethod::Post,
        path: "/projects",
        summary: "Creates a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Delete Project",
        method: HttpMethod::Delete,
        path: "/projects/{id}",
        summary: "Deletes a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Get Project",
        method: HttpMethod::Get,
        path: "/projects/{id}",
        summary: "Returns a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Update Project",
        method: HttpMethod::Put,
        path: "/projects/{id}",
        summary: "Updates a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Invoice Project",
        method: HttpMethod::Post,
        path: "/projects/{id}/invoice",
        summary: "Creates an invoice based on project line items",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "List Project Line Items",
        method: HttpMethod::Get,
        path: "/projects/{id}/line_items",
        summary: "Returns a list of Project Line Items",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Create Project Line item",
        method: HttpMethod::Post,
        path: "/projects/{id}/line_items",
        summary: "Creates a Project Line Item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Delete Project Line item",
        method: HttpMethod::Delete,
        path: "/projects/{id}/line_items/{line_item_id}",
        summary: "Deletes a Project Line item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Get Project Line Item",
        method: HttpMethod::Get,
        path: "/projects/{id}/line_items/{line_item_id}",
        summary: "Returns a Project Line Item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Update Project Line item",
        method: HttpMethod::Put,
        path: "/projects/{id}/line_items/{line_item_id}",
        summary: "Updates a Project Line item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Get Project Line Item Summary",
        method: HttpMethod::Get,
        path: "/projects/{id}/line_items/summary",
        summary: "Returns a list of Project Line Item summaries",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "List Projects",
        method: HttpMethod::Get,
        path: "/v1/projects",
        summary: "Returns a list of Projects",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Create Project",
        method: HttpMethod::Post,
        path: "/v1/projects",
        summary: "Creates a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Delete Project",
        method: HttpMethod::Delete,
        path: "/v1/projects/{id}",
        summary: "Deletes a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Get Project",
        method: HttpMethod::Get,
        path: "/v1/projects/{id}",
        summary: "Returns a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Update Project",
        method: HttpMethod::Put,
        path: "/v1/projects/{id}",
        summary: "Updates a Project",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Invoice Project",
        method: HttpMethod::Post,
        path: "/v1/projects/{id}/invoice",
        summary: "Creates an invoice based on project line items",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "List Project Line Items",
        method: HttpMethod::Get,
        path: "/v1/projects/{id}/line_items",
        summary: "Returns a list of Project Line Items",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Create Project Line item",
        method: HttpMethod::Post,
        path: "/v1/projects/{id}/line_items",
        summary: "Creates a Project Line Item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Delete Project Line item",
        method: HttpMethod::Delete,
        path: "/v1/projects/{id}/line_items/{line_item_id}",
        summary: "Deletes a Project Line item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Get Project Line Item",
        method: HttpMethod::Get,
        path: "/v1/projects/{id}/line_items/{line_item_id}",
        summary: "Returns a Project Line Item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Update Project Line item",
        method: HttpMethod::Put,
        path: "/v1/projects/{id}/line_items/{line_item_id}",
        summary: "Updates a Project Line item",
    },
    OperationSpec {
        tag: "projects",
        operation_id: "Get Project Line Item Summary",
        method: HttpMethod::Get,
        path: "/v1/projects/{id}/line_items/summary",
        summary: "Returns a list of Project Line Item summaries",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard balance",
        method: HttpMethod::Get,
        path: "/reports/summaries/balance",
        summary: "Returns key organization balances for the current date",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard btw",
        method: HttpMethod::Get,
        path: "/reports/summaries/btw",
        summary: "Returns a list of summarized btw periods",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard cash and bank",
        method: HttpMethod::Get,
        path: "/reports/summaries/cash_and_bank",
        summary: "Returns a summary of bank accounts, cash and liquid assets",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard invoices",
        method: HttpMethod::Get,
        path: "/reports/summaries/invoices",
        summary: "Returns a summary of invoices for the current year",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard profit and loss",
        method: HttpMethod::Get,
        path: "/reports/summaries/profit_and_loss",
        summary: "Returns a summary of profit and loss for the current year",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard balance",
        method: HttpMethod::Get,
        path: "/v1/reports/summaries/balance",
        summary: "Returns key organization balances for the current date",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard btw",
        method: HttpMethod::Get,
        path: "/v1/reports/summaries/btw",
        summary: "Returns a list of summarized btw periods",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard cash and bank",
        method: HttpMethod::Get,
        path: "/v1/reports/summaries/cash_and_bank",
        summary: "Returns a summary of bank accounts, cash and liquid assets",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard invoices",
        method: HttpMethod::Get,
        path: "/v1/reports/summaries/invoices",
        summary: "Returns a summary of invoices for the current year",
    },
    OperationSpec {
        tag: "reports",
        operation_id: "Dashboard profit and loss",
        method: HttpMethod::Get,
        path: "/v1/reports/summaries/profit_and_loss",
        summary: "Returns a summary of profit and loss for the current year",
    },
    OperationSpec {
        tag: "tradenames",
        operation_id: "List tradenames",
        method: HttpMethod::Get,
        path: "/tradenames",
        summary: "Returns a list of tradenames",
    },
    OperationSpec {
        tag: "tradenames",
        operation_id: "List tradenames",
        method: HttpMethod::Get,
        path: "/v1/tradenames",
        summary: "Returns a list of tradenames",
    },
    OperationSpec {
        tag: "v2",
        operation_id: "List Estimates v2",
        method: HttpMethod::Get,
        path: "/v2/estimates",
        summary: "Returns a list of Estimates",
    },
    OperationSpec {
        tag: "v2",
        operation_id: "Get Estimate by ID v2",
        method: HttpMethod::Get,
        path: "/v2/estimates/{id}",
        summary: "",
    },
    OperationSpec {
        tag: "v2",
        operation_id: "Get estimate send settings v2",
        method: HttpMethod::Get,
        path: "/v2/estimates/{id}/send_settings",
        summary: "",
    },
    OperationSpec {
        tag: "v2",
        operation_id: "List Invoices v2",
        method: HttpMethod::Get,
        path: "/v2/invoices",
        summary: "Returns a list of Invoices",
    },
    OperationSpec {
        tag: "v2",
        operation_id: "Get Invoice by ID v2",
        method: HttpMethod::Get,
        path: "/v2/invoices/{id}",
        summary: "",
    },
];

/// Number of operations extracted from the pinned OpenAPI snapshot (2026-05-05).
pub const OPERATION_COUNT: usize = 126;
