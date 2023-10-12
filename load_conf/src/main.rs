#![allow(dead_code)]

use config_file::FromConfigFile;
use serde::{Deserialize, Serialize};
use serde_valid::validation::Error::Custom;
use serde_valid::Validate;

fn odd_only(val: &u8) -> Result<(), serde_valid::validation::Error> {
    if val % 2 == 0 {
        return Err(Custom(String::from("Only odd number is accepted !")));
    }

    Ok(())
}

#[derive(Deserialize, Debug, Serialize, Validate)]
struct Config {
    #[validate(
        pattern = r"^([a-zA-Z0-9-]+-)+[a-zA-Z0-9]+$",
        message = "The full name should be like: <CLIENT-NAME>-<PROJECT-NAME>"
    )]
    full_name: String,
    alias: Option<String>,
    #[validate(custom(odd_only))]
    serial_no: u8,
    #[validate]
    ingress_rules: Rules,
    #[validate]
    egress_rules: Rules,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
struct Rules {
    #[validate(min_items = 1)]
    l3: Vec<String>,
    #[validate(min_items = 1)]
    l4: Vec<L4Entry>,
}

#[derive(Deserialize, Debug, Serialize)]
struct L4Entry {
    ip: String,
    port: u16,
}

fn main() {
    match Config::from_config_file("config/conf.yaml") {
        Ok(conf) => {
            if let Err(e) = conf.validate() {
                println!("Validator: {}", e.to_string());
            } else {
                println!("Config file's content:\n\n{}", serde_yaml::to_string(&conf).unwrap());
            }
        }
        Err(e) => println!("Error occurred when reading the config: {:?}", e),
    }
}
