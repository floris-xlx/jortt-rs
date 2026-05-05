use jortt::{OAuthClient, OAuthConfig, Scope};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                Scope::ReportsRead,
                Scope::OrganizationsRead,
            ],
        )
        .await?;

    println!("access_token: {}", token_set.access_token);
    println!("token_type: {}", token_set.token_type);
    println!(
        "refresh_token: {}",
        token_set.refresh_token.as_deref().unwrap_or("<none>")
    );
    println!(
        "expires_in: {}",
        token_set
            .expires_in
            .map(|v| v.to_string())
            .unwrap_or_else(|| "<none>".to_string())
    );
    println!("scope: {}", token_set.scope.as_deref().unwrap_or("<none>"));

    Ok(())
}
