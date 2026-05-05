// @generated from docs/openapi/operations-2026-05-05.json
// Do not edit manually.

use serde_json::Value;

use crate::api::OperationRequest;
use crate::api::operations::{
    BankAccountsOperation, CustomersOperation, EstimatesOperation, ExpensesOperation,
    FilesOperation, InboxOperation, InvoicesOperation, LabelsOperation, LedgerAccountsOperation,
    LoonjournaalpostenOperation, OrganizationsOperation, ProjectsOperation, ReportsOperation,
    TradenamesOperation, TypedOperation, V2Operation,
};
use crate::client::JorttClient;
use crate::error::JorttError;

/// Per-operation method surface grouped by OpenAPI tag.
#[derive(Clone)]
pub struct ApiMethods {
    client: JorttClient,
}

impl ApiMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// Returns the method group for `bank-accounts` operations.
    pub fn bank_accounts(&self) -> BankAccountsMethods {
        BankAccountsMethods::new(self.client.clone())
    }

    /// Returns the method group for `customers` operations.
    pub fn customers(&self) -> CustomersMethods {
        CustomersMethods::new(self.client.clone())
    }

    /// Returns the method group for `estimates` operations.
    pub fn estimates(&self) -> EstimatesMethods {
        EstimatesMethods::new(self.client.clone())
    }

    /// Returns the method group for `expenses` operations.
    pub fn expenses(&self) -> ExpensesMethods {
        ExpensesMethods::new(self.client.clone())
    }

    /// Returns the method group for `files` operations.
    pub fn files(&self) -> FilesMethods {
        FilesMethods::new(self.client.clone())
    }

    /// Returns the method group for `inbox` operations.
    pub fn inbox(&self) -> InboxMethods {
        InboxMethods::new(self.client.clone())
    }

    /// Returns the method group for `invoices` operations.
    pub fn invoices(&self) -> InvoicesMethods {
        InvoicesMethods::new(self.client.clone())
    }

    /// Returns the method group for `labels` operations.
    pub fn labels(&self) -> LabelsMethods {
        LabelsMethods::new(self.client.clone())
    }

    /// Returns the method group for `ledger_accounts` operations.
    pub fn ledger_accounts(&self) -> LedgerAccountsMethods {
        LedgerAccountsMethods::new(self.client.clone())
    }

    /// Returns the method group for `loonjournaalposten` operations.
    pub fn loonjournaalposten(&self) -> LoonjournaalpostenMethods {
        LoonjournaalpostenMethods::new(self.client.clone())
    }

    /// Returns the method group for `organizations` operations.
    pub fn organizations(&self) -> OrganizationsMethods {
        OrganizationsMethods::new(self.client.clone())
    }

    /// Returns the method group for `projects` operations.
    pub fn projects(&self) -> ProjectsMethods {
        ProjectsMethods::new(self.client.clone())
    }

    /// Returns the method group for `reports` operations.
    pub fn reports(&self) -> ReportsMethods {
        ReportsMethods::new(self.client.clone())
    }

    /// Returns the method group for `tradenames` operations.
    pub fn tradenames(&self) -> TradenamesMethods {
        TradenamesMethods::new(self.client.clone())
    }

    /// Returns the method group for `v2` operations.
    pub fn v2(&self) -> V2Methods {
        V2Methods::new(self.client.clone())
    }
}

/// Operation methods for the `bank-accounts` tag.
#[derive(Clone)]
pub struct BankAccountsMethods {
    client: JorttClient,
}

