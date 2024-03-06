#![allow(dead_code)]

use config_file::FromConfigFile;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Config {
    core_kube_inventories: Root,
}

#[derive(Deserialize, Debug)]
struct Root {
    rafalapi: RFComponent,
    postgresql: PSQLComponent,
    clickhouse: CHComponent,
}

#[derive(Deserialize, Debug)]
struct RFComponent {
    secret_env: Vec<NameValue>,
    env: Vec<NameValue>,
}

#[derive(Deserialize, Debug)]
struct PSQLComponent {
    name: String,
    namespace: String,
}

#[derive(Deserialize, Debug)]
struct CHComponent {
    shards_count: u32,
    replicas_count: u32,
    name: String,
    namespace: String,
    chouse_cluster_name: String,
    ports: Vec<Ports>,
}

#[derive(Deserialize, Debug)]
struct NameValue {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct Ports {
    port_name: String,
    container_port: u32,
}

fn main() {
    let path = env::args().nth(1).expect("Need a path to config file");
    match Config::from_config_file(path) {
        Ok(conf) => {
            println!("Config file's content:\n    {:?}", conf);
        }
        Err(e) => println!("Error occurred when reading the config: {:?}", e),
    }
}
