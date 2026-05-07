use std::sync::Arc;

use httpmock::Method::{DELETE, GET, POST, PUT};
use httpmock::MockServer;
use jortt::{
    CreateInvoiceRequest, HttpMethod, InvoiceLineItemRequest, JorttClient, ListCustomersQuery,
    ListInvoicesQuery, Money, OperationRequest, RequestBuilder, StaticAccessToken,
    UpsertCustomerRequest,
};
use serde_json::json;
use url::Url;

const TOKEN: &str = "test-token";
const CUSTOMER_ID: &str = "408d4652-b07a-4195-817e-0390bb0c9428";
const INVOICE_ID: &str = "b3b0b7c4-9dab-4ca7-b1c0-16b0ebbb75f7";

fn test_client(base_url: Url) -> JorttClient {
    JorttClient::builder()
        .with_base_url(base_url)
        .with_token_source(Arc::new(StaticAccessToken::new(TOKEN)))
        .build()
        .expect("client should build")
}

fn customer_payload(name: &str) -> serde_json::Value {
    json!({
        "id": CUSTOMER_ID,
        "is_private": false,
        "customer_name": name,
        "address_street": "Transistorstraat 71C",
        "address_postal_code": "1322CK",
        "address_city": "Almere",
        "address_country_code": "NL",
        "email": "sdk-e2e@example.com"
    })
}

fn invoice_payload(status: &str) -> serde_json::Value {
    json!({
        "id": INVOICE_ID,
        "customer_id": CUSTOMER_ID,
        "invoice_number": "2026-0001",
        "invoice_status": status
    })
}