impl BankAccountsMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /v3/bank-accounts` (`List Bank Accounts V3`).
    pub async fn get_v3_bank_accounts(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(BankAccountsOperation::GetV3BankAccounts.spec(), request)
            .await
    }

    /// `GET /v3/bank-accounts/{id}/bank-transactions` (`List Bank Transactions V3`).
    pub async fn get_v3_bank_accounts_by_id_bank_transactions(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                BankAccountsOperation::GetV3BankAccountsByIdBankTransactions.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `customers` tag.
#[derive(Clone)]
pub struct CustomersMethods {
    client: JorttClient,
}

impl CustomersMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /customers` (`List Customers`).
    pub async fn get_customers(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(CustomersOperation::GetCustomers.spec(), request)
            .await
    }

    /// `POST /customers` (`Create Customer`).
    pub async fn post_customers(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(CustomersOperation::PostCustomers.spec(), request)
            .await
    }

    /// `DELETE /customers/{customer_id}` (`Delete a Customer`).
    pub async fn delete_customers_by_customer_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::DeleteCustomersByCustomerId.spec(),
                request,
            )
            .await
    }

    /// `GET /customers/{customer_id}` (`Get Customer by ID`).
    pub async fn get_customers_by_customer_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(CustomersOperation::GetCustomersByCustomerId.spec(), request)
            .await
    }

    /// `PUT /customers/{customer_id}` (`Update Customer`).
    pub async fn put_customers_by_customer_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(CustomersOperation::PutCustomersByCustomerId.spec(), request)
            .await
    }

    /// `POST /customers/{customer_id}/direct_debit_mandate` (`Send direct debit authorization to a Customer`).
    pub async fn post_customers_by_customer_id_direct_debit_mandate(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::PostCustomersByCustomerIdDirectDebitMandate.spec(),
                request,
            )
            .await
    }

    /// `GET /customers/{customer_id}/extra_details` (`Get Customer's details by ID`).
    pub async fn get_customers_by_customer_id_extra_details(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::GetCustomersByCustomerIdExtraDetails.spec(),
                request,
            )
            .await
    }

    /// `PUT /customers/{customer_id}/set_archived` (`Set Customer Archived`).
    pub async fn put_customers_by_customer_id_set_archived(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::PutCustomersByCustomerIdSetArchived.spec(),
                request,
            )
            .await
    }

    /// `GET /customers/{customer_id}/vat-percentages` (`Get vat percentages for a Customer by ID`).
    pub async fn get_customers_by_customer_id_vat_percentages(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::GetCustomersByCustomerIdVatPercentages.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/customers` (`List Customers`).
    pub async fn get_v1_customers(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(CustomersOperation::GetV1Customers.spec(), request)
            .await
    }

    /// `POST /v1/customers` (`Create Customer`).
    pub async fn post_v1_customers(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(CustomersOperation::PostV1Customers.spec(), request)
            .await
    }

    /// `DELETE /v1/customers/{customer_id}` (`Delete a Customer`).
    pub async fn delete_v1_customers_by_customer_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::DeleteV1CustomersByCustomerId.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/customers/{customer_id}` (`Get Customer by ID`).
    pub async fn get_v1_customers_by_customer_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::GetV1CustomersByCustomerId.spec(),
                request,
            )
            .await
    }

    /// `PUT /v1/customers/{customer_id}` (`Update Customer`).
    pub async fn put_v1_customers_by_customer_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::PutV1CustomersByCustomerId.spec(),
                request,
            )
            .await
    }

    /// `POST /v1/customers/{customer_id}/direct_debit_mandate` (`Send direct debit authorization to a Customer`).
    pub async fn post_v1_customers_by_customer_id_direct_debit_mandate(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::PostV1CustomersByCustomerIdDirectDebitMandate.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/customers/{customer_id}/extra_details` (`Get Customer's details by ID`).
    pub async fn get_v1_customers_by_customer_id_extra_details(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::GetV1CustomersByCustomerIdExtraDetails.spec(),
                request,
            )
            .await
    }

    /// `PUT /v1/customers/{customer_id}/set_archived` (`Set Customer Archived`).
    pub async fn put_v1_customers_by_customer_id_set_archived(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::PutV1CustomersByCustomerIdSetArchived.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/customers/{customer_id}/vat-percentages` (`Get vat percentages for a Customer by ID`).
    pub async fn get_v1_customers_by_customer_id_vat_percentages(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::GetV1CustomersByCustomerIdVatPercentages.spec(),
                request,
            )
            .await
    }

    /// `GET /v2/customers/{customer_id}/vat-percentages` (`Get vat percentages for a Customer by ID V2`).
    pub async fn get_v2_customers_by_customer_id_vat_percentages(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                CustomersOperation::GetV2CustomersByCustomerIdVatPercentages.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `estimates` tag.
#[derive(Clone)]
pub struct EstimatesMethods {
    client: JorttClient,
}

impl EstimatesMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `POST /v2/estimates` (`Create (and optionally send) an Estimate V2`).
    pub async fn post_v2_estimates(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(EstimatesOperation::PostV2Estimates.spec(), request)
            .await
    }

    /// `PUT /v2/estimates/{id}` (`Edit Estimate V2`).
    pub async fn put_v2_estimates_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(EstimatesOperation::PutV2EstimatesById.spec(), request)
            .await
    }

    /// `POST /v2/estimates/{id}/send` (`Send Estimate V2`).
    pub async fn post_v2_estimates_by_id_send(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(EstimatesOperation::PostV2EstimatesByIdSend.spec(), request)
            .await
    }
}

/// Operation methods for the `expenses` tag.
#[derive(Clone)]
pub struct ExpensesMethods {
    client: JorttClient,
}

impl ExpensesMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /v3/expenses` (`List Expenses`).
    pub async fn get_v3_expenses(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ExpensesOperation::GetV3Expenses.spec(), request)
            .await
    }

    /// `POST /v3/expenses` (`Create an Expense`).
    pub async fn post_v3_expenses(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ExpensesOperation::PostV3Expenses.spec(), request)
            .await
    }

    /// `GET /v3/expenses/id/{id}` (`Get Expense by ID`).
    pub async fn get_v3_expenses_id_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ExpensesOperation::GetV3ExpensesIdById.spec(), request)
            .await
    }

    /// `POST /v3/expenses/id/{id}` (`Update an Expense`).
    pub async fn post_v3_expenses_id_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ExpensesOperation::PostV3ExpensesIdById.spec(), request)
            .await
    }

    /// `POST /v3/expenses/id/{id}/receipt` (`Attach Receipt to Expense`).
    pub async fn post_v3_expenses_id_by_id_receipt(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ExpensesOperation::PostV3ExpensesIdByIdReceipt.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `files` tag.
#[derive(Clone)]
pub struct FilesMethods {
    client: JorttClient,
}

impl FilesMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /files/put_url` (`Upload URL for attachments`).
    pub async fn get_files_put_url(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(FilesOperation::GetFilesPutUrl.spec(), request)
            .await
    }

    /// `GET /v1/files/put_url` (`Upload URL for attachments`).
    pub async fn get_v1_files_put_url(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(FilesOperation::GetV1FilesPutUrl.spec(), request)
            .await
    }
}

/// Operation methods for the `inbox` tag.
#[derive(Clone)]
pub struct InboxMethods {
    client: JorttClient,
}

impl InboxMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `POST /inbox/images` (`Upload receipt`).
    pub async fn post_inbox_images(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InboxOperation::PostInboxImages.spec(), request)
            .await
    }

    /// `POST /v1/inbox/images` (`Upload receipt`).
    pub async fn post_v1_inbox_images(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InboxOperation::PostV1InboxImages.spec(), request)
            .await
    }
}

/// Operation methods for the `invoices` tag.
#[derive(Clone)]
pub struct InvoicesMethods {
    client: JorttClient,
}

impl InvoicesMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /invoices` (`List Invoices`).
    pub async fn get_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::GetInvoices.spec(), request)
            .await
    }

    /// `POST /invoices` (`Create (and optionally send) an Invoice`).
    pub async fn post_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostInvoices.spec(), request)
            .await
    }

    /// `DELETE /invoices/{id}` (`Delete Invoice by ID`).
    pub async fn delete_invoices_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::DeleteInvoicesById.spec(), request)
            .await
    }

    /// `GET /invoices/{id}` (`Get Invoice by ID`).
    pub async fn get_invoices_by_id(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::GetInvoicesById.spec(), request)
            .await
    }

    /// `PUT /invoices/{id}` (`Edit Invoice`).
    pub async fn put_invoices_by_id(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PutInvoicesById.spec(), request)
            .await
    }

    /// `POST /invoices/{id}/copy` (`Copy an Invoice`).
    pub async fn post_invoices_by_id_copy(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostInvoicesByIdCopy.spec(), request)
            .await
    }

    /// `POST /invoices/{id}/credit` (`Creates (and optionally sends) a credit Invoice`).
    pub async fn post_invoices_by_id_credit(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostInvoicesByIdCredit.spec(), request)
            .await
    }

    /// `GET /invoices/{id}/download` (`Download Invoice PDF`).
    pub async fn get_invoices_by_id_download(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::GetInvoicesByIdDownload.spec(), request)
            .await
    }

    /// `GET /invoices/{id}/line_item_suggestions` (`Get line item suggestions`).
    pub async fn get_invoices_by_id_line_item_suggestions(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetInvoicesByIdLineItemSuggestions.spec(),
                request,
            )
            .await
    }

    /// `GET /invoices/{id}/next_possible_invoice_number` (`Get next possible invoice number`).
    pub async fn get_invoices_by_id_next_possible_invoice_number(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetInvoicesByIdNextPossibleInvoiceNumber.spec(),
                request,
            )
            .await
    }

    /// `POST /invoices/{id}/send` (`Send Invoice`).
    pub async fn post_invoices_by_id_send(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostInvoicesByIdSend.spec(), request)
            .await
    }

    /// `GET /invoices/{id}/send_settings` (`Get send settings`).
    pub async fn get_invoices_by_id_send_settings(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetInvoicesByIdSendSettings.spec(),
                request,
            )
            .await
    }

    /// `PUT /invoices/{id}/set_labels` (`Set invoice labels`).
    pub async fn put_invoices_by_id_set_labels(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PutInvoicesByIdSetLabels.spec(), request)
            .await
    }

    /// `GET /v1/invoices` (`List Invoices`).
    pub async fn get_v1_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::GetV1Invoices.spec(), request)
            .await
    }

    /// `POST /v1/invoices` (`Create (and optionally send) an Invoice`).
    pub async fn post_v1_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostV1Invoices.spec(), request)
            .await
    }

    /// `DELETE /v1/invoices/{id}` (`Delete Invoice by ID`).
    pub async fn delete_v1_invoices_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::DeleteV1InvoicesById.spec(), request)
            .await
    }

    /// `GET /v1/invoices/{id}` (`Get Invoice by ID`).
    pub async fn get_v1_invoices_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::GetV1InvoicesById.spec(), request)
            .await
    }

    /// `PUT /v1/invoices/{id}` (`Edit Invoice`).
    pub async fn put_v1_invoices_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PutV1InvoicesById.spec(), request)
            .await
    }

    /// `POST /v1/invoices/{id}/copy` (`Copy an Invoice`).
    pub async fn post_v1_invoices_by_id_copy(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostV1InvoicesByIdCopy.spec(), request)
            .await
    }

    /// `POST /v1/invoices/{id}/credit` (`Creates (and optionally sends) a credit Invoice`).
    pub async fn post_v1_invoices_by_id_credit(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostV1InvoicesByIdCredit.spec(), request)
            .await
    }

    /// `GET /v1/invoices/{id}/download` (`Download Invoice PDF`).
    pub async fn get_v1_invoices_by_id_download(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::GetV1InvoicesByIdDownload.spec(), request)
            .await
    }

    /// `GET /v1/invoices/{id}/line_item_suggestions` (`Get line item suggestions`).
    pub async fn get_v1_invoices_by_id_line_item_suggestions(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetV1InvoicesByIdLineItemSuggestions.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/invoices/{id}/next_possible_invoice_number` (`Get next possible invoice number`).
    pub async fn get_v1_invoices_by_id_next_possible_invoice_number(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetV1InvoicesByIdNextPossibleInvoiceNumber.spec(),
                request,
            )
            .await
    }

    /// `POST /v1/invoices/{id}/send` (`Send Invoice`).
    pub async fn post_v1_invoices_by_id_send(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostV1InvoicesByIdSend.spec(), request)
            .await
    }

    /// `GET /v1/invoices/{id}/send_settings` (`Get send settings`).
    pub async fn get_v1_invoices_by_id_send_settings(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetV1InvoicesByIdSendSettings.spec(),
                request,
            )
            .await
    }

    /// `PUT /v1/invoices/{id}/set_labels` (`Set invoice labels`).
    pub async fn put_v1_invoices_by_id_set_labels(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::PutV1InvoicesByIdSetLabels.spec(),
                request,
            )
            .await
    }

    /// `POST /v2/invoices` (`Create (and optionally send) an Invoice V2`).
    pub async fn post_v2_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostV2Invoices.spec(), request)
            .await
    }

    /// `PUT /v2/invoices/{id}` (`Edit Invoice V2`).
    pub async fn put_v2_invoices_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PutV2InvoicesById.spec(), request)
            .await
    }

    /// `GET /v2/invoices/{id}/line_item_suggestions` (`Get line item suggestions`).
    pub async fn get_v2_invoices_by_id_line_item_suggestions(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetV2InvoicesByIdLineItemSuggestions.spec(),
                request,
            )
            .await
    }

    /// `GET /v2/invoices/peppol-scheme-catalog` (`Get peppol scheme catalog`).
    pub async fn get_v2_invoices_peppol_scheme_catalog(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                InvoicesOperation::GetV2InvoicesPeppolSchemeCatalog.spec(),
                request,
            )
            .await
    }

    /// `POST /v3/invoices` (`Create (and optionally send) an Invoice V3`).
    pub async fn post_v3_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(InvoicesOperation::PostV3Invoices.spec(), request)
            .await
    }
}

/// Operation methods for the `labels` tag.
#[derive(Clone)]
pub struct LabelsMethods {
    client: JorttClient,
}

impl LabelsMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /labels` (`List labels`).
    pub async fn get_labels(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(LabelsOperation::GetLabels.spec(), request)
            .await
    }

    /// `POST /labels` (`Create a Label`).
    pub async fn post_labels(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(LabelsOperation::PostLabels.spec(), request)
            .await
    }

    /// `GET /v1/labels` (`List labels`).
    pub async fn get_v1_labels(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(LabelsOperation::GetV1Labels.spec(), request)
            .await
    }

    /// `POST /v1/labels` (`Create a Label`).
    pub async fn post_v1_labels(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(LabelsOperation::PostV1Labels.spec(), request)
            .await
    }
}

