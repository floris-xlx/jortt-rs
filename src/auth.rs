use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

use crate::error::JorttError;

/// Source for obtaining bearer tokens at request time.
#[async_trait]
pub trait AccessTokenSource: Send + Sync {
    /// Returns an access token without the `Bearer ` prefix.
    async fn access_token(&self) -> Result<String, JorttError>;
}

/// Static token source for simple integrations.
#[derive(Debug, Clone)]
pub struct StaticAccessToken {
    token: String,
}

impl StaticAccessToken {
    /// Creates a static token source.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl AccessTokenSource for StaticAccessToken {
    async fn access_token(&self) -> Result<String, JorttError> {
        Ok(self.token.clone())
    }
}

/// OAuth scope values supported by Jortt.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    /// `customers:read`
    CustomersRead,
    /// `customers:write`
    CustomersWrite,
    /// `estimates:read`
    EstimatesRead,
    /// `estimates:write`
    EstimatesWrite,
    /// `expenses:read`
    ExpensesRead,
    /// `expenses:write`
    ExpensesWrite,
    /// `financing:read`
    FinancingRead,
    /// `inbox:write`
    InboxWrite,
    /// `invoices:read`
    InvoicesRead,
    /// `invoices:write`
    InvoicesWrite,
    /// `organizations:read`
    OrganizationsRead,
    /// `organizations:write`
    OrganizationsWrite,
    /// `payroll:read`
    PayrollRead,
    /// `payroll:write`
    PayrollWrite,
    /// `reports:read`
    ReportsRead,
    /// Any other future scope.
    Custom(String),
}

impl Scope {
    /// Returns the scope token string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::CustomersRead => "customers:read",
            Self::CustomersWrite => "customers:write",
            Self::EstimatesRead => "estimates:read",
            Self::EstimatesWrite => "estimates:write",
            Self::ExpensesRead => "expenses:read",
            Self::ExpensesWrite => "expenses:write",
            Self::FinancingRead => "financing:read",
            Self::InboxWrite => "inbox:write",
            Self::InvoicesRead => "invoices:read",
            Self::InvoicesWrite => "invoices:write",
            Self::OrganizationsRead => "organizations:read",
            Self::OrganizationsWrite => "organizations:write",
            Self::PayrollRead => "payroll:read",
            Self::PayrollWrite => "payroll:write",
            Self::ReportsRead => "reports:read",
            Self::Custom(value) => value,
        }
    }
}

/// OAuth token response payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenSet {
    /// Bearer access token.
    pub access_token: String,
    /// Usually `Bearer`.
    pub token_type: String,
    /// Optional refresh token.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// Lifetime in seconds.
    #[serde(default)]
    pub expires_in: Option<u64>,
    /// Space-separated scopes.
    #[serde(default)]
    pub scope: Option<String>,
}

/// OAuth endpoint configuration.
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// Authorization URL for auth-code flow.
    pub authorization_url: Url,
    /// Token endpoint URL.
    pub token_url: Url,
    /// Request timeout.
    pub timeout: Duration,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            authorization_url: Url::parse("https://app.jortt.nl/oauth-provider/oauth/authorize")
                .expect("static authorization URL must be valid"),
            token_url: Url::parse("https://app.jortt.nl/oauth-provider/oauth/token")
                .expect("static token URL must be valid"),
            timeout: Duration::from_secs(30),
        }
    }
}

/// OAuth helper client.
#[derive(Debug, Clone)]
pub struct OAuthClient {
    config: OAuthConfig,
    http: Client,
}

impl OAuthClient {
    /// Creates a new OAuth helper client.
    pub fn new(config: OAuthConfig) -> Result<Self, JorttError> {
        let http = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(JorttError::Transport)?;

        Ok(Self { config, http })
    }

    /// Returns the configured auth-code authorization URL.
    pub fn authorization_url(&self) -> &Url {
        &self.config.authorization_url
    }

    /// Exchanges an authorization code for tokens.
    pub async fn exchange_authorization_code(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<TokenSet, JorttError> {
        let response = self
            .http
            .post(self.config.token_url.clone())
            .basic_auth(client_id, Some(client_secret))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", redirect_uri),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<TokenSet>().await?)
        } else {
            let status: reqwest::StatusCode = response.status();
            let body: String = response.text().await.unwrap_or_default();
            Err(JorttError::Http { status, body })
        }
    }

    /// Exchanges client credentials for an access token.
    pub async fn exchange_client_credentials(
        &self,
        client_id: &str,
        client_secret: &str,
        scopes: &[Scope],
    ) -> Result<TokenSet, JorttError> {
        let scope_value: String = scopes
            .iter()
            .map(Scope::as_str)
            .collect::<Vec<_>>()
            .join(" ");

        let response = self
            .http
            .post(self.config.token_url.clone())
            .basic_auth(client_id, Some(client_secret))
            .form(&[
                ("grant_type", "client_credentials"),
                ("scope", scope_value.as_str()),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<TokenSet>().await?)
        } else {
            let status: reqwest::StatusCode = response.status();
            let body: String = response.text().await.unwrap_or_default();
            Err(JorttError::Http { status, body })
        }
    }

    /// Exchanges a refresh token for a new token set.
    pub async fn refresh_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
        refresh_token: &str,
    ) -> Result<TokenSet, JorttError> {
        let response = self
            .http
            .post(self.config.token_url.clone())
            .basic_auth(client_id, Some(client_secret))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<TokenSet>().await?)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(JorttError::Http { status, body })
        }
    }
}
