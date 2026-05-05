//! Raw operation escape hatch.

use serde_json::Value;

use crate::api::OperationRequest;
use crate::api::operations::{HttpMethod, OperationSpec};
use crate::client::JorttClient;
use crate::error::JorttError;

/// Low-level API for calling arbitrary Jortt operations.
#[derive(Clone)]
pub struct RawClient {
    client: JorttClient,
}

impl RawClient {
    /// Creates a raw client bound to a high-level [`JorttClient`].
    pub fn new(client: JorttClient) -> Self {
        Self { client }
    }

    /// Executes using an explicit operation descriptor.
    pub async fn execute_spec(
        &self,
        spec: OperationSpec,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client.execute_spec(spec, request).await
    }

    /// Executes an ad-hoc method/path call.
    pub async fn execute(
        &self,
        method: HttpMethod,
        path_template: &str,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client
            .execute(
                method,
                path_template,
                request.path_params,
                request.query_params,
                request.body,
                request.accept,
            )
            .await
    }
}

impl JorttClient {
    /// Returns the raw escape hatch client.
    pub fn raw(&self) -> RawClient {
        RawClient::new(self.clone())
    }
}
