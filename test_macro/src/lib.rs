use std::collections::HashMap;

pub mod macros;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    String(String),
    Array(Vec<Json>),
    Obj(Box<HashMap<String, Json>>),
}

// implement From trait for numerical value for Json
macro_rules! gen_from_json_num {
    ( $( $n: ident )* ) => {
        $(
            impl From<$n> for Json {
                fn from(num: $n) -> Self {
                    Json::Num(num as f64)
                }
            }
        )*
    };
}

gen_from_json_num!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

// implement From trait for bool for Json
impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Bool(b)
    }
}

// implement From trait for String for Json
impl From<String> for Json {
    fn from(s: String) -> Self {
        Json::String(s)
    }
}

// implement From trait for str for Json
impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Self {
        Json::String(s.to_string())
    }
}

#[test]
fn test_json_null() {
    assert_eq!(json!(null), Json::Null);
    assert_eq!(
        json!([3, true, "rust"]),
        Json::Array(vec![
            Json::Num(3.0),
            Json::Bool(true),
            Json::String("rust".to_string())
        ])
    );
    assert_eq!(
        json!([
            {
                "rustacean": "huo",
                "credit": 10,
            },
            {
                "cat": "fufu",
            },
        ]),
        Json::Array(vec![
            Json::Obj(Box::new(
                vec![
                    ("rustacean".to_string(), Json::String("huo".to_string())),
                    ("credit".to_string(), Json::Num(10.0))
                ]
                .into_iter()
                .collect()
            )),
            Json::Obj(Box::new(
                vec![("cat".to_string(), Json::String("fufu".to_string()))]
                    .into_iter()
                    .collect()
            )),
        ])
    );
}
