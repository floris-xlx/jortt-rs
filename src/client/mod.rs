//! High-level Jortt API client.

use std::sync::Arc;
use std::time::Duration;

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Method, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::time::sleep;

use crate::api::endpoints::ApiMethods;
use crate::api::operations::{
    BankAccountsOperation, CustomersOperation, EstimatesOperation, ExpensesOperation,
    FilesOperation, HttpMethod, InboxOperation, InvoicesOperation, LabelsOperation,
    LedgerAccountsOperation, LoonjournaalpostenOperation, OperationSpec, OrganizationsOperation,
    ProjectsOperation, ReportsOperation, TradenamesOperation, V2Operation,
};
use crate::api::{DomainApi, OperationRequest};
use crate::auth::AccessTokenSource;
use crate::error::{ErrorBuilder, ErrorEnvelope, JorttError};
use crate::models::common::DataEnvelope;
use crate::models::customers::{
    Customer, ListCustomersQuery, ListCustomersResponse, UpsertCustomerRequest,
};
use crate::models::invoices::{
    CreateInvoiceRequest, Invoice, InvoiceDownload, ListInvoicesQuery, ListInvoicesResponse,
};
use crate::models::ledger_accounts::{LedgerAccount, ListLedgerAccountsResponse};

const PATH_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'/')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

struct ClientInner {
    http: reqwest::Client,
    base_url: Url,
    token_source: Option<Arc<dyn AccessTokenSource>>,
    max_retries: u8,
}

/// Builder for [`JorttClient`].
#[derive(Clone)]
pub struct JorttClientBuilder {
    base_url: Url,
    timeout: Duration,
    user_agent: String,
    token_source: Option<Arc<dyn AccessTokenSource>>,
    max_retries: u8,
}

impl Default for JorttClientBuilder {
    fn default() -> Self {
        Self {
            base_url: Url::parse("https://api.jortt.nl/").expect("static URL must be valid"),
            timeout: Duration::from_secs(30),
            user_agent: format!("jortt-rs/{}", env!("CARGO_PKG_VERSION")),
            token_source: None,
            max_retries: 2,
        }
    }
}

impl JorttClientBuilder {
    /// Creates a default builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the API base URL.
    pub fn with_base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self
    }

    /// Sets request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets user-agent header value.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Sets retry count for transport and retryable HTTP statuses.
    pub fn with_max_retries(mut self, max_retries: u8) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets bearer token source.
    pub fn with_token_source(mut self, token_source: Arc<dyn AccessTokenSource>) -> Self {
        self.token_source = Some(token_source);
        self
    }

    /// Builds the SDK client.
    pub fn build(self) -> Result<JorttClient, JorttError> {
        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(JorttError::Transport)?;

        Ok(JorttClient {
            inner: Arc::new(ClientInner {
                http,
                base_url: self.base_url,
                token_source: self.token_source,
                max_retries: self.max_retries,
            }),
            user_agent: self.user_agent,
        })
    }
}

/// Main Jortt API client.
#[derive(Clone)]
pub struct JorttClient {
    inner: Arc<ClientInner>,
    user_agent: String,
}

impl JorttClient {
    /// Returns a new builder.
    pub fn builder() -> JorttClientBuilder {
        JorttClientBuilder::new()
    }

    /// Returns the configured base URL.
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    /// Returns grouped generated methods for every OpenAPI operation.
    pub fn methods(&self) -> ApiMethods {
        ApiMethods::new(self.clone())
    }

    /// Typed operation access for `customers` tag.
    pub fn customers(&self) -> DomainApi<CustomersOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `invoices` tag.
    pub fn invoices(&self) -> DomainApi<InvoicesOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `ledger_accounts` tag.
    pub fn ledger_accounts(&self) -> DomainApi<LedgerAccountsOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `projects` tag.
    pub fn projects(&self) -> DomainApi<ProjectsOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `reports` tag.
    pub fn reports(&self) -> DomainApi<ReportsOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `expenses` tag.
    pub fn expenses(&self) -> DomainApi<ExpensesOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `customers` tag.
    pub fn estimates(&self) -> DomainApi<EstimatesOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `files` tag.
    pub fn files(&self) -> DomainApi<FilesOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `inbox` tag.
    pub fn inbox(&self) -> DomainApi<InboxOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `labels` tag.
    pub fn labels(&self) -> DomainApi<LabelsOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `loonjournaalposten` tag.
    pub fn loonjournaalposten(&self) -> DomainApi<LoonjournaalpostenOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `organizations` tag.
    pub fn organizations(&self) -> DomainApi<OrganizationsOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `tradenames` tag.
    pub fn tradenames(&self) -> DomainApi<TradenamesOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `bank-accounts` tag.
    pub fn bank_accounts(&self) -> DomainApi<BankAccountsOperation> {
        DomainApi::new(self.clone())
    }