/// Operation methods for the `ledger_accounts` tag.
#[derive(Clone)]
pub struct LedgerAccountsMethods {
    client: JorttClient,
}

impl LedgerAccountsMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /ledger_accounts/invoices` (`List Invoice Ledger Accounts`).
    pub async fn get_ledger_accounts_invoices(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetLedgerAccountsInvoices.spec(),
                request,
            )
            .await
    }

    /// `GET /ledger_accounts/invoices/default` (`The default Ledger Account for invoice line items`).
    pub async fn get_ledger_accounts_invoices_default(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetLedgerAccountsInvoicesDefault.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/ledger_accounts/invoices` (`List Invoice Ledger Accounts`).
    pub async fn get_v1_ledger_accounts_invoices(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetV1LedgerAccountsInvoices.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/ledger_accounts/invoices/default` (`The default Ledger Account for invoice line items`).
    pub async fn get_v1_ledger_accounts_invoices_default(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetV1LedgerAccountsInvoicesDefault.spec(),
                request,
            )
            .await
    }

    /// `GET /v3/ledger_accounts/expenses/balance` (`getV3LedgerAccountsExpensesBalance`).
    pub async fn get_v3_ledger_accounts_expenses_balance(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetV3LedgerAccountsExpensesBalance.spec(),
                request,
            )
            .await
    }

    /// `GET /v3/ledger_accounts/expenses/cost` (`getV3LedgerAccountsExpensesCost`).
    pub async fn get_v3_ledger_accounts_expenses_cost(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetV3LedgerAccountsExpensesCost.spec(),
                request,
            )
            .await
    }

    /// `GET /v3/ledger_accounts/expenses/income` (`getV3LedgerAccountsExpensesIncome`).
    pub async fn get_v3_ledger_accounts_expenses_income(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LedgerAccountsOperation::GetV3LedgerAccountsExpensesIncome.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `loonjournaalposten` tag.
#[derive(Clone)]
pub struct LoonjournaalpostenMethods {
    client: JorttClient,
}

impl LoonjournaalpostenMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /loonjournaalposten` (`List Loonjournaalposten`).
    pub async fn get_loonjournaalposten(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::GetLoonjournaalposten.spec(),
                request,
            )
            .await
    }

    /// `POST /loonjournaalposten` (`Create a Loonjournaalpost`).
    pub async fn post_loonjournaalposten(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::PostLoonjournaalposten.spec(),
                request,
            )
            .await
    }

    /// `DELETE /loonjournaalposten/{loonjournaalpost_id}` (`Delete a Loonjournaalpost`).
    pub async fn delete_loonjournaalposten_by_loonjournaalpost_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::DeleteLoonjournaalpostenByLoonjournaalpostId.spec(),
                request,
            )
            .await
    }

    /// `PUT /loonjournaalposten/{loonjournaalpost_id}` (`Update a Loonjournaalpost`).
    pub async fn put_loonjournaalposten_by_loonjournaalpost_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::PutLoonjournaalpostenByLoonjournaalpostId.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/loonjournaalposten` (`List Loonjournaalposten`).
    pub async fn get_v1_loonjournaalposten(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::GetV1Loonjournaalposten.spec(),
                request,
            )
            .await
    }

    /// `POST /v1/loonjournaalposten` (`Create a Loonjournaalpost`).
    pub async fn post_v1_loonjournaalposten(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::PostV1Loonjournaalposten.spec(),
                request,
            )
            .await
    }

    /// `DELETE /v1/loonjournaalposten/{loonjournaalpost_id}` (`Delete a Loonjournaalpost`).
    pub async fn delete_v1_loonjournaalposten_by_loonjournaalpost_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::DeleteV1LoonjournaalpostenByLoonjournaalpostId.spec(),
                request,
            )
            .await
    }

    /// `PUT /v1/loonjournaalposten/{loonjournaalpost_id}` (`Update a Loonjournaalpost`).
    pub async fn put_v1_loonjournaalposten_by_loonjournaalpost_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                LoonjournaalpostenOperation::PutV1LoonjournaalpostenByLoonjournaalpostId.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `organizations` tag.
#[derive(Clone)]
pub struct OrganizationsMethods {
    client: JorttClient,
}

impl OrganizationsMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /organizations/me` (`Get the organization associated with the api credentials`).
    pub async fn get_organizations_me(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(OrganizationsOperation::GetOrganizationsMe.spec(), request)
            .await
    }

    /// `GET /v1/organizations/me` (`Get the organization associated with the api credentials`).
    pub async fn get_v1_organizations_me(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(OrganizationsOperation::GetV1OrganizationsMe.spec(), request)
            .await
    }
}

