use anyhow::Result;
use openidconnect::core::*;
use openidconnect::*;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://foo.bar.com/auth/realms/customer".to_string())?,
        &http_client,
    )
    .await?;

    println!("{:?}", provider_metadata.token_endpoint());
    Ok(())
}
