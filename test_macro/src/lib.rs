use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    String(String),
    Array(Vec<Json>),
    Obj(Box<HashMap<String, Json>>)
}

#[macro_export]
macro_rules! json {
    (null) => {
        Json::Null
    }
}

#[test]
fn test_json_null() {
    assert_eq!(json!(null), Json::Null);
}
