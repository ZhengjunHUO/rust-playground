#![allow(dead_code)]

use config_file::FromConfigFile;
use regex::Regex;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    core: CoreBM,
    global_parameters: GlobalParams,
}

#[derive(Deserialize, Debug)]
struct CoreBM {
    clickhouse: CHComponentBM,
    postgresql: PQComponentBM,
}

#[derive(Deserialize, Debug)]
struct CHComponentBM {
    inventory: Inventory,
    ports: PortsBM,
    ssl: SSL,
}

#[derive(Deserialize, Debug)]
struct PQComponentBM {
    inventory: Inventory,
    ports: PortsBM,
    ssl: SSL,
    secrets: SecretsBM,
}

#[derive(Deserialize, Debug)]
pub struct Inventory {
    host: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct PortsBM {
    port: u32,
    tcp: u32,
}

#[derive(Deserialize, Debug)]
struct SecretsBM {
    user: String,
    password: String,
    db_name: String,
}

#[derive(Deserialize, Debug)]
struct SSL {
    enable: bool,
}

#[derive(Deserialize, Debug)]
struct GlobalParams {
    include: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CliConfigBMExtra {
    core_svc_configs: CoreBMConfig,
}

#[derive(Deserialize, Debug)]
pub struct CoreBMConfig {
    clickhouse_replication_factor: u32,
}

fn main() {
    let path = env::args().nth(1).expect("Need a path to config file");
    match Config::from_config_file(&path) {
        Ok(conf) => {
            println!("Config file's content:\n    {:?}", conf);
            let included_config_path_stub;
            match path.rsplit_once('/') {
                Some((stub, _)) => match stub.rsplit_once('/') {
                    Some((trunk, _)) => included_config_path_stub = String::from(trunk),
                    None => included_config_path_stub = String::from(stub),
                },
                None => included_config_path_stub = path.clone(),
            }
            println!("{}", included_config_path_stub);

            let mut included_config_path = String::default();
            let rx = Regex::new(".*core.*").unwrap();
            for s in conf.global_parameters.include.iter() {
                match rx.find(s) {
                    Some(f) => {
                        included_config_path =
                            format!("{}/{}", included_config_path_stub, f.as_str())
                    }
                    None => (),
                }
            }
            println!("{}", included_config_path);

            match CliConfigBMExtra::from_config_file(&included_config_path) {
                Ok(extra) => {
                    println!("{:?}", extra);
                }
                Err(e) => println!(
                    "Error occurred when reading the included config {}: {:?}",
                    included_config_path, e
                ),
            }
        }
        Err(e) => println!("Error occurred when reading the config: {:?}", e),
    }
}
