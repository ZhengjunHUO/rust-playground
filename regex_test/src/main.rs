use anyhow::Result;
use regex::Regex;
use std::collections::hash_set::HashSet;
use test_macro::vec_string;

fn main() -> Result<()> {
    let raw = HashSet::<String>::from_iter(vec_string![
        "foo",
        "default",
        "shard_foo",
        "shard_bar",
        "baz_shard_bar",
        "monitoring",
        "foobar"
    ]);
    //let filters = vec_string!["foo"];
    let filters = vec_string!["shard_.*"];

    for filter in filters {
        let rx = Regex::new(&filter)?;

        for n in raw.iter() {
            match rx.find(n) {
                Some(f) => {
                    println!("{:?}", f);
                    println!("{}", n.len() == f.as_str().len());
                }
                None => continue,
            }
        }
    }

    Ok(())
}
