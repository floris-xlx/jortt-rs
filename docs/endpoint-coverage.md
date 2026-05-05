# Endpoint Coverage Matrix

Pinned snapshot: `docs/openapi/jortt-swagger-2026-05-05.json`

| Tag | Method | Path | Rust Enum Variant | Operation ID |
| --- | --- | --- | --- | --- |
| `bank-accounts` | `GET` | `/v3/bank-accounts` | `BankAccountsOperation::GetV3BankAccounts` | `List Bank Accounts V3` |
| `bank-accounts` | `GET` | `/v3/bank-accounts/{id}/bank-transactions` | `BankAccountsOperation::GetV3BankAccountsByIdBankTransactions` | `List Bank Transactions V3` |
| `customers` | `GET` | `/customers` | `CustomersOperation::GetCustomers` | `List Customers` |
| `customers` | `POST` | `/customers` | `CustomersOperation::PostCustomers` | `Create Customer` |
| `customers` | `DELETE` | `/customers/{customer_id}` | `CustomersOperation::DeleteCustomersByCustomerId` | `Delete a Customer` |
| `customers` | `GET` | `/customers/{customer_id}` | `CustomersOperation::GetCustomersByCustomerId` | `Get Customer by ID` |
| `customers` | `PUT` | `/customers/{customer_id}` | `CustomersOperation::PutCustomersByCustomerId` | `Update Customer` |
| `customers` | `POST` | `/customers/{customer_id}/direct_debit_mandate` | `CustomersOperation::PostCustomersByCustomerIdDirectDebitMandate` | `Send direct debit authorization to a Customer` |
| `customers` | `GET` | `/customers/{customer_id}/extra_details` | `CustomersOperation::GetCustomersByCustomerIdExtraDetails` | `Get Customer's details by ID` |
| `customers` | `PUT` | `/customers/{customer_id}/set_archived` | `CustomersOperation::PutCustomersByCustomerIdSetArchived` | `Set Customer Archived` |
| `customers` | `GET` | `/customers/{customer_id}/vat-percentages` | `CustomersOperation::GetCustomersByCustomerIdVatPercentages` | `Get vat percentages for a Customer by ID` |
| `customers` | `GET` | `/v1/customers` | `CustomersOperation::GetV1Customers` | `List Customers` |
| `customers` | `POST` | `/v1/customers` | `CustomersOperation::PostV1Customers` | `Create Customer` |
| `customers` | `DELETE` | `/v1/customers/{customer_id}` | `CustomersOperation::DeleteV1CustomersByCustomerId` | `Delete a Customer` |
| `customers` | `GET` | `/v1/customers/{customer_id}` | `CustomersOperation::GetV1CustomersByCustomerId` | `Get Customer by ID` |
| `customers` | `PUT` | `/v1/customers/{customer_id}` | `CustomersOperation::PutV1CustomersByCustomerId` | `Update Customer` |
| `customers` | `POST` | `/v1/customers/{customer_id}/direct_debit_mandate` | `CustomersOperation::PostV1CustomersByCustomerIdDirectDebitMandate` | `Send direct debit authorization to a Customer` |
| `customers` | `GET` | `/v1/customers/{customer_id}/extra_details` | `CustomersOperation::GetV1CustomersByCustomerIdExtraDetails` | `Get Customer's details by ID` |
| `customers` | `PUT` | `/v1/customers/{customer_id}/set_archived` | `CustomersOperation::PutV1CustomersByCustomerIdSetArchived` | `Set Customer Archived` |
| `customers` | `GET` | `/v1/customers/{customer_id}/vat-percentages` | `CustomersOperation::GetV1CustomersByCustomerIdVatPercentages` | `Get vat percentages for a Customer by ID` |
| `customers` | `GET` | `/v2/customers/{customer_id}/vat-percentages` | `CustomersOperation::GetV2CustomersByCustomerIdVatPercentages` | `Get vat percentages for a Customer by ID V2` |
| `estimates` | `POST` | `/v2/estimates` | `EstimatesOperation::PostV2Estimates` | `Create (and optionally send) an Estimate V2` |
| `estimates` | `PUT` | `/v2/estimates/{id}` | `EstimatesOperation::PutV2EstimatesById` | `Edit Estimate V2` |
| `estimates` | `POST` | `/v2/estimates/{id}/send` | `EstimatesOperation::PostV2EstimatesByIdSend` | `Send Estimate V2` |
| `expenses` | `GET` | `/v3/expenses` | `ExpensesOperation::GetV3Expenses` | `List Expenses` |
| `expenses` | `POST` | `/v3/expenses` | `ExpensesOperation::PostV3Expenses` | `Create an Expense` |
| `expenses` | `GET` | `/v3/expenses/id/{id}` | `ExpensesOperation::GetV3ExpensesIdById` | `Get Expense by ID` |
| `expenses` | `POST` | `/v3/expenses/id/{id}` | `ExpensesOperation::PostV3ExpensesIdById` | `Update an Expense` |
| `expenses` | `POST` | `/v3/expenses/id/{id}/receipt` | `ExpensesOperation::PostV3ExpensesIdByIdReceipt` | `Attach Receipt to Expense` |
| `files` | `GET` | `/files/put_url` | `FilesOperation::GetFilesPutUrl` | `Upload URL for attachments` |
| `files` | `GET` | `/v1/files/put_url` | `FilesOperation::GetV1FilesPutUrl` | `Upload URL for attachments` |
| `inbox` | `POST` | `/inbox/images` | `InboxOperation::PostInboxImages` | `Upload receipt` |
| `inbox` | `POST` | `/v1/inbox/images` | `InboxOperation::PostV1InboxImages` | `Upload receipt` |
| `invoices` | `GET` | `/invoices` | `InvoicesOperation::GetInvoices` | `List Invoices` |
| `invoices` | `POST` | `/invoices` | `InvoicesOperation::PostInvoices` | `Create (and optionally send) an Invoice` |
| `invoices` | `DELETE` | `/invoices/{id}` | `InvoicesOperation::DeleteInvoicesById` | `Delete Invoice by ID` |
| `invoices` | `GET` | `/invoices/{id}` | `InvoicesOperation::GetInvoicesById` | `Get Invoice by ID` |
| `invoices` | `PUT` | `/invoices/{id}` | `InvoicesOperation::PutInvoicesById` | `Edit Invoice` |
| `invoices` | `POST` | `/invoices/{id}/copy` | `InvoicesOperation::PostInvoicesByIdCopy` | `Copy an Invoice` |
| `invoices` | `POST` | `/invoices/{id}/credit` | `InvoicesOperation::PostInvoicesByIdCredit` | `Creates (and optionally sends) a credit Invoice` |
| `invoices` | `GET` | `/invoices/{id}/download` | `InvoicesOperation::GetInvoicesByIdDownload` | `Download Invoice PDF` |
| `invoices` | `GET` | `/invoices/{id}/line_item_suggestions` | `InvoicesOperation::GetInvoicesByIdLineItemSuggestions` | `Get line item suggestions` |
| `invoices` | `GET` | `/invoices/{id}/next_possible_invoice_number` | `InvoicesOperation::GetInvoicesByIdNextPossibleInvoiceNumber` | `Get next possible invoice number` |
| `invoices` | `POST` | `/invoices/{id}/send` | `InvoicesOperation::PostInvoicesByIdSend` | `Send Invoice` |
| `invoices` | `GET` | `/invoices/{id}/send_settings` | `InvoicesOperation::GetInvoicesByIdSendSettings` | `Get send settings` |
| `invoices` | `PUT` | `/invoices/{id}/set_labels` | `InvoicesOperation::PutInvoicesByIdSetLabels` | `Set invoice labels` |
| `invoices` | `GET` | `/v1/invoices` | `InvoicesOperation::GetV1Invoices` | `List Invoices` |
| `invoices` | `POST` | `/v1/invoices` | `InvoicesOperation::PostV1Invoices` | `Create (and optionally send) an Invoice` |
| `invoices` | `DELETE` | `/v1/invoices/{id}` | `InvoicesOperation::DeleteV1InvoicesById` | `Delete Invoice by ID` |
| `invoices` | `GET` | `/v1/invoices/{id}` | `InvoicesOperation::GetV1InvoicesById` | `Get Invoice by ID` |
| `invoices` | `PUT` | `/v1/invoices/{id}` | `InvoicesOperation::PutV1InvoicesById` | `Edit Invoice` |
| `invoices` | `POST` | `/v1/invoices/{id}/copy` | `InvoicesOperation::PostV1InvoicesByIdCopy` | `Copy an Invoice` |
| `invoices` | `POST` | `/v1/invoices/{id}/credit` | `InvoicesOperation::PostV1InvoicesByIdCredit` | `Creates (and optionally sends) a credit Invoice` |
| `invoices` | `GET` | `/v1/invoices/{id}/download` | `InvoicesOperation::GetV1InvoicesByIdDownload` | `Download Invoice PDF` |
| `invoices` | `GET` | `/v1/invoices/{id}/line_item_suggestions` | `InvoicesOperation::GetV1InvoicesByIdLineItemSuggestions` | `Get line item suggestions` |
| `invoices` | `GET` | `/v1/invoices/{id}/next_possible_invoice_number` | `InvoicesOperation::GetV1InvoicesByIdNextPossibleInvoiceNumber` | `Get next possible invoice number` |
| `invoices` | `POST` | `/v1/invoices/{id}/send` | `InvoicesOperation::PostV1InvoicesByIdSend` | `Send Invoice` |
| `invoices` | `GET` | `/v1/invoices/{id}/send_settings` | `InvoicesOperation::GetV1InvoicesByIdSendSettings` | `Get send settings` |
| `invoices` | `PUT` | `/v1/invoices/{id}/set_labels` | `InvoicesOperation::PutV1InvoicesByIdSetLabels` | `Set invoice labels` |
| `invoices` | `POST` | `/v2/invoices` | `InvoicesOperation::PostV2Invoices` | `Create (and optionally send) an Invoice V2` |
| `invoices` | `PUT` | `/v2/invoices/{id}` | `InvoicesOperation::PutV2InvoicesById` | `Edit Invoice V2` |
| `invoices` | `GET` | `/v2/invoices/{id}/line_item_suggestions` | `InvoicesOperation::GetV2InvoicesByIdLineItemSuggestions` | `Get line item suggestions` |
| `invoices` | `GET` | `/v2/invoices/peppol-scheme-catalog` | `InvoicesOperation::GetV2InvoicesPeppolSchemeCatalog` | `Get peppol scheme catalog` |
| `invoices` | `POST` | `/v3/invoices` | `InvoicesOperation::PostV3Invoices` | `Create (and optionally send) an Invoice V3` |
| `labels` | `GET` | `/labels` | `LabelsOperation::GetLabels` | `List labels` |
| `labels` | `POST` | `/labels` | `LabelsOperation::PostLabels` | `Create a Label` |
| `labels` | `GET` | `/v1/labels` | `LabelsOperation::GetV1Labels` | `List labels` |
| `labels` | `POST` | `/v1/labels` | `LabelsOperation::PostV1Labels` | `Create a Label` |
| `ledger_accounts` | `GET` | `/ledger_accounts/invoices` | `LedgerAccountsOperation::GetLedgerAccountsInvoices` | `List Invoice Ledger Accounts` |
| `ledger_accounts` | `GET` | `/ledger_accounts/invoices/default` | `LedgerAccountsOperation::GetLedgerAccountsInvoicesDefault` | `The default Ledger Account for invoice line items` |
| `ledger_accounts` | `GET` | `/v1/ledger_accounts/invoices` | `LedgerAccountsOperation::GetV1LedgerAccountsInvoices` | `List Invoice Ledger Accounts` |
| `ledger_accounts` | `GET` | `/v1/ledger_accounts/invoices/default` | `LedgerAccountsOperation::GetV1LedgerAccountsInvoicesDefault` | `The default Ledger Account for invoice line items` |
| `ledger_accounts` | `GET` | `/v3/ledger_accounts/expenses/balance` | `LedgerAccountsOperation::GetV3LedgerAccountsExpensesBalance` | `getV3LedgerAccountsExpensesBalance` |
| `ledger_accounts` | `GET` | `/v3/ledger_accounts/expenses/cost` | `LedgerAccountsOperation::GetV3LedgerAccountsExpensesCost` | `getV3LedgerAccountsExpensesCost` |
| `ledger_accounts` | `GET` | `/v3/ledger_accounts/expenses/income` | `LedgerAccountsOperation::GetV3LedgerAccountsExpensesIncome` | `getV3LedgerAccountsExpensesIncome` |
| `loonjournaalposten` | `GET` | `/loonjournaalposten` | `LoonjournaalpostenOperation::GetLoonjournaalposten` | `List Loonjournaalposten` |
| `loonjournaalposten` | `POST` | `/loonjournaalposten` | `LoonjournaalpostenOperation::PostLoonjournaalposten` | `Create a Loonjournaalpost` |
| `loonjournaalposten` | `DELETE` | `/loonjournaalposten/{loonjournaalpost_id}` | `LoonjournaalpostenOperation::DeleteLoonjournaalpostenByLoonjournaalpostId` | `Delete a Loonjournaalpost` |
| `loonjournaalposten` | `PUT` | `/loonjournaalposten/{loonjournaalpost_id}` | `LoonjournaalpostenOperation::PutLoonjournaalpostenByLoonjournaalpostId` | `Update a Loonjournaalpost` |
| `loonjournaalposten` | `GET` | `/v1/loonjournaalposten` | `LoonjournaalpostenOperation::GetV1Loonjournaalposten` | `List Loonjournaalposten` |
| `loonjournaalposten` | `POST` | `/v1/loonjournaalposten` | `LoonjournaalpostenOperation::PostV1Loonjournaalposten` | `Create a Loonjournaalpost` |
| `loonjournaalposten` | `DELETE` | `/v1/loonjournaalposten/{loonjournaalpost_id}` | `LoonjournaalpostenOperation::DeleteV1LoonjournaalpostenByLoonjournaalpostId` | `Delete a Loonjournaalpost` |
| `loonjournaalposten` | `PUT` | `/v1/loonjournaalposten/{loonjournaalpost_id}` | `LoonjournaalpostenOperation::PutV1LoonjournaalpostenByLoonjournaalpostId` | `Update a Loonjournaalpost` |
| `organizations` | `GET` | `/organizations/me` | `OrganizationsOperation::GetOrganizationsMe` | `Get the organization associated with the api credentials` |
| `organizations` | `GET` | `/v1/organizations/me` | `OrganizationsOperation::GetV1OrganizationsMe` | `Get the organization associated with the api credentials` |
| `projects` | `GET` | `/projects` | `ProjectsOperation::GetProjects` | `List Projects` |
| `projects` | `POST` | `/projects` | `ProjectsOperation::PostProjects` | `Create Project` |
| `projects` | `DELETE` | `/projects/{id}` | `ProjectsOperation::DeleteProjectsById` | `Delete Project` |
| `projects` | `GET` | `/projects/{id}` | `ProjectsOperation::GetProjectsById` | `Get Project` |
| `projects` | `PUT` | `/projects/{id}` | `ProjectsOperation::PutProjectsById` | `Update Project` |
| `projects` | `POST` | `/projects/{id}/invoice` | `ProjectsOperation::PostProjectsByIdInvoice` | `Invoice Project` |
| `projects` | `GET` | `/projects/{id}/line_items` | `ProjectsOperation::GetProjectsByIdLineItems` | `List Project Line Items` |
| `projects` | `POST` | `/projects/{id}/line_items` | `ProjectsOperation::PostProjectsByIdLineItems` | `Create Project Line item` |
| `projects` | `DELETE` | `/projects/{id}/line_items/{line_item_id}` | `ProjectsOperation::DeleteProjectsByIdLineItemsByLineItemId` | `Delete Project Line item` |
| `projects` | `GET` | `/projects/{id}/line_items/{line_item_id}` | `ProjectsOperation::GetProjectsByIdLineItemsByLineItemId` | `Get Project Line Item` |
| `projects` | `PUT` | `/projects/{id}/line_items/{line_item_id}` | `ProjectsOperation::PutProjectsByIdLineItemsByLineItemId` | `Update Project Line item` |
| `projects` | `GET` | `/projects/{id}/line_items/summary` | `ProjectsOperation::GetProjectsByIdLineItemsSummary` | `Get Project Line Item Summary` |
| `projects` | `GET` | `/v1/projects` | `ProjectsOperation::GetV1Projects` | `List Projects` |
| `projects` | `POST` | `/v1/projects` | `ProjectsOperation::PostV1Projects` | `Create Project` |
| `projects` | `DELETE` | `/v1/projects/{id}` | `ProjectsOperation::DeleteV1ProjectsById` | `Delete Project` |
| `projects` | `GET` | `/v1/projects/{id}` | `ProjectsOperation::GetV1ProjectsById` | `Get Project` |
| `projects` | `PUT` | `/v1/projects/{id}` | `ProjectsOperation::PutV1ProjectsById` | `Update Project` |
| `projects` | `POST` | `/v1/projects/{id}/invoice` | `ProjectsOperation::PostV1ProjectsByIdInvoice` | `Invoice Project` |
| `projects` | `GET` | `/v1/projects/{id}/line_items` | `ProjectsOperation::GetV1ProjectsByIdLineItems` | `List Project Line Items` |
| `projects` | `POST` | `/v1/projects/{id}/line_items` | `ProjectsOperation::PostV1ProjectsByIdLineItems` | `Create Project Line item` |
| `projects` | `DELETE` | `/v1/projects/{id}/line_items/{line_item_id}` | `ProjectsOperation::DeleteV1ProjectsByIdLineItemsByLineItemId` | `Delete Project Line item` |
| `projects` | `GET` | `/v1/projects/{id}/line_items/{line_item_id}` | `ProjectsOperation::GetV1ProjectsByIdLineItemsByLineItemId` | `Get Project Line Item` |
| `projects` | `PUT` | `/v1/projects/{id}/line_items/{line_item_id}` | `ProjectsOperation::PutV1ProjectsByIdLineItemsByLineItemId` | `Update Project Line item` |
| `projects` | `GET` | `/v1/projects/{id}/line_items/summary` | `ProjectsOperation::GetV1ProjectsByIdLineItemsSummary` | `Get Project Line Item Summary` |
| `reports` | `GET` | `/reports/summaries/balance` | `ReportsOperation::GetReportsSummariesBalance` | `Dashboard balance` |
| `reports` | `GET` | `/reports/summaries/btw` | `ReportsOperation::GetReportsSummariesBtw` | `Dashboard btw` |
| `reports` | `GET` | `/reports/summaries/cash_and_bank` | `ReportsOperation::GetReportsSummariesCashAndBank` | `Dashboard cash and bank` |
| `reports` | `GET` | `/reports/summaries/invoices` | `ReportsOperation::GetReportsSummariesInvoices` | `Dashboard invoices` |
| `reports` | `GET` | `/reports/summaries/profit_and_loss` | `ReportsOperation::GetReportsSummariesProfitAndLoss` | `Dashboard profit and loss` |
| `reports` | `GET` | `/v1/reports/summaries/balance` | `ReportsOperation::GetV1ReportsSummariesBalance` | `Dashboard balance` |
| `reports` | `GET` | `/v1/reports/summaries/btw` | `ReportsOperation::GetV1ReportsSummariesBtw` | `Dashboard btw` |
| `reports` | `GET` | `/v1/reports/summaries/cash_and_bank` | `ReportsOperation::GetV1ReportsSummariesCashAndBank` | `Dashboard cash and bank` |
| `reports` | `GET` | `/v1/reports/summaries/invoices` | `ReportsOperation::GetV1ReportsSummariesInvoices` | `Dashboard invoices` |
| `reports` | `GET` | `/v1/reports/summaries/profit_and_loss` | `ReportsOperation::GetV1ReportsSummariesProfitAndLoss` | `Dashboard profit and loss` |
| `tradenames` | `GET` | `/tradenames` | `TradenamesOperation::GetTradenames` | `List tradenames` |
| `tradenames` | `GET` | `/v1/tradenames` | `TradenamesOperation::GetV1Tradenames` | `List tradenames` |
| `v2` | `GET` | `/v2/estimates` | `V2Operation::GetV2Estimates` | `List Estimates v2` |
| `v2` | `GET` | `/v2/estimates/{id}` | `V2Operation::GetV2EstimatesById` | `Get Estimate by ID v2` |
| `v2` | `GET` | `/v2/estimates/{id}/send_settings` | `V2Operation::GetV2EstimatesByIdSendSettings` | `Get estimate send settings v2` |
| `v2` | `GET` | `/v2/invoices` | `V2Operation::GetV2Invoices` | `List Invoices v2` |
| `v2` | `GET` | `/v2/invoices/{id}` | `V2Operation::GetV2InvoicesById` | `Get Invoice by ID v2` |
