use config_file::FromConfigFile;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Config {
    podName: String,
    ingressRules: Rules,
    egressRules: Rules,
}

#[derive(Deserialize, Debug)]
struct Rules {
    l3: Vec<String>,
    l4: Vec<L4Entry>,
}

#[derive(Deserialize, Debug)]
struct L4Entry {
    ip: String,
    port: u16,
}

fn main() {
    let conf = Config::from_config_file("config/conf.yaml").unwrap();
    println!("Config file's content: {:?}", conf);
}
