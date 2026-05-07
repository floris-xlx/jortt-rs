use std::collections::HashMap;
use std::sync::Arc;

use jortt::{
    ALL_OPERATION_SPECS, HttpMethod, JorttClient, OAuthClient, OAuthConfig, RequestBuilder,
    Scope, StaticAccessToken,
};
use serde_json::Value;
use url::Url;

#[tokio::test]
#[ignore = "requires real credentials and explicit opt-in (`JORTT_E2E_LIVE=true`)"]
async fn live_read_only_workflow_executes_core_paths() -> Result<(), Box<dyn std::error::Error>> {
    if !env_bool("JORTT_E2E_LIVE") {
        eprintln!("skipping: set JORTT_E2E_LIVE=true to execute this test");
        return Ok(());
    }

    let client = build_live_client().await?;

    let customers = client
        .list_customers(&jortt::ListCustomersQuery {
            query: Some("acm".to_string()),
            page: Some(1),
        })
        .await?;
    assert!(
        customers
            .data
            .iter()
            .all(|customer| !customer.id.is_empty())
    );

    let invoices = client
        .list_invoices(&jortt::ListInvoicesQuery {
            query: None,
            invoice_status: None,
            page: Some(1),
        })
        .await?;
    assert!(invoices.data.iter().all(|invoice| !invoice.id.is_empty()));

    let organization = client
        .methods()
        .organizations()
        .get_organizations_me(RequestBuilder::new().build())
        .await?;
    assert!(organization.is_object());

    if let Ok(customer_id) = std::env::var("JORTT_PARAM_CUSTOMER_ID") {
        let customer = client.get_customer(&customer_id).await?;
        assert_eq!(customer.id, customer_id);
    }

    if let Ok(invoice_id) = std::env::var("JORTT_PARAM_INVOICE_ID") {
        let invoice = client.get_invoice(&invoice_id).await?;
        assert_eq!(invoice.id, invoice_id);

        let download = client.download_invoice_pdf_url(&invoice_id).await?;
        assert!(!download.download_location.trim().is_empty());
    }

    Ok(())
}

#[tokio::test]
#[ignore = "full live OpenAPI smoke; requires `JORTT_E2E_LIVE=true` and `JORTT_E2E_FULL_SMOKE=true`"]
async fn live_openapi_catalog_smoke_runs_without_failures() -> Result<(), Box<dyn std::error::Error>>
{
    if !env_bool("JORTT_E2E_LIVE") || !env_bool("JORTT_E2E_FULL_SMOKE") {
        eprintln!(
            "skipping: set JORTT_E2E_LIVE=true and JORTT_E2E_FULL_SMOKE=true to execute this test"
        );
        return Ok(());
    }

    let client = build_live_client().await?;
    let run_mutations = env_bool("JORTT_RUN_MUTATIONS");

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    for spec in ALL_OPERATION_SPECS {
        if is_mutation(spec.method) && !run_mutations {
            skipped += 1;
            continue;
        }

        let path_param_names = extract_path_param_names(spec.path);
        let mut path_params = HashMap::new();
        let mut missing_path_param = None;
        for name in path_param_names {
            let env_key = format!("JORTT_PARAM_{}", to_env_segment(&name));
            match std::env::var(&env_key) {
                Ok(value) => {
                    path_params.insert(name, value);
                }
                Err(_) => {
                    missing_path_param = Some(env_key);
                    break;
                }
            }
        }

        if missing_path_param.is_some() {
            skipped += 1;
            continue;
        }

        let key = operation_env_key(spec.method, spec.path);
        let query = std::env::var(format!("JORTT_QUERY_{key}")).ok();
        let body = std::env::var(format!("JORTT_BODY_{key}")).ok();
        let accept = std::env::var(format!("JORTT_ACCEPT_{key}")).ok();

        let mut request_builder = RequestBuilder::new();
        for (name, value) in path_params {
            request_builder = request_builder.path_param(name, value);
        }

        if let Some(query_pairs) = query {
            for (name, value) in parse_query_pairs(&query_pairs) {
                request_builder = request_builder.query_param(name, value);
            }
        }

        if let Some(body_json) = body {
            let parsed: Value = serde_json::from_str(&body_json)?;
            request_builder = request_builder.body_value(parsed);
        }

        if let Some(accept) = accept {
            request_builder = request_builder.accept(accept);
        }

        let request = request_builder.build();
        match client.raw().execute(spec.method, spec.path, request).await {
            Ok(_) => passed += 1,
            Err(_) => failed += 1,
        }
    }

    assert_eq!(
        failed,
        0,
        "live OpenAPI smoke failures: total={} passed={} failed={} skipped={}",
        ALL_OPERATION_SPECS.len(),
        passed,
        failed,
        skipped
    );

    Ok(())
}

async fn build_live_client() -> Result<JorttClient, Box<dyn std::error::Error>> {
    let token = resolve_access_token().await?;

    let mut builder =
        JorttClient::builder().with_token_source(Arc::new(StaticAccessToken::new(token)));
    if let Some(base_url) = std::env::var("JORTT_BASE_URL")
        .ok()
        .map(|raw| Url::parse(&raw))
        .transpose()?
    {
        builder = builder.with_base_url(base_url);
    }

    Ok(builder.build()?)
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
                Scope::EstimatesRead,
                Scope::EstimatesWrite,
                Scope::ExpensesRead,
                Scope::ExpensesWrite,
                Scope::FinancingRead,
                Scope::InboxWrite,
                Scope::InvoicesRead,
                Scope::InvoicesWrite,
                Scope::OrganizationsRead,
                Scope::OrganizationsWrite,
                Scope::PayrollRead,
                Scope::PayrollWrite,
                Scope::ReportsRead,
            ],
        )
        .await?;

    Ok(token_set.access_token)
}

fn is_mutation(method: HttpMethod) -> bool {
    matches!(
        method,
        HttpMethod::Post | HttpMethod::Put | HttpMethod::Delete | HttpMethod::Patch
    )
}

fn env_bool(key: &str) -> bool {
    std::env::var(key)
        .map(|value| value.eq_ignore_ascii_case("true") || value == "1")
        .unwrap_or(false)
}

fn parse_query_pairs(raw: &str) -> Vec<(String, String)> {
    raw.split('&')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            if let Some((key, value)) = segment.split_once('=') {
                (key.to_string(), value.to_string())
            } else {
                (segment.to_string(), String::new())
            }
        })
        .collect()
}

fn extract_path_param_names(path: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut current = String::new();
    let mut in_braces = false;

    for ch in path.chars() {
        match ch {
            '{' => {
                in_braces = true;
                current.clear();
            }
            '}' => {
                if in_braces && !current.is_empty() {
                    names.push(current.clone());
                }
                in_braces = false;
                current.clear();
            }
            _ => {
                if in_braces {
                    current.push(ch);
                }
            }
        }
    }

    names
}

fn to_env_segment(input: &str) -> String {
    let mut result = String::new();
    let mut last_was_underscore = false;

    for ch in input.chars() {
        let normalized = if ch.is_ascii_alphanumeric() {
            ch.to_ascii_uppercase()
        } else {
            '_'
        };

        if normalized == '_' {
            if !last_was_underscore {
                result.push('_');
                last_was_underscore = true;
            }
        } else {
            result.push(normalized);
            last_was_underscore = false;
        }
    }

    result.trim_matches('_').to_string()
}

fn operation_env_key(method: HttpMethod, path: &str) -> String {
    format!("{}_{}", method.as_str(), to_env_segment(path))
}
