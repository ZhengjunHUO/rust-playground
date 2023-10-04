use std::env;
use viperus::{Format, Viperus};

fn main() {
    let path_to_file = env::args()
        .nth(1)
        .expect("Wait for a path to config file !");

    let mut vip = Viperus::new();
    vip.load_file(&path_to_file, Format::YAML).unwrap();
    println!("Viper content: {:?}", vip);

    /*
    let s: String = vip.get("clickhouse.endpoints.shard").unwrap();
    println!("{}", s);

    let b = vip.get::<bool>("clickhouse.insecure_connection").unwrap();
    println!("{}", b);

    let i = vip.get::<i32>("retry").unwrap();
    println!("{}", i);
    */
}
