pub use std::boxed::Box;
pub use std::collections::HashMap;
pub use std::string::ToString;

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([ $( $elem:tt ),* ]) => {
        $crate::Json::Array(<[_]>::into_vec($crate::macros::Box::new([ $( json!($elem) ),* ])))
    };
    ([ $( $elem:tt ),* ,]) => {   // support an optional extra comma at the end of array
        json!([$( $elem ),*])
    };
    ({ $( $key:tt : $value:tt ),* }) => {
        $crate::Json::Obj($crate::macros::Box::new(vec![ $( ($crate::macros::ToString::to_string($key), json!($value)) ),* ].into_iter().collect()))
    };
    ({ $( $key:tt : $value:tt ),* ,}) => {
        json!({$( $key : $value ),*})
    };
    ( $prim:tt ) => { $crate::Json::from($prim) };  // cover Bool, Num, String
}
