use std::sync::Arc;

use jortt::{
    CreateInvoiceRequest, InvoiceLineItemRequest, JorttClient, ListCustomersQuery,
    ListInvoicesQuery, Money, OAuthClient, OAuthConfig, Scope, StaticAccessToken,
    UpsertCustomerRequest,
};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("JORTT_BASE_URL")
        .ok()
        .map(|raw| Url::parse(&raw))
        .transpose()?;

    let token = resolve_access_token().await?;

    let mut builder =
        JorttClient::builder().with_token_source(Arc::new(StaticAccessToken::new(token)));
    if let Some(base_url) = base_url {
        builder = builder.with_base_url(base_url);
    }
    let client = builder.build()?;

    let customers = client
        .list_customers(&ListCustomersQuery {
            query: Some("a".repeat(3)),
            page: Some(1),
        })
        .await?;
    println!("list_customers -> {}", customers.data.len());

    if let Ok(customer_id) = std::env::var("JORTT_PARAM_CUSTOMER_ID") {
        let customer = client.get_customer(&customer_id).await?;
        println!("get_customer -> {}", customer.customer_name);
    }

    let invoices = client
        .list_invoices(&ListInvoicesQuery {
            query: None,
            invoice_status: None,
            page: Some(1),
        })
        .await?;
    println!("list_invoices -> {}", invoices.data.len());

    if let Ok(invoice_id) = std::env::var("JORTT_PARAM_INVOICE_ID") {
        let invoice = client.get_invoice(&invoice_id).await?;
        println!("get_invoice -> {}", invoice.id);

        let pdf = client.download_invoice_pdf_url(&invoice_id).await?;
        println!("download_invoice_pdf_url -> {}", pdf.download_location);
    }

    let allow_writes = std::env::var("JORTT_ALLOW_WRITES")
        .ok()
        .map(|value| value.eq_ignore_ascii_case("true") || value == "1")
        .unwrap_or(false);

    if allow_writes {
        let created = client
            .create_customer(&UpsertCustomerRequest {
                is_private: false,
                customer_name: format!("SDK Live Example {}", chrono::Utc::now().timestamp()),
                address_street: Some("Transistorstraat 71C".to_string()),
                address_postal_code: Some("1322CK".to_string()),
                address_city: Some("Almere".to_string()),
                address_country_code: Some("NL".to_string()),
                attn: None,
                email: None,
                cc_emails: None,
                payment_term: Some(14),
            })
            .await?;
        println!("create_customer -> {}", created.id);

        let invoice = client
            .create_invoice(&CreateInvoiceRequest {
                customer_id: created.id.clone(),
                invoice_date: chrono::Utc::now().date_naive().to_string(),
                delivery_period: chrono::Utc::now().date_naive().to_string(),
                reference: Some("sdk-live-workflow".to_string()),
                payment_term: Some(14),
                net_amounts: Some(true),
                send_method: Some("self".to_string()),
                introduction: Some("Created by jortt-rs typed_live_workflow example".to_string()),
                remarks: Some("example".to_string()),
                payment_method: Some("pay_later".to_string()),
                line_items: vec![InvoiceLineItemRequest {
                    description: "SDK example line".to_string(),
                    vat: 21,
                    ledger_account_id: None,
                    units: 1,
                    amount_per_unit: Money {
                        value: "121.00".to_string(),
                        currency: "EUR".to_string(),
                    },
                }],
            })
            .await?;

        println!("create_invoice -> {}", invoice.id);
    }

    Ok(())
}

async fn resolve_access_token() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(token) = std::env::var("JORTT_ACCESS_TOKEN") {
        if !token.trim().is_empty() {
            return Ok(token);
        }
    }

    let client_id = std::env::var("JORTT_CLIENT_ID")?;
    let client_secret = std::env::var("JORTT_CLIENT_SECRET")?;

    let oauth = OAuthClient::new(OAuthConfig::default())?;
    let token_set = oauth
        .exchange_client_credentials(
            &client_id,
            &client_secret,
            &[
                Scope::CustomersRead,
                Scope::CustomersWrite,
                Scope::InvoicesRead,
                Scope::InvoicesWrite,
            ],
        )
        .await?;

    Ok(token_set.access_token)
}