/// Operation methods for the `projects` tag.
#[derive(Clone)]
pub struct ProjectsMethods {
    client: JorttClient,
}

impl ProjectsMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /projects` (`List Projects`).
    pub async fn get_projects(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::GetProjects.spec(), request)
            .await
    }

    /// `POST /projects` (`Create Project`).
    pub async fn post_projects(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PostProjects.spec(), request)
            .await
    }

    /// `DELETE /projects/{id}` (`Delete Project`).
    pub async fn delete_projects_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::DeleteProjectsById.spec(), request)
            .await
    }

    /// `GET /projects/{id}` (`Get Project`).
    pub async fn get_projects_by_id(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::GetProjectsById.spec(), request)
            .await
    }

    /// `PUT /projects/{id}` (`Update Project`).
    pub async fn put_projects_by_id(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PutProjectsById.spec(), request)
            .await
    }

    /// `POST /projects/{id}/invoice` (`Invoice Project`).
    pub async fn post_projects_by_id_invoice(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PostProjectsByIdInvoice.spec(), request)
            .await
    }

    /// `GET /projects/{id}/line_items` (`List Project Line Items`).
    pub async fn get_projects_by_id_line_items(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::GetProjectsByIdLineItems.spec(), request)
            .await
    }

    /// `POST /projects/{id}/line_items` (`Create Project Line item`).
    pub async fn post_projects_by_id_line_items(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PostProjectsByIdLineItems.spec(), request)
            .await
    }

    /// `DELETE /projects/{id}/line_items/{line_item_id}` (`Delete Project Line item`).
    pub async fn delete_projects_by_id_line_items_by_line_item_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::DeleteProjectsByIdLineItemsByLineItemId.spec(),
                request,
            )
            .await
    }

    /// `GET /projects/{id}/line_items/{line_item_id}` (`Get Project Line Item`).
    pub async fn get_projects_by_id_line_items_by_line_item_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::GetProjectsByIdLineItemsByLineItemId.spec(),
                request,
            )
            .await
    }

    /// `PUT /projects/{id}/line_items/{line_item_id}` (`Update Project Line item`).
    pub async fn put_projects_by_id_line_items_by_line_item_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::PutProjectsByIdLineItemsByLineItemId.spec(),
                request,
            )
            .await
    }

    /// `GET /projects/{id}/line_items/summary` (`Get Project Line Item Summary`).
    pub async fn get_projects_by_id_line_items_summary(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::GetProjectsByIdLineItemsSummary.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/projects` (`List Projects`).
    pub async fn get_v1_projects(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::GetV1Projects.spec(), request)
            .await
    }

    /// `POST /v1/projects` (`Create Project`).
    pub async fn post_v1_projects(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PostV1Projects.spec(), request)
            .await
    }

    /// `DELETE /v1/projects/{id}` (`Delete Project`).
    pub async fn delete_v1_projects_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::DeleteV1ProjectsById.spec(), request)
            .await
    }

    /// `GET /v1/projects/{id}` (`Get Project`).
    pub async fn get_v1_projects_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::GetV1ProjectsById.spec(), request)
            .await
    }

    /// `PUT /v1/projects/{id}` (`Update Project`).
    pub async fn put_v1_projects_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PutV1ProjectsById.spec(), request)
            .await
    }

    /// `POST /v1/projects/{id}/invoice` (`Invoice Project`).
    pub async fn post_v1_projects_by_id_invoice(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ProjectsOperation::PostV1ProjectsByIdInvoice.spec(), request)
            .await
    }

    /// `GET /v1/projects/{id}/line_items` (`List Project Line Items`).
    pub async fn get_v1_projects_by_id_line_items(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::GetV1ProjectsByIdLineItems.spec(),
                request,
            )
            .await
    }

    /// `POST /v1/projects/{id}/line_items` (`Create Project Line item`).
    pub async fn post_v1_projects_by_id_line_items(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::PostV1ProjectsByIdLineItems.spec(),
                request,
            )
            .await
    }

    /// `DELETE /v1/projects/{id}/line_items/{line_item_id}` (`Delete Project Line item`).
    pub async fn delete_v1_projects_by_id_line_items_by_line_item_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::DeleteV1ProjectsByIdLineItemsByLineItemId.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/projects/{id}/line_items/{line_item_id}` (`Get Project Line Item`).
    pub async fn get_v1_projects_by_id_line_items_by_line_item_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::GetV1ProjectsByIdLineItemsByLineItemId.spec(),
                request,
            )
            .await
    }

    /// `PUT /v1/projects/{id}/line_items/{line_item_id}` (`Update Project Line item`).
    pub async fn put_v1_projects_by_id_line_items_by_line_item_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::PutV1ProjectsByIdLineItemsByLineItemId.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/projects/{id}/line_items/summary` (`Get Project Line Item Summary`).
    pub async fn get_v1_projects_by_id_line_items_summary(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ProjectsOperation::GetV1ProjectsByIdLineItemsSummary.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `reports` tag.
#[derive(Clone)]
pub struct ReportsMethods {
    client: JorttClient,
}

impl ReportsMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /reports/summaries/balance` (`Dashboard balance`).
    pub async fn get_reports_summaries_balance(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ReportsOperation::GetReportsSummariesBalance.spec(), request)
            .await
    }

    /// `GET /reports/summaries/btw` (`Dashboard btw`).
    pub async fn get_reports_summaries_btw(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ReportsOperation::GetReportsSummariesBtw.spec(), request)
            .await
    }

    /// `GET /reports/summaries/cash_and_bank` (`Dashboard cash and bank`).
    pub async fn get_reports_summaries_cash_and_bank(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetReportsSummariesCashAndBank.spec(),
                request,
            )
            .await
    }

    /// `GET /reports/summaries/invoices` (`Dashboard invoices`).
    pub async fn get_reports_summaries_invoices(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetReportsSummariesInvoices.spec(),
                request,
            )
            .await
    }

    /// `GET /reports/summaries/profit_and_loss` (`Dashboard profit and loss`).
    pub async fn get_reports_summaries_profit_and_loss(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetReportsSummariesProfitAndLoss.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/reports/summaries/balance` (`Dashboard balance`).
    pub async fn get_v1_reports_summaries_balance(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetV1ReportsSummariesBalance.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/reports/summaries/btw` (`Dashboard btw`).
    pub async fn get_v1_reports_summaries_btw(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(ReportsOperation::GetV1ReportsSummariesBtw.spec(), request)
            .await
    }

    /// `GET /v1/reports/summaries/cash_and_bank` (`Dashboard cash and bank`).
    pub async fn get_v1_reports_summaries_cash_and_bank(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetV1ReportsSummariesCashAndBank.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/reports/summaries/invoices` (`Dashboard invoices`).
    pub async fn get_v1_reports_summaries_invoices(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetV1ReportsSummariesInvoices.spec(),
                request,
            )
            .await
    }

    /// `GET /v1/reports/summaries/profit_and_loss` (`Dashboard profit and loss`).
    pub async fn get_v1_reports_summaries_profit_and_loss(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(
                ReportsOperation::GetV1ReportsSummariesProfitAndLoss.spec(),
                request,
            )
            .await
    }
}

