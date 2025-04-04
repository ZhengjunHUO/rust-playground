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

#[macro_export]
macro_rules! vec_string {
    ($( $elem:expr ),*) => { vec![$($elem.to_string()),*] };
}

#[macro_export]
macro_rules! rendered_from_env {
    ($target:expr, $envname:literal) => {
        if let Ok(val) = env::var($envname) {
            $target = Some(val);
        }
    };
}

#[macro_export]
macro_rules! is_activated {
    ($envname:literal) => {
        if std::env::var($envname).is_ok() && std::env::var($envname) == Ok(String::from("1")) {
            true
        } else {
            false
        }
    };
}
