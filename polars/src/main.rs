use color_eyre::Result;
use polars::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let df = df! [
        "id"      => [1, 2, 3, 4, 5],
        "env"     => ["k8s", "k8s", "baremetal", "baremetal", "k8s"],
        "plugin"  => ["cilium", "default", "cilium", "cilium", "cilium"],
        "size"    => [6, 8, 32, 12, 9],
        "coef"    => [Some(-10), Some(120), None, Some(25), Some(-40)],
    ]?;
    println!("Data frame: {:?}\n", df);

    let sel = df
        .clone()
        .lazy()
        .select([col("id"), lit("env"), col("env")])
        .collect()?;
    println!("Selected columns: {:?}\n", sel);

    Ok(())
}