#[tokio::test]
async fn sdk_end_to_end_flow_is_wired_across_typed_generated_and_raw_paths() {
    let server = MockServer::start_async().await;

    let list_customers = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/customers")
                .header("authorization", "Bearer test-token")
                .query_param("query", "acm")
                .query_param("page", "1");
            then.status(200)
                .json_body(json!({ "data": [customer_payload("Acme Seed")] }));
        })
        .await;

    let create_customer = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/customers")
                .header("authorization", "Bearer test-token");
            then.status(201)
                .json_body(json!({ "data": customer_payload("Acme Created") }));
        })
        .await;

    let get_customer = server
        .mock_async(|when, then| {
            when.method(GET)
                .path(format!("/customers/{CUSTOMER_ID}"))
                .header("authorization", "Bearer test-token");
            then.status(200)
                .json_body(json!({ "data": customer_payload("Acme Created") }));
        })
        .await;

    let update_customer = server
        .mock_async(|when, then| {
            when.method(PUT)
                .path(format!("/customers/{CUSTOMER_ID}"))
                .header("authorization", "Bearer test-token");
            then.status(200)
                .json_body(json!({ "data": customer_payload("Acme Updated") }));
        })
        .await;

    let delete_customer = server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path(format!("/customers/{CUSTOMER_ID}"))
                .header("authorization", "Bearer test-token");
            then.status(200)
                .json_body(json!({ "data": customer_payload("Acme Updated") }));
        })
        .await;

    let list_invoices = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/invoices")
                .header("authorization", "Bearer test-token")
                .query_param("page", "1");
            then.status(200)
                .json_body(json!({ "data": [invoice_payload("draft")] }));
        })
        .await;

    let create_invoice = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/invoices")
                .header("authorization", "Bearer test-token");
            then.status(201)
                .json_body(json!({ "data": invoice_payload("open") }));
        })
        .await;

    let get_invoice = server
        .mock_async(|when, then| {
            when.method(GET)
                .path(format!("/invoices/{INVOICE_ID}"))
                .header("authorization", "Bearer test-token");
            then.status(200)
                .json_body(json!({ "data": invoice_payload("open") }));
        })
        .await;

    let get_invoice_download = server
        .mock_async(|when, then| {
            when.method(GET)
                .path(format!("/invoices/{INVOICE_ID}/download"))
                .header("authorization", "Bearer test-token");
            then.status(200).json_body(json!({
                "data": {
                    "download_location": "https://files.example.test/invoice.pdf"
                }
            }));
        })
        .await;

    let list_ledger_accounts = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/ledger_accounts/invoices")
                .header("authorization", "Bearer test-token");
            then.status(200).json_body(json!({
                "data": [
                    { "id": "revenue", "name": "Revenue" },
                    { "id": "services", "name": "Services" }
                ]
            }));
        })
        .await;

    let get_organizations_me = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/organizations/me")
                .header("authorization", "Bearer test-token");
            then.status(200).json_body(json!({
                "data": { "id": "org-123", "name": "Acme Org" }
            }));
        })
        .await;

    let get_v1_organizations_me = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/organizations/me")
                .header("authorization", "Bearer test-token");
            then.status(200).json_body(json!({
                "data": { "id": "org-123", "name": "Acme Org" }
            }));
        })
        .await;

    let client = test_client(Url::parse(&server.url("/")).expect("URL should parse"));

    let customers = client
        .list_customers(&ListCustomersQuery {
            query: Some("acm".to_string()),
            page: Some(1),
        })
        .await
        .expect("list_customers should succeed");
    assert_eq!(customers.data[0].customer_name, "Acme Seed");

    let created_customer = client
        .create_customer(&UpsertCustomerRequest {
            is_private: false,
            customer_name: "Acme Created".to_string(),
            address_street: Some("Transistorstraat 71C".to_string()),
            address_postal_code: Some("1322CK".to_string()),
            address_city: Some("Almere".to_string()),
            address_country_code: Some("NL".to_string()),
            attn: None,
            email: Some("sdk-e2e@example.com".to_string()),
            cc_emails: None,
            payment_term: Some(14),
        })
        .await
        .expect("create_customer should succeed");
    assert_eq!(created_customer.id, CUSTOMER_ID);

    let fetched_customer = client
        .get_customer(CUSTOMER_ID)
        .await
        .expect("get_customer should succeed");
    assert_eq!(fetched_customer.customer_name, "Acme Created");

    let updated_customer = client
        .update_customer(
            CUSTOMER_ID,
            &UpsertCustomerRequest {
                is_private: false,
                customer_name: "Acme Updated".to_string(),
                address_street: Some("Transistorstraat 71C".to_string()),
                address_postal_code: Some("1322CK".to_string()),
                address_city: Some("Almere".to_string()),
                address_country_code: Some("NL".to_string()),
                attn: None,
                email: Some("sdk-e2e@example.com".to_string()),
                cc_emails: None,
                payment_term: Some(14),
            },
        )
        .await
        .expect("update_customer should succeed");
    assert_eq!(updated_customer.customer_name, "Acme Updated");

    let invoices = client
        .list_invoices(&ListInvoicesQuery {
            query: None,
            invoice_status: None,
            page: Some(1),
        })
        .await
        .expect("list_invoices should succeed");
    assert_eq!(invoices.data[0].id, INVOICE_ID);

    let created_invoice = client
        .create_invoice(&CreateInvoiceRequest {
            customer_id: CUSTOMER_ID.to_string(),
            invoice_date: "2026-05-01".to_string(),
            delivery_period: "2026-05-01".to_string(),
            reference: Some("sdk-e2e".to_string()),
            payment_term: Some(14),
            net_amounts: Some(true),
            send_method: Some("self".to_string()),
            introduction: Some("Created by test suite".to_string()),
            remarks: Some("e2e".to_string()),
            payment_method: Some("pay_later".to_string()),
            line_items: vec![InvoiceLineItemRequest {
                description: "SDK e2e line".to_string(),
                vat: 21,
                ledger_account_id: None,
                units: 1,
                amount_per_unit: Money {
                    value: "121.00".to_string(),
                    currency: "EUR".to_string(),
                },
            }],
        })
        .await
        .expect("create_invoice should succeed");
    assert_eq!(created_invoice.invoice_status.as_deref(), Some("open"));

    let fetched_invoice = client
        .get_invoice(INVOICE_ID)
        .await
        .expect("get_invoice should succeed");
    assert_eq!(fetched_invoice.id, INVOICE_ID);

    let pdf = client
        .download_invoice_pdf_url(INVOICE_ID)
        .await
        .expect("download_invoice_pdf_url should succeed");
    assert_eq!(
        pdf.download_location,
        "https://files.example.test/invoice.pdf"
    );

    let ledger_accounts = client
        .list_invoice_ledger_accounts()
        .await
        .expect("list_invoice_ledger_accounts should succeed");
    assert_eq!(ledger_accounts.len(), 2);

    let organization_via_generated = client
        .methods()
        .organizations()
        .get_organizations_me(RequestBuilder::new().build())
        .await
        .expect("generated organizations call should succeed");
    assert_eq!(organization_via_generated["data"]["id"], json!("org-123"));

    let organization_via_raw = client
        .raw()
        .execute(
            HttpMethod::Get,
            "/v1/organizations/me",
            OperationRequest::new(),
        )
        .await
        .expect("raw organizations call should succeed");
    assert_eq!(organization_via_raw["data"]["id"], json!("org-123"));

    let deleted_customer = client
        .delete_customer(CUSTOMER_ID)
        .await
        .expect("delete_customer should succeed");
    assert_eq!(deleted_customer.customer_name, "Acme Updated");

    list_customers.assert_async().await;
    create_customer.assert_async().await;
    get_customer.assert_async().await;
    update_customer.assert_async().await;
    delete_customer.assert_async().await;
    list_invoices.assert_async().await;
    create_invoice.assert_async().await;
    get_invoice.assert_async().await;
    get_invoice_download.assert_async().await;
    list_ledger_accounts.assert_async().await;
    get_organizations_me.assert_async().await;
    get_v1_organizations_me.assert_async().await;
}
