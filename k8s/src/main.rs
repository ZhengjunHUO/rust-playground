use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_namespace = String::from("opensee-chouse-install");
    let target_svc = String::from("clickhouse.altinity.com/Service=host");
    let client = Client::try_default().await?;

    //let pods: Api<Pod> = Api::default_namespaced(client);
    //let pods: Api<Pod> = Api::namespaced(client, &target_namespace);
    let svc: Api<Service> = Api::namespaced(client, &target_namespace);
    let lp = ListParams::default().labels(&target_svc);
    //for s in svc.list(&ListParams::default()).await? {
    for s in svc.list(&lp).await? {
        println!("service: [{}]", s.name_any());
    }
    Ok(())
}
