use anyhow::Result;
use regex::Regex;
use std::collections::hash_set::HashSet;

pub fn filter_tables(raw: HashSet<String>, filters: Vec<String>) -> Result<HashSet<String>> {
    let mut rslt = HashSet::new();

    for filter in filters {
        let rx = Regex::new(&filter)?;
        let matched: HashSet<String> = raw
            .iter()
            .filter(|n| match rx.find(n) {
                Some(f) => n.len() == f.as_str().len(),
                None => false,
            })
            .map(|s| s.to_owned())
            .collect();
        rslt = rslt.union(&matched).map(|s| s.to_owned()).collect();
    }

    Ok(rslt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_macro::vec_string;

    #[test]
    fn test_filter_tables() {
        assert_eq!(
            filter_tables(
                HashSet::from_iter(vec_string![
                    "default",
                    "shard_foo",
                    "shard_bar",
                    "monitoring",
                    "foobar"
                ]),
                vec_string!["^shard_.*", ".*foo.*"]
            )
            .unwrap(),
            HashSet::from_iter(vec_string!["shard_foo", "foobar", "shard_bar"])
        );
    }

    #[test]
    fn test_filter_tables_another() {
        assert_eq!(
            filter_tables(
                HashSet::from_iter(vec_string![
                    "foo",
                    "default",
                    "shard_foo",
                    "shard_bar",
                    "monitoring",
                    "foobar"
                ]),
                vec_string!["foo"]
            )
            .unwrap(),
            HashSet::from_iter(vec_string!["foo"])
        );
    }
}
