use std::sync::Arc;

use httpmock::Method::{DELETE, GET, POST};
use httpmock::MockServer;
use jortt::{
    ApiError, ErrorBuilder, HttpMethod, JorttClient, JorttError, ListCustomersQuery,
    OperationRequest, RequestBuilder, StaticAccessToken, UpsertCustomerRequest,
};
use serde_json::json;
use url::Url;

fn test_client(base_url: Url) -> JorttClient {
    JorttClient::builder()
        .with_base_url(base_url)
        .with_token_source(Arc::new(StaticAccessToken::new("test-token")))
        .build()
        .expect("client should build")
}

#[tokio::test]
async fn create_customer_sends_auth_and_parses_data() {
    let server = MockServer::start_async().await;

    let _mock = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/customers")
                .header("authorization", "Bearer test-token");
            then.status(201).json_body(json!({
                "data": {
                    "id": "408d4652-b07a-4195-817e-0390bb0c9428",
                    "is_private": false,
                    "customer_name": "Jortt B.V.",
                    "address_street": "Transistorstraat 71C",
                    "address_postal_code": "1322CK",
                    "address_city": "Almere",
                    "address_country_code": "NL",
                    "email": "example@email.com"
                }
            }));
        })
        .await;

    let client = test_client(Url::parse(&server.url("/")).expect("URL should parse"));

    let customer = client
        .create_customer(&UpsertCustomerRequest {
            is_private: false,
            customer_name: "Jortt B.V.".to_string(),
            address_street: None,
            address_postal_code: None,
            address_city: None,
            address_country_code: None,
            attn: None,
            email: None,
            cc_emails: None,
            payment_term: None,
        })
        .await
        .expect("customer should parse");

    assert_eq!(customer.customer_name, "Jortt B.V.");
}

#[tokio::test]
async fn list_customers_serializes_query() {
    let server = MockServer::start_async().await;

    let _mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/customers")
                .query_param("query", "janssen")
                .query_param("page", "2");
            then.status(200).json_body(json!({
                "data": [],
                "_links": {}
            }));
        })
        .await;

    let client = test_client(Url::parse(&server.url("/")).expect("URL should parse"));

    let response = client
        .list_customers(&ListCustomersQuery {
            query: Some("janssen".to_string()),
            page: Some(2),
        })
        .await
        .expect("response should parse");

    assert_eq!(response.data.len(), 0);
}

#[tokio::test]
async fn api_error_payload_is_preserved() {
    let server = MockServer::start_async().await;

    let _mock = server
        .mock_async(|when, then| {
            when.method(DELETE).path("/customers/invalid");
            then.status(422).json_body(json!({
                "error": {
                    "code": 422,
                    "key": "params.invalid",
                    "message": "The parameters are invalid.",
                    "details": [{
                        "param": "customer_id",
                        "key": "invalid_format",
                        "message": "must be uuid"
                    }]
                }
            }));
        })
        .await;

    let client = test_client(Url::parse(&server.url("/")).expect("URL should parse"));

    let error = client
        .delete_customer("invalid")
        .await
        .expect_err("request should fail");

    match error {
        JorttError::Api { payload, .. } => {
            assert_eq!(payload.key, "params.invalid");
            assert_eq!(payload.details.len(), 1);
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[tokio::test]
async fn raw_client_returns_missing_path_param() {
    let server = MockServer::start_async().await;
    let client = test_client(Url::parse(&server.url("/")).expect("URL should parse"));

    let err = client
        .raw()
        .execute(
            HttpMethod::Get,
            "/customers/{customer_id}",
            OperationRequest::new(),
        )
        .await
        .expect_err("expected missing path parameter error");

    match err {
        JorttError::MissingPathParam { name, .. } => assert_eq!(name, "customer_id"),
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn api_error_type_is_constructible() {
    let err = ApiError {
        code: 422,
        key: "params.invalid".to_string(),
        message: "invalid".to_string(),
        details: vec![],
    };

    assert_eq!(err.code, 422);
}

#[tokio::test]
async fn request_builder_is_the_single_path_for_generated_methods() {
    let server = MockServer::start_async().await;

    let _mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/customers")
                .query_param("query", "acme");
            then.status(200).json_body(json!({
                "data": []
            }));
        })
        .await;

    let client = test_client(Url::parse(&server.url("/")).expect("URL should parse"));

    let value = client
        .methods()
        .customers()
        .get_customers(RequestBuilder::new().query_param("query", "acme").build())
        .await
        .expect("generated method should execute");

    assert_eq!(value["data"], json!([]));
}

#[test]
fn error_builder_produces_structured_errors() {
    let api_payload = ApiError {
        code: 422,
        key: "params.invalid".to_string(),
        message: "invalid params".to_string(),
        details: vec![],
    };

    let api_error =
        ErrorBuilder::api(reqwest::StatusCode::UNPROCESSABLE_ENTITY, api_payload).build();
    match api_error {
        JorttError::Api {
            status, error_key, ..
        } => {
            assert_eq!(status, reqwest::StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(error_key, "params.invalid");
        }
        other => panic!("unexpected error: {other:?}"),
    }

    let http_error = ErrorBuilder::http(reqwest::StatusCode::BAD_REQUEST)
        .body("bad request")
        .build();
    match http_error {
        JorttError::Http { status, body } => {
            assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(body, "bad request");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
