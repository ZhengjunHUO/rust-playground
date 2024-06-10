use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_namespace = String::from("opensee-chouse-install");
    let target_svc_label = String::from("clickhouse.altinity.com/Service=host");
    let target_suffix = String::from("svc.cluster.local:8123");

    let client = Client::try_default().await?;
    let svc_list = probe_eps(client, &target_namespace, &target_svc_label, &target_suffix).await?;
    let result = group_eps(svc_list);
    println!("{:?}", result);
    Ok(())
}

fn group_eps(nodes: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for node in nodes.into_iter() {
        if let Some(group_id) = node.split('.').next().and_then(|s| s.rsplit('-').nth(1)) {
            groups.entry(group_id.to_string()).or_default().push(node);
        }
    }
    //println!("[DEBUG]{:?}", groups);

    let mut result: Vec<Vec<String>> = groups.into_values().collect();
    result.sort_by_key(|group| group[0].clone());

    result
}

async fn probe_eps(
    client: Client,
    target_namespace: &str,
    target_svc_label: &str,
    target_suffix: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let svc: Api<Service> = Api::namespaced(client, target_namespace);
    let lp = ListParams::default().labels(target_svc_label);
    Ok(svc
        .list(&lp)
        .await?
        .iter()
        .map(|s| format!("{}.{}.{}", s.name_any(), target_namespace, target_suffix))
        .collect::<Vec<String>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_eps() {
        let eps = vec![
            String::from(
                "chi-statefulset-standard-0-0.opensee-chouse-install.svc.cluster.local:8123",
            ),
            String::from(
                "chi-statefulset-standard-0-1.opensee-chouse-install.svc.cluster.local:8123",
            ),
            String::from(
                "chi-statefulset-standard-1-0.opensee-chouse-install.svc.cluster.local:8123",
            ),
            String::from(
                "chi-statefulset-standard-1-1.opensee-chouse-install.svc.cluster.local:8123",
            ),
        ];
        let result = group_eps(eps);
        let expected = vec![
            vec![
                String::from(
                    "chi-statefulset-standard-0-0.opensee-chouse-install.svc.cluster.local:8123",
                ),
                String::from(
                    "chi-statefulset-standard-0-1.opensee-chouse-install.svc.cluster.local:8123",
                ),
            ],
            vec![
                String::from(
                    "chi-statefulset-standard-1-0.opensee-chouse-install.svc.cluster.local:8123",
                ),
                String::from(
                    "chi-statefulset-standard-1-1.opensee-chouse-install.svc.cluster.local:8123",
                ),
            ],
        ];
        assert_eq!(result, expected);
    }
}