/// Operation methods for the `tradenames` tag.
#[derive(Clone)]
pub struct TradenamesMethods {
    client: JorttClient,
}

impl TradenamesMethods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /tradenames` (`List tradenames`).
    pub async fn get_tradenames(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(TradenamesOperation::GetTradenames.spec(), request)
            .await
    }

    /// `GET /v1/tradenames` (`List tradenames`).
    pub async fn get_v1_tradenames(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(TradenamesOperation::GetV1Tradenames.spec(), request)
            .await
    }
}

/// Operation methods for the `v2` tag.
#[derive(Clone)]
pub struct V2Methods {
    client: JorttClient,
}

impl V2Methods {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// `GET /v2/estimates` (`List Estimates v2`).
    pub async fn get_v2_estimates(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(V2Operation::GetV2Estimates.spec(), request)
            .await
    }

    /// `GET /v2/estimates/{id}` (`Get Estimate by ID v2`).
    pub async fn get_v2_estimates_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(V2Operation::GetV2EstimatesById.spec(), request)
            .await
    }

    /// `GET /v2/estimates/{id}/send_settings` (`Get estimate send settings v2`).
    pub async fn get_v2_estimates_by_id_send_settings(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(V2Operation::GetV2EstimatesByIdSendSettings.spec(), request)
            .await
    }

    /// `GET /v2/invoices` (`List Invoices v2`).
    pub async fn get_v2_invoices(&self, request: OperationRequest) -> Result<Value, JorttError> {
        self.client
            .execute_spec(V2Operation::GetV2Invoices.spec(), request)
            .await
    }

    /// `GET /v2/invoices/{id}` (`Get Invoice by ID v2`).
    pub async fn get_v2_invoices_by_id(
        &self,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute_spec(V2Operation::GetV2InvoicesById.spec(), request)
            .await
    }
}

/// Number of generated endpoint methods across all tags.
pub const GENERATED_METHOD_COUNT: usize = 126;