    /// Typed operation access for `v2` tag.
    pub fn v2(&self) -> DomainApi<V2Operation> {
        DomainApi::new(self.clone())
    }

    /// Creates a customer using the unversioned endpoint.
    pub async fn create_customer(
        &self,
        request: &UpsertCustomerRequest,
    ) -> Result<Customer, JorttError> {
        let req = OperationRequest::new().with_json_body(request)?;
        let value = self
            .customers()
            .execute(CustomersOperation::PostCustomers, req)
            .await?;
        parse_data_envelope(value)
    }

    /// Updates a customer using the unversioned endpoint.
    pub async fn update_customer(
        &self,
        customer_id: &str,
        request: &UpsertCustomerRequest,
    ) -> Result<Customer, JorttError> {
        let req = OperationRequest::new()
            .with_path_param("customer_id", customer_id)
            .with_json_body(request)?;

        let value = self
            .customers()
            .execute(CustomersOperation::PutCustomersByCustomerId, req)
            .await?;

        parse_data_envelope(value)
    }

    /// Fetches a customer by ID.
    pub async fn get_customer(&self, customer_id: &str) -> Result<Customer, JorttError> {
        let req = OperationRequest::new().with_path_param("customer_id", customer_id);
        let value = self
            .customers()
            .execute(CustomersOperation::GetCustomersByCustomerId, req)
            .await?;
        parse_data_envelope(value)
    }

    /// Lists customers, optionally filtered with query parameters.
    pub async fn list_customers(
        &self,
        query: &ListCustomersQuery,
    ) -> Result<ListCustomersResponse, JorttError> {
        let mut req = OperationRequest::new();
        if let Some(value) = &query.query {
            req = req.with_query_param("query", value);
        }
        if let Some(value) = query.page {
            req = req.with_query_param("page", value);
        }
        let value = self
            .customers()
            .execute(CustomersOperation::GetCustomers, req)
            .await?;
        parse_json(value)
    }

    /// Sends a direct debit mandate request to the customer.
    pub async fn send_customer_direct_debit_mandate(
        &self,
        customer_id: &str,
    ) -> Result<Customer, JorttError> {
        let req = OperationRequest::new().with_path_param("customer_id", customer_id);
        let value = self
            .customers()
            .execute(
                CustomersOperation::PostCustomersByCustomerIdDirectDebitMandate,
                req,
            )
            .await?;
        parse_data_envelope(value)
    }

    /// Deletes a customer and returns the deleted resource snapshot.
    pub async fn delete_customer(&self, customer_id: &str) -> Result<Customer, JorttError> {
        let req = OperationRequest::new().with_path_param("customer_id", customer_id);
        let value = self
            .customers()
            .execute(CustomersOperation::DeleteCustomersByCustomerId, req)
            .await?;
        parse_data_envelope(value)
    }

    /// Creates an invoice using the unversioned endpoint.
    pub async fn create_invoice(
        &self,
        request: &CreateInvoiceRequest,
    ) -> Result<Invoice, JorttError> {
        let req = OperationRequest::new().with_json_body(request)?;
        let value = self
            .invoices()
            .execute(InvoicesOperation::PostInvoices, req)
            .await?;
        parse_data_envelope(value)
    }

    /// Lists invoices with optional search/status filters.
    pub async fn list_invoices(
        &self,
        query: &ListInvoicesQuery,
    ) -> Result<ListInvoicesResponse, JorttError> {
        let mut req = OperationRequest::new();
        if let Some(value) = &query.query {
            req = req.with_query_param("query", value);
        }
        if let Some(value) = &query.invoice_status {
            req = req.with_query_param("invoice_status", value);
        }
        if let Some(value) = query.page {
            req = req.with_query_param("page", value);
        }
        let value = self
            .invoices()
            .execute(InvoicesOperation::GetInvoices, req)
            .await?;
        parse_json(value)
    }

    /// Fetches a single invoice by ID.
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Invoice, JorttError> {
        let req = OperationRequest::new().with_path_param("id", invoice_id);
        let value = self
            .invoices()
            .execute(InvoicesOperation::GetInvoicesById, req)
            .await?;
        parse_data_envelope(value)
    }

    /// Requests a temporary invoice PDF download URL.
    pub async fn download_invoice_pdf_url(
        &self,
        invoice_id: &str,
    ) -> Result<InvoiceDownload, JorttError> {
        let req = OperationRequest::new().with_path_param("id", invoice_id);
        let value = self
            .invoices()
            .execute(InvoicesOperation::GetInvoicesByIdDownload, req)
            .await?;
        parse_data_envelope(value)
    }

