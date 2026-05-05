use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Structured API error payload returned by Jortt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiError {
    /// HTTP-like Jortt error code.
    pub code: i32,
    /// Stable machine-readable error key.
    pub key: String,
    /// Human-readable message.
    pub message: String,
    /// Optional field-level details.
    #[serde(default)]
    pub details: Vec<ApiErrorDetail>,
}

/// Nested detail object in a Jortt API error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiErrorDetail {
    /// Parameter name when applicable.
    #[serde(default)]
    pub param: Option<String>,
    /// Machine key for the detail.
    pub key: String,
    /// Human-readable detail text.
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ErrorEnvelope {
    pub error: ApiError,
}

/// Main SDK error type.
#[derive(Debug, Error)]
pub enum JorttError {
    /// SDK configuration is invalid.
    #[error("invalid SDK configuration: {0}")]
    Config(String),

    /// Required path parameter is missing.
    #[error("missing path parameter `{name}` for template `{template}`")]
    MissingPathParam {
        /// Parameter name.
        name: String,
        /// Original URI template.
        template: String,
    },

    /// Request failed before a valid HTTP response was received.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// Response body could not be parsed as JSON.
    #[error("response deserialization failed: {0}")]
    Deserialize(#[from] serde_json::Error),

    /// API returned a structured error payload.
    #[error("jortt api error {status}: {error_key} - {message}")]
    Api {
        /// HTTP status code.
        status: StatusCode,
        /// API error key.
        error_key: String,
        /// Human-readable message.
        message: String,
        /// Parsed API payload.
        payload: Box<ApiError>,
    },

    /// API returned non-success and non-JSON payload.
    #[error("http error {status}: {body}")]
    Http {
        /// HTTP status code.
        status: StatusCode,
        /// Response body snippet.
        body: String,
    },

    /// Serialization of outbound JSON payload failed.
    #[error("request serialization failed: {0}")]
    Serialize(String),
}

impl JorttError {
    pub(crate) fn from_serialize(err: serde_json::Error) -> Self {
        Self::Serialize(err.to_string())
    }
}

/// Canonical builder for producing [`JorttError`] values.
#[derive(Debug, Clone, Default)]
pub struct ErrorBuilder {
    status: Option<StatusCode>,
    error_key: Option<String>,
    message: Option<String>,
    payload: Option<ApiError>,
    body: Option<String>,
    config: Option<String>,
    missing_name: Option<String>,
    missing_template: Option<String>,
}

impl ErrorBuilder {
    /// Starts an API error builder with known status and payload.
    pub fn api(status: StatusCode, payload: ApiError) -> Self {
        Self {
            status: Some(status),
            error_key: Some(payload.key.clone()),
            message: Some(payload.message.clone()),
            payload: Some(payload),
            ..Self::default()
        }
    }

    /// Starts an HTTP body error builder with known status.
    pub fn http(status: StatusCode) -> Self {
        Self {
            status: Some(status),
            ..Self::default()
        }
    }

    /// Starts a configuration error builder.
    pub fn config(message: impl Into<String>) -> Self {
        Self {
            config: Some(message.into()),
            ..Self::default()
        }
    }

    /// Starts a missing path parameter error builder.
    pub fn missing_path_param(name: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            missing_name: Some(name.into()),
            missing_template: Some(template.into()),
            ..Self::default()
        }
    }

    /// Sets the HTTP body text for an HTTP error.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Overrides the API error key.
    pub fn error_key(mut self, key: impl Into<String>) -> Self {
        self.error_key = Some(key.into());
        self
    }

    /// Overrides the API human-readable message.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Finalizes the error object.
    pub fn build(self) -> JorttError {
        if let Some(message) = self.config {
            return JorttError::Config(message);
        }

        if let Some(name) = self.missing_name {
            return JorttError::MissingPathParam {
                name,
                template: self
                    .missing_template
                    .unwrap_or_else(|| "<unknown template>".to_string()),
            };
        }

        if let Some(payload) = self.payload {
            return JorttError::Api {
                status: self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                error_key: self.error_key.unwrap_or_else(|| payload.key.clone()),
                message: self.message.unwrap_or_else(|| payload.message.clone()),
                payload: Box::new(payload),
            };
        }

        if let Some(status) = self.status {
            return JorttError::Http {
                status,
                body: self.body.unwrap_or_default(),
            };
        }

        JorttError::Config("invalid ErrorBuilder state".to_string())
    }
}
