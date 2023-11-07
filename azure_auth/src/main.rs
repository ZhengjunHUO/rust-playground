use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredentialBuilder;
use std::{env::var, error::Error};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Need to set env var AZURE_SUBSCRIPTION_ID
    let sub_id = var("AZURE_SUBSCRIPTION_ID")?;
    println!("Subscription id: {}", sub_id);

    // Need also AZURE_TENANT_ID, AZURE_CLIENT_ID, AZURE_CLIENT_SECRET of
    // Microsoft Entra ID / App registrations, which require a Reader role assigned
    let creds = DefaultAzureCredentialBuilder::new()
        .exclude_azure_cli_credential()
        .exclude_managed_identity_credential()
        .build();
    let res = creds
        .get_token("https://management.azure.com/")
        .await
        .unwrap();
    println!("{res:?}");

    /*
    let url = Url::parse(&format!(
                 "https://management.azure.com/subscriptions/{sub_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"
             ))?;
    */

    let rg = var("AZURE_RESSOURCE_GROUP")?;
    println!("Ressource group: {}", rg);

    let c_name = var("AZURE_CLUSTER_NAME")?;
    println!("Cluster name: {}", c_name);

    let url = Url::parse(&format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}?api-version=2023-07-01",
        sub_id,
        rg,
        c_name))?
        .to_string();

    let resp = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", res.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    println!("Get result:\n{}", resp);
    Ok(())
}
