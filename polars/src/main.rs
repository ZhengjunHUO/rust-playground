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

    let sel_reg = df
        .clone()
        .lazy()
        .select([col("^id|size$").sum()])
        .collect()?;
    println!("Select using regex: {:?}\n", sel_reg);

    let sel_mult = df
        .clone()
        .lazy()
        .select([cols(["id", "coef"]).sum()])
        .collect()?;
    println!("Select multiple cols: {:?}\n", sel_mult);

    let sel_rev = df
        .clone()
        .lazy()
        .select([col("id"), all().reverse().suffix("_rev")])
        .collect()?;
    println!("Select reverse: {:?}\n", sel_rev);

    let pred = col("size").gt(10);

    let sel_pred = df.clone().lazy().select([pred.clone()]).collect()?;
    println!("Select predict: {:?}\n", sel_pred);

    let sel_filter = df.clone().lazy().filter(pred).collect()?;
    println!("Select filtered with predict: {:?}\n", sel_filter);

    Ok(())
}
