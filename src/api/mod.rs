//! Typed operation execution API.

use std::marker::PhantomData;

use serde::Serialize;
use serde_json::Value;

use crate::api::operations::TypedOperation;
use crate::client::JorttClient;
use crate::error::JorttError;

/// Generated per-operation method groups.
pub mod endpoints;
/// Generated typed operation enums and metadata.
pub mod operations;

/// Path parameter used to render operation URI templates.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathParam {
    /// Parameter name without braces.
    pub name: String,
    /// Parameter value.
    pub value: String,
}

impl PathParam {
    /// Creates a new path parameter.
    pub fn new(name: impl Into<String>, value: impl ToString) -> Self {
        Self {
            name: name.into(),
            value: value.to_string(),
        }
    }
}

/// Query parameter for operation execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryParam {
    /// Query key.
    pub name: String,
    /// Query value.
    pub value: String,
}

impl QueryParam {
    /// Creates a new query parameter.
    pub fn new(name: impl Into<String>, value: impl ToString) -> Self {
        Self {
            name: name.into(),
            value: value.to_string(),
        }
    }
}

/// Request builder for executing a typed operation.
#[derive(Debug, Clone, Default)]
pub struct OperationRequest {
    /// Path template values.
    pub path_params: Vec<PathParam>,
    /// Query string pairs.
    pub query_params: Vec<QueryParam>,
    /// Optional JSON body.
    pub body: Option<Value>,
    /// Optional explicit `Accept` header.
    pub accept: Option<String>,
}

impl OperationRequest {
    /// Creates a fluent request builder.
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    /// Creates an empty operation request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a path parameter.
    pub fn with_path_param(mut self, name: impl Into<String>, value: impl ToString) -> Self {
        self.path_params.push(PathParam::new(name, value));
        self
    }

    /// Adds a query parameter.
    pub fn with_query_param(mut self, name: impl Into<String>, value: impl ToString) -> Self {
        self.query_params.push(QueryParam::new(name, value));
        self
    }

    /// Sets a JSON body from a serializable value.
    pub fn with_json_body<T: Serialize>(mut self, value: &T) -> Result<Self, JorttError> {
        self.body = Some(serde_json::to_value(value).map_err(JorttError::from_serialize)?);
        Ok(self)
    }

    /// Sets a JSON body directly.
    pub fn with_body_value(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }

    /// Sets an explicit `Accept` header value.
    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }
}

/// Canonical request builder for all operations.
#[derive(Debug, Clone, Default)]
pub struct RequestBuilder {
    request: OperationRequest,
}

impl RequestBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a path parameter.
    pub fn path_param(mut self, name: impl Into<String>, value: impl ToString) -> Self {
        self.request.path_params.push(PathParam::new(name, value));
        self
    }

    /// Adds a query parameter.
    pub fn query_param(mut self, name: impl Into<String>, value: impl ToString) -> Self {
        self.request.query_params.push(QueryParam::new(name, value));
        self
    }

    /// Sets a JSON request body.
    pub fn json_body<T: Serialize>(mut self, value: &T) -> Result<Self, JorttError> {
        self.request.body = Some(serde_json::to_value(value).map_err(JorttError::from_serialize)?);
        Ok(self)
    }

    /// Sets a JSON body directly from a serde value.
    pub fn body_value(mut self, value: Value) -> Self {
        self.request.body = Some(value);
        self
    }

    /// Sets an explicit `Accept` header.
    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.request.accept = Some(accept.into());
        self
    }

    /// Finalizes the request.
    pub fn build(self) -> OperationRequest {
        self.request
    }
}

/// Generic API executor for a typed operation enum.
#[derive(Clone)]
pub struct DomainApi<O> {
    client: JorttClient,
    _marker: PhantomData<O>,
}

impl<O> DomainApi<O> {
    pub(crate) fn new(client: JorttClient) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

impl<O: TypedOperation> DomainApi<O> {
    /// Executes an operation with the provided request payload and returns raw JSON.
    pub async fn execute(
        &self,
        operation: O,
        request: OperationRequest,
    ) -> Result<Value, JorttError> {
        self.client.execute_spec(operation.spec(), request).await
    }
}
