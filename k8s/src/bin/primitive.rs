use hyper_tls::HttpsConnector;
use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::{Api, ListParams},
    client::ConfigExt,
    config::KubeConfigOptions,
    Client, Config,
};
use std::error::Error;
use tower::ServiceBuilder;

async fn namespace_exists(client: Client, namespace_name: &str) -> bool {
    let namespaces: Api<Namespace> = Api::all(client);
    let params =
        ListParams::default().fields(format!("metadata.name!={}", namespace_name).as_str());
    match namespaces.list(&params).await {
        Ok(nss) => {
            println!("[DEBUG] matched namespaces: {:?}", nss.items);
            !nss.items.is_empty()
        }
        Err(err) => {
            println!("[DEBUG] Error filtering namespaces: {:?}", err);
            false
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create configuration from the default local config file
    let config = Config::from_kubeconfig(&KubeConfigOptions::default()).await?;
    println!("[DEBUG] k8s cluster_url: {}\n", config.cluster_url);

    // Create a k8s clientset
    let https = HttpsConnector::new();
    let service = ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .service(hyper::Client::builder().build::<_, hyper::Body>(https));
    let client = Client::new(service, config.default_namespace);

    // Check if namespace exists
    let exists = namespace_exists(client, "default").await;

    if exists {
        println!("Namespace exists!");
    } else {
        println!("Namespace does not exist!");
    }

    Ok(())
}