    /// Lists invoice ledger accounts.
    pub async fn list_invoice_ledger_accounts(&self) -> Result<Vec<LedgerAccount>, JorttError> {
        let value = self
            .ledger_accounts()
            .execute(
                LedgerAccountsOperation::GetLedgerAccountsInvoices,
                OperationRequest::new(),
            )
            .await?;
        let parsed: ListLedgerAccountsResponse = parse_json(value)?;
        Ok(parsed.data)
    }

    pub(crate) async fn execute_spec(
        &self,
        spec: OperationSpec,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.execute(
            spec.method,
            spec.path,
            request.path_params,
            request.query_params,
            request.body,
            request.accept,
        )
        .await
    }

    pub(crate) async fn execute(
        &self,
        method: HttpMethod,
        path_template: &str,
        path_params: Vec<crate::api::PathParam>,
        query_params: Vec<crate::api::QueryParam>,
        body: Option<Value>,
        accept: Option<String>,
    ) -> Result<Value, JorttError> {
        let path = render_path(path_template, &path_params)?;
        let url = self
            .inner
            .base_url
            .join(path.trim_start_matches('/'))
            .map_err(|err| ErrorBuilder::config(err.to_string()).build())?;

        let method = to_reqwest_method(method);

        for attempt in 0..=self.inner.max_retries {
            let mut req = self
                .inner
                .http
                .request(method.clone(), url.clone())
                .header(USER_AGENT, &self.user_agent);

            if let Some(accept) = &accept {
                req = req.header(ACCEPT, accept);
            }

            if !query_params.is_empty() {
                let pairs = query_params
                    .iter()
                    .map(|param| (param.name.as_str(), param.value.as_str()))
                    .collect::<Vec<_>>();
                req = req.query(&pairs);
            }

            if let Some(token_source) = &self.inner.token_source {
                let token = token_source.access_token().await?;
                req = req.header(AUTHORIZATION, format!("Bearer {token}"));
            }

            if let Some(body) = &body {
                req = req.json(body);
            }

            match req.send().await {
                Ok(response) => {
                    let status = response.status();

                    if should_retry(status) && attempt < self.inner.max_retries {
                        sleep(backoff_delay(attempt)).await;
                        continue;
                    }

                    if status.is_success() {
                        if status == StatusCode::NO_CONTENT {
                            return Ok(Value::Null);
                        }

                        let bytes = response.bytes().await?;
                        if bytes.is_empty() {
                            return Ok(Value::Null);
                        }
                        return Ok(serde_json::from_slice(&bytes)?);
                    }

                    let raw_body = response.text().await.unwrap_or_default();
                    if let Ok(envelope) = serde_json::from_str::<ErrorEnvelope>(&raw_body) {
                        return Err(ErrorBuilder::api(status, envelope.error).build());
                    }

                    return Err(ErrorBuilder::http(status).body(raw_body).build());
                }
                Err(err) => {
                    if attempt < self.inner.max_retries {
                        sleep(backoff_delay(attempt)).await;
                        continue;
                    }
                    return Err(JorttError::Transport(err));
                }
            }
        }

        Err(ErrorBuilder::config("retry loop exhausted unexpectedly").build())
    }
}

fn to_reqwest_method(method: HttpMethod) -> Method {
    match method {
        HttpMethod::Get => Method::GET,
        HttpMethod::Post => Method::POST,
        HttpMethod::Put => Method::PUT,
        HttpMethod::Delete => Method::DELETE,
        HttpMethod::Patch => Method::PATCH,
        HttpMethod::Options => Method::OPTIONS,
        HttpMethod::Head => Method::HEAD,
    }
}

fn render_path(
    path_template: &str,
    path_params: &[crate::api::PathParam],
) -> Result<String, JorttError> {
    let mut rendered = path_template.to_string();

    for param in path_params {
        let encoded = utf8_percent_encode(&param.value, PATH_ENCODE_SET).to_string();
        rendered = rendered.replace(&format!("{{{}}}", param.name), &encoded);
    }

    if let Some(start) = rendered.find('{') {
        let tail = &rendered[start + 1..];
        let end = tail.find('}').unwrap_or(tail.len());
        let name = tail[..end].to_string();
        return Err(ErrorBuilder::missing_path_param(name, path_template).build());
    }

    Ok(rendered)
}

fn should_retry(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn backoff_delay(attempt: u8) -> Duration {
    let multiplier = 1u64 << attempt;
    Duration::from_millis(100 * multiplier)
}

fn parse_json<T: DeserializeOwned>(value: Value) -> Result<T, JorttError> {
    serde_json::from_value(value).map_err(JorttError::Deserialize)
}

fn parse_data_envelope<T: DeserializeOwned>(value: Value) -> Result<T, JorttError> {
    let envelope: DataEnvelope<T> = parse_json(value)?;
    Ok(envelope.data)
}
